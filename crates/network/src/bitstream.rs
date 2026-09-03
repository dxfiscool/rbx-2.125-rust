//! `RakNet::BitStream` — minimal bit cursor backing the network physics codecs.
//!
//! Wire order is MSB-first inside each byte: stream bit `k` lives at
//! `byte[k >> 3] & (0x80 >> (k & 7))` (IDA 0x9bee0a). The `Write<T>` /
//! `Read<T>` scalar templates normalize multi-byte values to big-endian on
//! the wire (`ReverseBytes` + `WriteBits`, IDA 0x962a24; the
//! `IsNetworkOrder` gate, IDA 0xa55f48, returns 0, so the reverse always
//! runs on little-endian hosts). Raw `WriteBits` callers (guid index bits,
//! code bytes) and the direct byte writers (`uint24_t`, strings) keep host
//! memory order instead. Short reads never advance the cursor, mirroring
//! `ReadBits` bounds checks (returns `false` without consuming).

#![allow(dead_code)]

/// Maximum string length accepted by the `RBX::operator<</>>` string helpers
/// (IDA 0x95ea28, 0x95eb72: `0x30D41`).
pub const MAX_STRING_LEN: usize = 0x30D41;

/// Bit-level stream with independent read/write cursors.
#[derive(Clone, Debug, Default)]
pub struct BitStream {
    bytes: Vec<u8>,
    read_bit: usize,
    written_bits: usize,
}

impl BitStream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            read_bit: 0,
            written_bits: bytes.len() * 8,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn bits_written(&self) -> usize {
        self.written_bits
    }

    pub fn bits_remaining(&self) -> usize {
        self.written_bits.saturating_sub(self.read_bit)
    }
    fn reserve_bit(&mut self) {
        let idx = self.written_bits >> 3;
        if idx >= self.bytes.len() {
            self.bytes.push(0);
        }
    }

    /// `RakNet::BitStream::Write0` / `Write1`.
    pub fn write_bit(&mut self, bit: bool) {
        self.reserve_bit();
        // IDA 0x9bee0a: mask is `0x80 >> (pos & 7)`, i.e. MSB-first.
        let idx = self.written_bits >> 3;
        let mask = 0x80 >> (self.written_bits & 7);
        if bit {
            self.bytes[idx] |= mask;
        } else {
            self.bytes[idx] &= !mask;
        }
        self.written_bits += 1;
    }

    /// `RakNet::BitStream::ReadBit` — returns `false` past the end.
    pub fn read_bit(&mut self) -> Option<bool> {
        if self.read_bit >= self.written_bits {
            return None;
        }
        let byte = self.bytes[self.read_bit >> 3];
        let bit = byte & (0x80 >> (self.read_bit & 7)) != 0;
        self.read_bit += 1;
        Some(bit)
    }

    /// `RakNet::BitStream::WriteBits` — low `count` bits of `value`, MSB first.
    pub fn write_bits(&mut self, value: u32, count: u8) {
        debug_assert!(count <= 32);
        for i in (0..count).rev() {
            self.write_bit(value >> i & 1 != 0);
        }
    }

    /// `RakNet::BitStream::ReadBits` — assembled MSB first; `None` (without
    /// consuming) when fewer than `count` bits remain.
    pub fn read_bits(&mut self, count: u8) -> Option<u32> {
        debug_assert!(count <= 32);
        if self.bits_remaining() < count as usize {
            return None;
        }
        let mut value = 0u32;
        for _ in 0..count {
            value = value << 1 | self.read_bit().unwrap_or(false) as u32;
        }
        Some(value)
    }

    pub fn write_u8(&mut self, value: u8) {
        self.write_bits(value as u32, 8);
    }
    pub fn read_u8(&mut self) -> Option<u8> {
        self.read_bits(8).map(|v| v as u8)
    }

    /// `Write<unsigned short>` / `Read<unsigned short>` (IDA 0x98a7e0
    /// magnitudes): big-endian on the wire (`ReverseBytes` + `WriteBits`,
    /// IDA 0x962a24 pattern; `IsNetworkOrder` is 0, IDA 0xa55f48).
    pub fn write_u16(&mut self, value: u16) {
        for byte in value.to_be_bytes() {
            self.write_u8(byte);
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let hi = self.read_u8()? as u16;
        let lo = self.read_u8()? as u16;
        Some(hi << 8 | lo)
    }

    /// `Write<short>` / `Read<short>` (IDA 0x96381c / 0x963930, armv7
    /// `short` is 2 bytes): same bytes as `u16` (two's complement).
    pub fn write_i16(&mut self, value: i16) {
        self.write_u16(value as u16);
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        self.read_u16().map(|v| v as i16)
    }

    /// `Write<unsigned int>` / `Read<unsigned int>` (IDA 0x963120 /
    /// 0x963234): big-endian, same `ReverseBytes` + `WriteBits` shape as
    /// `Write<int>` (IDA 0x962a24).
    pub fn write_u32(&mut self, value: u32) {
        for byte in value.to_be_bytes() {
            self.write_u8(byte);
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let mut out = 0u32;
        for _ in 0..4 {
            out = out << 8 | self.read_u8()? as u32;
        }
        Some(out)
    }

    /// `Write<int>` / `Read<int>` (IDA 0x962a24 / 0x962ff8) and
    /// `Write<long>` / `Read<long>` (armv7 `long` is 4 bytes): the template
    /// reverses the little-endian host bytes (`ReverseBytes`, IDA 0xa55f4c)
    /// and emits 32 bits MSB-first — i.e. big-endian.
    pub fn write_i32(&mut self, value: i32) {
        self.write_u32(value as u32);
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        self.read_u32().map(|v| v as i32)
    }

    /// `Write<unsigned long long>` / `Read<unsigned long long>`
    /// (IDA 0x962c60 / 0x962d98): big-endian 8 bytes.
    pub fn write_u64(&mut self, value: u64) {
        for byte in value.to_be_bytes() {
            self.write_u8(byte);
        }
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        let mut out = 0u64;
        for _ in 0..8 {
            out = out << 8 | self.read_u8()? as u64;
        }
        Some(out)
    }

    pub fn write_i64(&mut self, value: i64) {
        self.write_u64(value as u64);
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        self.read_u64().map(|v| v as i64)
    }

    /// `Write<float>` / `Read<float>`: the `f32` bits through the `u32`
    /// template shape (big-endian).
    pub fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        self.read_u32().map(f32::from_bits)
    }

    /// `Write<double>` / `Read<double>` (IDA 0x963598 / 0x9636d0):
    /// big-endian `f64` bits.
    pub fn write_f64(&mut self, value: f64) {
        self.write_u64(value.to_bits());
    }

    pub fn read_f64(&mut self) -> Option<f64> {
        self.read_u64().map(f64::from_bits)
    }

    /// `RakNet::BitStream::IsNetworkOrder` (IDA 0xa55f48): the internal
    /// check returns 0, so every `Write<T>`/`Read<T>` template takes the
    /// `ReverseBytes` arm on little-endian hosts.
    pub fn is_network_order() -> bool {
        false
    }

    /// Advance the write cursor to the next byte boundary, zero-filling
    /// (IDA 0xa77d60: `*this += ((u8)*this + 7) & 7 ^ 7`).
    ///
    /// FIDELITY: the original just advances over whatever the allocator
    /// left there; zeros are the observable contents of fresh buffers.
    fn align_write_up(&mut self) {
        while self.written_bits & 7 != 0 {
            self.write_bit(false);
        }
    }

    /// Advance the read cursor to the next byte boundary (IDA 0xa77ea4).
    /// Like the original, the alignment is consumed even when the following
    /// bounds check fails.
    fn align_read_up(&mut self) {
        let pad = (8 - (self.read_bit & 7)) & 7;
        self.read_bit = self.read_bit.saturating_add(pad).min(self.written_bits);
    }

    /// `Write<RakNet::uint24_t>` (IDA 0xa77d60): align-up, then the low 3
    /// bytes in host (little-endian) order — no reversal on this path.
    pub fn write_uint24(&mut self, value: u32) {
        self.align_write_up();
        for byte in value.to_le_bytes()[..3].iter() {
            self.write_u8(*byte);
        }
    }

    /// `Read<RakNet::uint24_t>` (IDA 0xa77ea4): align-up (consumed even on
    /// failure), bounds-check 24 bits, then 3 bytes little-endian.
    pub fn read_uint24(&mut self) -> Option<u32> {
        self.align_read_up();
        if self.bits_remaining() < 24 {
            return None;
        }
        let b0 = self.read_u8()?;
        let b1 = self.read_u8()?;
        let b2 = self.read_u8()?;
        Some(u32::from_le_bytes([b0, b1, b2, 0]))
    }

    /// `WriteCompressed<unsigned int>` template (IDA 0xa7b9b4): `ReverseBytes`
    /// to big-endian, then the core below. `WriteCompressed<unsigned long
    /// long>` shares the core with a wider buffer.
    pub fn write_compressed_u32(&mut self, value: u32) {
        Self::write_compressed_bytes(self, &value.to_be_bytes());
    }

    /// Core `WriteCompressed(src, nbits, rightAligned = 1)` (IDA 0xa55c9c)
    /// over an already-normalized big-endian buffer: one `Write1` per
    /// trailing zero byte (at most `len - 1`), then `Write0` and the
    /// remaining head bytes — except a lone trailing byte under `0x10`,
    /// which is `Write1` plus its low 4 bits.
    fn write_compressed_bytes(stream: &mut BitStream, bytes: &[u8]) {
        let mut len = bytes.len();
        while len > 1 && bytes[len - 1] == 0 {
            stream.write_bit(true);
            len -= 1;
        }
        let head = &bytes[..len];
        if len == 1 && head[0] < 0x10 {
            stream.write_bit(true);
            stream.write_bits(u32::from(head[0]), 4);
        } else {
            stream.write_bit(false);
            for &b in head {
                stream.write_u8(b);
            }
        }
    }

    /// `ReadCompressed<unsigned int>` template (IDA 0xa7bac8): core below,
    /// then `ReverseBytes` back to host order.
    ///
    /// FIDELITY: the original consumes the flag bit before failing a short
    /// body read; this consumes nothing on failure, like [`BitStream::read_bits`].
    pub fn read_compressed_u32(&mut self) -> Option<u32> {
        let mut buf = [0u8; 4];
        let mut len = 4usize;
        loop {
            // IDA 0xa55d2c: single-byte tail — `Read1` selects 4 vs 8 bits.
            if len == 1 {
                if self.read_bit()? {
                    buf[0] = self.read_bits(4)? as u8;
                } else {
                    buf[0] = self.read_u8()?;
                }
                break;
            }
            // IDA 0xa55d2c: `Read1` elides a trailing zero byte, `Read0`
            // ends the prefix and the head bytes follow.
            if !self.read_bit()? {
                for b in buf.iter_mut().take(len) {
                    *b = self.read_u8()?;
                }
                break;
            }
            len -= 1;
        }
        Some(u32::from_be_bytes(buf))
    }

    /// `RakNet::BitStream::WriteVector<float>` / `ReadVector<float>`.
    pub fn write_vector3(&mut self, v: [f32; 3]) {
        for c in v {
            self.write_f32(c);
        }
    }

    pub fn read_vector3(&mut self) -> Option<[f32; 3]> {
        Some([self.read_f32()?, self.read_f32()?, self.read_f32()?])
    }
    /// `RakNet::BitStream::WriteNormQuat<float>` (IDA 0x98a7e0): one sign
    /// bit each for w, x, y, z, then `u16(|x| * 65535)` magnitudes for x, y
    /// and z; w is rebuilt from the unit constraint on read.
    pub fn write_norm_quat(&mut self, w: f32, x: f32, y: f32, z: f32) {
        // IDA 0x98a812..0x98a85c: `>= 0.0 -> Write0 else Write1`, spelled as
        // `!(v >= 0.0)` so NaN takes the Write1 arm exactly like the original.
        self.write_bit(!(w >= 0.0));
        self.write_bit(!(x >= 0.0));
        self.write_bit(!(y >= 0.0));
        self.write_bit(!(z >= 0.0));
        // IDA 0x98a87a..0x98a8d2: `(int)(fabs(c) * 65535.0)`, truncated.
        self.write_u16((x.abs() * 65535.0) as u16);
        self.write_u16((y.abs() * 65535.0) as u16);
        self.write_u16((z.abs() * 65535.0) as u16);
    }

    /// `RakNet::BitStream::ReadNormQuat<float>` (IDA 0x98b0e8). Short sign
    /// reads behave as clear bits without consuming; a short magnitude read
    /// fails the call without touching the outs. Returns `[w, x, y, z]`.
    pub fn read_norm_quat(&mut self) -> Option<[f32; 4]> {
        // IDA 0x98b0f6..0x98b196: four sign bits, defaulting to clear.
        let w_sign = self.read_bit().unwrap_or(false);
        let x_sign = self.read_bit().unwrap_or(false);
        let y_sign = self.read_bit().unwrap_or(false);
        let z_sign = self.read_bit().unwrap_or(false);
        // IDA 0x98b1a0..0x98b1bc: a `Read<unsigned short>` failure returns 0.
        // Magnitudes are double-divided by 65535 before narrowing (IDA
        // 0x98b1d8..0x98b208).
        let x = (f64::from(self.read_u16()?) / 65535.0) as f32;
        let y = (f64::from(self.read_u16()?) / 65535.0) as f32;
        let z = (f64::from(self.read_u16()?) / 65535.0) as f32;
        // IDA 0x98b214..0x98b24a: apply the sign bits.
        let x = if x_sign { -x } else { x };
        let y = if y_sign { -y } else { y };
        let z = if z_sign { -z } else { z };
        // IDA 0x98b250..0x98b292: `w = sqrt(max(0, 1-x^2-y^2-z^2))`,
        // negated by the first sign bit.
        let w_mag = (1.0 - x * x - y * y - z * z).max(0.0).sqrt();
        let w = if w_sign { -w_mag } else { w_mag };
        Some([w, x, y, z])
    }

    /// `RBX::operator<<(RakNet::BitStream &,std::string const&)` framing
    /// (IDA 0x95e9f4): big-endian u32 length (`Write<unsigned long>`
    /// template) + payload, rejecting `len >= 0x30D41`.
    ///
    /// FIDELITY: the original Huffman-codes the payload through
    /// `RakNet::StringCompressor::EncodeString` (IDA 0x95ea74); this keeps the
    /// exact framing and length limit but stores raw bytes until the fixed
    /// English Huffman table is ported.
    pub fn write_string(&mut self, s: &str) {
        let len = s.len() as u32;
        if len as usize >= MAX_STRING_LEN {
            panic!("BitStream string write: String too long: {len}");
        }
        self.write_u32(len);
        for byte in s.bytes() {
            self.write_u8(byte);
        }
    }

    /// `RBX::operator>>(RakNet::BitStream &,std::string &)` framing
    /// (IDA 0x95eb04). Panics mirror the original `std::runtime_error` throws.
    pub fn read_string(&mut self) -> String {
        let len = self
            .read_u32()
            .expect("BitStream >> std::string: failed to read length");
        if len as usize >= MAX_STRING_LEN {
            panic!("BitStream >> std::string: Bad string length");
        }
        let mut out = Vec::with_capacity(len as usize);
        for _ in 0..len {
            out.push(self.read_u8().expect("BitStream >> std::string: truncated"));
        }
        String::from_utf8(out).expect("BitStream >> std::string: bad utf8")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bits_roundtrip_msb_first() {
        let mut s = BitStream::new();
        s.write_bits(0b101, 3);
        s.write_bits(0x1FFF, 13);
        // 3 + 13 bits: `101` then thirteen ones, MSB-first.
        assert_eq!(s.into_bytes(), vec![0b1011_1111, 0b1111_1111]);
    }

    #[test]
    fn short_read_consumes_nothing() {
        let mut s = BitStream::from_bytes(&[0xFF]);
        assert!(s.read_bits(9).is_none());
        assert_eq!(s.bits_remaining(), 8);
    }

    #[test]
    fn norm_quat_roundtrip_unit() {
        // Identity-ish quaternion with positive w.
        let mut s = BitStream::new();
        s.write_norm_quat(0.968_245_8, 0.144_337_57, 0.144_337_57, 0.144_337_57);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let [w, x, y, z] = r.read_norm_quat().expect("norm quat");
        // u16 magnitude grid: each of x, y, z lands within one step.
        assert!((x - 0.144_337_57).abs() < 1.0 / 65535.0 + 1e-6);
        assert!((y - 0.144_337_57).abs() < 1.0 / 65535.0 + 1e-6);
        assert!((z - 0.144_337_57).abs() < 1.0 / 65535.0 + 1e-6);
        // w is rebuilt from the unit constraint, non-negative here.
        assert!((w - (1.0 - x * x - y * y - z * z).max(0.0).sqrt()).abs() < 1e-6);
        assert!(w >= 0.0);
    }

    #[test]
    fn norm_quat_sign_bits_survive() {
        let mut s = BitStream::new();
        s.write_norm_quat(0.5, -0.5, -0.5, 0.5);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let [w, x, y, z] = r.read_norm_quat().expect("norm quat");
        assert!(x < 0.0 && y < 0.0 && z > 0.0 && w > 0.0);
    }

    #[test]
    fn norm_quat_short_magnitude_fails_clean() {
        // Four sign bits plus one u16: drop the last magnitude byte.
        let mut s = BitStream::new();
        s.write_norm_quat(1.0, 0.0, 0.0, 0.0);
        let mut bytes = s.into_bytes();
        bytes.truncate(bytes.len() - 1);
        let mut r = BitStream::from_bytes(&bytes);
        assert!(r.read_norm_quat().is_none());
    }

    #[test]
    fn scalars_are_big_endian_on_the_wire() {
        // IDA 0x962a24: `ReverseBytes` + `WriteBits` — the template
        // normalizes to network order (`IsNetworkOrder` is 0, 0xa55f48).
        assert!(!BitStream::is_network_order());
        let mut s = BitStream::new();
        s.write_i32(0x01020304);
        s.write_u16(0x0506);
        s.write_i16(-2);
        s.write_u64(0x0102030405060708);
        s.write_f64(1.0);
        assert_eq!(
            s.into_bytes(),
            vec![
                0x01, 0x02, 0x03, 0x04, //
                0x05, 0x06, //
                0xFF, 0xFE, //
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, //
                0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            ]
        );
        let mut r = BitStream::from_bytes(&[
            0x01, 0x02, 0x03, 0x04, //
            0x05, 0x06, //
            0xFF, 0xFE, //
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, //
            0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
        ]);
        assert_eq!(r.read_i32(), Some(0x01020304));
        assert_eq!(r.read_u16(), Some(0x0506));
        assert_eq!(r.read_i16(), Some(-2));
        assert_eq!(r.read_u64(), Some(0x0102030405060708));
        assert_eq!(r.read_f64(), Some(1.0));
    }

    #[test]
    fn uint24_is_raw_le_triple_with_align() {
        // IDA 0xa77d60/0xa77ea4: align-up, then 3 host-order bytes.
        let mut s = BitStream::new();
        s.write_bit(true);
        s.write_uint24(0x123456);
        // One flag bit, seven zero pad bits, then the LE triple.
        assert_eq!(s.into_bytes(), vec![0b1000_0000, 0x56, 0x34, 0x12]);
        let mut r = BitStream::from_bytes(&[0b1000_0000, 0x56, 0x34, 0x12]);
        assert_eq!(r.read_bit(), Some(true));
        assert_eq!(r.read_uint24(), Some(0x123456));
        // Short triple fails (alignment still consumed, IDA 0xa77ea4).
        let mut r = BitStream::from_bytes(&[0xFF, 0x56]);
        assert!(r.read_uint24().is_none());
    }

    #[test]
    fn compressed_u32_roundtrip() {
        // IDA 0xa7b9b4/0xa7bac8 via the 0xa55c9c/0xa55d2c cores.
        for value in [0, 1, 5, 0x0F, 0x10, 0xAB, 0x100, 0x12345600, 0x12345678, u32::MAX] {
            let mut s = BitStream::new();
            s.write_compressed_u32(value);
            let mut r = BitStream::from_bytes(&s.into_bytes());
            assert_eq!(r.read_compressed_u32(), Some(value), "value {value:#x}");
        }
        // Zero elides three bytes then takes the 4-bit tail (IDA LABEL_5):
        // `111` + `1` + `0000` = 8 bits total.
        let mut s = BitStream::new();
        s.write_compressed_u32(0);
        assert_eq!(s.into_bytes(), vec![0b1111_0000]);
        // A full 32-bit value costs one flag bit plus 32 bits.
        let mut s = BitStream::new();
        s.write_compressed_u32(0x12345678);
        assert_eq!(s.bits_written(), 33);
    }
}
