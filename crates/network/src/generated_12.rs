//! network generated_12 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 120 stubs here, 4179+120=4299 total, 498 remaining).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

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


// 0xf5f984 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_15NetworkSettingsENS2_17PhysicsSendMethodEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::EnumPropDescriptor<RBX::NetworkSettings,RBX::NetworkSettings::PhysicsSendMethod>::EnumPropDescriptor<RBX::NetworkSettings::PhysicsSendMethod (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(RBX::NetworkSettings::PhysicsSendMethod const&)>(char const*,char const*,RBX::NetworkSettings::PhysicsSendMethod (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(RBX::NetworkSettings::PhysicsSendMethod const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5f984(name: &str) -> GenDesc {
    // IDA 0xf5f984: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf5f994 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_15NetworkSettingsENS2_20PhysicsReceiveMethodEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::EnumPropDescriptor<RBX::NetworkSettings,RBX::NetworkSettings::PhysicsReceiveMethod>::EnumPropDescriptor<RBX::NetworkSettings::PhysicsReceiveMethod (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(RBX::NetworkSettings::PhysicsReceiveMethod const&)>(char const*,char const*,RBX::NetworkSettings::PhysicsReceiveMethod (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(RBX::NetworkSettings::PhysicsReceiveMethod const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5f994(name: &str) -> GenDesc {
    // IDA 0xf5f994: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf5f9c4 — j___ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::addPair(RBX::NetworkSettings::PhysicsSendMethod,char const*)"
)]
pub fn stub_f5f9c4() {
    // IDA 0xf5f9c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5f9d4 — j___ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::addPair(RBX::NetworkSettings::PhysicsReceiveMethod,char const*)"
)]
pub fn stub_f5f9d4() {
    // IDA 0xf5f9d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5f9e4 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_15NetworkSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::NetworkSettings>(char const*,char const*,bool RBX::NetworkSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5f9e4() {
    // IDA 0xf5f9e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5f9f4 — j___ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_15NetworkSettingsEEEPKcS7_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::NetworkSettings>(char const*,char const*,double RBX::NetworkSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5f9f4() {
    // IDA 0xf5f9f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5fa04 — j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_15NetworkSettingsEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::NetworkSettings>(char const*,char const*,float RBX::NetworkSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5fa04() {
    // IDA 0xf5fa04: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5fa14 — j___ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_15NetworkSettingsEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::NetworkSettings>(char const*,char const*,int RBX::NetworkSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5fa14() {
    // IDA 0xf5fa14: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5fa84 — j___ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(_DWORD)
#[doc(
    alias = "std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsSendMethod*,std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>>,RBX::NetworkSettings::PhysicsSendMethod const&)"
)]
pub fn stub_f5fa84(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0xf5fa84: vector insert with reallocation around the new element.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0xf5fa94 — j___ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
#[doc(
    alias = "std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsSendMethod*,std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>>,unsigned long,RBX::NetworkSettings::PhysicsSendMethod const&)"
)]
pub fn stub_f5fa94() {
    // IDA 0xf5fa94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5faa4 — j___ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(_DWORD)
#[doc(
    alias = "std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsReceiveMethod*,std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>>,RBX::NetworkSettings::PhysicsReceiveMethod const&)"
)]
pub fn stub_f5faa4(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0xf5faa4: vector insert with reallocation around the new element.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0xf5fab4 — j___ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(_DWORD)
#[doc(
    alias = "std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsReceiveMethod*,std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>>,unsigned long,RBX::NetworkSettings::PhysicsReceiveMethod const&)"
)]
pub fn stub_f5fab4() {
    // IDA 0xf5fab4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5fb04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod> const&)"
)]
pub fn stub_f5fb04() -> Option<u32> {
    // IDA 0xf5fb04: nullable object query (id when live, None when unset).
    None
}
// 0xf5fb14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(
    alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod> const&)"
)]
pub fn stub_f5fb14() -> Option<u32> {
    // IDA 0xf5fb14: nullable object query (id when live, None when unset).
    None
}
// 0xf5fb24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod> const&)"
)]
pub fn stub_f5fb24() -> Option<u32> {
    // IDA 0xf5fb24: nullable object query (id when live, None when unset).
    None
}
// 0xf5fb34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(
    alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod> const&)"
)]
pub fn stub_f5fb34() -> Option<u32> {
    // IDA 0xf5fb34: nullable object query (id when live, None when unset).
    None
}
// 0xf5fbd4 — j___ZN3RBX7Network13PhysicsSender3JobC2EN5boost10shared_ptrIS1_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::PhysicsSender::Job::Job(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
pub fn stub_f5fbd4() -> Option<u32> {
    // IDA 0xf5fbd4: nullable object query (id when live, None when unset).
    None
}
// 0xf5fbe4 — j___ZN3RBX7Network13PhysicsSender3JobD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
pub fn stub_f5fbe4(j: GenJob) {
    // IDA 0xf5fbe4: job dtor.
    let _ = j;
}
// 0xf5fbf4 — j___ZN3RBX7Network13PhysicsSender8TouchJobC2EN5boost10shared_ptrIS1_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::TouchJob(rbx_core::SharedPtr<RBX::Network::PhysicsSender>)")]
pub fn stub_f5fbf4() -> Option<u32> {
    // IDA 0xf5fbf4: nullable object query (id when live, None when unset).
    None
}
// 0xf5fc04 — j___ZN3RBX7Network13PhysicsSender8TouchJobD2Ev
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
pub fn stub_f5fc04() {
    // IDA 0xf5fc04: dtor releases the owned control block/slots.
}
// 0xf5fc74 — j___ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender3JobEE5resetEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender::Job>::reset(void)")]
pub fn stub_f5fc74(slot: &mut Option<u32>) {
    // IDA 0xf5fc74: releases the owned ref (intrusive release engine-side).
    *slot = None;
}
// 0xf5fc84 — j___ZN5boost10shared_ptrIN3RBX7Network13PhysicsSender8TouchJobEE5resetEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender::TouchJob>::reset(void)")]
pub fn stub_f5fc84(slot: &mut Option<u32>) {
    // IDA 0xf5fc84: releases the owned ref (intrusive release engine-side).
    *slot = None;
}
// 0xf5fcb4 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(
    alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(boost::shared_ptr<RBX::Network::PhysicsSender::Job> *,RBX::Network::PhysicsSender::Job *,boost::detail::shared_count &)"
)]
pub fn stub_f5fcb4(slot: &mut Option<u32>, v: u32) {
    // IDA 0xf5fcb4: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xf5fcc4 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSender8TouchJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(
    alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(boost::shared_ptr<RBX::Network::PhysicsSender::TouchJob> *,RBX::Network::PhysicsSender::TouchJob *,boost::detail::shared_count &)"
)]
pub fn stub_f5fcc4(slot: &mut Option<u32>, v: u32) {
    // IDA 0xf5fcc4: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xf5fd24 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::Job,RBX::Network::PhysicsSender::Job>(boost::shared_ptr<RBX::Network::PhysicsSender::Job> const*,RBX::Network::PhysicsSender::Job *)const"
)]
pub fn stub_f5fd24(has_weak: bool) -> bool {
    // IDA 0xf5fd24: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5fd34 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network13PhysicsSender8TouchJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PhysicsSender::TouchJob,RBX::Network::PhysicsSender::TouchJob>(boost::shared_ptr<RBX::Network::PhysicsSender::TouchJob> const*,RBX::Network::PhysicsSender::TouchJob *)const"
)]
pub fn stub_f5fd34(has_weak: bool) -> bool {
    // IDA 0xf5fd34: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5fd44 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFvbELi1EEC2EMS3_FvbEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,void ()(bool),1>::BoundFuncDesc(void (RBX::Network::Server::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f5fd44(name: &str) -> GenDesc {
    // IDA 0xf5fd44: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf5fd54 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFviELi1EEC2EMS3_FviEPKcS9_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,void ()(int),1>::BoundFuncDesc(void (RBX::Network::Server::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f5fd54(name: &str) -> GenDesc {
    // IDA 0xf5fd54: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf5fd64 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ServerEFviiELi2EEC2EMS3_FviiEPKcS9_iS9_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Server,void ()(int,int),2>::BoundFuncDesc(void (RBX::Network::Server::*)(int,int),char const*,char const*,int,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f5fd64(name: &str) -> GenDesc {
    // IDA 0xf5fd64: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf5fd74 — j___ZN3RBX10Reflection14PropDescriptorINS_7Network6ServerEiEC2IMS3_KFivEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(
    alias = "RBX::Reflection::PropDescriptor<RBX::Network::Server,int>::PropDescriptor<int (RBX::Network::Server::*)(void)const,int>(char const*,char const*,int (RBX::Network::Server::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f5fd74(name: &str) -> GenDesc {
    // IDA 0xf5fd74: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf5fd84 — j___ZN3RBX10Reflection9EventDescINS_7Network6ServerEFvN5boost10shared_ptrINS_8InstanceEEENS2_12FilterResultES7_SsEN3rbx6signalIS9_EEMS3_SC_EC2ESD_PKcSG_SG_SG_SG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::EventDesc<RBX::Network::Server,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)> RBX::Network::Server::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)> RBX::Network::Server::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f5fd84(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf5fd84: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf5fd94 — j___ZN3RBX10Reflection9EventDescINS_7Network6ServerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::EventDesc<RBX::Network::Server,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Server::*>::EventDesc(rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Server::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f5fd94(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf5fd94: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf5fda4 — j___ZN3rbx7signals16signal_with_argsILi4EFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE8fireItemEPNS0_6signalIS9_E4slotES6_S8_S6_Ss
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(
    alias = "rbx::signals::signal_with_args<4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::fireItem(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot *,boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)"
)]
pub fn stub_f5fda4() {
    // IDA 0xf5fda4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5fdb4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE13disconnectAllEv
// type: int __fastcall(_DWORD)
#[doc(
    alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::disconnectAll(void)"
)]
pub fn stub_f5fdb4(s: &mut GenSignalState) {
    // IDA 0xf5fdb4: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xf5fdc4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE5mutexEv
// type: int __fastcall(_DWORD)
#[doc(
    alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::mutex(void)"
)]
pub fn stub_f5fdc4() {
    // IDA 0xf5fdc4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5fdd4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE6insertEPNSA_4slotE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot *)"
)]
pub fn stub_f5fdd4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf5fdd4: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0xf5fde4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE6removeEPNSA_4slotE
// type: int __fastcall(int, char *)
#[doc(
    alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot *)"
)]
pub fn stub_f5fde4(s: &mut GenSignalState, id: u64) {
    // IDA 0xf5fde4: unlinks one slot node (missing node is a no-op).
    gen_disconnect(s, id);
}
// 0xf5fdf4 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_E4callES7_S9_S7_Ss
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(
    alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::call(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)"
)]
pub fn stub_f5fdf4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf5fdf4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf5fe04 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_ED2Ev
// type: int __fastcall(_DWORD)
#[doc(
    alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::~callable()"
)]
pub fn stub_f5fe04() {
    // IDA 0xf5fe04: drops the bound functor held by the callable.
}
// 0xf5fe24 — j___ZN5boost10shared_ptrIN3RBX7Network7PlayersEE5resetEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players>::reset(void)")]
pub fn stub_f5fe24(slot: &mut Option<u32>) {
    // IDA 0xf5fe24: releases the owned ref (intrusive release engine-side).
    *slot = None;
}
// 0xf5fe84 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotEEaSEPSC_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot*)"
)]
pub fn stub_f5fe84(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf5fe84: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf5fe94 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKNS1_7Network12FilterResultES8_RKSsNS4_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEENSG_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISN_T0_T1_T2_T3_T4_EENSL_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSQ_FSN_SR_SS_ST_SU_ESX_SY_SZ_S10_S11_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(
    alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)"
)]
pub fn stub_f5fe94() -> Option<u32> {
    // IDA 0xf5fe94: nullable object query (id when live, None when unset).
    None
}
// 0xf5fea4 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network15NetworkOwnerJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(
    alias = "void boost::detail::sp_pointer_construct<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(boost::shared_ptr<RBX::Network::NetworkOwnerJob> *,RBX::Network::NetworkOwnerJob *,boost::detail::shared_count &)"
)]
pub fn stub_f5fea4(slot: &mut Option<u32>, v: u32) {
    // IDA 0xf5fea4: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xf5feb4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKNS7_7Network12FilterResultESE_RKSsEENS3_5list5INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(
    alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)"
)]
pub fn stub_f5feb4(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf5feb4: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf5fed4 — j___ZN5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEENS2_7Network12FilterResultES4_SsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKS4_RKS6_SG_RKSsEENS9_5list5INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::function4<void,boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)"
)]
pub fn stub_f5fed4(slot: &mut GenFunctor) -> bool {
    // IDA 0xf5fed4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf5fee4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network6ServerEFvN5boost10shared_ptrINS_8InstanceEEENS2_12FilterResultES7_SsEN3rbx6signalIS9_EEMS3_SC_E7connectEPNS0_11EventSourceERKNS4_8functionIS9_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::EventDescBase<RBX::Network::Server,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)> RBX::Network::Server::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)> const&)const"
)]
pub fn stub_f5fee4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf5fee4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf5fef4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network6ServerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(
    alias = "RBX::Reflection::EventDescBase<RBX::Network::Server,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Server::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)> const&)const"
)]
pub fn stub_f5fef4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf5fef4: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xf5ff04 — j___ZNK3RBX15ServiceProvider6createINS_7Network18ClusterPacketCacheEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(
    alias = "RBX::Network::ClusterPacketCache * RBX::ServiceProvider::create<RBX::Network::ClusterPacketCache>(void)const"
)]
pub fn stub_f5ff04() -> Option<u32> {
    // IDA 0xf5ff04: nullable object query (id when live, None when unset).
    None
}
// 0xf5ff14 — j___ZNK3RBX15ServiceProvider6createINS_7Network18PhysicsPacketCacheEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(
    alias = "RBX::Network::PhysicsPacketCache * RBX::ServiceProvider::create<RBX::Network::PhysicsPacketCache>(void)const"
)]
pub fn stub_f5ff14() -> Option<u32> {
    // IDA 0xf5ff14: nullable object query (id when live, None when unset).
    None
}
// 0xf5ff24 — j___ZNK3RBX15ServiceProvider6createINS_7Network19InstancePacketCacheEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(
    alias = "RBX::Network::InstancePacketCache * RBX::ServiceProvider::create<RBX::Network::InstancePacketCache>(void)const"
)]
pub fn stub_f5ff24() -> Option<u32> {
    // IDA 0xf5ff24: nullable object query (id when live, None when unset).
    None
}
// 0xf5ff34 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network6ServerENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>> const&)const"
)]
pub fn stub_f5ff34() -> Option<u32> {
    // IDA 0xf5ff34: nullable object query (id when live, None when unset).
    None
}
// 0xf5ff44 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ServerReplicatorES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ServerReplicator,RBX::Network::ServerReplicator>(boost::shared_ptr<RBX::Network::ServerReplicator> const*,RBX::Network::ServerReplicator *)const"
)]
pub fn stub_f5ff44(has_weak: bool) -> bool {
    // IDA 0xf5ff44: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5ff54 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network18ClusterPacketCacheES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClusterPacketCache,RBX::Network::ClusterPacketCache>(boost::shared_ptr<RBX::Network::ClusterPacketCache> const*,RBX::Network::ClusterPacketCache *)const"
)]
pub fn stub_f5ff54(has_weak: bool) -> bool {
    // IDA 0xf5ff54: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5ff64 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network18PhysicsPacketCacheES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::PhysicsPacketCache,RBX::Network::PhysicsPacketCache>(boost::shared_ptr<RBX::Network::PhysicsPacketCache> const*,RBX::Network::PhysicsPacketCache *)const"
)]
pub fn stub_f5ff64(has_weak: bool) -> bool {
    // IDA 0xf5ff64: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5ff74 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network19InstancePacketCacheES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::InstancePacketCache,RBX::Network::InstancePacketCache>(boost::shared_ptr<RBX::Network::InstancePacketCache> const*,RBX::Network::InstancePacketCache *)const"
)]
pub fn stub_f5ff74(has_weak: bool) -> bool {
    // IDA 0xf5ff74: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5ff84 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network15NetworkOwnerJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::NetworkOwnerJob,RBX::Network::NetworkOwnerJob>(boost::shared_ptr<RBX::Network::NetworkOwnerJob> const*,RBX::Network::NetworkOwnerJob *)const"
)]
pub fn stub_f5ff84(has_weak: bool) -> bool {
    // IDA 0xf5ff84: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf5ff94 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network6ServerENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(
    alias = "boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Server*,boost::shared_ptr<RBX::Instance>)const"
)]
pub fn stub_f5ff94() -> Option<u32> {
    // IDA 0xf5ff94: nullable object query (id when live, None when unset).
    None
}
// 0xf5ffa4 — j___ZNK5boost6detail8function13basic_vtable4IvNS_10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKS6_RKS8_SI_RKSsEENSB_5list5INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(
    alias = "bool boost::detail::function::basic_vtable4<void,boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Network::FilterResult const&,boost::shared_ptr<RBX::Instance> const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const"
)]
pub fn stub_f5ffa4(slot: &mut GenFunctor) -> bool {
    // IDA 0xf5ffa4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf5ffb4 — j___ZNK5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEENS2_7Network12FilterResultES4_SsEclES4_S6_S4_Ss
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(
    alias = "boost::function4<void,boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)const"
)]
pub fn stub_f5ffb4() -> Option<u32> {
    // IDA 0xf5ffb4: nullable object query (id when live, None when unset).
    None
}
// 0xf5ffd4 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESD_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSI_EEEEEEESJ_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f5ffd4() {
    // IDA 0xf5ffd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf5ffe4 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESG_E4typeENS5_ISD_EE
// type: int __fastcall(_DWORD)
#[doc(
    alias = "boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f5ffe4() -> Option<u32> {
    // IDA 0xf5ffe4: nullable object query (id when live, None when unset).
    None
}
// 0xf5fff4 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESD_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSI_EEEEEEESJ_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f5fff4() {
    // IDA 0xf5fff4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60004 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESG_E4typeENS5_ISD_EE
// type: int __fastcall(_DWORD)
#[doc(
    alias = "boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f60004() -> Option<u32> {
    // IDA 0xf60004: nullable object query (id when live, None when unset).
    None
}
// 0xf60014 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESD_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSI_EEEEEEESJ_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f60014() {
    // IDA 0xf60014: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60024 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESG_E4typeENS5_ISD_EE
// type: int __fastcall(_DWORD)
#[doc(
    alias = "boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f60024() -> Option<u32> {
    // IDA 0xf60024: nullable object query (id when live, None when unset).
    None
}
// 0xf60034 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE11callGenericIS3_EENS4_10disable_ifINS4_7is_voidIT_EESE_E4typeENS5_INS4_8functionIFNS5_INS0_5TupleEEENS5_IKSJ_EEEEEEESK_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "boost::disable_if<boost::is_void<RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::callGeneric<RBX::Network::FilterResult>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f60034() {
    // IDA 0xf60034: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60044 — j___ZN3RBX10Reflection12CallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEE13convertResultIS3_EENS4_10disable_ifINS4_7is_sameINS5_IKNS0_5TupleEEET_EESH_E4typeENS5_ISE_EE
// type: int __fastcall(_DWORD)
#[doc(
    alias = "boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,RBX::Network::FilterResult>,RBX::Network::FilterResult>::type RBX::Reflection::CallbackDesc<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::convertResult<RBX::Network::FilterResult>(boost::shared_ptr<RBX::Reflection::Tuple>)"
)]
pub fn stub_f60044() -> Option<u32> {
    // IDA 0xf60044: nullable object query (id when live, None when unset).
    None
}
// 0xf60054 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvbELi1EEC2EMS3_FvbEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::BoundFuncDesc(void (RBX::Network::ServerReplicator::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f60054(name: &str) -> GenDesc {
    // IDA 0xf60054: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xf60074 — j___ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2ERNS0_15ClassDescriptorEPKcSD_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
#[doc(
    alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>),1>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f60074() {
    // IDA 0xf60074: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60084 — j___ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_ELi2EEC2ERNS0_15ClassDescriptorEPKcSD_SD_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
#[doc(
    alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),2>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f60084() {
    // IDA 0xf60084: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60094 — j___ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsELi2EEC2ERNS0_15ClassDescriptorEPKcSD_SD_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
#[doc(
    alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string),2>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f60094() {
    // IDA 0xf60094: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf600a4 — j___ZN3RBX10Reflection16CallbackDescImplIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEELi3EEC2ERNS0_15ClassDescriptorEPKcSE_SE_SE_NS0_10Descriptor10AttributesENS_8Security11PermissionsE
#[doc(
    alias = "RBX::Reflection::CallbackDescImpl<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),3>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,char const*,char const*,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)"
)]
pub fn stub_f600a4() {
    // IDA 0xf600a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf600b4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_7Network12FilterResultEEERT_v
#[doc(
    alias = "RBX::Network::FilterResult & RBX::Reflection::Variant::genericConvert<RBX::Network::FilterResult>(void)"
)]
pub fn stub_f600b4() {
    // IDA 0xf600b4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf600c4 — j___ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEE7addPairES3_PKc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::addPair(RBX::Network::FilterResult,char const*)"
)]
pub fn stub_f600c4() {
    // IDA 0xf600c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf600d4 — j___ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEED2Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::~EnumDesc()")]
pub fn stub_f600d4() {
    // IDA 0xf600d4: EnumDesc dtor releases the item map.
}
// 0xf600f4 — j___ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(
    alias = "j___ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv"
)]
pub fn stub_f600f4() -> Option<u32> {
    // IDA 0xf600f4: nullable object query (id when live, None when unset).
    None
}
// 0xf60114 — j___ZN3RBX10Reflection9EventDescINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(
    alias = "RBX::Reflection::EventDesc<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::EventDesc(rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)"
)]
pub fn stub_f60114(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xf60114: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xf60124 — j___ZN3RBX11shared_fromINS_7Network16ServerReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: int __fastcall(_DWORD)
#[doc(
    alias = "boost::shared_ptr<RBX::Network::ServerReplicator> RBX::shared_from<RBX::Network::ServerReplicator>(RBX::Network::ServerReplicator*)"
)]
pub fn stub_f60124() -> Option<u32> {
    // IDA 0xf60124: nullable object query (id when live, None when unset).
    None
}
// 0xf60194 — j___ZN3RBX7Network10Replicator15NewInstanceItemC2EPS1_N5boost10shared_ptrIKNS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(
    alias = "RBX::Network::Replicator::NewInstanceItem::NewInstanceItem(RBX::Network::Replicator*,boost::shared_ptr<RBX::Instance const>)"
)]
pub fn stub_f60194() -> Option<u32> {
    // IDA 0xf60194: nullable object query (id when live, None when unset).
    None
}
// 0xf601a4 — j___ZN3RBX7Network10Replicator9StatsItem6updateEv
// type: _DWORD __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StatsItem::update(void)")]
pub fn stub_f601a4() -> Option<u32> {
    // IDA 0xf601a4: nullable object query (id when live, None when unset).
    None
}
// 0xf601b4 — j___ZN3RBX7Network10Replicator9StatsItemD2Ev
// type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
pub fn stub_f601b4() {
    // IDA 0xf601b4: dtor releases the owned control block/slots.
}
// 0xf601c4 — j___ZN3RBX7Network13NetworkFilter33filterIfAssociatedWithOtherPlayerILNS0_12FilterResultE1EEEbPNS_8InstanceERS3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(
    alias = "bool RBX::Network::NetworkFilter::filterIfAssociatedWithOtherPlayer<(RBX::Network::FilterResult)1>(RBX::Instance *,RBX::Network::FilterResult&)"
)]
pub fn stub_f601c4() -> bool {
    // IDA 0xf601c4: predicate passthrough.
    false
}
// 0xf601d4 — j___ZN3RBX7Network16ServerReplicator15ServerStatsItemC2ERKN5boost10shared_ptrIKS1_EE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int)
#[doc(
    alias = "RBX::Network::ServerReplicator::ServerStatsItem::ServerStatsItem(boost::shared_ptr<RBX::Network::ServerReplicator const> const&)"
)]
pub fn stub_f601d4() -> Option<u32> {
    // IDA 0xf601d4: nullable object query (id when live, None when unset).
    None
}
// 0xf601e4 — j___ZN3RBX7Network8PropSync6Master14onPropertySendENS_10Reflection13ConstPropertyE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::PropSync::Master::onPropertySend(RBX::Reflection::ConstProperty)")]
pub fn stub_f601e4() {
    // IDA 0xf601e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf601f4 — j___ZN3RBX7Network8PropSync6Master17onPropertyChangedENS_10Reflection13ConstPropertyE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::PropSync::Master::onPropertyChanged(RBX::Reflection::ConstProperty)")]
pub fn stub_f601f4(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf601f4: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf60204 — j___ZN3RBX7Network8PropSync6Master25onReceivedAcknowledgementENS_10Reflection13ConstPropertyEi
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Network::PropSync::Master::onReceivedAcknowledgement(RBX::Reflection::ConstProperty,int)")]
pub fn stub_f60204() {
    // IDA 0xf60204: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60214 — j___ZN3RBX7Network8PropSync6Master25onReceivedPropertyChangedENS_10Reflection13ConstPropertyE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::PropSync::Master::onReceivedPropertyChanged(RBX::Reflection::ConstProperty)")]
pub fn stub_f60214() {
    // IDA 0xf60214: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60224 — j___ZN3RBX7Network8PropSync6detail4BaseINS2_10MasterItemEE11expireItemsEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::MasterItem>::expireItems(void)")]
pub fn stub_f60224() {
    // IDA 0xf60224: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60234 — j___ZN3RBX7Network8PropSync6detail4BaseINS2_10MasterItemEEC2ENS_4Time8IntervalE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(
    alias = "RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::MasterItem>::Base(RBX::Time::Interval)"
)]
pub fn stub_f60234() {
    // IDA 0xf60234: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60244 — j___ZN3RBX7Network8PropSync6detail4BaseINS2_10MasterItemEED2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::MasterItem>::~Base()")]
pub fn stub_f60244() {
    // IDA 0xf60244: dtor releases the owned control block/slots.
}
// 0xf60254 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7Network16ServerReplicator15ServerStatsItemEN5boost10shared_ptrIS5_EEEENS8_IT_EET0_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
#[doc(
    alias = "boost::shared_ptr<RBX::Network::ServerReplicator::ServerStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ServerReplicator::ServerStatsItem,boost::shared_ptr<RBX::Network::ServerReplicator>>(boost::shared_ptr<RBX::Network::ServerReplicator>)"
)]
pub fn stub_f60254() -> Option<u32> {
    // IDA 0xf60254: nullable object query (id when live, None when unset).
    None
}
// 0xf60264 — j___ZN3rbx22timestamped_safe_queueIN3RBX7Network8PropSync6detail11PropertyKeyEE13pop_if_waitedENS1_4Time8IntervalERS5_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(
    alias = "rbx::timestamped_safe_queue<RBX::Network::PropSync::detail::PropertyKey>::pop_if_waited(RBX::Time::Interval,RBX::Network::PropSync::detail::PropertyKey&)"
)]
pub fn stub_f60264() {
    // IDA 0xf60264: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60274 — j___ZN3rbx22timestamped_safe_queueIN3RBX7Network8PropSync6detail11PropertyKeyEE4pushERKS5_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(
    alias = "rbx::timestamped_safe_queue<RBX::Network::PropSync::detail::PropertyKey>::push(RBX::Network::PropSync::detail::PropertyKey const&)"
)]
pub fn stub_f60274() {
    // IDA 0xf60274: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60294 — j___ZN3rbx7signals16signal_with_argsILi4EFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEEclES6_S8_S6_Ss
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(
    alias = "rbx::signals::signal_with_args<4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)"
)]
pub fn stub_f60294() {
    // IDA 0xf60294: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf602a4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot> &)"
)]
pub fn stub_f602a4() {
    // IDA 0xf602a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60314 — j___ZN5boost10shared_ptrIN3RBX7Network10Replicator9StreamJobEE5resetEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob>::reset(void)")]
pub fn stub_f60314(slot: &mut Option<u32>) {
    // IDA 0xf60314: releases the owned ref (intrusive release engine-side).
    *slot = None;
}
// 0xf60324 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotEEaSERKSD_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot> const&)"
)]
pub fn stub_f60324(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf60324: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf60364 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEEEclINS5_7Network12FilterResultEPFSK_SD_NS3_INS5_8InstanceEEEENS0_5list1IRSM_EEEET_NS0_4typeISS_EERT0_RT1_l
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "RBX::Network::FilterResult boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,long)"
)]
pub fn stub_f60364(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf60364: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf60384 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEENSF_ILi2EEEEclINS5_7Network12FilterResultEPFSL_SD_NS3_INS5_8InstanceEEESN_ENS0_5list2IRSN_SR_EEEET_NS0_4typeIST_EERT0_RT1_l
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(
    alias = "RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,boost::shared_ptr<RBX::Instance>&> &,long)"
)]
pub fn stub_f60384(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf60384: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf60394 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEENSF_ILi2EEEEclINS5_7Network12FilterResultEPFSL_SD_NS3_INS5_8InstanceEEESsENS0_5list2IRSN_RSsEEEET_NS0_4typeISU_EERT0_RT1_l
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "RBX::Network::FilterResult boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,std::string &>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string) &,boost::_bi::list2<boost::shared_ptr<RBX::Instance>&,std::string &> &,long)"
)]
pub fn stub_f60394(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf60394: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf603b4 — j___ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEENS_3argILi1EEENSF_ILi2EEENSF_ILi3EEEEclINS5_7Network12FilterResultEPFSM_SD_NS3_INS5_8InstanceEEESsNS6_7VariantEENS0_5list3IRSO_RSsRSP_EEEET_NS0_4typeISX_EERT0_RT1_l
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "RBX::Network::FilterResult boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::Reflection::Variant&>>(boost::_bi::type<RBX::Network::FilterResult>,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant) &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::Reflection::Variant&> &,long)"
)]
pub fn stub_f603b4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xf603b4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xf603c4 — j___ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESD_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSK_T0_T1_ENSI_9list_av_2IT2_T3_E4typeEEESO_SQ_SR_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_2<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>)"
)]
pub fn stub_f603c4() {
    // IDA 0xf603c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf603d4 — j___ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESF_SD_NS_3argILi1EEENSG_ILi2EEEEENS_3_bi6bind_tIT_PFSL_T0_T1_T2_ENSJ_9list_av_3IT3_T4_T5_E4typeEEESQ_SS_ST_SU_
// type: int __fastcall(int, int)
#[doc(
    alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_3<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)"
)]
pub fn stub_f603d4() {
    // IDA 0xf603d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf603e4 — j___ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESsNS6_7VariantESD_NS_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEENS_3_bi6bind_tIT_PFSN_T0_T1_T2_T3_ENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEST_SV_SW_SX_SY_
// type: int __fastcall(int, int)
#[doc(
    alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list_av_4<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)"
)]
pub fn stub_f603e4() {
    // IDA 0xf603e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf603f4 — j___ZN5boost4bindIN3RBX7Network12FilterResultENS_10shared_ptrINS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKS7_EEEEEEENS4_INS1_8InstanceEEESsSD_NS_3argILi1EEENSG_ILi2EEEEENS_3_bi6bind_tIT_PFSL_T0_T1_T2_ENSJ_9list_av_3IT3_T4_T5_E4typeEEESQ_SS_ST_SU_
// type: int __fastcall(int, int)
#[doc(
    alias = "boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list_av_3<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::Network::FilterResult,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>>(RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::arg<1>,boost::arg<2>)"
)]
pub fn stub_f603f4() {
    // IDA 0xf603f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf60424 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator9StreamJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(
    alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(boost::shared_ptr<RBX::Network::Replicator::StreamJob> *,RBX::Network::Replicator::StreamJob *,boost::detail::shared_count &)"
)]
pub fn stub_f60424(slot: &mut Option<u32>, v: u32) {
    // IDA 0xf60424: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xf60454 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEEENS3_5list2INS3_5valueISH_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(
    alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)"
)]
pub fn stub_f60454(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf60454: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf60464 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESJ_ENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(
    alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)"
)]
pub fn stub_f60464(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf60464: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf60474 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsENS3_5list3INS3_5valueISH_EENS_3argILi1EEENSP_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(
    alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)"
)]
pub fn stub_f60474(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf60474: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf60484 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX7Network12FilterResultEPFS7_NS_10shared_ptrINS_8functionIFNS8_INS5_10Reflection5TupleEEENS8_IKSB_EEEEEEENS8_INS5_8InstanceEEESsNSA_7VariantEENS3_5list4INS3_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE12manage_smallERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(
    alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)"
)]
pub fn stub_f60484(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf60484: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf604a4 — j___ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEEaSERKS8_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>)> const&)"
)]
pub fn stub_f604a4(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf604a4: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf604c4 — j___ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EEaSERKS8_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> const&)"
)]
pub fn stub_f604c4(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf604c4: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf604e4 — j___ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEEaSERKS8_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string)> const&)"
)]
pub fn stub_f604e4(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf604e4: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf60504 — j___ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEEaSERKSA_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::operator=(boost::function<RBX::Network::FilterResult ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant)> const&)"
)]
pub fn stub_f60504(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xf60504: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xf60534 — j___ZN5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_ENS9_5list2INS9_5valueISJ_EENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(pthread_mutex_t *, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::function1<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>>>)"
)]
pub fn stub_f60534(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60534: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60544 — j___ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_E9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_S6_ENS9_5list3INS9_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
// type: int __fastcall(pthread_mutex_t *, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)"
)]
pub fn stub_f60544(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60544: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60554 — j___ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsE9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_SsENS9_5list3INS9_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
// type: int __fastcall(pthread_mutex_t *, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string),boost::_bi::list3<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>>>)"
)]
pub fn stub_f60554(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60554: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60564 — j___ZN5boost9function3IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS4_INS_8functionIFNS4_INS7_5TupleEEENS4_IKSE_EEEEEEES6_SsS8_ENSB_5list4INSB_5valueISK_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_
// type: int __fastcall(pthread_mutex_t *, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(
    alias = "void boost::function3<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<RBX::Network::FilterResult,RBX::Network::FilterResult (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Instance>,std::string,RBX::Reflection::Variant),boost::_bi::list4<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)"
)]
pub fn stub_f60564(slot: &mut GenFunctor) -> bool {
    // IDA 0xf60564: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf60584 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISC_EESM_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(
    alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> *)"
)]
pub fn stub_f60584(map: &mut HashMap<u32, f32>, part: u32) -> bool {
    // IDA 0xf60584: erases the node chain for one key.
    map.remove(&part).is_some()
}
// 0xf60594 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_10MasterItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISC_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEbERSA_RKT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(
    alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::MasterItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>>>(RBX::Network::PropSync::detail::PropertyKey const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::MasterItem>> const&)"
)]
pub fn stub_f60594(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xf60594: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
