//! `RBX::Network::CustomSerializer` — compressed unit-vector codec.
//!
//! Decompiled from 0x9bedec (`readVector`) and 0x9c30ac (`writeVector`).
//! A vector is stored as its length (raw `f32`) plus two quantized unit
//! components; the third is rebuilt from the unit-length constraint and the
//! length. Short mode packs each component into 8 bits, full mode into 16.

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

use super::bitstream::BitStream;

/// `SFFlag::getAllowPhysicsPacketCompression` (IDA 0x9be1ce, 0x9c2aba):
/// global switch selecting the compressed vector path. The original reads a
/// server flag; this models it as runtime state defaulting to on.
static ALLOW_PHYSICS_PACKET_COMPRESSION: AtomicBool = AtomicBool::new(true);

pub fn allow_physics_packet_compression() -> bool {
    ALLOW_PHYSICS_PACKET_COMPRESSION.load(Ordering::Relaxed)
}

pub fn set_allow_physics_packet_compression(value: bool) {
    ALLOW_PHYSICS_PACKET_COMPRESSION.store(value, Ordering::Relaxed);
}

/// `RBX::NetworkSettings::heavyCompressionEnabled` (IDA 0x9c2cae).
static HEAVY_COMPRESSION_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn heavy_compression_enabled() -> bool {
    HEAVY_COMPRESSION_ENABLED.load(Ordering::Relaxed)
}

pub fn set_heavy_compression_enabled(value: bool) {
    HEAVY_COMPRESSION_ENABLED.store(value, Ordering::Relaxed);
}

/// `DFInt::PhysicsCompressionSizeFilter` (IDA 0x9c3116): vectors no longer
/// than this use the 8-bit packing in heavy mode. Runtime-tunable upstream;
/// defaults to `0.0` (full precision unless configured).
static PHYSICS_COMPRESSION_SIZE_FILTER: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

pub fn physics_compression_size_filter() -> f32 {
    f32::from_bits(PHYSICS_COMPRESSION_SIZE_FILTER.load(Ordering::Relaxed))
}

pub fn set_physics_compression_size_filter(value: f32) {
    PHYSICS_COMPRESSION_SIZE_FILTER.store(value.to_bits(), Ordering::Relaxed);
}

/// Short-mode (8-bit) component bias, `flt_9C3270` (IDA 0x9c3156).
pub const SHORT_COMPONENT_BIAS: f32 = f32::from_bits(0x3B00_8081);
/// Full-mode (16-bit) component bias, `flt_9C326C` (IDA 0x9c3158).
pub const FULL_COMPONENT_BIAS: f32 = f32::from_bits(0x3700_0080);
/// Short-mode zero threshold (IDA 0x9bee42).
pub const SHORT_EPS: f32 = f32::from_bits(0x3B00_8081);
/// Full-mode zero threshold (IDA 0x9bee3e): exact `2^-17`.
pub const FULL_EPS: f32 = f32::from_bits(0x3700_0000);

/// `RBX::Network::CustomSerializer::readVector(float &,float &,float &,
/// RakNet::BitStream &)` (IDA 0x9bedec).
///
/// The first (`this`) parameter is the x out-param passed as receiver. Reads
/// the short flag, the length, two signed quantized components, and the z
/// sign, then scales by the length. Returns `false` only when the length
/// itself cannot be read (IDA 0x9bf00c); later short reads reuse the previous
/// component value instead of the original's uninitialized stack slot.
pub fn read_vector(stream: &mut BitStream, out: &mut [f32; 3]) -> bool {
    let [x, y, z] = out;
    // IDA 0x9bee0a: one-bit short flag.
    let Some(short) = stream.read_bit() else {
        return false;
    };
    // IDA 0x9bee2a: `BitStream::Read<float>` for the length.
    let Some(mag) = stream.read_f32() else {
        return false;
    };
    let eps = if short { SHORT_EPS } else { FULL_EPS };
    if mag <= eps {
        // IDA 0x9beea4: zero-vector fast path.
        *x = 0.0;
        *y = 0.0;
        *z = 0.0;
        return true;
    }
    // IDA 0x9bee50..0x9beee6: sign bit + 8/16-bit magnitude, read as
    // `raw / scale - bias`, negated when the sign bit is set.
    let component = |stream: &mut BitStream, prev: f32| -> f32 {
        let sign = stream.read_bit().unwrap_or(false);
        let mag_bits = if short {
            stream.read_u8().map(|b| b as f32 / 255.0 - SHORT_COMPONENT_BIAS)
        } else {
            stream
                .read_u16()
                .map(|w| w as f32 / 32767.0 - FULL_COMPONENT_BIAS)
        };
        let value = mag_bits.unwrap_or(prev);
        if sign { -value } else { value }
    };
    *x = component(stream, *x);
    *y = component(stream, *y);
    // IDA 0x9bef7c: z keeps only its sign bit; note the inverted test
    // (`== 0`, IDA 0x9bef9c): bit set means *positive*.
    let z_positive = stream.read_bit().unwrap_or(true);
    let rest = (1.0 - *x * *x - *y * *y).max(0.0).sqrt();
    // IDA 0x9befc2: `sqrt(max(0, 1 - x*x - y*y))`.
    *z = if z_positive { rest } else { -rest };
    // IDA 0x9befd8: scale the unit vector back by the length.
    *x *= mag;
    *y *= mag;
    *z *= mag;
    true
}

/// `RBX::Network::CustomSerializer::writeVector(bool,float const &,
/// float const &,float const &,RakNet::BitStream &)` (IDA 0x9c30ac).
///
/// `heavy` arrives as the `this` pointer slot (fastcall bool). Short mode is
/// selected when `heavy && len <= PhysicsCompressionSizeFilter` (IDA
/// 0x9c3116). Write scales (256.0 / 32640.0) deliberately differ from the
/// read scales (255.0 / 32767.0); preserved as-is.
pub fn write_vector(heavy: bool, x: f32, y: f32, z: f32, stream: &mut BitStream) {
    let len = (x * x + y * y + z * z).sqrt();
    // IDA 0x9c311a/0x9c3128: short flag first, length second (IDA 0x9c3134).
    let short = heavy && len <= physics_compression_size_filter();
    stream.write_bit(short);
    stream.write_f32(len);
    let eps = if short { SHORT_EPS } else { FULL_EPS };
    // IDA 0x9c3144.
    if len > eps {
        // IDA 0x9c3150/0x9c31ce: normalized x, y with explicit sign bits.
        let component = |n: f32, stream: &mut BitStream| {
            stream.write_bit(n < 0.0);
            let (bias, scale) = if short {
                (SHORT_COMPONENT_BIAS, 256.0)
            } else {
                (FULL_COMPONENT_BIAS, 32640.0)
            };
            // IDA 0x9c3180/0x9c31f4: `bias + min(1, |n|)`, scaled with
            // round-to-nearest-even (`vcvt.s32.f32`, IDA 0x9c3194).
            let quant = (bias + n.abs().min(1.0)) * scale;
            let q = quant.round_ties_even() as i32;
            if short {
                stream.write_u8(q as u8);
            } else {
                stream.write_u16(q as u16);
            }
        };
        component(x / len, stream);
        component(y / len, stream);
        // IDA 0x9c3246: z keeps only `z > 0`.
        stream.write_bit(z > 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_vector_short_circuits() {
        let mut s = BitStream::new();
        write_vector(true, 0.0, 0.0, 0.0, &mut s);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [1.0, 2.0, 3.0];
        assert!(read_vector(&mut r, &mut out));
        assert_eq!(out, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn unit_vectors_roundtrip_full_mode() {
        set_physics_compression_size_filter(0.0);
        let mut s = BitStream::new();
        write_vector(false, 0.0, 0.0, 5.0, &mut s);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [0.0; 3];
        assert!(read_vector(&mut r, &mut out));
        assert!(out[0].abs() < 0.01 && out[1].abs() < 0.01 && (out[2] - 5.0).abs() < 0.01);
    }

    #[test]
    fn truncated_stream_returns_false() {
        let mut r = BitStream::from_bytes(&[]);
        let mut out = [0.0; 3];
        assert!(!read_vector(&mut r, &mut out));
    }
}
