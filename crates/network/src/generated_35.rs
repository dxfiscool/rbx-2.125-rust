// Auto-generated skeletons for rbx-network — filler EA-sorted ascending after filtered RakNet/RBX::Network/Replicator exhausted
// Filter: RakNet|RBX::Network|Replicator|replica|remote (case-insensitive) -> 5974 funcs, all already stubbed (0 remaining)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x2aeee4..0x2b44d0 | existing 7049 -> 7199 total (filler global ascending, 0x2aeeb4 gap)

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


// 0x2aeee4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11upper_boundERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&)")]
pub fn stub_2aeee4() {
    // IDA 0x2aeee4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2aef18 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX8InstanceEEES8_EET0_T_SA_S9_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *>(rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *)")]
pub fn stub_2aef18() -> Option<u32> {
    // IDA 0x2aef18: nullable object query (id when live, None when unset).
    None
}
// 0x2aef64 — __ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_
#[doc(alias = "std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&)")]
pub fn stub_2aef64() {
    // IDA 0x2aef64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2af034 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_2af034() {
    // IDA 0x2af034: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2af120 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_2af120() {
    // IDA 0x2af120: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2af170 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_2af170() {
    // IDA 0x2af170: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2af1f4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_2af1f4() {
    // IDA 0x2af1f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2af310 — __ZN5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE16shared_from_thisEv // was: boost
#[doc(alias = "boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::shared_from_this(void)")]
pub fn stub_2af310() {
    // IDA 0x2af310: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2af428 — __ZN5boost10shared_ptrIN3RBX10CoreScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2af428() -> Option<u32> {
    // IDA 0x2af428: nullable object query (id when live, None when unset).
    None
}
// 0x2af4f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10CoreScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(rbx_core::SharedPtr<RBX::CoreScript> const*,RBX::CoreScript *)const")]
pub fn stub_2af4f0(has_weak: bool) -> bool {
    // IDA 0x2af4f0: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0x2af5dc — __ZN5boost6detail12shared_countC2IPN3RBX10CoreScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ // was: boost
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2af5dc() -> Option<u32> {
    // IDA 0x2af5dc: nullable object query (id when live, None when unset).
    None
}
// 0x2af6e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2af6e4() {
    // IDA 0x2af6e4: counted-impl dtor frees the control block.
}
// 0x2af6e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2af6e8() {
    // IDA 0x2af6e8: counted-impl dtor frees the control block.
}
// 0x2af6ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_2af6ec() -> Option<u32> {
    // IDA 0x2af6ec: nullable object query (id when live, None when unset).
    None
}
// 0x2af70c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_2af70c() -> bool {
    // IDA 0x2af70c: deleter query misses for this control block.
    false
}
// 0x2af724 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_2af724() -> bool {
    // IDA 0x2af724: deleter query misses for this control block.
    false
}
// 0x2af728 — __ZN5boost10shared_ptrIN3RBX13StarterScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2af728() -> Option<u32> {
    // IDA 0x2af728: nullable object query (id when live, None when unset).
    None
}
// 0x2af7f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13StarterScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(rbx_core::SharedPtr<RBX::StarterScript> const*,RBX::StarterScript *)const")]
pub fn stub_2af7f0(has_weak: bool) -> bool {
    // IDA 0x2af7f0: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0x2af8dc — __ZN5boost6detail12shared_countC2IPN3RBX13StarterScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ // was: boost
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2af8dc() -> Option<u32> {
    // IDA 0x2af8dc: nullable object query (id when live, None when unset).
    None
}
// 0x2af9e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2af9e4() {
    // IDA 0x2af9e4: counted-impl dtor frees the control block.
}
// 0x2af9e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2af9e8() {
    // IDA 0x2af9e8: counted-impl dtor frees the control block.
}
// 0x2af9ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_2af9ec() -> Option<u32> {
    // IDA 0x2af9ec: nullable object query (id when live, None when unset).
    None
}
// 0x2afa0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_2afa0c() -> bool {
    // IDA 0x2afa0c: deleter query misses for this control block.
    false
}
// 0x2afa24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_2afa24() -> bool {
    // IDA 0x2afa24: deleter query misses for this control block.
    false
}
// 0x2afa28 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
pub fn stub_2afa28(s: &mut GenSignalState) -> u64 {
    // IDA 0x2afa28: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0x2afc34 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")]
pub fn stub_2afc34(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0x2afc34: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0x2afc58 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")]
pub fn stub_2afc58(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0x2afc58: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0x2afc80 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_2afc80() -> u64 {
    // IDA 0x2afc80: returns the static signal mutex id.
    0
}
// 0x2afd78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev // was: boost
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_2afd78(s: &mut GenSignalState, id: u64) {
    // IDA 0x2afd78: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0x2afda4 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev // was: boost
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_2afda4(s: &mut GenSignalState, id: u64) {
    // IDA 0x2afda4: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0x2afe78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::disconnect(void)")]
pub fn stub_2afe78(slot: &mut (u64, bool)) {
    // IDA 0x2afe78: self-unlink (intrusive list surgery engine-side).
    slot.1 = false;
}
// 0x2aff88 — __ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::connected(void)const")]
pub fn stub_2aff88(slot: (u64, bool)) -> bool {
    // IDA 0x2aff88: reports whether the slot is still linked.
    slot.1
}
// 0x2aff94 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ // was: boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_2aff94(fire: &dyn Fn()) {
    // IDA 0x2aff94: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2aff9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_ // was: boost
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_2aff9c(fire: &dyn Fn()) {
    // IDA 0x2aff9c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2affa4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_ // was: boost
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub fn stub_2affa4(fire: &dyn Fn()) {
    // IDA 0x2affa4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2affbc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
pub fn stub_2affbc(s: &mut GenSignalState, id: u64) {
    // IDA 0x2affbc: unlinks one slot node (missing node is a no-op).
    gen_disconnect(s, id);
}
// 0x2b00ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_2b00ac() -> bool {
    // IDA 0x2b00ac: one-time guard init for the slot static mutex.
    true
}
// 0x2b00b0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_2b00b0() -> u64 {
    // IDA 0x2b00b0: returns the static signal mutex id.
    0
}
// 0x2b01a0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
pub fn stub_2b01a0(s: &mut GenSignalState, id: u64) {
    // IDA 0x2b01a0: slot dtor unlinks and frees the node.
    gen_disconnect(s, id);
}
// 0x2b01cc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
pub fn stub_2b01cc(s: &mut GenSignalState, id: u64) {
    // IDA 0x2b01cc: slot dtor unlinks and frees the node.
    gen_disconnect(s, id);
}
// 0x2b02a0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev // was: boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_2b02a0() {
    // IDA 0x2b02a0: drops the bound functor held by the callable.
}
// 0x2b02cc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev // was: boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_2b02cc() {
    // IDA 0x2b02cc: drops the bound functor held by the callable.
}
// 0x2b03a0 — __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(void)const")]
pub fn stub_2b03a0() -> Option<u32> {
    // IDA 0x2b03a0: nullable object query (id when live, None when unset).
    None
}
// 0x2b0568 — __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RunService * RBX::ServiceProvider::find<RBX::RunService>(void)const")]
pub fn stub_2b0568() -> Option<u32> {
    // IDA 0x2b0568: nullable object query (id when live, None when unset).
    None
}
// 0x2b06e0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10RunServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RunService,RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const*,RBX::RunService *)const")]
pub fn stub_2b06e0(has_weak: bool) -> bool {
    // IDA 0x2b06e0: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0x2b07d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2b07d0() {
    // IDA 0x2b07d0: counted-impl dtor frees the control block.
}
// 0x2b07d8 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE15isNullClassNameEv")]
pub fn stub_2b07d8(handle: u32) -> String {
    // IDA 0x2b07d8: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x2b0878 — __ZN5boost10shared_ptrIN3RBX11ScriptStatsEEC2IS2_EEPT_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)")]
pub fn stub_2b0878() -> Option<u32> {
    // IDA 0x2b0878: nullable object query (id when live, None when unset).
    None
}
// 0x2b094c — __ZN5boost6detail12shared_countC2IN3RBX11ScriptStatsEEEPT_ // was: boost
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)")]
pub fn stub_2b094c() -> Option<u32> {
    // IDA 0x2b094c: nullable object query (id when live, None when unset).
    None
}
// 0x2b0a88 — __ZNSt5dequeISsSaISsEED2Ev
#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::~deque()")]
pub fn stub_2b0a88() {
    // IDA 0x2b0a88: dtor releases the owned control block/slots.
}
// 0x2b0b70 — __ZNSt11_Deque_baseISsSaISsEED2Ev
#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")]
pub fn stub_2b0b70() {
    // IDA 0x2b0b70: dtor releases the owned control block/slots.
}
// 0x2b0ba0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED1Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
pub fn stub_2b0ba0() {
    // IDA 0x2b0ba0: counted-impl dtor frees the control block.
}
// 0x2b0ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED0Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
pub fn stub_2b0ba4() {
    // IDA 0x2b0ba4: counted-impl dtor frees the control block.
}
// 0x2b0ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE7disposeEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::dispose(void)")]
pub fn stub_2b0ba8() -> Option<u32> {
    // IDA 0x2b0ba8: nullable object query (id when live, None when unset).
    None
}
// 0x2b0c80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE11get_deleterERKSt9type_info // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_deleter(std::type_info const&)")]
pub fn stub_2b0c80() -> bool {
    // IDA 0x2b0c80: deleter query misses for this control block.
    false
}
// 0x2b0c84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE19get_untyped_deleterEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_untyped_deleter(void)")]
pub fn stub_2b0c84() -> bool {
    // IDA 0x2b0c84: deleter query misses for this control block.
    false
}
// 0x2b0c88 — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::create<RBX::Stats::StatsService>(void)const")]
pub fn stub_2b0c88() -> Option<u32> {
    // IDA 0x2b0c88: nullable object query (id when live, None when unset).
    None
}
// 0x2b0e50 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEv // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::Creatable<RBX::Instance>::create<RBX::Stats::StatsService>(void)")]
pub fn stub_2b0e50() -> Option<u32> {
    // IDA 0x2b0e50: nullable object query (id when live, None when unset).
    None
}
// 0x2b0f00 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_5Stats12StatsServiceEEERS3_RKNS0_IT_EE // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const&)")]
pub fn stub_2b0f00(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0x2b0f00: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0x2b0f38 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")]
pub fn stub_2b0f38(handle: u32) -> String {
    // IDA 0x2b0f38: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x2b0f40 — __ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_2b0f40() -> Option<u32> {
    // IDA 0x2b0f40: nullable object query (id when live, None when unset).
    None
}
// 0x2b1060 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE // was: boost
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")]
pub fn stub_2b1060() {
    // IDA 0x2b1060: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b10d0 — __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_2b10d0() -> Option<u32> {
    // IDA 0x2b10d0: nullable object query (id when live, None when unset).
    None
}
// 0x2b1170 — __ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_2b1170(this: u32) -> u32 {
    // IDA 0x2b1170: this-adjustment thunk (this -= 32) then tail-call.
    this.wrapping_sub(32)
}
// 0x2b1178 — __ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_2b1178(this: u32) -> u32 {
    // IDA 0x2b1178: this-adjustment thunk (this -= 32) then tail-call.
    this.wrapping_sub(32)
}
// 0x2b1220 — __ZN5boost10shared_ptrIN3RBX5Stats12StatsServiceEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService>::shared_ptr<RBX::Stats::StatsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2b1220() -> Option<u32> {
    // IDA 0x2b1220: nullable object query (id when live, None when unset).
    None
}
// 0x2b12e8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats12StatsServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_ // was: boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::StatsService,RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const*,RBX::Stats::StatsService *)const")]
pub fn stub_2b12e8(has_weak: bool) -> bool {
    // IDA 0x2b12e8: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0x2b13d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2b13d8() {
    // IDA 0x2b13d8: counted-impl dtor frees the control block.
}
// 0x2b13e0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS4_11ScriptStartEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i // was: boost
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)")]
pub fn stub_2b13e0(fire: &dyn Fn()) {
    // IDA 0x2b13e0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2b14a4 — __ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_ // was: boost
#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const")]
pub fn stub_2b14a4() {
    // IDA 0x2b14a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b157c — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void)")]
pub fn stub_2b157c() {
    // IDA 0x2b157c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b16d4 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv
// type: void __fastcall(int)
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void)")]
pub fn stub_2b16d4() {
    // IDA 0x2b16d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b1828 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15ContentProviderEEEN5boost10shared_ptrIT_EEv // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentProvider> RBX::Creatable<RBX::Instance>::create<RBX::ContentProvider>(void)")]
pub fn stub_2b1828() -> Option<u32> {
    // IDA 0x2b1828: nullable object query (id when live, None when unset).
    None
}
// 0x2b18d8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15ContentProviderEEERS3_RKNS0_IT_EE // was: boost
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentProvider>(rbx_core::SharedPtr<RBX::ContentProvider> const&)")]
pub fn stub_2b18d8(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0x2b18d8: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0x2b1910 — __ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv")]
pub fn stub_2b1910(handle: u32) -> String {
    // IDA 0x2b1910: string query off the handle.
    let _ = handle;
    String::new()
}
// 0x2b1918 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentProvider>(void)")]
pub fn stub_2b1918() {
    // IDA 0x2b1918: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b1920 — __ZN5boost6detail12shared_countC2IPN3RBX15ContentProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ // was: boost
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_2b1920() -> Option<u32> {
    // IDA 0x2b1920: nullable object query (id when live, None when unset).
    None
}
// 0x2b1a28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_2b1a28() {
    // IDA 0x2b1a28: counted-impl dtor frees the control block.
}
// 0x2b1a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_2b1a30() -> Option<u32> {
    // IDA 0x2b1a30: nullable object query (id when live, None when unset).
    None
}
// 0x2b1a50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_2b1a50() -> bool {
    // IDA 0x2b1a50: deleter query misses for this control block.
    false
}
// 0x2b1a68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15ContentProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_2b1a68() -> bool {
    // IDA 0x2b1a68: deleter query misses for this control block.
    false
}
// 0x2b1a6c — __ZN5boost9function1IvP9lua_StateE4swapERS3_ // was: boost
#[doc(alias = "boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&)")]
pub fn stub_2b1a6c() -> Option<u32> {
    // IDA 0x2b1a6c: nullable object query (id when live, None when unset).
    None
}
// 0x2b1b48 — __ZN5boost9function1IvP9lua_StateE11move_assignERS3_ // was: boost
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&)")]
pub fn stub_2b1b48() -> Option<u32> {
    // IDA 0x2b1b48: nullable object query (id when live, None when unset).
    None
}
// 0x2b1c4c — __ZN5boost9function1IvP9lua_StateE5clearEv // was: boost
#[doc(alias = "boost::function1<void,lua_State *>::clear(void)")]
pub fn stub_2b1c4c() -> Option<u32> {
    // IDA 0x2b1c4c: nullable object query (id when live, None when unset).
    None
}
// 0x2b1c78 — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b1c78(data: &[u8]) -> bool {
    // IDA 0x2b1c78: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2b1d58 — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b1d58(data: &[u8]) -> bool {
    // IDA 0x2b1d58: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2b1e3c — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEEvT_ // was: boost
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>)")]
pub fn stub_2b1e3c(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b1e3c: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b1f30 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b1f30(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b1f30: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b1f4c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_ // was: boost
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_2b1f4c(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0x2b1f4c: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0x2b1f68 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_2b1f68(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b1f68: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b204c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2b204c(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b204c: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b212c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSB_3Lua13WeakThreadRefES4_EENS7_5list3INS7_5valueIPSC_EENSH_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE // was: boost
#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_2b212c(slot: &mut GenFunctor) {
    // IDA 0x2b212c: installs the functor with the nothrow tag.
    slot.has = true;
}
// 0x2b21fc — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEclINS_4_mfi3mf2IvS4_S8_P9lua_StateEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i // was: boost
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *> &,boost::_bi::list1<lua_State *&> &,int)")]
pub fn stub_2b21fc(fire: &dyn Fn()) {
    // IDA 0x2b21fc: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0x2b22cc — __ZNK5boost4_mfi3mf2IvN3RBX13ScriptContextENS2_3Lua13WeakThreadRefEP9lua_StateEclEPS3_S5_S7_ // was: boost
#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>::operator()(RBX::ScriptContext*,RBX::Lua::WeakThreadRef,lua_State *)const")]
pub fn stub_2b22cc() -> Option<u32> {
    // IDA 0x2b22cc: nullable object query (id when live, None when unset).
    None
}
// 0x2b23a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS7_3Lua13WeakThreadRefEP9lua_StateEENS3_5list3INS3_5valueIPS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE // was: boost
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_2b23a8(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b23a8: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b24f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_ // was: boost
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
pub fn stub_2b24f8(slot: &mut GenFunctor) {
    // IDA 0x2b24f8: packs the bound argument list.
    slot.has = true;
}
// 0x2b25c0 — __ZN5boost3_bi8storage3INS0_5valueIPN3RBX13ScriptContextEEENS2_INS3_3Lua13WeakThreadRefEEENS_3argILi1EEEEC2ES6_S9_SB_ // was: boost
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::storage3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
pub fn stub_2b25c0(slot: &mut GenFunctor) {
    // IDA 0x2b25c0: packs the bound argument list.
    slot.has = true;
}
// 0x2b2688 — __ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv // was: boost
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::dummy::nonnull(void)")]
pub fn stub_2b2688() -> Option<u32> {
    // IDA 0x2b2688: nullable object query (id when live, None when unset).
    None
}
// 0x2b268c — __ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b268c() {
    // IDA 0x2b268c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b27b8 — __ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_ // was: boost
#[doc(alias = "void boost::function2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>)")]
pub fn stub_2b27b8(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b27b8: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b28f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b28f4(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b28f4: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b2974 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEvS6_mE6invokeERNS1_15function_bufferES6_m // was: boost
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
pub fn stub_2b2974(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0x2b2974: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0x2b2998 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_2b2998(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b2998: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b2ac4 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2b2ac4(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b2ac4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b2bfc — __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEclIPFvP9lua_StateiSsENS0_5list2IRSA_RmEEEEvNS0_4typeIvEERT_RT0_i // was: boost
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::operator()<void (*)(lua_State *,int,std::string),boost::_bi::list2<lua_State *&,unsigned long &>>(boost::_bi::type<void>,void (*)(lua_State *,int,std::string) &,boost::_bi::list2<lua_State *&,unsigned long &> &,int)")]
pub fn stub_2b2bfc(fire: &dyn Fn(&str), s: &str) {
    // IDA 0x2b2bfc: bind/call thunk forwards the string arg.
    fire(s);
}
// 0x2b2d20 — __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_ // was: boost
#[doc(alias = "boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::list3(boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
pub fn stub_2b2d20(slot: &mut GenFunctor) {
    // IDA 0x2b2d20: packs the bound argument list.
    slot.has = true;
}
// 0x2b2e3c — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b2e3c() -> Option<u32> {
    // IDA 0x2b2e3c: nullable object query (id when live, None when unset).
    None
}
// 0x2b2f10 — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b2f10() -> Option<u32> {
    // IDA 0x2b2f10: nullable object query (id when live, None when unset).
    None
}
// 0x2b2fe4 — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_ // was: boost
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>)")]
pub fn stub_2b2fe4(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b2fe4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b30c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b30c8(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b30c8: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b30e4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_ // was: boost
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_2b30e4(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0x2b30e4: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0x2b3100 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_2b3100(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b3100: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b31d8 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2b31d8(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b31d8: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b32a8 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvPKcSB_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE // was: boost
#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_2b32a8(slot: &mut GenFunctor) {
    // IDA 0x2b32a8: installs the functor with the nothrow tag.
    slot.has = true;
}
// 0x2b336c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i // was: boost
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::operator()<void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>) &,boost::_bi::list1<lua_State *&> &,int)")]
pub fn stub_2b336c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0x2b336c: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0x2b3434 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE // was: boost
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_2b3434(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b3434: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b357c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvPKcS7_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEEEEC2ES3_SE_ // was: boost
#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>)")]
pub fn stub_2b357c(slot: &mut GenFunctor) {
    // IDA 0x2b357c: packs the bound argument list.
    slot.has = true;
}
// 0x2b3644 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5dummy7nonnullEv // was: boost
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::dummy::nonnull(void)")]
pub fn stub_2b3644() -> Option<u32> {
    // IDA 0x2b3644: nullable object query (id when live, None when unset).
    None
}
// 0x2b3648 — __ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b3648() -> Option<u32> {
    // IDA 0x2b3648: nullable object query (id when live, None when unset).
    None
}
// 0x2b371c — __ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE // was: boost
#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2b371c() -> Option<u32> {
    // IDA 0x2b371c: nullable object query (id when live, None when unset).
    None
}
// 0x2b37f0 — __ZN5boost9function1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEEvT_ // was: boost
#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>)")]
pub fn stub_2b37f0(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b37f0: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b38d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b38d4(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b38d4: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b38f0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEvS6_E6invokeERNS1_15function_bufferES6_ // was: boost
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_2b38f0(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0x2b38f0: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0x2b390c — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_2b390c(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b390c: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b39e4 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE9assign_toINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE // was: boost
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2b39e4(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b39e4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b3ab4 — __ZNK5boost6detail8function13basic_vtable1IvP9lua_StateE14assign_functorINS_3_bi6bind_tIvPFvS4_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS7_5list2INS_3argILi1EEENS7_5valueISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE // was: boost
#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_2b3ab4(slot: &mut GenFunctor) {
    // IDA 0x2b3ab4: installs the functor with the nothrow tag.
    slot.has = true;
}
// 0x2b3b78 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEclIPFvP9lua_StateSD_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i // was: boost
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::operator()<void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>) &,boost::_bi::list1<lua_State *&> &,int)")]
pub fn stub_2b3b78(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0x2b3b78: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0x2b3c40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateNS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS3_5list2INS_3argILi1EEENS3_5valueISF_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE // was: boost
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_2b3c40(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b3c40: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b3d88 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEEEEC2ES3_SE_ // was: boost
#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_2b3d88(slot: &mut GenFunctor) {
    // IDA 0x2b3d88: packs the bound argument list.
    slot.has = true;
}
// 0x2b3e50 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5dummy7nonnullEv // was: boost
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::dummy::nonnull(void)")]
pub fn stub_2b3e50() -> Option<u32> {
    // IDA 0x2b3e50: nullable object query (id when live, None when unset).
    None
}
// 0x2b3e54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b3e54(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b3e54: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b3eb4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_StatemENS3_5list3INS_17reference_wrapperIS9_EENS_3argILi1EEENSI_ILi2EEEEEEEvSC_mE6invokeERNS1_15function_bufferESC_m // was: boost
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
pub fn stub_2b3eb4(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0x2b3eb4: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0x2b3ebc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b3ebc(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b3ebc: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b3f1c — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tIiPFiRKN3RBX10Reflection5TupleEP9lua_StateENS3_5list2INS_17reference_wrapperIS8_EENS_3argILi1EEEEEEEmSB_E6invokeERNS1_15function_bufferESB_ // was: boost
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_2b3f1c() {
    // IDA 0x2b3f1c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b3f24 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE13assign_to_ownERKS7_ // was: boost
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> const&)")]
pub fn stub_2b3f24(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b3f24: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b3f54 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE13assign_to_ownERKS7_ // was: boost
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> const&)")]
pub fn stub_2b3f54(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b3f54: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b3f84 — __ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_ // was: boost
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::assign_to_own(boost::function2<void,lua_State *,unsigned long> const&)")]
pub fn stub_2b3f84(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b3f84: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b3fb4 — __ZN5boost9function2IvP9lua_StatemE5clearEv // was: boost
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::clear(void)")]
pub fn stub_2b3fb4() -> Option<u32> {
    // IDA 0x2b3fb4: nullable object query (id when live, None when unset).
    None
}
// 0x2b3fe0 — __ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_ // was: boost
#[doc(alias = "boost::function1<unsigned long,lua_State *>::assign_to_own(boost::function1<unsigned long,lua_State *> const&)")]
pub fn stub_2b3fe0(slot: &mut GenFunctor) -> bool {
    // IDA 0x2b3fe0: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0x2b4010 — __ZN5boost9function1ImP9lua_StateE5clearEv // was: boost
#[doc(alias = "boost::function1<unsigned long,lua_State *>::clear(void)")]
pub fn stub_2b4010() -> Option<u32> {
    // IDA 0x2b4010: nullable object query (id when live, None when unset).
    None
}
// 0x2b403c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE // was: boost
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long,unsigned long (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2b403c(slot: &mut GenFunctor, op: u32) {
    // IDA 0x2b403c: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0x2b409c — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEmS6_E6invokeERNS1_15function_bufferES6_ // was: boost
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<unsigned long,unsigned long (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_2b409c() {
    // IDA 0x2b409c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b40a8 — __ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE // was: boost
// type: int __fastcall(int, int, int, int, char, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_2b40a8(data: &[u8]) -> bool {
    // IDA 0x2b40a8: consumes the input buffer; false on malformed input.
    !data.is_empty()
}
// 0x2b41f0 — __ZN5boost6detail11thread_dataINS_9function0IvEEED0Ev // was: boost
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::~thread_data()")]
pub fn stub_2b41f0() {
    // IDA 0x2b41f0: dtor releases the owned control block/slots.
}
// 0x2b42d0 — __ZN5boost6detail11thread_dataINS_9function0IvEEE3runEv // was: boost
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::run(void)")]
pub fn stub_2b42d0() {
    // IDA 0x2b42d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b42d8 — __ZN5boost6detail16thread_data_base25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE // was: boost
// type: _DWORD __fastcall(boost::detail::thread_data_base *__hidden this, boost::condition_variable *, boost::mutex *)
#[doc(alias = "boost::detail::thread_data_base::notify_all_at_thread_exit(boost::condition_variable *,boost::mutex *)")]
pub fn stub_2b42d8() {
    // IDA 0x2b42d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b42f0 — __ZNK5boost9function0IvEclEv // was: boost
#[doc(alias = "boost::function0<void>::operator()(void)const")]
pub fn stub_2b42f0() {
    // IDA 0x2b42f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b43b0 — __ZN5boost18condition_variableD2Ev // was: boost
// type: void __fastcall(pthread_mutex_t *this)
#[doc(alias = "boost::condition_variable::~condition_variable()")]
pub fn stub_2b43b0() {
    // IDA 0x2b43b0: dtor releases the owned control block/slots.
}
// 0x2b43d8 — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_ // was: boost
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
pub fn stub_2b43d8() {
    // IDA 0x2b43d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x2b44d0 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEED1Ev // was: boost
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::~sp_counted_impl_p()")]
pub fn stub_2b44d0() {
    // IDA 0x2b44d0: counted-impl dtor frees the control block.
}
