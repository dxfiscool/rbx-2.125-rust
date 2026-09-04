//! network generated_188 — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Filter: RakNet|Network|Replicator -> 5109 funcs, 650 remaining before batch; batch EA-sorted asc 150 gap filler
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0xa9d108..0xaba5ac | existing 20709 -> 20859 total (rbx_core::SharedPtr not boost)

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



// 0xa9d108 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player14MembershipTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType> const&)")]
pub fn stub_a9d108() -> Option<u32> {
    // IDA 0xa9d108: nullable object query (id when live, None when unset).
    None
}
// 0xa9d2bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player14MembershipTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType> const&)")]
pub fn stub_a9d2bc() -> Option<u32> {
    // IDA 0xa9d2bc: nullable object query (id when live, None when unset).
    None
}
// 0xa9d3ac — __ZNSt6vectorIN3RBX7Network6Player14MembershipTypeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Player::MembershipType*,std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>>,RBX::Network::Player::MembershipType const&)")]
pub fn stub_a9d3ac(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0xa9d3ac: vector insert with reallocation around the new element.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0xa9d4bc — __ZNSt6vectorIN3RBX7Network6Player14MembershipTypeESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Player::MembershipType*,std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>>,unsigned long,RBX::Network::Player::MembershipType const&)")]
pub fn stub_a9d4bc() -> Option<u32> {
    // IDA 0xa9d4bc: nullable object query (id when live, None when unset).
    None
}
// 0xaa2324 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceEbi
// type: int __fastcall(int, int, char, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::replicateEvent(RBX::Reflection::EventSource *,bool,int)")]
pub fn stub_aa2324(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaa2324: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaa2d08 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6PlayerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_aa2d08(s: &mut GenSignalState, id: u64) {
    // IDA 0xaa2d08: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xaa2d64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6PlayerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_aa2d64(s: &mut GenSignalState, id: u64) {
    // IDA 0xaa2d64: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xaa2e6c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6PlayerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_aa2e6c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xaa2e6c: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xaa2f88 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6PlayerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_aa2f88(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xaa2f88: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xaa31f4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network6PlayerENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_aa31f4() -> Option<u32> {
    // IDA 0xaa31f4: nullable object query (id when live, None when unset).
    None
}
// 0xaa4bf0 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvRNSA_8weak_ptrINS2_7Network6PlayerEEEPKNS2_15ServiceProviderEENSB_5list2INSB_5valueISG_EENSO_ISK_EEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")]
pub fn stub_aa4bf0(s: &mut GenSignalState, id: u64) {
    // IDA 0xaa4bf0: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xaa4bfc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvRNSA_8weak_ptrINS2_7Network6PlayerEEEPKNS2_15ServiceProviderEENSB_5list2INSB_5valueISG_EENSO_ISK_EEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")]
pub fn stub_aa4bfc(s: &mut GenSignalState, id: u64) {
    // IDA 0xaa4bfc: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xaa4cb0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_E4callES7_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_aa4cb0(fire: &dyn Fn()) {
    // IDA 0xaa4cb0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xaa4cc0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_E4callES7_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_aa4cc0(fire: &dyn Fn()) {
    // IDA 0xaa4cc0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xaa4cd0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_aa4cd0() {
    // IDA 0xaa4cd0: drops the bound functor held by the callable.
}
// 0xaa4ea8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_aa4ea8() {
    // IDA 0xaa4ea8: drops the bound functor held by the callable.
}
// 0xaa4eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_aa4eb4() {
    // IDA 0xaa4eb4: drops the bound functor held by the callable.
}
// 0xaa5318 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>)")]
pub fn stub_aa5318(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa5318: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa5500 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa5500(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa5500: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa5524 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_aa5524(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xaa5524: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xaa5534 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS8_15ServiceProviderEENS5_5list2INS5_5valueISB_EENSJ_ISF_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_aa5534(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa5534: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa5708 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS8_15ServiceProviderEENS5_5list2INS5_5valueISB_EENSJ_ISF_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_aa5708(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa5708: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa5940 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa5940(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa5940: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa5b38 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_IPKNS4_15ServiceProviderEEEEC2ES8_SC_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>::list2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>)")]
pub fn stub_aa5b38(slot: &mut GenFunctor) {
    // IDA 0xaa5b38: packs the bound argument list.
    slot.has = true;
}
// 0xaa5cfc — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_IPKNS4_15ServiceProviderEEEEC2ES8_SC_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>)")]
pub fn stub_aa5cfc(slot: &mut GenFunctor) {
    // IDA 0xaa5cfc: packs the bound argument list.
    slot.has = true;
}
// 0xaa60bc — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ERKSF_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::storage4(boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>> const&)")]
pub fn stub_aa60bc(slot: &mut GenFunctor) {
    // IDA 0xaa60bc: packs the bound argument list.
    slot.has = true;
}
// 0xaa6274 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS6_5list4INS_3argILi1EEENSI_ILi2EEENS6_5valueISC_EENSL_ISE_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>)")]
pub fn stub_aa6274(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa6274: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa662c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa662c(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa662c: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa6650 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
pub fn stub_aa6650(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xaa6650: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xaa6670 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEclIPFvPSsPSt9exceptionSA_SD_ENS0_5list2IRSH_RSJ_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(pthread_mutex_t *, int, int *, int *), pthread_mutex_t ***, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::operator()<void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
pub fn stub_aa6670(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xaa6670: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xaa6970 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa6970(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa6970: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa6ab4 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::~storage4()")]
pub fn stub_aa6ab4() {
    // IDA 0xaa6ab4: argument-list dtor (values released).
}
// 0xaa6c74 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ES3_S4_SB_SE_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>)")]
pub fn stub_aa6c74() -> Option<u32> {
    // IDA 0xaa6c74: nullable object query (id when live, None when unset).
    None
}
// 0xaa6f70 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ES3_S4_SB_SE_
// type: pthread_mutex_t **__fastcall(pthread_mutex_t **, pthread_mutex_t **, pthread_mutex_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>)")]
pub fn stub_aa6f70(slot: &mut GenFunctor) {
    // IDA 0xaa6f70: packs the bound argument list.
    slot.has = true;
}
// 0xaa71e4 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEclIPFvS7_NS3_INS4_8InstanceEEEbENS0_5list1IRNS_10shared_ptrISE_EEEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int *, _DWORD), int **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> &>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> &> &,int)")]
pub fn stub_aa71e4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xaa71e4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xaa74dc — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEC2ES8_SA_SB_
// type: int __fastcall(int, int *, unsigned __int8, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>)")]
pub fn stub_aa74dc(slot: &mut GenFunctor) {
    // IDA 0xaa74dc: packs the bound argument list.
    slot.has = true;
}
// 0xaa76a4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEC2ES8_SA_SB_
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>)")]
pub fn stub_aa76a4(slot: &mut GenFunctor) {
    // IDA 0xaa76a4: packs the bound argument list.
    slot.has = true;
}
// 0xaa786c — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>)")]
pub fn stub_aa786c(slot: &mut GenFunctor) {
    // IDA 0xaa786c: packs the bound argument list.
    slot.has = true;
}
// 0xaa7e00 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>)")]
pub fn stub_aa7e00(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa7e00: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa8278 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa8278(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa8278: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa829c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_aa829c(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xaa829c: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xaa82bc — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_aa82bc(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa82bc: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa85a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa85a4(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa85a4: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa8738 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>)")]
pub fn stub_aa8738() -> Option<u32> {
    // IDA 0xaa8738: nullable object query (id when live, None when unset).
    None
}
// 0xaa8f2c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa8f2c(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa8f2c: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa8f50 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(_DWORD *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_aa8f50(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xaa8f50: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xaa8f68 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENSE_IbEENSE_IPKcEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_aa8f68(slot: &mut GenFunctor) -> bool {
    // IDA 0xaa8f68: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaa9408 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEENS2_IbEENS2_IPKcEEEclINS_4_mfi3mf2IvS6_bSsEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(_DWORD *, void (__fastcall **)(int))
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>::operator()<boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string> &,boost::_bi::list0 &,int)")]
pub fn stub_aa9408(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xaa9408: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xaa95e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa95e0(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaa95e0: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaa9780 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEENS2_IbEENS2_IPKcEEEC2ES8_S9_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>)")]
pub fn stub_aa9780(slot: &mut GenFunctor) {
    // IDA 0xaa9780: packs the bound argument list.
    slot.has = true;
}
// 0xaa9be0 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEENS2_IbEEEC2ES8_S9_
// type: int __fastcall(int, unsigned int *, char, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>)")]
pub fn stub_aa9be0(slot: &mut GenFunctor) {
    // IDA 0xaa9be0: packs the bound argument list.
    slot.has = true;
}
// 0xaa9e38 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6PlayerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")]
pub fn stub_aa9e38(s: &mut GenSignalState, id: u64) {
    // IDA 0xaa9e38: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xaa9e94 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6PlayerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")]
pub fn stub_aa9e94(s: &mut GenSignalState, id: u64) {
    // IDA 0xaa9e94: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xaa9f9c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6PlayerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")]
pub fn stub_aa9f9c(fire: &dyn Fn()) {
    // IDA 0xaa9f9c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xaa9fb8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6PlayerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")]
pub fn stub_aa9fb8(fire: &dyn Fn()) {
    // IDA 0xaa9fb8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xaaa378 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSH_5list2INSH_5valueISM_EENS_3argILi1EEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>)")]
pub fn stub_aaa378(slot: &mut GenFunctor) -> bool {
    // IDA 0xaaa378: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaaa55c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aaa55c(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaaa55c: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaaa580 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEvSM_E6invokeERNS1_15function_bufferESM_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>,void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_aaa580(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xaaa580: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xaaa598 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS5_7Network6PlayerEEESG_ENSJ_5list2INSJ_5valueISO_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_aaa598(slot: &mut GenFunctor) -> bool {
    // IDA 0xaaa598: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaaa764 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS5_7Network6PlayerEEESG_ENSJ_5list2INSJ_5valueISO_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_aaa764(slot: &mut GenFunctor) -> bool {
    // IDA 0xaaa764: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xaaa960 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEclIPFvS7_NS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSG_EEEEEENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, __int32 *), pthread_mutex_t **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) &,boost::_bi::list1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&> &,int)")]
pub fn stub_aaa960(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xaaa960: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xaaad08 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aaad08(slot: &mut GenFunctor, op: u32) {
    // IDA 0xaaad08: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xaaae28 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEC2ES8_SA_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>)")]
pub fn stub_aaae28(slot: &mut GenFunctor) {
    // IDA 0xaaae28: packs the bound argument list.
    slot.has = true;
}
// 0xaaafe8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network19PersistentDataStoreES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PersistentDataStore,RBX::Network::PersistentDataStore>(rbx_core::SharedPtr<RBX::Network::PersistentDataStore> *,RBX::Network::PersistentDataStore *,boost::detail::shared_count &)")]
pub fn stub_aaafe8(slot: &mut Option<u32>, v: u32) {
    // IDA 0xaaafe8: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xaab194 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::~sp_counted_impl_p()")]
pub fn stub_aab194() {
    // IDA 0xaab194: counted-impl dtor frees the control block.
}
// 0xaab198 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::~sp_counted_impl_p()")]
pub fn stub_aab198() {
    // IDA 0xaab198: counted-impl dtor frees the control block.
}
// 0xaab1a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEE7disposeEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::dispose(void)")]
pub fn stub_aab1a4(slot: &mut Option<u32>) {
    // IDA 0xaab1a4: disposes the managed object (intrusive counts engine-side).
    *slot = None;
}
// 0xaab24c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::get_deleter(std::type_info const&)")]
pub fn stub_aab24c() -> bool {
    // IDA 0xaab24c: deleter query misses for this control block.
    false
}
// 0xaab250 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::get_untyped_deleter(void)")]
pub fn stub_aab250() -> bool {
    // IDA 0xaab250: deleter query misses for this control block.
    false
}
// 0xaab35c — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceESs
// type: int __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
pub fn stub_aab35c(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaab35c: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaad1dc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEEC2IMS3_KFS5_vEMS3_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, __guard *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::EnumPropDescriptor<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>(char const*,char const*,RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_aad1dc(name: &str) -> GenDesc {
    // IDA 0xaad1dc: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad4dc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::~EnumPropDescriptor()")]
pub fn stub_aad4dc(d: GenDesc) {
    // IDA 0xaad4dc: prop descriptor dtor.
    let _ = d;
}
// 0xaad504 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::isReadOnly(void)const")]
pub fn stub_aad504(d: &GenDesc) -> bool {
    // IDA 0xaad504: read-only when no setter was installed.
    !d.writable
}
// 0xaad514 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::isWriteOnly(void)const")]
pub fn stub_aad514(d: &GenDesc) -> bool {
    // IDA 0xaad514: write-only when no getter was installed.
    !d.readable
}
// 0xaad524 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE11equalValuesEPKNS0_13DescribedBaseES9_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aad524(name: &str) -> GenDesc {
    // IDA 0xaad524: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad54c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_aad54c(name: &str) -> GenDesc {
    // IDA 0xaad54c: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad5fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_aad5fc(name: &str) -> GenDesc {
    // IDA 0xaad5fc: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad618 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE9copyValueEPKNS0_13DescribedBaseEPS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_aad618(name: &str) -> GenDesc {
    // IDA 0xaad618: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::hasStringValue(void)const")]
pub fn stub_aad63c(name: &str) -> GenDesc {
    // IDA 0xaad63c: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad640 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aad640(name: &str) -> GenDesc {
    // IDA 0xaad640: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad664 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_aad664(name: &str) -> GenDesc {
    // IDA 0xaad664: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad708 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_aad708(name: &str) -> GenDesc {
    // IDA 0xaad708: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad728 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_aad728(name: &str) -> GenDesc {
    // IDA 0xaad728: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaad9ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aad9ec(name: &str) -> GenDesc {
    // IDA 0xaad9ec: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaada68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_aada68(name: &str) -> GenDesc {
    // IDA 0xaada68: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaada9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aada9c(name: &str) -> GenDesc {
    // IDA 0xaada9c: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaadaac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_aadaac(name: &str) -> GenDesc {
    // IDA 0xaadaac: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaadb78 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aadb78(name: &str) -> GenDesc {
    // IDA 0xaadb78: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaadb98 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_aadb98(name: &str) -> GenDesc {
    // IDA 0xaadb98: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xaadc2c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isReadOnly(void)const")]
pub fn stub_aadc2c(d: &GenDesc) -> bool {
    // IDA 0xaadc2c: read-only when no setter was installed.
    !d.writable
}
// 0xaadc30 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isWriteOnly(void)const")]
pub fn stub_aadc30(d: &GenDesc) -> bool {
    // IDA 0xaadc30: write-only when no getter was installed.
    !d.readable
}
// 0xaadc34 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aadc34(d: &GenDesc) -> i32 {
    // IDA 0xaadc34: virtual getter dispatch; returns the scalar.
    d.value
}
// 0xaadc58 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraMode const&)const")]
pub fn stub_aadc58(d: &mut GenDesc, v: i32) {
    // IDA 0xaadc58: virtual setter dispatch; stores the scalar.
    d.value = v;
}
// 0xaadc80 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_aadc80(d: GenDesc) {
    // IDA 0xaadc80: event descriptor dtor.
    let _ = d;
}
// 0xaadd5c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(_DWORD *, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_aadd5c(s: &mut GenSignalState) -> u64 {
    // IDA 0xaadd5c: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xaae1f4 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_aae1f4(d: &GenDesc) -> bool {
    // IDA 0xaae1f4: scriptable flag from the descriptor.
    d.scriptable
}
// 0xaae1fc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_aae1fc(d: &GenDesc) -> bool {
    // IDA 0xaae1fc: broadcast flag from the descriptor.
    d.broadcast
}
// 0xaae204 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aae204(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaae204: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaae40c — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aae40c(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaae40c: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaae424 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_aae424(s: &mut GenSignalState) {
    // IDA 0xaae424: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xaae600 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_aae600(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaae600: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaae8b0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_aae8b0(d: GenDesc) {
    // IDA 0xaae8b0: event descriptor dtor.
    let _ = d;
}
// 0xaae8f8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_aae8f8(d: GenDesc) {
    // IDA 0xaae8f8: event descriptor dtor.
    let _ = d;
}
// 0xaae9d4 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::~RemoteEventDesc()")]
pub fn stub_aae9d4(d: GenDesc) {
    // IDA 0xaae9d4: event descriptor dtor.
    let _ = d;
}
// 0xaaeab0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_aaeab0(s: &mut GenSignalState) -> u64 {
    // IDA 0xaaeab0: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xaaef48 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isScriptable(void)const")]
pub fn stub_aaef48(d: &GenDesc) -> bool {
    // IDA 0xaaef48: scriptable flag from the descriptor.
    d.scriptable
}
// 0xaaef50 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isBroadcast(void)const")]
pub fn stub_aaef50(d: &GenDesc) -> bool {
    // IDA 0xaaef50: broadcast flag from the descriptor.
    d.broadcast
}
// 0xaaef58 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aaef58(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaaef58: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaaf1cc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aaf1cc(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaaf1cc: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaaf1e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_aaf1e4(s: &mut GenSignalState) {
    // IDA 0xaaf1e4: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xab12d4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab12d4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab12d4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab1670 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab1670(d: GenDesc) {
    // IDA 0xab1670: event descriptor dtor.
    let _ = d;
}
// 0xab16b8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab16b8(d: GenDesc) {
    // IDA 0xab16b8: event descriptor dtor.
    let _ = d;
}
// 0xab1794 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::~RemoteEventDesc()")]
pub fn stub_ab1794(d: GenDesc) {
    // IDA 0xab1794: event descriptor dtor.
    let _ = d;
}
// 0xab1870 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab1870(s: &mut GenSignalState) -> u64 {
    // IDA 0xab1870: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab1d08 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isScriptable(void)const")]
pub fn stub_ab1d08(d: &GenDesc) -> bool {
    // IDA 0xab1d08: scriptable flag from the descriptor.
    d.scriptable
}
// 0xab1d10 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isBroadcast(void)const")]
pub fn stub_ab1d10(d: &GenDesc) -> bool {
    // IDA 0xab1d10: broadcast flag from the descriptor.
    d.broadcast
}
// 0xab1d18 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab1d18(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab1d18: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab2108 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab2108(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab2108: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab2120 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab2120(s: &mut GenSignalState) {
    // IDA 0xab2120: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xab412c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, RBX::Name *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab412c(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab412c: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab45b4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab45b4(d: GenDesc) {
    // IDA 0xab45b4: event descriptor dtor.
    let _ = d;
}
// 0xab45fc — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab45fc(d: GenDesc) {
    // IDA 0xab45fc: event descriptor dtor.
    let _ = d;
}
// 0xab46d8 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
pub fn stub_ab46d8(d: GenDesc) {
    // IDA 0xab46d8: event descriptor dtor.
    let _ = d;
}
// 0xab47b4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, void *, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab47b4(s: &mut GenSignalState) -> u64 {
    // IDA 0xab47b4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab4fd4 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_ab4fd4(d: &GenDesc) -> bool {
    // IDA 0xab4fd4: scriptable flag from the descriptor.
    d.scriptable
}
// 0xab4fdc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
pub fn stub_ab4fdc(d: &GenDesc) -> bool {
    // IDA 0xab4fdc: broadcast flag from the descriptor.
    d.broadcast
}
// 0xab4fe4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: int __fastcall(int, int, __int64)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab4fe4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab4fe4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab5058 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab5058(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab5058: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab5070 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab5070(s: &mut GenSignalState) {
    // IDA 0xab5070: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xab524c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab524c(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab524c: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab55e8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab55e8(d: GenDesc) {
    // IDA 0xab55e8: event descriptor dtor.
    let _ = d;
}
// 0xab56c4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab56c4(s: &mut GenSignalState) -> u64 {
    // IDA 0xab56c4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab5b48 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab5b48(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab5b48: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab5f20 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab5f20(s: &mut GenSignalState) {
    // IDA 0xab5f20: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xab60e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
// type: void __fastcall(_DWORD *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const")]
pub fn stub_ab60e4(s: &mut GenSignalState) -> u64 {
    // IDA 0xab60e4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab8950 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(double)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab8950(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab8950: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab8c00 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab8c00(d: GenDesc) {
    // IDA 0xab8c00: event descriptor dtor.
    let _ = d;
}
// 0xab8cdc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab8cdc(s: &mut GenSignalState) -> u64 {
    // IDA 0xab8cdc: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab9160 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab9160(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab9160: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab92d4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab92d4(s: &mut GenSignalState) {
    // IDA 0xab92d4: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xab92ec — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(double)> const&)const")]
pub fn stub_ab92ec(s: &mut GenSignalState) -> u64 {
    // IDA 0xab92ec: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab94c0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab94c0(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab94c0: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xab9770 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab9770(d: GenDesc) {
    // IDA 0xab9770: event descriptor dtor.
    let _ = d;
}
// 0xab984c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab984c(s: &mut GenSignalState) -> u64 {
    // IDA 0xab984c: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xab9cd0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab9cd0(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xab9cd0: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaba024 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_aba024(s: &mut GenSignalState) {
    // IDA 0xaba024: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xaba03c — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
pub fn stub_aba03c(s: &mut GenSignalState) -> u64 {
    // IDA 0xaba03c: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xaba210 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_aba210(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xaba210: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xaba5ac — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_aba5ac(d: GenDesc) {
    // IDA 0xaba5ac: event descriptor dtor.
    let _ = d;
}
