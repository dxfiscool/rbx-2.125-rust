//! `RBX::Network::Item::ItemType` wire codec.
//!
//! Decompiled from 0x9add90 (`writeItemType`) and 0x9addcc (`readItemType`).
//! Types 1..=3 pack into 2 bits; anything else (0, 4..=15) is prefixed with
//! two zero bits followed by the 4-bit value.

#![allow(dead_code)]

use super::bitstream::BitStream;

/// `RBX::Network::Item::writeItemType` (IDA 0x9add90).
pub fn write_item_type(stream: &mut BitStream, item_type: u8) {
    // IDA 0x9add9e: `(type - 1) > 2` selects the long form.
    if item_type.wrapping_sub(1) > 2 {
        // IDA 0x9addac: zero prefix...
        stream.write_bits(0, 2);
        // IDA 0x9addc8: ...then the 4-bit value.
        stream.write_bits(item_type as u32, 4);
    } else {
        // IDA 0x9adda4: values 1..=3 fit in 2 bits.
        stream.write_bits(item_type as u32, 2);
    }
}

/// `RBX::Network::Item::readItemType` (IDA 0x9addcc).
pub fn read_item_type(stream: &mut BitStream) -> u8 {
    // IDA 0x9addd6: the out-param is zeroed first, then 2 bits are read.
    let mut value = 0u8;
    if let Some(bits) = stream.read_bits(2) {
        value = bits as u8;
    }
    if value == 0 {
        // IDA 0x9addf2: zero prefix means a 4-bit value follows.
        if let Some(bits) = stream.read_bits(4) {
            value = bits as u8;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_types_roundtrip() {
        for t in 0..16u8 {
            let mut s = BitStream::new();
            write_item_type(&mut s, t);
            let mut r = BitStream::from_bytes(&s.into_bytes());
            assert_eq!(read_item_type(&mut r), t, "type {t}");
        }
    }

    #[test]
    fn short_form_uses_two_bits() {
        let mut s = BitStream::new();
        write_item_type(&mut s, 2);
        assert_eq!(s.bits_written(), 2);
        let mut l = BitStream::new();
        write_item_type(&mut l, 9);
        assert_eq!(l.bits_written(), 6);
    }
}
