//! `RakNet::HuffmanEncodingTree` — byte Huffman codec.
//!
//! Decompiled from `GenerateFromFrequencyTable` (IDA 0xa57a3c),
//! `EncodeArray` (IDA 0xa58090), and `DecodeArray` (IDA 0xa580f0).
//! Zero weights count as 1; codes are root-to-leaf bit paths with the
//! left child contributing 0. The merged-node tie order below is a
//! stable textbook equivalent of the original's sorted-list insertion.

#![allow(dead_code)]

use super::bitstream::BitStream;

#[derive(Clone, Copy, Debug)]
struct Node {
    weight: u32,
    symbol: Option<u8>,
    left: usize,
    right: usize,
}

/// `RakNet::HuffmanEncodingTree`.
#[derive(Clone, Debug)]
pub struct HuffmanTree {
 nodes: Vec<Node>,
 root: usize,
 /// Per-symbol `(code, length)`; codes are the low `length` bits.
 codes: [(u32, u8); 256],
}

impl Default for HuffmanTree {
 fn default() -> Self {
 Self { nodes: Vec::new(), root: 0, codes: [(0, 0); 256] }
 }
}

impl HuffmanTree {
    /// `HuffmanEncodingTree::HuffmanEncodingTree` (IDA 0xa57874).
    pub fn new() -> Self {
        Self::default()
    }

    /// `HuffmanEncodingTree::FreeMemory` (IDA 0xa5788c): releases the
    /// tree; the destructor (IDA 0xa5787c) does the same implicitly.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.codes = [(0, 0); 256];
    }

    /// `GenerateFromFrequencyTable` (IDA 0xa57a3c): builds the merge
    /// tree over the 256 weights, then records each leaf's root-to-leaf
    /// path as its code.
    pub fn generate(&mut self, freq: &[u32; 256]) {
        self.clear();
        let mut nodes: Vec<Node> = (0..256)
            .map(|i| Node {
                weight: freq[i].max(1),
                symbol: Some(i as u8),
                left: usize::MAX,
                right: usize::MAX,
            })
            .collect();
        // IDA 0xa57b9e..0xa57c76: stable ascending insert (ties keep
        // symbol order via the strict `<` walk).
        let mut queue: Vec<usize> = (0..256).collect();
        queue.sort_by_key(|&i| nodes[i].weight);
        // IDA 0xa57ca4..0xa57e9a: repeatedly merge the two lightest.
        while queue.len() > 1 {
            let a = queue.remove(0);
            let b = queue.remove(0);
            let weight = nodes[a].weight + nodes[b].weight;
            nodes.push(Node { weight, symbol: None, left: a, right: b });
            let at = nodes.len() - 1;
            let pos = queue.iter().position(|&i| nodes[i].weight >= weight).unwrap_or(queue.len());
            queue.insert(pos, at);
        }
        let root = queue[0];
        // IDA 0xa57ee0..0xa57f84: leaf-to-root walks, emitted root-first.
        let mut codes = [(0u32, 0u8); 256];
        let mut stack = vec![(root, 0u32, 0u8)];
        while let Some((at, code, len)) = stack.pop() {
            if let Some(sym) = nodes[at].symbol {
                codes[sym as usize] = (code, len);
            } else {
                // Left (0) pushed last so the right subtree assigns first;
                // assignment order is unobservable, only paths matter.
                stack.push((nodes[at].left, (code << 1), len + 1));
                stack.push((nodes[at].right, (code << 1) | 1, len + 1));
            }
        }
        self.root = root;
        self.nodes = nodes;
        self.codes = codes;
    }

    /// `EncodeArray` (IDA 0xa58090): each byte's code bits, then, when
    /// the stream is left unaligned, the leading bits of the first code
    /// longer than the remainder (padding).
    pub fn encode(&self, stream: &mut BitStream, data: &[u8]) {
        for &b in data {
            let (code, len) = self.codes[b as usize];
            stream.write_bits(code, len);
        }
        let rem = (8 - (stream.bits_written() & 7)) & 7;
        if rem != 0 {
            if let Some(&(code, len)) = self.codes.iter().find(|&&(_, len)| (len as usize) > rem) {
                stream.write_bits(code >> (len as usize - rem), rem as u8);
            }
        }
    }

    /// `DecodeArray` (IDA 0xa580f0): walks at most `max_bits` bits,
    /// restarting at the root per leaf. Returns the decoded symbol
    /// count, which may exceed `out.len()` (extra symbols are counted
    /// but not stored).
    pub fn decode(&self, stream: &mut BitStream, max_bits: usize, out: &mut [u8]) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        let mut at = self.root;
        let mut decoded = 0;
        for _ in 0..max_bits {
            // `ReadBit` past the end reads 0, like the original.
            let bit = stream.read_bit().unwrap_or(false);
            at = if bit { self.nodes[at].right } else { self.nodes[at].left };
            if let Some(sym) = self.nodes[at].symbol {
                if decoded < out.len() {
                    out[decoded] = sym;
                }
                decoded += 1;
                at = self.root;
            }
        }
        decoded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skewed_roundtrip() {
        // IDA 0xa57a3c/0xa58090/0xa580f0: frequent symbols go short.
        let mut freq = [0u32; 256];
        freq[b'a' as usize] = 100;
        freq[b'b' as usize] = 10;
        freq[b'c' as usize] = 1;
        let mut tree = HuffmanTree::new();
        tree.generate(&freq);
        let (_, la) = tree.codes[b'a' as usize];
        let (_, lb) = tree.codes[b'b' as usize];
        let (_, lc) = tree.codes[b'c' as usize];
        assert!(la <= lb && lb <= lc, "{la} {lb} {lc}");
        let mut s = BitStream::new();
        tree.encode(&mut s, b"aaabc");
        // Decode exactly the code bits (the byte pad is not data).
        let bits = b"aaabc".iter().map(|&b| tree.codes[b as usize].1 as usize).sum();
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = [0u8; 5];
        assert_eq!(tree.decode(&mut r, bits, &mut out), 5);
        assert_eq!(&out, b"aaabc");
        // IDA 0xa5788c: clear empties the tree.
        tree.clear();
        assert_eq!(tree.decode(&mut r, 8, &mut out), 0);
    }
}
