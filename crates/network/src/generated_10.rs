//! network generated_10 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 100 stubs here, 3959 combined -> 4059 total, 738 remaining).
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


// 0xf20320 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim")]
pub fn stub_f20320() -> Option<u32> {
    // IDA 0xf20320: nullable object query (id when live, None when unset).
    None
}
// 0xf22078 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb$shim")]
pub fn stub_f22078(state: &mut GenEventState, mode: bool) {
    // IDA 0xf22078: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf22084 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim")]
pub fn stub_f22084(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf22084: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf22090 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb$shim")]
pub fn stub_f22090(state: &mut GenEventState, mode: bool) {
    // IDA 0xf22090: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf2209c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim")]
pub fn stub_f2209c(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf2209c: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf220f0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_f220f0() -> Option<u32> {
    // IDA 0xf220f0: nullable object query (id when live, None when unset).
    None
}
// 0xf220fc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_f220fc() -> Option<u32> {
    // IDA 0xf220fc: nullable object query (id when live, None when unset).
    None
}
// 0xf2212c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_f2212c() -> Option<u32> {
    // IDA 0xf2212c: nullable object query (id when live, None when unset).
    None
}
// 0xf22180 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")]
pub fn stub_f22180() -> Option<u32> {
    // IDA 0xf22180: nullable object query (id when live, None when unset).
    None
}
// 0xf2218c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_f2218c() -> Option<u32> {
    // IDA 0xf2218c: nullable object query (id when live, None when unset).
    None
}
// 0xf2248c — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb$shim")]
pub fn stub_f2248c(state: &mut GenEventState, mode: bool) {
    // IDA 0xf2248c: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf22498 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim")]
pub fn stub_f22498(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf22498: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf224ec — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")]
pub fn stub_f224ec() -> Option<u32> {
    // IDA 0xf224ec: nullable object query (id when live, None when unset).
    None
}
// 0xf224f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim
// type: int()
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")]
pub fn stub_f224f8() -> Option<u32> {
    // IDA 0xf224f8: nullable object query (id when live, None when unset).
    None
}
// 0xf31c34 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
pub fn stub_f31c34(state: &mut GenEventState, mode: bool) {
    // IDA 0xf31c34: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf31c44 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f31c44(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf31c44: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf31c54 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
pub fn stub_f31c54(state: &mut GenEventState) -> i32 {
    // IDA 0xf31c54: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf31c64 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
pub fn stub_f31c64() {
    // IDA 0xf31c64: dtor releases the owned control block/slots.
}
// 0xf31c74 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
pub fn stub_f31c74(state: &mut GenEventState, mode: bool) {
    // IDA 0xf31c74: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf31c84 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f31c84(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf31c84: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf31c94 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
pub fn stub_f31c94(state: &mut GenEventState) -> i32 {
    // IDA 0xf31c94: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf31ca4 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
pub fn stub_f31ca4() {
    // IDA 0xf31ca4: dtor releases the owned control block/slots.
}
// 0xf31dd4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
pub fn stub_f31dd4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf31dd4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf31e64 — j___ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_f31e64(s: &mut GenSignalState) -> u64 {
    // IDA 0xf31e64: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf31e94 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
pub fn stub_f31e94(s: &mut GenSignalState) -> u64 {
    // IDA 0xf31e94: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf31ea4 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
pub fn stub_f31ea4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf31ea4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf31f54 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_f31f54(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf31f54: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf31f64 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
pub fn stub_f31f64(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf31f64: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf31f74 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
pub fn stub_f31f74(fire: &dyn Fn(u32, f32, f32), axis: u32, x: f32, y: f32) {
    // IDA 0xf31f74: bind/call thunk forwards axis + two floats (mf3).
    fire(axis, x, y);
}
// 0xf31f94 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
pub fn stub_f31f94(fire: &dyn Fn(u32), axis: u32) {
    // IDA 0xf31f94: bind/call thunk forwards the axis (mf1).
    fire(axis);
}
// 0xf32aa4 — j___ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
pub fn stub_f32aa4() -> Option<u32> {
    // IDA 0xf32aa4: nullable object query (id when live, None when unset).
    None
}
// 0xf385e4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(rbx_core::SharedPtr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
pub fn stub_f385e4(has_weak: bool) -> bool {
    // IDA 0xf385e4: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xf3fb84 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
pub fn stub_f3fb84(state: &mut GenEventState, mode: bool) {
    // IDA 0xf3fb84: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf3fb94 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f3fb94(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf3fb94: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf3fba4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
pub fn stub_f3fba4(state: &mut GenEventState) -> i32 {
    // IDA 0xf3fba4: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf3fbb4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")]
pub fn stub_f3fbb4() {
    // IDA 0xf3fbb4: dtor releases the owned control block/slots.
}
// 0xf3fbc4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
pub fn stub_f3fbc4(state: &mut GenEventState, mode: bool) {
    // IDA 0xf3fbc4: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf3fbd4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f3fbd4(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf3fbd4: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf3fbe4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
pub fn stub_f3fbe4(state: &mut GenEventState) -> i32 {
    // IDA 0xf3fbe4: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf3fbf4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")]
pub fn stub_f3fbf4() {
    // IDA 0xf3fbf4: dtor releases the owned control block/slots.
}
// 0xf3fc04 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
pub fn stub_f3fc04(state: &mut GenEventState, mode: bool) {
    // IDA 0xf3fc04: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf3fc14 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f3fc14(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf3fc14: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf3fc24 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
pub fn stub_f3fc24(state: &mut GenEventState) -> i32 {
    // IDA 0xf3fc24: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf3fc34 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
pub fn stub_f3fc34(state: &mut GenEventState, mode: bool) {
    // IDA 0xf3fc34: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf3fc44 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f3fc44(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf3fc44: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf3fc54 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")]
pub fn stub_f3fc54(state: &mut GenEventState) -> i32 {
    // IDA 0xf3fc54: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf3fd64 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
pub fn stub_f3fd64(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fd64: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fdd4 — j___ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_f3fdd4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fdd4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fde4 — j___ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_f3fde4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fde4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fe14 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
pub fn stub_f3fe14(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fe14: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fe24 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
pub fn stub_f3fe24(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fe24: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fe34 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
pub fn stub_f3fe34(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fe34: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fe44 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
pub fn stub_f3fe44(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fe44: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3fe54 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")]
pub fn stub_f3fe54(s: &mut GenSignalState) -> u64 {
    // IDA 0xf3fe54: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf3ff64 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
pub fn stub_f3ff64(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0xf3ff64: bind/call thunk forwards UDim2 (mf1).
    fire(v);
}
// 0xf3ff94 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
pub fn stub_f3ff94(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf3ff94: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf3ffa4 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
pub fn stub_f3ffa4(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf3ffa4: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf3ffb4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
pub fn stub_f3ffb4(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf3ffb4: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf3ffc4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
pub fn stub_f3ffc4(fire: &dyn Fn()) {
    // IDA 0xf3ffc4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf3ffd4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
pub fn stub_f3ffd4(fire: &dyn Fn([f32; 4]), v: [f32; 4]) {
    // IDA 0xf3ffd4: bind/call thunk forwards UDim2 (mf1).
    fire(v);
}
// 0xf3ffe4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")]
pub fn stub_f3ffe4(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xf3ffe4: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xf3fff4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int()
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
pub fn stub_f3fff4(fire: &dyn Fn()) {
    // IDA 0xf3fff4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xf41ad4 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")]
pub fn stub_f41ad4(state: &mut GenEventState, mode: bool) {
    // IDA 0xf41ad4: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf41ae4 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f41ae4(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf41ae4: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf41af4 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::listenerConnectionAdded(void)")]
pub fn stub_f41af4(state: &mut GenEventState) -> i32 {
    // IDA 0xf41af4: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf41b04 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::~EventReplicatorBase()")]
pub fn stub_f41b04() {
    // IDA 0xf41b04: dtor releases the owned control block/slots.
}
// 0xf41b14 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")]
pub fn stub_f41b14(state: &mut GenEventState, mode: bool) {
    // IDA 0xf41b14: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf41b24 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_f41b24(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf41b24: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf41b34 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")]
pub fn stub_f41b34(state: &mut GenEventState) -> i32 {
    // IDA 0xf41b34: reads the listener count, keeps a listener while count>=1, returns the count.
    let n = state.count;
    state.listener = n >= 1;
    n
}
// 0xf41b44 — j___ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::~EventReplicatorBase()")]
pub fn stub_f41b44() {
    // IDA 0xf41b44: dtor releases the owned control block/slots.
}
// 0xf41c44 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>> const&)")]
pub fn stub_f41c44(s: &mut GenSignalState) -> u64 {
    // IDA 0xf41c44: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf41cd4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_f41cd4(s: &mut GenSignalState) -> u64 {
    // IDA 0xf41cd4: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf41d04 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
pub fn stub_f41d04(s: &mut GenSignalState) -> u64 {
    // IDA 0xf41d04: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf41d14 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
pub fn stub_f41d14(s: &mut GenSignalState) -> u64 {
    // IDA 0xf41d14: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
// 0xf41d94 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i
// type: int()
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list2<RBX::NormalId&,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,boost::_bi::list2<RBX::NormalId&,float &> &,int)")]
pub fn stub_f41d94(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0xf41d94: bind/call thunk forwards (NormalId, float) (mf2).
    fire(id, x);
}
// 0xf41da4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int()
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
pub fn stub_f41da4(fire: &dyn Fn(u32), id: u32) {
    // IDA 0xf41da4: bind/call thunk forwards NormalId (mf1).
    fire(id);
}
// 0xf41db4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int()
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
pub fn stub_f41db4(fire: &dyn Fn(u32, f32), id: u32, x: f32) {
    // IDA 0xf41db4: bind/call thunk forwards (NormalId, float) (mf2).
    fire(id, x);
}
// 0xf41dd4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int()
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
pub fn stub_f41dd4(fire: &dyn Fn(u32), id: u32) {
    // IDA 0xf41dd4: bind/call thunk forwards NormalId (mf1).
    fire(id);
}
// 0xf450a4 — j___ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
// type: int __fastcall(RBX::Network::NetworkOwner *this)
#[doc(alias = "RBX::Network::NetworkOwner::ServerUnassigned(void)")]
pub fn stub_f450a4() -> GenOwner {
    // IDA 0xf450a4: guard-once unassigned sentinel.
    GenOwner { ip: 0, port: 0, server: false }
}
// 0xf450b4 — j___ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
// type: int()
#[doc(alias = "RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")]
pub fn stub_f450b4(addr: &GenOwner) -> u32 {
    // IDA 0xf450b4: hashes the address to a debug color.
    addr.ip ^ ((addr.port as u32) << 16)
}
// 0xf450c4 — j___ZN3RBX7Network12NetworkOwner6ServerEv
// type: int __fastcall(RBX::Network::NetworkOwner *this)
#[doc(alias = "RBX::Network::NetworkOwner::Server(void)")]
pub fn stub_f450c4() -> GenOwner {
    // IDA 0xf450c4: well-known server owner address.
    GenOwner { ip: 0, port: 0, server: true }
}
// 0xf450d4 — j___ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
// type: int()
#[doc(alias = "RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")]
pub fn stub_f450d4(addr: &GenOwner) -> bool {
    // IDA 0xf450d4: client when the owner is not the server.
    !addr.server
}
// 0xf48a24 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::~EventReplicatorBase()")]
pub fn stub_f48a24() {
    // IDA 0xf48a24: dtor releases the owned control block/slots.
}
// 0xf48a34 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::~EventReplicatorBase()")]
pub fn stub_f48a34() {
    // IDA 0xf48a34: dtor releases the owned control block/slots.
}
// 0xf49344 — j___ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_f49344(weak: Option<u32>) -> Option<u32> {
    // IDA 0xf49344: nothrow lock of the weak ptr (expired -> None).
    weak
}
// 0xf49374 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
pub fn stub_f49374(slot: &mut GenFunctor) {
    // IDA 0xf49374: packs the bound argument list.
    slot.has = true;
}
// 0xf49384 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
pub fn stub_f49384(fire: &dyn Fn(Option<u32>), player: Option<u32>) {
    // IDA 0xf49384: bind/call thunk forwards the locked weak player (mf1).
    fire(player);
}
// 0xf493c4 — j___ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)")]
pub fn stub_f493c4() -> Option<u32> {
    // IDA 0xf493c4: nullable object query (id when live, None when unset).
    None
}
// 0xf49414 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_f49414(slot: &mut GenFunctor, op: u32) {
    // IDA 0xf49414: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xf49444 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)")]
pub fn stub_f49444(slot: &mut GenFunctor) -> bool {
    // IDA 0xf49444: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf494e4 — j___ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const")]
pub fn stub_f494e4() -> Option<u32> {
    // IDA 0xf494e4: nullable object query (id when live, None when unset).
    None
}
// 0xf494f4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_f494f4(slot: &mut GenFunctor) {
    // IDA 0xf494f4: installs the functor with the nothrow tag.
    slot.has = true;
}
// 0xf49504 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_f49504(slot: &mut GenFunctor) -> bool {
    // IDA 0xf49504: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf49514 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f49514(slot: &mut GenFunctor) -> bool {
    // IDA 0xf49514: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xf4bb44 — j___ZN3RBX7Network7Players11getGameModeEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getGameMode(RBX::Instance const*)")]
pub fn stub_f4bb44() -> Option<u32> {
    // IDA 0xf4bb44: nullable object query (id when live, None when unset).
    None
}
// 0xf52114 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_f52114() -> u64 {
    // IDA 0xf52114: returns the static signal mutex id.
    0
}
// 0xf52124 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv
// type: int(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_f52124() -> u64 {
    // IDA 0xf52124: returns the static signal mutex id.
    0
}
// 0xf52134 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
pub fn stub_f52134(s: &mut GenSignalState) -> u64 {
    // IDA 0xf52134: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0xf52144 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
pub fn stub_f52144(s: &mut GenSignalState, id: u64) {
    // IDA 0xf52144: unlinks one slot node (missing node is a no-op).
    gen_disconnect(s, id);
}
// 0xf52154 — j___ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
pub fn stub_f52154(s: &mut GenSignalState) -> u64 {
    // IDA 0xf52154: intrusive slot alloc + mutex insert; returns the connection id.
    gen_connect(s)
}
