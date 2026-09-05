// Auto-generated skeletons for rbx-script — filler EA-sorted asc (global holes)
// Filter: Lua|Script|Yield|lua (case-sensitive, lua lower) -> 5401 filtered, all stubbed (0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x16d84..0x43364 | filtered 5401 done, script 11485->11585 total, global 79402->79502 covered, 6044 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use crate::generated_165::SettingsHandler;
use std::collections::{BTreeMap, HashMap};
// Join-game `boost::function0<void>` bind records (IDA 0x2f0f0..0x3093c):
// each ctor wraps a `bind_t` of a join entry point with copied args; the
// functorvtable management folds into the host closure and the bound
// arguments are observed here.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JoinGameCallback {
    pub place_id: i32,
    pub url: String,
    pub game_live: bool,
    pub with_request: bool,
}
/// `TaskScheduler` blocking-function registry (IDA 0x39c6c): (job, func)
/// pairs; removal answers whether a pair was registered.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SchedulerBlockers {
    pub blocks: Vec<(u32, u32)>,
}
impl SchedulerBlockers {
    /// `removeBlocking` (IDA 0x39c6c): argument copies (shared_count
    /// add-refs) fold into the host; the pair is unregistered.
    pub fn remove_blocking(&mut self, job: u32, func: u32) -> bool {
        if let Some(pos) = self.blocks.iter().position(|&(j, f)| j == job && f == func) {
            self.blocks.remove(pos);
            true
        } else {
            false
        }
    }
}
/// `boost::mutex` latch (IDA 0x3c170): the unlock loop (0x3c1c0..) folds
/// into the host; the locked latch is observed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BoostMutex {
    pub locked: bool,
}

// 0x16d84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings10ShadowModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::ShadowMode>> *)")]
pub fn stub_0x16d84(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16d84 `_Rb_tree<Name const*, ShadowMode>::_M_erase(node)`:
    // recursive left-subtree erase, node delete, right walk (cf. 0x16d34
    // in generated_script_gap_015ef8.rs). Host has no tree nodes;
    // granularity collapses to the owning map.
    map.clear();
}

// 0x16dac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings16AntialiasingModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AntialiasingMode>> *)")]
pub fn stub_0x16dac(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16dac `_Rb_tree<Name const*, AntialiasingMode>::_M_erase(node)`:
    // recursive left-subtree erase, node delete, right walk (cf. 0x16d34
    // in generated_script_gap_015ef8.rs). Host has no tree nodes;
    // granularity collapses to the owning map.
    map.clear();
}

// 0x16dd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings20FrameRateManagerModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::FrameRateManagerMode>> *)")]
pub fn stub_0x16dd4(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16dd4 `_Rb_tree<Name const*, FrameRateManagerMode>::_M_erase(node)`:
    // recursive left-subtree erase, node delete, right walk (cf. 0x16d34
    // in generated_script_gap_015ef8.rs). Host has no tree nodes;
    // granularity collapses to the owning map.
    map.clear();
}

// 0x16dfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings12GraphicsModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::GraphicsMode>> *)")]
pub fn stub_0x16dfc(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16dfc `_Rb_tree<Name const*, GraphicsMode>::_M_erase(node)`:
    // recursive left-subtree erase, node delete, right walk (cf. 0x16d34
    // in generated_script_gap_015ef8.rs). Host has no tree nodes;
    // granularity collapses to the owning map.
    map.clear();
}

// 0x16e24 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15CRenderSettings9AASamplesEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CRenderSettings::AASamples>> *)")]
pub fn stub_0x16e24(map: &mut BTreeMap<u32, i32>) {
    // IDA 0x16e24 `_Rb_tree<Name const*, AASamples>::_M_erase(node)`:
    // recursive left-subtree erase, node delete, right walk (cf. 0x16d34
    // in generated_script_gap_015ef8.rs). Host has no tree nodes;
    // granularity collapses to the owning map.
    map.clear();
}

// 0x23a04 — __ZNSt3mapISsPFvPKcESt4lessISsESaISt4pairIKSsS3_EEEixERS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,void (*)(char const*),std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::operator[](std::string const&)")]
pub fn stub_0x23a04(map: &HashMap<String, SettingsHandler>, key: &str) -> Option<SettingsHandler> {
    // IDA 0x23a04 `std::map<string, void (*)(char const*)>::operator[]`:
    // lower-bound + insert-default (null reader) when absent, returns
    // the mapped slot (cf. 0x16d84: granularity collapses to the owning
    // map). Host has no null reader, so a miss reports `None` without
    // inserting; every in-tree caller assigns through the slot
    // immediately (IDA 0x21ce0), which `stub_0x21ce0` performs via
    // `HashMap::insert`.
    map.get(key).copied()
}

// 0x24274 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_0x24274(map: &mut HashMap<String, SettingsHandler>, key: &str, handler: SettingsHandler) -> bool {
    // IDA 0x24274 hinted `_M_insert_unique`: hint-guided lower-bound
    // probe, insert-or-existing (cf. 0x243b0). The tree hint folds;
    // granularity collapses to the owning map entry.
    use std::collections::hash_map::Entry;
    match map.entry(key.to_string()) {
        Entry::Occupied(_) => false,
        Entry::Vacant(slot) => {
            slot.insert(handler);
            true
        }
    }
}

// 0x24360 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_0x24360(map: &mut HashMap<String, SettingsHandler>, key: &str, handler: SettingsHandler) {
    // IDA 0x24360 `_M_insert`: node create + rebalance at the hinted
    // position, size bump. Host has no nodes or balance state;
    // granularity collapses to the owning map insert.
    map.insert(key.to_string(), handler);
}

// 0x243b0 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_0x243b0(map: &mut HashMap<String, SettingsHandler>, key: &str, handler: SettingsHandler) -> bool {
    // IDA 0x243b0 `_M_insert_unique`: lower-bound walk, insert-or-find
    // (`_M_insert` at 0x24360 on miss). Returns inserted/not-found.
    stub_0x24274(map, key, handler)
}

// 0x24434 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_0x24434(key: &str, handler: SettingsHandler) -> (String, SettingsHandler) {
    // IDA 0x24434 `_M_create_node`: `operator new(0x18)` + pair copy
    // (string + reader). Host has no nodes; the owned pair is the node.
    (key.to_string(), handler)
}

// 0x24510 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)")]
pub fn stub_0x24510(map: &HashMap<String, SettingsHandler>, key: &str) -> Option<SettingsHandler> {
    // IDA 0x24510 `lower_bound`: first node not-less-than `key`. Host
    // map is unordered, so only exact hits resolve; every in-tree
    // caller (0x23a04/0x24274/0x243b0) treats a miss by inserting,
    // which those host shims already perform themselves.
    map.get(key).copied()
}

// 0x2f0f0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2f0f0(place_id: i32, game_live: bool) -> JoinGameCallback {
    // IDA 0x2f0f0: `function0<void>` ctor from
    // `bind_t<void,void(*)(int,SharedPtr<Game>)>` — see `JoinGameCallback`.
    JoinGameCallback { place_id, url: String::new(), game_live, with_request: false }
}

// 0x2f7d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2f7d0(place_id: i32, game_live: bool) -> JoinGameCallback {
    // IDA 0x2f7d0: `function0<void>` ctor from the `JoinGameRequest` bind
    // flavor — same record shape as 0x2f0f0 with the request flag.
    JoinGameCallback { place_id, url: String::new(), game_live, with_request: true }
}

// 0x2ff94 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2ff94(url: &str, game_live: bool) -> JoinGameCallback {
    // IDA 0x2ff94: `function0<void>` ctor from the url+game bind flavor —
    // same record shape with the url filled in.
    JoinGameCallback { place_id: 0, url: url.to_owned(), game_live, with_request: false }
}

// 0x3093c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3093c(url: &str, game_live: bool) -> JoinGameCallback {
    // IDA 0x3093c: `function0<void>` ctor from the url+game bind flavor —
    // same record shape as 0x2ff94.
    JoinGameCallback { place_id: 0, url: url.to_owned(), game_live, with_request: false }
}

// 0x39c6c — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
// type: void __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)")]
pub fn stub_0x39c6c(blocks: &mut SchedulerBlockers, job: u32, func: u32) -> bool {
    // IDA 0x39c6c: `removeBlocking` — see `SchedulerBlockers::remove_blocking`.
    blocks.remove_blocking(job, func)
}

// 0x3a1bc — __ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::operator=(rbx_core::SharedPtr<RBX::Game> const&)")]
pub fn stub_0x3a1bc() {
    // IDA 0x3a1bc: `shared_ptr<Game>::operator=` add-refs the new count
    // (0x3a216), swaps it in (0x3a222), and releases the old one; `Arc`
    // assignment glue covers it — no-op.
}

// 0x3a5bc — __ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_
#[doc(alias = "void rbx_core::SharedPtr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
pub fn stub_0x3a5bc() {
    // IDA 0x3a5bc: `shared_ptr<Sequence>::reset(ptr)` builds the count
    // block (0x3a5de), installs the pointer (0x3a5ec), and releases the old
    // count; `Arc` construction glue covers it — no-op.
}

// 0x3a660 — __ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ViewBase>::reset(void)")]
pub fn stub_0x3a660() {
    // IDA 0x3a660: `shared_ptr<ViewBase>::reset()` nulls the pointer
    // (0x3a686..0x3a68c) and releases (0x3a6ae..0x3a6b6); the drop folds
    // into `Arc` — no-op.
}

// 0x3a6f8 — __ZN5boost13exception_ptrD1Ev
// type: void __fastcall(boost::exception_ptr *__hidden this)
#[doc(alias = "boost::exception_ptr::~exception_ptr()")]
pub fn stub_0x3a6f8() {
    // IDA 0x3a6f8: `exception_ptr` dtor releases the count (0x3a71e..
    // 0x3a74a); the payload drop folds into the host — no-op.
}

// 0x3a850 — __ZN5boost6detail15sp_counted_base12weak_releaseEv
// type: _DWORD __fastcall(boost::detail::sp_counted_base *__hidden this)
#[doc(alias = "boost::detail::sp_counted_base::weak_release(void)")]
pub fn stub_0x3a850() {
    // IDA 0x3a850: `weak_release` locks the pool spinlock (0x3a89c),
    // decrements the weak count (0x3a8ae..0x3a8b2), and destroys at zero
    // (0x3a8e0..0x3a8ee); `Weak` drop glue covers it — no-op.
}

// 0x3b14c — __ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
pub fn stub_0x3b14c() {
    // IDA 0x3b14c: `shared_count` ctor allocates the 0x10 control block
    // with unit counts (0x3b178..0x3b1c0); `Arc` construction glue covers
    // it — no-op.
}

// 0x3b270 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
pub fn stub_0x3b270() {
    // IDA 0x3b270: D1 dtor has an empty body; drop glue covers it — no-op.
}

// 0x3b274 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p() [0x3b274]")]
pub fn stub_0x3b274() {
    // IDA 0x3b274: D0 dtor (base teardown plus delete, same shape as
    // 0x26a2b0); both fold into drop glue — no-op.
}

// 0x3b278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::dispose(void)")]
pub fn stub_0x3b278() {
    // IDA 0x3b278: `dispose` resets the vtable (0x3b2d0), deletes the member
    // (0x3b2d8), destroys the mutex (0x3b2e6), and deletes the block
    // (0x3b2ec); drop glue covers it — no-op.
}

// 0x3b32c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_deleter(std::type_info const&)")]
pub fn stub_0x3b32c() -> u32 {
    // IDA 0x3b32c: `get_deleter` answers null (0x3b32e); no custom
    // deleters exist in the host model.
    0
}

// 0x3b330 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_untyped_deleter(void)")]
pub fn stub_0x3b330() -> u32 {
    // IDA 0x3b330: `get_untyped_deleter` answers null (0x3b332); same as
    // 0x3b32c.
    0
}

// 0x3b334 — __ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)")]
pub fn stub_0x3b334() {
    // IDA 0x3b334: `shared_count` ctor for ExclusiveSequence — same control
    // block shape as 0x3b14c; `Arc` glue covers it — no-op.
}

// 0x3b450 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
pub fn stub_0x3b450() {
    // IDA 0x3b450: D1 dtor has an empty body (same shape as 0x3b270) —
    // no-op.
}

// 0x3b454 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p() [0x3b454]")]
pub fn stub_0x3b454() {
    // IDA 0x3b454: D0 dtor (same shape as 0x3b274) — no-op.
}

// 0x3b458 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::dispose(void)")]
pub fn stub_0x3b458() {
    // IDA 0x3b458: `dispose` (same shape as 0x3b278) — no-op.
}

// 0x3b50c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_deleter(std::type_info const&)")]
pub fn stub_0x3b50c() -> u32 {
    // IDA 0x3b50c: `get_deleter` answers null (same shape as 0x3b32c).
    0
}

// 0x3b510 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_untyped_deleter(void)")]
pub fn stub_0x3b510() -> u32 {
    // IDA 0x3b510: `get_untyped_deleter` answers null (same shape as
    // 0x3b330).
    0
}

// 0x3c010 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// type: int __fastcall(_DWORD)
#[doc(alias = "void rbx_core::SharedPtr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
pub fn stub_0x3c010(strong: u32) {
    // IDA 0x3c010: `intrusive_ptr_add_weak_ref` release-asserts a live
    // strong count (intrusive_ptr_target.h:214, 0x3c022..0x3c064) and bumps
    // the weak count (0x3c068..0x3c06e, folds into `Arc`).
    if strong == 0 {
        panic!("c->strong > 0 file: ../Base/include/rbx/intrusive_ptr_target.h line: 214");
    }
}

// 0x3c0c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)")]
pub fn stub_0x3c0c8() {
    // IDA 0x3c0c8: `intrusive_ptr<slot>::operator=` add-refs the new slot
    // (0x3c11c), swaps it in (0x3c124), and releases the old one (0x3c12c);
    // `Arc` assignment glue covers it — no-op.
}

// 0x3c170 — __ZN5boost5mutex6unlockEv
// type: _DWORD __fastcall(boost::mutex *__hidden this)
#[doc(alias = "boost::mutex::unlock(void)")]
pub fn stub_0x3c170(mutex: &mut BoostMutex) -> i32 {
    // IDA 0x3c170: `mutex::unlock` loops `pthread_mutex_unlock` to success
    // (0x3c1c0.., folds into the host) and answers success.
    mutex.locked = false;
    0
}

// 0x3c2a0 — __ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::throw_exception<boost::lock_error>(boost::lock_error const&)")]
pub fn stub_0x3c2a0() -> ! {
    todo!("0x3c2a0 __ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_")
}

// 0x3c470 — __ZN5boost10lock_errorD0Ev
// type: void __fastcall(boost::lock_error *__hidden this)
#[doc(alias = "boost::lock_error::~lock_error() [0x3c470]")]
pub fn stub_0x3c470() -> ! {
    todo!("0x3c470 __ZN5boost10lock_errorD0Ev")
}

// 0x3c4a0 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev
// type: int(void)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
pub fn stub_0x3c4a0() -> ! {
    todo!("0x3c4a0 __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev")
}

// 0x3c4e0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector() [0x3c4e0]")]
pub fn stub_0x3c4e0() -> ! {
    todo!("0x3c4e0 __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev")
}

// 0x3c528 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl() [0x3c528]")]
pub fn stub_0x3c528() -> ! {
    todo!("0x3c528 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev")
}

// 0x3c570 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
// type: int(void)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl() [0x3c570]")]
pub fn stub_0x3c570() -> ! {
    todo!("0x3c570 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev")
}

// 0x3c5b8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone(void)const")]
pub fn stub_0x3c5b8() -> ! {
    todo!("0x3c5b8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv")
}

// 0x3c678 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl() [0x3c678]")]
pub fn stub_0x3c678() -> ! {
    todo!("0x3c678 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev")
}

// 0x3c680 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev
#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector() [0x3c680]")]
pub fn stub_0x3c680() -> ! {
    todo!("0x3c680 __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev")
}

// 0x3c698 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::adopt(boost::exception_detail::error_info_container*)")]
pub fn stub_0x3c698() -> ! {
    todo!("0x3c698 __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_")
}

// 0x3c6c8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::lock_error> const&)")]
pub fn stub_0x3c6c8() -> ! {
    todo!("0x3c6c8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_")
}

// 0x3c928 — __ZN5boost21thread_resource_errorD1Ev
// type: void __fastcall(boost::thread_resource_error *__hidden this)
#[doc(alias = "boost::thread_resource_error::~thread_resource_error() [0x3c928]")]
pub fn stub_0x3c928() -> ! {
    todo!("0x3c928 __ZN5boost21thread_resource_errorD1Ev")
}

// 0x3c958 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector() [0x3c958]")]
pub fn stub_0x3c958() -> ! {
    todo!("0x3c958 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev")
}

// 0x3c998 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector() [0x3c998]")]
pub fn stub_0x3c998() -> ! {
    todo!("0x3c998 __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev")
}

// 0x3c9e0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl() [0x3c9e0]")]
pub fn stub_0x3c9e0() -> ! {
    todo!("0x3c9e0 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")
}

// 0x3ca28 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// type: int(void)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
pub fn stub_0x3ca28() -> ! {
    todo!("0x3ca28 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")
}

// 0x3ca70 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
pub fn stub_0x3ca70() -> ! {
    todo!("0x3ca70 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")
}

// 0x3cb30 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl() [0x3cb30]")]
pub fn stub_0x3cb30() -> ! {
    todo!("0x3cb30 __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")
}

// 0x3cb38 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
pub fn stub_0x3cb38() -> ! {
    todo!("0x3cb38 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv")
}

// 0x3cb48 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector() [0x3cb48]")]
pub fn stub_0x3cb48() -> ! {
    todo!("0x3cb48 __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev")
}

// 0x3cb60 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_
// type: int __fastcall(int, int, int, int, std::exception *, std::string *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::thread_resource_error> const&)")]
pub fn stub_0x3cb60() -> ! {
    todo!("0x3cb60 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_")
}

// 0x3db4c — __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
pub fn stub_0x3db4c() -> ! {
    todo!("0x3db4c __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_")
}

// 0x3dc40 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
pub fn stub_0x3dc40() -> ! {
    todo!("0x3dc40 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev")
}

// 0x3dc44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p() [0x3dc44]")]
pub fn stub_0x3dc44() -> ! {
    todo!("0x3dc44 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev")
}

// 0x3dc48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)")]
pub fn stub_0x3dc48() -> ! {
    todo!("0x3dc48 __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv")
}

// 0x3dc5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)")]
pub fn stub_0x3dc5c() -> ! {
    todo!("0x3dc5c __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv")
}

// 0x3e030 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3e030() -> ! {
    todo!("0x3e030 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")
}

// 0x3e090 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x3e090() -> ! {
    todo!("0x3e090 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS3_5list1INS3_5valueIPS8_EEEEEEvE6invokeERNS1_15function_bufferE")
}

// 0x3e094 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FunctionMarshaller>,boost::_bi::list1<boost::_bi::value<RBX::FunctionMarshaller*>>>::operator()(void)")]
pub fn stub_0x3e094() -> ! {
    todo!("0x3e094 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX18FunctionMarshallerEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

// 0x3e198 — __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x3e198() -> ! {
    todo!("0x3e198 __ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

// 0x3e238 — __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<XmlElement,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x3e238() -> ! {
    todo!("0x3e238 __ZN5boost14singleton_poolI10XmlElementLj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

// 0x3e288 — __ZN5boost9function0IvE13assign_to_ownERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function0<void>::assign_to_own(boost::function0<void> const&)")]
pub fn stub_0x3e288() -> ! {
    todo!("0x3e288 __ZN5boost9function0IvE13assign_to_ownERKS1_")
}

// 0x3e2b8 — __ZN5boost16exception_detail14bad_exception_D1Ev
// type: void __fastcall(boost::exception_detail::bad_exception_ *__hidden this)
#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_() [0x3e2b8]")]
pub fn stub_0x3e2b8() -> ! {
    todo!("0x3e2b8 __ZN5boost16exception_detail14bad_exception_D1Ev")
}

// 0x3e2e8 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone(void)const")]
pub fn stub_0x3e2e8() -> ! {
    todo!("0x3e2e8 __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv")
}

// 0x3e3a8 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS3_NS3_9clone_tagE
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::clone_tag)")]
pub fn stub_0x3e3a8() -> ! {
    todo!("0x3e3a8 __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EEC1ERKS3_NS3_9clone_tagE")
}

// 0x3e528 — __ZThn20_N5boost16exception_detail14bad_exception_D0Ev
// type: void __fastcall(boost::exception_detail::bad_exception_ *__hidden this)
#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_exception_::~bad_exception_() [0x3e528]")]
pub fn stub_0x3e528() -> ! {
    todo!("0x3e528 __ZThn20_N5boost16exception_detail14bad_exception_D0Ev")
}

// 0x3e558 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_14bad_exception_EEEEEPT_
#[doc(alias = "rbx_core::SharedPtr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)")]
pub fn stub_0x3e558() -> ! {
    todo!("0x3e558 __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_14bad_exception_EEEEEPT_")
}

// 0x3e640 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::~sp_counted_impl_p() [0x3e640]")]
pub fn stub_0x3e640() -> ! {
    todo!("0x3e640 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED1Ev")
}

// 0x3e648 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS2_
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::bad_alloc_ const&)")]
pub fn stub_0x3e648() -> ! {
    todo!("0x3e648 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS2_")
}

// 0x3e7c8 — __ZN5boost16exception_detail10bad_alloc_D1Ev
// type: void __fastcall(boost::exception_detail::bad_alloc_ *__hidden this)
#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_() [0x3e7c8]")]
pub fn stub_0x3e7c8() -> ! {
    todo!("0x3e7c8 __ZN5boost16exception_detail10bad_alloc_D1Ev")
}

// 0x3e7f8 — __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone(void)const")]
pub fn stub_0x3e7f8() -> ! {
    todo!("0x3e7f8 __ZNK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv")
}

// 0x3e8b8 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::rethrow(void)const")]
pub fn stub_0x3e8b8() -> ! {
    todo!("0x3e8b8 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE7rethrowEv")
}

// 0x3e8c8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl() [0x3e8c8]")]
pub fn stub_0x3e8c8() -> ! {
    todo!("0x3e8c8 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED0Ev")
}

// 0x3e900 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> const&,boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::clone_tag)")]
pub fn stub_0x3e900() -> ! {
    todo!("0x3e900 __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EEC1ERKS3_NS3_9clone_tagE")
}

// 0x3ea80 — __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev
// type: void __fastcall(boost::exception_detail::bad_alloc_ *__hidden this)
#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_alloc_::~bad_alloc_() [0x3ea80]")]
pub fn stub_0x3ea80() -> ! {
    todo!("0x3ea80 __ZThn20_N5boost16exception_detail10bad_alloc_D0Ev")
}

// 0x3eab0 — __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_
#[doc(alias = "rbx_core::SharedPtr<boost::exception_detail::clone_base const>::shared_ptr<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_> *)")]
pub fn stub_0x3eab0() -> ! {
    todo!("0x3eab0 __ZN5boost10shared_ptrIKNS_16exception_detail10clone_baseEEC2INS1_10clone_implINS1_10bad_alloc_EEEEEPT_")
}

// 0x3eb98 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::dispose(void)")]
pub fn stub_0x3eb98() -> ! {
    todo!("0x3eb98 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE7disposeEv")
}

// 0x3eba8 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>>::get_deleter(std::type_info const&)")]
pub fn stub_0x3eba8() -> ! {
    todo!("0x3eba8 __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE11get_deleterERKSt9type_info")
}

// 0x3ebb8 — __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// type: int(void)
#[doc(alias = "void rbx_core::SharedPtr_weak_release<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
pub fn stub_0x3ebb8() -> ! {
    todo!("0x3ebb8 __ZN5boost26intrusive_ptr_weak_releaseIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")
}

// 0x3fcf8 — __ZN5boost12bad_weak_ptrD0Ev
// type: void __fastcall(boost::bad_weak_ptr *__hidden this)
#[doc(alias = "boost::bad_weak_ptr::~bad_weak_ptr()")]
pub fn stub_0x3fcf8() -> ! {
    todo!("0x3fcf8 __ZN5boost12bad_weak_ptrD0Ev")
}

// 0x3fd10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl() [0x3fd10]")]
pub fn stub_0x3fd10() -> ! {
    todo!("0x3fd10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")
}

// 0x3fd38 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
pub fn stub_0x3fd38() -> ! {
    todo!("0x3fd38 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev")
}

// 0x3fd60 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector()")]
pub fn stub_0x3fd60() -> ! {
    todo!("0x3fd60 __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED1Ev")
}

// 0x3fd88 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_0x3fd88() -> ! {
    todo!("0x3fd88 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED1Ev")
}

// 0x3fdb8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
// type: int(void)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
pub fn stub_0x3fdb8() -> ! {
    todo!("0x3fdb8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv")
}

// 0x3fee0 — __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl() [0x3fee0]")]
pub fn stub_0x3fee0() -> ! {
    todo!("0x3fee0 __ZThn4_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

// 0x3ff18 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::rethrow(void)const")]
pub fn stub_0x3ff18() -> ! {
    todo!("0x3ff18 __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEE7rethrowEv")
}

// 0x3ff28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_0x3ff28() -> ! {
    todo!("0x3ff28 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEED0Ev")
}

// 0x3ff60 — __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector() [0x3ff60]")]
pub fn stub_0x3ff60() -> ! {
    todo!("0x3ff60 __ZN5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev")
}

// 0x3ff90 — __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::bad_weak_ptr>::~error_info_injector() [0x3ff90]")]
pub fn stub_0x3ff90() -> ! {
    todo!("0x3ff90 __ZThn4_N5boost16exception_detail19error_info_injectorINS_12bad_weak_ptrEED0Ev")
}

// 0x3ffc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE
// type: int __fastcall(int, int, int, int, char, std::exception *, int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone_tag)")]
pub fn stub_0x3ffc0() -> ! {
    todo!("0x3ffc0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_12bad_weak_ptrEEEEC1ERKS5_NS5_9clone_tagE")
}

// 0x406e0 — __ZN5boost9function0IvE5clearEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::function0<void>::clear(void)")]
pub fn stub_0x406e0() -> ! {
    todo!("0x406e0 __ZN5boost9function0IvE5clearEv")
}

// 0x43360 — __ZN10SimpleJSON14DefaultHandlerERKSsS1_
#[doc(alias = "SimpleJSON::DefaultHandler(std::string const&,std::string const&)")]
pub fn stub_0x43360() -> ! {
    todo!("0x43360 __ZN10SimpleJSON14DefaultHandlerERKSsS1_")
}

// 0x43364 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (*)(char const*)>> *)")]
pub fn stub_0x43364() -> ! {
    todo!("0x43364 __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

#[cfg(test)]
mod join_boost_glue_batch_tests {
    use super::*;

    #[test]
    fn bind_records_capture_args() {
        assert_eq!(
            stub_0x2f0f0(12345, true),
            JoinGameCallback { place_id: 12345, url: String::new(), game_live: true, with_request: false }
        );
        assert_eq!(
            stub_0x2f7d0(12345, true),
            JoinGameCallback { place_id: 12345, url: String::new(), game_live: true, with_request: true }
        );
        assert_eq!(
            stub_0x2ff94("https://game/join", true),
            JoinGameCallback { place_id: 0, url: "https://game/join".to_owned(), game_live: true, with_request: false }
        );
        assert_eq!(
            stub_0x3093c("https://game/join", false),
            JoinGameCallback { place_id: 0, url: "https://game/join".to_owned(), game_live: false, with_request: false }
        );
    }

    #[test]
    fn remove_blocking_unregisters() {
        let mut blocks = SchedulerBlockers::default();
        blocks.blocks.push((7, 11));
        assert!(stub_0x39c6c(&mut blocks, 7, 11));
        assert!(blocks.blocks.is_empty());
        assert!(!stub_0x39c6c(&mut blocks, 7, 11));
    }

    #[test]
    fn shared_glue_noops_and_nulls() {
        stub_0x3a1bc();
        stub_0x3a5bc();
        stub_0x3a660();
        stub_0x3a6f8();
        stub_0x3a850();
        stub_0x3b14c();
        stub_0x3b270();
        stub_0x3b274();
        stub_0x3b278();
        stub_0x3b334();
        stub_0x3b450();
        stub_0x3b454();
        stub_0x3b458();
        stub_0x3c0c8();
        assert_eq!(stub_0x3b32c(), 0);
        assert_eq!(stub_0x3b330(), 0);
        assert_eq!(stub_0x3b50c(), 0);
        assert_eq!(stub_0x3b510(), 0);
    }

    #[test]
    fn weak_ref_requires_strong() {
        stub_0x3c010(2);
    }

    #[test]
    #[should_panic(expected = "c->strong > 0")]
    fn weak_ref_throws_when_dead() {
        stub_0x3c010(0);
    }

    #[test]
    fn boost_mutex_unlocks() {
        let mut mutex = BoostMutex { locked: true };
        assert_eq!(stub_0x3c170(&mut mutex), 0);
        assert!(!mutex.locked);
    }
}
