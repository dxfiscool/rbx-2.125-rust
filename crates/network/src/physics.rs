//! `RBX::Network` physics replication: rotation packing, the `Compressor`
//! translation codec, and the `PhysicsReceiver` / `PhysicsSender` packet
//! helpers.
//!
//! Ground truth: `Compressor::readTranslation` (0x989268) /
//! `writeTranslation` (0x988c40) / `readCompressionType` (0x989114),
//! `Math::rotationFromByte` (0x3570e8) / `rotationToByte` (0x356ff0) /
//! `rotationToByteBase` (0x356e6c) / `segSizeRadians` (0x356e34),
//! `PhysicsReceiver::readVelocity` (0x9be164) / `readCompactCFrame`
//! (0x9be2ec) / `readMotorAngles` (0x9bcba8),
//! `PhysicsSender::writeVelocity` (0x9c2aa4) / `writeCompactCFrame`
//! (0x9c2b10) / `writeMotorAngles` (0x9c29c0).

#![allow(dead_code)]

use std::f32::consts::{PI, TAU};

use super::bitstream::BitStream;
use super::custom_serializer::{
    allow_physics_packet_compression, heavy_compression_enabled, read_vector, write_vector,
};

/// `RBX::segSizeRadians()` (IDA 0x356e34): `2*pi/256` cached in
/// `dword_131EEB0` (IDA 0x356e5e, bits 1019809755).
pub const SEG_SIZE_RADIANS: f32 = f32::from_bits(1019809755);
/// `Math::pif()` as float (IDA 0x356f22, bits 1078530011).
pub const PI_F32: f32 = f32::from_bits(1078530011);

/// `RBX::Math::rotationFromByte` (IDA 0x3570e8): `byte * seg - pi`.
pub fn rotation_from_byte(byte: u8) -> f32 {
    byte as f32 * SEG_SIZE_RADIANS - PI_F32 // IDA 0x3570f0..0x357114
}

/// `RBX::Math::rotationToByte` + `rotationToByteBase` (IDA 0x356ff0,
/// 0x356e6c): wrap to `[-pi, pi]`, then `lrint((angle + pi) / seg)` clamped
/// to `0..=255` (256 folds to 255, IDA 0x356fd6).
pub fn rotation_to_byte(mut angle: f32) -> u8 {
    // IDA 0x35700c..0x357058: wrap out-of-range angles.
    if angle < -PI || angle >= PI {
        angle -= (angle + PI).div_euclid(TAU).floor() * TAU;
        debug_assert!(
            angle >= -PI && angle <= PI,
            "(answer >= -pi()) && (answer <= pi()) include/Util/Math.h line: 346"
        );
    }
    // IDA 0x356e88..0x356f1e.
    debug_assert!(
        angle <= PI_F32 + 0.0001,
        "angle <= (Math::pif()+0.0001f) Client/App/util/Math.cpp line: 368"
    );
    debug_assert!(
        angle >= -(PI_F32 + 0.0001),
        "angle >= -(Math::pif()+0.0001f) Client/App/util/Math.cpp line: 369"
    );
    // IDA 0x356f22..0x356f3e: `lrintf` rounds half-to-even.
    let i = ((angle + PI_F32) / SEG_SIZE_RADIANS).round_ties_even() as i32;
    debug_assert!(i >= -1, "iAngle >= -1 Client/App/util/Math.cpp line: 374");
    debug_assert!(i <= 256, "iAngle <= 256 Client/App/util/Math.cpp line: 375");
    // IDA 0x356fce..0x356fd8.
    if i <= 0 {
        0
    } else if i >= 255 {
        255
    } else {
        i as u8
    }
}

fn is_nan_inf(v: f32) -> bool {
    // `RBX::Math::isNanInf`.
    v.is_nan() || v.is_infinite()
}

fn is_nan_inf_vec3(v: [f32; 3]) -> bool {
    // `RBX::Math::isNanInfVector3`.
    v.iter().any(|&c| is_nan_inf(c))
}

/// `RBX::CompactCFrame`: axis (12 B) + angle (4 B) + translation (12 B) =
/// 28 B (IDA 0x9bcd22: stride `0x1C`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CompactCFrame {
    pub axis: [f32; 3],
    pub angle: f32,
    pub translation: [f32; 3],
}

impl CompactCFrame {
    pub const ZERO: Self = Self {
        axis: [0.0; 3],
        angle: 0.0,
        translation: [0.0; 3],
    };
}

/// `RBX::Velocity`: linear + angular vectors (IDA 0x9be266..0x9be27a).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Velocity {
    pub linear: [f32; 3],
    pub angular: [f32; 3],
}

impl Velocity {
    /// `RBX::Velocity::zero()` (IDA 0x9be20e..0x9be252).
    pub const ZERO: Self = Self {
        linear: [0.0; 3],
        angular: [0.0; 3],
    };
}

/// `RBX::Network::Compressor::CompressionType` (IDA 0x988c40).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum CompressionType {
    /// Raw floats via `operator<</>>` (IDA 0x989402, 0x988d9e).
    Raw = 0,
    /// Full vectors via `WriteVector`/`ReadVector` (IDA 0x9892d6, 0x988cd4).
    Vector = 1,
    /// Quantized 15/14/15-bit packing (IDA 0x98933e, 0x988cd6).
    #[default]
    Quantized = 2,
}

impl CompressionType {
    fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            0 => Some(Self::Raw),
            1 => Some(Self::Vector),
            2 => Some(Self::Quantized),
            _ => None,
        }
    }
}

/// `RBX::Network::Compressor::readCompressionType` (IDA 0x989114): a 2-bit
/// tag; short reads throw `std::runtime_error`, mirrored here as panics.
fn read_compression_type(stream: &mut BitStream) -> CompressionType {
    // IDA 0x989134: the 2-byte buffer is zeroed before the read.
    let Some(bits) = stream.read_bits(2) else {
        panic!("Compressor::readTranslation failed reading compressionType");
    };
    let tag = bits as u16;
    // IDA 0x989192.
    CompressionType::from_bits(tag as u32).unwrap_or_else(|| {
        debug_assert!(
            false,
            "0 Client/Network/Compressor.cpp line: 201 (IDA 0x98942a)"
        );
        CompressionType::Raw
    })
}

/// `RBX::Network::Compressor::readTranslation` (IDA 0x989268).
pub fn read_translation(stream: &mut BitStream, out: &mut [f32; 3]) {
    match read_compression_type(stream) {
        // IDA 0x98933e..0x9893ec: x/z span +/-1024 in 15 bits, y spans
        // +/-512 in 14 bits: `raw / 16 - offset`.
        CompressionType::Quantized => {
            let x = stream
                .read_bits(15)
                .expect("Compressor::readTranslation failed reading x");
            let y = stream
                .read_bits(14)
                .expect("Compressor::readTranslation failed reading y");
            let z = stream
                .read_bits(15)
                .expect("Compressor::readTranslation failed reading z");
            out[0] = x as f32 / 16.0 - 1024.0; // IDA 0x9893a4
            out[1] = y as f32 / 16.0 - 512.0; // IDA 0x9893d0
            out[2] = z as f32 / 16.0 - 1024.0; // IDA 0x9893ec
        }
        // IDA 0x9892d6.
        CompressionType::Vector => {
            *out = stream
                .read_vector3()
                .expect("Compressor::readTranslation failed reading vector");
        }
        // IDA 0x989402: three raw floats, no failure check.
        CompressionType::Raw => {
            if let Some(v) = stream.read_f32() {
                out[0] = v;
            }
            if let Some(v) = stream.read_f32() {
                out[1] = v;
            }
            if let Some(v) = stream.read_f32() {
                out[2] = v;
            }
        }
    }
}

/// `RBX::Network::Compressor::writeTranslation` (IDA 0x988c40).
///
/// A `Quantized` request downgrades to `Vector` unless x/z fit in +/-1024
/// and y fits in +/-512 (IDA 0x988c4e..0x988cae). Component rounding is
/// round-half-even (`vcvt.s32.f32`); the x/z low clamp (`<= -1 -> 0xFFFF`,
/// IDA 0x988d4a) and y high clamp (`>= 0x4000 -> 0xFFFF`, IDA 0x988d5a) are
/// preserved exactly, including their asymmetry.
pub fn write_translation(stream: &mut BitStream, v: [f32; 3], requested: CompressionType) {
    let mut ty = requested;
    if requested == CompressionType::Quantized {
        // IDA 0x988c54..0x988cae.
        ty = CompressionType::Vector;
        if v[0] <= 1024.0 && v[0] >= -1024.0 && v[1] <= 512.0 && v[1] >= -512.0 {
            if v[2] <= 1024.0 && v[2] >= -1024.0 {
                ty = CompressionType::Quantized;
            }
        }
    }
    // IDA 0x988cb8: 2-bit tag.
    stream.write_bits(ty as u32, 2);
    match ty {
        CompressionType::Quantized => {
            // IDA 0x988cd6..0x988d46: `(c + offset) * 16`, exact float
            // factors from the original (`(x+1024)*32768/2048`,
            // `(y+512)*16384/1024`).
            let qx = ((v[0] + 1024.0) * 32768.0 / 2048.0).round_ties_even() as i32;
            let qy = ((v[1] + 512.0) * 16384.0 / 1024.0).round_ties_even() as i32;
            let qz = ((v[2] + 1024.0) * 32768.0 / 2048.0).round_ties_even() as i32;
            let wx = if qx <= -1 { 0xFFFF } else { qx as u16 };
            let wy = if qy >= 0x4000 { 0xFFFF } else { qy as u16 };
            let wz = if qz <= -1 { 0xFFFF } else { qz as u16 };
            // IDA 0x988d78..0x988d90: 15/14/15 bits.
            stream.write_bits(wx as u32, 15);
            stream.write_bits(wy as u32, 14);
            stream.write_bits(wz as u32, 15);
        }
        // IDA 0x988cd4.
        CompressionType::Vector => stream.write_vector3(v),
        // IDA 0x988d9e..0x988db4.
        CompressionType::Raw => {
            for c in v {
                stream.write_f32(c);
            }
        }
    }
}

/// `RBX::Network::PhysicsReceiver` packet state relevant to these helpers:
/// the replicator compression gate (IDA offset chain `+3716 -> +160`,
/// 0x9be198) and the `SFFlag` packet-compression switch (0x9be1ce).
#[derive(Clone, Debug)]
pub struct PhysicsReceiver {
    pub compression_enabled: bool,
    pub packet_compression_allowed: bool,
    /// Verbose touch/motor log gate (`+94` of the same block, 0x9bcc0e).
    pub verbose_logging: bool,
}

impl PhysicsReceiver {
    /// `RBX::Network::PhysicsReceiver::readVelocity` (IDA 0x9be164).
    pub fn read_velocity(&self, stream: &mut BitStream, velocity: &mut Velocity) {
        // IDA 0x9be198.
        if self.compression_enabled {
            // IDA 0x9be1ce.
            if self.packet_compression_allowed {
                // IDA 0x9be1dc, 0x9be1f2.
                let mut linear = velocity.linear;
                let mut angular = velocity.angular;
                read_vector(stream, &mut linear);
                read_vector(stream, &mut angular);
                velocity.linear = linear;
                velocity.angular = angular;
            } else {
                // IDA 0x9be28a, 0x9be2a0.
                if let Some(v) = stream.read_vector3() {
                    velocity.linear = v;
                }
                if let Some(v) = stream.read_vector3() {
                    velocity.angular = v;
                }
            }
        } else {
            // IDA 0x9be20e..0x9be27a: `Velocity::zero()`.
            *velocity = Velocity::ZERO;
        }
    }

    /// `RBX::Network::PhysicsReceiver::readCompactCFrame` (IDA 0x9be2ec).
    pub fn read_compact_cframe(&self, stream: &mut BitStream, frame: &mut CompactCFrame) {
        // IDA 0x9be2f4: `ReadBit` selects the full-rotation fast path.
        if stream.read_bit().unwrap_or(false) {
            // IDA 0x9be30a..0x9be32a: one rotation byte; the frame is a pure
            // Z rotation (`CompactCFrame(zero, unitZ, angle)`).
            let byte = stream.read_u8().unwrap_or(0);
            frame.axis = [0.0, 0.0, 1.0]; // `G3D::Vector3::unitZ`
            frame.angle = rotation_from_byte(byte); // IDA 0x9be31e
            frame.translation = [0.0; 3]; // `G3D::Vector3::zero`
            debug_assert!(
                !is_nan_inf_vec3(frame.axis),
                "!Math::isNanInfVector3(cFrame.getAxis()) Client/Network/PhysicsReceiver.cpp line: 301"
            );
            debug_assert!(
                !is_nan_inf(frame.angle),
                "!Math::isNanInf(cFrame.getAngle()) Client/Network/PhysicsReceiver.cpp line: 302"
            );
            debug_assert!(
                !is_nan_inf_vec3(frame.translation),
                "!Math::isNanInfVector3(cFrame.translation) Client/Network/PhysicsReceiver.cpp line: 303"
            );
            return;
        }
        // IDA 0x9be44c, 0x9be450.
        let has_translation = stream.read_bit().unwrap_or(false);
        let has_rotation = stream.read_bit().unwrap_or(false);
        if has_translation {
            // IDA 0x9be462.
            let mut t = frame.translation;
            read_translation(stream, &mut t);
            frame.translation = t;
        } else {
            // IDA 0x9be468: zero translation.
            frame.translation = [0.0; 3];
        }
        if has_rotation {
            // IDA 0x9be484..0x9be4c6: axis vector, compressed or raw.
            let mut axis = frame.axis;
            if self.packet_compression_allowed {
                read_vector(stream, &mut axis);
            } else if let Some(v) = stream.read_vector3() {
                axis = v;
            }
            // IDA 0x9be4ce: rotation byte; `operator>>` result unchecked.
            let byte = stream.read_u8().unwrap_or(0);
            let angle = rotation_from_byte(byte); // IDA 0x9be4da
            // IDA 0x9be4ea..0x9be510: canonicalize to a non-negative angle.
            if angle >= 0.0 {
                frame.axis = axis;
                frame.angle = angle;
            } else {
                frame.axis = [-axis[0], -axis[1], -axis[2]];
                frame.angle = -angle;
            }
        } else {
            // IDA 0x9be4a8..0x9be4ba: identity rotation.
            frame.axis = [1.0, 0.0, 0.0]; // `G3D::Vector3::unitX`
            frame.angle = 0.0;
        }
        debug_assert!(
            !is_nan_inf_vec3(frame.axis),
            "!Math::isNanInfVector3(cFrame.getAxis()) Client/Network/PhysicsReceiver.cpp line: 339"
        );
        debug_assert!(
            !is_nan_inf(frame.angle),
            "!Math::isNanInf(cFrame.getAngle()) Client/Network/PhysicsReceiver.cpp line: 340"
        );
        debug_assert!(
            !is_nan_inf_vec3(frame.translation),
            "!Math::isNanInfVector3(cFrame.translation) Client/Network/PhysicsReceiver.cpp line: 341"
        );
    }

    /// `RBX::Network::PhysicsReceiver::readMotorAngles` (IDA 0x9bcba8):
    /// u8 motor count followed by that many compact cframes.
    pub fn read_motor_angles(&self, stream: &mut BitStream, out: &mut Vec<CompactCFrame>) {
        // IDA 0x9bcbd0: `operator>><unsigned char>`.
        let count = stream.read_u8().unwrap_or(0); // IDA 0x9bcbde
        // IDA 0x9bcc06..0x9bcc38: warn on suspiciously large counts.
        if count >= 0x33 && self.verbose_logging {
            eprintln!("Physics-in has {count} motors");
        }
        // IDA 0x9bcd00: `G3D::Array::resize`.
        out.resize(count as usize, CompactCFrame::ZERO);
        // IDA 0x9bcd0e..0x9bcd22: 28-byte stride loop.
        for frame in out.iter_mut() {
            self.read_compact_cframe(stream, frame); // IDA 0x9bcd1c
        }
    }
}

/// `RBX::Network::PhysicsSender` packet state: the compression gate
/// (`+0x44 -> +0xE84 -> +0xA0`, 0x9c2aaa..0x9c2ab8), the translation
/// compression member at +8 (IDA 0x9c2c8c), and the scratch motor-angle
/// array at +0x30 (IDA 0x9c29ca).
#[derive(Clone, Debug, Default)]
pub struct PhysicsSender {
    pub packet_compression: bool,
    pub translation_compression: CompressionType,
    pub temp_motor_angles: Vec<CompactCFrame>,
}

/// Approximate-zero test used by `writeCompactCFrame` (IDA 0x9c2b54..0x9c2b8e):
/// `|v| == 0 || |v| <= (|v| + 1) * 1e-8`, with the `G3D::inf()` guard.
fn approx_zero(v: f32) -> bool {
    if v == 0.0 {
        return true;
    }
    let mag = v.abs();
    let bound = mag + 1.0;
    if bound == f32::INFINITY {
        return false;
    }
    mag <= bound * 0.000_000_01
}

impl PhysicsSender {
    /// `RBX::Network::PhysicsSender::writeVelocity` (IDA 0x9c2aa4).
    pub fn write_velocity(&self, stream: &mut BitStream, velocity: &Velocity) {
        // IDA 0x9c2ab8.
        if !self.packet_compression {
            return; // IDA 0x9c2b08
        }
        // IDA 0x9c2aba: `SFFlag::getAllowPhysicsPacketCompression()`.
        if allow_physics_packet_compression() {
            // IDA 0x9c2ac2..0x9c2aea: heavy flag from `NetworkSettings`.
            let heavy = heavy_compression_enabled();
            write_vector(
                heavy,
                velocity.linear[0],
                velocity.linear[1],
                velocity.linear[2],
                stream,
            ); // IDA 0x9c2ad6
            write_vector(
                heavy,
                velocity.angular[0],
                velocity.angular[1],
                velocity.angular[2],
                stream,
            ); // IDA 0x9c2aea
        } else {
            // IDA 0x9c2af2..0x9c2b04.
            stream.write_vector3(velocity.linear);
            stream.write_vector3(velocity.angular);
        }
    }

    /// `RBX::Network::PhysicsSender::writeCompactCFrame` (IDA 0x9c2b10).
    pub fn write_compact_cframe(&self, stream: &mut BitStream, frame: &CompactCFrame) {
        // IDA 0x9c2b26..0x9c2b8e: translation ~zero (squared norm, no sqrt).
        let t = frame.translation;
        let translation_is_zero = approx_zero(t[0] * t[0] + t[1] * t[1] + t[2] * t[2]);
        // IDA 0x9c2b92..0x9c2bf6: axis.z ~ +/-1 (unit axis => pure Z turn).
        let axis_is_z = (frame.axis[2].abs() - 1.0).abs()
            <= (frame.axis[2].abs() + 1.0) * 0.000_000_01
            || frame.axis[2].abs() == 1.0;
        // IDA 0x9c2bfa..0x9c2c54: rotation present.
        let has_rotation = !approx_zero(frame.angle);
        if translation_is_zero && has_rotation && axis_is_z {
            // IDA 0x9c2cdc..0x9c2cf8: pure-Z fast path, one rotation byte of
            // `axis.z * angle`.
            stream.write_bit(true);
            stream.write_u8(rotation_to_byte(frame.axis[2] * frame.angle));
            return;
        }
        // IDA 0x9c2c6a..0x9c2c7a.
        stream.write_bit(false);
        stream.write_bit(!translation_is_zero); // IDA 0x9c2c72
        stream.write_bit(has_rotation); // IDA 0x9c2c7a
        if !translation_is_zero {
            // IDA 0x9c2c8c: compression member at sender +8.
            write_translation(stream, t, self.translation_compression);
        }
        if has_rotation {
            // IDA 0x9c2c94..0x9c2cc8.
            if allow_physics_packet_compression() {
                write_vector(
                    heavy_compression_enabled(),
                    frame.axis[0],
                    frame.axis[1],
                    frame.axis[2],
                    stream,
                ); // IDA 0x9c2cbc
            } else {
                stream.write_vector3(frame.axis);
            }
            // IDA 0x9c2cd4: `rotationToByte(angle)`.
            stream.write_u8(rotation_to_byte(frame.angle));
        }
    }

    /// `RBX::Network::PhysicsSender::writeMotorAngles` (IDA 0x9c29c0).
    ///
    /// The original pulls `Assembly::getPhysics` into its +0x30 scratch
    /// array; here the caller passes that physics slice directly.
    pub fn write_motor_angles(&mut self, stream: &mut BitStream, physics: &[CompactCFrame]) {
        // IDA 0x9c29d4..0x9c2a18: grow/zero the scratch array, then fill.
        self.temp_motor_angles
            .resize(physics.len(), CompactCFrame::ZERO);
        self.temp_motor_angles.copy_from_slice(physics);
        // IDA 0x9c2a30..0x9c2a70: `tempMotorAngles.size() < 255`.
        debug_assert!(
            self.temp_motor_angles.len() < 0xFF,
            "tempMotorAngles.size() < 255 Client/Network/PhysicsSender.cpp line: 404"
        );
        // IDA 0x9c2a74: `operator<<(bitstream, uchar)`.
        stream.write_u8(self.temp_motor_angles.len() as u8);
        // IDA 0x9c2a84..0x9c2a9c: 28-byte stride loop.
        let scratch = std::mem::take(&mut self.temp_motor_angles);
        for frame in &scratch {
            self.write_compact_cframe(stream, frame); // IDA 0x9c2a90
        }
        self.temp_motor_angles = scratch;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_byte_roundtrip() {
        for b in [0u8, 1, 64, 128, 200, 255] {
            let back = rotation_to_byte(rotation_from_byte(b));
            assert!((back as i16 - b as i16).abs() <= 1, "byte {b} -> {back}");
        }
    }

    #[test]
    fn translation_quantized_roundtrip() {
        let mut s = BitStream::new();
        write_translation(&mut s, [100.0, -100.0, 0.0], CompressionType::Quantized);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [0.0; 3];
        read_translation(&mut r, &mut out);
        for (a, b) in [100.0, -100.0, 0.0].iter().zip(out.iter()) {
            assert!((a - b).abs() < 0.2, "{a} vs {b}");
        }
    }

    #[test]
    fn cframe_identity_roundtrip() {
        let sender = PhysicsSender {
            packet_compression: true,
            ..Default::default()
        };
        let receiver = PhysicsReceiver {
            compression_enabled: true,
            packet_compression_allowed: true,
            verbose_logging: false,
        };
        let mut s = BitStream::new();
        sender.write_compact_cframe(
            &mut s,
            &CompactCFrame {
                axis: [0.0, 0.0, 1.0],
                angle: 1.0,
                translation: [0.0; 3],
            },
        );
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut frame = CompactCFrame::ZERO;
        receiver.read_compact_cframe(&mut r, &mut frame);
        assert!((frame.angle - 1.0).abs() < 0.05);
        assert_eq!(frame.translation, [0.0; 3]);
    }

    #[test]
    fn motor_angles_roundtrip() {
        let mut sender = PhysicsSender {
            packet_compression: true,
            ..Default::default()
        };
        let receiver = PhysicsReceiver {
            compression_enabled: true,
            packet_compression_allowed: true,
            verbose_logging: false,
        };
        let physics = vec![
            CompactCFrame {
                axis: [1.0, 0.0, 0.0],
                angle: 0.0,
                translation: [1.0, 2.0, 3.0],
            },
            CompactCFrame::ZERO,
        ];
        let mut s = BitStream::new();
        sender.write_motor_angles(&mut s, &physics);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = Vec::new();
        receiver.read_motor_angles(&mut r, &mut out);
        assert_eq!(out.len(), 2);
        assert!((out[0].translation[0] - 1.0).abs() < 0.2);
    }
}
