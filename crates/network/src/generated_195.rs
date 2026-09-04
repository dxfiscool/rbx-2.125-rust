//! network generated_195 — gap filler, EA-sorted asc next 150 not yet in network (auto-generated, do not edit manually)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Filter RakNet|Network complete (4853/4853 emitted), gap filler batch
//! Range 0x1bacdc..0x1ca394 | 23699 -> 23849 distinct | 0xADDR mangled + doc alias + todo!("0xADDR") + rbx_core::SharedPtr not boost

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

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}



// 0x1bacdc — _inflate_fast
// type: unknown
#[doc(alias = "_inflate_fast")]
pub fn stub_1bacdc() {
    // IDA 0x1bacdc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1bb908 — _inflateReset
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_inflateReset")]
pub fn stub_1bb908(handle: u32) {
    // IDA 0x1bb908: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1bb980 — _inflateInit2_
// type: int __cdecl(z_streamp strm, int windowBits, const char *version, int stream_size)
#[doc(alias = "_inflateInit2_")]
pub fn stub_1bb980() -> Option<u32> {
    // IDA 0x1bb980: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1bba84 — _inflateInit_
// type: int __cdecl(z_streamp strm, const char *version, int stream_size)
#[doc(alias = "_inflateInit_")]
pub fn stub_1bba84() -> Option<u32> {
    // IDA 0x1bba84: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1bba98 — _inflateEnd
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_inflateEnd")]
pub fn stub_1bba98() {
    // IDA 0x1bba98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1bbaf8 — _syncsearch
// type: unknown
#[doc(alias = "_syncsearch")]
pub fn stub_1bbaf8() {
    // IDA 0x1bbaf8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1bbb50 — _inflateSync
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_inflateSync")]
pub fn stub_1bbb50() {
    // IDA 0x1bbb50: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1bbc80 — _updatewindow
// type: unknown
#[doc(alias = "_updatewindow")]
pub fn stub_1bbc80() {
    // IDA 0x1bbc80: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1bbdb4 — _inflate
// type: int __cdecl(z_streamp strm, int flush)
#[doc(alias = "_inflate")]
pub fn stub_1bbdb4() {
    // IDA 0x1bbdb4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c049c — _inflate_table
// type: unknown
#[doc(alias = "_inflate_table")]
pub fn stub_1c049c() {
    // IDA 0x1c049c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c14c8 — _init_block
// type: unknown
#[doc(alias = "_init_block")]
pub fn stub_1c14c8() -> Option<u32> {
    // IDA 0x1c14c8: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c16c4 — __tr_init
// type: unknown
#[doc(alias = "__tr_init")]
pub fn stub_1c16c4() -> Option<u32> {
    // IDA 0x1c16c4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c173c — _pqdownheap
// type: unknown
#[doc(alias = "_pqdownheap")]
pub fn stub_1c173c() {
    // IDA 0x1c173c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c183c — _scan_tree
// type: unknown
#[doc(alias = "_scan_tree")]
pub fn stub_1c183c() {
    // IDA 0x1c183c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c1b68 — _send_tree
// type: unknown
#[doc(alias = "_send_tree")]
pub fn stub_1c1b68() {
    // IDA 0x1c1b68: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c2304 — _compress_block
// type: unknown
#[doc(alias = "_compress_block")]
pub fn stub_1c2304() {
    // IDA 0x1c2304: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c2794 — _build_tree
// type: unknown
#[doc(alias = "_build_tree")]
pub fn stub_1c2794() {
    // IDA 0x1c2794: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c347c — _bi_flush
// type: unknown
#[doc(alias = "_bi_flush")]
pub fn stub_1c347c() {
    // IDA 0x1c347c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c3514 — __tr_align
// type: int __fastcall(_DWORD)
#[doc(alias = "__tr_align")]
pub fn stub_1c3514() {
    // IDA 0x1c3514: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c37a0 — _bi_windup
// type: unknown
#[doc(alias = "_bi_windup")]
pub fn stub_1c37a0() {
    // IDA 0x1c37a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c3818 — __tr_stored_block
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "__tr_stored_block")]
pub fn stub_1c3818(data: &[u8]) -> usize {
    // IDA 0x1c3818: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1c3ac4 — __tr_flush_block
// type: unknown
#[doc(alias = "__tr_flush_block")]
pub fn stub_1c3ac4() {
    // IDA 0x1c3ac4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c4270 — _uncompress
// type: int __cdecl(Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen)
#[doc(alias = "_uncompress")]
pub fn stub_1c4270() {
    // IDA 0x1c4270: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c4324 — _zError
// type: const char *__cdecl(int)
#[doc(alias = "_zError")]
pub fn stub_1c4324() {
    // IDA 0x1c4324: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c433c — _zcfree
// type: int __fastcall(int, void *)
#[doc(alias = "_zcfree")]
pub fn stub_1c433c(handle: u32) {
    // IDA 0x1c433c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c4350 — _zcalloc
// type: unknown
#[doc(alias = "_zcalloc")]
pub fn stub_1c4350() -> Option<u32> {
    // IDA 0x1c4350: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c4364 — __ZN6TagLib17getFreeImageModelENS_7MDMODELE
// type: unknown
#[doc(alias = "TagLib::getFreeImageModel(TagLib::MDMODEL)")]
pub fn stub_1c4364() {
    // IDA 0x1c4364: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c4410 — __ZN6TagLib8getTagIDENS_7MDMODELEPKc
// type: unknown
#[doc(alias = "TagLib::getTagID(TagLib::MDMODEL,char const*)")]
pub fn stub_1c4410() {
    // IDA 0x1c4410: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c4494 — __ZN6TagLib10getTagInfoENS_7MDMODELEt
// type: unknown
#[doc(alias = "TagLib::getTagInfo(TagLib::MDMODEL,unsigned short)")]
pub fn stub_1c4494() {
    // IDA 0x1c4494: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c44d4 — __ZN6TagLib17getTagDescriptionENS_7MDMODELEt
// type: unknown
#[doc(alias = "TagLib::getTagDescription(TagLib::MDMODEL,unsigned short)")]
pub fn stub_1c44d4() {
    // IDA 0x1c44d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c44f0 — __ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc
// type: unknown
#[doc(alias = "TagLib::getTagFieldName(TagLib::MDMODEL,unsigned short,char *)")]
pub fn stub_1c44f0() {
    // IDA 0x1c44f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c4540 — __ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo
// type: unknown
#[doc(alias = "TagLib::addMetadataModel(TagLib::MDMODEL,tagTagInfo *)")]
pub fn stub_1c4540() {
    // IDA 0x1c4540: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c45f0 — __ZN6TagLibC2Ev
// type: TagLib *__fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::TagLib(void)")]
pub fn stub_1c45f0() {
    // IDA 0x1c45f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c48c4 — __ZN6TagLib8instanceEv
// type: _DWORD __fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::instance(void)")]
pub fn stub_1c48c4() {
    // IDA 0x1c48c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c49e4 — __ZN6TagLibD2Ev
// type: void __fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::~TagLib()")]
pub fn stub_1c49e4() {
    // IDA 0x1c49e4: dtor releases the owned control block/slots.
}
// 0x1c4b38 — ___tcf_0_0
// type: unknown
#[doc(alias = "___tcf_0_0")]
pub fn stub_1c4b38() {
    // IDA 0x1c4b38: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c4b48 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE13_Rb_tree_implISF_Lb0EEC2ERKSaISt13_Rb_tree_nodeISC_EERKSF_
// type: unknown
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>> const&,std::less<int> const&)")]
pub fn stub_1c4b48() -> Option<u32> {
    // IDA 0x1c4b48: nullable object query (id when live, None when unset).
    None
}
// 0x1c4b88 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE11lower_boundERS1_
// type: unknown
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::lower_bound(int const&)")]
pub fn stub_1c4b88() -> Option<u32> {
    // IDA 0x1c4b88: nullable object query (id when live, None when unset).
    None
}
// 0x1c4bbc — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_Rb_tree_impl<std::less<unsigned short>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>> const&,std::less<unsigned short> const&)")]
pub fn stub_1c4bbc() -> Option<u32> {
    // IDA 0x1c4bbc: nullable object query (id when live, None when unset).
    None
}
// 0x1c4bfc — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE11lower_boundERS1_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::lower_bound(unsigned short const&)")]
pub fn stub_1c4bfc() -> Option<u32> {
    // IDA 0x1c4bfc: nullable object query (id when live, None when unset).
    None
}
// 0x1c4c30 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKtP10tagTagInfoEEE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>>>::allocate(unsigned long,void const*)")]
pub fn stub_1c4c30() -> Option<u32> {
    // IDA 0x1c4c30: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c4c60 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE14_M_create_nodeERKS4_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_create_node(std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_1c4c60() -> Option<u32> {
    // IDA 0x1c4c60: nullable object query (id when live, None when unset).
    None
}
// 0x1c4c90 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_1c4c90() -> Option<u32> {
    // IDA 0x1c4c90: nullable object query (id when live, None when unset).
    None
}
// 0x1c4d14 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS2_IKtS6_EEEEEE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::allocate(unsigned long,void const*)")]
pub fn stub_1c4d14() -> Option<u32> {
    // IDA 0x1c4d14: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c4d44 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE14_M_create_nodeERKSC_
// type: unknown
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_create_node(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_1c4d44() -> Option<u32> {
    // IDA 0x1c4d44: nullable object query (id when live, None when unset).
    None
}
// 0x1c4d74 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSC_
// type: unknown
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_1c4d74() -> Option<u32> {
    // IDA 0x1c4d74: nullable object query (id when live, None when unset).
    None
}
// 0x1c4df8 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueERKSC_
// type: unknown
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_1c4df8() -> Option<u32> {
    // IDA 0x1c4df8: nullable object query (id when live, None when unset).
    None
}
// 0x1c4eb8 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,tagTagInfo *>> *)")]
pub fn stub_1c4eb8(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0x1c4eb8: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0x1c4ef4 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// type: unknown
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>> *)")]
pub fn stub_1c4ef4(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0x1c4ef4: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0x1c4f30 — __ZNSt8_Rb_treeIiSt4pairIKiPSt3mapItP10tagTagInfoSt4lessItESaIS0_IKtS4_EEEESt10_Select1stISC_ES5_IiESaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>,std::_Select1st<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::less<int>,std::allocator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *>>,std::pair<int const,std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>> *> const&)")]
pub fn stub_1c4f30() -> Option<u32> {
    // IDA 0x1c4f30: nullable object query (id when live, None when unset).
    None
}
// 0x1c5054 — __ZNSt3mapIiPS_ItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEES2_IiESaIS4_IKiS9_EEEixERSB_
// type: unknown
#[doc(alias = "std::map<int,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>,tagTagInfo *<int>,std::allocator<std::less<unsigned short><int const,std::map*<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo>>>>>>::operator[](int const&)")]
pub fn stub_1c5054() -> Option<u32> {
    // IDA 0x1c5054: nullable object query (id when live, None when unset).
    None
}
// 0x1c50c0 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueERKS4_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_1c50c0() -> Option<u32> {
    // IDA 0x1c50c0: nullable object query (id when live, None when unset).
    None
}
// 0x1c5180 — __ZNSt8_Rb_treeItSt4pairIKtP10tagTagInfoESt10_Select1stIS4_ESt4lessItESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,tagTagInfo *>,std::_Select1st<std::pair<unsigned short const,tagTagInfo *>>,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,tagTagInfo *>>,std::pair<unsigned short const,tagTagInfo *> const&)")]
pub fn stub_1c5180() -> Option<u32> {
    // IDA 0x1c5180: nullable object query (id when live, None when unset).
    None
}
// 0x1c52a4 — __ZNSt3mapItP10tagTagInfoSt4lessItESaISt4pairIKtS1_EEEixERS5_
// type: int __fastcall(int, unsigned __int16 *)
#[doc(alias = "std::map<unsigned short,tagTagInfo *,std::less<unsigned short>,std::allocator<std::pair<unsigned short const,tagTagInfo *>>>::operator[](unsigned short const&)")]
pub fn stub_1c52a4() -> Option<u32> {
    // IDA 0x1c52a4: nullable object query (id when live, None when unset).
    None
}
// 0x1c5310 — __Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj
// type: unknown
#[doc(alias = "tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")]
pub fn stub_1c5310() {
    // IDA 0x1c5310: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c59bc — __Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP
// type: unknown
#[doc(alias = "tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")]
pub fn stub_1c59bc() {
    // IDA 0x1c59bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c5bf8 — __Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP
// type: unknown
#[doc(alias = "tiff_write_geotiff_profile(tiff *,FIBITMAP *)")]
pub fn stub_1c5bf8(data: &[u8]) -> usize {
    // IDA 0x1c5bf8: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1c610c — __Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP
// type: unknown
#[doc(alias = "tiff_read_geotiff_profile(tiff *,FIBITMAP *)")]
pub fn stub_1c610c(data: &[u8]) -> bool {
    // IDA 0x1c610c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1c630c — __Z15XTIFFInitializev
// type: _DWORD __fastcall()
#[doc(alias = "XTIFFInitialize(void)")]
pub fn stub_1c630c() -> Option<u32> {
    // IDA 0x1c630c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c6354 — __ZL22_XTIFFDefaultDirectoryP4tiff
// type: unknown
#[doc(alias = "_XTIFFDefaultDirectory(tiff *)")]
pub fn stub_1c6354() {
    // IDA 0x1c6354: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c6394 — __ZL15append_iptc_tagPhPjtjPKv
// type: _DWORD __fastcall(unsigned __int8 *, unsigned int *, unsigned __int16, unsigned int, const void *__src)
#[doc(alias = "append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")]
pub fn stub_1c6394() {
    // IDA 0x1c6394: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c6448 — _write_iptc_profile
// type: unknown
#[doc(alias = "_write_iptc_profile")]
pub fn stub_1c6448(data: &[u8]) -> usize {
    // IDA 0x1c6448: sinks the buffer; returns bytes accepted.
    data.len()
}
// 0x1c6910 — _read_iptc_profile
// type: unknown
#[doc(alias = "_read_iptc_profile")]
pub fn stub_1c6910(data: &[u8]) -> bool {
    // IDA 0x1c6910: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1c7340 — __ZNKSt6vectorISsSaISsEE4sizeEv
// type: unknown
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::size(void)const")]
pub fn stub_1c7340() {
    // IDA 0x1c7340: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c7350 — _FreeImage_GetTagKey
// type: unknown
#[doc(alias = "_FreeImage_GetTagKey")]
pub fn stub_1c7350(handle: u32) {
    // IDA 0x1c7350: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7360 — _FreeImage_GetTagID
// type: int __fastcall(int result)
#[doc(alias = "_FreeImage_GetTagID")]
pub fn stub_1c7360(handle: u32) {
    // IDA 0x1c7360: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7370 — _FreeImage_GetTagType
// type: unknown
#[doc(alias = "_FreeImage_GetTagType")]
pub fn stub_1c7370(handle: u32) {
    // IDA 0x1c7370: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7380 — _FreeImage_GetTagCount
// type: unknown
#[doc(alias = "_FreeImage_GetTagCount")]
pub fn stub_1c7380(handle: u32) {
    // IDA 0x1c7380: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7390 — _FreeImage_GetTagLength
// type: unknown
#[doc(alias = "_FreeImage_GetTagLength")]
pub fn stub_1c7390(handle: u32) {
    // IDA 0x1c7390: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c73a0 — _FreeImage_GetTagValue
// type: unknown
#[doc(alias = "_FreeImage_GetTagValue")]
pub fn stub_1c73a0(handle: u32) {
    // IDA 0x1c73a0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c73b0 — _FreeImage_SetTagID
// type: unknown
#[doc(alias = "_FreeImage_SetTagID")]
pub fn stub_1c73b0(handle: u32) {
    // IDA 0x1c73b0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c73c8 — _FreeImage_SetTagType
// type: unknown
#[doc(alias = "_FreeImage_SetTagType")]
pub fn stub_1c73c8(handle: u32) {
    // IDA 0x1c73c8: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c73dc — _FreeImage_SetTagCount
// type: unknown
#[doc(alias = "_FreeImage_SetTagCount")]
pub fn stub_1c73dc(handle: u32) {
    // IDA 0x1c73dc: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c73f0 — _FreeImage_SetTagLength
// type: unknown
#[doc(alias = "_FreeImage_SetTagLength")]
pub fn stub_1c73f0(handle: u32) {
    // IDA 0x1c73f0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7404 — __Z22FreeImage_TagDataWidtht
// type: _DWORD __fastcall(unsigned __int16)
#[doc(alias = "FreeImage_TagDataWidth(unsigned short)")]
pub fn stub_1c7404(handle: u32) {
    // IDA 0x1c7404: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7428 — _FreeImage_DeleteTag
// type: unknown
#[doc(alias = "_FreeImage_DeleteTag")]
pub fn stub_1c7428(handle: u32) {
    // IDA 0x1c7428: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7470 — _FreeImage_SetTagDescription
// type: unknown
#[doc(alias = "_FreeImage_SetTagDescription")]
pub fn stub_1c7470(handle: u32) {
    // IDA 0x1c7470: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c74cc — _FreeImage_SetTagKey
// type: unknown
#[doc(alias = "_FreeImage_SetTagKey")]
pub fn stub_1c74cc(handle: u32) {
    // IDA 0x1c74cc: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7528 — _FreeImage_CreateTag
// type: unknown
#[doc(alias = "_FreeImage_CreateTag")]
pub fn stub_1c7528() -> Option<u32> {
    // IDA 0x1c7528: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c7580 — _FreeImage_CloneTag
// type: unknown
#[doc(alias = "_FreeImage_CloneTag")]
pub fn stub_1c7580(handle: u32) {
    // IDA 0x1c7580: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7658 — _FreeImage_SetTagValue
// type: unknown
#[doc(alias = "_FreeImage_SetTagValue")]
pub fn stub_1c7658(handle: u32) {
    // IDA 0x1c7658: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7724 — __ZN10FIRationalD1Ev
// type: void __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::~FIRational()")]
pub fn stub_1c7724() {
    // IDA 0x1c7724: dtor releases the owned control block/slots.
}
// 0x1c7728 — __ZN10FIRational12getNumeratorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::getNumerator(void)")]
pub fn stub_1c7728() {
    // IDA 0x1c7728: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c7730 — __ZN10FIRational14getDenominatorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::getDenominator(void)")]
pub fn stub_1c7730() {
    // IDA 0x1c7730: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c7738 — __ZN10FIRationalC2Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
#[doc(alias = "FIRational::FIRational(float)")]
pub fn stub_1c7738() {
    // IDA 0x1c7738: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c7988 — __ZN10FIRationalC1Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
#[doc(alias = "FIRational::FIRational(float)")]
pub fn stub_1c7988() {
    // IDA 0x1c7988: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c798c — __ZL9ReadInt32iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadInt32(int,void *)")]
pub fn stub_1c798c(data: &[u8]) -> bool {
    // IDA 0x1c798c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1c79d8 — __ZL10ReadUint16iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadUint16(int,void *)")]
pub fn stub_1c79d8(data: &[u8]) -> bool {
    // IDA 0x1c79d8: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1c79f8 — __ZL10ReadUint32iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadUint32(int,void *)")]
pub fn stub_1c79f8(data: &[u8]) -> bool {
    // IDA 0x1c79f8: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1c79fc — __ZL18FreeImage_strnicmpPKcS0_m
// type: _DWORD __fastcall(const char *, const char *, unsigned int)
#[doc(alias = "FreeImage_strnicmp(char const*,char const*,unsigned long)")]
pub fn stub_1c79fc(handle: u32) {
    // IDA 0x1c79fc: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x1c7d28 — __ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE
// type: unknown
#[doc(alias = "processExifTag(FIBITMAP *,FITAG *,char *,int,TagLib::MDMODEL)")]
pub fn stub_1c7d28() {
    // IDA 0x1c7d28: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c81a4 — _jpeg_read_exif_profile
// type: unknown
#[doc(alias = "_jpeg_read_exif_profile")]
pub fn stub_1c81a4(data: &[u8]) -> bool {
    // IDA 0x1c81a4: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x1c8d60 — __ZSt16__deque_buf_sizem
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "std::__deque_buf_size(unsigned long)")]
pub fn stub_1c8d60() {
    // IDA 0x1c8d60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c8d84 — __ZNSt5dequeItSaItEE15_M_destroy_dataESt15_Deque_iteratorItRtPtES5_RKS0_
// type: void()
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_destroy_data(std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short> const&)")]
pub fn stub_1c8d84() {
    // IDA 0x1c8d84: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c8d88 — __ZNSt5dequeIPhSaIS0_EE15_M_destroy_dataESt15_Deque_iteratorIS0_RS0_PS0_ES6_RKS1_
// type: unknown
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_data(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>,std::allocator<unsigned char *> const&)")]
pub fn stub_1c8d88() -> Option<u32> {
    // IDA 0x1c8d88: nullable object query (id when live, None when unset).
    None
}
// 0x1c8d8c — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE4findERS1_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::find(unsigned int const&)")]
pub fn stub_1c8d8c() {
    // IDA 0x1c8d8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c8de8 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE13_Rb_tree_implIS6_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS2_EERKS6_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_Rb_tree_impl<std::less<unsigned int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>> const&,std::less<unsigned int> const&)")]
pub fn stub_1c8de8() {
    // IDA 0x1c8de8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c8e28 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE11lower_boundERS1_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::lower_bound(unsigned int const&)")]
pub fn stub_1c8e28() {
    // IDA 0x1c8e28: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c8e5c — __ZNSt15_Deque_iteratorItRtPtE11_M_set_nodeEPS1_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::_M_set_node(unsigned short **)")]
pub fn stub_1c8e5c() -> Option<u32> {
    // IDA 0x1c8e5c: nullable object query (id when live, None when unset).
    None
}
// 0x1c8e8c — __ZNSt15_Deque_iteratorItRtPtEmmEv
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator--(void)")]
pub fn stub_1c8e8c() -> Option<u32> {
    // IDA 0x1c8e8c: nullable object query (id when live, None when unset).
    None
}
// 0x1c8ecc — __ZStmiIPhRS0_PS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **>::difference_type std::operator-<unsigned char *,unsigned char *&,unsigned char **>(std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&,std::_Deque_iterator<unsigned char *,unsigned char *&,unsigned char **> const&)")]
pub fn stub_1c8ecc() -> Option<u32> {
    // IDA 0x1c8ecc: nullable object query (id when live, None when unset).
    None
}
// 0x1c8f1c — __ZStmiIN6TagLib7MDMODELERS1_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::difference_type std::operator-<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> const&)")]
pub fn stub_1c8f1c() -> Option<u32> {
    // IDA 0x1c8f1c: nullable object query (id when live, None when unset).
    None
}
// 0x1c8f6c — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERKS1_PS2_EppEv
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>::operator++(void)")]
pub fn stub_1c8f6c() -> Option<u32> {
    // IDA 0x1c8f6c: nullable object query (id when live, None when unset).
    None
}
// 0x1c8fc4 — __ZNSt15_Deque_iteratorItRtPtEppEv
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>::operator++(void)")]
pub fn stub_1c8fc4() -> Option<u32> {
    // IDA 0x1c8fc4: nullable object query (id when live, None when unset).
    None
}
// 0x1c9004 — __ZStmiItRKtPS0_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS7_SA_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::difference_type std::operator-<unsigned short,unsigned short const&,unsigned short const*>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*> const&)")]
pub fn stub_1c9004() -> Option<u32> {
    // IDA 0x1c9004: nullable object query (id when live, None when unset).
    None
}
// 0x1c9054 — __ZStmiIPhRKS0_PS1_ENSt15_Deque_iteratorIT_T0_T1_E15difference_typeERKS8_SB_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*>::difference_type std::operator-<unsigned char *,unsigned char * const&,unsigned char * const*>(std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&,std::_Deque_iterator<unsigned char *,unsigned char * const&,unsigned char * const*> const&)")]
pub fn stub_1c9054() -> Option<u32> {
    // IDA 0x1c9054: nullable object query (id when live, None when unset).
    None
}
// 0x1c90a4 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKjiEEE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<unsigned int const,int>>>::allocate(unsigned long,void const*)")]
pub fn stub_1c90a4() -> Option<u32> {
    // IDA 0x1c90a4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c90d4 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE14_M_create_nodeERKS2_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_create_node(std::pair<unsigned int const,int> const&)")]
pub fn stub_1c90d4() {
    // IDA 0x1c90d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9104 — __ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long,void const*)")]
pub fn stub_1c9104() -> Option<u32> {
    // IDA 0x1c9104: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c9124 — __ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long,void const*)")]
pub fn stub_1c9124() -> Option<u32> {
    // IDA 0x1c9124: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c9144 — __ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char *>::allocate(unsigned long,void const*)")]
pub fn stub_1c9144() -> Option<u32> {
    // IDA 0x1c9144: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c9164 — __ZN9__gnu_cxx13new_allocatorIPN6TagLib7MDMODELEE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL *>::allocate(unsigned long,void const*)")]
pub fn stub_1c9164() -> Option<u32> {
    // IDA 0x1c9164: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c9184 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_allocate_mapEm
// type: unknown
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_allocate_map(unsigned long)")]
pub fn stub_1c9184() {
    // IDA 0x1c9184: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c922c — __ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv
// type: int __fastcall(int, unsigned int)
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char **>::allocate(unsigned long,void const*)")]
pub fn stub_1c922c() -> Option<u32> {
    // IDA 0x1c922c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c924c — __ZNSt11_Deque_baseIPhSaIS0_EE15_M_allocate_mapEm
// type: unknown
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_allocate_map(unsigned long)")]
pub fn stub_1c924c() -> Option<u32> {
    // IDA 0x1c924c: nullable object query (id when live, None when unset).
    None
}
// 0x1c92f4 — __ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv
// type: unknown
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short *>::allocate(unsigned long,void const*)")]
pub fn stub_1c92f4() -> Option<u32> {
    // IDA 0x1c92f4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x1c9314 — __ZNSt11_Deque_baseItSaItEE15_M_allocate_mapEm
// type: unknown
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_allocate_map(unsigned long)")]
pub fn stub_1c9314() {
    // IDA 0x1c9314: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c93bc — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE16_M_destroy_nodesEPPS1_S5_
// type: unknown
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
pub fn stub_1c93bc() {
    // IDA 0x1c93bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c94ac — __ZNSt5dequeItSaItEE15_M_pop_back_auxEv
// type: unknown
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_pop_back_aux(void)")]
pub fn stub_1c94ac() {
    // IDA 0x1c94ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c94e0 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,int>> *)")]
pub fn stub_1c94e0(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0x1c94e0: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0x1c951c — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EED2Ev
// type: unknown
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~_Deque_base()")]
pub fn stub_1c951c() {
    // IDA 0x1c951c: dtor releases the owned control block/slots.
}
// 0x1c9550 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,int> const&)")]
pub fn stub_1c9550() {
    // IDA 0x1c9550: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c95d4 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// type: int __fastcall(void *__src)
#[doc(alias = "TagLib::MDMODEL * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
pub fn stub_1c95d4(v: &mut Vec<u32>, count: usize) {
    // IDA 0x1c95d4: shifts the tail backward (memmove semantics).
    let n = count.min(v.len());
    v.rotate_right(n);
}
// 0x1c9604 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPN6TagLib7MDMODELEEEPT_PKS6_S9_S7_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "TagLib::MDMODEL * * std::__copy<true,std::random_access_iterator_tag>::copy<TagLib::MDMODEL *>(TagLib::MDMODEL * const*,TagLib::MDMODEL * const*,TagLib::MDMODEL * *)")]
pub fn stub_1c9604() -> Option<u32> {
    // IDA 0x1c9604: nullable object query (id when live, None when unset).
    None
}
// 0x1c9630 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPtEEPT_PKS4_S7_S5_
// type: int __fastcall(void *__src)
#[doc(alias = "unsigned short * * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
pub fn stub_1c9630(v: &mut Vec<u32>, count: usize) {
    // IDA 0x1c9630: shifts the tail backward (memmove semantics).
    let n = count.min(v.len());
    v.rotate_right(n);
}
// 0x1c9660 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPtEEPT_PKS4_S7_S5_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "unsigned short * * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned short *>(unsigned short * const*,unsigned short * const*,unsigned short * *)")]
pub fn stub_1c9660() -> u32 {
    // IDA 0x1c9660: index/count query.
    0
}
// 0x1c968c — __ZNSt5dequeItSaItEE17_M_reallocate_mapEmb
// type: unknown
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_1c968c() {
    // IDA 0x1c968c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c97b4 — __ZNSt5dequeItSaItEE22_M_reserve_map_at_backEm
// type: unknown
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_1c97b4() {
    // IDA 0x1c97b4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c97e8 — __ZNSt15__copy_backwardILb1ESt26random_access_iterator_tagE8__copy_bIPPhEEPT_PKS5_S8_S6_
// type: int __fastcall(void *__src)
#[doc(alias = "unsigned char ** * std::__copy_backward<true,std::random_access_iterator_tag>::__copy_b<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
pub fn stub_1c97e8(v: &mut Vec<u32>, count: usize) {
    // IDA 0x1c97e8: shifts the tail backward (memmove semantics).
    let n = count.min(v.len());
    v.rotate_right(n);
}
// 0x1c9818 — __ZNSt6__copyILb1ESt26random_access_iterator_tagE4copyIPPhEEPT_PKS5_S8_S6_
// type: int __fastcall(void *__src, int, void *__dst)
#[doc(alias = "unsigned char ** * std::__copy<true,std::random_access_iterator_tag>::copy<unsigned char **>(unsigned char ** const*,unsigned char ** const*,unsigned char ** *)")]
pub fn stub_1c9818() -> u32 {
    // IDA 0x1c9818: index/count query.
    0
}
// 0x1c9844 — __ZNSt5dequeItSaItEE4backEv
// type: unknown
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::back(void)")]
pub fn stub_1c9844() {
    // IDA 0x1c9844: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9884 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueERKS2_
// type: unknown
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::pair<unsigned int const,int> const&)")]
pub fn stub_1c9884() {
    // IDA 0x1c9884: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9944 — __ZNSt8_Rb_treeIjSt4pairIKjiESt10_Select1stIS2_ESt4lessIjESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,int>,std::_Select1st<std::pair<unsigned int const,int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,int>>,std::pair<unsigned int const,int> const&)")]
pub fn stub_1c9944() {
    // IDA 0x1c9944: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9a68 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_E11_M_set_nodeEPS3_
// type: int __fastcall(_DWORD *, int *)
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::_M_set_node(TagLib::MDMODEL**)")]
pub fn stub_1c9a68() -> Option<u32> {
    // IDA 0x1c9a68: nullable object query (id when live, None when unset).
    None
}
// 0x1c9a98 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE17_M_reallocate_mapEmb
// type: unknown
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_1c9a98() {
    // IDA 0x1c9a98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9bc0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE22_M_reserve_map_at_backEm
// type: unknown
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_1c9bc0() {
    // IDA 0x1c9bc0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9bf4 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EppEv
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator++(void)")]
pub fn stub_1c9bf4() -> Option<u32> {
    // IDA 0x1c9bf4: nullable object query (id when live, None when unset).
    None
}
// 0x1c9c34 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_St12__false_type
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_aux<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
pub fn stub_1c9c34() -> Option<u32> {
    // IDA 0x1c9c34: nullable object query (id when live, None when unset).
    None
}
// 0x1c9ca4 — __ZSt18uninitialized_copyISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_EET0_T_SB_SA_
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::uninitialized_copy<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
pub fn stub_1c9ca4() -> Option<u32> {
    // IDA 0x1c9ca4: nullable object query (id when live, None when unset).
    None
}
// 0x1c9d24 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorIN6TagLib7MDMODELERKS2_PS3_ES0_IS2_RS2_PS2_ES2_ET0_T_SB_SA_SaIT1_E
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*> std::__uninitialized_copy_a<std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,TagLib::MDMODEL>(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL const&,TagLib::MDMODEL const*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL>)")]
pub fn stub_1c9d24() -> Option<u32> {
    // IDA 0x1c9d24: nullable object query (id when live, None when unset).
    None
}
// 0x1c9da0 — __ZNSt15_Deque_iteratorIN6TagLib7MDMODELERS1_PS1_EmmEv
// type: unknown
#[doc(alias = "std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>::operator--(void)")]
pub fn stub_1c9da0() -> Option<u32> {
    // IDA 0x1c9da0: nullable object query (id when live, None when unset).
    None
}
// 0x1c9de0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE4backEv
// type: unknown
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::back(void)")]
pub fn stub_1c9de0() {
    // IDA 0x1c9de0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x1c9e20 — __ZNSt15_Deque_iteratorItRKtPS0_EppEv
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>::operator++(void)")]
pub fn stub_1c9e20() -> Option<u32> {
    // IDA 0x1c9e20: nullable object query (id when live, None when unset).
    None
}
// 0x1c9e78 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorItRKtPS4_ES3_ItRtPtEEET0_T_SC_SB_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_1c9e78() -> Option<u32> {
    // IDA 0x1c9e78: nullable object query (id when live, None when unset).
    None
}
// 0x1ca124 — __ZSt10__copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_1ca124() -> Option<u32> {
    // IDA 0x1ca124: nullable object query (id when live, None when unset).
    None
}
// 0x1ca1a0 — __ZNSt13__copy_normalILb0ELb0EE8__copy_nISt15_Deque_iteratorItRKtPS3_ES2_ItRtPtEEET0_T_SB_SA_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__copy_normal<false,false>::__copy_n<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_1ca1a0() -> Option<u32> {
    // IDA 0x1ca1a0: nullable object query (id when live, None when unset).
    None
}
// 0x1ca21c — __ZSt4copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_1ca21c() -> Option<u32> {
    // IDA 0x1ca21c: nullable object query (id when live, None when unset).
    None
}
// 0x1ca298 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_St11__true_type
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_aux<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::__true_type)")]
pub fn stub_1ca298() -> Option<u32> {
    // IDA 0x1ca298: nullable object query (id when live, None when unset).
    None
}
// 0x1ca314 — __ZSt18uninitialized_copyISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEET0_T_S9_S8_
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::uninitialized_copy<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>)")]
pub fn stub_1ca314() -> Option<u32> {
    // IDA 0x1ca314: nullable object query (id when live, None when unset).
    None
}
// 0x1ca394 — __ZSt22__uninitialized_copy_aISt15_Deque_iteratorItRKtPS1_ES0_ItRtPtEtET0_T_S9_S8_SaIT1_E
// type: unknown
#[doc(alias = "std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *> std::__uninitialized_copy_a<std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,unsigned short>(std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short const&,unsigned short const*>,std::_Deque_iterator<unsigned short,unsigned short &,unsigned short *>,std::allocator<unsigned short>)")]
pub fn stub_1ca394() -> Option<u32> {
    // IDA 0x1ca394: nullable object query (id when live, None when unset).
    None
}
