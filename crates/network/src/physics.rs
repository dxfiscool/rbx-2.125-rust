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

use std::collections::{HashMap, HashSet};
use std::f32::consts::{PI, TAU};

use super::bitstream::BitStream;
use super::custom_serializer::{
    allow_physics_packet_compression, heavy_compression_enabled, read_norm_quat, read_vector,
    write_norm_quat, write_vector,
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
    /// Touch-pair set at +20 (IDA 0x9c0a9c).
    pub touches: HashSet<TouchPair>,
    /// Live `Workspace` touch-signal connection (`scoped_connection` at +44,
    /// IDA 0x9c0ab8/0x9c2230).
    pub touches_connected: bool,
    /// Job (`+4`, IDA 0x9c0f14) and `TouchJob` (`+12`, IDA 0x9c1370) slots
    /// submitted to the `TaskScheduler` by `start` (IDA 0x9c0dd4).
    pub jobs_started: bool,
    /// Double at +80, bits `0x3FA999999999999A` = 0.05 (IDA 0x9c09fe..0x9c0a00).
    pub interval_80: f64,
    /// Byte at +108 set to 1 (IDA 0x9c0a12).
    pub flag_108: bool,
    /// Dword at +112 set to 1 (IDA 0x9c0a18).
    pub field_112: u32,
}

impl PhysicsSender {
    /// `RBX::Network::PhysicsSender::PhysicsSender` (IDA 0x9c0908, C2):
    /// vtable + cleared touch set with prime-sized buckets (IDA 0x9c0976..0x9c09b0;
    /// `HashSet` manages its own buckets), zeroed job slots, the 0.05
    /// interval, and the +108/+112 flags. The `Replicator &` link (IDA
    pub fn new() -> Self {
        Self {
            packet_compression: false,
            translation_compression: CompressionType::default(),
            temp_motor_angles: Vec::new(),
            touches: HashSet::new(),
            touches_connected: false,
            jobs_started: false,
            interval_80: 0.05, // IDA 0x9c09fe..0x9c0a00
            flag_108: true,    // IDA 0x9c0a12
            field_112: 1,      // IDA 0x9c0a18
        }
    }

    /// `RBX::Network::PhysicsSender::onTouchStep` (IDA 0x9c0a9c): emplaces
    /// the pair into the set at +20 (disasm `ADD R1, R0, #0x14` + unordered
    /// `emplace`). Returns whether it was newly inserted.
    pub fn on_touch_step(&mut self, pair: TouchPair) -> bool {
        self.touches.insert(pair)
    }

    /// `RBX::Network::PhysicsSender::connectTouches` (IDA 0x9c0ab8): walks
    /// the replicator to its root `ServiceProvider`, requires the
    /// `Workspace`, and inserts an `onTouchStep` slot into its touch signal
    /// at +588 (IDA 0x9c0bd4..0x9c0c2a), storing the `scoped_connection` at
    /// +44 and replacing any previous one (IDA 0x9c0c40..0x9c0c5e). With no
    /// crate-side `Workspace` type, this records the live connection.
    pub fn connect_touches(&mut self) {
        self.touches_connected = true;
    }

    /// `RBX::Network::PhysicsSender::start` (IDA 0x9c0dd4): `connectTouches`
    /// (0x9c0df6), then a `Job` (0x1F8 bytes, stored at +4) and a `TouchJob`
    /// (stored at +12), each submitted via `TaskScheduler::singleton()->add`
    /// (0x9c1176/0x9c11cc and 0x9c15d0/0x9c1626). Job construction and
    /// scheduler submission stay engine-side; this records the touch
    /// connection and both slots as live.
    pub fn start(&mut self) {
        self.connect_touches();
        self.jobs_started = true;
    }

    /// `RBX::Network::PhysicsSender::~PhysicsSender` (IDA 0x9c1f50, D2):
    /// removes both scheduler jobs (IDA 0x9c2016/0x9c2148), resets the job
    /// pointers (IDA 0x9c20da/0x9c220c), disconnects the touch connection
    /// (IDA 0x9c2230), and clears the touch set (IDA 0x9c223c).
    pub fn tear_down(&mut self) {
        self.touches.clear();
        self.touches_connected = false;
        self.jobs_started = false;
    }

    /// `boost::shared_ptr<PhysicsSender::Job>::reset` (IDA 0x9c2f6c) and
    /// `shared_ptr<PhysicsSender::TouchJob>::reset` (IDA 0x9c300c):
    /// release one scheduler job slot. Per-slot storage stays
    /// engine-side; the crate tracks both slots jointly in
    /// `jobs_started`, so either reset clears it.
    pub fn reset_job_slot(&mut self) {
        self.jobs_started = false;
    }
}

impl Drop for PhysicsSender {
    /// D0 (IDA 0x9c1ea4) is D2 plus `operator delete`; D1 (IDA 0x9c1f44)
    /// tail-calls D2 (IDA 0x9c1f48). Rust runs this then frees the box,
    /// covering all three.
    fn drop(&mut self) {
        self.tear_down();
    }
}

/// `RBX::TouchPair`: the two touching primitives. The original stores
/// pointers (IDA 0x9c0a9c emplaces into the set at sender +20); the ids
/// stand in for them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TouchPair {
    pub first: u32,
    pub second: u32,
}

impl TouchPair {
    pub fn new(first: u32, second: u32) -> Self {
        Self { first, second }
    }
}

/// `RBX::TaskScheduler::Job::computeStandardSleepTime` inputs (IDA 0x24a210).
#[derive(Clone, Copy, Debug)]
pub struct SleepContext {
    /// Throttle floor: `throttledSleepTime` when the job's data is throttled,
    /// else `0.0` (IDA 0x24a2c2..0x24a2e0).
    pub throttled_floor: f64,
    /// `RBX::TaskScheduler::Job::sleepAdjustMethod` (IDA 0x24a2fc).
    pub adjust_method: u32,
    /// `*(stats + 35)` double used by adjust method 1 (IDA 0x24a314).
    pub avg_sleep: f64,
    /// `*(stats + 36)` double used by adjust method 2 (IDA 0x24a346).
    pub avg_run: f64,
    /// `now - *(stats + 252)` used by adjust method 2 (IDA 0x24a338).
    pub since_last_run: f64,
}

impl Default for SleepContext {
    fn default() -> Self {
        Self {
            throttled_floor: 0.0,
            adjust_method: 0,
            avg_sleep: 0.0,
            avg_run: 0.0,
            since_last_run: 0.0,
        }
    }
}

/// `RBX::TaskScheduler::Job::computeStandardSleepTime` (IDA 0x24a210),
/// reduced to its pure inputs. `elapsed` arrives packed in the low dword of
/// the rate double (IDA 0x24a230); the rate itself is the float at stats+496
/// (IDA 0x9c58ea, 0x9c6222).
pub fn standard_sleep_time(elapsed: f64, rate_hz: f32, ctx: &SleepContext) -> f64 {
    // IDA 0x24a2f4: desired period.
    let desired = 1.0 / f64::from(rate_hz);
    if ctx.adjust_method == 1 {
        // IDA 0x24a314..0x24a31a: overrunning average sleeps the floor.
        if ctx.avg_sleep > desired * 1.05 {
            return ctx.throttled_floor;
        }
    } else if ctx.adjust_method == 2 {
        // IDA 0x24a35a..0x24a36c: overrunning run time sleeps the floor.
        let run = if ctx.since_last_run > ctx.avg_run + ctx.avg_run {
            ctx.since_last_run
        } else {
            ctx.avg_run
        };
        if run > desired * 1.05 {
            return ctx.throttled_floor;
        }
    }
    // IDA 0x24a372..0x24a38a: `max(floor, desired - elapsed)`.
    (desired - elapsed).max(ctx.throttled_floor)
}

/// The 12-byte scheduler error output written to `this` by
/// `TouchJob::error` / `Job::error` (IDA 0x9c5afe..0x9c5b18) and filled by
/// `computeStandardError` (IDA 0x24a208..0x24a20c): an error double plus a
/// zero flag; the tail bytes are padding.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct StandardError {
    pub value: f64,
    pub flag: u8,
    pub _pad: [u8; 3],
}

impl StandardError {
    /// The gated-off shape (IDA 0x9c5afe..0x9c5b18).
    // BUG: the original copies uninitialized stack bytes into the padding
    // (IDA 0x9c5b14..0x9c5b18); this zeroes it instead.
    pub fn zero() -> Self {
        Self {
            value: 0.0,
            flag: 0,
            _pad: [0; 3],
        }
    }

    /// `RBX::TaskScheduler::Job::computeStandardError` (IDA 0x24a1f8):
    /// `value = error * rate`, flag cleared.
    pub fn compute(error: f64, rate_hz: f32) -> Self {
        Self {
            value: error * f64::from(rate_hz), // IDA 0x24a208
            flag: 0,                           // IDA 0x24a20c
            _pad: [0; 3],
        }
    }
}

/// `RBX::Network::ReplicatorJob::canSendPacket` gate (IDA 0xae1000),
/// reduced to its pure inputs.
#[derive(Clone, Copy, Debug, Default)]
pub struct SendGate {
    /// Replicator pointer non-null (IDA 0xae1004).
    pub replicator_alive: bool,
    /// `*(replicator + 1200)` connection present (IDA 0xae1006).
    pub connected: bool,
    /// `*(replicator + 3344 + 132)` draining (IDA 0xae1026).
    pub draining: bool,
    /// `*(settings + 184)` draining (IDA 0xae1026).
    pub settings_draining: bool,
    /// `*(replicator + 3344 + 120)` paused (IDA 0xae102e).
    pub paused: bool,
    /// `*(settings + 185)` paused (IDA 0xae1034).
    pub settings_paused: bool,
    /// `*(settings + 172)` budget (IDA 0xae102e..0xae1052).
    pub budget: i32,
    /// `*(replicator + 3344 + 144 + 4 * priority)` used (IDA 0xae102e).
    pub used: i32,
}

/// `RBX::Network::ReplicatorJob::canSendPacket` (IDA 0xae1000).
pub fn can_send_packet(gate: &SendGate) -> bool {
    if !gate.replicator_alive || !gate.connected {
        return false; // IDA 0xae1002..0xae100a
    }
    if gate.draining && gate.settings_draining {
        return false; // IDA 0xae1026: both draining flags set
    }
    if gate.paused && gate.settings_paused {
        return false; // IDA 0xae1034..0xae1038
    }
    // IDA 0xae102e/0xae1052: remaining budget for this priority.
    gate.budget - gate.used > 0
}

/// `RBX::Network::PhysicsSender::TouchJob` (IDA 0x9c58dc..0x9c5e38).
/// Stateless: the original's fields are scheduler bookkeeping.
#[derive(Clone, Copy, Debug, Default)]
pub struct TouchJob;

impl TouchJob {
    /// `RBX::Network::PhysicsSender::TouchJob::sleepTime` (IDA 0x9c58dc):
    /// forwards the stats rate at +496 to `computeStandardSleepTime`
    /// (IDA 0x9c58f2).
    pub fn sleep_time(elapsed: f64, rate_hz: f32, ctx: &SleepContext) -> f64 {
        standard_sleep_time(elapsed, rate_hz, ctx)
    }

    /// `RBX::Network::PhysicsSender::TouchJob::error` (IDA 0x9c58fc).
    /// `replicator_present` is the stats +123 job-data pointer (IDA 0x9c5960),
    /// `job_pending` the stats +122 liveness check (IDA 0x9c59c2..0x9c59ca).
    pub fn error(
        gate: &SendGate,
        replicator_present: bool,
        job_pending: bool,
        error: f64,
        rate_hz: f32,
    ) -> StandardError {
        // IDA 0x9c5958/0x9c5966: gated off or no replicator => zero shape.
        if !can_send_packet(gate) || !replicator_present {
            return StandardError::zero();
        }
        // IDA 0x9c59c2..0x9c59ca: replicator job gone => zero shape.
        if !job_pending {
            return StandardError::zero();
        }
        // IDA 0x9c59e4: `computeStandardError(this, stats, err, rate)`.
        StandardError::compute(error, rate_hz)
    }

    /// `RBX::Network::PhysicsSender::TouchJob::stepDataModelJob` (IDA
    /// 0x9c5bfc): without the +123 stats pointer there is nothing to step
    /// (0x9c5c50/0x9c5d50). Otherwise the +122 job steps the +125 context
    /// (0x9c5cbe, engine-side) and the verdict reports whether that job
    /// was present. Refcount traffic stays engine-side.
    pub fn step_data_model_job(
        stats_present: bool,
        job_present: bool,
        step: &mut dyn FnMut(),
    ) -> bool {
        if !stats_present {
            return false;
        }
        if job_present {
            step();
            return true;
        }
        false
    }
}

/// `RBX::Network::PhysicsSender::Job` (IDA 0x9c6214..0x9c6568).
/// Stateless: the original's fields are scheduler bookkeeping.
#[derive(Clone, Copy, Debug, Default)]
pub struct SendJob;

impl SendJob {
    /// `RBX::Network::PhysicsSender::Job::sleepTime` (IDA 0x9c6214):
    /// same forwarding as [`TouchJob::sleep_time`] (IDA 0x9c622a).
    pub fn sleep_time(elapsed: f64, rate_hz: f32, ctx: &SleepContext) -> f64 {
        standard_sleep_time(elapsed, rate_hz, ctx)
    }

    /// `RBX::Network::PhysicsSender::Job::error` (IDA 0x9c6234): gates only
    /// on `canSendPacket` (IDA 0x9c6248), then `computeStandardError`
    /// (IDA 0x9c6264), else the zero shape (IDA 0x9c6272..0x9c627e).
    // BUG: the original also returns an uninitialized stack byte
    // (IDA 0x9c626a); only the `this` shape is observable, so this returns
    // the shape.
    pub fn error(gate: &SendGate, error: f64, rate_hz: f32) -> StandardError {
        if !can_send_packet(gate) {
            return StandardError::zero();
        }
        StandardError::compute(error, rate_hz)
    }

    /// `RBX::Network::PhysicsSender::Job::stepDataModelJob` (IDA 0x9c6288):
    /// same stats/job gates as the `TouchJob` step; the engine-side body
    /// replicates into `physicsSenderStats`, samples the
    /// buffer-availability average, and steps. The verdict is whether both
    /// were present. Refcount traffic stays engine-side.
    pub fn step_data_model_job(
        stats_present: bool,
        job_present: bool,
        step: &mut dyn FnMut(),
    ) -> bool {
        if !stats_present {
            return false;
        }
        if job_present {
            step();
            return true;
        }
        false
    }
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
/// Normalize a quaternion exactly like `Compressor::writeRotation` does
/// (IDA 0x988b02..0x988b58): `inv = 1 / sqrt(x^2+y^2+z^2+w^2)`, each
/// component scaled. There is no zero guard; a zero input propagates
/// INF/NaN per IEEE, matching the `VDIV`/`VMUL` sequence.
pub fn normalize_quat(q: [f32; 4]) -> [f32; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    let inv = 1.0 / len;
    [q[0] * inv, q[1] * inv, q[2] * inv, q[3] * inv]
}

/// `RBX::Network::Compressor::writeRotation(RakNet::BitStream &,
/// G3D::Matrix3 const&, CompressionType)` (IDA 0x988ad8).
///
/// The caller passes the primitive quaternion (`Quaternion::Quaternion`
/// of the rotation matrix, IDA 0x988afa); the matrix conversion itself
/// stays engine-side.
pub fn write_rotation(stream: &mut BitStream, quat: [f32; 4], compression: CompressionType) {
    // IDA 0x988aec..0x988af2: the requested tag goes out first, verbatim.
    stream.write_bits(compression as u32, 2);
    let [x, y, z, w] = normalize_quat(quat);
    match compression {
        // IDA 0x988b60..0x988ba6: Quantized honors the packet-compression
        // switch (IDA 0x988b7c).
        CompressionType::Quantized => {
            if allow_physics_packet_compression() {
                write_norm_quat(heavy_compression_enabled(), w, x, y, z, stream);
            } else {
                stream.write_norm_quat(w, x, y, z);
            }
        }
        // IDA 0x988b62..0x988b7a: Vector always uses the RakNet packing.
        CompressionType::Vector => stream.write_norm_quat(w, x, y, z),
        // IDA 0x988baa..0x988bcc: Raw writes w first, then x, y, z.
        CompressionType::Raw => {
            stream.write_f32(w);
            stream.write_f32(x);
            stream.write_f32(y);
            stream.write_f32(z);
        }
    }
    // NOTE: an out-of-range tag writes its low 2 bits above, then hits
    // `ReleaseAssert("0", Compressor.cpp:56)` (IDA 0x988bee..0x988c3c);
    // `CompressionType` cannot spell such a tag, so that arm has no
    // reachable counterpart here.
}
/// `RBX::Network::Compressor::readRotation(RakNet::BitStream &,
/// G3D::Matrix3 &)` (IDA 0x988e14), reduced to the quaternion: the trailing
/// `Quaternion::toRotationMatrix` (IDA 0x98901c) stays engine-side.
/// Short reads throw `std::runtime_error` in the original, mirrored here as
/// panics, matching [`read_translation`].
pub fn read_rotation(stream: &mut BitStream, out: &mut [f32; 4]) {
    // IDA 0x988e48..0x988e56: w defaults to 1.0 (identity), x/y/z to 0.
    *out = [1.0, 0.0, 0.0, 0.0];
    match read_compression_type(stream) {
        CompressionType::Quantized => {
            if allow_physics_packet_compression() {
                let (mut w, mut x, mut y, mut z) = (out[0], out[1], out[2], out[3]);
                read_norm_quat(stream, &mut w, &mut x, &mut y, &mut z);
                *out = [w, x, y, z];
            } else if let Some(q) = stream.read_norm_quat() {
                *out = q;
            } else {
                panic!("Failed to read Quaternion");
            }
        }
        // IDA 0x988e9a..0x988ef8.
        CompressionType::Vector => {
            if let Some(q) = stream.read_norm_quat() {
                *out = q;
            } else {
                panic!("Failed to read Quaternion");
            }
        }
        // IDA 0x988f2c..0x988f4a: four raw floats w, x, y, z, no check.
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
            if let Some(v) = stream.read_f32() {
                out[3] = v;
            }
        }
    }
}

/// `RBX::Network::Compressor::writeCompressed` (IDA 0x989738): the input
/// runs through `boost::iostreams::basic_gzip_compressor` and lands on the
/// wire framed as `operator<<(size)` + raw `Write(bytes)`.
///
/// FIDELITY: the gzip transform itself (like the `StringCompressor` Huffman
/// table) stays engine-side — no gzip crate may enter this crate's
/// dependency closure — so the payload passes through framed but
/// uncompressed. The framing (big-endian size + raw bytes) and length are
/// exact, and the pair round-trips.
///
/// The original returns uninitialized stack (`v53->__sig`); this returns
/// the framed payload length instead.
pub fn write_compressed(stream: &mut BitStream, data: &[u8]) -> u32 {
    // IDA 0x989738 tail: `operator<<(size)` then `Write(data, size)`.
    stream.write_u32(data.len() as u32);
    for byte in data {
        stream.write_u8(*byte);
    }
    data.len() as u32
}

/// `RBX::Network::Compressor::readCompressed` (IDA 0x98a0e0):
/// `operator>><uint>(size)`, raw `Read(size)` bytes, then
/// `basic_gzip_decompressor` into the out string. Panics mirror the
/// original's throws on short reads. See [`write_compressed`] for the
/// payload caveat; returns the payload length like the original's chain
/// result slot.
pub fn read_compressed(stream: &mut BitStream, out: &mut Vec<u8>) -> u32 {
    // IDA 0x98a0e0: size first, then the raw byte block.
    let len = stream
        .read_u32()
        .expect("Compressor::readCompressed: failed to read length");
    out.clear();
    out.reserve(len as usize);
    for _ in 0..len {
        out.push(stream.read_u8().expect("Compressor::readCompressed: truncated"));
    }
    len
}

/// One assembly's physics payload for [`PhysicsSender::write_assembly`].
/// The original pulls these out of `Assembly::getConstAssemblyPrimitive`
/// (IDA 0x9c2962) and `Primitive::getPV` (IDA 0x9c296c); the caller supplies
/// them directly.
#[derive(Clone, Debug)]
pub struct AssemblyPacket<'a> {
    /// `PV + 36`: position vector (IDA 0x9c2974).
    pub translation: [f32; 3],
    /// Rotation as the primitive quaternion (IDA 0x988afa).
    pub rotation: [f32; 4],
    /// `PV + 48`: velocity (IDA 0x9c298a).
    pub velocity: Velocity,
    /// `Assembly::getPhysics` slice written by `writeMotorAngles`
    /// (IDA 0x9c2994).
    pub motor_frames: &'a [CompactCFrame],
    /// Low nibble of the byte at primitive + 124 (IDA 0x9c29ac).
    pub flags_nibble: u8,
}

impl PhysicsSender {
    /// `RBX::Network::PhysicsSender::writeAssembly` (IDA 0x9c2950):
    /// translation, rotation, velocity, motor angles, then the 4-bit
    /// flags nibble.
    pub fn write_assembly(&mut self, stream: &mut BitStream, packet: &AssemblyPacket<'_>) {
        // IDA 0x9c2974: `writeTranslation` with the sender's compression
        // member at +16.
        write_translation(stream, packet.translation, self.translation_compression);
        // IDA 0x9c297e.
        write_rotation(stream, packet.rotation, self.translation_compression);
        // IDA 0x9c298a.
        self.write_velocity(stream, &packet.velocity);
        // IDA 0x9c2994.
        self.write_motor_angles(stream, packet.motor_frames);
        // IDA 0x9c29ac..0x9c29bc: `WriteBits(nibble & 0xF, 4)`.
        stream.write_bits((packet.flags_nibble & 0xF) as u32, 4);
    }
}

/// `RBX::Network::PhysicsPacketCache` entry keyed by assembly: the
/// fingerprint of the last packet written for it.
#[derive(Clone, Debug, Default)]
pub struct PhysicsPacketCache {
    fingerprints: HashMap<u32, u64>,
}

impl PhysicsPacketCache {
    /// `RBX::Network::PhysicsPacketCache::fetchIfUpToDate` (IDA 0x9a8924):
    /// a hit replays the cached bytes into the stream and skips the write.
    pub fn fetch_if_up_to_date(&self, key: u32, fingerprint: u64) -> bool {
        self.fingerprints.get(&key) == Some(&fingerprint)
    }

    /// `RBX::Network::PhysicsPacketCache::update` (IDA 0x9a8974): records
    /// the bytes just written. The original reports allocation failure to
    /// its caller (IDA 0x9a8998..0x9a8aae, `ReleaseAssert("0", ...:311)`);
    /// insertion cannot fail here, so this is infallible.
    pub fn update(&mut self, key: u32, fingerprint: u64) {
        self.fingerprints.insert(key, fingerprint);
    }
}

/// `RBX::Network::ErrorCompPhysicsSender` (IDA 0x9a88ec): a
/// [`PhysicsSender`] plus the packet cache at +47 and the flag byte at
/// +196.
#[derive(Clone, Debug, Default)]
pub struct ErrorCompSender {
    pub cache: Option<PhysicsPacketCache>,
}

impl ErrorCompSender {
    /// `RBX::Network::ErrorCompPhysicsSender::writeAssembly` (IDA 0x9a88ec).
    /// `fingerprint` summarizes the packet the base write would emit; the
    /// caller (engine) derives it from the live assembly. On a cache hit
    /// the cached bytes stand (IDA 0x9a8918..0x9a8962); on a miss the base
    /// [`PhysicsSender::write_assembly`] runs inside a bit-cursor snapshot
    /// (IDA 0x9a8964..0x9a896e) and the cache records the result (IDA
    /// 0x9a8974). With no cache the base write runs directly (IDA
    /// 0x9a8ac4).
    pub fn write_assembly(
        &mut self,
        sender: &mut PhysicsSender,
        stream: &mut BitStream,
        key: u32,
        packet: &AssemblyPacket<'_>,
        fingerprint: u64,
    ) {
        let Some(cache) = self.cache.as_mut() else {
            sender.write_assembly(stream, packet);
            return;
        };
        if cache.fetch_if_up_to_date(key, fingerprint) {
            return;
        }
        let start = stream.bits_written();
        sender.write_assembly(stream, packet);
        let _len = stream.bits_written() - start;
        cache.update(key, fingerprint);
    }
}
/// One `receiveMechanismCFrames` iteration payload (IDA 0x9bb4ec): the
/// translation plus the rotation quaternion read for a fresh part. Applying
/// it (`PartInstance::setPhysics` + `addInterpolationSample`, IDA
/// 0x9bb6c4..0x9bb6d4) stays engine-side.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CFrameSample {
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
}

impl PhysicsReceiver {
    /// Freshness gate in `receiveMechanismCFrames` (IDA 0x9bb582..0x9bb594)
    /// and `DirectPhysicsReceiver::receivePacket` (IDA 0x9a3b1a..0x9a3b2a):
    /// the 64-bit stamp at part +164 is fresh when older than `now`.
    pub fn cframe_is_fresh(stamp_lo: u32, stamp_hi: u32, now_lo: u32, now_hi: u32) -> bool {
        stamp_hi < now_hi || (stamp_hi == now_hi && stamp_lo <= now_lo)
    }

    /// Read one `receiveMechanismCFrames` sample (IDA 0x9bb698..0x9bb6b4):
    /// `Compressor::readTranslation` then `readRotation`. The caller loops
    /// this until `receivePart` fails (IDA 0x9bb568), drops stale parts
    /// (see [`PhysicsReceiver::cframe_is_fresh`], IDA 0x9bb594..0x9bb682),
    /// and applies fresh samples engine-side.
    pub fn read_cframe_sample(&self, stream: &mut BitStream) -> CFrameSample {
        let mut sample = CFrameSample::default();
        read_translation(stream, &mut sample.translation);
        let mut rotation = [1.0, 0.0, 0.0, 0.0];
        read_rotation(stream, &mut rotation);
        sample.rotation = rotation;
        sample
    }
}

/// One `MechanismItem` for [`PhysicsReceiver::set_physics_batch`]: the
/// part id (`None` skips, IDA 0x9be6c0), the replicator filter verdict
/// (IDA 0x9be6d8), the `primitive->getWorld()` assert input (IDA
/// 0x9be724..0x9be770), and the assembly-root/grounded gates (IDA
/// 0x9be770..0x9be78e).
#[derive(Clone, Debug)]
pub struct MechanismItemSample<'a> {
    pub part: Option<u32>,
    pub name: &'a str,
    pub filtered: bool,
    pub has_world: bool,
    pub assembly_root: bool,
    pub grounded: bool,
}

/// An applied `setPhysics` item: the part plus the byte stored at
/// assembly +57 — the item +28 flag when this is the first item, else 0
/// (IDA 0x9be8ce..0x9be8dc). Interpolation-vs-direct `PartInstance` writes
/// (IDA 0x9be8e8..0x9be908, gated on item +29) stay engine-side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppliedItem {
    pub part: u32,
    pub root_flag: bool,
}

impl PhysicsReceiver {
    /// `RBX::Network::PhysicsReceiver::setPhysics` (IDA 0x9be624) over a
    /// caller-provided item slice: null parts are skipped, filtered or
    /// grounded or non-root parts only log (gated on the receiver's
    /// [`PhysicsReceiver::verbose_logging`], IDA
    /// 0x9be6e0/0x9be79a/0x9be7e0), and ungrounded root parts apply via
    /// `Assembly::setPhysics` (IDA 0x9be8c8).
    pub fn set_physics_batch(
        &self,
        items: &[MechanismItemSample<'_>],
        first_flag_28: bool,
    ) -> Vec<AppliedItem> {
        let mut applied = Vec::new();
        for (index, item) in items.iter().enumerate() {
            // IDA 0x9be6c0.
            let Some(part) = item.part else { continue };
            // IDA 0x9be6d8..0x9be6e6.
            if item.filtered {
                if self.verbose_logging {
                    eprintln!("filterPhysics {}", item.name); // IDA 0x9be716
                }
                continue;
            }
            // IDA 0x9be724..0x9be770: `primitive->getWorld()`.
            debug_assert!(item.has_world, "primitive->getWorld()");
            // IDA 0x9be770.
            if !item.assembly_root {
                if self.verbose_logging {
                    eprintln!("!isAssemblyRootPrimitive {}", item.name); // IDA 0x9be816
                }
                continue;
            }
            // IDA 0x9be78e.
            if item.grounded {
                if self.verbose_logging {
                    eprintln!("computeIsGrounded {}", item.name); // IDA 0x9be7d0
                }
                continue;
            }
            // IDA 0x9be8c8..0x9be8dc.
            applied.push(AppliedItem {
                part,
                root_flag: index == 0 && first_flag_28,
            });
        }
        applied
    }
}
/// `RBX::IndexedTree` nesting reduced to owned children for the
/// `PhysicsSender` bind visitors (IDA 0x9c35a0..0x9c3850).
#[derive(Clone, Debug, Default)]
pub struct AssemblyTree {
    pub id: u32,
    pub children: Vec<AssemblyTree>,
}

impl AssemblyTree {
    /// `RBX::IndexedTree::visitConstMeAndChildren<Assembly, ...>`
    /// (IDA 0x9c35a0): invoke the bind on self (the `a3 & 1` thunk select,
    /// IDA 0x9c35c6..0x9c35d2, is a `boost::bind` dispatch detail with no
    /// Rust counterpart), then recurse into each child. The
    /// `indexOf(array[n]) == n` invariant (IDA 0x9c35fa..0x9c360a,
    /// `IndexArray.h:103`) holds by construction for owned children and is
    /// re-checked.
    pub fn visit_const_me_and_children(&self, visit: &mut dyn FnMut(&AssemblyTree)) {
        visit(self);
        for (n, child) in self.children.iter().enumerate() {
            debug_assert_eq!(
                self.children
                    .iter()
                    .position(|c| core::ptr::eq(c, child)),
                Some(n),
                "indexOf(array[n]) == n"
            );
            child.visit_const_me_and_children(visit);
        }
    }
}

/// `RBX::Mechanism` nesting for `visitPrimitivesImpl` (IDA 0x9c3664): the
/// assembly primitive plus child mechanisms.
#[derive(Clone, Debug, Default)]
pub struct MechanismTree {
    pub primitive: u32,
    pub children: Vec<MechanismTree>,
}

impl MechanismTree {
    /// `RBX::Mechanism::visitPrimitivesImpl<...>` (IDA 0x9c3664): assert
    /// the assembly primitive (`p`, `Assembly.h:203`, IDA
    /// 0x9c368c..0x9c36d2), visit it through the assembly step (IDA
    /// 0x9c36d4), then recurse into each child mechanism (IDA 0x9c3760)
    /// with the same index invariant as above (IDA 0x9c3702..0x9c3712).
    pub fn visit_primitives(&self, visit: &mut dyn FnMut(u32)) {
        debug_assert_ne!(self.primitive, 0, "p Assembly.h:203");
        visit(self.primitive);
        for (n, child) in self.children.iter().enumerate() {
            debug_assert_eq!(
                self.children
                    .iter()
                    .position(|c| core::ptr::eq(c, child)),
                Some(n),
                "indexOf(array[n]) == n"
            );
            child.visit_primitives(visit);
        }
    }
}

/// `PhysicsSender` per-packet compression select, shared by
/// `sendMechanismCFrames` (IDA 0x9c2696..0x9c26a2) and `sendMechanism` (IDA
/// 0x9c27da..0x9c27e6): without the streaming-complex flag or a
/// complex-moving mechanism the `a4` moving bit picks `Vector` (1) over
/// `Quantized` (2); a complex-moving mechanism under the flag keeps `Raw`
/// (0). The values match [`CompressionType`] discriminants exactly.
pub fn select_mechanism_mode(flag_set: bool, complex_moving: bool, moving: bool) -> CompressionType {
    if !flag_set || !complex_moving {
        if moving {
            CompressionType::Vector
        } else {
            CompressionType::Quantized
        }
    } else {
        CompressionType::Raw
    }
}

/// `RBX::Network::PhysicsSender::sendMechanismCFrames` (IDA 0x9c25b8):
/// assert streaming (PhysicsSender.cpp:270) and a non-null mechanism
/// (:276), select the +16 mode, write the coordinate frame behind the
/// id gate (`isReplicationContainer` + `trySerializeId`), visit the
/// mechanism primitives via `sendChildPrimitiveCoordinateFrame`
/// (engine-side), and return the trailing null-`serializeId` (0x9c2756).
/// Mechanism/root lookups and the primitive visit stay engine-side.
#[allow(clippy::too_many_arguments)]
pub fn send_mechanism_cframes(
    stream: &mut BitStream,
    streaming_enabled: bool,
    mechanism_present: bool,
    mode: CompressionType,
    replication_container: bool,
    try_serialize_id: &mut dyn FnMut(&mut BitStream) -> bool,
    translation: [f32; 3],
    rotation: [f32; 4],
    visit_children: &mut dyn FnMut(),
    serialize_null_id: &mut dyn FnMut(&mut BitStream) -> bool,
) -> bool {
    debug_assert!(
        streaming_enabled,
        "replicator.isStreamingEnabled() Client/Network/PhysicsSender.cpp line: 270"
    );
    debug_assert!(
        mechanism_present,
        "mechanism Client/Network/PhysicsSender.cpp line: 276"
    );
    // IDA 0x9c26bc..0x9c26d8.
    if replication_container && try_serialize_id(stream) {
        write_translation(stream, translation, mode);
        write_rotation(stream, rotation, mode);
    }
    // IDA 0x9c272a: visitPrimitivesImpl<sendChildPrimitiveCoordinateFrame>.
    visit_children();
    // IDA 0x9c2756: trailing null-id.
    serialize_null_id(stream)
}

/// `RBX::Network::PhysicsSender::sendMechanism` (IDA 0x9c2758): assert a
/// non-null assembly (:296), select the +16 mode, write the motor-count
/// presence bit plus the count byte, write the root body through the
/// virtual (the `ErrorCompPhysicsSender` override is selected
/// engine-side), visit each child assembly via `sendChildAssembly`
/// (engine-side), and close with the `true` terminator (0x9c2892/0x9c28a2).
/// Assembly lookups and child visits stay engine-side.
#[allow(clippy::too_many_arguments)]
pub fn send_mechanism(
    stream: &mut BitStream,
    assembly_present: bool,
    motor_count: u8,
    write_root: &mut dyn FnMut(&mut BitStream),
    child_count: usize,
    visit_child: &mut dyn FnMut(usize, &mut BitStream),
) {
    debug_assert!(
        assembly_present,
        "assembly Client/Network/PhysicsSender.cpp line: 296"
    );
    // IDA 0x9c27f0..0x9c2802: `v14 ? 1 : 0` bit, then the count byte.
    stream.write_bool(motor_count != 0);
    if motor_count != 0 {
        stream.write_u8(motor_count);
    }
    // IDA 0x9c2812: virtual root-body write.
    write_root(stream);
    // IDA 0x9c281e..0x9c2892: indexed child visits with the IndexArray
    // invariant (`indexOf(array[n]) == n`, IndexArray.h:113).
    for n in 0..child_count {
        visit_child(n, stream);
    }
    // IDA 0x9c2892/0x9c28a2: trailing `true` on both paths.
    stream.write_bool(true);
}

/// `RBX::Assembly` primitive nesting for `visitPrimitivesImpl` (IDA
/// 0x9c3778): each node invokes the bind on its own primitive, then
/// recurses into children that are not assembly roots
/// (`isAssemblyRootPrimitive`, IDA 0x9c3824).
#[derive(Clone, Debug, Default)]
pub struct PrimitiveNode {
    pub id: u32,
    pub is_assembly_root: bool,
    pub children: Vec<PrimitiveNode>,
}

impl PrimitiveNode {
    /// `RBX::Assembly::visitPrimitivesImpl<...>` (IDA 0x9c3778).
    pub fn visit_primitives(&self, visit: &mut dyn FnMut(u32)) {
        // IDA 0x9c3794..0x9c37a6: bind/this dispatch, then invoke.
        visit(self.id);
        for (n, child) in self.children.iter().enumerate() {
            // IDA 0x9c37d6..0x9c3812: `indexOf(array[n]) == n`.
            debug_assert_eq!(
                self.children
                    .iter()
                    .position(|c| core::ptr::eq(c, child)),
                Some(n),
                "indexOf(array[n]) == n"
            );
            // IDA 0x9c3824..0x9c383a.
            if !child.is_assembly_root {
                child.visit_primitives(visit);
            }
        }
    }
}

/// `RBX::Network::PhysicsSender::canSend` (IDA 0x9c2d18): assert the part
/// belongs to the assembly (PhysicsSender.cpp:466), refuse nulls, apply
/// the streaming per-part gate when its flag is set (0x9c2d9c..0x9c2dca),
/// and otherwise send unless the part is still serialize-pending when
/// streaming is off (0x9c2db0..0x9c2dc6). The replicator gate and pending
/// table stay engine-side.
pub fn can_send(
    assembly_present: bool,
    part_present: bool,
    primitives_match: bool,
    stream_gate: Option<bool>,
    streaming_enabled: bool,
    serialize_pending: bool,
) -> bool {
    debug_assert!(
        !assembly_present
            || !part_present
            || primitives_match,
        "!assembly || !part || (assembly->getConstAssemblyPrimitive() == part->getConstPartPrimitive()) Client/Network/PhysicsSender.cpp line: 466"
    );
    // IDA 0x9c2d88..0x9c2d8e.
    if !assembly_present || !part_present {
        return false;
    }
    // IDA 0x9c2d9c..0x9c2dca.
    if let Some(pass) = stream_gate {
        if !pass {
            return false;
        }
    }
    // IDA 0x9c2db0..0x9c2dd2.
    if !streaming_enabled {
        return !serialize_pending;
    }
    true
}

/// `RBX::Network::PhysicsSender::sendPhysicsData` (IDA 0x9c2dd4): assert
/// the part (:483), require an assembly-root primitive whose `canSend`
/// passes, then either the streaming branch (extents check selects
/// `sendMechanismCFrames` on a region miss, else an id-gated fallthrough
/// to `sendMechanism`) or the direct branch (`trySerializeId`- or
/// `serializeId`-gated `sendMechanism`). Extents, ids, and mechanism
/// writes stay engine-side behind the closures.
#[allow(clippy::too_many_arguments)]
pub fn send_physics_data(
    stream: &mut BitStream,
    part_present: bool,
    assembly_root: bool,
    sendable: bool,
    streaming_enabled: bool,
    in_streamed_regions: bool,
    try_serialize_id: &mut dyn FnMut(&mut BitStream) -> bool,
    serialize_null_id: &mut dyn FnMut(&mut BitStream),
    serialize_id: &mut dyn FnMut(&mut BitStream),
    use_try_serialize_id: bool,
    send_cframes: &mut dyn FnMut(&mut BitStream),
    send_mechanism_body: &mut dyn FnMut(&mut BitStream),
) -> bool {
    debug_assert!(part_present, "part Client/Network/PhysicsSender.cpp line: 483");
    // IDA 0x9c2e38..0x9c2e5e: null part, non-root, or failed canSend send nothing.
    if !part_present || !assembly_root || !sendable {
        return false;
    }
    // IDA 0x9c2e6a..0x9c2f10: streaming branch.
    if streaming_enabled {
        stream.write_bool(false);
        if !in_streamed_regions {
            stream.write_bool(true);
            send_cframes(stream);
            return true;
        }
        stream.write_bool(false);
        if !try_serialize_id(stream) {
            serialize_null_id(stream);
            return false;
        }
    } else if use_try_serialize_id {
        // IDA 0x9c2f1e..0x9c2f2c.
        if !try_serialize_id(stream) {
            return false;
        }
    } else {
        // IDA 0x9c2f4c.
        serialize_id(stream);
    }
    // IDA 0x9c2f58: sendMechanism, sent.
    send_mechanism_body(stream);
    true
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
    fn compressed_framing_roundtrip() {
        // IDA 0x989738/0x98a0e0: `operator<<(size)` + raw bytes; the gzip
        // transform itself stays engine-side, so the framed payload
        // round-trips byte-identical.
        let data = b"hello replication";
        let mut s = BitStream::new();
        let n = write_compressed(&mut s, data);
        assert_eq!(n as usize, data.len());
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = Vec::new();
        assert_eq!(read_compressed(&mut r, &mut out), data.len() as u32);
        assert_eq!(out, data);
    }

    #[test]
    fn cframe_identity_roundtrip() {
        let mut sender = PhysicsSender::new();
        sender.packet_compression = true;
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
        let mut sender = PhysicsSender::new();
        sender.packet_compression = true;
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
    #[test]
    fn rotation_raw_roundtrip() {
        let mut s = BitStream::new();
        write_rotation(&mut s, [0.0, 0.0, 0.0, 1.0], CompressionType::Raw);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [0.0; 4];
        read_rotation(&mut r, &mut out);
        assert_eq!(out, [1.0, 0.0, 0.0, 0.0]);
    }
    #[test]
    fn rotation_vector_roundtrip() {
        // The Vector tag selects the RakNet packing on both sides without
        // consulting the packet-compression switch (IDA 0x988b62/0x988e9a).

        let mut s = BitStream::new();
        write_rotation(&mut s, [0.5, 0.5, 0.5, 0.5], CompressionType::Vector);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [0.0; 4];
        read_rotation(&mut r, &mut out);
        // RakNet u16 grid: x/y/z within one step, w rebuilt positive.
        for (got, want) in out[1..].iter().zip([0.5, 0.5, 0.5]) {
            assert!((got - want).abs() < 1.0 / 65535.0 + 1e-6, "{got} vs {want}");
        }
        assert!(out[0] > 0.0);
    }
    #[test]
    fn mechanism_mode_select_truth_table() {
        // IDA 0x9c2696..0x9c26a2: flag+complex keeps Raw, else moving picks.
        assert_eq!(select_mechanism_mode(true, true, false), CompressionType::Raw);
        assert_eq!(select_mechanism_mode(true, true, true), CompressionType::Raw);
        assert_eq!(select_mechanism_mode(true, false, true), CompressionType::Vector);
        assert_eq!(select_mechanism_mode(true, false, false), CompressionType::Quantized);
        assert_eq!(select_mechanism_mode(false, true, true), CompressionType::Vector);
        assert_eq!(select_mechanism_mode(false, true, false), CompressionType::Quantized);
    }

    #[test]
    fn mechanism_cframes_gates_and_terminates() {
        // IDA 0x9c25b8: container+id gate around the CF write, trailing null-id verdict out.
        use crate::bitstream::BitStream;
        let mut s = BitStream::new();
        let sent = send_mechanism_cframes(
            &mut s,
            true,
            true,
            CompressionType::Raw,
            true,
            &mut |_| true,
            [1.0, 2.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
            &mut || {},
            &mut |_| true,
        );
        assert!(sent);
        assert!(s.bits_written() > 0);
        let mut s = BitStream::new();
        let mut visited = false;
        let sent = send_mechanism_cframes(
            &mut s,
            true,
            true,
            CompressionType::Raw,
            false,
            &mut |_| panic!("gated"),
            [0.0; 3],
            [0.0, 0.0, 0.0, 1.0],
            &mut || visited = true,
            &mut |_| false,
        );
        assert!(!sent);
        assert!(visited);
    }

    #[test]
    fn mechanism_frames_motors_children_terminator() {
        // IDA 0x9c2758: motor bit+byte, root, indexed children, trailing true.
        use crate::bitstream::BitStream;
        let mut s = BitStream::new();
        let mut seen = Vec::new();
        send_mechanism(&mut s, true, 3, &mut |st| st.write_bool(true), 2, &mut |n, st| {
            seen.push(n);
            st.write_bool(false);
        });
        assert_eq!(seen, vec![0, 1]);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_bool(), Some(true));
        assert_eq!(r.read_u8(), Some(3));
        assert_eq!(r.read_bool(), Some(true));
        assert_eq!(r.read_bool(), Some(false));
        assert_eq!(r.read_bool(), Some(false));
        assert_eq!(r.read_bool(), Some(true));
    }

    #[test]
    fn rotation_quantized_custom_roundtrip() {
        crate::custom_serializer::set_allow_physics_packet_compression(true);
        crate::custom_serializer::set_heavy_compression_enabled(true);
        let mut s = BitStream::new();
        write_rotation(&mut s, [0.0, 0.0, 0.0, 1.0], CompressionType::Quantized);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [0.0; 4];
        read_rotation(&mut r, &mut out);
        assert!(out[1].abs() < 0.01 && out[2].abs() < 0.01 && out[3].abs() < 0.01);
        assert!((out[0] - 1.0).abs() < 0.01);
    }

    #[test]
    fn normalize_quat_scales_to_unit() {
        let q = normalize_quat([2.0, 0.0, 0.0, 0.0]);
        assert_eq!(q, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn can_send_gates() {
        // IDA 0x9c2d18: nulls refuse, stream gate vetoes, pending blocks only when not streaming.
        assert!(!can_send(false, true, true, None, true, false));
        assert!(!can_send(true, false, true, None, true, false));
        assert!(!can_send(true, true, true, Some(false), true, false));
        assert!(can_send(true, true, true, Some(true), true, false));
        assert!(can_send(true, true, true, None, true, true));
        assert!(!can_send(true, true, true, None, false, true));
        assert!(can_send(true, true, true, None, false, false));
    }

    #[test]
    fn physics_data_branches() {
        // IDA 0x9c2dd4: region miss -> cframes; region hit + id fail -> null-id; direct -> mechanism.
        use crate::bitstream::BitStream;
        let mut s = BitStream::new();
        let mut cframes = false;
        let sent = send_physics_data(
            &mut s, true, true, true, true, false,
            &mut |_| panic!("id gated by region miss"),
            &mut |_| panic!("no null id on miss"),
            &mut |_| panic!("no direct id in streaming"),
            false,
            &mut |_| cframes = true,
            &mut |_| panic!("mechanism not reached on miss"),
        );
        assert!(sent && cframes);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!((r.read_bool(), r.read_bool()), (Some(false), Some(true)));
        let mut s = BitStream::new();
        let mut nulled = false;
        let sent = send_physics_data(
            &mut s, true, true, true, true, true,
            &mut |_| false,
            &mut |_| nulled = true,
            &mut |_| panic!("no direct id in streaming"),
            false,
            &mut |_| panic!("no cframes on region hit"),
            &mut |_| panic!("no mechanism after id fail"),
        );
        assert!(!sent && nulled);
        let mut s = BitStream::new();
        let mut mechanism = false;
        let sent = send_physics_data(
            &mut s, true, true, true, false, true,
            &mut |_| panic!("flag off skips try"),
            &mut |_| panic!("no null id off streaming"),
            &mut |st| st.write_bool(true),
            false,
            &mut |_| panic!("no cframes off streaming"),
            &mut |_| mechanism = true,
        );
        assert!(sent && mechanism);
        assert!(!send_physics_data(
            &mut s, true, false, true, false, true,
            &mut |_| true, &mut |_| {}, &mut |_| {}, false, &mut |_| {}, &mut |_| {},
        ));
    }
}

#[cfg(test)]
mod sender_tests {
    use super::*;

    fn open_gate() -> SendGate {
        SendGate {
            replicator_alive: true,
            connected: true,
            budget: 100,
            used: 10,
            ..Default::default()
        }
    }

    #[test]
    fn ctor_sets_interval_and_flags() {
        let sender = PhysicsSender::new();
        assert_eq!(sender.interval_80, 0.05);
        assert!(sender.flag_108);
        assert_eq!(sender.field_112, 1);
        assert!(sender.touches.is_empty());
        assert!(!sender.touches_connected);
    }

    #[test]
    fn touch_step_dedups() {
        let mut sender = PhysicsSender::new();
        let pair = TouchPair::new(1, 2);
        assert!(sender.on_touch_step(pair));
        assert!(!sender.on_touch_step(pair));
        sender.connect_touches();
        assert!(sender.touches_connected);
        sender.tear_down();
        assert!(sender.touches.is_empty());
        assert!(!sender.touches_connected);
    }

    #[test]
    fn start_connects_and_submits_jobs() {
        // IDA 0x9c0dd4: connectTouches + Job/TouchJob scheduler submission.
        let mut sender = PhysicsSender::new();
        assert!(!sender.jobs_started);
        sender.start();
        assert!(sender.touches_connected);
        assert!(sender.jobs_started);
        sender.tear_down();
        assert!(!sender.jobs_started);
    }

    #[test]
    fn sleep_time_defaults_to_period_minus_elapsed() {
        let ctx = SleepContext::default();
        let sleep = TouchJob::sleep_time(0.01, 20.0, &ctx);
        assert!((sleep - 0.04).abs() < 1e-9);
        assert_eq!(SendJob::sleep_time(0.01, 20.0, &ctx), sleep);
    }

    #[test]
    fn sleep_time_overrun_sleeps_floor() {
        let ctx = SleepContext {
            adjust_method: 1,
            avg_sleep: 1.0,
            throttled_floor: 0.25,
            ..Default::default()
        };
        assert_eq!(TouchJob::sleep_time(0.0, 20.0, &ctx), 0.25);
    }

    #[test]
    fn touch_error_gating() {
        let err = TouchJob::error(&open_gate(), true, true, 2.0, 20.0);
        assert_eq!(err.value, 40.0);
        assert_eq!(err.flag, 0);
        let closed = SendGate::default();
        assert_eq!(TouchJob::error(&closed, true, true, 2.0, 20.0).value, 0.0);
        assert_eq!(TouchJob::error(&open_gate(), false, true, 2.0, 20.0).value, 0.0);
        assert_eq!(TouchJob::error(&open_gate(), true, false, 2.0, 20.0).value, 0.0);
    }

    #[test]
    fn send_job_error_gates_on_packet_only() {
        let err = SendJob::error(&open_gate(), 3.0, 10.0);
        assert_eq!(err.value, 30.0);
        let starved = SendGate {
            budget: 5,
            used: 5,
            ..open_gate()
        };
        assert_eq!(SendJob::error(&starved, 3.0, 10.0).value, 0.0);
    }

    #[test]
    fn gate_requires_budget() {
        let draining = SendGate {
            draining: true,
            settings_draining: true,
            ..open_gate()
        };
        assert!(!can_send_packet(&draining));
        assert!(can_send_packet(&open_gate()));
        assert!(!can_send_packet(&SendGate::default()));
    }
    #[test]
    fn write_assembly_roundtrip_tagged() {
        crate::custom_serializer::set_allow_physics_packet_compression(true);
        crate::custom_serializer::set_heavy_compression_enabled(true);
        let mut sender = PhysicsSender::new();
        sender.translation_compression = CompressionType::Quantized;
        let packet = AssemblyPacket {
            translation: [10.0, -20.0, 30.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: Velocity::ZERO,
            motor_frames: &[],
            flags_nibble: 0xA,
        };
        let mut s = BitStream::new();
        sender.write_assembly(&mut s, &packet);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        // Translation tag + value first (IDA 0x9c2974).
        let mut t = [0.0; 3];
        read_translation(&mut r, &mut t);
        assert!((t[0] - 10.0).abs() < 0.2 && (t[1] + 20.0).abs() < 0.2);
        // Rotation, empty velocity (no packet compression), no motors,
        // then the nibble (IDA 0x9c297e..0x9c29bc).
        let mut q = [0.0; 4];
        read_rotation(&mut r, &mut q);
        assert!((q[0] - 1.0).abs() < 0.01);
        assert_eq!(r.read_u8(), Some(0));
        assert_eq!(r.read_bits(4), Some(0xA));
        // 89 bits written into 12 bytes; the 7 pad bits read back zero.
        assert_eq!(r.read_bits(7), Some(0));
        assert_eq!(r.bits_remaining(), 0);
    }

    #[test]
    fn error_comp_hit_skips_write() {
        let mut sender = PhysicsSender::new();
        sender.translation_compression = CompressionType::Raw;
        let packet = AssemblyPacket {
            translation: [1.0, 2.0, 3.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: Velocity::ZERO,
            motor_frames: &[],
            flags_nibble: 0,
        };
        let mut ec = ErrorCompSender {
            cache: Some(PhysicsPacketCache::default()),
        };
        let mut s = BitStream::new();
        ec.write_assembly(&mut sender, &mut s, 7, &packet, 0xABCD);
        let after_miss = s.bits_written();
        assert!(after_miss > 0);
        // Same fingerprint: hit, stream untouched (IDA 0x9a8918..0x9a8962).
        ec.write_assembly(&mut sender, &mut s, 7, &packet, 0xABCD);
        assert_eq!(s.bits_written(), after_miss);
        // New fingerprint: miss, appends again.
        ec.write_assembly(&mut sender, &mut s, 7, &packet, 0x1234);
        assert_eq!(s.bits_written(), after_miss * 2);
        // No cache: straight base write (IDA 0x9a8ac4).
        let mut bare = ErrorCompSender { cache: None };
        let mut s2 = BitStream::new();
        bare.write_assembly(&mut sender, &mut s2, 7, &packet, 0xABCD);
        assert_eq!(s2.bits_written(), after_miss);
    }

    #[test]
    fn cframe_freshness_gate() {
        assert!(PhysicsReceiver::cframe_is_fresh(5, 9, 6, 9));
        assert!(PhysicsReceiver::cframe_is_fresh(6, 9, 6, 9));
        assert!(!PhysicsReceiver::cframe_is_fresh(7, 9, 6, 9));
        assert!(PhysicsReceiver::cframe_is_fresh(0xFFFF_FFFF, 8, 0, 9));
        assert!(!PhysicsReceiver::cframe_is_fresh(0, 10, 0, 9));
    }

    #[test]
    fn set_physics_batch_applies_roots_only() {
        let receiver = PhysicsReceiver {
            compression_enabled: false,
            packet_compression_allowed: false,
            verbose_logging: false,
        };
        let items = [
            MechanismItemSample {
                part: None,
                name: "null",
                filtered: false,
                has_world: true,
                assembly_root: true,
                grounded: false,
            },
            MechanismItemSample {
                part: Some(11),
                name: "filtered",
                filtered: true,
                has_world: true,
                assembly_root: true,
                grounded: false,
            },
            MechanismItemSample {
                part: Some(22),
                name: "root",
                filtered: false,
                has_world: true,
                assembly_root: true,
                grounded: false,
            },
            MechanismItemSample {
                part: Some(33),
                name: "grounded",
                filtered: false,
                has_world: true,
                assembly_root: true,
                grounded: true,
            },
            MechanismItemSample {
                part: Some(44),
                name: "child",
                filtered: false,
                has_world: true,
                assembly_root: false,
                grounded: false,
            },
        ];
        let applied = receiver.set_physics_batch(&items, true);
        // Only the ungrounded root applies; it is not first, so no flag.
        assert_eq!(
            applied,
            vec![AppliedItem {
                part: 22,
                root_flag: false
            }]
        );
        let first = receiver.set_physics_batch(&items[2..3], true);
        assert_eq!(
            first,
            vec![AppliedItem {
                part: 22,
                root_flag: true
            }]
        );
    }

    #[test]
    fn visitors_walk_preorder_skipping_roots() {
        let tree = AssemblyTree {
            id: 1,
            children: vec![
                AssemblyTree {
                    id: 2,
                    children: vec![],
                },
                AssemblyTree {
                    id: 3,
                    children: vec![AssemblyTree {
                        id: 4,
                        children: vec![],
                    }],
                },
            ],
        };
        let mut order = Vec::new();
        tree.visit_const_me_and_children(&mut |n: &AssemblyTree| order.push(n.id));
        assert_eq!(order, vec![1, 2, 3, 4]);
        let mech = MechanismTree {
            primitive: 10,
            children: vec![MechanismTree {
                primitive: 20,
                children: vec![],
            }],
        };
        let mut seen = Vec::new();
        mech.visit_primitives(&mut |p| seen.push(p));
        assert_eq!(seen, vec![10, 20]);
        let prim = PrimitiveNode {
            id: 100,
            is_assembly_root: true,
            children: vec![
                PrimitiveNode {
                    id: 200,
                    is_assembly_root: true,
                    children: vec![],
                },
                PrimitiveNode {
                    id: 300,
                    is_assembly_root: false,
                    children: vec![],
                },
            ],
        };
        let mut visited = Vec::new();
        prim.visit_primitives(&mut |p| visited.push(p));
        // Roots are not recursed into (IDA 0x9c3824).
        assert_eq!(visited, vec![100, 300]);
    }

    #[test]
    fn step_data_model_job_gates() {
        // IDA 0x9c5bfc: no stats -> false without stepping; no job -> false; both -> step + true.
        let mut stepped = 0;
        assert!(!TouchJob::step_data_model_job(false, true, &mut || stepped += 1));
        assert!(!TouchJob::step_data_model_job(true, false, &mut || stepped += 1));
        assert!(TouchJob::step_data_model_job(true, true, &mut || stepped += 1));
        assert_eq!(stepped, 1);
    }
}
