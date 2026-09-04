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
/// `RBX::Network::CustomSerializer::writeNormQuat(bool,float const & x4,
/// RakNet::BitStream &)` (IDA 0x98ae20).
///
/// `heavy` arrives in the `this` slot (fastcall bool, IDA 0x98ae42). The
/// call site passes the quaternion w first (IDA 0x988b84..0x988ba2), so the
/// order is `(w, x, y, z)`: w keeps only its sign bit, x/y/z each get a
/// sign bit plus a biased magnitude — 8 bits scaled by 255 in heavy mode,
/// 16 bits scaled by 32767 otherwise. Magnitudes are truncated
/// (`vcvt.s32.f32`); the biases reuse [`SHORT_COMPONENT_BIAS`] /
/// [`FULL_COMPONENT_BIAS`].
pub fn write_norm_quat(heavy: bool, w: f32, x: f32, y: f32, z: f32, stream: &mut BitStream) {
    // IDA 0x98ae42..0x98ae4a: `this == 1 -> Write1 else Write0`.
    stream.write_bit(heavy);
    // IDA 0x98ae5c..0x98ae64: w sign only; w is rebuilt by sqrt on read.
    stream.write_bit(!(w >= 0.0));
    let (bias, scale) = if heavy {
        // IDA 0x98ae70..0x98ae78: bits 1132396544 == 255.0.
        (SHORT_COMPONENT_BIAS, 255.0f32)
    } else {
        // Full-mode scale bits 1191181824 == 32767.0.
        (FULL_COMPONENT_BIAS, 32767.0f32)
    };
    // IDA 0x98ae88..0x98afae: sign bit, then
    // `(int)((bias + min(1, |c|)) * scale)` in 8 or 16 bits.
    let mut component = |c: f32| {
        stream.write_bit(!(c >= 0.0));
        let q = ((bias + c.abs().min(1.0)) * scale) as u32;
        if heavy {
            stream.write_bits(q, 8);
        } else {
            stream.write_u16(q as u16);
        }
    };
    component(x);
    component(y);
    component(z);
}

/// `RBX::Network::CustomSerializer::readNormQuat(float &,float &,float &,
/// float &,RakNet::BitStream &)` (IDA 0x98b2a8).
///
/// The `this` receiver is the w out-param; the remaining outs are
/// `(x, y, z)`. Bit order is the heavy flag, the w sign, then per
/// component a sign bit and the biased magnitude (8 bits over 255.0 in
/// heavy mode, 16 bits over 32767.0 otherwise). Short reads leave that
/// out-param untouched and fall through to the final `return 1` (IDA
/// 0x98b342/0x98b3d2/0x98b460), so this has no failure return.
pub fn read_norm_quat(
    stream: &mut BitStream,
    w: &mut f32,
    x: &mut f32,
    y: &mut f32,
    z: &mut f32,
) {
    // IDA 0x98b2bc..0x98b2e0: heavy flag; IDA 0x98b2e6..0x98b308: w sign.
    let heavy = stream.read_bit().unwrap_or(false);
    let w_sign = stream.read_bit().unwrap_or(false);
    // Read biases are the exact negations of the write ones (heavy bits
    // -1157595007 == -SHORT_COMPONENT_BIAS, light bits -1224736640 ==
    // -FULL_COMPONENT_BIAS).
    let component = |stream: &mut BitStream, out: &mut f32| {
        // IDA 0x98b30c..0x98b328 / 0x98b39a..0x98b3b6 / 0x98b428..0x98b444.
        let sign = stream.read_bit().unwrap_or(false);
        if heavy {
            // IDA 0x98b332..0x98b382: `raw / 255.0 + bias`.
            if let Some(raw) = stream.read_bits(8) {
                *out = raw as f32 / 255.0 - SHORT_COMPONENT_BIAS;
            }
        // IDA 0x98b35c..0x98b382: `raw / 32767.0 + bias`.
        } else if let Some(raw) = stream.read_u16() {
            *out = f32::from(raw) / 32767.0 - FULL_COMPONENT_BIAS;
        }
        // IDA 0x98b38a..0x98b392 and twins.
        if sign {
            *out = -*out;
        }
    };
    component(stream, x);
    component(stream, y);
    component(stream, z);
    // IDA 0x98b4b8..0x98b4fa: w rebuilt from the unit constraint.
    let w_mag = (1.0 - *x * *x - *y * *y - *z * *z).max(0.0).sqrt();
    *w = if w_sign { -w_mag } else { w_mag };
}

/// `RBX::Network::serializeEnum` (IDA 0x95d5d0): the item's value index
/// goes out over `bits` bits. The null-item and range asserts stay
/// engine-side in release; debug builds check the range here.
pub fn serialize_enum(stream: &mut BitStream, value_index: u32, enum_count: u32, bits: u8) {
 debug_assert!(value_index < enum_count, "valueIndex < enumDesc->getEnumCount()");
 stream.write_bits(value_index, bits);
}

/// `RBX::Network::serializeEnumProperty` (IDA 0x95d968): same framing;
/// the value comes from the property getter engine-side.
pub fn serialize_enum_property(stream: &mut BitStream, value: u32, enum_count: u32, bits: u8) {
 serialize_enum(stream, value, enum_count, bits);
}

/// `RBX::Network::deserializeEnum` (IDA 0x95d694): reads the index and
/// converts it into the variant engine-side. Short reads panic with the
/// original message.
pub fn deserialize_enum(stream: &mut BitStream, enum_count: u32, bits: u8) -> u32 {
 let value = stream.read_bits(bits).expect("deserializeEnum failed");
 debug_assert!(value < enum_count, "value<(int)enumDesc->getEnumCount()");
 value
}

/// `RBX::Network::deserializeEnumProperty` (IDA 0x95da34): same framing;
/// the property setter runs engine-side.
pub fn deserialize_enum_property(stream: &mut BitStream, enum_count: u32, bits: u8) -> u32 {
 deserialize_enum(stream, enum_count, bits)
}

/// `RBX::operator<<(BitStream &, ContentId const&)` (IDA 0x95edd0) and
/// `serialize<ContentId>` (IDA 0x95fe40): both tail-call the string
/// writer.
pub fn write_content_id(stream: &mut BitStream, id: &str) {
 stream.write_string(id);
}

/// `RBX::operator>><ContentId>` (IDA 0x95ede0): reads the string and
/// fixes backslashes, then stamps the null name engine-side.
#[must_use]
pub fn read_content_id(stream: &mut BitStream) -> String {
 stream.read_string().replace('\\', "/")
}

/// `RBX::operator<<(BitStream &, BrickColor const&)` (IDA 0x95efcc): the
/// closest-palette index over 6 bits. The palette lookup stays
/// engine-side.
pub fn write_brick_color(stream: &mut BitStream, palette: u32) {
 stream.write_bits(palette, 6);
}

/// `RBX::operator>><BrickColor>` (IDA 0x95fc74): the 6-bit index; the
/// palette dereference runs engine-side.
#[must_use]
pub fn read_brick_color(stream: &mut BitStream) -> u32 {
 stream.read_bits(6).expect("BitStream >> BrickColor failed")
}

/// `RBX::operator<<(BitStream &, UDim const&)` (IDA 0x95eff0): scale as
/// `float`, then the offset truncated to 16 bits but sent as `int`.
/// `operator>><UDim>` (IDA 0x95fbb8) mirrors it.
pub fn write_udim(stream: &mut BitStream, scale: f32, offset: i32) {
 stream.write_f32(scale);
 stream.write_i32(offset as i16 as i32);
}

#[must_use]
pub fn read_udim(stream: &mut BitStream) -> (f32, i32) {
 let scale = stream.read_f32().expect("BitStream >> UDim scale failed");
 let offset = stream.read_i32().expect("BitStream >> UDim offset failed") as i16 as i32;
 (scale, offset)
}

/// `RBX::operator<<(BitStream &, UDim2 const&)` (IDA 0x95f014): two
/// UDims; `operator>><UDim2>` (IDA 0x95fbd8) mirrors it.
pub fn write_udim2(stream: &mut BitStream, x_scale: f32, x_offset: i32, y_scale: f32, y_offset: i32) {
 write_udim(stream, x_scale, x_offset);
 write_udim(stream, y_scale, y_offset);
}

#[must_use]
pub fn read_udim2(stream: &mut BitStream) -> (f32, i32, f32, i32) {
 let (xs, xo) = read_udim(stream);
 let (ys, yo) = read_udim(stream);
 (xs, xo, ys, yo)
}

/// `RBX::operator<<(BitStream &, RbxRay const&)` (IDA 0x95f050): origin
/// then direction, six floats; `operator>><RbxRay>` (IDA 0x95fc10)
/// mirrors it.
pub fn write_rbx_ray(stream: &mut BitStream, origin: [f32; 3], direction: [f32; 3]) {
 for c in origin {
 stream.write_f32(c);
 }
 for c in direction {
 stream.write_f32(c);
 }
}

#[must_use]
pub fn read_rbx_ray(stream: &mut BitStream) -> ([f32; 3], [f32; 3]) {
 let mut origin = [0.0; 3];
 let mut direction = [0.0; 3];
 for c in &mut origin {
 *c = stream.read_f32().expect("BitStream >> RbxRay failed");
 }
 for c in &mut direction {
 *c = stream.read_f32().expect("BitStream >> RbxRay failed");
 }
 (origin, direction)
}

/// `RBX::operator<<(BitStream &, Vector3 const&)` (IDA 0x95f0d8) and
/// `operator>><Vector3>` (IDA 0x95f7dc): three floats.
pub fn write_vector3(stream: &mut BitStream, v: [f32; 3]) {
 for c in v {
 stream.write_f32(c);
 }
}

#[must_use]
pub fn read_vector3(stream: &mut BitStream) -> [f32; 3] {
 [stream.read_f32().expect("BitStream >> Vector3 failed"), stream.read_f32().expect("BitStream >> Vector3 failed"), stream.read_f32().expect("BitStream >> Vector3 failed")]
}

/// `RBX::operator<<(BitStream &, Vector2 const&)` (IDA 0x95f664) and
/// `operator>><Vector2>` (IDA 0x95f69c): two floats.
pub fn write_vector2(stream: &mut BitStream, v: [f32; 2]) {
 for c in v {
 stream.write_f32(c);
 }
}

#[must_use]
pub fn read_vector2(stream: &mut BitStream) -> [f32; 2] {
 [stream.read_f32().expect("BitStream >> Vector2 failed"), stream.read_f32().expect("BitStream >> Vector2 failed")]
}

/// `RBX::operator<<(BitStream &, Color3 const&)` (IDA 0x95f144) and
/// `operator>><Color3>` (IDA 0x95fde0): three floats.
pub fn write_color3(stream: &mut BitStream, v: [f32; 3]) {
 write_vector3(stream, v);
}

#[must_use]
pub fn read_color3(stream: &mut BitStream) -> [f32; 3] {
 read_vector3(stream)
}

/// `RBX::operator<<(BitStream &, Faces const&)` (IDA 0x95f124) and
/// `operator<<(BitStream &, Axes const&)` (IDA 0x95f134): one `int`;
/// the `>>` twins (IDA 0x95fc54/0x95fc64) mirror them.
pub fn write_faces(stream: &mut BitStream, faces: i32) {
 stream.write_i32(faces);
}

#[must_use]
pub fn read_faces(stream: &mut BitStream) -> i32 {
 stream.read_i32().expect("BitStream >> Faces failed")
}

pub fn write_axes(stream: &mut BitStream, axes: i32) {
 stream.write_i32(axes);
}

#[must_use]
pub fn read_axes(stream: &mut BitStream) -> i32 {
 stream.read_i32().expect("BitStream >> Axes failed")
}

/// `RBX::operator<<(BitStream &, short)` (IDA 0x95f800) and
/// `operator>><short>` (IDA 0x95f818).
pub fn write_short(stream: &mut BitStream, value: i16) {
 stream.write_i16(value);
}

#[must_use]
pub fn read_short(stream: &mut BitStream) -> i16 {
 stream.read_i16().expect("BitStream >> short failed")
}

/// `RBX::operator<<(BitStream &, Vector3int16 const&)` (IDA 0x95f828)
/// and `operator<<(BitStream &, Vector2int16 const&)` (IDA 0x95f884);
/// the `>>` twins (IDA 0x95f864/0x95f8b0) mirror them.
pub fn write_vector3i16(stream: &mut BitStream, v: [i16; 3]) {
 for c in v {
 stream.write_i16(c);
 }
}

#[must_use]
pub fn read_vector3i16(stream: &mut BitStream) -> [i16; 3] {
 [stream.read_i16().expect("BitStream >> Vector3int16 failed"), stream.read_i16().expect("BitStream >> Vector3int16 failed"), stream.read_i16().expect("BitStream >> Vector3int16 failed")]
}

pub fn write_vector2i16(stream: &mut BitStream, v: [i16; 2]) {
 for c in v {
 stream.write_i16(c);
 }
}

#[must_use]
pub fn read_vector2i16(stream: &mut BitStream) -> [i16; 2] {
 [stream.read_i16().expect("BitStream >> Vector2int16 failed"), stream.read_i16().expect("BitStream >> Vector2int16 failed")]
}

/// `RBX::operator<<(BitStream &, StreamRegion2Id const&)` (IDA 0x95f6b4):
/// three `int`s, or one flag bit plus three bytes when every component
/// fits in `[-128, 127]`; `operator>><StreamRegion2Id>` (IDA 0x95f750)
/// mirrors it.
pub fn write_region2_id(stream: &mut BitStream, v: [i32; 3]) {
 if v.iter().all(|&c| (-128..=127).contains(&c)) {
 stream.write_bit(false);
 for c in v {
 stream.write_bits(c as u32 & 0xFF, 8);
 }
 } else {
 stream.write_bit(true);
 for c in v {
 stream.write_i32(c);
 }
 }
}

#[must_use]
pub fn read_region2_id(stream: &mut BitStream) -> [i32; 3] {
 if stream.read_bit().expect("BitStream >> StreamRegion2Id flag failed") {
 [stream.read_i32().expect("BitStream >> StreamRegion2Id failed"), stream.read_i32().expect("BitStream >> StreamRegion2Id failed"), stream.read_i32().expect("BitStream >> StreamRegion2Id failed")]
 } else {
 let mut out = [0; 3];
 for c in &mut out {
 *c = stream.read_bits(8).expect("BitStream >> StreamRegion2Id failed") as i8 as i32;
 }
 out
 }
}

/// `RBX::operator<<(BitStream &, SystemAddress)` (IDA 0x95fe04): the
/// binary address then the port; `operator>><SystemAddress>`
/// (IDA 0x95fe28) mirrors it.
pub fn write_system_address(stream: &mut BitStream, address: u32, port: u16) {
 stream.write_u32(address);
 stream.write_u16(port);
}

#[must_use]
pub fn read_system_address(stream: &mut BitStream) -> (u32, u16) {
 (stream.read_u32().expect("BitStream >> SystemAddress failed"), stream.read_u16().expect("BitStream >> SystemAddress failed"))
}

/// `RBX::Network::writeBrickVector` quantization gate (IDA 0x95f168):
/// `|x|,|z| < 512`, `0 <= y < 204.8`, `2x`/`2z` integral, and `10y`
/// within `0.0005` of integral.
#[must_use]
pub fn brick_vector_quantized(v: [f32; 3]) -> Option<[i16; 3]> {
 let [x, y, z] = v;
 if x.abs() >= 512.0 || z.abs() >= 512.0 || y < 0.0 || y >= 204.8 {
 return None;
 }
 let sx = 2.0 * x;
 let sz = 2.0 * z;
 let sy = 10.0 * y;
 if sx as i32 as f32 != sx || sz as i32 as f32 != sz {
 return None;
 }
 let qy = sy as i32;
 if qy as f32 != sy && (qy as f32 - sy).abs() > 0.0005 {
 return None;
 }
 Some([sx as i32 as i16, qy as i16, sz as i32 as i16])
}

/// `RBX::Network::writeBrickVector` (IDA 0x95f168): one flag bit, then
/// either three 11-bit quantized components or three floats.
pub fn write_brick_vector(stream: &mut BitStream, v: [f32; 3]) {
 if let Some([qx, qy, qz]) = brick_vector_quantized(v) {
 stream.write_bit(true);
 stream.write_bits(qx as u32 & 0x7FF, 11);
 stream.write_bits(qy as u32 & 0x7FF, 11);
 stream.write_bits(qz as u32 & 0x7FF, 11);
 } else {
 stream.write_bit(false);
 write_vector3(stream, v);
 }
}

/// `RBX::Network::readBrickVector` (IDA 0x95f2cc): inverts
/// [`write_brick_vector`]; short reads panic with the original messages.
#[must_use]
pub fn read_brick_vector(stream: &mut BitStream) -> [f32; 3] {
 if stream.read_bit().expect("readBrickVector flag failed") {
 let sign_extend_11 = |raw: u32| ((raw << 21) as i32 >> 21) as f32;
 let qx = stream.read_bits(11).expect("readBrickVector x failed");
 let qy = stream.read_bits(11).expect("readBrickVector y failed");
 let qz = stream.read_bits(11).expect("readBrickVector z failed");
 // IDA 0x95f386/0x95f3a4: x/z sign-extend from bit 10, y is unsigned.
 [sign_extend_11(qx) * 0.5, qy as f32 / 10.0, sign_extend_11(qz) * 0.5]
 } else {
 read_vector3(stream)
 }
}

/// `RBX::operator<<(BitStream &, CoordinateFrame const&)` translation
/// clamp (IDA 0x95f8ea..0x95f948): finite components below `-1e6` become
/// `-1e6`; a non-finite translation becomes `(-1e6, 0, 0)`.
#[must_use]
pub fn clamp_brick_translation(v: [f32; 3]) -> [f32; 3] {
 if v.iter().all(|c| c.is_finite()) {
 v.map(|c| if c < -1_000_000.0 { -1_000_000.0 } else { c })
 } else {
 [-1_000_000.0, 0.0, 0.0]
 }
}

/// `RBX::operator<<(BitStream &, CoordinateFrame const&)` (IDA 0x95f8c8):
/// the clamped translation via [`write_brick_vector`], then one flag bit
/// plus the 6-bit orient id when axis-aligned (the id stays engine-side),
/// else four `float16` quaternion components over `[-1, 1]`.
/// `operator>><CoordinateFrame>` (IDA 0x95f9d0) mirrors the framing;
/// matrix rebuild stays engine-side.
pub fn write_coordinate_frame(stream: &mut BitStream, translation: [f32; 3], orient: Option<u32>, quat: [f32; 4]) {
 write_brick_vector(stream, clamp_brick_translation(translation));
 if let Some(id) = orient {
 stream.write_bit(true);
 stream.write_bits(id, 6);
 } else {
 stream.write_bit(false);
 for c in quat {
 stream.write_float16(c, -1.0, 1.0);
 }
 }
}

#[must_use]
pub fn read_coordinate_frame(stream: &mut BitStream) -> ([f32; 3], Option<u32>, [f32; 4]) {
 let translation = read_brick_vector(stream);
 if stream.read_bit().expect("BitStream >> CoordinateFrame flag failed") {
 let id = stream.read_bits(6).expect("BitStream >> CoordinateFrame orientId failed");
 (translation, Some(id), [0.0, 0.0, 0.0, 1.0])
 } else {
 let mut quat = [0.0; 4];
 for c in &mut quat {
 *c = stream.read_float16(-1.0, 1.0).expect("BitStream >> CoordinateFrame quat failed");
 }
 (translation, None, quat)
 }
}

/// `Compressor::writeCompressed` (IDA 0x989738): gzip-compresses the
/// bytes (boost::iostreams stays engine-side), then writes the
/// compressed length as `uint` followed by the raw body.
pub fn write_compressed(stream: &mut BitStream, data: &[u8], compress: &mut dyn FnMut(&[u8]) -> Vec<u8>) {
 let out = compress(data);
 stream.write_u32(out.len() as u32);
 for b in &out {
 stream.write_u8(*b);
 }
}

/// `Compressor::readCompressed` (IDA 0x98a0e0): inverts
/// [`write_compressed`]; short reads panic with original-flavored
/// messages.
#[must_use]
pub fn read_compressed(stream: &mut BitStream, decompress: &mut dyn FnMut(&[u8]) -> Vec<u8>) -> Vec<u8> {
 let len = stream.read_u32().expect("Compressor::readCompressed failed reading length");
 let mut buf = vec![0u8; len as usize];
 for b in &mut buf {
 *b = stream.read_u8().expect("Compressor::readCompressed failed reading body");
 }
 decompress(&buf)
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
    #[test]
    fn norm_quat_heavy_roundtrip() {
        let mut s = BitStream::new();
        write_norm_quat(true, 0.9, 0.3, -0.2, 0.25, &mut s);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let (mut w, mut x, mut y, mut z) = (0.0, 0.0, 0.0, 0.0);
        read_norm_quat(&mut r, &mut w, &mut x, &mut y, &mut z);
        // 8-bit grid over the biased range plus the bias roundtrip.
        assert!((x - 0.3).abs() < 0.01, "x={x}");
        assert!((y + 0.2).abs() < 0.01, "y={y}");
        assert!((z - 0.25).abs() < 0.01, "z={z}");
        assert!(w > 0.0, "w={w}");
    }

    #[test]
    fn norm_quat_light_roundtrip_negative_w() {
        let mut s = BitStream::new();
        write_norm_quat(false, -0.8, 0.4, 0.3, -0.316_227_77, &mut s);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let (mut w, mut x, mut y, mut z) = (1.0, 0.0, 0.0, 0.0);
        read_norm_quat(&mut r, &mut w, &mut x, &mut y, &mut z);
        assert!((x - 0.4).abs() < 0.005, "x={x}");
        assert!((y - 0.3).abs() < 0.005, "y={y}");
        assert!((z + 0.316_227_77).abs() < 0.005, "z={z}");
        assert!(w < 0.0, "w={w}");
    }

    #[test]
    fn norm_quat_short_read_keeps_outs() {
        // Empty stream: every read misses, outs untouched except the
        // rebuilt w (sqrt of the stale constraint), final `return 1`.
        let mut r = BitStream::from_bytes(&[]);
        let (mut w, mut x, mut y, mut z) = (1.0, 0.0, 0.0, 0.0);
        read_norm_quat(&mut r, &mut w, &mut x, &mut y, &mut z);
        assert_eq!((x, y, z), (0.0, 0.0, 0.0));
        assert_eq!(w, 1.0);
    }
    #[test]
    fn scalar_codecs_roundtrip() {
        let mut s = BitStream::new();
        serialize_enum(&mut s, 2, 3, 2);
        write_content_id(&mut s, "rbxasset://x");
        write_brick_color(&mut s, 41);
        write_udim(&mut s, 0.5, -7);
        write_udim2(&mut s, 0.5, -7, 1.0, 9);
        write_faces(&mut s, 63);
        write_axes(&mut s, 7);
        write_short(&mut s, -300);
        write_system_address(&mut s, 0x7F00_0001, 53640);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(deserialize_enum(&mut r, 3, 2), 2);
        assert_eq!(r.read_string(), "rbxasset://x");
        assert_eq!(read_brick_color(&mut r), 41);
        assert_eq!(read_udim(&mut r), (0.5, -7));
        assert_eq!(read_udim2(&mut r), (0.5, -7, 1.0, 9));
        assert_eq!(read_faces(&mut r), 63);
        assert_eq!(read_axes(&mut r), 7);
        assert_eq!(read_short(&mut r), -300);
        assert_eq!(read_system_address(&mut r), (0x7F00_0001, 53640));
    }

    #[test]
    fn vector_codecs_roundtrip() {
        let mut s = BitStream::new();
        write_vector3(&mut s, [1.0, -2.0, 3.0]);
        write_vector2(&mut s, [0.5, -0.5]);
        write_color3(&mut s, [1.0, 0.0, 0.5]);
        write_rbx_ray(&mut s, [1.0, 2.0, 3.0], [0.0, 1.0, 0.0]);
        write_vector3i16(&mut s, [-1, 2, -3]);
        write_vector2i16(&mut s, [4, -5]);
        write_region2_id(&mut s, [-5, 6, 127]);
        write_region2_id(&mut s, [1000, -2000, 3000]);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(read_vector3(&mut r), [1.0, -2.0, 3.0]);
        assert_eq!(read_vector2(&mut r), [0.5, -0.5]);
        assert_eq!(read_color3(&mut r), [1.0, 0.0, 0.5]);
        assert_eq!(read_rbx_ray(&mut r), ([1.0, 2.0, 3.0], [0.0, 1.0, 0.0]));
        assert_eq!(read_vector3i16(&mut r), [-1, 2, -3]);
        assert_eq!(read_vector2i16(&mut r), [4, -5]);
        assert_eq!(read_region2_id(&mut r), [-5, 6, 127]);
        assert_eq!(read_region2_id(&mut r), [1000, -2000, 3000]);
    }

    #[test]
    fn brick_vector_quantized_prefers_bits() {
        // IDA 0x95f168: small integral-half vectors take the 11-bit path.
        assert_eq!(brick_vector_quantized([1.0, 2.0, -3.0]), Some([2, 20, -6]));
        assert_eq!(brick_vector_quantized([600.0, 2.0, 0.0]), None);
        assert_eq!(brick_vector_quantized([1.0, -1.0, 0.0]), None);
        assert_eq!(brick_vector_quantized([0.3, 0.0, 0.0]), None);
        let mut s = BitStream::new();
        write_brick_vector(&mut s, [1.0, 2.0, -3.0]);
        write_brick_vector(&mut s, [600.0, 2.0, 0.0]);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(read_brick_vector(&mut r), [1.0, 2.0, -3.0]);
        assert_eq!(read_brick_vector(&mut r), [600.0, 2.0, 0.0]);
    }

    #[test]
    fn coordinate_frame_clamp_and_paths() {
        // IDA 0x95f8ea: low-side clamp, non-finite reset.
        assert_eq!(clamp_brick_translation([-2e6, 1.0, 2.0]), [-1e6, 1.0, 2.0]);
        assert_eq!(clamp_brick_translation([f32::NAN, 0.0, 0.0]), [-1e6, 0.0, 0.0]);
        let mut s = BitStream::new();
        write_coordinate_frame(&mut s, [1.0, 2.0, 3.0], Some(9), [0.0, 0.0, 0.0, 1.0]);
        write_coordinate_frame(&mut s, [1.0, 2.0, 3.0], None, [0.0, 0.0, 0.0, 1.0]);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let (t, orient, _) = read_coordinate_frame(&mut r);
        assert_eq!((t, orient), ([1.0, 2.0, 3.0], Some(9)));
        let (t, orient, q) = read_coordinate_frame(&mut r);
        assert_eq!(orient, None);
        assert_eq!(t, [1.0, 2.0, 3.0]);
        assert!((q[3] - 1.0).abs() < 0.001, "q={q:?}");
    }

    #[test]
    fn content_id_backslash_fixed() {
        // IDA 0x95ede0: backslashes become forward slashes.
        let mut s = BitStream::new();
        write_content_id(&mut s, "a\\b");
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(read_content_id(&mut r), "a/b");
    }
    #[test]
    fn compressed_framing_roundtrip() {
        // IDA 0x989738/0x98a0e0: length-prefixed body; the codec itself
        // is engine-side (identity stands in here).
        let mut s = BitStream::new();
        write_compressed(&mut s, b"hello", &mut |d| d.to_vec());
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(read_compressed(&mut r, &mut |d| d.to_vec()), b"hello");
    }
}
