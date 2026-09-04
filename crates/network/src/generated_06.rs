//! network generated_06 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 100 stubs here, 3639 combined, 1158 remaining).
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


// 0xb213b8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")]
pub fn stub_b213b8(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb213b8: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb21550 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21550() {
    // IDA 0xb21550: counted-impl dtor frees the control block.
}
// 0xb21554 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21554() {
    // IDA 0xb21554: counted-impl dtor frees the control block.
}
// 0xb21560 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")]
pub fn stub_b21560() {
    // IDA 0xb21560: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb21574 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b21574() -> bool {
    // IDA 0xb21574: deleter query misses for this control block.
    false
}
// 0xb21578 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b21578() -> bool {
    // IDA 0xb21578: deleter query misses for this control block.
    false
}
// 0xb221c8 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_b221c8() -> Option<u32> {
    // IDA 0xb221c8: nullable object query (id when live, None when unset).
    None
}
// 0xb22618 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_b22618(slot: &mut GenFunctor) {
    // IDA 0xb22618: packs the bound argument list.
    slot.has = true;
}
// 0xb22a68 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>)")]
pub fn stub_b22a68(slot: &mut GenFunctor) {
    // IDA 0xb22a68: packs the bound argument list.
    slot.has = true;
}
// 0xb22cb8 — __ZNK3RBX15ServiceProvider4findINS_7Network18ClusterPacketCacheEEEPT_v
// type: __guard *__fastcall(_DWORD *, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClusterPacketCache * RBX::ServiceProvider::find<RBX::Network::ClusterPacketCache>(void)const")]
pub fn stub_b22cb8() -> Option<u32> {
    // IDA 0xb22cb8: nullable object query (id when live, None when unset).
    None
}
// 0xb23400 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network18ClusterPacketCacheEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::ClusterPacketCache>(void)")]
pub fn stub_b23400() -> Option<u32> {
    // IDA 0xb23400: nullable object query (id when live, None when unset).
    None
}
// 0xb234c8 — __ZNK3RBX15ServiceProvider4findINS_7Network19InstancePacketCacheEEEPT_v
// type: __guard *__fastcall(_DWORD *, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::InstancePacketCache * RBX::ServiceProvider::find<RBX::Network::InstancePacketCache>(void)const")]
pub fn stub_b234c8() -> Option<u32> {
    // IDA 0xb234c8: nullable object query (id when live, None when unset).
    None
}
// 0xb23c10 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19InstancePacketCacheEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::InstancePacketCache>(void)")]
pub fn stub_b23c10() -> Option<u32> {
    // IDA 0xb23c10: nullable object query (id when live, None when unset).
    None
}
// 0xb23cd8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")]
pub fn stub_b23cd8(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb23cd8: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb23e88 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> const*,RBX::Network::Replicator::PingJob *)const")]
pub fn stub_b23e88(has_weak: bool) -> bool {
    // IDA 0xb23e88: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xb24134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
pub fn stub_b24134() {
    // IDA 0xb24134: counted-impl dtor frees the control block.
}
// 0xb24138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
pub fn stub_b24138() {
    // IDA 0xb24138: counted-impl dtor frees the control block.
}
// 0xb24144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::dispose(void)")]
pub fn stub_b24144() -> Option<u32> {
    // IDA 0xb24144: nullable object query (id when live, None when unset).
    None
}
// 0xb24158 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_deleter(std::type_info const&)")]
pub fn stub_b24158() -> bool {
    // IDA 0xb24158: deleter query misses for this control block.
    false
}
// 0xb2415c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_untyped_deleter(void)")]
pub fn stub_b2415c() -> bool {
    // IDA 0xb2415c: deleter query misses for this control block.
    false
}
// 0xb24160 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> *,RBX::Network::Replicator::ProcessPacketsJob *,boost::detail::shared_count &)")]
pub fn stub_b24160(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb24160: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb24310 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> const*,RBX::Network::Replicator::ProcessPacketsJob *)const")]
pub fn stub_b24310(has_weak: bool) -> bool {
    // IDA 0xb24310: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xb245bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
pub fn stub_b245bc() {
    // IDA 0xb245bc: counted-impl dtor frees the control block.
}
// 0xb245c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
pub fn stub_b245c0() {
    // IDA 0xb245c0: counted-impl dtor frees the control block.
}
// 0xb245cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::dispose(void)")]
pub fn stub_b245cc() -> Option<u32> {
    // IDA 0xb245cc: nullable object query (id when live, None when unset).
    None
}
// 0xb245e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_deleter(std::type_info const&)")]
pub fn stub_b245e0() -> bool {
    // IDA 0xb245e0: deleter query misses for this control block.
    false
}
// 0xb245e4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_untyped_deleter(void)")]
pub fn stub_b245e4() -> bool {
    // IDA 0xb245e4: deleter query misses for this control block.
    false
}
// 0xb245e8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> *,RBX::Network::Replicator::SendClusterJob *,boost::detail::shared_count &)")]
pub fn stub_b245e8(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb245e8: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb24798 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> const*,RBX::Network::Replicator::SendClusterJob *)const")]
pub fn stub_b24798(has_weak: bool) -> bool {
    // IDA 0xb24798: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xb24a44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
pub fn stub_b24a44() {
    // IDA 0xb24a44: counted-impl dtor frees the control block.
}
// 0xb24a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
pub fn stub_b24a48() {
    // IDA 0xb24a48: counted-impl dtor frees the control block.
}
// 0xb24a54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::dispose(void)")]
pub fn stub_b24a54() -> Option<u32> {
    // IDA 0xb24a54: nullable object query (id when live, None when unset).
    None
}
// 0xb24a68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_deleter(std::type_info const&)")]
pub fn stub_b24a68() -> bool {
    // IDA 0xb24a68: deleter query misses for this control block.
    false
}
// 0xb24a6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_untyped_deleter(void)")]
pub fn stub_b24a6c() -> bool {
    // IDA 0xb24a6c: deleter query misses for this control block.
    false
}
// 0xb24a70 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> *,RBX::Network::Replicator::SendDataJob *,boost::detail::shared_count &)")]
pub fn stub_b24a70(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb24a70: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb24c20 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> const*,RBX::Network::Replicator::SendDataJob *)const")]
pub fn stub_b24c20(has_weak: bool) -> bool {
    // IDA 0xb24c20: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xb24ecc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
pub fn stub_b24ecc() {
    // IDA 0xb24ecc: counted-impl dtor frees the control block.
}
// 0xb24ed0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
pub fn stub_b24ed0() {
    // IDA 0xb24ed0: counted-impl dtor frees the control block.
}
// 0xb24edc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::dispose(void)")]
pub fn stub_b24edc() -> Option<u32> {
    // IDA 0xb24edc: nullable object query (id when live, None when unset).
    None
}
// 0xb24ef0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_deleter(std::type_info const&)")]
pub fn stub_b24ef0() -> bool {
    // IDA 0xb24ef0: deleter query misses for this control block.
    false
}
// 0xb24ef4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_untyped_deleter(void)")]
pub fn stub_b24ef4() -> bool {
    // IDA 0xb24ef4: deleter query misses for this control block.
    false
}
// 0xb25570 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_b25570() -> Option<u32> {
    // IDA 0xb25570: nullable object query (id when live, None when unset).
    None
}
// 0xb28f08 — __ZNK3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(void)const")]
pub fn stub_b28f08() -> Option<u32> {
    // IDA 0xb28f08: nullable object query (id when live, None when unset).
    None
}
// 0xb29650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_b29650() {
    // IDA 0xb29650: counted-impl dtor frees the control block.
}
// 0xb29658 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_b29658() -> Option<u32> {
    // IDA 0xb29658: nullable object query (id when live, None when unset).
    None
}
// 0xb2af98 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10Replicator12JoinDataItemENS_10shared_ptrIKNS7_8InstanceEEEEENS3_5list2INS3_5valueIPSA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::JoinDataItem,rbx_core::SharedPtr<RBX::Instance const>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::JoinDataItem*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_b2af98(slot: &mut GenFunctor, op: u32) {
    // IDA 0xb2af98: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xb2aff8 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10Replicator12JoinDataItemENS_10shared_ptrIKNS7_8InstanceEEEEENS3_5list2INS3_5valueIPSA_EENS_3argILi1EEEEEEEvNSB_ISC_EEE6invokeERNS1_15function_bufferESO_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::JoinDataItem,rbx_core::SharedPtr<RBX::Instance const>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::JoinDataItem*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b2aff8(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xb2aff8: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xb2b254 — __ZNK5boost4_mfi3mf1IvN3RBX7Network10Replicator12JoinDataItemENS_10shared_ptrIKNS2_8InstanceEEEEclEPS5_S9_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Replicator::JoinDataItem,rbx_core::SharedPtr<RBX::Instance const>>::operator()(RBX::Network::Replicator::JoinDataItem*,rbx_core::SharedPtr<RBX::Instance const>)const")]
pub fn stub_b2b254() -> Option<u32> {
    // IDA 0xb2b254: nullable object query (id when live, None when unset).
    None
}
// 0xb2b4cc — __ZNK5boost4_mfi3mf1IbN3RBX7Network10ReplicatorENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: int __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b2b4cc() -> Option<u32> {
    // IDA 0xb2b4cc: nullable object query (id when live, None when unset).
    None
}
// 0xb2b748 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network10ReplicatorEEENS_3argILi1EEENS2_INS_8functionIFvNS_10shared_ptrINS3_8InstanceEEEEEEEEEclINS_4_mfi3mf2IvS5_SD_SF_EENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, int, int **, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator *>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::operator()<boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_b2b748(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb2b748: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb2ba44 — __ZNK5boost4_mfi3mf2IvN3RBX7Network10ReplicatorENS_10shared_ptrINS2_8InstanceEEENS_8functionIFvS7_EEEEclEPS4_S7_SA_
// type: void __fastcall(char **, int, int *, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::operator()(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)const")]
pub fn stub_b2ba44() -> Option<u32> {
    // IDA 0xb2ba44: nullable object query (id when live, None when unset).
    None
}
// 0xb2bd50 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network10ReplicatorEEENS_3argILi1EEENS2_INS_8functionIFvNS_10shared_ptrINS3_8InstanceEEEEEEEEEC2ES7_S9_SG_
// type: int __fastcall(int, int, int *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator *>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>::list3(boost::_bi::value<RBX::Network::Replicator *>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>)")]
pub fn stub_b2bd50(slot: &mut GenFunctor) {
    // IDA 0xb2bd50: packs the bound argument list.
    slot.has = true;
}
// 0xb2bfa0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::DirectPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::DirectPhysicsReceiver *,boost::detail::shared_count &)")]
pub fn stub_b2bfa0(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb2bfa0: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb2c138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
pub fn stub_b2c138() {
    // IDA 0xb2c138: counted-impl dtor frees the control block.
}
// 0xb2c13c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
pub fn stub_b2c13c() {
    // IDA 0xb2c13c: counted-impl dtor frees the control block.
}
// 0xb2c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::dispose(void)")]
pub fn stub_b2c148() {
    // IDA 0xb2c148: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb2c15c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_deleter(std::type_info const&)")]
pub fn stub_b2c15c() -> bool {
    // IDA 0xb2c15c: deleter query misses for this control block.
    false
}
// 0xb2c160 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_untyped_deleter(void)")]
pub fn stub_b2c160() -> bool {
    // IDA 0xb2c160: deleter query misses for this control block.
    false
}
// 0xb2c164 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::InterpolatingPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::InterpolatingPhysicsReceiver *,boost::detail::shared_count &)")]
pub fn stub_b2c164(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb2c164: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb2c2fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
pub fn stub_b2c2fc() {
    // IDA 0xb2c2fc: counted-impl dtor frees the control block.
}
// 0xb2c300 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
pub fn stub_b2c300() {
    // IDA 0xb2c300: counted-impl dtor frees the control block.
}
// 0xb2c30c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::dispose(void)")]
pub fn stub_b2c30c() {
    // IDA 0xb2c30c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb2c320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_deleter(std::type_info const&)")]
pub fn stub_b2c320() -> bool {
    // IDA 0xb2c320: deleter query misses for this control block.
    false
}
// 0xb2c324 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_untyped_deleter(void)")]
pub fn stub_b2c324() -> bool {
    // IDA 0xb2c324: deleter query misses for this control block.
    false
}
// 0xb2c400 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_b2c400(slot: &mut GenFunctor, op: u32) {
    // IDA 0xb2c400: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xb2c460 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,RBX::Network::ConcurrentRakPeerStats const&>::invoke(boost::detail::function::function_buffer &,RBX::Network::ConcurrentRakPeerStats const&)")]
pub fn stub_b2c460(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xb2c460: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xb2c47c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::deque(std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>> const&)")]
pub fn stub_b2c47c() -> Option<u32> {
    // IDA 0xb2c47c: nullable object query (id when live, None when unset).
    None
}
// 0xb2c5c4 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type
// type: void __fastcall(_DWORD *, _DWORD *, int, _DWORD *, int, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>,std::__false_type)")]
pub fn stub_b2c5c4() -> Option<u32> {
    // IDA 0xb2c5c4: nullable object query (id when live, None when unset).
    None
}
// 0xb2c7a4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::_M_initialize_map(unsigned long)")]
pub fn stub_b2c7a4() -> Option<u32> {
    // IDA 0xb2c7a4: nullable object query (id when live, None when unset).
    None
}
// 0xb2c960 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::~deque()")]
pub fn stub_b2c960() {
    // IDA 0xb2c960: dtor releases the owned control block/slots.
}
// 0xb2cb04 — __ZN5boost9unordered13unordered_mapIN3RBX10Reflection13ConstPropertyENS_9intrusive13list_iteratorINS5_9list_implINS5_7listoptINS5_6detail16base_hook_traitsINS2_7Network4ItemENS5_16list_node_traitsIPvEELNS5_14link_mode_typeE1ENSB_7ItemTagELi1EEEmLb1EEEEELb0EEENS_4hashIS4_EESt8equal_toIS4_ENS_19fast_pool_allocatorIS4_NS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEEC2EmRKSN_RKSP_RKST_
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::unordered_map<RBX::Reflection::ConstProperty,boost::intrusive::list_iterator<boost::intrusive::list_impl<boost::intrusive::listopt<boost::intrusive::detail::base_hook_traits<RBX::Network::Item,boost::intrusive::list_node_traits<void *>,(boost::intrusive::link_mode_type)1,RBX::Network::ItemTag,1>,unsigned long,true>>,false>,boost::hash<RBX::Reflection::ConstProperty>,std::equal_to<RBX::Reflection::ConstProperty>,boost::fast_pool_allocator<RBX::Reflection::ConstProperty,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::unordered_map(unsigned long,boost::hash<RBX::Reflection::ConstProperty> const&,std::equal_to<RBX::Reflection::ConstProperty> const&,boost::fast_pool_allocator<RBX::Reflection::ConstProperty,boost::default_user_allocator_new_delete,boost::mutex,32u,0u> const&)")]
pub fn stub_b2cb04() -> Option<u32> {
    // IDA 0xb2cb04: nullable object query (id when live, None when unset).
    None
}
// 0xb2cc54 — __ZN3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Network::IdSerializer *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b2cc54() -> Option<u32> {
    // IDA 0xb2cc54: nullable object query (id when live, None when unset).
    None
}
// 0xb2cc60 — __ZN3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Network::IdSerializer *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b2cc60() -> Option<u32> {
    // IDA 0xb2cc60: nullable object query (id when live, None when unset).
    None
}
// 0xb2cd00 — __ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
pub fn stub_b2cd00() -> Option<u32> {
    // IDA 0xb2cd00: nullable object query (id when live, None when unset).
    None
}
// 0xb2cdfc — __ZThn32_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b2cdfc() -> Option<u32> {
    // IDA 0xb2cdfc: nullable object query (id when live, None when unset).
    None
}
// 0xb2ce08 — __ZThn32_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b2ce08() -> Option<u32> {
    // IDA 0xb2ce08: nullable object query (id when live, None when unset).
    None
}
// 0xb2ceac — __ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
pub fn stub_b2ceac() -> Option<u32> {
    // IDA 0xb2ceac: nullable object query (id when live, None when unset).
    None
}
// 0xb2cfa8 — __ZThn36_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b2cfa8() -> Option<u32> {
    // IDA 0xb2cfa8: nullable object query (id when live, None when unset).
    None
}
// 0xb2cfb4 — __ZThn36_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_7Network10ReplicatorENS1_12IdSerializerELZNS1_11sReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b2cfb4() -> Option<u32> {
    // IDA 0xb2cfb4: nullable object query (id when live, None when unset).
    None
}
// 0xb2d058 — __ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")]
pub fn stub_b2d058() -> Option<u32> {
    // IDA 0xb2d058: nullable object query (id when live, None when unset).
    None
}
// 0xb2d128 — __ZN3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Network::IdSerializer *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b2d128() -> Option<u32> {
    // IDA 0xb2d128: nullable object query (id when live, None when unset).
    None
}
// 0xb2d134 — __ZN3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Network::IdSerializer *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b2d134() -> Option<u32> {
    // IDA 0xb2d134: nullable object query (id when live, None when unset).
    None
}
// 0xb2d1d4 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b2d1d4() -> Option<u32> {
    // IDA 0xb2d1d4: nullable object query (id when live, None when unset).
    None
}
// 0xb2d1e0 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b2d1e0() -> Option<u32> {
    // IDA 0xb2d1e0: nullable object query (id when live, None when unset).
    None
}
// 0xb2d284 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b2d284() -> Option<u32> {
    // IDA 0xb2d284: nullable object query (id when live, None when unset).
    None
}
// 0xb2d290 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network10ReplicatorELZNS2_11sReplicatorEENS_17NonFactoryProductINS2_12IdSerializerELZNS2_11sReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b2d290() -> Option<u32> {
    // IDA 0xb2d290: nullable object query (id when live, None when unset).
    None
}
// 0xb2d334 — __ZN3RBX7Network12IdSerializerD2Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d334() {
    // IDA 0xb2d334: dtor releases the owned control block/slots.
}
// 0xb2d584 — __ZN3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d584() {
    // IDA 0xb2d584: dtor releases the owned control block/slots.
}
// 0xb2d590 — __ZN3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d590() {
    // IDA 0xb2d590: dtor releases the owned control block/slots.
}
// 0xb2d630 — __ZThn32_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d630(fire: &dyn Fn()) {
    // IDA 0xb2d630: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb2d63c — __ZThn32_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d63c(fire: &dyn Fn()) {
    // IDA 0xb2d63c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb2d6e0 — __ZThn36_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d6e0(fire: &dyn Fn()) {
    // IDA 0xb2d6e0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb2d6ec — __ZThn36_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
pub fn stub_b2d6ec(fire: &dyn Fn()) {
    // IDA 0xb2d6ec: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb2d790 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>> *)")]
pub fn stub_b2d790(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0xb2d790: Rb_tree erase of one node.
    map.remove(&key).is_some()
}
// 0xb2d880 — __ZN3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsEC2IMS3_KFSsvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, char, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::PropDescriptor<std::string (RBX::Network::Replicator::*)(void)const,int>(char const*,char const*,std::string (RBX::Network::Replicator::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_b2d880(name: &str) -> GenDesc {
    // IDA 0xb2d880: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xb2da94 — __ZN3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::~PropDescriptor()")]
pub fn stub_b2da94(d: GenDesc) {
    // IDA 0xb2da94: prop descriptor dtor.
    let _ = d;
}
// 0xb2dc78 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsE7GetImplIMS3_KFSsvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::GetImpl<std::string (RBX::Network::Replicator::*)(void)const>::isReadOnly(void)const")]
pub fn stub_b2dc78(d: &GenDesc) -> bool {
    // IDA 0xb2dc78: read-only when no setter was installed.
    !d.writable
}
// 0xb2dc7c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsE7GetImplIMS3_KFSsvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::GetImpl<std::string (RBX::Network::Replicator::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_b2dc7c(d: &GenDesc) -> bool {
    // IDA 0xb2dc7c: write-only when no getter was installed.
    !d.readable
}
// 0xb2dc80 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsE7GetImplIMS3_KFSsvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::GetImpl<std::string (RBX::Network::Replicator::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_b2dc80(d: &GenDesc) -> String {
    // IDA 0xb2dc80: virtual getter dispatch; returns the text.
    d.text.clone()
}
// 0xb2dca8 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsE7GetImplIMS3_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::GetImpl<std::string (RBX::Network::Replicator::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_b2dca8(d: &mut GenDesc, v: &str) {
    // IDA 0xb2dca8: virtual setter dispatch; stores the text.
    d.text = v.to_owned();
}
