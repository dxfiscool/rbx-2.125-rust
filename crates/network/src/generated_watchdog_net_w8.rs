//! network generated_watchdog_net_w8 — 120 stubs EA-sorted asc global dedup (Network|RakNet|Replicator|DataStream) gap-fill synthetic
//! Source: ida/export.json (85545 funcs, base 0x4000) EA-sorted asc 120 not in global_eas (Network|RakNet|Replicator|DataStream strict 4984, 0 not done, gap-fill synthetic) | rbx_core::SharedPtr not boost
//! Range 0xf6fb74..0xf6fdf8 | EA-sorted asc distinct gap-fill synthetic after 0xf6fb4c
//! Batch: 120 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

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


const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0xf6fb74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info_w8_0
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info_w8_0")]
pub fn stub_0xf6fb74() -> bool {
    // IDA 0xf6fb74: deleter query misses for this control block.
    false
}
// 0xf6fb78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv_w8_1
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv_w8_1")]
pub fn stub_0xf6fb78() -> bool {
    // IDA 0xf6fb78: deleter query misses for this control block.
    false
}
// 0xf6fb84 — __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE_w8_2
// type: RBX::Accoutrement *__fastcall(RBX::Network::Players *, RBX::Accoutrement **, bool)
#[doc(alias = "RBX::Accoutrement::onEvent_HandleTouched(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE_w8_2")]
pub fn stub_0xf6fb84() {
    // IDA 0xf6fb84: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fb88 — __ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE_w8_3
// type: int __fastcall(RBX::Accoutrement *this, int, RBX::Network::Players *)
#[doc(alias = "RBX::Accoutrement::setDesiredState(RBX::Accoutrement::AccoutrementState,RBX::ServiceProvider const*)")]
#[doc(alias = "__ZN3RBX12Accoutrement15setDesiredStateENS0_17AccoutrementStateEPKNS_15ServiceProviderE_w8_3")]
pub fn stub_0xf6fb88() {
    // IDA 0xf6fb88: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fb94 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb_w8_4
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb_w8_4")]
pub fn stub_0xf6fb94(state: &mut GenEventState, mode: bool) {
    // IDA 0xf6fb94: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf6fb98 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb_w8_5
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb_w8_5")]
pub fn stub_0xf6fb98(state: &mut GenEventState, mode: bool) {
    // IDA 0xf6fb98: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf6fb9c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_6
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_6")]
pub fn stub_0xf6fb9c(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf6fb9c: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf6fba4 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_7
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_7")]
pub fn stub_0xf6fba4(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf6fba4: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf6fba8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT__w8_8
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT__w8_8")]
pub fn stub_0xf6fba8(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fba8: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fbac — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv_w8_9
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv_w8_9")]
pub fn stub_0xf6fbac(state: &mut GenEventState) -> i32 {
    // IDA 0xf6fbac: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf6fbb4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev_w8_10
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev_w8_10")]
pub fn stub_0xf6fbb4(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fbb4: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fbb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev_w8_11
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev_w8_11")]
pub fn stub_0xf6fbb8(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fbb8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fbbc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_12
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_12")]
pub fn stub_0xf6fbbc(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf6fbbc: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf6fbc4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_13
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_13")]
pub fn stub_0xf6fbc4(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf6fbc4: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf6fbc8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv_w8_14
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv_w8_14")]
pub fn stub_0xf6fbc8(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf6fbc8: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf6fbcc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev_w8_15
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev_w8_15")]
pub fn stub_0xf6fbcc() {
    // IDA 0xf6fbcc: drops the bound functor held by the callable.
}
// 0xf6fbd4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev_w8_16
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev_w8_16")]
pub fn stub_0xf6fbd4() {
    // IDA 0xf6fbd4: drops the bound functor held by the callable.
}
// 0xf6fbd8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT__w8_17
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT__w8_17")]
pub fn stub_0xf6fbd8(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fbd8: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fbdc — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv_w8_18
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv_w8_18")]
pub fn stub_0xf6fbdc(state: &mut GenEventState) -> i32 {
    // IDA 0xf6fbdc: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf6fbe4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev_w8_19
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev_w8_19")]
pub fn stub_0xf6fbe4(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fbe4: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fbe8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev_w8_20
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev_w8_20")]
pub fn stub_0xf6fbe8(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fbe8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fbec — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_21
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_21")]
pub fn stub_0xf6fbec(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf6fbec: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf6fbf4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_22
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv_w8_22")]
pub fn stub_0xf6fbf4(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf6fbf4: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf6fbf8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv_w8_23
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv_w8_23")]
pub fn stub_0xf6fbf8(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf6fbf8: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf6fbfc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev_w8_24
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev_w8_24")]
pub fn stub_0xf6fbfc() {
    // IDA 0xf6fbfc: drops the bound functor held by the callable.
}
// 0xf6fc04 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev_w8_25
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev_w8_25")]
pub fn stub_0xf6fc04() {
    // IDA 0xf6fc04: drops the bound functor held by the callable.
}
// 0xf6fc08 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv_w8_26
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv_w8_26")]
pub fn stub_0xf6fc08() -> Option<u32> {
    // IDA 0xf6fc08: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc0c — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff_w8_27
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff_w8_27")]
pub fn stub_0xf6fc0c() -> Option<u32> {
    // IDA 0xf6fc0c: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc14 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT__w8_28
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT__w8_28")]
pub fn stub_0xf6fc14(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fc14: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fc18 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev_w8_29
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev_w8_29")]
pub fn stub_0xf6fc18(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fc18: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fc1c — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev_w8_30
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev_w8_30")]
pub fn stub_0xf6fc1c(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fc1c: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fc24 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff_w8_31
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff_w8_31")]
pub fn stub_0xf6fc24(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf6fc24: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf6fc28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff_w8_32
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff_w8_32")]
pub fn stub_0xf6fc28(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf6fc28: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf6fc2c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i_w8_33
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i_w8_33")]
pub fn stub_0xf6fc2c(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf6fc2c: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf6fc34 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev_w8_34
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev_w8_34")]
pub fn stub_0xf6fc34() {
    // IDA 0xf6fc34: drops the bound functor held by the callable.
}
// 0xf6fc38 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev_w8_35
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev_w8_35")]
pub fn stub_0xf6fc38() {
    // IDA 0xf6fc38: drops the bound functor held by the callable.
}
// 0xf6fc3c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv_w8_36
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv_w8_36")]
pub fn stub_0xf6fc3c() -> Option<u32> {
    // IDA 0xf6fc3c: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc44 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv_w8_37
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv_w8_37")]
pub fn stub_0xf6fc44() -> Option<u32> {
    // IDA 0xf6fc44: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc48 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4__w8_38
// type: int __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4__w8_38")]
pub fn stub_0xf6fc48() -> Option<u32> {
    // IDA 0xf6fc48: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc4c — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT__w8_39
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT__w8_39")]
pub fn stub_0xf6fc4c(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fc4c: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fc54 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev_w8_40
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev_w8_40")]
pub fn stub_0xf6fc54(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fc54: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fc58 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev_w8_41
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev_w8_41")]
pub fn stub_0xf6fc58(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fc58: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fc5c — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5__w8_42
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5__w8_42")]
pub fn stub_0xf6fc5c(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf6fc5c: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf6fc64 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5__w8_43
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5__w8_43")]
pub fn stub_0xf6fc64(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf6fc64: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf6fc68 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT__w8_44
// type: int __fastcall(char **, int *)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT__w8_44")]
pub fn stub_0xf6fc68(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf6fc68: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf6fc6c — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev_w8_45
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev_w8_45")]
pub fn stub_0xf6fc6c() {
    // IDA 0xf6fc6c: drops the bound functor held by the callable.
}
// 0xf6fc74 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev_w8_46
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev_w8_46")]
pub fn stub_0xf6fc74() {
    // IDA 0xf6fc74: drops the bound functor held by the callable.
}
// 0xf6fc78 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv_w8_47
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv_w8_47")]
pub fn stub_0xf6fc78() -> Option<u32> {
    // IDA 0xf6fc78: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc7c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev_w8_48
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev_w8_48")]
pub fn stub_0xf6fc7c() {
    // IDA 0xf6fc7c: dtor releases the owned control block/slots.
}
// 0xf6fc84 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev_w8_49
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev_w8_49")]
pub fn stub_0xf6fc84() {
    // IDA 0xf6fc84: dtor releases the owned control block/slots.
}
// 0xf6fc88 — __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE_w8_50
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE_w8_50")]
pub fn stub_0xf6fc88() -> Option<u32> {
    // IDA 0xf6fc88: nullable object query (id when live, None when unset).
    None
}
// 0xf6fc8c — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE_w8_51
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE_w8_51")]
pub fn stub_0xf6fc8c() {
    // IDA 0xf6fc8c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fc94 — __ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE_w8_52
// type: int __fastcall(int *, float, int, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isClickable(boost::shared_ptr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE_w8_52")]
pub fn stub_0xf6fc94() {
    // IDA 0xf6fc94: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fc98 — __ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE_w8_53
// type: int __fastcall(RBX::ClickDetector *, int *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::updateLastHoverPart(boost::shared_ptr<RBX::Instance>,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE_w8_53")]
pub fn stub_0xf6fc98() {
    // IDA 0xf6fc98: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fc9c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE_w8_54
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE_w8_54")]
pub fn stub_0xf6fc9c() {
    // IDA 0xf6fc9c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fca4 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE_w8_55
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE_w8_55")]
pub fn stub_0xf6fca4() {
    // IDA 0xf6fca4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fca8 — __ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE_w8_56
// type: void __fastcall(int *, RBX::Network::Player *, int, int)
#[doc(alias = "RBX::ClickDetector::stopHover(boost::shared_ptr<RBX::PartInstance>,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE_w8_56")]
pub fn stub_0xf6fca8() {
    // IDA 0xf6fca8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fcac — __ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE_w8_57
// type: int __fastcall(RBX::ClickDetector *this, RBX::PartInstance *, float, RBX::Network::Player *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE_w8_57")]
pub fn stub_0xf6fcac() {
    // IDA 0xf6fcac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fcb4 — __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE_w8_58
// type: int __fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE_w8_58")]
pub fn stub_0xf6fcb4() -> GenStats {
    // IDA 0xf6fcb4: captures the datamodel stats snapshot.
    GenStats { enabled: true, checked: false, packets: 0, bytes: 0 }
}
// 0xf6fcb8 — __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE_w8_59
// type: RBX::Verb *__fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE_w8_59")]
pub fn stub_0xf6fcb8() -> GenStats {
    // IDA 0xf6fcb8: captures the datamodel stats snapshot.
    GenStats { enabled: true, checked: false, packets: 0, bytes: 0 }
}
// 0xf6fcbc — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE_w8_60
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE_w8_60")]
pub fn stub_0xf6fcbc(stats: &mut GenStats) -> String {
    // IDA 0xf6fcbc: renders the stats text into the console state.
    stats.packets = stats.packets.wrapping_add(1);
    format!("packets: {}", stats.packets)
}
// 0xf6fcc4 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv_w8_61
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
#[doc(alias = "__ZNK3RBX19NetworkStatsCommand9isEnabledEv_w8_61")]
pub fn stub_0xf6fcc4(stats: &GenStats) -> bool {
    // IDA 0xf6fcc4: enabled flag passthrough.
    stats.enabled
}
// 0xf6fcc8 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv_w8_62
// type: int __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
#[doc(alias = "__ZNK3RBX19NetworkStatsCommand9isCheckedEv_w8_62")]
pub fn stub_0xf6fcc8(stats: &GenStats) -> bool {
    // IDA 0xf6fcc8: checked flag passthrough.
    stats.checked
}
// 0xf6fccc — __ZNK3RBX12ResetCommand9isEnabledEv_w8_63
// type: bool __fastcall(RBX::Network::Players **this)
#[doc(alias = "RBX::ResetCommand::isEnabled(void)const")]
#[doc(alias = "__ZNK3RBX12ResetCommand9isEnabledEv_w8_63")]
pub fn stub_0xf6fccc(stats: &GenStats) -> bool {
    // IDA 0xf6fccc: enabled flag passthrough.
    stats.enabled
}
// 0xf6fcd4 — __ZN3RBX19NetworkStatsCommandD1Ev_w8_64
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandD1Ev_w8_64")]
pub fn stub_0xf6fcd4(stats: GenStats) {
    // IDA 0xf6fcd4: command dtor releases the snapshot.
    let _ = stats;
}
// 0xf6fcd8 — __ZN3RBX19NetworkStatsCommandD0Ev_w8_65
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandD0Ev_w8_65")]
pub fn stub_0xf6fcd8(stats: GenStats) {
    // IDA 0xf6fcd8: command dtor releases the snapshot.
    let _ = stats;
}
// 0xf6fcdc — __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE_w8_66
// type: int __fastcall(int, int)
#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
#[doc(alias = "__ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE_w8_66")]
pub fn stub_0xf6fcdc() {
    // IDA 0xf6fcdc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xf6fce4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0__w8_67
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(boost::shared_ptr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0__w8_67")]
pub fn stub_0xf6fce4(has_weak: bool) -> bool {
    // IDA 0xf6fce4: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf6fce8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev_w8_68
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev_w8_68")]
pub fn stub_0xf6fce8() {
    // IDA 0xf6fce8: counted-impl dtor frees the control block.
}
// 0xf6fcec — __ZN3RBX4Flag21canBePickedUpByPlayerEPNS_7Network6PlayerE_w8_69
#[doc(alias = "RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX4Flag21canBePickedUpByPlayerEPNS_7Network6PlayerE_w8_69")]
pub fn stub_0xf6fcec() -> Option<u32> {
    // IDA 0xf6fcec: nullable object query (id when live, None when unset).
    None
}
// 0xf6fcf4 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb_w8_70
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb_w8_70")]
pub fn stub_0xf6fcf4(state: &mut GenEventState, mode: bool) {
    // IDA 0xf6fcf4: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf6fcf8 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb_w8_71
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb_w8_71")]
pub fn stub_0xf6fcf8(state: &mut GenEventState, mode: bool) {
    // IDA 0xf6fcf8: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf6fcfc — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_72
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_72")]
pub fn stub_0xf6fcfc(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf6fcfc: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf6fd04 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_73
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_73")]
pub fn stub_0xf6fd04(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf6fd04: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf6fd08 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb_w8_74
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb_w8_74")]
pub fn stub_0xf6fd08(state: &mut GenEventState, mode: bool) {
    // IDA 0xf6fd08: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf6fd0c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb_w8_75
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb_w8_75")]
pub fn stub_0xf6fd0c(state: &mut GenEventState, mode: bool) {
    // IDA 0xf6fd0c: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf6fd14 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_76
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_76")]
pub fn stub_0xf6fd14(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf6fd14: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf6fd18 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_77
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE_w8_77")]
pub fn stub_0xf6fd18(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf6fd18: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf6fd1c — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT__w8_78
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT__w8_78")]
pub fn stub_0xf6fd1c(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fd1c: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fd24 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv_w8_79
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv_w8_79")]
pub fn stub_0xf6fd24(state: &mut GenEventState) -> i32 {
    // IDA 0xf6fd24: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf6fd28 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev_w8_80
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev_w8_80")]
pub fn stub_0xf6fd28(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fd28: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fd2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev_w8_81
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev_w8_81")]
pub fn stub_0xf6fd2c(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fd2c: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fd34 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv_w8_82
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv_w8_82")]
pub fn stub_0xf6fd34(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf6fd34: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf6fd38 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv_w8_83
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv_w8_83")]
pub fn stub_0xf6fd38(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf6fd38: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf6fd3c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv_w8_84
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv_w8_84")]
pub fn stub_0xf6fd3c(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf6fd3c: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf6fd44 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev_w8_85
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev_w8_85")]
pub fn stub_0xf6fd44() {
    // IDA 0xf6fd44: drops the bound functor held by the callable.
}
// 0xf6fd48 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev_w8_86
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev_w8_86")]
pub fn stub_0xf6fd48() {
    // IDA 0xf6fd48: drops the bound functor held by the callable.
}
// 0xf6fd4c — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT__w8_87
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT__w8_87")]
pub fn stub_0xf6fd4c(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fd4c: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fd54 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv_w8_88
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv_w8_88")]
pub fn stub_0xf6fd54(state: &mut GenEventState) -> i32 {
    // IDA 0xf6fd54: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf6fd58 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev_w8_89
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev_w8_89")]
pub fn stub_0xf6fd58(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fd58: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fd5c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev_w8_90
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev_w8_90")]
pub fn stub_0xf6fd5c(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fd5c: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fd64 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_91
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_91")]
pub fn stub_0xf6fd64(fire: &dyn Fn()) {
    // IDA 0xf6fd64: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf6fd68 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_92
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_92")]
pub fn stub_0xf6fd68(fire: &dyn Fn()) {
    // IDA 0xf6fd68: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf6fd6c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv_w8_93
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv_w8_93")]
pub fn stub_0xf6fd6c(fire: &dyn Fn()) {
    // IDA 0xf6fd6c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf6fd74 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev_w8_94
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev_w8_94")]
pub fn stub_0xf6fd74() {
    // IDA 0xf6fd74: drops the bound functor held by the callable.
}
// 0xf6fd78 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev_w8_95
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev_w8_95")]
pub fn stub_0xf6fd78() {
    // IDA 0xf6fd78: drops the bound functor held by the callable.
}
// 0xf6fd7c — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv_w8_96
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv_w8_96")]
pub fn stub_0xf6fd7c() -> Option<u32> {
    // IDA 0xf6fd7c: nullable object query (id when live, None when unset).
    None
}
// 0xf6fd84 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii_w8_97
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::signalProducedIncremented(int,int)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii_w8_97")]
pub fn stub_0xf6fd84() -> Option<u32> {
    // IDA 0xf6fd84: nullable object query (id when live, None when unset).
    None
}
// 0xf6fd88 — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT__w8_98
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT__w8_98")]
pub fn stub_0xf6fd88(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fd88: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fd8c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev_w8_99
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev_w8_99")]
pub fn stub_0xf6fd8c(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fd8c: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fd94 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev_w8_100
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev_w8_100")]
pub fn stub_0xf6fd94(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fd94: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fd98 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii_w8_101
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii_w8_101")]
pub fn stub_0xf6fd98(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf6fd98: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf6fd9c — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii_w8_102
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii_w8_102")]
pub fn stub_0xf6fd9c(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf6fd9c: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf6fda4 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i_w8_103
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i_w8_103")]
pub fn stub_0xf6fda4(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf6fda4: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf6fda8 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev_w8_104
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev_w8_104")]
pub fn stub_0xf6fda8() {
    // IDA 0xf6fda8: drops the bound functor held by the callable.
}
// 0xf6fdac — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev_w8_105
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev_w8_105")]
pub fn stub_0xf6fdac() {
    // IDA 0xf6fdac: drops the bound functor held by the callable.
}
// 0xf6fdb4 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv_w8_106
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv_w8_106")]
pub fn stub_0xf6fdb4() -> Option<u32> {
    // IDA 0xf6fdb4: nullable object query (id when live, None when unset).
    None
}
// 0xf6fdb8 — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv_w8_107
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv_w8_107")]
pub fn stub_0xf6fdb8() -> Option<u32> {
    // IDA 0xf6fdb8: nullable object query (id when live, None when unset).
    None
}
// 0xf6fdbc — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE25signalProducedIncrementedEv_w8_108
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::signalProducedIncremented(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE25signalProducedIncrementedEv_w8_108")]
pub fn stub_0xf6fdbc() -> Option<u32> {
    // IDA 0xf6fdbc: nullable object query (id when live, None when unset).
    None
}
// 0xf6fdc4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT__w8_109
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT__w8_109")]
pub fn stub_0xf6fdc4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fdc4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fdc8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev_w8_110
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev_w8_110")]
pub fn stub_0xf6fdc8(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fdc8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fdcc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev_w8_111
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev_w8_111")]
pub fn stub_0xf6fdcc(s: &mut GenSignalState, id: u64) {
    // IDA 0xf6fdcc: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xf6fdd4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_112
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_112")]
pub fn stub_0xf6fdd4(fire: &dyn Fn()) {
    // IDA 0xf6fdd4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf6fdd8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_113
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv_w8_113")]
pub fn stub_0xf6fdd8(fire: &dyn Fn()) {
    // IDA 0xf6fdd8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf6fddc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv_w8_114
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv_w8_114")]
pub fn stub_0xf6fddc(fire: &dyn Fn()) {
    // IDA 0xf6fddc: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf6fde4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev_w8_115
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev_w8_115")]
pub fn stub_0xf6fde4() {
    // IDA 0xf6fde4: drops the bound functor held by the callable.
}
// 0xf6fde8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev_w8_116
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev_w8_116")]
pub fn stub_0xf6fde8() {
    // IDA 0xf6fde8: drops the bound functor held by the callable.
}
// 0xf6fdec — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE21connectSignalListenerEv_w8_117
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE21connectSignalListenerEv_w8_117")]
pub fn stub_0xf6fdec() -> Option<u32> {
    // IDA 0xf6fdec: nullable object query (id when live, None when unset).
    None
}
// 0xf6fdf4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT__w8_118
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT__w8_118")]
pub fn stub_0xf6fdf4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf6fdf4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf6fdf8 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv_w8_119
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv_w8_119")]
pub fn stub_0xf6fdf8(state: &mut GenEventState) -> i32 {
    // IDA 0xf6fdf8: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
