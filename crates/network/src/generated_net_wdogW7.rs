//! network generated_net_wdogW7 — auto-generated, do not edit manually
//! Filter: RakNet|RakPeer|Network|Replicator|DataStream (5296 matched, 10 remaining) -> 120 EA-sorted asc (remaining + fill to reach 120)
//! Source: ida/export.json (85545 funcs, base 0x4000) EA-sorted asc
//! Range 0x17e68..0xf68774 | rbx_core::SharedPtr (not boost)
//! Batch: 120 stubs | // 0xADDR — mangled + #[doc(alias)] + pub fn stub_ADDR() { todo!("0xADDR") }

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]


/// `RBX::ClickDetector` hover state (IDA 0x3f12e0: last-hover shared_ptr at +0x7C).
#[derive(Clone, Debug, Default)]
pub struct ClickDetector {
 pub hover_part: Option<usize>,
 pub max_distance: f32,
 pub enabled: bool,
}

/// `RBX::NetworkStatsCommand` verb state (IDA 0x3f7df4 et al.).
#[derive(Clone, Debug, Default)]
pub struct NetworkStatsCommand {
 pub enabled: bool,
 pub checked: bool,
}

/// `RBX::DataModel` network-metric slot (IDA 0x427db8: field at +0xBB8).
#[derive(Clone, Debug, Default)]
pub struct DataModelMetric {
 pub this: usize,
 pub metric: Option<usize>,
}

/// `RBX::DataModel` physics-instruction fields (IDA 0x425d58: SimSendFilter words + 0xC00/0xC04/0xC08).
#[derive(Clone, Debug, Default)]
pub struct PhysicsSimFilter {
 pub sim_address: u32,
 pub filter: u32,
 pub mode_words: [u32; 2],
 pub flag: bool,
}

/// `EventReplicatorBase` listener side (IDA 0x3a7f68 et al.).
#[derive(Clone, Debug, Default)]
pub struct EventReplicator {
    pub listener_mode: bool,
    pub connected: bool,
    pub watched: bool,
}

/// `rbx::signals` void-slot connection (IDA 0x3a98d0 et al.).
#[derive(Clone, Debug, Default)]
pub struct VoidSlotConn {
    pub id: u64,
    pub target: usize,
    pub live: bool,
}
use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x17e68 — -[Appirater connectedToNetwork]
// type: char __cdecl(Appirater *self, SEL)
#[doc(alias = "-[Appirater connectedToNetwork]")]
pub fn stub_17e68(reachable: bool, connection_opened: bool) -> bool {
    // IDA 0x17e68: -[Appirater connectedToNetwork] — zero-address SCNetworkReachability flags (0x17ea8..0x17ece) plus an apple.com probe connection (0x17ede..0x17f3a); reachable (flags&6==2 || flags&1, 0x17f4a) reports the connection (0x17f52), no-flags logs + 0 (0x17f60..0x17f64). System reachability folds; the decision stays 1:1.
        reachable && connection_opened}

// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_33454(block: usize, type_name: &str) -> usize {
    // IDA 0x33454: match "N3RBX9CreatableINS_8InstanceEE7DeleterE" → block + 16, else 0.
    if type_name == "N3RBX9CreatableINS_8InstanceEE7DeleterE" {
        block + 16
    } else {
        0
    }
}

// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_3346c(block: usize) -> usize {
    // IDA 0x3346c: return block + 16.
    block + 16
}

// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb")]
pub fn stub_3a7f68(rep: &mut EventReplicator, mode: bool, watch: &mut dyn FnMut(bool)) {
    // IDA 0x3a7f68: set the listener mode bit; (re)watch or unwatch below truncation.
    rep.listener_mode = mode;
    watch(mode);
}

// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb")]
pub fn stub_3a80c8(rep: &mut EventReplicator, mode: bool, watch: &mut dyn FnMut(bool)) {
    // IDA 0x3a80c8: set the listener mode bit; (re)watch or unwatch below truncation.
    rep.listener_mode = mode;
    watch(mode);
}

// 0x3a8228 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_3a8228(rep: &mut EventReplicator, prop_matches: bool, listener_count: i32, ensure_watching: &mut dyn FnMut()) {
    // IDA 0x3a8228: unconnected + property match: query listener count; < 1 and still unconnected →
    // watch.
    if !rep.connected && prop_matches && listener_count < 1 {
        ensure_watching();
        rep.watched = true;
    }
}

// 0x3a8288 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_3a8288(rep: &mut EventReplicator, prop_matches: bool, listener_count: i32, ensure_watching: &mut dyn FnMut()) {
    // IDA 0x3a8288: unconnected + property match: query listener count; < 1 and still unconnected →
    // watch.
    if !rep.connected && prop_matches && listener_count < 1 {
        ensure_watching();
        rep.watched = true;
    }
}

// 0x3a98d0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_3a98d0(slots: &mut Vec<VoidSlotConn>, target: usize) -> u64 {
    // IDA 0x3a98d0: operator new islot(28); callable ctor; signal::insert; connection.
    let id = slots.len() as u64;
    slots.push(VoidSlotConn { id, target, live: true });
    id
}

// 0x3a9944 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv")]
pub fn stub_3a9944(count: i32, add_listener: &mut dyn FnMut(i32) -> i32) -> i32 {
    // IDA 0x3a9944: idx = count >= 0 ? count + 1 : 1; listener-added hook.
    let idx = if count >= 0 { count + 1 } else { 1 };
    add_listener(idx)
}

// 0x3a9990 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev")]
pub fn stub_3a9990(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9990: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3a99bc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev")]
pub fn stub_3a99bc(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a99bc: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3a9a90 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_3a9a90(target: usize, invoke: &mut dyn FnMut(usize)) {
    // IDA 0x3a9a90: callable::call forwards to the bind_t at +16.
    invoke(target);
}

// 0x3a9a98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_3a9a98(target: usize, invoke: &mut dyn FnMut(usize)) {
    // IDA 0x3a9a98: non-virtual thunk adjusts to +12 then tail-calls.
    invoke(target);
}

// 0x3a9aa0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_3a9aa0(obj: usize, is_virtual: bool, call: &mut dyn FnMut(usize, bool)) {
    // IDA 0x3a9aa0: mf0 dispatch (virtual adjust when bit 1 set); obj->method().
    call(obj, is_virtual);
}

// 0x3a9ab8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev")]
pub fn stub_3a9ab8(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9ab8: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3a9ae4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev")]
pub fn stub_3a9ae4(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9ae4: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3a9bb8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_3a9bb8(slots: &mut Vec<VoidSlotConn>, target: usize) -> u64 {
    // IDA 0x3a9bb8: operator new islot(28); callable ctor; signal::insert; connection (Axis flavor).
    let id = slots.len() as u64;
    slots.push(VoidSlotConn { id, target, live: true });
    id
}

// 0x3a9c2c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv")]
pub fn stub_3a9c2c(count: i32, add_listener: &mut dyn FnMut(i32) -> i32) -> i32 {
    // IDA 0x3a9c2c: idx = count >= 0 ? count + 1 : 1; listener-added hook (Axis flavor).
    let idx = if count >= 0 { count + 1 } else { 1 };
    add_listener(idx)
}

// 0x3a9c78 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev")]
pub fn stub_3a9c78(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9c78: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3a9ca4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev")]
pub fn stub_3a9ca4(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9ca4: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3a9d78 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_3a9d78(target: usize, invoke: &mut dyn FnMut(usize)) {
    // IDA 0x3a9d78: callable::call forwards to the bind_t at +16.
    invoke(target);
}

// 0x3a9d80 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_3a9d80(target: usize, invoke: &mut dyn FnMut(usize)) {
    // IDA 0x3a9d80: non-virtual thunk adjusts to +12 then tail-calls.
    invoke(target);
}

// 0x3a9d88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_3a9d88(obj: usize, is_virtual: bool, call: &mut dyn FnMut(usize, bool)) {
    // IDA 0x3a9d88: mf0 dispatch (virtual adjust when bit 1 set); obj->method().
    call(obj, is_virtual);
}

// 0x3a9da0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev")]
pub fn stub_3a9da0(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9da0: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3a9dcc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev")]
pub fn stub_3a9dcc(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3a9dcc: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3aa448 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")]
pub fn stub_3aa448(rep: &mut EventReplicator, connect: &mut dyn FnMut() -> u64) -> u64 {
    // IDA 0x3aa448: build the mf0 bind_t listener; connect; return the connection.
    let id = connect();
    rep.connected = true;
    id
}

// 0x3aa5a4 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff")]
pub fn stub_3aa5a4(instance: usize, desc: usize, axis: i32, f1: f32, f2: f32, replicate: &mut dyn FnMut(usize, usize, i32, f32, f32) -> i32) -> i32 {
    // IDA 0x3aa5a4: instance word (+36 when set); replicateEvent(desc, instance, axis, f1, f2).
    replicate(desc, instance, axis, f1, f2)
}

// 0x3aa764 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_3aa764(slots: &mut Vec<VoidSlotConn>, target: usize) -> u64 {
    // IDA 0x3aa764: operator new islot; callable ctor; signal::insert; connection (mf3 flavor).
    let id = slots.len() as u64;
    slots.push(VoidSlotConn { id, target, live: true });
    id
}

// 0x3aaa08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev")]
pub fn stub_3aaa08(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3aaa08: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3aaa34 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev")]
pub fn stub_3aaa34(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3aaa34: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3aac24 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff")]
pub fn stub_3aac24(target: usize, axis: i32, f1: f32, f2: f32, invoke: &mut dyn FnMut(usize, i32, f32, f32)) {
    // IDA 0x3aac24: callable::call builds the list3 refs; bind_t::operator() forwards them.
    invoke(target, axis, f1, f2);
}

// 0x3aac50 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff")]
pub fn stub_3aac50(target: usize, axis: i32, f1: f32, f2: f32, invoke: &mut dyn FnMut(usize, i32, f32, f32)) {
    // IDA 0x3aac50: non-virtual thunk adjusts inward then tail-calls the operator().
    invoke(target, axis, f1, f2);
}

// 0x3aac7c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_3aac7c(obj: usize, is_virtual: bool, axis: i32, f1: f32, f2: f32, call: &mut dyn FnMut(usize, bool, i32, f32, f32)) {
    // IDA 0x3aac7c: mf3 dispatch (virtual adjust); obj->method(axis, f1, f2).
    call(obj, is_virtual, axis, f1, f2);
}

// 0x3aafa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev")]
pub fn stub_3aafa0(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3aafa0: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3aafcc — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev")]
pub fn stub_3aafcc(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3aafcc: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3ab0a0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")]
pub fn stub_3ab0a0() {
    // IDA 0x3ab0a0: empty base connectSignalListener body.
}

// 0x3ab0a4 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")]
pub fn stub_3ab0a4(rep: &mut EventReplicator, connect: &mut dyn FnMut() -> u64) -> u64 {
    // IDA 0x3ab0a4: build the mf1 bind_t listener; connect; return the connection.
    let id = connect();
    rep.connected = true;
    id
}

// 0x3ab200 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_")]
pub fn stub_3ab200(instance: usize, desc: usize, axis: i32, replicate: &mut dyn FnMut(usize, usize, i32) -> i32) -> i32 {
    // IDA 0x3ab200: instance word (+36 when set); replicateEvent(desc, instance, axis).
    replicate(desc, instance, axis)
}

// 0x3ab360 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_3ab360(slots: &mut Vec<VoidSlotConn>, target: usize) -> u64 {
    // IDA 0x3ab360: operator new islot; callable ctor; signal::insert; connection (mf1 flavor).
    let id = slots.len() as u64;
    slots.push(VoidSlotConn { id, target, live: true });
    id
}

// 0x3ab604 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev")]
pub fn stub_3ab604(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3ab604: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3ab630 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev")]
pub fn stub_3ab630(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3ab630: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3ab820 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_3ab820(target: usize, axis: i32, invoke: &mut dyn FnMut(usize, i32)) {
    // IDA 0x3ab820: callable::call builds the axis ref; bind_t::operator() forwards it.
    invoke(target, axis);
}

// 0x3ab834 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_3ab834(target: usize, axis: i32, invoke: &mut dyn FnMut(usize, i32)) {
    // IDA 0x3ab834: non-virtual thunk adjusts inward then tail-calls the operator().
    invoke(target, axis);
}

// 0x3ab848 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(char **, int *)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_")]
pub fn stub_3ab848(obj: usize, is_virtual: bool, axis: i32, call: &mut dyn FnMut(usize, bool, i32)) {
    // IDA 0x3ab848: mf1 dispatch (virtual adjust); obj->method(axis).
    call(obj, is_virtual, axis);
}

// 0x3abb44 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
pub fn stub_3abb44(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3abb44: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x3abb70 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
pub fn stub_3abb70(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x3abb70: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x3abc44 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")]
pub fn stub_3abc44() {
    // IDA 0x3abc44: empty base connectSignalListener body.
}

// 0x3b05bc — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev")]
pub fn stub_3b05bc(rep: &mut EventReplicator, conn5: &mut bool, conn6: &mut bool, disconnect: &mut dyn FnMut(u8)) {
    // IDA 0x3b05bc: vtable reset; disconnect live connections (+5/+6); weak releases (below truncation).
    if *conn5 {
        disconnect(5);
        *conn5 = false;
    }
    if *conn6 {
        disconnect(6);
        *conn6 = false;
    }
    rep.connected = false;
    rep.watched = false;
}

// 0x3b06ec — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev")]
pub fn stub_3b06ec(rep: &mut EventReplicator, conn5: &mut bool, conn6: &mut bool, disconnect: &mut dyn FnMut(u8)) {
    // IDA 0x3b06ec: vtable reset; disconnect live connections (+5/+6); weak releases (below truncation).
    if *conn5 {
        disconnect(5);
        *conn5 = false;
    }
    if *conn6 {
        disconnect(6);
        *conn6 = false;
    }
    rep.connected = false;
    rep.watched = false;
}

// 0x3c9c4c — __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE")]
pub fn stub_3c9c4c(has_provider: bool, create: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3c9c4c: findServiceProvider ? create<Players>() : 0.
    if has_provider {
        create()
    } else {
        0
    }
}

// 0x3f1114 — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE")]
pub fn stub_3f1114(distance: f32, player: usize, fire: &mut dyn FnMut(f32, usize)) {
    // IDA 0x3f1114: ClickDetector::fireMouseClick — impersonate + fire the MouseClick signal
    // (below truncation).
    fire(distance, player);
}

// 0x3f1234 — __ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE
// type: int __fastcall(int *, float, int, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isClickable(boost::shared_ptr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE")]
pub fn stub_3f1234(has_part: bool, is_instance: bool, in_range: bool) -> bool {
    // IDA 0x3f1234: ClickDetector::isClickable — null gate, isA(Instance) gate, range checks.
    has_part && is_instance && in_range
}

// 0x3f12e0 — __ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *, int *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::updateLastHoverPart(boost::shared_ptr<RBX::Instance>,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE")]
pub fn stub_3f12e0(det: &mut ClickDetector, part: Option<usize>, player: usize, fire_hover: &mut dyn FnMut(usize)) -> bool {
    // IDA 0x3f12e0: changed ? (fireMouseHover when set; store part) : same -> 0.
    if part != det.hover_part {
        if part.is_some() {
            fire_hover(player);
        }
        det.hover_part = part;
        true
    } else {
        false
    }
}

// 0x3f130c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE")]
pub fn stub_3f130c(player: usize, fire: &mut dyn FnMut(usize)) {
    // IDA 0x3f130c: ClickDetector::fireMouseHover — shared_from<Player> + fire MouseHover (below truncation).
    fire(player);
}

// 0x3f1410 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE")]
pub fn stub_3f1410(player: usize, fire: &mut dyn FnMut(usize)) {
    // IDA 0x3f1410: ClickDetector::fireMouseHoverLeave — shared_from<Player> + fire leave (below truncation).
    fire(player);
}

// 0x3f154c — __ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE
// type: void __fastcall(int *, RBX::Network::Player *, int, int)
#[doc(alias = "RBX::ClickDetector::stopHover(boost::shared_ptr<RBX::PartInstance>,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE")]
pub fn stub_3f154c(has_part: bool, is_instance: bool, has_detector: bool, player: usize, fire_leave: &mut dyn FnMut(usize)) {
    // IDA 0x3f154c: null/instance gates; findConstFirstChildOfType<ClickDetector> -> fireMouseHoverLeave.
    if has_part && is_instance && has_detector {
        fire_leave(player);
    }
}

// 0x3f15b8 — __ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *this, RBX::PartInstance *, float, RBX::Network::Player *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE")]
pub fn stub_3f15b8(is_current: bool, in_range: bool, fire: &mut dyn FnMut()) -> bool {
    // IDA 0x3f15b8: ClickDetector::isHovered — hover-part identity + distance checks (below truncation).
    let ok = is_current && in_range;
    if ok {
        fire();
    }
    ok
}

// 0x3f7df0 — __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE")]
pub fn stub_3f7df0(cmd: usize, model: usize, init: &mut dyn FnMut(usize, usize)) {
    // IDA 0x3f7df0: C1 thunk tail-calls the C2 constructor.
    init(cmd, model);
}

// 0x3f7df4 — __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE")]
pub fn stub_3f7df4(cmd: usize, model: usize, init_verb: &mut dyn FnMut(usize, usize)) -> usize {
    // IDA 0x3f7df4: NetworkStatsCommand::NetworkStatsCommand — Verb base + fields (below truncation).
    init_verb(cmd, model);
    cmd
}

// 0x3f7f80 — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE")]
pub fn stub_3f7f80(show: &mut dyn FnMut()) {
    // IDA 0x3f7f80: NetworkStatsCommand::doIt — FLog::Verbs + stats UI display (below truncation).
    show();
}

// 0x3f8268 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
#[doc(alias = "__ZNK3RBX19NetworkStatsCommand9isEnabledEv")]
pub fn stub_3f8268(found: bool) -> bool {
    // IDA 0x3f8268: NetworkStatsCommand::isEnabled — workspace child lookup gates (below truncation).
    found
}

// 0x3f83e4 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv
// type: int __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
#[doc(alias = "__ZNK3RBX19NetworkStatsCommand9isCheckedEv")]
pub fn stub_3f83e4(cmd: &NetworkStatsCommand) -> bool {
    // IDA 0x3f83e4: NetworkStatsCommand::isChecked — checked flag.
    cmd.checked
}

// 0x3fe628 — __ZN3RBX19NetworkStatsCommandD1Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandD1Ev")]
pub fn stub_3fe628(destroy_verb: &mut dyn FnMut()) {
    // IDA 0x3fe628: D1 tail-calls Verb::~Verb.
    destroy_verb();
}

// 0x3fe62c — __ZN3RBX19NetworkStatsCommandD0Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandD0Ev")]
pub fn stub_3fe62c(destroy_verb: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x3fe62c: D0: Verb::~Verb + operator delete.
    destroy_verb();
    free();
}

// 0x3ff478 — __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_3ff478(guard: &mut bool, slot: &mut usize, init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x3ff478: guarded one-time Players classDescriptor init; return the descriptor.
    if !*guard {
        *slot = init();
        *guard = true;
    }
    *slot
}

// 0x401cec — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_401cec(check_asserts: bool, constructed: bool, name: &str) -> &str {
    // IDA 0x401cec: ReleaseAssert(wasConstructed()) when FLog::Asserts; return Name::declare(sNetworkSettings).
    if check_asserts {
        assert!(constructed, "wasConstructed() file: include/Util/Object.h line: 236");
    }
    name
}

// 0x401d58 — __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")]
pub fn stub_401d58(has_name: bool, null_name: usize, once: &mut dyn FnMut(), declared: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x401d58: null sNetworkSettings -> getNullName; else call_once(callDoDeclare) + doDeclare.
    if !has_name {
        return null_name;
    }
    once();
    declared()
}

// 0x401d9c — __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")]
pub fn stub_401d9c(guard: &mut bool, cached: &mut usize, declare: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x401d9c: cxa_guard one-time Name::declare(sNetworkSettings).
    if !*guard {
        *cached = declare();
        *guard = true;
    }
    *cached
}

// 0x425d58 — __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
#[doc(alias = "__ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE")]
pub fn stub_425d58(mode: u32, state: &mut PhysicsSimFilter, apply: &mut dyn FnMut(u32)) {
    // IDA 0x425d58: getSimSendFilter + findLocalSimulatorAddress; zero 0xC00/0xC04/0xC08; mode switch
    // (below truncation).
    state.mode_words = [0, 0];
    state.flag = false;
    apply(mode);
}

// 0x427db8 — __ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE
// type: int __fastcall(int this, IMetric *)
#[doc(alias = "RBX::DataModel::setNetworkMetric(RBX::IMetric *)")]
#[doc(alias = "__ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE")]
pub fn stub_427db8(state: &mut DataModelMetric, metric: usize) -> usize {
    // IDA 0x427db8: STR metric, [this, #0xBB8]; return this.
    state.metric = Some(metric);
    state.this
}

// 0x44ab28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(boost::shared_ptr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_44ab28(use_count: u32, adopt: &mut dyn FnMut(), share: &mut dyn FnMut()) {
    // IDA 0x44ab28: weak_count::use_count gates the weak_this store (below truncation).
    if use_count == 0 {
        adopt();
    } else {
        share();
    }
}

// 0x44ac18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_44ac18(block: usize, free: &mut dyn FnMut(usize)) {
    // IDA 0x44ac18: D0 thunk tail-calls operator delete.
    free(block);
}

// 0x4da8f8 — __ZN3RBX10Reflection9DescribedINS_7Network10ChatFilterELZNS2_11sChatFilterEENS_17NonFactoryProductINS_8InstanceELZNS2_11sChatFilterEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network10ChatFilterELZNS2_11sChatFilterEENS_17NonFactoryProductINS_8InstanceELZNS2_11sChatFilterEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4da8f8(guard: &mut bool, slot: &mut usize, init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4da8f8: guarded one-time ChatFilter classDescriptor init; return the descriptor.
    if !*guard {
        *slot = init();
        *guard = true;
    }
    *slot
}

// 0x4daa18 — __ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18ClusterPacketCacheELZNS2_19sClusterPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sClusterPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4daa18(guard: &mut bool, slot: &mut usize, init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4daa18: guarded one-time ClusterPacketCache classDescriptor init; return the descriptor.
    if !*guard {
        *slot = init();
        *guard = true;
    }
    *slot
}

// 0x4dab38 — __ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network19InstancePacketCacheELZNS2_20sInstancePacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_20sInstancePacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4dab38(guard: &mut bool, slot: &mut usize, init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4dab38: guarded one-time InstancePacketCache classDescriptor init; return the descriptor.
    if !*guard {
        *slot = init();
        *guard = true;
    }
    *slot
}

// 0x4dac58 — __ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network18PhysicsPacketCacheELZNS2_19sPhysicsPacketCacheEENS_17NonFactoryProductINS_8InstanceELZNS2_19sPhysicsPacketCacheEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_4dac58(guard: &mut bool, slot: &mut usize, init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x4dac58: guarded one-time PhysicsPacketCache classDescriptor init; return the descriptor.
    if !*guard {
        *slot = init();
        *guard = true;
    }
    *slot
}

// 0x4f1df8 — __ZN3RBX4Flag21canBePickedUpByPlayerEPNS_7Network6PlayerE
#[doc(alias = "RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX4Flag21canBePickedUpByPlayerEPNS_7Network6PlayerE")]
pub fn stub_4f1df8(player_team: u32, flag_neutral: bool, flag_team: u32) -> bool {
    // IDA 0x4f1df8: !neutral[104] ? team[100] != player[472] : false.
    if flag_neutral {
        false
    } else {
        flag_team != player_team
    }
}

// 0x516a30 — __ZN3RBX10GuiBuilder17buildNetworkStatsEv
// type: _DWORD __fastcall(RBX::GuiBuilder *__hidden this)
#[doc(alias = "RBX::GuiBuilder::buildNetworkStats(void)")]
#[doc(alias = "__ZN3RBX10GuiBuilder17buildNetworkStatsEv")]
pub fn stub_516a30(build: &mut dyn FnMut()) {
    // IDA 0x516a30: GuiBuilder::buildNetworkStats — stats GuiItem tree (below truncation).
    build();
}

// 0x518284 — __ZN3RBX10GuiBuilder18buildNetworkStats2Ev
// type: _DWORD __fastcall(RBX::GuiBuilder *__hidden this)
#[doc(alias = "RBX::GuiBuilder::buildNetworkStats2(void)")]
#[doc(alias = "__ZN3RBX10GuiBuilder18buildNetworkStats2Ev")]
pub fn stub_518284(build: &mut dyn FnMut()) {
    // IDA 0x518284: GuiBuilder::buildNetworkStats2 — stats GuiItem tree (below truncation).
    build();
}

// 0x52d620 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb")]
pub fn stub_52d620(rep: &mut EventReplicator, mode: bool, watch: &mut dyn FnMut(bool)) {
    // IDA 0x52d620: setListenerMode — store mode; attach/detach watch on transition (below truncation).
    rep.listener_mode = mode;
    watch(mode);
    rep.watched = mode;
}

// 0x52d780 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb")]
pub fn stub_52d780(rep: &mut EventReplicator, mode: bool, watch: &mut dyn FnMut(bool)) {
    // IDA 0x52d780: setListenerMode — store mode; attach/detach watch on transition (below truncation).
    rep.listener_mode = mode;
    watch(mode);
    rep.watched = mode;
}

// 0x52d9c4 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_52d9c4(connected20: bool, desc_match: bool, instance: usize, desc: usize, get: &mut dyn FnMut(usize, usize) -> i32, fire: &mut dyn FnMut(i32)) {
    // IDA 0x52d9c4: unconnected gate; descriptor match; getter read; event fire (below truncation).
    if connected20 || !desc_match {
        return;
    }
    let v = get(desc, instance);
    fire(v);
}

// 0x52da24 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_52da24(connected20: bool, desc_match: bool, instance: usize, desc: usize, get: &mut dyn FnMut(usize, usize) -> i32, fire: &mut dyn FnMut(i32)) {
    // IDA 0x52da24: unconnected gate; descriptor match; getter read; event fire (below truncation).
    if connected20 || !desc_match {
        return;
    }
    let v = get(desc, instance);
    fire(v);
}

// 0x52e250 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb")]
pub fn stub_52e250(rep: &mut EventReplicator, mode: bool, watch: &mut dyn FnMut(bool)) {
    // IDA 0x52e250: setListenerMode — store mode; attach/detach watch on transition (below truncation).
    rep.listener_mode = mode;
    watch(mode);
    rep.watched = mode;
}

// 0x52e3b0 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb")]
pub fn stub_52e3b0(rep: &mut EventReplicator, mode: bool, watch: &mut dyn FnMut(bool)) {
    // IDA 0x52e3b0: setListenerMode — store mode; attach/detach watch on transition (below truncation).
    rep.listener_mode = mode;
    watch(mode);
    rep.watched = mode;
}

// 0x52e510 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_52e510(connected20: bool, desc_match: bool, instance: usize, desc: usize, get: &mut dyn FnMut(usize, usize) -> i32, fire: &mut dyn FnMut(i32)) {
    // IDA 0x52e510: unconnected gate; descriptor match; getter read; event fire (below truncation).
    if connected20 || !desc_match {
        return;
    }
    let v = get(desc, instance);
    fire(v);
}

// 0x52e570 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_52e570(connected20: bool, desc_match: bool, instance: usize, desc: usize, get: &mut dyn FnMut(usize, usize) -> i32, fire: &mut dyn FnMut(i32)) {
    // IDA 0x52e570: unconnected gate; descriptor match; getter read; event fire (below truncation).
    if connected20 || !desc_match {
        return;
    }
    let v = get(desc, instance);
    fire(v);
}

// 0x52ee40 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_52ee40(slots: &mut Vec<VoidSlotConn>, target: usize) -> u64 {
    // IDA 0x52ee40: operator new islot; callable ctor; signal::insert; connection (mf0 flavor).
    let id = slots.len() as u64;
    slots.push(VoidSlotConn { id, target, live: true });
    id
}

// 0x52eeb4 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv")]
pub fn stub_52eeb4(instance: usize, desc: usize, read: &mut dyn FnMut(usize, usize) -> i32, write: &mut dyn FnMut(usize, usize, i32)) {
    // IDA 0x52eeb4: read current; push (current+1) or 1 when negative.
    let cur = read(desc, instance);
    let next = if cur >= 0 { cur + 1 } else { 1 };
    write(desc, instance, next);
}

// 0x52ef00 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev")]
pub fn stub_52ef00(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x52ef00: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x52ef2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev")]
pub fn stub_52ef2c(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x52ef2c: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x52f000 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
pub fn stub_52f000(invoke: &mut dyn FnMut()) {
    // IDA 0x52f000: callable::call tail-calls bind_t::operator().
    invoke();
}

// 0x52f008 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv")]
pub fn stub_52f008(invoke: &mut dyn FnMut()) {
    // IDA 0x52f008: non-virtual thunk tail-calls the operator().
    invoke();
}

// 0x52f010 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
pub fn stub_52f010(obj: usize, is_virtual: bool, call: &mut dyn FnMut(usize, bool)) {
    // IDA 0x52f010: mf0 dispatch (virtual adjust); obj->method().
    call(obj, is_virtual);
}

// 0x52f028 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev")]
pub fn stub_52f028(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x52f028: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x52f054 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev")]
pub fn stub_52f054(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x52f054: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x52f128 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_52f128(slots: &mut Vec<VoidSlotConn>, target: usize) -> u64 {
    // IDA 0x52f128: operator new islot; callable ctor; signal::insert; connection (mf0 flavor).
    let id = slots.len() as u64;
    slots.push(VoidSlotConn { id, target, live: true });
    id
}

// 0x52f19c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv")]
pub fn stub_52f19c(instance: usize, desc: usize, read: &mut dyn FnMut(usize, usize) -> i32, write: &mut dyn FnMut(usize, usize, i32)) {
    // IDA 0x52f19c: read current; push (current+1) or 1 when negative.
    let cur = read(desc, instance);
    let next = if cur >= 0 { cur + 1 } else { 1 };
    write(desc, instance, next);
}

// 0x52f1e8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev")]
pub fn stub_52f1e8(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x52f1e8: D1: vtable resets; intrusive release (no delete).
    if let Some(s) = slots.iter_mut().find(|s| s.id == id) {
        s.live = false;
        release(s.id);
    }
}

// 0x52f214 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev")]
pub fn stub_52f214(slots: &mut Vec<VoidSlotConn>, id: u64, release: &mut dyn FnMut(u64)) {
    // IDA 0x52f214: D0: vtable resets; intrusive release; operator delete.
    if let Some(pos) = slots.iter().position(|s| s.id == id) {
        let s = slots.remove(pos);
        release(s.id);
    }
}

// 0x52f2e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
pub fn stub_52f2e8(invoke: &mut dyn FnMut()) {
    // IDA 0x52f2e8: callable::call tail-calls bind_t::operator().
    invoke();
}

// 0x52f2f0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
pub fn stub_52f2f0() -> ! {
    todo!("0x52f2f0 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")
}

// 0x52f2f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
pub fn stub_52f2f8() -> ! {
    todo!("0x52f2f8 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")
}

// 0x52f310 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev")]
pub fn stub_52f310() -> ! {
    todo!("0x52f310 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev")
}

// 0x52f33c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev")]
pub fn stub_52f33c() -> ! {
    todo!("0x52f33c __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev")
}

// 0x52f55c — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv")]
pub fn stub_52f55c() -> ! {
    todo!("0x52f55c __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv")
}

// 0x52f6b8 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::signalProducedIncremented(int,int)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii")]
pub fn stub_52f6b8() -> ! {
    todo!("0x52f6b8 __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii")
}

// 0x52f83c — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_52f83c() -> ! {
    todo!("0x52f83c __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")
}

// 0x52fae0 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")]
pub fn stub_52fae0() -> ! {
    todo!("0x52fae0 __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")
}

// 0x52fb0c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")]
pub fn stub_52fb0c() -> ! {
    todo!("0x52fb0c __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")
}

// 0xf672c4 — j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>*)")]
#[doc(alias = "j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_")]
pub fn stub_f672c4() -> ! {
    todo!("0xf672c4 j___ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_16MemoryDataStreamEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_")
}

// 0xf672f4 — j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
#[doc(alias = "j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
pub fn stub_f672f4() -> ! {
    todo!("0xf672f4 j___ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

// 0xf67324 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_")]
pub fn stub_f67324() -> ! {
    todo!("0xf67324 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_")
}

// 0xf67334 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
pub fn stub_f67334() -> ! {
    todo!("0xf67334 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")
}

// 0xf67344 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
pub fn stub_f67344() -> ! {
    todo!("0xf67344 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")
}

// 0xf67354 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
pub fn stub_f67354() -> ! {
    todo!("0xf67354 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")
}

// 0xf67364 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<Ogre::MemoryDataStream>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
pub fn stub_f67364() -> ! {
    todo!("0xf67364 j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_16MemoryDataStreamEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")
}

// 0xf67e34 — j___ZN4Ogre9SharedPtrINS_10DataStreamEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::operator=(Ogre::SharedPtr<Ogre::DataStream> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_10DataStreamEEaSERKS2_")]
pub fn stub_f67e34() -> ! {
    todo!("0xf67e34 j___ZN4Ogre9SharedPtrINS_10DataStreamEEaSERKS2_")
}

// 0xf68744 — j___ZN4Ogre9SharedPtrINS_16MemoryDataStreamEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::MemoryDataStream>::operator=(Ogre::SharedPtr<Ogre::MemoryDataStream> const&)")]
#[doc(alias = "j___ZN4Ogre9SharedPtrINS_16MemoryDataStreamEEaSERKS2_")]
pub fn stub_f68744() -> ! {
    todo!("0xf68744 j___ZN4Ogre9SharedPtrINS_16MemoryDataStreamEEaSERKS2_")
}

// 0xf68774 — j___ZNSt4pairIN4Ogre9SharedPtrINS0_16MemoryDataStreamEEENS1_INS0_5Codec9CodecDataEEEED2Ev
#[doc(alias = "std::pair<Ogre::SharedPtr<Ogre::MemoryDataStream>,Ogre::SharedPtr<Ogre::Codec::CodecData>>::~pair()")]
#[doc(alias = "j___ZNSt4pairIN4Ogre9SharedPtrINS0_16MemoryDataStreamEEENS1_INS0_5Codec9CodecDataEEEED2Ev")]
pub fn stub_f68774() -> ! {
    todo!("0xf68774 j___ZNSt4pairIN4Ogre9SharedPtrINS0_16MemoryDataStreamEEENS1_INS0_5Codec9CodecDataEEEED2Ev")
}
