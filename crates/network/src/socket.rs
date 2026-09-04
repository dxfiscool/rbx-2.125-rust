//! RakNet runtime objects: `PluginInterface2`, `RakNetSocket`, the
//! `_RakMalloc_Ex` family, and `StatisticsToString`.
//!
//! Decompiled from the plugin ctor/dtor/set (IDA 0xa5a2ac/0xa5a2c4/
//! 0xa5a2d0/0xa5a2d4/0xa5a2d8), the allocator wrappers (IDA
//! 0xa5a900/0xa5a90c/0xa5a918), the socket ctor/dtor (IDA
//! 0xa5af38/0xa5af50), and `StatisticsToString` (IDA 0xa5b5b0). File
//! descriptors, vtables, and the full statistics struct stay engine-side.

#![allow(dead_code)]

/// `RakNet::PluginInterface2` reduced to its peer handle (+4).
#[derive(Clone, Copy, Debug, Default)]
pub struct PluginInterface2 {
    pub rak_peer: Option<u32>,
}

impl PluginInterface2 {
    /// `PluginInterface2::PluginInterface2` (IDA 0xa5a2ac).
    pub fn new() -> Self {
        Self::default()
    }

    /// `PluginInterface2::SetRakPeerInterface` (IDA 0xa5a2d8):
    /// `*(this + 4) = peer`.
    pub fn set_rak_peer_interface(&mut self, peer: Option<u32>) {
        self.rak_peer = peer;
    }
}

/// `RakNet::_RakMalloc_Ex` (IDA 0xa5a900): zeroed bytes; the file/line
/// tracking stays engine-side.
#[must_use]
pub fn rak_malloc(size: usize) -> Vec<u8> {
    vec![0u8; size]
}

/// `RakNet::_RakRealloc_Ex` (IDA 0xa5a90c): grown tails zero-fill here
/// where the original leaves them uninitialized.
#[must_use]
pub fn rak_realloc(buf: Vec<u8>, size: usize) -> Vec<u8> {
    let mut buf = buf;
    buf.resize(size, 0);
    buf
}

/// `RakNet::_RakFree_Ex` (IDA 0xa5a918).
pub fn rak_free(_buf: Vec<u8>) {}
/// `RakNet::RakNetSocket::RakNetSocket` (IDA 0xa5af38): descriptor and
/// buffer init stays engine-side.
pub fn init_raknet_socket() {}
/// `RakNet::RakNetSocket::~RakNetSocket` (IDA 0xa5af50): closing the
/// descriptor stays engine-side.
pub fn free_raknet_socket() {}

/// `RakNet::StatisticsToString` null arm (IDA 0xa5b5e8).
#[must_use]
pub fn statistics_null_text() -> &'static str {
    "stats is a NULL pointer in statsToString\n"
}

/// `RakNet::StatisticsToString` brief arm (IDA 0xa5b72a): three lines
/// over bytes-sent/received per second and current packetloss.
#[must_use]
pub fn statistics_brief_text(sent_per_sec: u64, received_per_sec: u64, packetloss: f32) -> String {
    format!(
        "Bytes per second sent     {sent_per_sec}\nBytes per second received {received_per_sec}\nCurrent packetloss        {packetloss:.1}%\n"
    )
}

/// `RakNet::StatisticsToString` (IDA 0xa5b5b0): null stats write the
/// null text, verbosity 0 writes the brief text, and higher verbosity
/// formats the full struct engine-side.
pub fn statistics_to_string(
    present: bool,
    verbose: u32,
    sent_per_sec: u64,
    received_per_sec: u64,
    packetloss: f32,
    full: &mut dyn FnMut() -> String,
) -> String {
    if !present {
        return statistics_null_text().to_owned();
    }
    if verbose == 0 {
        return statistics_brief_text(sent_per_sec, received_per_sec, packetloss);
    }
    full()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_objects() {
        // IDA 0xa5a2ac/0xa5a2d8: plugin handle store.
        let mut plugin = PluginInterface2::new();
        assert_eq!(plugin.rak_peer, None);
        plugin.set_rak_peer_interface(Some(9));
        assert_eq!(plugin.rak_peer, Some(9));
        // IDA 0xa5a900/0xa5a90c/0xa5a918: allocator roundtrip.
        let buf = rak_malloc(4);
        assert_eq!(buf, vec![0, 0, 0, 0]);
        let buf = rak_realloc(buf, 6);
        assert_eq!(buf, vec![0, 0, 0, 0, 0, 0]);
        rak_free(buf);
        init_raknet_socket();
        free_raknet_socket();
        // IDA 0xa5b5b0: null/brief/full arms.
        assert_eq!(
            statistics_to_string(false, 0, 0, 0, 0.0, &mut || unreachable!()),
            "stats is a NULL pointer in statsToString\n"
        );
        assert_eq!(
            statistics_to_string(true, 0, 10, 20, 1.5, &mut || unreachable!()),
            "Bytes per second sent     10\nBytes per second received 20\nCurrent packetloss        1.5%\n"
        );
        assert_eq!(
            statistics_to_string(true, 2, 0, 0, 0.0, &mut || "full".to_owned()),
            "full"
        );
    }
}
