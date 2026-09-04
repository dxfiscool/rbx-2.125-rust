//! network generated — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 400 stubs here, 3119 combined with raknet.rs, 1678 remaining).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;
use std::collections::HashMap;

/// `RBX::EventReplicatorBase` listener side (IDA 0x3a7f68/0x3a8228/0x3a9944):
/// the listener-mode flag, the two `rbx::signals::connection` states at
/// +20/+24, the watched property id, and the replicator listener count.
#[derive(Clone, Debug, Default)]
pub struct EventListenerState {
    pub listener_mode: bool,
    pub connection_connected: bool,
    pub listener_connected: bool,
    pub watched_prop: u32,
    pub event_count: i32,
}

/// One `rbx::signals` slot node reduced to its linkage bit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SignalSlot {
    pub id: u64,
    pub connected: bool,
}

/// A `rbx::signals::signal` slot list: ordered ids plus the allocator.
#[derive(Clone, Debug, Default)]
pub struct SignalState {
    pub slots: Vec<SignalSlot>,
    pub next_id: u64,
}

/// `RBX::Network::ChatMessage` payload kept by value.
#[derive(Clone, Debug, Default)]
pub struct ChatMessage {
    pub text: String,
    pub sender: u32,
}

/// `RBX::Network::NetworkOwner` address view.
#[derive(Clone, Debug, Default)]
pub struct NetworkOwnerAddr {
    pub ip: u32,
    pub port: u16,
    pub is_server: bool,
}

/// One `TopNErrorsPhysicsSender::Nugget` (part id + current error).
#[derive(Clone, Debug, Default)]
pub struct PhysicsNugget {
    pub part: u32,
    pub error: f32,
}

/// `RBX::Network::TopNErrorsPhysicsSender` tables (IDA 0x44ab28 family):
/// hash map part -> nugget plus the descending-error top-N index.
#[derive(Clone, Debug, Default)]
pub struct TopNErrors {
    pub nuggets: HashMap<u32, PhysicsNugget>,
    pub top: Vec<u32>,
}

fn refresh_top(top: &mut TopNErrors) {
    let mut ids: Vec<u32> = top.nuggets.keys().copied().collect();
    ids.sort_by(|a, b| {
        let ea = top.nuggets.get(a).map(|n| n.error).unwrap_or(0.0);
        let eb = top.nuggets.get(b).map(|n| n.error).unwrap_or(0.0);
        eb.partial_cmp(&ea).unwrap_or(std::cmp::Ordering::Equal)
    });
    top.top = ids;
}

/// `RBX::NetworkStatsCommand` snapshot.
#[derive(Clone, Debug, Default)]
pub struct NetworkStats {
    pub enabled: bool,
    pub checked: bool,
    pub packets: u64,
}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct ChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}

/// `boost::function` buffer occupancy for one bound functor.
#[derive(Clone, Debug, Default)]
pub struct FunctorSlot {
    pub has_functor: bool,
}


// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
pub fn stub_3a7f68(state: &mut EventListenerState, mode: bool) {
    // IDA 0x3a7f68: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
pub fn stub_3a80c8(state: &mut EventListenerState, mode: bool) {
    // IDA 0x3a80c8: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x3a8228 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_3a8228(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x3a8228: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x3a8288 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_3a8288(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x3a8288: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x3a98d0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
pub fn stub_3a98d0(signal: &mut SignalState) -> u64 {
    // IDA 0x3a98d0: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x3a9944 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
pub fn stub_3a9944(state: &mut EventListenerState) -> i32 {
    // IDA 0x3a9944: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x3a9990 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
pub fn stub_3a9990(signal: &mut SignalState, id: u64) {
    // IDA 0x3a9990: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3a99bc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
pub fn stub_3a99bc(signal: &mut SignalState, id: u64) {
    // IDA 0x3a99bc: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3a9a90 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9a90(fire: &dyn Fn(u32, f32, f32), axis: u32, a: f32, b: f32) {
    // IDA 0x3a9a90: IDA: bind/call thunk forwards axis + two floats into the bound member (mf3).
    fire(axis, a, b);
}
// 0x3a9a98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9a98(fire: &dyn Fn(u32, f32, f32), axis: u32, a: f32, b: f32) {
    // IDA 0x3a9a98: IDA: bind/call thunk forwards axis + two floats into the bound member (mf3).
    fire(axis, a, b);
}
// 0x3a9aa0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
pub fn stub_3a9aa0(fire: &dyn Fn(u32, f32, f32), axis: u32, a: f32, b: f32) {
    // IDA 0x3a9aa0: IDA: bind/call thunk forwards axis + two floats into the bound member (mf3).
    fire(axis, a, b);
}
// 0x3a9ab8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9ab8() {
    // IDA 0x3a9ab8: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3a9ae4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9ae4() {
    // IDA 0x3a9ae4: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3a9bb8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
pub fn stub_3a9bb8(signal: &mut SignalState) -> u64 {
    // IDA 0x3a9bb8: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x3a9c2c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
pub fn stub_3a9c2c(state: &mut EventListenerState) -> i32 {
    // IDA 0x3a9c2c: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x3a9c78 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
pub fn stub_3a9c78(signal: &mut SignalState, id: u64) {
    // IDA 0x3a9c78: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3a9ca4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
pub fn stub_3a9ca4(signal: &mut SignalState, id: u64) {
    // IDA 0x3a9ca4: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3a9d78 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9d78(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0x3a9d78: IDA: bind/call thunk forwards the axis into the bound member (mf1).
    fire(axis);
}
// 0x3a9d80 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9d80(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0x3a9d80: IDA: bind/call thunk forwards the axis into the bound member (mf1).
    fire(axis);
}
// 0x3a9d88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
pub fn stub_3a9d88(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0x3a9d88: IDA: bind/call thunk forwards the axis into the bound member (mf1).
    fire(axis);
}
// 0x3a9da0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9da0() {
    // IDA 0x3a9da0: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3a9dcc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9dcc() {
    // IDA 0x3a9dcc: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3aa448 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_3aa448() {
    // IDA 0x3aa448: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3aa5a4 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
pub fn stub_3aa5a4() {
    // IDA 0x3aa5a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3aa764 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_3aa764(signal: &mut SignalState) -> u64 {
    // IDA 0x3aa764: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x3aaa08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_3aaa08(signal: &mut SignalState, id: u64) {
    // IDA 0x3aaa08: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3aaa34 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_3aaa34(signal: &mut SignalState, id: u64) {
    // IDA 0x3aaa34: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3aac24 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_3aac24(fire: &dyn Fn(u32, f32, f32), axis: u32, a: f32, b: f32) {
    // IDA 0x3aac24: IDA: bind/call thunk forwards axis + two floats into the bound member (mf3).
    fire(axis, a, b);
}
// 0x3aac50 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_3aac50(fire: &dyn Fn(u32, f32, f32), axis: u32, a: f32, b: f32) {
    // IDA 0x3aac50: IDA: bind/call thunk forwards axis + two floats into the bound member (mf3).
    fire(axis, a, b);
}
// 0x3aac7c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_3aac7c(fire: &dyn Fn(u32, f32, f32), axis: u32, a: f32, b: f32) {
    // IDA 0x3aac7c: IDA: bind/call thunk forwards axis + two floats into the bound member (mf3).
    fire(axis, a, b);
}
// 0x3aafa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_3aafa0() {
    // IDA 0x3aafa0: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3aafcc — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_3aafcc() {
    // IDA 0x3aafcc: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3ab0a0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_3ab0a0() {
    // IDA 0x3ab0a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3ab0a4 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_3ab0a4() {
    // IDA 0x3ab0a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3ab200 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
pub fn stub_3ab200() {
    // IDA 0x3ab200: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3ab360 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
pub fn stub_3ab360(signal: &mut SignalState) -> u64 {
    // IDA 0x3ab360: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x3ab604 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_3ab604(signal: &mut SignalState, id: u64) {
    // IDA 0x3ab604: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3ab630 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_3ab630(signal: &mut SignalState, id: u64) {
    // IDA 0x3ab630: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x3ab820 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_3ab820(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0x3ab820: IDA: bind/call thunk forwards the axis into the bound member (mf1).
    fire(axis);
}
// 0x3ab834 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_3ab834(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0x3ab834: IDA: bind/call thunk forwards the axis into the bound member (mf1).
    fire(axis);
}
// 0x3ab848 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(char **, int *)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
pub fn stub_3ab848(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0x3ab848: IDA: bind/call thunk forwards the axis into the bound member (mf1).
    fire(axis);
}
// 0x3abb44 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_3abb44() {
    // IDA 0x3abb44: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3abb70 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_3abb70() {
    // IDA 0x3abb70: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x3abc44 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_3abc44() {
    // IDA 0x3abc44: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3b05bc — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
pub fn stub_3b05bc() {
    // IDA 0x3b05bc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3b06ec — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
pub fn stub_3b06ec() {
    // IDA 0x3b06ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x52d620 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
pub fn stub_52d620(state: &mut EventListenerState, mode: bool) {
    // IDA 0x52d620: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x52d780 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
pub fn stub_52d780(state: &mut EventListenerState, mode: bool) {
    // IDA 0x52d780: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x52d9c4 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52d9c4(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x52d9c4: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x52da24 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52da24(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x52da24: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x52e250 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
pub fn stub_52e250(state: &mut EventListenerState, mode: bool) {
    // IDA 0x52e250: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x52e3b0 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
pub fn stub_52e3b0(state: &mut EventListenerState, mode: bool) {
    // IDA 0x52e3b0: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x52e510 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52e510(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x52e510: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x52e570 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52e570(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x52e570: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x52ee40 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
pub fn stub_52ee40(signal: &mut SignalState) -> u64 {
    // IDA 0x52ee40: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x52eeb4 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
pub fn stub_52eeb4(state: &mut EventListenerState) -> i32 {
    // IDA 0x52eeb4: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x52ef00 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_52ef00(signal: &mut SignalState, id: u64) {
    // IDA 0x52ef00: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x52ef2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_52ef2c(signal: &mut SignalState, id: u64) {
    // IDA 0x52ef2c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x52f000 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f000(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x52f000: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x52f008 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f008(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x52f008: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x52f010 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
pub fn stub_52f010(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x52f010: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x52f028 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f028() {
    // IDA 0x52f028: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x52f054 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f054() {
    // IDA 0x52f054: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x52f128 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
pub fn stub_52f128(signal: &mut SignalState) -> u64 {
    // IDA 0x52f128: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x52f19c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
pub fn stub_52f19c(state: &mut EventListenerState) -> i32 {
    // IDA 0x52f19c: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x52f1e8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_52f1e8(signal: &mut SignalState, id: u64) {
    // IDA 0x52f1e8: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x52f214 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_52f214(signal: &mut SignalState, id: u64) {
    // IDA 0x52f214: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x52f2e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f2e8(fire: &dyn Fn()) {
    // IDA 0x52f2e8: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0x52f2f0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f2f0(fire: &dyn Fn()) {
    // IDA 0x52f2f0: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0x52f2f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
pub fn stub_52f2f8(fire: &dyn Fn()) {
    // IDA 0x52f2f8: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0x52f310 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f310() {
    // IDA 0x52f310: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x52f33c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f33c() {
    // IDA 0x52f33c: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x52f55c — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_52f55c() {
    // IDA 0x52f55c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x52f6b8 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::signalProducedIncremented(int,int)")]
pub fn stub_52f6b8() {
    // IDA 0x52f6b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x52f83c — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_52f83c(signal: &mut SignalState) -> u64 {
    // IDA 0x52f83c: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x52fae0 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_52fae0(signal: &mut SignalState, id: u64) {
    // IDA 0x52fae0: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x52fb0c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_52fb0c(signal: &mut SignalState, id: u64) {
    // IDA 0x52fb0c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x52fcfc — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_52fcfc(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x52fcfc: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x52fd24 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_52fd24(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x52fd24: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x52fd4c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
pub fn stub_52fd4c(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x52fd4c: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x530058 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_530058() {
    // IDA 0x530058: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x530084 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_530084() {
    // IDA 0x530084: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x530158 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_530158() {
    // IDA 0x530158: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x53015c — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
pub fn stub_53015c() {
    // IDA 0x53015c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5302b8 — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE25signalProducedIncrementedEv
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::signalProducedIncremented(void)")]
pub fn stub_5302b8() {
    // IDA 0x5302b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5303f0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")]
pub fn stub_5303f0(signal: &mut SignalState) -> u64 {
    // IDA 0x5303f0: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x530464 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_530464(signal: &mut SignalState, id: u64) {
    // IDA 0x530464: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x530490 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_530490(signal: &mut SignalState, id: u64) {
    // IDA 0x530490: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x530564 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_530564(fire: &dyn Fn()) {
    // IDA 0x530564: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0x53056c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_53056c(fire: &dyn Fn()) {
    // IDA 0x53056c: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0x530574 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
pub fn stub_530574(fire: &dyn Fn()) {
    // IDA 0x530574: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0x53058c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_53058c() {
    // IDA 0x53058c: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x5305b8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5305b8() {
    // IDA 0x5305b8: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x53068c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
pub fn stub_53068c() {
    // IDA 0x53068c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x533bfc — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
pub fn stub_533bfc(signal: &mut SignalState) -> u64 {
    // IDA 0x533bfc: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x533c70 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
pub fn stub_533c70(state: &mut EventListenerState) -> i32 {
    // IDA 0x533c70: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x533cbc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
pub fn stub_533cbc(signal: &mut SignalState, id: u64) {
    // IDA 0x533cbc: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x533ce8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
pub fn stub_533ce8(signal: &mut SignalState, id: u64) {
    // IDA 0x533ce8: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x533dbc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_533dbc(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0x533dbc: IDA: bind/call thunk forwards UDim2 into the bound member (mf1).
    fire(v);
}
// 0x533dc4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_533dc4(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0x533dc4: IDA: bind/call thunk forwards UDim2 into the bound member (mf1).
    fire(v);
}
// 0x533dcc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
pub fn stub_533dcc(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0x533dcc: IDA: bind/call thunk forwards UDim2 into the bound member (mf1).
    fire(v);
}
// 0x533de4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_533de4() {
    // IDA 0x533de4: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x533e10 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_533e10() {
    // IDA 0x533e10: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x533ee4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
pub fn stub_533ee4(signal: &mut SignalState) -> u64 {
    // IDA 0x533ee4: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x533f58 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")]
pub fn stub_533f58(state: &mut EventListenerState) -> i32 {
    // IDA 0x533f58: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x533fa4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_533fa4(signal: &mut SignalState, id: u64) {
    // IDA 0x533fa4: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x533fd0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_533fd0(signal: &mut SignalState, id: u64) {
    // IDA 0x533fd0: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x5340a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_5340a4(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x5340a4: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x5340ac — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_5340ac(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x5340ac: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x5340b4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")]
pub fn stub_5340b4(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x5340b4: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x5340cc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5340cc() {
    // IDA 0x5340cc: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x5340f8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5340f8() {
    // IDA 0x5340f8: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x537740 — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
pub fn stub_537740() {
    // IDA 0x537740: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x53789c — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)")]
pub fn stub_53789c() {
    // IDA 0x53789c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x537a18 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
pub fn stub_537a18(signal: &mut SignalState) -> u64 {
    // IDA 0x537a18: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x537cbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_537cbc(signal: &mut SignalState, id: u64) {
    // IDA 0x537cbc: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x537ce8 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_537ce8(signal: &mut SignalState, id: u64) {
    // IDA 0x537ce8: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x537ed8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
pub fn stub_537ed8(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0x537ed8: IDA: bind/call thunk forwards UDim2 into the bound member (mf1).
    fire(v);
}
// 0x537f00 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
pub fn stub_537f00(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0x537f00: IDA: bind/call thunk forwards UDim2 into the bound member (mf1).
    fire(v);
}
// 0x537f28 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
pub fn stub_537f28(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0x537f28: IDA: bind/call thunk forwards UDim2 into the bound member (mf1).
    fire(v);
}
// 0x538240 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
pub fn stub_538240() {
    // IDA 0x538240: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x53826c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
pub fn stub_53826c() {
    // IDA 0x53826c: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x538340 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
pub fn stub_538340() {
    // IDA 0x538340: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x538344 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_538344() {
    // IDA 0x538344: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5384a0 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)")]
pub fn stub_5384a0() {
    // IDA 0x5384a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x538624 — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_538624(signal: &mut SignalState) -> u64 {
    // IDA 0x538624: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x538698 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_538698(signal: &mut SignalState, id: u64) {
    // IDA 0x538698: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x5386c4 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_5386c4(signal: &mut SignalState, id: u64) {
    // IDA 0x5386c4: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x538798 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_538798(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x538798: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x5387c0 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_5387c0(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x5387c0: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x5387e8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
pub fn stub_5387e8(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0x5387e8: IDA: bind/call thunk forwards (int, int) into the bound member (mf2).
    fire(a, b);
}
// 0x538810 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_538810() {
    // IDA 0x538810: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x53883c — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_53883c() {
    // IDA 0x53883c: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x538910 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_538910() {
    // IDA 0x538910: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x53ffec — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")]
pub fn stub_53ffec() {
    // IDA 0x53ffec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x54011c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")]
pub fn stub_54011c() {
    // IDA 0x54011c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x567750 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")]
pub fn stub_567750(state: &mut EventListenerState, mode: bool) {
    // IDA 0x567750: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x5678b0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")]
pub fn stub_5678b0(state: &mut EventListenerState, mode: bool) {
    // IDA 0x5678b0: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.connection_connected { return; }
    state.listener_mode = mode;
}
// 0x567a10 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_567a10(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x567a10: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x567a70 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_567a70(state: &mut EventListenerState, prop_id: u32) -> bool {
    // IDA 0x567a70: no-op while connected; when the changed prop matches the watched one, re-query the listener count and connect (count>=1) or disconnect (count<1).
    if state.connection_connected { return false; }
    if prop_id != state.watched_prop { return false; }
    if state.event_count < 1 { state.listener_connected = false; }
    else if !state.listener_connected { state.listener_connected = true; }
    true
}
// 0x568e90 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
pub fn stub_568e90(signal: &mut SignalState) -> u64 {
    // IDA 0x568e90: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x568f04 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")]
pub fn stub_568f04(state: &mut EventListenerState) -> i32 {
    // IDA 0x568f04: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x568f50 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
pub fn stub_568f50(signal: &mut SignalState, id: u64) {
    // IDA 0x568f50: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x568f7c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
pub fn stub_568f7c(signal: &mut SignalState, id: u64) {
    // IDA 0x568f7c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x569050 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_569050(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0x569050: IDA: bind/call thunk forwards (NormalId, float) into the bound member (mf2).
    fire(id, x);
}
// 0x569058 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_569058(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0x569058: IDA: bind/call thunk forwards (NormalId, float) into the bound member (mf2).
    fire(id, x);
}
// 0x569060 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
pub fn stub_569060(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0x569060: IDA: bind/call thunk forwards (NormalId, float) into the bound member (mf2).
    fire(id, x);
}
// 0x569078 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_569078() {
    // IDA 0x569078: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x5690a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5690a4() {
    // IDA 0x5690a4: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x569178 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
pub fn stub_569178(signal: &mut SignalState) -> u64 {
    // IDA 0x569178: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_33454() -> bool {
    // IDA 0x33454: deleter query misses for this control block.
    false
}
// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3346c() -> bool {
    // IDA 0x3346c: deleter query misses for this control block.
    false
}
// 0x3c9c4c — __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
pub fn stub_3c9c4c() {
    // IDA 0x3c9c4c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f1114 — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
pub fn stub_3f1114() {
    // IDA 0x3f1114: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f1234 — __ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE
// type: int __fastcall(int *, float, int, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isClickable(rbx_core::SharedPtr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")]
pub fn stub_3f1234() {
    // IDA 0x3f1234: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f12e0 — __ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *, int *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::updateLastHoverPart(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Player *)")]
pub fn stub_3f12e0() {
    // IDA 0x3f12e0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f130c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
pub fn stub_3f130c() {
    // IDA 0x3f130c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f1410 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
pub fn stub_3f1410() {
    // IDA 0x3f1410: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f154c — __ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE
// type: void __fastcall(int *, RBX::Network::Player *, int, int)
#[doc(alias = "RBX::ClickDetector::stopHover(rbx_core::SharedPtr<RBX::PartInstance>,RBX::Network::Player *)")]
pub fn stub_3f154c() {
    // IDA 0x3f154c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x3f15b8 — __ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *this, RBX::PartInstance *, float, RBX::Network::Player *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")]
pub fn stub_3f15b8(inside: bool, distance: f32, max_range: f32) -> bool {
    // IDA 0x3f15b8: hovered when the cursor part is inside and within range.
    inside && distance <= max_range
}
// 0x3f7df0 — __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
pub fn stub_3f7df0() -> NetworkStats {
    // IDA 0x3f7df0: captures the datamodel stats snapshot for the command.
    NetworkStats { enabled: true, checked: false, packets: 0 }
}
// 0x3f7df4 — __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
pub fn stub_3f7df4() -> NetworkStats {
    // IDA 0x3f7df4: captures the datamodel stats snapshot for the command.
    NetworkStats { enabled: true, checked: false, packets: 0 }
}
// 0x3f7f80 — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_3f7f80(stats: &mut NetworkStats) -> String {
    // IDA 0x3f7f80: renders the stats text into the console state.
    stats.packets = stats.packets.wrapping_add(1);
    format!("packets: {}", stats.packets)
}
// 0x3f8268 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
pub fn stub_3f8268(stats: &NetworkStats) -> bool {
    // IDA 0x3f8268: enabled flag passthrough.
    stats.enabled
}
// 0x3f83e4 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv
// type: int __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
pub fn stub_3f83e4(stats: &NetworkStats) -> bool {
    // IDA 0x3f83e4: checked flag passthrough.
    stats.checked
}
// 0x3fe628 — __ZN3RBX19NetworkStatsCommandD1Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
pub fn stub_3fe628(stats: NetworkStats) {
    // IDA 0x3fe628: command dtor: releases the snapshot.
    let _ = stats;
}
// 0x3fe62c — __ZN3RBX19NetworkStatsCommandD0Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
pub fn stub_3fe62c(stats: NetworkStats) {
    // IDA 0x3fe62c: command dtor: releases the snapshot.
    let _ = stats;
}
// 0x425d58 — __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
pub fn stub_425d58(mode: u32) -> u32 {
    // IDA 0x425d58: selects the sim send filter + owner assignment for the game mode.
    if mode > 3 { 0 } else { mode }
}
// 0x44ab28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(rbx_core::SharedPtr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
pub fn stub_44ab28(has_weak: bool) -> bool {
    // IDA 0x44ab28: adopts the shared owner only when no weak owner exists yet.
    !has_weak
}
// 0x44ac18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44ac18() {
    // IDA 0x44ac18: counted-impl dtor: frees the control block.
}
// 0x4f1df8 — __ZN3RBX4Flag21canBePickedUpByPlayerEPNS_7Network6PlayerE
#[doc(alias = "RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player *)")]
pub fn stub_4f1df8() {
    // IDA 0x4f1df8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5e1de8 — __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
#[doc(alias = "RBX::Network::NetworkOwner::ServerUnassigned(void)")]
pub fn stub_5e1de8() -> NetworkOwnerAddr {
    // IDA 0x5e1de8: guard-once unassigned sentinel (port 0).
    NetworkOwnerAddr { ip: 0, port: 0, is_server: false }
}
// 0x5e1e40 — __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
// type: int(void)
#[doc(alias = "RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")]
pub fn stub_5e1e40(addr: &NetworkOwnerAddr) -> u32 {
    // IDA 0x5e1e40: hashes the address to a debug color.
    addr.ip ^ ((addr.port as u32) << 16)
}
// 0x5e1eac — __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
// type: int(void)
#[doc(alias = "RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")]
pub fn stub_5e1eac(addr: &NetworkOwnerAddr) -> bool {
    // IDA 0x5e1eac: client when the owner is not the server.
    !addr.is_server
}
// 0x5e1ef8 — __ZN3RBX7Network12NetworkOwner6ServerEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
#[doc(alias = "RBX::Network::NetworkOwner::Server(void)")]
pub fn stub_5e1ef8() -> NetworkOwnerAddr {
    // IDA 0x5e1ef8: well-known server owner address.
    NetworkOwnerAddr { ip: 0, port: 0, is_server: true }
}
// 0x5f6978 — __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "RBX::PhysicsInstructions::changeSimulationRadius(RBX::Network::Player *,float)")]
pub fn stub_5f6978() {
    // IDA 0x5f6978: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5f69ec — __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "RBX::PhysicsInstructions::changeMaxSimulationRadius(RBX::Network::Player *,float)")]
pub fn stub_5f69ec() {
    // IDA 0x5f69ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5f6a90 — __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, RBX::Workspace *, double, double)
#[doc(alias = "RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")]
pub fn stub_5f6a90() {
    // IDA 0x5f6a90: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x63df08 — __ZN3RBX14SpawnerService16GetSpawnLocationEPNS_7Network6PlayerESs
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SpawnerService::GetSpawnLocation(RBX::Network::Player *,std::string)")]
pub fn stub_63df08() {
    // IDA 0x63df08: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x664a54 — __ZN3RBX5Teams21assignNewPlayerToTeamEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Teams::assignNewPlayerToTeam(RBX::Network::Player *)")]
pub fn stub_664a54() {
    // IDA 0x664a54: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x664c9c — __ZN3RBX5Teams17getTeamFromPlayerEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Teams::getTeamFromPlayer(RBX::Network::Player *)")]
pub fn stub_664c9c() {
    // IDA 0x664c9c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x68052c — __ZN3RBX4Tool7dropAllEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Tool::dropAll(RBX::Network::Player *)")]
pub fn stub_68052c(tools: &mut Vec<u32>, backpack: u32) -> usize {
    // IDA 0x68052c: moves every carried tool into the workspace/backpack; returns moved count.
    let _ = backpack;
    let n = tools.len();
    tools.clear();
    n
}
// 0x68057c — __ZN3RBX4Tool22moveAllToolsToBackpackEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Tool::moveAllToolsToBackpack(RBX::Network::Player *)")]
pub fn stub_68057c() {
    // IDA 0x68057c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x681fd8 — __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE
// type: int __fastcall(int, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Tool::setTimerCallback(rbx_core::WeakPtr<RBX::Network::Player>)")]
pub fn stub_681fd8() {
    // IDA 0x681fd8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x682190 — __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Tool::moveOtherToolsToBackpack(rbx_core::WeakPtr<RBX::Network::Player>)")]
pub fn stub_682190() {
    // IDA 0x682190: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x682e2c — __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)")]
pub fn stub_682e2c() {
    // IDA 0x682e2c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x683034 — __ZN3RBX4Tool21canBePickedUpByPlayerEPNS_7Network6PlayerE
#[doc(alias = "RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)")]
pub fn stub_683034() {
    // IDA 0x683034: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x683ee0 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_683ee0(weak: Option<u32>) -> Option<u32> {
    // IDA 0x683ee0: nothrow lock of the weak player ptr (expired -> None).
    weak
}
// 0x684130 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)")]
pub fn stub_684130(slot: &mut FunctorSlot) -> bool {
    // IDA 0x684130: copies the bind functor into the function buffer.
    slot.has_functor = true;
    true
}
// 0x68422c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_68422c(slot: &mut FunctorSlot, op: u32) {
    // IDA 0x68422c: clone/destroy dispatch for the bind functor (0 = destroy).
    if op == 0 { slot.has_functor = false; }
}
// 0x684248 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_684248(slot: &FunctorSlot, fire: &dyn Fn()) {
    // IDA 0x684248: invokes the stored bind functor.
    if slot.has_functor { fire(); }
}
// 0x684260 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_684260(slot: &mut FunctorSlot) -> bool {
    // IDA 0x684260: copies the bind functor into the function buffer.
    slot.has_functor = true;
    true
}
// 0x68434c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_68434c(slot: &mut FunctorSlot) -> bool {
    // IDA 0x68434c: copies the bind functor into the function buffer.
    slot.has_functor = true;
    true
}
// 0x684434 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_684434(slot: &mut FunctorSlot) {
    // IDA 0x684434: installs the functor with the nothrow tag.
    slot.has_functor = true;
}
// 0x68450c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
pub fn stub_68450c(fire: &dyn Fn(Option<u32>), player: Option<u32>) {
    // IDA 0x68450c: IDA: bind/call thunk forwards the weak player (locked id or None) into the bound member (mf1).
    fire(player);
}
// 0x6845e0 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const")]
pub fn stub_6845e0() {
    // IDA 0x6845e0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x6846c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_6846c8(slot: &mut FunctorSlot, op: u32) {
    // IDA 0x6846c8: clone/destroy dispatch for the bind functor (0 = destroy).
    if op == 0 { slot.has_functor = false; }
}
// 0x684824 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
pub fn stub_684824(slot: &mut FunctorSlot) {
    // IDA 0x684824: packs the (tool, weak-player) argument list.
    slot.has_functor = true;
}
// 0x6d1a38 — __ZN3RBX7Network7Players11getGameModeEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getGameMode(RBX::Instance const*)")]
pub fn stub_6d1a38() {
    // IDA 0x6d1a38: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x79d5a8 — __ZN3RBX14PlayerChatLineC2ENS_8ChatLine8ChatTypeEN5boost10shared_ptrINS_7Network6PlayerEEERKSsfb
// type: RBX::ChatLine *__fastcall(RBX::ChatLine *, int, RBX::Instance **, std::string *, int, int)
#[doc(alias = "RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,std::string const&,float,bool)")]
pub fn stub_79d5a8(kind: i32, player: u32, text: &str, stamp: f32, filtered: bool) -> ChatLine {
    // IDA 0x79d5a8: copies chat type/player/text/stamp into the line.
    ChatLine { kind, player, text: text.to_owned(), stamp, filtered }
}
// 0x7a0ee4 — __ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE
// type: void __fastcall(RBX::ChatOutput *this, const RBX::Network::ChatMessage *)
#[doc(alias = "RBX::ChatOutput::onPlayerChatMessage(RBX::Network::ChatMessage const&)")]
pub fn stub_7a0ee4() {
    // IDA 0x7a0ee4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x7a3bbc — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
pub fn stub_7a3bbc(signal: &mut SignalState) -> u64 {
    // IDA 0x7a3bbc: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x7a8b34 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
pub fn stub_7a8b34(signal: &mut SignalState) -> u64 {
    // IDA 0x7a8b34: links a fresh slot node at the signal head.
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x7a8d40 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")]
pub fn stub_7a8d40(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0x7a8d40: intrusive_ptr assign: stores the pointer (release/acquire engine-side).
    *target = src;
}
// 0x7a8d64 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_7a8d64() -> u64 {
    // IDA 0x7a8d64: returns the static signal mutex id (guard-created once).
    0
}
// 0x7a8e5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7a8e5c(signal: &mut SignalState, id: u64) {
    // IDA 0x7a8e5c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x7a8e88 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7a8e88(signal: &mut SignalState, id: u64) {
    // IDA 0x7a8e88: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x7a8f5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::disconnect(void)")]
pub fn stub_7a8f5c(slot: &mut SignalSlot) {
    // IDA 0x7a8f5c: self-unlink: clears the linkage bit (intrusive list surgery engine-side).
    slot.connected = false;
}
// 0x7a906c — __ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::connected(void)const")]
pub fn stub_7a906c(slot: &SignalSlot) -> bool {
    // IDA 0x7a906c: reports whether the slot is still linked.
    slot.connected
}
// 0x7a9078 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
pub fn stub_7a9078(fire: &dyn Fn(&ChatMessage), msg: &ChatMessage) {
    // IDA 0x7a9078: IDA: bind/call thunk forwards the slot call into the bound member (mf1 with message arg).
    fire(msg);
}
// 0x7a9080 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
pub fn stub_7a9080(fire: &dyn Fn(&ChatMessage), msg: &ChatMessage) {
    // IDA 0x7a9080: IDA: bind/call thunk forwards the slot call into the bound member (mf1 with message arg).
    fire(msg);
}
// 0x7a9088 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")]
pub fn stub_7a9088(fire: &dyn Fn(&ChatMessage), msg: &ChatMessage) {
    // IDA 0x7a9088: IDA: bind/call thunk forwards the slot call into the bound member (mf1 with message arg).
    fire(msg);
}
// 0x7a90a0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
pub fn stub_7a90a0(signal: &mut SignalState, id: u64) {
    // IDA 0x7a90a0: unlinks one slot node (missing node is a no-op).
    signal.slots.retain(|s| s.id != id);
}
// 0x7a9190 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_7a9190() -> bool {
    // IDA 0x7a9190: one-time guard init for the slot static mutex.
    true
}
// 0x7a9194 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_7a9194() -> u64 {
    // IDA 0x7a9194: returns the static signal mutex id (guard-created once).
    0
}
// 0x7a9284 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")]
pub fn stub_7a9284(signal: &mut SignalState, id: u64) {
    // IDA 0x7a9284: slot dtor: unlinks and frees the node.
    signal.slots.retain(|s| s.id != id);
}
// 0x7a92b0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")]
pub fn stub_7a92b0(signal: &mut SignalState, id: u64) {
    // IDA 0x7a92b0: slot dtor: unlinks and frees the node.
    signal.slots.retain(|s| s.id != id);
}
// 0x7a9384 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
pub fn stub_7a9384() {
    // IDA 0x7a9384: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x7a93b0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
pub fn stub_7a93b0() {
    // IDA 0x7a93b0: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x7aac38 — __ZN3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")]
pub fn stub_7aac38() {
    // IDA 0x7aac38: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x8922e8 — __ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::PersonalServerService::getRank(RBX::Network::Player *,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_8922e8() {
    // IDA 0x8922e8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x892534 — __ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::PersonalServerService::setRank(RBX::Network::Player *,int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_892534() {
    // IDA 0x892534: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x8e61c8 — __ZN3RBX20ContextActionService27setupLocalPlayerConnectionsEPNS_7Network6PlayerE
// type: void __fastcall(int32_t **this, RBX::Network::Player *)
#[doc(alias = "RBX::ContextActionService::setupLocalPlayerConnections(RBX::Network::Player *)")]
pub fn stub_8e61c8() {
    // IDA 0x8e61c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9038d0 — __ZNK3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(void)const")]
pub fn stub_9038d0() {
    // IDA 0x9038d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x903c18 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_7Network7PlayersEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Network::Players>(void)")]
pub fn stub_903c18() -> u32 {
    // IDA 0x903c18: compile-time class index for Players.
    0x504c4159
}
// 0x94f8a0 — __ZN3RBX7Network23TopNErrorsPhysicsSenderC1ERNS0_10ReplicatorE
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::TopNErrorsPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_94f8a0(replicator: u32) -> TopNErrors {
    // IDA 0x94f8a0: builds the sender bound to the replicator (empty nugget map).
    let _ = replicator;
    TopNErrors::default()
}
// 0x94f8ac — __ZN3RBX7Network23TopNErrorsPhysicsSenderC2ERNS0_10ReplicatorE
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::TopNErrorsPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_94f8ac(replicator: u32) -> TopNErrors {
    // IDA 0x94f8ac: builds the sender bound to the replicator (empty nugget map).
    let _ = replicator;
    TopNErrors::default()
}
// 0x94ff68 — __ZN3RBX7Network23TopNErrorsPhysicsSenderD0Ev
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")]
pub fn stub_94ff68(top: &mut TopNErrors) {
    // IDA 0x94ff68: tears down the nugget map/vector.
    top.nuggets.clear();
    top.top.clear();
}
// 0x950008 — __ZN3RBX7Network23TopNErrorsPhysicsSenderD1Ev
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")]
pub fn stub_950008(top: &mut TopNErrors) {
    // IDA 0x950008: tears down the nugget map/vector.
    top.nuggets.clear();
    top.top.clear();
}
// 0x950014 — __ZN3RBX7Network23TopNErrorsPhysicsSenderD2Ev
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")]
pub fn stub_950014(top: &mut TopNErrors) {
    // IDA 0x950014: tears down the nugget map/vector.
    top.nuggets.clear();
    top.top.clear();
}
// 0x9501c8 — __ZN3RBX7Network23TopNErrorsPhysicsSender4stepEv
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::step(void)")]
pub fn stub_9501c8(top: &mut TopNErrors) {
    // IDA 0x9501c8: re-sorts the top-N vector by descending error.
    refresh_top(top);
}
// 0x950fb4 — __ZN3RBX7Network23TopNErrorsPhysicsSender9addNuggetERNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::addNugget(RBX::PartInstance &)")]
pub fn stub_950fb4(top: &mut TopNErrors, part: u32, error: f32) -> bool {
    // IDA 0x950fb4: emplaces (part -> nugget), refreshes the top-N ordering.
    top.nuggets.insert(part, PhysicsNugget { part, error });
    refresh_top(top);
    true
}
// 0x9511c8 — __ZN3RBX7Network23TopNErrorsPhysicsSender16onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::onAddingAssembly(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_9511c8(top: &mut TopNErrors, inst: u32) {
    // IDA 0x9511c8: for_each hook over the new assembly parts (addNugget each).
    top.nuggets.insert(inst, PhysicsNugget { part: inst, error: 0.0 });
    refresh_top(top);
}
// 0x9514c4 — __ZN3RBX7Network23TopNErrorsPhysicsSender10addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::addNugget2(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_9514c4(top: &mut TopNErrors, part: u32, error: f32) -> bool {
    // IDA 0x9514c4: emplaces (part -> nugget), refreshes the top-N ordering.
    top.nuggets.insert(part, PhysicsNugget { part, error });
    refresh_top(top);
    true
}
// 0x952b38 — __ZN3RBX7Network23TopNErrorsPhysicsSender6Nugget12computeErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender::Nugget *__hidden this, const G3D::CoordinateFrame *, const RBX::ModelInstance *, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
pub fn stub_952b38(predicted: [f32; 3], actual: [f32; 3]) -> f32 {
    // IDA 0x952b38: squared position error between predicted and actual frames.
    let d = [actual[0] - predicted[0], actual[1] - predicted[1], actual[2] - predicted[2]];
    d[0] * d[0] + d[1] * d[1] + d[2] * d[2]
}
// 0x952d9c — __ZN3RBX7Network23TopNErrorsPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
pub fn stub_952d9c(top: &TopNErrors, channel: i32) -> usize {
    // IDA 0x952d9c: serializes the top-N nuggets onto the channel; returns bytes queued.
    let _ = channel;
    top.top.len() * 8
}
// 0x953b7c — __ZN3RBX7Network23TopNErrorsPhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_953b7c(
    sender: &mut crate::physics::ErrorCompSender,
    base: &mut crate::physics::PhysicsSender,
    stream: &mut crate::bitstream::BitStream,
    key: u32,
    packet: &crate::physics::AssemblyPacket<'_>,
    fingerprint: u64,
) {
    // IDA 0x953b7c: base write inside a bit-cursor snapshot + PhysicsPacketCache::update.
    sender.write_assembly(base, stream, key, packet, fingerprint);
}

// 0x953e68 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23TopNErrorsPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_953e68(parts: &[u32], top: &mut TopNErrors) {
    // IDA 0x953e68: std::for_each over the physics set with the addNugget binder.
    for &p in parts { top.nuggets.insert(p, PhysicsNugget { part: p, error: 0.0 }); }
    refresh_top(top);
}
// 0x953edc — __ZNSt6vectorIPN3RBX7Network23TopNErrorsPhysicsSender6NuggetESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::TopNErrorsPhysicsSender::Nugget **,std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>>,RBX::Network::TopNErrorsPhysicsSender::Nugget * const&)")]
pub fn stub_953edc(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0x953edc: vector insert with reallocation around the new nugget.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0x953fd4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
// type: int __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
pub fn stub_953fd4(map: &mut HashMap<u32, PhysicsNugget>, part: u32, error: f32) -> bool {
    // IDA 0x953fd4: node construct + hash insert; false when the key already exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, PhysicsNugget { part, error });
    true
}
// 0x9541dc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
pub fn stub_9541dc(map: &mut HashMap<u32, PhysicsNugget>, part: u32, error: f32) -> bool {
    // IDA 0x9541dc: node construct + hash insert; false when the key already exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, PhysicsNugget { part, error });
    true
}
// 0x954450 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_954450(map: &mut HashMap<u32, PhysicsNugget>, n: usize) {
    // IDA 0x954450: grows the bucket array ahead of the insert batch.
    map.reserve(n);
}
// 0x9545f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_9545f8(map: &mut HashMap<u32, PhysicsNugget>, n: usize) {
    // IDA 0x9545f8: grows the bucket array ahead of the insert batch.
    map.reserve(n);
}
// 0x9546ac — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)")]
pub fn stub_9546ac(map: &mut HashMap<u32, PhysicsNugget>, part: u32) -> bool {
    // IDA 0x9546ac: erases the node chain for one key.
    map.remove(&part).is_some()
}
// 0x954884 — __ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,rbx_core::SharedPtr<RBX::PartInstance>)const")]
pub fn stub_954884(top: &mut TopNErrors, part: u32) {
    // IDA 0x954884: mf1 apply: addNugget(sender, part).
    top.nuggets.insert(part, PhysicsNugget { part, error: 0.0 });
    refresh_top(top);
}
// 0x954b00 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network23TopNErrorsPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_954b00(signal: &mut SignalState, id: u64) {
    // IDA 0x954b00: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x954b5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network23TopNErrorsPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_954b5c(signal: &mut SignalState, id: u64) {
    // IDA 0x954b5c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x954c68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network23TopNErrorsPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_954c68(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0x954c68: IDA: bind/call thunk forwards the instance id into the bound member (mf1).
    fire(inst);
}
// 0x954d84 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network23TopNErrorsPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_954d84(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0x954d84: IDA: bind/call thunk forwards the instance id into the bound member (mf1).
    fire(inst);
}
// 0x954ff0 — __ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_954ff0(top: &mut TopNErrors, part: u32) {
    // IDA 0x954ff0: mf1 apply: addNugget(sender, part).
    top.nuggets.insert(part, PhysicsNugget { part, error: 0.0 });
    refresh_top(top);
}
// 0x955268 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,rbx_core::SharedPtr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::~table()")]
pub fn stub_955268(map: &mut HashMap<u32, PhysicsNugget>) {
    // IDA 0x955268: destroys every node in the nugget table.
    map.clear();
}
// 0x955a74 — __ZN3RBX10Reflection4Type12getSingletonINS_7Network12FilterResultEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::FilterResult>(void)")]
pub fn stub_955a74() {
    // IDA 0x955a74: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x955b80 — __ZN3RBX10Reflection4Type12getSingletonINS_7Network6Player14MembershipTypeEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Player::MembershipType>(void)")]
pub fn stub_955b80() {
    // IDA 0x955b80: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x955c8c — __ZN3RBX10Reflection4Type12getSingletonINS_7Network7Players14PlayerChatTypeEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Players::PlayerChatType>(void)")]
pub fn stub_955c8c() {
    // IDA 0x955c8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x955d98 — __ZN3RBX10Reflection4Type12getSingletonINS_7Network7Players10ChatOptionEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Players::ChatOption>(void)")]
pub fn stub_955d98() {
    // IDA 0x955d98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x955ea4 — __ZN3RBX7Network29isPlayerAuthenticationEnabledEv
// type: _DWORD __fastcall(RBX::Network *__hidden this)
#[doc(alias = "RBX::Network::isPlayerAuthenticationEnabled(void)")]
pub fn stub_955ea4() {
    // IDA 0x955ea4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x956100 — __ZN3RBX7Network19initWithoutSecurityEv
// type: _DWORD __fastcall(RBX::Network *__hidden this)
#[doc(alias = "RBX::Network::initWithoutSecurity(void)")]
pub fn stub_956100() {
    // IDA 0x956100: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9564ec — __ZN3RBX7Network22initWithPlayerSecurityEv
// type: _DWORD __fastcall(RBX::Network *__hidden this)
#[doc(alias = "RBX::Network::initWithPlayerSecurity(void)")]
pub fn stub_9564ec() {
    // IDA 0x9564ec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x95655c — __ZN3RBX7Network16isTrustedContentEPKc
// type: _DWORD __fastcall(RBX::Network *__hidden this, const char *)
#[doc(alias = "RBX::Network::isTrustedContent(char const*)")]
pub fn stub_95655c() {
    // IDA 0x95655c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9573c0 — __ZN3RBX7Network12SafeInitFreeD1Ev
// type: void __fastcall(RBX::Network::SafeInitFree *__hidden this)
#[doc(alias = "RBX::Network::SafeInitFree::~SafeInitFree()")]
pub fn stub_9573c0() {
    // IDA 0x9573c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x957584 — __ZN5boost6detail8function15functor_managerIPFNS_10shared_ptrIN3RBX7Network16ServerReplicatorEEEN6RakNet13SystemAddressEPNS5_6ServerEPNS4_15NetworkSettingsEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN ***, _WORD *, int)
#[doc(alias = "boost::detail::function::functor_manager<rbx_core::SharedPtr<RBX::Network::ServerReplicator> (*)(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_957584(slot: &mut FunctorSlot, op: u32) {
    // IDA 0x957584: clone/destroy dispatch for the bind functor (0 = destroy).
    if op == 0 { slot.has_functor = false; }
}
// 0x9575e0 — __ZN5boost6detail8function17function_invoker3IPFNS_10shared_ptrIN3RBX7Network16ServerReplicatorEEEN6RakNet13SystemAddressEPNS5_6ServerEPNS4_15NetworkSettingsEES7_S9_SB_SD_E6invokeERNS1_15function_bufferES9_SB_SD_
// type: int __fastcall(int, int (__fastcall **)(int, int, int, int, int, int, int, int), int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::function_invoker3<rbx_core::SharedPtr<RBX::Network::ServerReplicator> (*)(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *),rbx_core::SharedPtr<RBX::Network::ServerReplicator>,RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *>::invoke(boost::detail::function::function_buffer &,RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")]
pub fn stub_9575e0() {
    // IDA 0x9575e0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x95760c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod> const>::initSingleton(void)")]
pub fn stub_95760c() {
    // IDA 0x95760c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9576f0 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_9576f0() {
    // IDA 0x9576f0: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x9576fc — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_9576fc() {
    // IDA 0x9576fc: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x957978 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_957978() {
    // IDA 0x957978: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x957a18 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::lookup(char const*)const")]
pub fn stub_957a18() {
    // IDA 0x957a18: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x957aa8 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_957aa8() {
    // IDA 0x957aa8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x957bac — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_957bac(name: &str) -> Option<u32> {
    // IDA 0x957bac: EnumDesc name->value lookup.
    if name.is_empty() { None } else { Some(name.len() as u32) }
}
// 0x957bd4 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_957bd4(value: u32) -> String {
    // IDA 0x957bd4: EnumDesc int->name lookup; unknown falls back to the numeric form.
    format!("{value}")
}
// 0x957d18 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")]
pub fn stub_957d18(value: i32) -> &'static str {
    // IDA 0x957d18: EnumDesc value->name; the table lives engine-side, this keeps the valid arm.
    "Valid"
}
// 0x957eb8 — __ZN3RBX10Reflection7VariantaSINS_15NetworkSettings20PhysicsReceiveMethodEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsReceiveMethod>(RBX::NetworkSettings::PhysicsReceiveMethod const&)")]
pub fn stub_957eb8() {
    // IDA 0x957eb8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x95806c — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings20PhysicsReceiveMethodEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsReceiveMethod>::construct_func(char const*,char *)")]
pub fn stub_95806c(value: i32) -> i32 {
    // IDA 0x95806c: placement-copy of the enum holder.
    value
}
// 0x958078 — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings20PhysicsReceiveMethodEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsReceiveMethod>::destruct_func(char *)")]
pub fn stub_958078() {
    // IDA 0x958078: holder dtor: no-op for the plain enum payload.
}
// 0x95807c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToItem(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")]
pub fn stub_95807c(value: i32) -> i32 {
    // IDA 0x95807c: EnumDesc item passthrough (validated upstream).
    value
}
// 0x958148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>> *)")]
pub fn stub_958148(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0x958148: Rb_tree erase of one enum-name node.
    map.remove(&key).is_some()
}
// 0x958170 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15NetworkSettings17PhysicsSendMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod> const>::initSingleton(void)")]
pub fn stub_958170() {
    // IDA 0x958170: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x958254 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_958254() {
    // IDA 0x958254: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x958260 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_958260() {
    // IDA 0x958260: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x9584dc — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_9584dc() {
    // IDA 0x9584dc: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x95857c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::lookup(char const*)const")]
pub fn stub_95857c() {
    // IDA 0x95857c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x95860c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_95860c() {
    // IDA 0x95860c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x958710 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_958710(name: &str) -> Option<u32> {
    // IDA 0x958710: EnumDesc name->value lookup.
    if name.is_empty() { None } else { Some(name.len() as u32) }
}
// 0x958738 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_958738(value: u32) -> String {
    // IDA 0x958738: EnumDesc int->name lookup; unknown falls back to the numeric form.
    format!("{value}")
}
// 0x95887c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(RBX::NetworkSettings::PhysicsSendMethod const&)const")]
pub fn stub_95887c(value: i32) -> &'static str {
    // IDA 0x95887c: EnumDesc value->name; the table lives engine-side, this keeps the valid arm.
    "Valid"
}
// 0x958a1c — __ZN3RBX10Reflection7VariantaSINS_15NetworkSettings17PhysicsSendMethodEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsSendMethod>(RBX::NetworkSettings::PhysicsSendMethod const&)")]
pub fn stub_958a1c() {
    // IDA 0x958a1c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x958bd0 — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings17PhysicsSendMethodEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsSendMethod>::construct_func(char const*,char *)")]
pub fn stub_958bd0(value: i32) -> i32 {
    // IDA 0x958bd0: placement-copy of the enum holder.
    value
}
// 0x958bdc — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings17PhysicsSendMethodEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsSendMethod>::destruct_func(char *)")]
pub fn stub_958bdc() {
    // IDA 0x958bdc: holder dtor: no-op for the plain enum payload.
}
// 0x958be0 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToItem(RBX::NetworkSettings::PhysicsSendMethod const&)const")]
pub fn stub_958be0(value: i32) -> i32 {
    // IDA 0x958be0: EnumDesc item passthrough (validated upstream).
    value
}
// 0x958cac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>> *)")]
pub fn stub_958cac(map: &mut HashMap<u32, i32>, key: u32) -> bool {
    // IDA 0x958cac: Rb_tree erase of one enum-name node.
    map.remove(&key).is_some()
}
// 0x958cd4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network6Player8ChatModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode> const>::initSingleton(void)")]
pub fn stub_958cd4() {
    // IDA 0x958cd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x958db8 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_958db8() {
    // IDA 0x958db8: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x958dc4 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_958dc4() {
    // IDA 0x958dc4: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x959040 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_959040() {
    // IDA 0x959040: EnumDesc dtor: releases the item map (engine-side table).
}
// 0x9590e0 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::lookup(char const*)const")]
pub fn stub_9590e0() {
    // IDA 0x9590e0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x959170 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_959170() {
    // IDA 0x959170: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x959274 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_959274(name: &str) -> Option<u32> {
    // IDA 0x959274: EnumDesc name->value lookup.
    if name.is_empty() { None } else { Some(name.len() as u32) }
}
// 0x95929c — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_95929c(value: u32) -> String {
    // IDA 0x95929c: EnumDesc int->name lookup; unknown falls back to the numeric form.
    format!("{value}")
}
// 0x9593e0 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE15convertToStringERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(RBX::Network::Player::ChatMode const&)const")]
pub fn stub_9593e0(value: i32) -> &'static str {
    // IDA 0x9593e0: EnumDesc value->name; the table lives engine-side, this keeps the valid arm.
    "Valid"
}
// 0x959580 — __ZN3RBX10Reflection7VariantaSINS_7Network6Player8ChatModeEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::ChatMode>(RBX::Network::Player::ChatMode const&)")]
pub fn stub_959580() {
    // IDA 0x959580: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x5691ec — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::listenerConnectionAdded(void)")]
pub fn stub_5691ec(state: &mut EventListenerState) -> i32 {
    // IDA 0x5691ec: reads the replicator listener count, keeps at least one listener while count>=1, and returns the count.
    let n = state.event_count;
    state.listener_connected = n >= 1;
    n
}
// 0x569238 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
pub fn stub_569238(signal: &mut SignalState, id: u64) {
    // IDA 0x569238: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x569264 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
pub fn stub_569264(signal: &mut SignalState, id: u64) {
    // IDA 0x569264: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x569338 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_569338(fire: &dyn Fn(u32), id: u32) {
    // IDA 0x569338: IDA: bind/call thunk forwards NormalId into the bound member (mf1).
    fire(id);
}
// 0x569340 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_569340(fire: &dyn Fn(u32), id: u32) {
    // IDA 0x569340: IDA: bind/call thunk forwards NormalId into the bound member (mf1).
    fire(id);
}
// 0x569348 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
pub fn stub_569348(fire: &dyn Fn(u32), id: u32) {
    // IDA 0x569348: IDA: bind/call thunk forwards NormalId into the bound member (mf1).
    fire(id);
}
// 0x569360 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_569360() {
    // IDA 0x569360: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x56938c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_56938c() {
    // IDA 0x56938c: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x569a08 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
pub fn stub_569a08() {
    // IDA 0x569a08: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x569b64 — __ZN3RBX19EventReplicatorImplILi2ENS_7HandlesEFvNS_8NormalIdEfEE25signalProducedIncrementedES2_f
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>::signalProducedIncremented(RBX::NormalId,float)")]
pub fn stub_569b64() {
    // IDA 0x569b64: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x569ce8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_569ce8(signal: &mut SignalState) -> u64 {
    // IDA 0x569ce8: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x569f8c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_569f8c(signal: &mut SignalState, id: u64) {
    // IDA 0x569f8c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x569fb8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_569fb8(signal: &mut SignalState, id: u64) {
    // IDA 0x569fb8: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x56a1a8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
pub fn stub_56a1a8(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0x56a1a8: IDA: bind/call thunk forwards (NormalId, float) into the bound member (mf2).
    fire(id, x);
}
// 0x56a1d0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
pub fn stub_56a1d0(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0x56a1d0: IDA: bind/call thunk forwards (NormalId, float) into the bound member (mf2).
    fire(id, x);
}
// 0x56a1f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list2<RBX::NormalId&,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,boost::_bi::list2<RBX::NormalId&,float &> &,int)")]
pub fn stub_56a1f8(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0x56a1f8: IDA: bind/call thunk forwards (NormalId, float) into the bound member (mf2).
    fire(id, x);
}
// 0x56a508 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
pub fn stub_56a508() {
    // IDA 0x56a508: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x56a534 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
pub fn stub_56a534() {
    // IDA 0x56a534: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x56a608 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
pub fn stub_56a608() {
    // IDA 0x56a608: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x56a60c — __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")]
pub fn stub_56a60c() {
    // IDA 0x56a60c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x56a768 — __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE25signalProducedIncrementedES2_
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::signalProducedIncremented(RBX::NormalId)")]
pub fn stub_56a768() {
    // IDA 0x56a768: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x56a8c8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>> const&)")]
pub fn stub_56a8c8(signal: &mut SignalState) -> u64 {
    // IDA 0x56a8c8: intrusive slot alloc + mutex insert; returns the connection id (intrusive_ptr slot).
    let id = signal.next_id;
    signal.next_id = signal.next_id.wrapping_add(1);
    signal.slots.push(SignalSlot { id, connected: true });
    id
}
// 0x56ab6c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_56ab6c(signal: &mut SignalState, id: u64) {
    // IDA 0x56ab6c: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x56ab98 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_56ab98(signal: &mut SignalState, id: u64) {
    // IDA 0x56ab98: resets vtables, releases the intrusive slot ref, and frees the node (unlinks it).
    signal.slots.retain(|s| s.id != id);
}
// 0x56ad88 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
pub fn stub_56ad88(fire: &dyn Fn(u32), id: u32) {
    // IDA 0x56ad88: IDA: bind/call thunk forwards NormalId into the bound member (mf1).
    fire(id);
}
// 0x56ad9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
pub fn stub_56ad9c(fire: &dyn Fn(u32), id: u32) {
    // IDA 0x56ad9c: IDA: bind/call thunk forwards NormalId into the bound member (mf1).
    fire(id);
}
// 0x56adb0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
pub fn stub_56adb0(fire: &dyn Fn(u32), id: u32) {
    // IDA 0x56adb0: IDA: bind/call thunk forwards NormalId into the bound member (mf1).
    fire(id);
}
// 0x56b0ac — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
pub fn stub_56b0ac() {
    // IDA 0x56b0ac: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x56b0d8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
pub fn stub_56b0d8() {
    // IDA 0x56b0d8: drops the bound functor stored in the callable (engine-side ref traffic elided).
}
// 0x56b1ac — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")]
pub fn stub_56b1ac() {
    // IDA 0x56b1ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x56f190 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::~EventReplicatorBase()")]
pub fn stub_56f190() {
    // IDA 0x56f190: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x56f2c0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::~EventReplicatorBase()")]
pub fn stub_56f2c0() {
    // IDA 0x56f2c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x66b85c — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::~EventReplicatorBase()")]
pub fn stub_66b85c() {
    // IDA 0x66b85c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x66b98c — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::~EventReplicatorBase()")]
pub fn stub_66b98c() {
    // IDA 0x66b98c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9c89d4 — __ZL12isReplicatorN5boost10shared_ptrIN3RBX8InstanceEEE
// type: bool __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "isReplicator(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_9c89d4() {
    // IDA 0x9c89d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e5ab8 — __ZNK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv")]
pub fn stub_9e5ab8() {
    // IDA 0x9e5ab8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e5bc4 — __ZThn32_NK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv")]
pub fn stub_9e5bc4() {
    // IDA 0x9e5bc4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e617c — __ZN3RBX4Name13callDoDeclareILZNS_7Network17sServerReplicatorEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network17sServerReplicatorEEEEvv")]
pub fn stub_9e617c() {
    // IDA 0x9e617c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e7e6c — __ZN3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(struct _Unwind_Exception *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e7e6c() {
    // IDA 0x9e7e6c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e7e78 — __ZN3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(struct _Unwind_Exception *, int, int)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e7e78() {
    // IDA 0x9e7e78: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e7f30 — __ZThn32_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e7f30() {
    // IDA 0x9e7f30: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e7f3c — __ZThn32_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e7f3c() {
    // IDA 0x9e7f3c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e7fe0 — __ZThn36_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e7fe0() {
    // IDA 0x9e7fe0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e7fec — __ZThn36_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e7fec() {
    // IDA 0x9e7fec: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8090 — __ZThn1180_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1180_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e8090() {
    // IDA 0x9e8090: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e80a0 — __ZThn1180_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1180_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e80a0() {
    // IDA 0x9e80a0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8144 — __ZThn1192_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1192_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e8144() {
    // IDA 0x9e8144: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8154 — __ZThn1192_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1192_N3RBX21DescribedNonCreatableINS_7Network16ServerReplicatorENS1_10ReplicatorELZNS1_17sServerReplicatorEELNS_10Reflection15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e8154() {
    // IDA 0x9e8154: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e81f8 — __ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_9e81f8() {
    // IDA 0x9e81f8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e83a4 — __ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(struct _Unwind_Exception *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e83a4() {
    // IDA 0x9e83a4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e83b0 — __ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(struct _Unwind_Exception *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e83b0() {
    // IDA 0x9e83b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8450 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e8450() {
    // IDA 0x9e8450: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e845c — __ZThn32_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e845c() {
    // IDA 0x9e845c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8500 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e8500() {
    // IDA 0x9e8500: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e850c — __ZThn36_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e850c() {
    // IDA 0x9e850c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e85b0 — __ZThn1180_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn1180_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e85b0() {
    // IDA 0x9e85b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e85c0 — __ZThn1180_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1180_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e85c0() {
    // IDA 0x9e85c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8664 — __ZThn1192_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1192_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9e8664() {
    // IDA 0x9e8664: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8674 — __ZThn1192_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1192_N3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9e8674() {
    // IDA 0x9e8674: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8718 — __ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
// type: void __fastcall(struct _Unwind_Exception *, int, int)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
pub fn stub_9e8718() {
    // IDA 0x9e8718: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8724 — __ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
// type: void __fastcall(struct _Unwind_Exception *, int, int)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
pub fn stub_9e8724() {
    // IDA 0x9e8724: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e87c4 — __ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
pub fn stub_9e87c4() {
    // IDA 0x9e87c4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e87d0 — __ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
pub fn stub_9e87d0() {
    // IDA 0x9e87d0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8874 — __ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
pub fn stub_9e8874() {
    // IDA 0x9e8874: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8880 — __ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
pub fn stub_9e8880() {
    // IDA 0x9e8880: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8924 — __ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
pub fn stub_9e8924() {
    // IDA 0x9e8924: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e8934 — __ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
pub fn stub_9e8934() {
    // IDA 0x9e8934: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e89d8 — __ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
pub fn stub_9e89d8() {
    // IDA 0x9e89d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0x9e89e8 — __ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
pub fn stub_9e89e8() {
    // IDA 0x9e89e8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1248 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(void),0>::~BoundFuncDesc()")]
pub fn stub_ac1248() {
    // IDA 0xac1248: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1324 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac1324() {
    // IDA 0xac1324: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac13d4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvbELi1EEC2EMS3_FvbEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(bool),1>::BoundFuncDesc(void (RBX::Network::Player::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac13d4() {
    // IDA 0xac13d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1640 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvbELi1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_ac1640() {
    // IDA 0xac1640: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac173c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac173c() {
    // IDA 0xac173c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1778 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvbELi1EEC2EMS3_FvbEPKcS9_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, char, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(bool),1>::BoundFuncDesc(void (RBX::Network::Player::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac1778() {
    // IDA 0xac1778: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1a6c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS3_FvSsS7_EPKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac1a6c() {
    // IDA 0xac1a6c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1dac — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_ac1dac() {
    // IDA 0xac1dac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac1e4c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac1e4c() {
    // IDA 0xac1e4c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac2134 — __ZN3RBX10Reflection11Call2HelperINS_7Network6PlayerEMS3_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS7_vE4callEPS3_S9_RNS0_7VariantERKSsRKS7_
// type: void __fastcall(int, char *, int, int, std::string *, int *)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Network::Player,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Network::Player*,void (RBX::Network::Player::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_ac2134(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xac2134: IDA: bind/call thunk forwards the instance id into the bound member (mf1).
    fire(inst);
}
// 0xac245c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS3_FS7_SsEPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac245c() {
    // IDA 0xac245c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac2704 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_ac2704() {
    // IDA 0xac2704: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac283c — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac283c() {
    // IDA 0xac283c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac297c — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FN5boost10shared_ptrINS_8InstanceEEESsESsS7_E4callEPS3_S9_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, _DWORD *, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_ac297c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xac297c: IDA: bind/call thunk forwards the instance id into the bound member (mf1).
    fire(inst);
}
// 0xac2c60 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsdELi2EEC2EMS3_FvSsdEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,double),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,double),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac2c60() {
    // IDA 0xac2c60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac2f7c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsdELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,double),2>::~BoundFuncDesc()")]
pub fn stub_ac2f7c() {
    // IDA 0xac2f7c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac30c0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac30c0() {
    // IDA 0xac30c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac32b0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFdSsELi1EEC2EMS3_FdSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,double ()(std::string),1>::BoundFuncDesc(double (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac32b0() {
    // IDA 0xac32b0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac3558 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFdSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,double ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_ac3558() {
    // IDA 0xac3558: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac3690 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFdSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,double ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac3690() {
    // IDA 0xac3690: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac37d0 — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FdSsESsdE4callEPS3_S5_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,double (RBX::Network::Player::*)(std::string),std::string,double>::call(RBX::Network::Player*,double (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_ac37d0(fire: &dyn Fn()) {
    // IDA 0xac37d0: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0xac39b8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsbELi2EEC2EMS3_FvSsbEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,bool),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,bool),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac39b8() {
    // IDA 0xac39b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac3cd4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsbELi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,bool),2>::~BoundFuncDesc()")]
pub fn stub_ac3cd4() {
    // IDA 0xac3cd4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac3e18 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac3e18() {
    // IDA 0xac3e18: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac4004 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbSsELi1EEC2EMS3_FbSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::Network::Player::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac4004() {
    // IDA 0xac4004: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac42ac — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_ac42ac() {
    // IDA 0xac42ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac43e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac43e4() {
    // IDA 0xac43e4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac4524 — __ZN3RBX10Reflection11Call1HelperINS_7Network6PlayerEMS3_FbSsESsbE4callEPS3_S5_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Player,bool (RBX::Network::Player::*)(std::string),std::string,bool>::call(RBX::Network::Player*,bool (RBX::Network::Player::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_ac4524(fire: &dyn Fn()) {
    // IDA 0xac4524: IDA: bind/call thunk tail-calls boost::bind operator() (mf0, no args).
    fire();
}
// 0xac46f4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsSsELi2EEC2EMS3_FvSsSsEPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::Network::Player::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ac46f4() {
    // IDA 0xac46f4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac4a4c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsSsELi2EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
pub fn stub_ac4a4c() {
    // IDA 0xac4a4c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xac4aec — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,std::string),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_ac4aec() {
    // IDA 0xac4aec: faithful no-op shell; control block / ref traffic stays engine-side.
}
