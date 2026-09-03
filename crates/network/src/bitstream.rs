//! `RakNet::BitStream` — minimal bit cursor backing the network physics codecs.
//!
//! Wire order is MSB-first inside each byte: stream bit `k` lives at
//! `byte[k >> 3] & (0x80 >> (k & 7))` (IDA 0x9bee0a). Multi-byte scalars are
//! stored little-endian, matching `BitStream::Write<T>` memcpy semantics on
//! ARM. Short reads never advance the cursor, mirroring `ReadBits` bounds
//! checks (returns `false` without consuming).

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

    pub fn write_u16(&mut self, value: u16) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let lo = self.read_u8()? as u16;
        let hi = self.read_u8()? as u16;
        Some(lo | hi << 8)
    }

    pub fn write_u32(&mut self, value: u32) {
        for byte in value.to_le_bytes() {
            self.write_u8(byte);
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let mut out = 0u32;
        for i in 0..4 {
            out |= (self.read_u8()? as u32) << (8 * i);
        }
        Some(out)
    }

    pub fn write_f32(&mut self, value: f32) {
        self.write_u32(value.to_bits());
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        self.read_u32().map(f32::from_bits)
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

    /// `RBX::operator<<(RakNet::BitStream &,std::string const&)` framing
    /// (IDA 0x95e9f4): u32 length + payload, rejecting `len >= 0x30D41`.
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
    fn string_framing_roundtrip() {
        let mut s = BitStream::new();
        s.write_string("hello");
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_string(), "hello");
    }
}
