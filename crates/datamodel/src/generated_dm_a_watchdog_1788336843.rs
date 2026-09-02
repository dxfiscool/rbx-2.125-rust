// Auto-generated skeletons for rbx-datamodel shard A — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|Workspace complete (10215) — fallback global gap filler EA-sorted asc not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000) — batch 120, range 0x3a9ea0..0x3aee10 | dm gap before 50833, after 50713
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ' stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3a9ea0 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")]
pub fn stub_0x3a9ea0() -> ! {
    todo!("0x3a9ea0 rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")
}

// 0x3a9ffc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::disconnectAll(void)")]
pub fn stub_0x3a9ffc() -> ! {
    todo!("0x3a9ffc rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::disconnectAll(void)")
}

// 0x3aa174 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")]
pub fn stub_0x3aa174() -> ! {
    todo!("0x3aa174 rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")
}

// 0x3aa2d0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::disconnectAll(void)")]
pub fn stub_0x3aa2d0() -> ! {
    todo!("0x3aa2d0 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::disconnectAll(void)")
}

// 0x3aa448 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_0x3aa448() -> ! {
    todo!("0x3aa448 RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")
}

// 0x3aa53c — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
pub fn stub_0x3aa53c() -> ! {
    todo!("0x3aa53c RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x3aa5a4 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aa5a4() -> ! {
    todo!("0x3aa5a4 RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")
}

// 0x3aa5c8 — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE14replicateEventEPNS0_11EventSourceES5_ff
// type: int __fastcall(int, int, void (__fastcall **)(int), int, float)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aa5c8() -> ! {
    todo!("0x3aa5c8 RBX::Reflection::RemoteEventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,G3D::Vector3::Axis,float,float)")
}

// 0x3aa764 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_0x3aa764() -> ! {
    todo!("0x3aa764 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0x3aa7d8 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
pub fn stub_0x3aa7d8() -> ! {
    todo!("0x3aa7d8 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")
}

// 0x3aa9e4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSEPS9_
// type: int *__fastcall(int *, int)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot*)")]
pub fn stub_0x3aa9e4() -> ! {
    todo!("0x3aa9e4 boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot*)")
}

// 0x3aaa08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0x3aaa08() -> ! {
    todo!("0x3aaa08 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x3aaa34 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0x3aaa34() -> ! {
    todo!("0x3aaa34 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x3aab08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::disconnect(void)")]
pub fn stub_0x3aab08() -> ! {
    todo!("0x3aab08 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::disconnect(void)")
}

// 0x3aac18 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::connected(void)const")]
pub fn stub_0x3aac18() -> ! {
    todo!("0x3aac18 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::connected(void)const")
}

// 0x3aac24 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aac24() -> ! {
    todo!("0x3aac24 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3aac50 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aac50() -> ! {
    todo!("0x3aac50 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3aac7c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_0x3aac7c() -> ! {
    todo!("0x3aac7c void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")
}

// 0x3aacbc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
pub fn stub_0x3aacbc() -> ! {
    todo!("0x3aacbc rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")
}

// 0x3aadac — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3aadac() -> ! {
    todo!("0x3aadac rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_init_mutex(void)")
}

// 0x3aadb0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x3aadb0() -> ! {
    todo!("0x3aadb0 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_do_get_mutex(void)")
}

// 0x3aaea0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
pub fn stub_0x3aaea0() -> ! {
    todo!("0x3aaea0 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")
}

// 0x3aaecc — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
pub fn stub_0x3aaecc() -> ! {
    todo!("0x3aaecc rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")
}

// 0x3aafa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_0x3aafa0() -> ! {
    todo!("0x3aafa0 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")
}

// 0x3aafcc — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_0x3aafcc() -> ! {
    todo!("0x3aafcc rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")
}

// 0x3ab0a0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_0x3ab0a0() -> ! {
    todo!("0x3ab0a0 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")
}

// 0x3ab0a4 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_0x3ab0a4() -> ! {
    todo!("0x3ab0a4 RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")
}

// 0x3ab198 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12getSignalPtrEPNS0_11EventSourceE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
pub fn stub_0x3ab198() -> ! {
    todo!("0x3ab198 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x3ab200 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
pub fn stub_0x3ab200() -> ! {
    todo!("0x3ab200 RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")
}

// 0x3ab214 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE14replicateEventEPNS0_11EventSourceES5_
// type: int __fastcall(int, int, void (__fastcall **)(int))
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::replicateEvent(RBX::Reflection::EventSource *,G3D::Vector3::Axis)")]
pub fn stub_0x3ab214() -> ! {
    todo!("0x3ab214 RBX::Reflection::RemoteEventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::replicateEvent(RBX::Reflection::EventSource *,G3D::Vector3::Axis)")
}

// 0x3ab360 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
pub fn stub_0x3ab360() -> ! {
    todo!("0x3ab360 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")
}

// 0x3ab3d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
pub fn stub_0x3ab3d4() -> ! {
    todo!("0x3ab3d4 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")
}

// 0x3ab5e0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSEPS9_
// type: int *__fastcall(int *, int)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot*)")]
pub fn stub_0x3ab5e0() -> ! {
    todo!("0x3ab5e0 boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot*)")
}

// 0x3ab604 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x3ab604() -> ! {
    todo!("0x3ab604 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3ab630 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x3ab630() -> ! {
    todo!("0x3ab630 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3ab704 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")]
pub fn stub_0x3ab704() -> ! {
    todo!("0x3ab704 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")
}

// 0x3ab814 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")]
pub fn stub_0x3ab814() -> ! {
    todo!("0x3ab814 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")
}

// 0x3ab820 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_0x3ab820() -> ! {
    todo!("0x3ab820 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")
}

// 0x3ab834 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_0x3ab834() -> ! {
    todo!("0x3ab834 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")
}

// 0x3ab848 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(char **, int *)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
pub fn stub_0x3ab848() -> ! {
    todo!("0x3ab848 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")
}

// 0x3ab860 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
pub fn stub_0x3ab860() -> ! {
    todo!("0x3ab860 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")
}

// 0x3ab950 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3ab950() -> ! {
    todo!("0x3ab950 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")
}

// 0x3ab954 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x3ab954() -> ! {
    todo!("0x3ab954 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")
}

// 0x3aba44 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
pub fn stub_0x3aba44() -> ! {
    todo!("0x3aba44 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")
}

// 0x3aba70 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
pub fn stub_0x3aba70() -> ! {
    todo!("0x3aba70 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")
}

// 0x3abb44 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_0x3abb44() -> ! {
    todo!("0x3abb44 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")
}

// 0x3abb70 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_0x3abb70() -> ! {
    todo!("0x3abb70 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")
}

// 0x3abc44 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_0x3abc44() -> ! {
    todo!("0x3abc44 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")
}

// 0x3abc48 — __ZNK3RBX11HandlesBase13getHandleTypeEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::getHandleType(void)const")]
pub fn stub_0x3abc48() -> ! {
    todo!("0x3abc48 RBX::HandlesBase::getHandleType(void)const")
}

// 0x3abc4c — __ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::getHandlesNormalIdMask(void)const")]
pub fn stub_0x3abc4c() -> ! {
    todo!("0x3abc4c RBX::HandlesBase::getHandlesNormalIdMask(void)const")
}

// 0x3abe4c — __ZN3RBX11HandlesBaseD2Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3abe4c() -> ! {
    todo!("0x3abe4c RBX::HandlesBase::~HandlesBase()")
}

// 0x3ac098 — __ZN3RBX11HandlesBaseD1Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac098() -> ! {
    todo!("0x3ac098 RBX::HandlesBase::~HandlesBase()")
}

// 0x3ac09c — __ZN3RBX11HandlesBaseD0Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac09c() -> ! {
    todo!("0x3ac09c RBX::HandlesBase::~HandlesBase()")
}

// 0x3ac13c — __ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
pub fn stub_0x3ac13c() -> ! {
    todo!("0x3ac13c __ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")
}

// 0x3ac140 — __ZThn32_N3RBX11HandlesBaseD1Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac140() -> ! {
    todo!("0x3ac140 non-virtual thunk toRBX::HandlesBase::~HandlesBase()")
}

// 0x3ac148 — __ZThn32_N3RBX11HandlesBaseD0Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac148() -> ! {
    todo!("0x3ac148 non-virtual thunk toRBX::HandlesBase::~HandlesBase()")
}

// 0x3ac1ec — __ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
pub fn stub_0x3ac1ec() -> ! {
    todo!("0x3ac1ec __ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")
}

// 0x3ac1f0 — __ZThn36_N3RBX11HandlesBaseD1Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac1f0() -> ! {
    todo!("0x3ac1f0 non-virtual thunk toRBX::HandlesBase::~HandlesBase()")
}

// 0x3ac1f8 — __ZThn36_N3RBX11HandlesBaseD0Ev
// type: void __fastcall(RBX::HandlesBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac1f8() -> ! {
    todo!("0x3ac1f8 non-virtual thunk toRBX::HandlesBase::~HandlesBase()")
}

// 0x3ac29c — __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")]
pub fn stub_0x3ac29c() -> ! {
    todo!("0x3ac29c __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")
}

// 0x3ac2e0 — __ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")]
pub fn stub_0x3ac2e0() -> ! {
    todo!("0x3ac2e0 __ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")
}

// 0x3ac2e4 — __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")]
pub fn stub_0x3ac2e4() -> ! {
    todo!("0x3ac2e4 __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")
}

// 0x3ac5b0 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
pub fn stub_0x3ac5b0() -> ! {
    todo!("0x3ac5b0 __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")
}

// 0x3ac858 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
pub fn stub_0x3ac858() -> ! {
    todo!("0x3ac858 __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")
}

// 0x3acb00 — __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")]
pub fn stub_0x3acb00() -> ! {
    todo!("0x3acb00 __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")
}

// 0x3acb44 — __ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")]
pub fn stub_0x3acb44() -> ! {
    todo!("0x3acb44 __ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")
}

// 0x3acb48 — __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")]
pub fn stub_0x3acb48() -> ! {
    todo!("0x3acb48 __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")
}

// 0x3acc2c — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_10ArcHandlesEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ArcHandles>(char const*,char const*,int RBX::ArcHandles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3acc2c() -> ! {
    todo!("0x3acc2c RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ArcHandles>(char const*,char const*,int RBX::ArcHandles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3acdbc — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_10ArcHandlesEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::isReadOnly(void)const")]
pub fn stub_0x3acdbc() -> ! {
    todo!("0x3acdbc RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::isReadOnly(void)const")
}

// 0x3acdc0 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_10ArcHandlesEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::isWriteOnly(void)const")]
pub fn stub_0x3acdc0() -> ! {
    todo!("0x3acdc0 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::isWriteOnly(void)const")
}

// 0x3acdc4 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_10ArcHandlesEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3acdc4() -> ! {
    todo!("0x3acdc4 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3acdd0 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_10ArcHandlesEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x3acdd0() -> ! {
    todo!("0x3acdd0 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x3ace20 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x3ace20() -> ! {
    todo!("0x3ace20 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc()")
}

// 0x3aced4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3aced4() -> ! {
    todo!("0x3aced4 RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3ad038 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isScriptable(void)const")]
pub fn stub_0x3ad038() -> ! {
    todo!("0x3ad038 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isScriptable(void)const")
}

// 0x3ad040 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isBroadcast(void)const")]
pub fn stub_0x3ad040() -> ! {
    todo!("0x3ad040 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isBroadcast(void)const")
}

// 0x3ad048 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ad048() -> ! {
    todo!("0x3ad048 RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ad0f0 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ad0f0() -> ! {
    todo!("0x3ad0f0 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ad100 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3ad100() -> ! {
    todo!("0x3ad100 RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3ad114 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisERKfSA_NS_10shared_ptrIS3_EENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISJ_T0_T1_T2_T3_EENSH_9list_av_4IT4_T5_T6_T7_E4typeEEEMSM_FSJ_SN_SO_SP_ESS_ST_SU_SV_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x3ad114() -> ! {
    todo!("0x3ad114 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x3ad230 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN3G3D7Vector34AxisEffEEvRKT_RKT0_RKT1_
// type: int __fastcall(int, void (__fastcall ***)(int), int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<G3D::Vector3::Axis,float,float>(G3D::Vector3::Axis const&,float const&,float const&)")]
pub fn stub_0x3ad230() -> ! {
    todo!("0x3ad230 void RBX::Reflection::GenericSlotWrapper::execute3<G3D::Vector3::Axis,float,float>(G3D::Vector3::Axis const&,float const&,float const&)")
}

// 0x3ad3bc — __ZN5boost9function3IvN3G3D7Vector34AxisEffE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::clear(void)")]
pub fn stub_0x3ad3bc() -> ! {
    todo!("0x3ad3bc boost::function3<void,G3D::Vector3::Axis,float,float>::clear(void)")
}

// 0x3ad3e8 — __ZN5boost8functionIFvN3G3D7Vector34AxisEffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSH_EENS7_5list4INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3G3D7Vector34AxisEffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSH_EENS7_5list4INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3ad3e8() -> ! {
    todo!("0x3ad3e8 __ZN5boost8functionIFvN3G3D7Vector34AxisEffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSH_EENS7_5list4INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

// 0x3ad4cc — __ZN5boost9function3IvN3G3D7Vector34AxisEffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSG_EENS6_5list4INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvN3G3D7Vector34AxisEffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSG_EENS6_5list4INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3ad4cc() -> ! {
    todo!("0x3ad4cc __ZN5boost9function3IvN3G3D7Vector34AxisEffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSG_EENS6_5list4INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0x3ad5b4 — __ZN5boost9function3IvN3G3D7Vector34AxisEffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSG_EENS6_5list4INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x3ad5b4() -> ! {
    todo!("0x3ad5b4 void boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x3ad6ac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisERKfSG_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3ad6ac() -> ! {
    todo!("0x3ad6ac boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3ad6c8 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisERKfSG_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEvSC_ffE6invokeERNS1_15function_bufferESC_ff
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,G3D::Vector3::Axis,float,float>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3ad6c8() -> ! {
    todo!("0x3ad6c8 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,G3D::Vector3::Axis,float,float>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis,float,float)")
}

// 0x3ad6f0 — __ZNK5boost6detail8function13basic_vtable3IvN3G3D7Vector34AxisEffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS5_RKfSI_EENS8_5list4INS8_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3ad6f0() -> ! {
    todo!("0x3ad6f0 bool boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x3ad7d8 — __ZNK5boost6detail8function13basic_vtable3IvN3G3D7Vector34AxisEffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS5_RKfSI_EENS8_5list4INS8_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3ad7d8() -> ! {
    todo!("0x3ad7d8 bool boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x3ad8bc — __ZNK5boost6detail8function13basic_vtable3IvN3G3D7Vector34AxisEffE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS5_RKfSI_EENS8_5list4INS8_5valueINS_10shared_ptrISE_EEEENS_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3ad8bc() -> ! {
    todo!("0x3ad8bc void boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x3ad990 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKN3G3D7Vector34AxisERKfSN_EENS0_5list3IRSJ_RfSR_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int *)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_0x3ad990() -> ! {
    todo!("0x3ad990 void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")
}

// 0x3ad9b8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisERKfSG_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3ad9b8() -> ! {
    todo!("0x3ad9b8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x3adb10 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::function<void ()(G3D::Vector3::Axis,float,float)>>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&)")]
pub fn stub_0x3adb10() -> ! {
    todo!("0x3adb10 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::function<void ()(G3D::Vector3::Axis,float,float)>>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&)")
}

// 0x3adc04 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_EC2IPS7_EERKSB_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*)")]
pub fn stub_0x3adc04() -> ! {
    todo!("0x3adc04 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*)")
}

// 0x3add00 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
pub fn stub_0x3add00() -> ! {
    todo!("0x3add00 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")
}

// 0x3ade10 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost8functionIS5_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
pub fn stub_0x3ade10() -> ! {
    todo!("0x3ade10 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")
}

// 0x3adf40 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3adf40() -> ! {
    todo!("0x3adf40 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3adf48 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_E4callES5_ff
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3adf48() -> ! {
    todo!("0x3adf48 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3adf50 — __ZNK5boost9function3IvN3G3D7Vector34AxisEffEclES3_ff
// type: void __fastcall(_DWORD *, int, int, int)
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")]
pub fn stub_0x3adf50() -> ! {
    todo!("0x3adf50 boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")
}

// 0x3ae028 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_0x3ae028() -> ! {
    todo!("0x3ae028 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")
}

// 0x3ae138 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost8functionIS6_EELi3ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_0x3ae138() -> ! {
    todo!("0x3ae138 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")
}

// 0x3ae268 — __ZN5boost9function3IvN3G3D7Vector34AxisEffE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")]
pub fn stub_0x3ae268() -> ! {
    todo!("0x3ae268 boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")
}

// 0x3ae298 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_EC2ESA_PKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3ae298() -> ! {
    todo!("0x3ae298 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3ae4f4 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")]
pub fn stub_0x3ae4f4() -> ! {
    todo!("0x3ae4f4 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")
}

// 0x3ae518 — __ZN3RBX10Reflection9EventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEMS2_S9_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")]
pub fn stub_0x3ae518() -> ! {
    todo!("0x3ae518 RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")
}

// 0x3ae5cc — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")]
pub fn stub_0x3ae5cc() -> ! {
    todo!("0x3ae5cc RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")
}

// 0x3ae680 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3ae680() -> ! {
    todo!("0x3ae680 RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3ae7e4 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isScriptable(void)const")]
pub fn stub_0x3ae7e4() -> ! {
    todo!("0x3ae7e4 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isScriptable(void)const")
}

// 0x3ae7ec — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isBroadcast(void)const")]
pub fn stub_0x3ae7ec() -> ! {
    todo!("0x3ae7ec RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isBroadcast(void)const")
}

// 0x3ae7f4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ae7f4() -> ! {
    todo!("0x3ae7f4 RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ae880 — __ZNK3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ae880() -> ! {
    todo!("0x3ae880 RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3ae890 — __ZNK3RBX10Reflection13EventDescBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEMS2_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3ae890() -> ! {
    todo!("0x3ae890 RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3ae8a4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
pub fn stub_0x3ae8a4() -> ! {
    todo!("0x3ae8a4 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x3ae9c0 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN3G3D7Vector34AxisEEEvRKT_
// type: int __fastcall(int, void (__fastcall ***)(int))
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")]
pub fn stub_0x3ae9c0() -> ! {
    todo!("0x3ae9c0 void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")
}

// 0x3aeb04 — __ZN5boost9function1IvN3G3D7Vector34AxisEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::clear(void)")]
pub fn stub_0x3aeb04() -> ! {
    todo!("0x3aeb04 boost::function1<void,G3D::Vector3::Axis>::clear(void)")
}

// 0x3aeb30 — __ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3aeb30() -> ! {
    todo!("0x3aeb30 __ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x3aec14 — __ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3aec14() -> ! {
    todo!("0x3aec14 __ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x3aecfc — __ZN5boost9function1IvN3G3D7Vector34AxisEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
pub fn stub_0x3aecfc() -> ! {
    todo!("0x3aecfc void boost::function1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x3aedf4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3aedf4() -> ! {
    todo!("0x3aedf4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3aee10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector34AxisEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3::Axis>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis)")]
pub fn stub_0x3aee10() -> ! {
    todo!("0x3aee10 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3::Axis>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis)")
}
