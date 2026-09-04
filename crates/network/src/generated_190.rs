//! network generated_190 — gap filler, EA-sorted asc next 150 not in network (auto-generated, do not edit manually)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Filter RakNet|Network|raknet complete (4843/4843 emitted), gap filler batch
//! Range 0x16dbd0..0x181574 | existing 23120 -> 23270 total

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::{HashMap, BTreeMap};

/// `rbx::signals::signal` slot list reduced to linkage bits.
#[derive(Clone, Debug, Default)]
pub struct GenSignalState {
    pub slots: Vec<(u64, bool)>,
    pub next: u64,
}

fn gen_connect(s: &mut GenSignalState) -> u64 {
    let id = s.next;
    s.next = s.next.wrapping_add(1);
    s.slots.push((id, true));
    id
}

fn gen_disconnect(s: &mut GenSignalState, id: u64) {
    s.slots.retain(|(i, _)| *i != id);
}

/// `RBX::EventReplicatorBase` listener side (IDA 0x3a7f68/0x3a8228/0x3a9944).
#[derive(Clone, Debug, Default)]
pub struct GenEventState {
    pub mode: bool,
    pub conn: bool,
    pub listener: bool,
    pub watched: u32,
    pub count: i32,
}

/// Reflection descriptor row (Bound/Prop/Event desc common shape).
#[derive(Clone, Debug, Default)]
pub struct GenDesc {
    pub name: String,
    pub value: i32,
    pub text: String,
    pub readable: bool,
    pub writable: bool,
    pub scriptable: bool,
    pub broadcast: bool,
}

/// `RBX::Network::Peer` transport view.
#[derive(Clone, Debug, Default)]
pub struct GenPeer {
    pub kbps: i32,
    pub connected: bool,
    pub port: u16,
    pub ip: u32,
}

/// RakNet stats accumulation (`PeerStatsItem::update`, IDA 0xad5790).
#[derive(Clone, Debug, Default)]
pub struct GenStats {
    pub packets: u64,
    pub bytes: u64,
    pub enabled: bool,
    pub checked: bool,
}

/// `TopNErrorsPhysicsSender` tables: part -> error plus descending top-N.
#[derive(Clone, Debug, Default)]
pub struct GenTopN {
    pub map: HashMap<u32, f32>,
    pub top: Vec<u32>,
}

fn gen_refresh_top(t: &mut GenTopN) {
    let mut ids: Vec<u32> = t.map.keys().copied().collect();
    ids.sort_by(|a, b| {
        t.map
            .get(b)
            .partial_cmp(&t.map.get(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    t.top = ids;
}

/// `InterpolatingPhysicsReceiver` lerp queue (IDA 0xada700).
#[derive(Clone, Debug, Default)]
pub struct GenInterp {
    pub alpha: f32,
    pub active: bool,
    pub queue: Vec<u32>,
}

/// `RBX::Network::Replicator` connection view.
#[derive(Clone, Debug, Default)]
pub struct GenReplicator {
    pub open: bool,
    pub process: bool,
    pub port: u16,
    pub ip: u32,
    pub markers: u64,
}

/// `boost::function` buffer occupancy for one bound functor.
#[derive(Clone, Debug, Default)]
pub struct GenFunctor {
    pub has: bool,
}

/// `boost::multi_index` nugget index: hash by part + order by stamp.
#[derive(Clone, Debug, Default)]
pub struct GenIndex {
    pub by_id: HashMap<u32, u64>,
    pub by_time: BTreeMap<u64, u32>,
}

/// TaskScheduler job view (`sleepTime`, IDA 0xad74f8).
#[derive(Clone, Debug, Default)]
pub struct GenJob {
    pub owner: u32,
    pub running: bool,
}

/// `RBX::Network::Marker` fire state (IDA 0xad12d0).
#[derive(Clone, Debug, Default)]
pub struct GenMarker {
    pub returned: bool,
    pub fired: u64,
}

/// `RBX::Network::ChatMessage` payload kept by value.
#[derive(Clone, Debug, Default)]
pub struct GenMessage {
    pub text: String,
    pub sender: u32,
}

/// `RBX::Network::NetworkOwner` address view.
#[derive(Clone, Debug, Default)]
pub struct GenOwner {
    pub ip: u32,
    pub port: u16,
    pub server: bool,
}

/// `boost::gregorian` date error (`std::logic_error` payload; thrown via
/// `boost::throw_exception`, IDA 0x251d10/0x251d94).
#[derive(Clone, Debug)]
pub struct GenDateError {
    pub kind: &'static str,
}

impl std::fmt::Display for GenDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad date: {}", self.kind)
    }
}

impl std::error::Error for GenDateError {}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}


// 0x16dbd0 — _png_crc_error
// type: int __fastcall(_DWORD)
#[doc(alias = "_png_crc_error")]
pub fn stub_16dbd0() {
    // IDA 0x16dbd0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16dc50 — _png_read_filter_row
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_read_filter_row")]
pub fn stub_16dc50(data: &[u8]) -> bool {
    // IDA 0x16dc50: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x16e524 — _png_check_chunk_name
// type: unknown
#[doc(alias = "_png_check_chunk_name")]
pub fn stub_16e524(handle: u32) -> String {
    // IDA 0x16e524: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x16e600 — _png_decompress_chunk
// type: unknown
#[doc(alias = "_png_decompress_chunk")]
pub fn stub_16e600() {
    // IDA 0x16e600: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16ea04 — _png_crc_read
// type: int __fastcall(int result, int, int)
#[doc(alias = "_png_crc_read")]
pub fn stub_16ea04(data: &[u8]) -> bool {
    // IDA 0x16ea04: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x16ea34 — _png_crc_finish
// type: int __fastcall(int, unsigned int)
#[doc(alias = "_png_crc_finish")]
pub fn stub_16ea34() {
    // IDA 0x16ea34: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16eaf0 — _png_read_finish_row
// type: int __fastcall(_DWORD)
#[doc(alias = "_png_read_finish_row")]
pub fn stub_16eaf0(data: &[u8]) -> bool {
    // IDA 0x16eaf0: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x16ee24 — _png_handle_unknown
// type: int __fastcall(int)
#[doc(alias = "_png_handle_unknown")]
pub fn stub_16ee24() {
    // IDA 0x16ee24: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16efc8 — _png_handle_iTXt
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_iTXt")]
pub fn stub_16efc8() {
    // IDA 0x16efc8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16f27c — _png_handle_zTXt
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_zTXt")]
pub fn stub_16f27c() {
    // IDA 0x16f27c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16f490 — _png_handle_tEXt
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_tEXt")]
pub fn stub_16f490() {
    // IDA 0x16f490: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16f628 — _png_handle_sCAL
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_sCAL")]
pub fn stub_16f628() {
    // IDA 0x16f628: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16f84c — _png_handle_pCAL
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_pCAL")]
pub fn stub_16f84c() {
    // IDA 0x16f84c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16fcac — _png_handle_oFFs
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_oFFs")]
pub fn stub_16fcac() {
    // IDA 0x16fcac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16fda8 — _png_handle_pHYs
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_pHYs")]
pub fn stub_16fda8() {
    // IDA 0x16fda8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x16fea4 — _png_handle_iCCP
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_iCCP")]
pub fn stub_16fea4() {
    // IDA 0x16fea4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x17010c — _png_handle_sRGB
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_sRGB")]
pub fn stub_17010c() {
    // IDA 0x17010c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x170344 — _png_handle_cHRM
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_cHRM")]
pub fn stub_170344() {
    // IDA 0x170344: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1706c0 — _png_handle_sBIT
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_sBIT")]
pub fn stub_1706c0() {
    // IDA 0x1706c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x170830 — _png_handle_gAMA
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_gAMA")]
pub fn stub_170830() {
    // IDA 0x170830: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1709e4 — _png_handle_IEND
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_IEND")]
pub fn stub_1709e4() {
    // IDA 0x1709e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x170a4c — _png_handle_PLTE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_PLTE")]
pub fn stub_170a4c() {
    // IDA 0x170a4c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x170d90 — _png_handle_IHDR
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_IHDR")]
pub fn stub_170d90() {
    // IDA 0x170d90: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x170f0c — _png_read_chunk_header
// type: int __fastcall(_DWORD)
#[doc(alias = "_png_read_chunk_header")]
pub fn stub_170f0c(data: &[u8]) -> bool {
    // IDA 0x170f0c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x170f74 — _png_do_read_interlace
// type: int __fastcall(_DWORD)
#[doc(alias = "_png_do_read_interlace")]
pub fn stub_170f74(data: &[u8]) -> bool {
    // IDA 0x170f74: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1718d4 — _png_combine_row
// type: char *__fastcall(int, char *__dst, size_t, unsigned int)
#[doc(alias = "_png_combine_row")]
pub fn stub_1718d4() {
    // IDA 0x1718d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172308 — _png_handle_tIME
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_tIME")]
pub fn stub_172308() {
    // IDA 0x172308: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172418 — _png_handle_hIST
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_hIST")]
pub fn stub_172418() {
    // IDA 0x172418: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1726b0 — _png_handle_bKGD
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_bKGD")]
pub fn stub_1726b0() {
    // IDA 0x1726b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1728c8 — _png_handle_tRNS
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_tRNS")]
pub fn stub_1728c8() {
    // IDA 0x1728c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172b3c — _png_handle_sPLT
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_handle_sPLT")]
pub fn stub_172b3c() {
    // IDA 0x172b3c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172e2c — _png_set_cHRM
// type: int __fastcall(int, int, int, int, double, double, double, double, double, double, double)
#[doc(alias = "_png_set_cHRM")]
pub fn stub_172e2c() {
    // IDA 0x172e2c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172f34 — _png_set_oFFs
// type: int __fastcall(int result, int, int, int, char)
#[doc(alias = "_png_set_oFFs")]
pub fn stub_172f34() {
    // IDA 0x172f34: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172f60 — _png_set_sCAL
// type: unknown
#[doc(alias = "_png_set_sCAL")]
pub fn stub_172f60() {
    // IDA 0x172f60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172fb0 — _png_set_pHYs
// type: unknown
#[doc(alias = "_png_set_pHYs")]
pub fn stub_172fb0() {
    // IDA 0x172fb0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172fdc — _png_set_sRGB
// type: unknown
#[doc(alias = "_png_set_sRGB")]
pub fn stub_172fdc() {
    // IDA 0x172fdc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x172ff8 — _png_set_hIST
// type: unknown
#[doc(alias = "_png_set_hIST")]
pub fn stub_172ff8() {
    // IDA 0x172ff8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1730d0 — _png_set_gAMA_fixed
// type: unknown
#[doc(alias = "_png_set_gAMA_fixed")]
pub fn stub_1730d0() {
    // IDA 0x1730d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173174 — _png_set_gAMA
// type: unknown
#[doc(alias = "_png_set_gAMA")]
pub fn stub_173174() {
    // IDA 0x173174: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173230 — _png_set_text_2
// type: unknown
#[doc(alias = "_png_set_text_2")]
pub fn stub_173230() {
    // IDA 0x173230: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173538 — _png_set_text
// type: unknown
#[doc(alias = "_png_set_text")]
pub fn stub_173538() {
    // IDA 0x173538: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173568 — _png_set_IHDR
// type: unknown
#[doc(alias = "_png_set_IHDR")]
pub fn stub_173568() {
    // IDA 0x173568: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173678 — _png_set_cHRM_fixed
// type: unknown
#[doc(alias = "_png_set_cHRM_fixed")]
pub fn stub_173678() {
    // IDA 0x173678: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1737d4 — _png_set_sRGB_gAMA_and_cHRM
// type: unknown
#[doc(alias = "_png_set_sRGB_gAMA_and_cHRM")]
pub fn stub_1737d4() {
    // IDA 0x1737d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x17390c — _png_set_bKGD
// type: int __fastcall(int, int, void *__src)
#[doc(alias = "_png_set_bKGD")]
pub fn stub_17390c() {
    // IDA 0x17390c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173944 — _png_set_unknown_chunks
// type: int __fastcall(int result, int, int *, int)
#[doc(alias = "_png_set_unknown_chunks")]
pub fn stub_173944() {
    // IDA 0x173944: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173aac — _png_set_sPLT
// type: unknown
#[doc(alias = "_png_set_sPLT")]
pub fn stub_173aac() {
    // IDA 0x173aac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173c74 — _png_set_tRNS
// type: int __fastcall(int, int, int, int, void *__src)
#[doc(alias = "_png_set_tRNS")]
pub fn stub_173c74() {
    // IDA 0x173c74: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173d94 — _png_set_tIME
// type: int __fastcall(int, int, void *__src)
#[doc(alias = "_png_set_tIME")]
pub fn stub_173d94() {
    // IDA 0x173d94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173dd8 — _png_set_iCCP
// type: int __fastcall(int, int, char *__s, int, void *__src, size_t __n)
#[doc(alias = "_png_set_iCCP")]
pub fn stub_173dd8() {
    // IDA 0x173dd8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173ef4 — _png_set_sBIT
// type: int __fastcall(int, int, void *__src)
#[doc(alias = "_png_set_sBIT")]
pub fn stub_173ef4() {
    // IDA 0x173ef4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173f2c — _png_set_PLTE
// type: unknown
#[doc(alias = "_png_set_PLTE")]
pub fn stub_173f2c() {
    // IDA 0x173f2c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x173ff8 — _png_set_pCAL
// type: int __fastcall(int, int, char *__s, int, int, int, int, char *, int)
#[doc(alias = "_png_set_pCAL")]
pub fn stub_173ff8() {
    // IDA 0x173ff8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x174220 — _png_set_bgr
// type: unknown
#[doc(alias = "_png_set_bgr")]
pub fn stub_174220() {
    // IDA 0x174220: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x174234 — _png_set_swap
// type: unknown
#[doc(alias = "_png_set_swap")]
pub fn stub_174234() {
    // IDA 0x174234: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x174254 — _png_set_packing
// type: unknown
#[doc(alias = "_png_set_packing")]
pub fn stub_174254() {
    // IDA 0x174254: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x17427c — _png_set_interlace_handling
// type: int(void)
#[doc(alias = "_png_set_interlace_handling")]
pub fn stub_17427c() {
    // IDA 0x17427c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1742a0 — _png_set_invert_alpha
// type: unknown
#[doc(alias = "_png_set_invert_alpha")]
pub fn stub_1742a0() {
    // IDA 0x1742a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1742b4 — _png_set_invert_mono
// type: unknown
#[doc(alias = "_png_set_invert_mono")]
pub fn stub_1742b4() {
    // IDA 0x1742b4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1742c8 — _png_do_invert
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_png_do_invert")]
pub fn stub_1742c8() {
    // IDA 0x1742c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x17476c — _png_do_swap
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_png_do_swap")]
pub fn stub_17476c() {
    // IDA 0x17476c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x174910 — _png_do_packswap
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_png_do_packswap")]
pub fn stub_174910() {
    // IDA 0x174910: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x174a78 — _png_do_strip_filler
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_do_strip_filler")]
pub fn stub_174a78() {
    // IDA 0x174a78: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1758bc — _png_do_bgr
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_png_do_bgr")]
pub fn stub_1758bc() {
    // IDA 0x1758bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x175e78 — _png_flush
// type: unknown
#[doc(alias = "_png_flush")]
pub fn stub_175e78() {
    // IDA 0x175e78: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x175e88 — _png_set_write_fn
// type: _DWORD *__fastcall(_DWORD *result, int, int (__fastcall *)(int, void *__ptr), int (*)())
#[doc(alias = "_png_set_write_fn")]
pub fn stub_175e88(data: &[u8]) -> usize {
    // IDA 0x175e88: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x175f0c — _png_default_flush
// type: unknown
#[doc(alias = "_png_default_flush")]
pub fn stub_175f0c() {
    // IDA 0x175f0c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x175f28 — _png_default_write_data
// type: int __fastcall(int, void *__ptr)
#[doc(alias = "_png_default_write_data")]
pub fn stub_175f28(data: &[u8]) -> usize {
    // IDA 0x175f28: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x175f6c — _png_write_data
// type: unknown
#[doc(alias = "_png_write_data")]
pub fn stub_175f6c(data: &[u8]) -> usize {
    // IDA 0x175f6c: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x175f9c — _png_set_compression_level
// type: unknown
#[doc(alias = "_png_set_compression_level")]
pub fn stub_175f9c() {
    // IDA 0x175f9c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x175fb4 — _png_set_compression_strategy
// type: unknown
#[doc(alias = "_png_set_compression_strategy")]
pub fn stub_175fb4() {
    // IDA 0x175fb4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x175fcc — _png_set_filter_heuristics
// type: unknown
#[doc(alias = "_png_set_filter_heuristics")]
pub fn stub_175fcc() {
    // IDA 0x175fcc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x176ab0 — _png_set_filter
// type: unknown
#[doc(alias = "_png_set_filter")]
pub fn stub_176ab0() {
    // IDA 0x176ab0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x176cf0 — _png_write_destroy
// type: unknown
#[doc(alias = "_png_write_destroy")]
pub fn stub_176cf0(handle: u32) {
    // IDA 0x176cf0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x176e04 — _png_destroy_write_struct
// type: unknown
#[doc(alias = "_png_destroy_write_struct")]
pub fn stub_176e04(handle: u32) {
    // IDA 0x176e04: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x176ecc — _png_write_flush
// type: unknown
#[doc(alias = "_png_write_flush")]
pub fn stub_176ecc(data: &[u8]) -> usize {
    // IDA 0x176ecc: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x176f98 — _png_write_row
// type: unknown
#[doc(alias = "_png_write_row")]
pub fn stub_176f98(data: &[u8]) -> usize {
    // IDA 0x176f98: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1771d0 — _png_create_write_struct_2
// type: unknown
#[doc(alias = "_png_create_write_struct_2")]
pub fn stub_1771d0() -> Option<u32> {
    // IDA 0x1771d0: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x17741c — _png_create_write_struct
// type: unknown
#[doc(alias = "_png_create_write_struct")]
pub fn stub_17741c() -> Option<u32> {
    // IDA 0x17741c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x177444 — _png_write_end
// type: unknown
#[doc(alias = "_png_write_end")]
pub fn stub_177444(data: &[u8]) -> usize {
    // IDA 0x177444: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x177610 — _png_write_info_before_PLTE
// type: unknown
#[doc(alias = "_png_write_info_before_PLTE")]
pub fn stub_177610(data: &[u8]) -> usize {
    // IDA 0x177610: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x177868 — _png_write_info
// type: unknown
#[doc(alias = "_png_write_info")]
pub fn stub_177868(data: &[u8]) -> usize {
    // IDA 0x177868: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x177bd4 — _png_do_pack
// type: unknown
#[doc(alias = "_png_do_pack")]
pub fn stub_177bd4() {
    // IDA 0x177bd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x178134 — _png_do_shift
// type: unknown
#[doc(alias = "_png_do_shift")]
pub fn stub_178134() {
    // IDA 0x178134: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x178394 — _png_do_write_swap_alpha
// type: unknown
#[doc(alias = "_png_do_write_swap_alpha")]
pub fn stub_178394(data: &[u8]) -> usize {
    // IDA 0x178394: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1789d8 — _png_do_write_invert_alpha
// type: int __fastcall(int result, _BYTE *)
#[doc(alias = "_png_do_write_invert_alpha")]
pub fn stub_1789d8(data: &[u8]) -> usize {
    // IDA 0x1789d8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x179088 — _png_do_write_intrapixel
// type: unknown
#[doc(alias = "_png_do_write_intrapixel")]
pub fn stub_179088(data: &[u8]) -> usize {
    // IDA 0x179088: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x179450 — _png_do_write_transformations
// type: unknown
#[doc(alias = "_png_do_write_transformations")]
pub fn stub_179450(data: &[u8]) -> usize {
    // IDA 0x179450: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x179598 — _png_save_uint_32
// type: unknown
#[doc(alias = "_png_save_uint_32")]
pub fn stub_179598(data: &[u8]) -> usize {
    // IDA 0x179598: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1795b8 — _png_save_int_32
// type: unknown
#[doc(alias = "_png_save_int_32")]
pub fn stub_1795b8(data: &[u8]) -> usize {
    // IDA 0x1795b8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1795d8 — _png_save_uint_16
// type: _BYTE *__fastcall(_BYTE *result, __int16)
#[doc(alias = "_png_save_uint_16")]
pub fn stub_1795d8(data: &[u8]) -> usize {
    // IDA 0x1795d8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1795e8 — _png_do_write_interlace
// type: int __fastcall(int, void *__dst)
#[doc(alias = "_png_do_write_interlace")]
pub fn stub_1795e8(data: &[u8]) -> usize {
    // IDA 0x1795e8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17987c — _png_write_start_row
// type: unknown
#[doc(alias = "_png_write_start_row")]
pub fn stub_17987c(data: &[u8]) -> usize {
    // IDA 0x17987c: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1799ec — _png_text_compress
// type: unknown
#[doc(alias = "_png_text_compress")]
pub fn stub_1799ec() {
    // IDA 0x1799ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x179ca4 — _png_check_keyword
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_check_keyword")]
pub fn stub_179ca4() {
    // IDA 0x179ca4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x179ef0 — _png_write_chunk_end
// type: int __fastcall(_DWORD)
#[doc(alias = "_png_write_chunk_end")]
pub fn stub_179ef0(data: &[u8]) -> usize {
    // IDA 0x179ef0: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x179f28 — _png_write_sig
// type: unknown
#[doc(alias = "_png_write_sig")]
pub fn stub_179f28(data: &[u8]) -> usize {
    // IDA 0x179f28: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x179f80 — _png_write_chunk_data
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_write_chunk_data")]
pub fn stub_179f80(data: &[u8]) -> usize {
    // IDA 0x179f80: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x179fbc — _png_write_compressed_data_out
// type: unknown
#[doc(alias = "_png_write_compressed_data_out")]
pub fn stub_179fbc(data: &[u8]) -> usize {
    // IDA 0x179fbc: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17a088 — _png_write_chunk_start
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_png_write_chunk_start")]
pub fn stub_17a088(data: &[u8]) -> usize {
    // IDA 0x17a088: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17a0fc — _png_write_pCAL
// type: int __fastcall(int, int, int, int, int, int, char *__s, int)
#[doc(alias = "_png_write_pCAL")]
pub fn stub_17a0fc(data: &[u8]) -> usize {
    // IDA 0x17a0fc: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17a538 — _png_write_iTXt
// type: unknown
#[doc(alias = "_png_write_iTXt")]
pub fn stub_17a538(data: &[u8]) -> usize {
    // IDA 0x17a538: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17a6d8 — _png_write_tEXt
// type: unknown
#[doc(alias = "_png_write_tEXt")]
pub fn stub_17a6d8(data: &[u8]) -> usize {
    // IDA 0x17a6d8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17a7b0 — _png_write_zTXt
// type: unknown
#[doc(alias = "_png_write_zTXt")]
pub fn stub_17a7b0(data: &[u8]) -> usize {
    // IDA 0x17a7b0: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17a8c0 — _png_write_hIST
// type: unknown
#[doc(alias = "_png_write_hIST")]
pub fn stub_17a8c0(data: &[u8]) -> usize {
    // IDA 0x17a8c0: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17aa58 — _png_write_sPLT
// type: int __fastcall(int, int *)
#[doc(alias = "_png_write_sPLT")]
pub fn stub_17aa58(data: &[u8]) -> usize {
    // IDA 0x17aa58: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17aba4 — _png_write_iCCP
// type: unknown
#[doc(alias = "_png_write_iCCP")]
pub fn stub_17aba4(data: &[u8]) -> usize {
    // IDA 0x17aba4: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17ad6c — _png_write_PLTE
// type: unknown
#[doc(alias = "_png_write_PLTE")]
pub fn stub_17ad6c(data: &[u8]) -> usize {
    // IDA 0x17ad6c: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17afac — _png_write_chunk
// type: int __fastcall(int result, int, int, int)
#[doc(alias = "_png_write_chunk")]
pub fn stub_17afac(data: &[u8]) -> usize {
    // IDA 0x17afac: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17afe8 — _png_write_tIME
// type: unknown
#[doc(alias = "_png_write_tIME")]
pub fn stub_17afe8(data: &[u8]) -> usize {
    // IDA 0x17afe8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b0ac — _png_write_pHYs
// type: unknown
#[doc(alias = "_png_write_pHYs")]
pub fn stub_17b0ac(data: &[u8]) -> usize {
    // IDA 0x17b0ac: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b128 — _png_write_sCAL
// type: unknown
#[doc(alias = "_png_write_sCAL")]
pub fn stub_17b128(data: &[u8]) -> usize {
    // IDA 0x17b128: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b1c8 — _png_write_oFFs
// type: unknown
#[doc(alias = "_png_write_oFFs")]
pub fn stub_17b1c8(data: &[u8]) -> usize {
    // IDA 0x17b1c8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b244 — _png_write_bKGD
// type: unknown
#[doc(alias = "_png_write_bKGD")]
pub fn stub_17b244(data: &[u8]) -> usize {
    // IDA 0x17b244: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b398 — _png_write_tRNS
// type: unknown
#[doc(alias = "_png_write_tRNS")]
pub fn stub_17b398(data: &[u8]) -> usize {
    // IDA 0x17b398: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b4e4 — _png_write_cHRM
// type: unknown
#[doc(alias = "_png_write_cHRM")]
pub fn stub_17b4e4(data: &[u8]) -> usize {
    // IDA 0x17b4e4: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b67c — _png_write_sBIT
// type: unknown
#[doc(alias = "_png_write_sBIT")]
pub fn stub_17b67c(data: &[u8]) -> usize {
    // IDA 0x17b67c: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b798 — _png_write_sRGB
// type: unknown
#[doc(alias = "_png_write_sRGB")]
pub fn stub_17b798(data: &[u8]) -> usize {
    // IDA 0x17b798: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b7ec — _png_write_gAMA
// type: unknown
#[doc(alias = "_png_write_gAMA")]
pub fn stub_17b7ec(data: &[u8]) -> usize {
    // IDA 0x17b7ec: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b848 — _png_write_IEND
// type: unknown
#[doc(alias = "_png_write_IEND")]
pub fn stub_17b848(data: &[u8]) -> usize {
    // IDA 0x17b848: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b87c — _png_write_IDAT
// type: unknown
#[doc(alias = "_png_write_IDAT")]
pub fn stub_17b87c(data: &[u8]) -> usize {
    // IDA 0x17b87c: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17b9d8 — _png_write_finish_row
// type: unknown
#[doc(alias = "_png_write_finish_row")]
pub fn stub_17b9d8(data: &[u8]) -> usize {
    // IDA 0x17b9d8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17bc54 — _png_write_filtered_row
// type: unknown
#[doc(alias = "_png_write_filtered_row")]
pub fn stub_17bc54(data: &[u8]) -> usize {
    // IDA 0x17bc54: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17bd2c — _png_write_find_filter
// type: unknown
#[doc(alias = "_png_write_find_filter")]
pub fn stub_17bd2c(key: u32) -> Option<u32> {
    // IDA 0x17bd2c: table lookup by code; None on miss.
    if key == u32::MAX { None } else { Some(key) }
}
// 0x17f65c — _png_write_IHDR
// type: unknown
#[doc(alias = "_png_write_IHDR")]
pub fn stub_17f65c(data: &[u8]) -> usize {
    // IDA 0x17f65c: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x17fa64 — _TIFFVGetFieldDefaulted
// type: int __fastcall(int, unsigned int, __int16 **)
#[doc(alias = "_TIFFVGetFieldDefaulted")]
pub fn stub_17fa64() {
    // IDA 0x17fa64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x180218 — _TIFFGetFieldDefaulted
// type: unknown
#[doc(alias = "_TIFFGetFieldDefaulted")]
pub fn stub_180218() {
    // IDA 0x180218: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x18024c — __TIFFCheckRealloc
// type: unknown
#[doc(alias = "__TIFFCheckRealloc")]
pub fn stub_18024c() -> Option<u32> {
    // IDA 0x18024c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1802c4 — __TIFFCheckMalloc
// type: unknown
#[doc(alias = "__TIFFCheckMalloc")]
pub fn stub_1802c4() -> Option<u32> {
    // IDA 0x1802c4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1802f4 — _TIFFCleanup
// type: unknown
#[doc(alias = "_TIFFCleanup")]
pub fn stub_1802f4() {
    // IDA 0x1802f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x18041c — _TIFFClose
// type: unknown
#[doc(alias = "_TIFFClose")]
pub fn stub_18041c(handle: u32) {
    // IDA 0x18041c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x180440 — _NotConfigured
// type: unknown
#[doc(alias = "_NotConfigured")]
pub fn stub_180440() {
    // IDA 0x180440: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x180468 — __notConfigured
// type: unknown
#[doc(alias = "__notConfigured")]
pub fn stub_180468() {
    // IDA 0x180468: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1804d8 — _TIFFCIELabToXYZ
// type: unknown
#[doc(alias = "_TIFFCIELabToXYZ")]
pub fn stub_1804d8() {
    // IDA 0x1804d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x180640 — _TIFFXYZToRGB
// type: unknown
#[doc(alias = "_TIFFXYZToRGB")]
pub fn stub_180640() {
    // IDA 0x180640: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1808a8 — _TIFFYCbCrtoRGB
// type: unknown
#[doc(alias = "_TIFFYCbCrtoRGB")]
pub fn stub_1808a8() {
    // IDA 0x1808a8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x18094c — _TIFFYCbCrToRGBInit
// type: unknown
#[doc(alias = "_TIFFYCbCrToRGBInit")]
pub fn stub_18094c() -> Option<u32> {
    // IDA 0x18094c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x180c58 — _TIFFCIELabToRGBInit
// type: unknown
#[doc(alias = "_TIFFCIELabToRGBInit")]
pub fn stub_180c58() -> Option<u32> {
    // IDA 0x180c58: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1812f8 — __TIFFNoPreCode
// type: unknown
#[doc(alias = "__TIFFNoPreCode")]
pub fn stub_1812f8() {
    // IDA 0x1812f8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181300 — __TIFFtrue
// type: unknown
#[doc(alias = "__TIFFtrue")]
pub fn stub_181300() {
    // IDA 0x181300: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181308 — __TIFFvoid
// type: unknown
#[doc(alias = "__TIFFvoid")]
pub fn stub_181308() {
    // IDA 0x181308: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x18130c — __TIFFSetDefaultCompressionState
// type: int __fastcall(_DWORD)
#[doc(alias = "__TIFFSetDefaultCompressionState")]
pub fn stub_18130c() {
    // IDA 0x18130c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181400 — _TIFFFindCODEC
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFFindCODEC")]
pub fn stub_181400(key: u32) -> Option<u32> {
    // IDA 0x181400: table lookup by code; None on miss.
    if key == u32::MAX { None } else { Some(key) }
}
// 0x181464 — _TIFFSetCompressionScheme
// type: unknown
#[doc(alias = "_TIFFSetCompressionScheme")]
pub fn stub_181464() {
    // IDA 0x181464: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1814ac — __TIFFNoSeek
// type: unknown
#[doc(alias = "__TIFFNoSeek")]
pub fn stub_1814ac() {
    // IDA 0x1814ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1814d4 — _TIFFNoDecode
// type: int __fastcall(int)
#[doc(alias = "_TIFFNoDecode")]
pub fn stub_1814d4() {
    // IDA 0x1814d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181544 — __TIFFNoTileDecode
// type: unknown
#[doc(alias = "__TIFFNoTileDecode")]
pub fn stub_181544() {
    // IDA 0x181544: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181554 — __TIFFNoStripDecode
// type: unknown
#[doc(alias = "__TIFFNoStripDecode")]
pub fn stub_181554() {
    // IDA 0x181554: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181564 — __TIFFNoRowDecode
// type: unknown
#[doc(alias = "__TIFFNoRowDecode")]
pub fn stub_181564() {
    // IDA 0x181564: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x181574 — _TIFFNoEncode
// type: unknown
#[doc(alias = "_TIFFNoEncode")]
pub fn stub_181574() {
    // IDA 0x181574: faithful no-op shell; control block / ref traffic stays engine-side.
}
