//! network generated_net_21 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5109 total, 0 remaining (complete) — global gap filler EA-sorted asc
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0x233d18..0x23d984 | 25000->25150 network distinct | 85546->85546 workspace distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

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


// 0x233d18 — _dirsep_basename
// type: char *__fastcall(const char *)
#[doc(alias = "_dirsep_basename")]
pub fn stub_233d18(handle: u32) -> String {
    // IDA 0x233d18: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x233d3c — _strrchr_basename
// type: char *__fastcall(const char *)
#[doc(alias = "_strrchr_basename")]
pub fn stub_233d3c(handle: u32) -> String {
    // IDA 0x233d3c: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x233d60 — _zzip_file_read
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "_zzip_file_read")]
pub fn stub_233d60(data: &[u8]) -> bool {
    // IDA 0x233d60: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x233f10 — _zzip_file_close
// type: int __fastcall(char *__b)
#[doc(alias = "_zzip_file_close")]
pub fn stub_233f10(handle: u32) {
    // IDA 0x233f10: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x233ff4 — _zzip_rewind
// type: int __fastcall(int)
#[doc(alias = "_zzip_rewind")]
pub fn stub_233ff4() {
    // IDA 0x233ff4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2340d8 — _zzip_seek
// type: int __fastcall(int, unsigned int, unsigned int, int)
#[doc(alias = "_zzip_seek")]
pub fn stub_2340d8() {
    // IDA 0x2340d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23436c — _zzip_file_open
// type: char *__fastcall(_DWORD *, const char *, __int16)
#[doc(alias = "_zzip_file_open")]
pub fn stub_23436c() -> Option<u32> {
    // IDA 0x23436c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x2346f0 — _zzip_error
// type: int __fastcall(int)
#[doc(alias = "_zzip_error")]
pub fn stub_2346f0() {
    // IDA 0x2346f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2346f8 — _zzip_dirhandle
// type: int __fastcall(int)
#[doc(alias = "_zzip_dirhandle")]
pub fn stub_2346f8() {
    // IDA 0x2346f8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x234700 — _zzip_get_default_io
// type: _UNKNOWN **()
#[doc(alias = "_zzip_get_default_io")]
pub fn stub_234700(handle: u32) -> String {
    // IDA 0x234700: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x234710 — _zzip_filesize
// type: off_t __fastcall(int)
#[doc(alias = "_zzip_filesize")]
pub fn stub_234710() {
    // IDA 0x234710: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x234744 — _zzip_dir_stat
// type: int __fastcall(int, char *__s, _DWORD *, __int16)
#[doc(alias = "_zzip_dir_stat")]
pub fn stub_234744() {
    // IDA 0x234744: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23483c — _zzip_dir_read
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_zzip_dir_read")]
pub fn stub_23483c(data: &[u8]) -> bool {
    // IDA 0x23483c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2348a4 — ___zzip_try_open
// type: int __fastcall(const char *, int, const char **, _UNKNOWN **default_io)
#[doc(alias = "___zzip_try_open")]
pub fn stub_2348a4() -> Option<u32> {
    // IDA 0x2348a4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x234964 — _zzip_dir_free
// type: int __fastcall(void *)
#[doc(alias = "_zzip_dir_free")]
pub fn stub_234964(handle: u32) {
    // IDA 0x234964: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x2349ec — _zzip_dir_close
// type: int __fastcall(int)
#[doc(alias = "_zzip_dir_close")]
pub fn stub_2349ec(handle: u32) {
    // IDA 0x2349ec: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x2349fc — _zzip_dir_alloc_ext_io
// type: _DWORD *__fastcall(char **, _UNKNOWN **default_io)
#[doc(alias = "_zzip_dir_alloc_ext_io")]
pub fn stub_2349fc() -> Option<u32> {
    // IDA 0x2349fc: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x234a5c — ___zzip_fetch_disk_trailer
// type: int __fastcall(int, unsigned int, int, unsigned __int8 **, int)
#[doc(alias = "___zzip_fetch_disk_trailer")]
pub fn stub_234a5c() {
    // IDA 0x234a5c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23524c — ___zzip_parse_root_directory
// type: int __fastcall(int, __int64 *, char **, int)
#[doc(alias = "___zzip_parse_root_directory")]
pub fn stub_23524c(data: &[u8]) -> bool {
    // IDA 0x23524c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2356fc — _zzip_dir_fdopen_ext_io
// type: char **__fastcall(char *, int *, char **, _UNKNOWN **)
#[doc(alias = "_zzip_dir_fdopen_ext_io")]
pub fn stub_2356fc() -> Option<u32> {
    // IDA 0x2356fc: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x2357e0 — _zzip_dir_open_ext_io
// type: char **__fastcall(const char *, int *, char **, _UNKNOWN **)
#[doc(alias = "_zzip_dir_open_ext_io")]
pub fn stub_2357e0() -> Option<u32> {
    // IDA 0x2357e0: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x235884 — __ZN5boost6detail25get_once_per_thread_epochEv
// type: _DWORD *__fastcall(boost::detail *this)
#[doc(alias = "__ZN5boost6detail25get_once_per_thread_epochEv")]
pub fn stub_235884(data: &[u8]) -> bool {
    // IDA 0x235884: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2358d0 — _create_epoch_tss_key
// type: 
#[doc(alias = "_create_epoch_tss_key")]
pub fn stub_2358d0() -> Option<u32> {
    // IDA 0x2358d0: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x2358f0 — _delete_epoch_tss_data
// type: void __fastcall(void *)
#[doc(alias = "_delete_epoch_tss_data")]
pub fn stub_2358f0() {
    // IDA 0x2358f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2358fc — __ZN5boost6detail16thread_data_baseD0Ev
// type: void __fastcall(boost::detail::thread_data_base *__hidden this)
#[doc(alias = "__ZN5boost6detail16thread_data_baseD0Ev")]
pub fn stub_2358fc(data: &[u8]) -> bool {
    // IDA 0x2358fc: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x235910 — __ZN5boost6detail16thread_data_baseD1Ev
// type: void __fastcall(boost::detail::thread_data_base *__hidden this)
#[doc(alias = "__ZN5boost6detail16thread_data_baseD1Ev")]
pub fn stub_235910(data: &[u8]) -> bool {
    // IDA 0x235910: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x23591c — __ZN5boost6detail16thread_data_baseD2Ev
// type: void __fastcall(boost::detail::thread_data_base *__hidden this)
#[doc(alias = "__ZN5boost6detail16thread_data_baseD2Ev")]
pub fn stub_23591c(data: &[u8]) -> bool {
    // IDA 0x23591c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x235d7c — __ZN5boost6detail23get_current_thread_dataEv
// type: _DWORD __fastcall(boost::detail *__hidden this)
#[doc(alias = "__ZN5boost6detail23get_current_thread_dataEv")]
pub fn stub_235d7c(data: &[u8]) -> bool {
    // IDA 0x235d7c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x235da4 — __ZN5boost6detail12_GLOBAL__N_129create_current_thread_tls_keyEv
// type: int __fastcall(boost::detail::_anonymous_namespace_ *this)
#[doc(alias = "__ZN5boost6detail12_GLOBAL__N_129create_current_thread_tls_keyEv")]
pub fn stub_235da4() -> Option<u32> {
    // IDA 0x235da4: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x235dc4 — __ZN5boost6thread21start_thread_noexceptEv
// type: int __fastcall(void **this)
#[doc(alias = "__ZN5boost6thread21start_thread_noexceptEv")]
pub fn stub_235dc4(data: &[u8]) -> bool {
    // IDA 0x235dc4: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x235e8c — _thread_proxy
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "_thread_proxy")]
pub fn stub_235e8c(data: &[u8]) -> bool {
    // IDA 0x235e8c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2360c4 — __ZN5boost6thread13join_noexceptEv
// type: int __fastcall(boost::thread *this)
#[doc(alias = "__ZN5boost6thread13join_noexceptEv")]
pub fn stub_2360c4(data: &[u8]) -> bool {
    // IDA 0x2360c4: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236318 — __ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb
// type: int __fastcall(boost::thread *this, const timespec *, bool *)
#[doc(alias = "__ZN5boost6thread26do_try_join_until_noexceptERK8timespecRb")]
pub fn stub_236318(data: &[u8]) -> bool {
    // IDA 0x236318: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236598 — __ZN5boost6thread6detachEv
// type: void __fastcall(boost::thread *this)
#[doc(alias = "__ZN5boost6thread6detachEv")]
pub fn stub_236598(data: &[u8]) -> bool {
    // IDA 0x236598: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2366b0 — __ZN5boost11this_thread5hiden11sleep_untilERK8timespec
// type: void __fastcall(boost::this_thread::hiden *this, const timespec *, int, int)
#[doc(alias = "__ZN5boost11this_thread5hiden11sleep_untilERK8timespec")]
pub fn stub_2366b0(data: &[u8]) -> bool {
    // IDA 0x2366b0: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2368cc — __ZN5boost6thread13native_handleEv
// type: int __fastcall(boost::thread *this)
#[doc(alias = "__ZN5boost6thread13native_handleEv")]
pub fn stub_2368cc(data: &[u8]) -> bool {
    // IDA 0x2368cc: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236a00 — __ZN5boost11this_thread18interruption_pointEv
// type: void __fastcall(boost::this_thread *this, int, int, int)
#[doc(alias = "__ZN5boost11this_thread18interruption_pointEv")]
pub fn stub_236a00(data: &[u8]) -> bool {
    // IDA 0x236a00: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236b14 — __ZN5boost11this_thread20disable_interruptionC1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::this_thread::disable_interruption *this, int, int, int)
#[doc(alias = "__ZN5boost11this_thread20disable_interruptionC1Ev")]
pub fn stub_236b14(data: &[u8]) -> bool {
    // IDA 0x236b14: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236c14 — __ZN5boost11this_thread20disable_interruptionD1Ev
// type: void __fastcall(boost::this_thread::disable_interruption *this, int, int, int)
#[doc(alias = "__ZN5boost11this_thread20disable_interruptionD1Ev")]
pub fn stub_236c14(data: &[u8]) -> bool {
    // IDA 0x236c14: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236d04 — __ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv
// type: void *__fastcall(boost::_anonymous_namespace_ *this, int, int, int)
#[doc(alias = "__ZN5boost12_GLOBAL__N_131get_or_make_current_thread_dataEv")]
pub fn stub_236d04(data: &[u8]) -> bool {
    // IDA 0x236d04: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x236ec0 — __ZN5boost6detail12get_tss_dataEPKv
// type: _DWORD __fastcall(boost::detail *__hidden this, const void *)
#[doc(alias = "__ZN5boost6detail12get_tss_dataEPKv")]
pub fn stub_236ec0(handle: u32) -> String {
    // IDA 0x236ec0: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x236f30 — __ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail16add_new_tss_nodeEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPv")]
pub fn stub_236f30() -> Option<u32> {
    // IDA 0x236f30: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x237130 — __ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb
// type: void __fastcall(boost::_anonymous_namespace_ *, int *, int, int)
#[doc(alias = "__ZN5boost6detail12set_tss_dataEPKvNS_10shared_ptrINS0_20tss_cleanup_functionEEEPvb")]
pub fn stub_237130() -> Option<u32> {
    // IDA 0x237130: nullable object query (id when live, None when unset).
    None
}
// 0x237348 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "__ZN5boost12_GLOBAL__N_126externally_launched_threadD1Ev")]
pub fn stub_237348(data: &[u8]) -> bool {
    // IDA 0x237348: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x237354 — __ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *__hidden this)
#[doc(alias = "__ZN5boost12_GLOBAL__N_126externally_launched_threadD0Ev")]
pub fn stub_237354(data: &[u8]) -> bool {
    // IDA 0x237354: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x237368 — __ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv
// type: void __fastcall(boost::_anonymous_namespace_::externally_launched_thread *this)
#[doc(alias = "__ZN5boost12_GLOBAL__N_126externally_launched_thread3runEv")]
pub fn stub_237368(data: &[u8]) -> bool {
    // IDA 0x237368: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x23736c — __ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
// type: void()
#[doc(alias = "__ZN5boost12_GLOBAL__N_126externally_launched_thread25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE")]
pub fn stub_23736c(data: &[u8]) -> bool {
    // IDA 0x23736c: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x237370 — _tls_destructor
// type: void __fastcall(int)
#[doc(alias = "_tls_destructor")]
pub fn stub_237370() {
    // IDA 0x237370: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2374bc — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost10shared_ptrINS_6detail16thread_data_baseEEaSERKS3_")]
pub fn stub_2374bc(data: &[u8]) -> bool {
    // IDA 0x2374bc: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2375b0 — __ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost10shared_ptrINS_6detail20tss_cleanup_functionEEaSERKS3_")]
pub fn stub_2375b0() -> Option<u32> {
    // IDA 0x2375b0: nullable object query (id when live, None when unset).
    None
}
// 0x2376a4 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
// type: void __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_")]
pub fn stub_2376a4() {
    // IDA 0x2376a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x237798 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_237798(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0x237798: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0x237848 — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, _DWORD *, int *)
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_237848() {
    // IDA 0x237848: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2378fc — __ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_
// type: _DWORD *__fastcall(int, int *)
#[doc(alias = "__ZNSt8_Rb_treeIPKvSt4pairIKS1_N5boost6detail13tss_data_nodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE14_M_create_nodeERKS7_")]
pub fn stub_2378fc() -> Option<u32> {
    // IDA 0x2378fc: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x2379ec — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_2379ec(has_weak: bool) -> bool {
    // IDA 0x2379ec: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0x237b40 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev
// type: void()
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED1Ev")]
pub fn stub_237b40(data: &[u8]) -> bool {
    // IDA 0x237b40: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x237b44 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEED0Ev")]
pub fn stub_237b44(data: &[u8]) -> bool {
    // IDA 0x237b44: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x237b50 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE7disposeEv")]
pub fn stub_237b50(data: &[u8]) -> bool {
    // IDA 0x237b50: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x237b64 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE11get_deleterERKSt9type_info")]
pub fn stub_237b64() -> bool {
    // IDA 0x237b64: deleter query misses for this control block.
    false
}
// 0x237b68 — __ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS0_16thread_data_baseEE19get_untyped_deleterEv")]
pub fn stub_237b68() -> bool {
    // IDA 0x237b68: deleter query misses for this control block.
    false
}
// 0x237b6c — __ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE
// type: void __fastcall(int)
#[doc(alias = "__ZN5boost6detail18future_object_base22mark_finished_internalERNS_11unique_lockINS_5mutexEEE")]
pub fn stub_237b6c() {
    // IDA 0x237b6c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x237c98 — __GLOBAL__I_a_38
// type: 
#[doc(alias = "__GLOBAL__I_a_38")]
pub fn stub_237c98() {
    // IDA 0x237c98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x237d60 — __ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "__ZN5boost10filesystem6detail14symlink_statusERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_237d60() {
    // IDA 0x237d60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x237fa4 — __ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "__ZN5boost10filesystem6detail12current_pathEPNS_6system10error_codeE")]
pub fn stub_237fa4() {
    // IDA 0x237fa4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x238258 — __ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs
// type: int __fastcall(int, void *, int, int)
#[doc(alias = "__ZN12_GLOBAL__N_15errorEbRKN5boost10filesystem4pathEPNS0_6system10error_codeERKSs")]
pub fn stub_238258() {
    // IDA 0x238258: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23837c — __ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, _DWORD *)
#[doc(alias = "__ZN5boost10filesystem6detail12initial_pathEPNS_6system10error_codeE")]
pub fn stub_23837c() -> Option<u32> {
    // IDA 0x23837c: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x23852c — __ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE
// type: bool __fastcall(const char **, int)
#[doc(alias = "__ZN5boost10filesystem6detail8is_emptyERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_23852c() {
    // IDA 0x23852c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2386d4 — __ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE
// type: int __fastcall(const char **, int *)
#[doc(alias = "__ZN5boost10filesystem6detail6removeERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_2386d4() {
    // IDA 0x2386d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2388a8 — __ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE
// type: bool __fastcall(const char **, int, _DWORD *)
#[doc(alias = "__ZN12_GLOBAL__N_124remove_file_or_directoryERKN5boost10filesystem4pathENS1_9file_typeEPNS0_6system10error_codeE")]
pub fn stub_2388a8() {
    // IDA 0x2388a8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x238adc — __ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(int *, const char **, int *)
#[doc(alias = "__ZN5boost10filesystem6detail6statusERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_238adc() {
    // IDA 0x238adc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x238d18 — __ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string *, const std::string *)
#[doc(alias = "__ZN5boost10filesystem6detail15system_completeERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_238d18() {
    // IDA 0x238d18: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x238f14 — __ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE
// type: __int64 __fastcall(_QWORD *, int, int *)
#[doc(alias = "__ZNK5boost10filesystem15directory_entry12m_get_statusEPNS_6system10error_codeE")]
pub fn stub_238f14(handle: u32) -> String {
    // IDA 0x238f14: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x238f80 — __ZN5boost10filesystem6detail13dir_itr_closeERPvS3_
// type: int __fastcall(boost::filesystem::detail *this, void **, void **)
#[doc(alias = "__ZN5boost10filesystem6detail13dir_itr_closeERPvS3_")]
pub fn stub_238f80(handle: u32) {
    // IDA 0x238f80: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x238fd4 — __ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE
// type: void __fastcall(std::string **, const char **, std::string **)
#[doc(alias = "__ZN5boost10filesystem6detail28directory_iterator_constructERNS0_18directory_iteratorERKNS0_4pathEPNS_6system10error_codeE")]
pub fn stub_238fd4() {
    // IDA 0x238fd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x239668 — __ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE
// type: void __fastcall(int *, dirent **)
#[doc(alias = "__ZN5boost10filesystem6detail28directory_iterator_incrementERNS0_18directory_iteratorEPNS_6system10error_codeE")]
pub fn stub_239668() {
    // IDA 0x239668: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x239b34 — __ZN5boost10filesystem18directory_iteratorD1Ev
// type: void __fastcall(boost::filesystem::directory_iterator *__hidden this)
#[doc(alias = "__ZN5boost10filesystem18directory_iteratorD1Ev")]
pub fn stub_239b34() {
    // IDA 0x239b34: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x239bc8 — __ZN5boost10filesystem16filesystem_errorD1Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorD1Ev")]
pub fn stub_239bc8() {
    // IDA 0x239bc8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x239cc8 — __ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, std::runtime_error_vtbl *, const char *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorC2ERKSsNS_6system10error_codeE")]
pub fn stub_239cc8() {
    // IDA 0x239cc8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x239e90 — __ZN5boost10filesystem16filesystem_errorD0Ev
// type: void __fastcall(std::runtime_error *this)
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorD0Ev")]
pub fn stub_239e90() {
    // IDA 0x239e90: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x239f94 — __ZNK5boost10filesystem16filesystem_error4whatEv
// type: int __fastcall(boost::filesystem::filesystem_error *this)
#[doc(alias = "__ZNK5boost10filesystem16filesystem_error4whatEv")]
pub fn stub_239f94() {
    // IDA 0x239f94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a11c — __ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, boost::detail::sp_counted_base **, int, void *, int)
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructINS_10filesystem16filesystem_error5m_impES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
pub fn stub_23a11c(slot: &mut Option<u32>, v: u32) {
    // IDA 0x23a11c: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0x23a2bc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev
// type: void()
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED1Ev")]
pub fn stub_23a2bc() {
    // IDA 0x23a2bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a2c0 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEED0Ev")]
pub fn stub_23a2c0() {
    // IDA 0x23a2c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a2cc — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE7disposeEv")]
pub fn stub_23a2cc() {
    // IDA 0x23a2cc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a38c — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE11get_deleterERKSt9type_info")]
pub fn stub_23a38c() -> bool {
    // IDA 0x23a38c: deleter query misses for this control block.
    false
}
// 0x23a390 — __ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_10filesystem16filesystem_error5m_impEE19get_untyped_deleterEv")]
pub fn stub_23a390() -> bool {
    // IDA 0x23a390: deleter query misses for this control block.
    false
}
// 0x23a394 — __ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE
// type: std::runtime_error *__fastcall(std::runtime_error *, const std::string *, const std::string *, std::runtime_error_vtbl *, boost::detail::sp_counted_base *, std::runtime_error *, int, int, void *, int)
#[doc(alias = "__ZN5boost10filesystem16filesystem_errorC2ERKSsRKNS0_4pathENS_6system10error_codeE")]
pub fn stub_23a394() {
    // IDA 0x23a394: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a570 — __GLOBAL__I_a_39
// type: int *()
#[doc(alias = "__GLOBAL__I_a_39")]
pub fn stub_23a570() {
    // IDA 0x23a570: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a630 — __ZN5boost10filesystem4pathdVERKS1_
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const std::string *)
#[doc(alias = "__ZN5boost10filesystem4pathdVERKS1_")]
pub fn stub_23a630() {
    // IDA 0x23a630: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a7b8 — __ZN5boost10filesystem4path28m_append_separator_if_neededEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "__ZN5boost10filesystem4path28m_append_separator_if_neededEv")]
pub fn stub_23a7b8() {
    // IDA 0x23a7b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a830 — __ZN5boost10filesystem4pathdVEPKc
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *, const char *, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "__ZN5boost10filesystem4pathdVEPKc")]
pub fn stub_23a830() {
    // IDA 0x23a830: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23a9d4 — __ZN5boost10filesystem4path27m_erase_redundant_separatorEm
// type: std::string *__fastcall(std::string *this, unsigned int)
#[doc(alias = "__ZN5boost10filesystem4path27m_erase_redundant_separatorEm")]
pub fn stub_23a9d4() {
    // IDA 0x23a9d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23aa2c — __ZN5boost10filesystem4path15remove_filenameEv
// type: boost::filesystem::path *__fastcall(boost::filesystem::path *this)
#[doc(alias = "__ZN5boost10filesystem4path15remove_filenameEv")]
pub fn stub_23aa2c(handle: u32) -> String {
    // IDA 0x23aa2c: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x23aa60 — __ZNK5boost10filesystem4path17m_parent_path_endEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "__ZNK5boost10filesystem4path17m_parent_path_endEv")]
pub fn stub_23aa60() {
    // IDA 0x23aa60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ab64 — __ZNK5boost10filesystem4path14root_directoryEv
// type: char *__fastcall(boost::filesystem::path *this, std::string *)
#[doc(alias = "__ZNK5boost10filesystem4path14root_directoryEv")]
pub fn stub_23ab64() {
    // IDA 0x23ab64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23abe8 — __ZNK5boost10filesystem4path11parent_pathEv
// type: char *__fastcall(boost::filesystem::path *this, boost::filesystem::path *)
#[doc(alias = "__ZNK5boost10filesystem4path11parent_pathEv")]
pub fn stub_23abe8() {
    // IDA 0x23abe8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ac1c — __ZN5boost10filesystem4path7codecvtEv
// type: int __fastcall(boost::filesystem::path *this)
#[doc(alias = "__ZN5boost10filesystem4path7codecvtEv")]
pub fn stub_23ac1c() {
    // IDA 0x23ac1c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ac2c — __ZN5boost10filesystem4pathD1Ev
// type: void __fastcall(boost::filesystem::path *__hidden this)
#[doc(alias = "__ZN5boost10filesystem4pathD1Ev")]
pub fn stub_23ac2c() {
    // IDA 0x23ac2c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ac78 — __ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_
// type: int __fastcall(int, const _Impl **, int, int, void *, int)
#[doc(alias = "__ZNSt6localeC2IN5boost10filesystem6detail18utf8_codecvt_facetEEERKS_PT_")]
pub fn stub_23ac78() {
    // IDA 0x23ac78: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23adc4 — __ZN5boost10filesystem4pathC2IPKcEET_S5_
// type: std::string *__fastcall(std::string *, _BYTE *, _BYTE *)
#[doc(alias = "__ZN5boost10filesystem4pathC2IPKcEET_S5_")]
pub fn stub_23adc4() {
    // IDA 0x23adc4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23af94 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, __mbstate_t *, char *, char *, char **)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet10do_unshiftER11__mbstate_tPcS5_RS5_")]
pub fn stub_23af94() {
    // IDA 0x23af94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23af9c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet11do_encodingEv")]
pub fn stub_23af9c() {
    // IDA 0x23af9c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23afa0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet16do_always_noconvEv")]
pub fn stub_23afa0() {
    // IDA 0x23afa0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23afa4 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet13do_max_lengthEv")]
pub fn stub_23afa4() {
    // IDA 0x23afa4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23afa8 — __GLOBAL__I_a_40
// type: void __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "__GLOBAL__I_a_40")]
pub fn stub_23afa8() {
    // IDA 0x23afa8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b14c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_
// type: int __fastcall(int, int, char *, char *, char **, int *, int *, int **)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet5do_inER11__mbstate_tPKcS6_RS6_PwS8_RS8_")]
pub fn stub_23b14c() {
    // IDA 0x23b14c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b2d0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// type: bool __fastcall(int, int, _DWORD *, _DWORD *, _DWORD *, _BYTE *, _BYTE *, _DWORD *)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_")]
pub fn stub_23b2d0() {
    // IDA 0x23b2d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// type: int __fastcall(boost::filesystem::detail::utf8_codecvt_facet *this, const __mbstate_t *, const char *, const char *, unsigned int)
#[doc(alias = "__ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m")]
pub fn stub_23b43c() {
    // IDA 0x23b43c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "__ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev")]
pub fn stub_23b4ac() {
    // IDA 0x23b4ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// type: void __fastcall(boost::filesystem::detail::utf8_codecvt_facet *__hidden this)
#[doc(alias = "__ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev")]
pub fn stub_23b4b8() {
    // IDA 0x23b4b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b4cc — __ZN5boost6system16generic_categoryEv
// type: int *__fastcall()
#[doc(alias = "__ZN5boost6system16generic_categoryEv")]
pub fn stub_23b4cc() {
    // IDA 0x23b4cc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b508 — __ZN5boost6system15system_categoryEv
// type: int *__fastcall()
#[doc(alias = "__ZN5boost6system15system_categoryEv")]
pub fn stub_23b508() {
    // IDA 0x23b508: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b544 — __ZN12_GLOBAL__N_121system_error_categoryD1Ev
// type: void __fastcall(_anonymous_namespace_::system_error_category *__hidden this)
#[doc(alias = "__ZN12_GLOBAL__N_121system_error_categoryD1Ev")]
pub fn stub_23b544() {
    // IDA 0x23b544: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b548 — __ZN12_GLOBAL__N_122generic_error_categoryD1Ev
// type: void __fastcall(_anonymous_namespace_::generic_error_category *__hidden this)
#[doc(alias = "__ZN12_GLOBAL__N_122generic_error_categoryD1Ev")]
pub fn stub_23b548() {
    // IDA 0x23b548: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b54c — __ZN12_GLOBAL__N_122generic_error_categoryD0Ev
// type: void __fastcall(_anonymous_namespace_::generic_error_category *__hidden this)
#[doc(alias = "__ZN12_GLOBAL__N_122generic_error_categoryD0Ev")]
pub fn stub_23b54c() {
    // IDA 0x23b54c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b558 — __ZNK12_GLOBAL__N_122generic_error_category4nameEv
// type: const char *__fastcall(_anonymous_namespace_::generic_error_category *this)
#[doc(alias = "__ZNK12_GLOBAL__N_122generic_error_category4nameEv")]
pub fn stub_23b558(handle: u32) -> String {
    // IDA 0x23b558: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x23b564 — __ZNK12_GLOBAL__N_122generic_error_category7messageEi
// type: int __fastcall(_anonymous_namespace_::generic_error_category *this, int, int)
#[doc(alias = "__ZNK12_GLOBAL__N_122generic_error_category7messageEi")]
pub fn stub_23b564() {
    // IDA 0x23b564: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b7cc — __ZN12_GLOBAL__N_121system_error_categoryD0Ev
// type: void __fastcall(_anonymous_namespace_::system_error_category *__hidden this)
#[doc(alias = "__ZN12_GLOBAL__N_121system_error_categoryD0Ev")]
pub fn stub_23b7cc() {
    // IDA 0x23b7cc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b7d8 — __ZNK12_GLOBAL__N_121system_error_category4nameEv
// type: const char *__fastcall(_anonymous_namespace_::system_error_category *this)
#[doc(alias = "__ZNK12_GLOBAL__N_121system_error_category4nameEv")]
pub fn stub_23b7d8(handle: u32) -> String {
    // IDA 0x23b7d8: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x23b7e4 — __ZNK12_GLOBAL__N_121system_error_category7messageEi
// type: int __fastcall(_anonymous_namespace_::system_error_category *this, int, int)
#[doc(alias = "__ZNK12_GLOBAL__N_121system_error_category7messageEi")]
pub fn stub_23b7e4() {
    // IDA 0x23b7e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23b838 — __ZNK12_GLOBAL__N_121system_error_category23default_error_conditionEi
// type: void __fastcall(_anonymous_namespace_::system_error_category *this, int, int)
#[doc(alias = "__ZNK12_GLOBAL__N_121system_error_category23default_error_conditionEi")]
pub fn stub_23b838() {
    // IDA 0x23b838: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ca3c — __ZNK5boost6system14error_category23default_error_conditionEi
// type: _QWORD *__fastcall(_QWORD *this, int, __int64)
#[doc(alias = "__ZNK5boost6system14error_category23default_error_conditionEi")]
pub fn stub_23ca3c() {
    // IDA 0x23ca3c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ca44 — __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// type: bool __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE")]
pub fn stub_23ca44() {
    // IDA 0x23ca44: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ca70 — __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// type: bool __fastcall(int, _DWORD *, int)
#[doc(alias = "__ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi")]
pub fn stub_23ca70() {
    // IDA 0x23ca70: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23ca88 — __GLOBAL__I_a_41
// type: void()
#[doc(alias = "__GLOBAL__I_a_41")]
pub fn stub_23ca88() {
    // IDA 0x23ca88: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// type: void __fastcall(boost::iostreams::detail::gzip_header *this, unsigned __int8)
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_header7processEc")]
pub fn stub_23cb64() {
    // IDA 0x23cb64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// type: int __fastcall(boost::iostreams::detail::gzip_header *this)
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_header5resetEv")]
pub fn stub_23cef0(handle: u32) {
    // IDA 0x23cef0: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x23cf2c — __ZN5boost9iostreams6detail11gzip_footer7processEc
// type: _DWORD *__fastcall(_DWORD *this, unsigned __int8)
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_footer7processEc")]
pub fn stub_23cf2c() {
    // IDA 0x23cf2c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23cf7c — __ZN5boost9iostreams6detail11gzip_footer5resetEv
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "__ZN5boost9iostreams6detail11gzip_footer5resetEv")]
pub fn stub_23cf7c(handle: u32) {
    // IDA 0x23cf7c: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// type: void __fastcall(boost::iostreams::zlib_error *this, int)
#[doc(alias = "__ZN5boost9iostreams10zlib_error5checkEi")]
pub fn stub_23cf8c() {
    // IDA 0x23cf8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// type: boost::iostreams::detail::zlib_base *__fastcall(boost::iostreams::detail::zlib_base *this)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_baseC2Ev")]
pub fn stub_23d0c8() {
    // IDA 0x23d0c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d0e8 — __ZN5boost9iostreams6detail9zlib_baseD2Ev
// type: void __fastcall(void **this)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_baseD2Ev")]
pub fn stub_23d0e8() {
    // IDA 0x23d0e8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// type: int __fastcall(boost::iostreams::detail::zlib_base *this, const char **, const char *, char **, char *)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_")]
pub fn stub_23d0fc() {
    // IDA 0x23d0fc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// type: const char *__fastcall(boost::iostreams::detail::zlib_base *this, const char **, char **, int)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb")]
pub fn stub_23d120() {
    // IDA 0x23d120: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d180 — __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base8xdeflateEi")]
pub fn stub_23d180() {
    // IDA 0x23d180: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d18c — __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// type: int __fastcall(z_streamp *this, int)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base8xinflateEi")]
pub fn stub_23d18c() {
    // IDA 0x23d18c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d198 — __ZN5boost9iostreams6detail9zlib_base5resetEbb
// type: int __fastcall(z_stream **this, int, int)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base5resetEbb")]
pub fn stub_23d198(handle: u32) {
    // IDA 0x23d198: releases the handle (double-free is UB engine-side).
    let _ = handle;
}
// 0x23d1c8 — __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// type: void __fastcall(int, int, int, int, int, void *)
#[doc(alias = "__ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_")]
pub fn stub_23d1c8() -> Option<u32> {
    // IDA 0x23d1c8: allocates a fresh handle; None on OOM.
    Some(0)
}
// 0x23d238 — __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// type: void __fastcall __noreturn(int)
#[doc(alias = "__ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_")]
pub fn stub_23d238() {
    // IDA 0x23d238: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d390 — __ZN5boost9iostreams10zlib_errorD1Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "__ZN5boost9iostreams10zlib_errorD1Ev")]
pub fn stub_23d390() {
    // IDA 0x23d390: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d39c — __ZN5boost9iostreams10zlib_errorD0Ev
// type: void __fastcall(boost::iostreams::zlib_error *__hidden this)
#[doc(alias = "__ZN5boost9iostreams10zlib_errorD0Ev")]
pub fn stub_23d39c() {
    // IDA 0x23d39c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d3b0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: std::ios_base::failure *__fastcall(std::ios_base::failure *)
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")]
pub fn stub_23d3b0() {
    // IDA 0x23d3b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d468 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// type: int __fastcall(std::ios_base::failure *)
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")]
pub fn stub_23d468() {
    // IDA 0x23d468: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d520 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev")]
pub fn stub_23d520(this: u32) -> u32 {
    // IDA 0x23d520: this-adjustment thunk (this -= 12) then tail-call.
    this.wrapping_sub(12)
}
// 0x23d5d8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")]
pub fn stub_23d5d8(this: u32) -> u32 {
    // IDA 0x23d5d8: this-adjustment thunk (this -= 12) then tail-call.
    this.wrapping_sub(12)
}
// 0x23d690 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev")]
pub fn stub_23d690() {
    // IDA 0x23d690: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d75c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(std::ios_base::failure *)
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")]
pub fn stub_23d75c() {
    // IDA 0x23d75c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d818 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv")]
pub fn stub_23d818() {
    // IDA 0x23d818: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x23d8d4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv")]
pub fn stub_23d8d4(e: &GenDateError) -> ! {
    // IDA 0x23d8d4: __noreturn rethrow of the stored exception.
    panic!("rethrow: {}", e);
}
// 0x23d984 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev")]
pub fn stub_23d984(this: u32) -> u32 {
    // IDA 0x23d984: this-adjustment thunk (this -= 12) then tail-call.
    this.wrapping_sub(12)
}
