//! `RakNet::StringCompressor` — language-keyed Huffman string codec.
//!
//! Decompiled from `AddReference` (IDA 0xa7b268, builds the singleton and
//! generates the English tree from `englishCharacterFrequencies`,
//! IDA 0x12b9f70), `RemoveReference` (IDA 0xa7b39c), `Instance`
//! (IDA 0xa7b470), the dtor (IDA 0xa7b480), `EncodeString` (IDA 0xa7b594),
//! and `DecodeString` (IDA 0xa7b764). The per-language map holds the
//! English tree under id 0; unknown languages make encode a silent no-op
//! and decode fail, exactly like the original's map-miss paths.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::bitstream::BitStream;
use super::huffman::HuffmanTree;

/// `englishCharacterFrequencies` (IDA 0x12b9f70): the 256 English weights
/// the singleton tree is generated from on first reference.
pub const ENGLISH_FREQUENCIES: [u32; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 722, 0, 0, 2, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    11084, 58, 63, 1, 0, 31, 0, 317,
    64, 64, 44, 0, 695, 62, 980, 266,
    69, 67, 56, 7, 73, 3, 14, 2,
    69, 1, 167, 9, 1, 2, 25, 94,
    0, 195, 139, 34, 96, 48, 103, 56,
    125, 653, 21, 5, 23, 64, 85, 44,
    34, 7, 92, 76, 147, 12, 14, 57,
    15, 39, 15, 1, 1, 1, 2, 3,
    0, 3611, 845, 1077, 1884, 5870, 841, 1057,
    2501, 3212, 164, 531, 2019, 1330, 3056, 4037,
    848, 47, 2586, 2919, 4771, 1707, 535, 1106,
    152, 1243, 100, 0, 2, 0, 10, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
];

/// Language id of the prebuilt English tree (map key `0`, IDA 0xa7b312).
pub const ENGLISH_LANGUAGE: u8 = 0;

/// `RakNet::StringCompressor`: language id to Huffman tree map.
#[derive(Debug, Default)]
pub struct StringCompressor {
    /// Sorted language-id to tree map (the original's `Map<int, ...>`).
    trees: BTreeMap<u8, HuffmanTree>,
}

impl StringCompressor {
    /// Build the singleton content: English tree from the IDA table.
    fn with_english() -> Self {
        let mut tree = HuffmanTree::new();
        tree.generate(&ENGLISH_FREQUENCIES);
        let mut trees = BTreeMap::new();
        trees.insert(ENGLISH_LANGUAGE, tree);
        Self { trees }
    }

    /// `EncodeString` (IDA 0xa7b594): Huffman-encode at most
    /// `max_chars - 1` bytes (`max_chars < 1` disables the cap), then frame
    /// as compressed bit length plus payload bits. Null input writes a zero
    /// length. Unknown languages are a silent no-op.
    pub fn encode_string(
        &self,
        text: Option<&[u8]>,
        max_chars: i32,
        language: u8,
        out: &mut BitStream,
    ) {
        let Some(tree) = self.trees.get(&language) else {
            return;
        };
        let Some(text) = text else {
            out.write_compressed_u32(0);
            return;
        };
        // IDA 0xa7b696: cap at `max_chars - 1` when it binds.
        let len = if max_chars >= 1 && text.len() >= max_chars as usize {
            max_chars as usize - 1
        } else {
            text.len()
        };
        let mut encoded = BitStream::new();
        tree.encode(&mut encoded, &text[..len]);
        let bits = encoded.bits_written() as u32;
        out.write_compressed_u32(bits);
        out.write_stream_bits(&mut encoded, bits as usize);
    }

    /// `DecodeString` (IDA 0xa7b764): read the framed bit length, decode at
    /// most `max_len` bytes, and return them (`None` for unknown languages,
    /// short reads, or overrun payloads).
    pub fn decode_string(
        &self,
        max_len: usize,
        language: u8,
        stream: &mut BitStream,
    ) -> Option<Vec<u8>> {
        let tree = self.trees.get(&language)?;
        if max_len < 1 {
            return None;
        }
        // IDA 0xa7b804: the buffer is NUL-cleared first; the Vec starts empty.
        let bits = stream.read_compressed_u32()? as usize;
        // IDA 0xa7b820: the payload must be fully present.
        if stream.bits_remaining() < bits {
            return None;
        }
        let mut out = vec![0u8; max_len];
        let decoded = tree.decode(stream, bits, &mut out);
        // IDA 0xa7b832: terminate at `decoded` (or `max_len - 1` on overrun).
        let len = decoded.min(max_len - 1);
        out.truncate(len);
        Some(out)
    }
}

/// Global singleton plus reference count (IDA `instance`/`referenceCount`).
struct Global {
    compressor: Mutex<Option<StringCompressor>>,
    references: AtomicUsize,
}

fn global() -> &'static Global {
    static GLOBAL: OnceLock<Global> = OnceLock::new();
    GLOBAL.get_or_init(|| Global {
        compressor: Mutex::new(None),
        references: AtomicUsize::new(0),
    })
}

/// `AddReference` (IDA 0xa7b268): first reference builds the singleton and
/// generates the English tree (IDA 0xa7b30a).
pub fn add_reference() {
    let global = global();
    if global.references.fetch_add(1, Ordering::SeqCst) == 0 {
        *global.compressor.lock().expect("compressor") =
            Some(StringCompressor::with_english());
    }
}

/// `RemoveReference` (IDA 0xa7b39c): drop the singleton when the last
/// reference goes away.
pub fn remove_reference() {
    let global = global();
    if global.references.fetch_sub(1, Ordering::SeqCst) == 1 {
        *global.compressor.lock().expect("compressor") = None;
    }
}

/// `Instance` (IDA 0xa7b470): run a closure against the singleton
/// (`None` when no references are held).
pub fn with_instance<R>(f: impl FnOnce(&StringCompressor) -> R) -> Option<R> {
    let global = global();
    let guard = global.compressor.lock().expect("compressor");
    guard.as_ref().map(f)
}

/// Whether the singleton currently exists.
#[must_use]
pub fn instance_exists() -> bool {
    with_instance(|_| ()).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;

    /// The singleton is process-global; serialize these tests so reference
    /// counts stay balanced.
    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn singleton_lifecycle() {
        let _guard = TEST_LOCK.lock().expect("test lock");
        // IDA 0xa7b268/0xa7b470/0xa7b39c: build, observe, release.
        assert!(!instance_exists());
        add_reference();
        add_reference();
        assert!(instance_exists());
        remove_reference();
        assert!(instance_exists());
        remove_reference();
        assert!(!instance_exists());
    }

    #[test]
    fn string_roundtrip_with_cap() {
        let _guard = TEST_LOCK.lock().expect("test lock");
        // IDA 0xa7b594/0xa7b764: frame, decode, NUL-terminate.
        add_reference();
        let text = b"Hello, World!";
        let mut stream = BitStream::new();
        with_instance(|c| c.encode_string(Some(text.as_slice()), 64, ENGLISH_LANGUAGE, &mut stream))
            .expect("instance");
        let back = with_instance(|c| {
            c.decode_string(64, ENGLISH_LANGUAGE, &mut stream)
        })
        .expect("instance")
        .expect("decode");
        assert_eq!(back, text);
        // Null input frames a zero length (IDA 0xa7b6aa).
        let mut null_stream = BitStream::new();
        with_instance(|c| c.encode_string(None, 64, ENGLISH_LANGUAGE, &mut null_stream))
            .expect("instance");
        let back = with_instance(|c| {
            c.decode_string(64, ENGLISH_LANGUAGE, &mut null_stream)
        })
        .expect("instance")
        .expect("decode");
        assert!(back.is_empty());
        // Unknown language: encode is a no-op, decode fails.
        let mut stream = BitStream::new();
        with_instance(|c| c.encode_string(Some(text.as_slice()), 64, 7, &mut stream))
            .expect("instance");
        assert_eq!(stream.bits_written(), 0);
        assert!(
            with_instance(|c| c.decode_string(64, 7, &mut stream))
                .expect("instance")
                .is_none()
        );
        remove_reference();
    }
}
