//! network generated — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 150 stubs here).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
pub fn stub_3a7f68() -> ! {
    todo!("0x3a7f68 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")
}

// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
pub fn stub_3a80c8() -> ! {
    todo!("0x3a80c8 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")
}

// 0x3a8228 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_3a8228() -> ! {
    todo!("0x3a8228 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3a8288 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_3a8288() -> ! {
    todo!("0x3a8288 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3a98d0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
pub fn stub_3a98d0() -> ! {
    todo!("0x3a98d0 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")
}

// 0x3a9944 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
pub fn stub_3a9944() -> ! {
    todo!("0x3a9944 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")
}

// 0x3a9990 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
pub fn stub_3a9990() -> ! {
    todo!("0x3a9990 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")
}

// 0x3a99bc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
pub fn stub_3a99bc() -> ! {
    todo!("0x3a99bc rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")
}

// 0x3a9a90 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9a90() -> ! {
    todo!("0x3a9a90 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9a98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9a98() -> ! {
    todo!("0x3a9a98 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9aa0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
pub fn stub_3a9aa0() -> ! {
    todo!("0x3a9aa0 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")
}

// 0x3a9ab8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9ab8() -> ! {
    todo!("0x3a9ab8 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")
}

// 0x3a9ae4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9ae4() -> ! {
    todo!("0x3a9ae4 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")
}

// 0x3a9bb8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
pub fn stub_3a9bb8() -> ! {
    todo!("0x3a9bb8 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")
}

// 0x3a9c2c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
pub fn stub_3a9c2c() -> ! {
    todo!("0x3a9c2c RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")
}

// 0x3a9c78 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
pub fn stub_3a9c78() -> ! {
    todo!("0x3a9c78 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")
}

// 0x3a9ca4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
pub fn stub_3a9ca4() -> ! {
    todo!("0x3a9ca4 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")
}

// 0x3a9d78 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9d78() -> ! {
    todo!("0x3a9d78 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9d80 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_3a9d80() -> ! {
    todo!("0x3a9d80 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9d88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
pub fn stub_3a9d88() -> ! {
    todo!("0x3a9d88 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")
}

// 0x3a9da0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9da0() -> ! {
    todo!("0x3a9da0 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")
}

// 0x3a9dcc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_3a9dcc() -> ! {
    todo!("0x3a9dcc rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")
}

// 0x3aa448 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_3aa448() -> ! {
    todo!("0x3aa448 RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")
}

// 0x3aa5a4 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff
#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
pub fn stub_3aa5a4() -> ! {
    todo!("0x3aa5a4 RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")
}

// 0x3aa764 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_3aa764() -> ! {
    todo!("0x3aa764 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0x3aaa08 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_3aaa08() -> ! {
    todo!("0x3aaa08 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x3aaa34 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_3aaa34() -> ! {
    todo!("0x3aaa34 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x3aac24 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_3aac24() -> ! {
    todo!("0x3aac24 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3aac50 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_E4callES5_ff
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_3aac50() -> ! {
    todo!("0x3aac50 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")
}

// 0x3aac7c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_3aac7c() -> ! {
    todo!("0x3aac7c void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")
}

// 0x3aafa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_3aafa0() -> ! {
    todo!("0x3aafa0 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")
}

// 0x3aafcc — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSE_10ArcHandlesES6_EES5_ffEENSA_5list4INSA_5valueIPSH_EENS9_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEELi3ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_3aafcc() -> ! {
    todo!("0x3aafcc rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")
}

// 0x3ab0a0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_3ab0a0() -> ! {
    todo!("0x3ab0a0 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")
}

// 0x3ab0a4 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_3ab0a4() -> ! {
    todo!("0x3ab0a4 RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")
}

// 0x3ab200 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
pub fn stub_3ab200() -> ! {
    todo!("0x3ab200 RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")
}

// 0x3ab360 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
pub fn stub_3ab360() -> ! {
    todo!("0x3ab360 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")
}

// 0x3ab604 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_3ab604() -> ! {
    todo!("0x3ab604 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3ab630 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_3ab630() -> ! {
    todo!("0x3ab630 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3ab820 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_3ab820() -> ! {
    todo!("0x3ab820 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")
}

// 0x3ab834 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_3ab834() -> ! {
    todo!("0x3ab834 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")
}

// 0x3ab848 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(char **, int *)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
pub fn stub_3ab848() -> ! {
    todo!("0x3ab848 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")
}

// 0x3abb44 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_3abb44() -> ! {
    todo!("0x3abb44 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")
}

// 0x3abb70 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_3abb70() -> ! {
    todo!("0x3abb70 rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")
}

// 0x3abc44 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_3abc44() -> ! {
    todo!("0x3abc44 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")
}

// 0x3b05bc — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
pub fn stub_3b05bc() -> ! {
    todo!("0x3b05bc RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")
}

// 0x3b06ec — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
pub fn stub_3b06ec() -> ! {
    todo!("0x3b06ec RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")
}

// 0x52d620 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
pub fn stub_52d620() -> ! {
    todo!("0x52d620 RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")
}

// 0x52d780 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
pub fn stub_52d780() -> ! {
    todo!("0x52d780 RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")
}

// 0x52d9c4 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52d9c4() -> ! {
    todo!("0x52d9c4 RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x52da24 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52da24() -> ! {
    todo!("0x52da24 RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x52e250 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
pub fn stub_52e250() -> ! {
    todo!("0x52e250 RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")
}

// 0x52e3b0 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
pub fn stub_52e3b0() -> ! {
    todo!("0x52e3b0 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")
}

// 0x52e510 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52e510() -> ! {
    todo!("0x52e510 RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x52e570 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_52e570() -> ! {
    todo!("0x52e570 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x52ee40 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
pub fn stub_52ee40() -> ! {
    todo!("0x52ee40 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")
}

// 0x52eeb4 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
pub fn stub_52eeb4() -> ! {
    todo!("0x52eeb4 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")
}

// 0x52ef00 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_52ef00() -> ! {
    todo!("0x52ef00 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")
}

// 0x52ef2c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_52ef2c() -> ! {
    todo!("0x52ef2c rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>::~callable_slot()")
}

// 0x52f000 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f000() -> ! {
    todo!("0x52f000 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")
}

// 0x52f008 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f008() -> ! {
    todo!("0x52f008 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")
}

// 0x52f010 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
pub fn stub_52f010() -> ! {
    todo!("0x52f010 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")
}

// 0x52f028 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f028() -> ! {
    todo!("0x52f028 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")
}

// 0x52f054 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f054() -> ! {
    todo!("0x52f054 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")
}

// 0x52f128 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
pub fn stub_52f128() -> ! {
    todo!("0x52f128 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")
}

// 0x52f19c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
pub fn stub_52f19c() -> ! {
    todo!("0x52f19c RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")
}

// 0x52f1e8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_52f1e8() -> ! {
    todo!("0x52f1e8 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")
}

// 0x52f214 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_52f214() -> ! {
    todo!("0x52f214 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")
}

// 0x52f2e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f2e8() -> ! {
    todo!("0x52f2e8 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")
}

// 0x52f2f0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_52f2f0() -> ! {
    todo!("0x52f2f0 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")
}

// 0x52f2f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
pub fn stub_52f2f8() -> ! {
    todo!("0x52f2f8 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")
}

// 0x52f310 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f310() -> ! {
    todo!("0x52f310 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")
}

// 0x52f33c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_52f33c() -> ! {
    todo!("0x52f33c rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")
}

// 0x52f55c — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_52f55c() -> ! {
    todo!("0x52f55c RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")
}

// 0x52f6b8 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiButtonEFviiEE25signalProducedIncrementedEii
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::signalProducedIncremented(int,int)")]
pub fn stub_52f6b8() -> ! {
    todo!("0x52f6b8 RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>::signalProducedIncremented(int,int)")
}

// 0x52f83c — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_52f83c() -> ! {
    todo!("0x52f83c rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x52fae0 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_52fae0() -> ! {
    todo!("0x52fae0 rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x52fb0c — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_52fb0c() -> ! {
    todo!("0x52fb0c rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x52fcfc — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_52fcfc() -> ! {
    todo!("0x52fcfc rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")
}

// 0x52fd24 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_52fd24() -> ! {
    todo!("0x52fd24 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")
}

// 0x52fd4c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
pub fn stub_52fd4c() -> ! {
    todo!("0x52fd4c void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")
}

// 0x530058 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_530058() -> ! {
    todo!("0x530058 rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")
}

// 0x530084 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiButtonES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_530084() -> ! {
    todo!("0x530084 rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")
}

// 0x530158 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_530158() -> ! {
    todo!("0x530158 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::connectSignalListener(void)")
}

// 0x53015c — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
pub fn stub_53015c() -> ! {
    todo!("0x53015c RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::connectSignalListener(void)")
}

// 0x5302b8 — __ZN3RBX19EventReplicatorImplILi0ENS_9GuiButtonEFvvEE25signalProducedIncrementedEv
#[doc(alias = "RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::signalProducedIncremented(void)")]
pub fn stub_5302b8() -> ! {
    todo!("0x5302b8 RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>::signalProducedIncremented(void)")
}

// 0x5303f0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")]
pub fn stub_5303f0() -> ! {
    todo!("0x5303f0 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")
}

// 0x530464 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_530464() -> ! {
    todo!("0x530464 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")
}

// 0x530490 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")]
pub fn stub_530490() -> ! {
    todo!("0x530490 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>::~callable_slot()")
}

// 0x530564 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_530564() -> ! {
    todo!("0x530564 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")
}

// 0x53056c — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_53056c() -> ! {
    todo!("0x53056c `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::call(void)")
}

// 0x530574 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
pub fn stub_530574() -> ! {
    todo!("0x530574 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>::operator()(void)")
}

// 0x53058c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_53058c() -> ! {
    todo!("0x53058c rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")
}

// 0x5305b8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSB_9GuiButtonES3_EEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5305b8() -> ! {
    todo!("0x5305b8 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>,0,void ()(void)>::~callable()")
}

// 0x53068c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::connectSignalListener(void)")]
pub fn stub_53068c() -> ! {
    todo!("0x53068c RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::connectSignalListener(void)")
}

// 0x533bfc — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
pub fn stub_533bfc() -> ! {
    todo!("0x533bfc rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")
}

// 0x533c70 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
pub fn stub_533c70() -> ! {
    todo!("0x533c70 RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")
}

// 0x533cbc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
pub fn stub_533cbc() -> ! {
    todo!("0x533cbc rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")
}

// 0x533ce8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")]
pub fn stub_533ce8() -> ! {
    todo!("0x533ce8 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>::~callable_slot()")
}

// 0x533dbc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_533dbc() -> ! {
    todo!("0x533dbc rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")
}

// 0x533dc4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_533dc4() -> ! {
    todo!("0x533dc4 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::call(void)")
}

// 0x533dcc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
pub fn stub_533dcc() -> ! {
    todo!("0x533dcc boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")
}

// 0x533de4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_533de4() -> ! {
    todo!("0x533de4 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")
}

// 0x533e10 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFvNSB_5UDim2EEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_533e10() -> ! {
    todo!("0x533e10 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>,0,void ()(void)>::~callable()")
}

// 0x533ee4 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
pub fn stub_533ee4() -> ! {
    todo!("0x533ee4 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")
}

// 0x533f58 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")]
pub fn stub_533f58() -> ! {
    todo!("0x533f58 RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")
}

// 0x533fa4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_533fa4() -> ! {
    todo!("0x533fa4 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")
}

// 0x533fd0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")]
pub fn stub_533fd0() -> ! {
    todo!("0x533fd0 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>::~callable_slot()")
}

// 0x5340a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_5340a4() -> ! {
    todo!("0x5340a4 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")
}

// 0x5340ac — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_5340ac() -> ! {
    todo!("0x5340ac `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::call(void)")
}

// 0x5340b4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")]
pub fn stub_5340b4() -> ! {
    todo!("0x5340b4 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>::operator()(void)")
}

// 0x5340cc — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5340cc() -> ! {
    todo!("0x5340cc rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")
}

// 0x5340f8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_9GuiObjectEFviiEEEEENS7_5list1INS7_5valueIPSF_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5340f8() -> ! {
    todo!("0x5340f8 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>,0,void ()(void)>::~callable()")
}

// 0x537740 — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
pub fn stub_537740() -> ! {
    todo!("0x537740 RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")
}

// 0x53789c — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)")]
pub fn stub_53789c() -> ! {
    todo!("0x53789c RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)")
}

// 0x537a18 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
pub fn stub_537a18() -> ! {
    todo!("0x537a18 rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")
}

// 0x537cbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_537cbc() -> ! {
    todo!("0x537cbc rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x537ce8 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_537ce8() -> ! {
    todo!("0x537ce8 rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x537ed8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
pub fn stub_537ed8() -> ! {
    todo!("0x537ed8 rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")
}

// 0x537f00 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
pub fn stub_537f00() -> ! {
    todo!("0x537f00 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")
}

// 0x537f28 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
pub fn stub_537f28() -> ! {
    todo!("0x537f28 void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")
}

// 0x538240 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
pub fn stub_538240() -> ! {
    todo!("0x538240 rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")
}

// 0x53826c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
pub fn stub_53826c() -> ! {
    todo!("0x53826c rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")
}

// 0x538340 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
// type: void()
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
pub fn stub_538340() -> ! {
    todo!("0x538340 RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")
}

// 0x538344 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_538344() -> ! {
    todo!("0x538344 RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")
}

// 0x5384a0 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)")]
pub fn stub_5384a0() -> ! {
    todo!("0x5384a0 RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)")
}

// 0x538624 — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_538624() -> ! {
    todo!("0x538624 rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x538698 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_538698() -> ! {
    todo!("0x538698 rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x5386c4 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_5386c4() -> ! {
    todo!("0x5386c4 rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x538798 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_538798() -> ! {
    todo!("0x538798 rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")
}

// 0x5387c0 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
pub fn stub_5387c0() -> ! {
    todo!("0x5387c0 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")
}

// 0x5387e8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
pub fn stub_5387e8() -> ! {
    todo!("0x5387e8 void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")
}

// 0x538810 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_538810() -> ! {
    todo!("0x538810 rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")
}

// 0x53883c — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
pub fn stub_53883c() -> ! {
    todo!("0x53883c rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")
}

// 0x538910 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
pub fn stub_538910() -> ! {
    todo!("0x538910 RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")
}

// 0x53ffec — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")]
pub fn stub_53ffec() -> ! {
    todo!("0x53ffec RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")
}

// 0x54011c — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")]
pub fn stub_54011c() -> ! {
    todo!("0x54011c RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")
}

// 0x567750 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")]
pub fn stub_567750() -> ! {
    todo!("0x567750 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::setListenerMode(bool)")
}

// 0x5678b0 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")]
pub fn stub_5678b0() -> ! {
    todo!("0x5678b0 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::setListenerMode(bool)")
}

// 0x567a10 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_567a10() -> ! {
    todo!("0x567a10 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x567a70 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_567a70() -> ! {
    todo!("0x567a70 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x568e90 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
pub fn stub_568e90() -> ! {
    todo!("0x568e90 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")
}

// 0x568f04 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")]
pub fn stub_568f04() -> ! {
    todo!("0x568f04 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::listenerConnectionAdded(void)")
}

// 0x568f50 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
pub fn stub_568f50() -> ! {
    todo!("0x568f50 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")
}

// 0x568f7c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
pub fn stub_568f7c() -> ! {
    todo!("0x568f7c rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")
}

// 0x569050 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_569050() -> ! {
    todo!("0x569050 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x569058 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_569058() -> ! {
    todo!("0x569058 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x569060 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
pub fn stub_569060() -> ! {
    todo!("0x569060 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")
}

// 0x569078 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_569078() -> ! {
    todo!("0x569078 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")
}

// 0x5690a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_5690a4() -> ! {
    todo!("0x5690a4 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")
}

// 0x569178 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
pub fn stub_569178() -> ! {
    todo!("0x569178 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")
}

// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_33454() -> ! {
    todo!("0x33454 boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3346c() -> ! {
    todo!("0x3346c boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3c9c4c — __ZN3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")]
pub fn stub_3c9c4c() -> ! {
    todo!("0x3c9c4c RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(RBX::Instance const*)")
}

// 0x3f1114 — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
pub fn stub_3f1114() -> ! {
    todo!("0x3f1114 RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")
}

// 0x3f1234 — __ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE
// type: int __fastcall(int *, float, int, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isClickable(boost::shared_ptr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")]
pub fn stub_3f1234() -> ! {
    todo!("0x3f1234 RBX::ClickDetector::isClickable(boost::shared_ptr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")
}

// 0x3f12e0 — __ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *, int *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::updateLastHoverPart(boost::shared_ptr<RBX::Instance>,RBX::Network::Player *)")]
pub fn stub_3f12e0() -> ! {
    todo!("0x3f12e0 RBX::ClickDetector::updateLastHoverPart(boost::shared_ptr<RBX::Instance>,RBX::Network::Player *)")
}

// 0x3f130c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
pub fn stub_3f130c() -> ! {
    todo!("0x3f130c RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")
}

// 0x3f1410 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
pub fn stub_3f1410() -> ! {
    todo!("0x3f1410 RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")
}

// 0x3f154c — __ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE
// type: void __fastcall(int *, RBX::Network::Player *, int, int)
#[doc(alias = "RBX::ClickDetector::stopHover(boost::shared_ptr<RBX::PartInstance>,RBX::Network::Player *)")]
pub fn stub_3f154c() -> ! {
    todo!("0x3f154c RBX::ClickDetector::stopHover(boost::shared_ptr<RBX::PartInstance>,RBX::Network::Player *)")
}

// 0x3f15b8 — __ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *this, RBX::PartInstance *, float, RBX::Network::Player *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")]
pub fn stub_3f15b8() -> ! {
    todo!("0x3f15b8 RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")
}

// 0x3f7df0 — __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
pub fn stub_3f7df0() -> ! {
    todo!("0x3f7df0 RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")
}

// 0x3f7df4 — __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
pub fn stub_3f7df4() -> ! {
    todo!("0x3f7df4 RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")
}

// 0x3f7f80 — __ZN3RBX19NetworkStatsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_3f7f80() -> ! {
    todo!("0x3f7f80 RBX::NetworkStatsCommand::doIt(RBX::IDataState *)")
}

// 0x3f8268 — __ZNK3RBX19NetworkStatsCommand9isEnabledEv
// type: bool __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isEnabled(void)const")]
pub fn stub_3f8268() -> ! {
    todo!("0x3f8268 RBX::NetworkStatsCommand::isEnabled(void)const")
}

// 0x3f83e4 — __ZNK3RBX19NetworkStatsCommand9isCheckedEv
// type: int __fastcall(RBX::NetworkStatsCommand *this)
#[doc(alias = "RBX::NetworkStatsCommand::isChecked(void)const")]
pub fn stub_3f83e4() -> ! {
    todo!("0x3f83e4 RBX::NetworkStatsCommand::isChecked(void)const")
}

// 0x3fe628 — __ZN3RBX19NetworkStatsCommandD1Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
pub fn stub_3fe628() -> ! {
    todo!("0x3fe628 RBX::NetworkStatsCommand::~NetworkStatsCommand()")
}

// 0x3fe62c — __ZN3RBX19NetworkStatsCommandD0Ev
// type: void __fastcall(RBX::NetworkStatsCommand *__hidden this)
#[doc(alias = "RBX::NetworkStatsCommand::~NetworkStatsCommand()")]
pub fn stub_3fe62c() -> ! {
    todo!("0x3fe62c RBX::NetworkStatsCommand::~NetworkStatsCommand()")
}

// 0x425d58 — __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
pub fn stub_425d58() -> ! {
    todo!("0x425d58 RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")
}

// 0x44ab28 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(boost::shared_ptr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
pub fn stub_44ab28() -> ! {
    todo!("0x44ab28 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(boost::shared_ptr<RBX::Network::Players> const*,RBX::Network::Players *)const")
}

// 0x44ac18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44ac18() -> ! {
    todo!("0x44ac18 boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4f1df8 — __ZN3RBX4Flag21canBePickedUpByPlayerEPNS_7Network6PlayerE
#[doc(alias = "RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player *)")]
pub fn stub_4f1df8() -> ! {
    todo!("0x4f1df8 RBX::Flag::canBePickedUpByPlayer(RBX::Network::Player *)")
}

// 0x5e1de8 — __ZN3RBX7Network12NetworkOwner16ServerUnassignedEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
#[doc(alias = "RBX::Network::NetworkOwner::ServerUnassigned(void)")]
pub fn stub_5e1de8() -> ! {
    todo!("0x5e1de8 RBX::Network::NetworkOwner::ServerUnassigned(void)")
}

// 0x5e1e40 — __ZN3RBX7Network12NetworkOwner16colorFromAddressERKNS_13SystemAddressE
// type: int(void)
#[doc(alias = "RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")]
pub fn stub_5e1e40() -> ! {
    todo!("0x5e1e40 RBX::Network::NetworkOwner::colorFromAddress(RBX::SystemAddress const&)")
}

// 0x5e1eac — __ZN3RBX7Network12NetworkOwner8isClientERKNS_13SystemAddressE
// type: int(void)
#[doc(alias = "RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")]
pub fn stub_5e1eac() -> ! {
    todo!("0x5e1eac RBX::Network::NetworkOwner::isClient(RBX::SystemAddress const&)")
}

// 0x5e1ef8 — __ZN3RBX7Network12NetworkOwner6ServerEv
// type: _DWORD __fastcall(RBX::Network::NetworkOwner *__hidden this)
#[doc(alias = "RBX::Network::NetworkOwner::Server(void)")]
pub fn stub_5e1ef8() -> ! {
    todo!("0x5e1ef8 RBX::Network::NetworkOwner::Server(void)")
}

// 0x5f6978 — __ZN3RBX19PhysicsInstructions22changeSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "RBX::PhysicsInstructions::changeSimulationRadius(RBX::Network::Player *,float)")]
pub fn stub_5f6978() -> ! {
    todo!("0x5f6978 RBX::PhysicsInstructions::changeSimulationRadius(RBX::Network::Player *,float)")
}

// 0x5f69ec — __ZN3RBX19PhysicsInstructions25changeMaxSimulationRadiusEPNS_7Network6PlayerEf
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, float)
#[doc(alias = "RBX::PhysicsInstructions::changeMaxSimulationRadius(RBX::Network::Player *,float)")]
pub fn stub_5f69ec() -> ! {
    todo!("0x5f69ec RBX::PhysicsInstructions::changeMaxSimulationRadius(RBX::Network::Player *,float)")
}

// 0x5f6a90 — __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
// type: _DWORD __fastcall(RBX::PhysicsInstructions *__hidden this, RBX::Network::Player *, RBX::Workspace *, double, double)
#[doc(alias = "RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")]
pub fn stub_5f6a90() -> ! {
    todo!("0x5f6a90 RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")
}

// 0x63df08 — __ZN3RBX14SpawnerService16GetSpawnLocationEPNS_7Network6PlayerESs
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::SpawnerService::GetSpawnLocation(RBX::Network::Player *,std::string)")]
pub fn stub_63df08() -> ! {
    todo!("0x63df08 RBX::SpawnerService::GetSpawnLocation(RBX::Network::Player *,std::string)")
}

// 0x664a54 — __ZN3RBX5Teams21assignNewPlayerToTeamEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Teams::assignNewPlayerToTeam(RBX::Network::Player *)")]
pub fn stub_664a54() -> ! {
    todo!("0x664a54 RBX::Teams::assignNewPlayerToTeam(RBX::Network::Player *)")
}

// 0x664c9c — __ZN3RBX5Teams17getTeamFromPlayerEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Teams *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Teams::getTeamFromPlayer(RBX::Network::Player *)")]
pub fn stub_664c9c() -> ! {
    todo!("0x664c9c RBX::Teams::getTeamFromPlayer(RBX::Network::Player *)")
}

// 0x68052c — __ZN3RBX4Tool7dropAllEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Tool::dropAll(RBX::Network::Player *)")]
pub fn stub_68052c() -> ! {
    todo!("0x68052c RBX::Tool::dropAll(RBX::Network::Player *)")
}

// 0x68057c — __ZN3RBX4Tool22moveAllToolsToBackpackEPNS_7Network6PlayerE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Tool::moveAllToolsToBackpack(RBX::Network::Player *)")]
pub fn stub_68057c() -> ! {
    todo!("0x68057c RBX::Tool::moveAllToolsToBackpack(RBX::Network::Player *)")
}

// 0x681fd8 — __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE
// type: int __fastcall(int, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Tool::setTimerCallback(boost::weak_ptr<RBX::Network::Player>)")]
pub fn stub_681fd8() -> ! {
    todo!("0x681fd8 RBX::Tool::setTimerCallback(boost::weak_ptr<RBX::Network::Player>)")
}

// 0x682190 — __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Tool::moveOtherToolsToBackpack(boost::weak_ptr<RBX::Network::Player>)")]
pub fn stub_682190() -> ! {
    todo!("0x682190 RBX::Tool::moveOtherToolsToBackpack(boost::weak_ptr<RBX::Network::Player>)")
}

// 0x682e2c — __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>,RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>(void (RBX::Tool::*)(boost::weak_ptr<RBX::Network::Player>),RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)")]
pub fn stub_682e2c() -> ! {
    todo!("0x682e2c boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>,RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>(void (RBX::Tool::*)(boost::weak_ptr<RBX::Network::Player>),RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)")
}

// 0x683034 — __ZN3RBX4Tool21canBePickedUpByPlayerEPNS_7Network6PlayerE
#[doc(alias = "RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)")]
pub fn stub_683034() -> ! {
    todo!("0x683034 RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)")
}

// 0x683ee0 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "boost::shared_ptr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(boost::weak_ptr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_683ee0() -> ! {
    todo!("0x683ee0 boost::shared_ptr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(boost::weak_ptr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")
}

// 0x684130 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>)")]
pub fn stub_684130() -> ! {
    todo!("0x684130 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>)")
}

// 0x68422c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_68422c() -> ! {
    todo!("0x68422c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x684248 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_684248() -> ! {
    todo!("0x684248 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x684260 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_684260() -> ! {
    todo!("0x684260 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")
}

// 0x68434c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_68434c() -> ! {
    todo!("0x68434c bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x684434 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_684434() -> ! {
    todo!("0x684434 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x68450c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
pub fn stub_68450c() -> ! {
    todo!("0x68450c void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")
}

// 0x6845e0 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>::operator()(RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)const")]
pub fn stub_6845e0() -> ! {
    todo!("0x6845e0 boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>::operator()(RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)const")
}

// 0x6846c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_6846c8() -> ! {
    todo!("0x6846c8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x684824 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>)")]
pub fn stub_684824() -> ! {
    todo!("0x684824 boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>)")
}

// 0x6d1a38 — __ZN3RBX7Network7Players11getGameModeEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getGameMode(RBX::Instance const*)")]
pub fn stub_6d1a38() -> ! {
    todo!("0x6d1a38 RBX::Network::Players::getGameMode(RBX::Instance const*)")
}

// 0x79d5a8 — __ZN3RBX14PlayerChatLineC2ENS_8ChatLine8ChatTypeEN5boost10shared_ptrINS_7Network6PlayerEEERKSsfb
// type: RBX::ChatLine *__fastcall(RBX::ChatLine *, int, RBX::Instance **, std::string *, int, int)
#[doc(alias = "RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,boost::shared_ptr<RBX::Network::Player>,std::string const&,float,bool)")]
pub fn stub_79d5a8() -> ! {
    todo!("0x79d5a8 RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,boost::shared_ptr<RBX::Network::Player>,std::string const&,float,bool)")
}

// 0x7a0ee4 — __ZN3RBX10ChatOutput19onPlayerChatMessageERKNS_7Network11ChatMessageE
// type: void __fastcall(RBX::ChatOutput *this, const RBX::Network::ChatMessage *)
#[doc(alias = "RBX::ChatOutput::onPlayerChatMessage(RBX::Network::ChatMessage const&)")]
pub fn stub_7a0ee4() -> ! {
    todo!("0x7a0ee4 RBX::ChatOutput::onPlayerChatMessage(RBX::Network::ChatMessage const&)")
}

// 0x7a3bbc — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
pub fn stub_7a3bbc() -> ! {
    todo!("0x7a3bbc rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")
}

// 0x7a8b34 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
pub fn stub_7a8b34() -> ! {
    todo!("0x7a8b34 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")
}

// 0x7a8d40 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")]
pub fn stub_7a8d40() -> ! {
    todo!("0x7a8d40 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")
}

// 0x7a8d64 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_7a8d64() -> ! {
    todo!("0x7a8d64 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_do_get_mutex(void)")
}

// 0x7a8e5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7a8e5c() -> ! {
    todo!("0x7a8e5c rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x7a8e88 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_7a8e88() -> ! {
    todo!("0x7a8e88 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x7a8f5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::disconnect(void)")]
pub fn stub_7a8f5c() -> ! {
    todo!("0x7a8f5c rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::disconnect(void)")
}

// 0x7a906c — __ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::connected(void)const")]
pub fn stub_7a906c() -> ! {
    todo!("0x7a906c rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::connected(void)const")
}

// 0x7a9078 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
pub fn stub_7a9078() -> ! {
    todo!("0x7a9078 rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")
}

// 0x7a9080 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
pub fn stub_7a9080() -> ! {
    todo!("0x7a9080 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")
}

// 0x7a9088 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")]
pub fn stub_7a9088() -> ! {
    todo!("0x7a9088 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")
}

// 0x7a90a0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
pub fn stub_7a90a0() -> ! {
    todo!("0x7a90a0 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")
}

// 0x7a9190 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_7a9190() -> ! {
    todo!("0x7a9190 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_init_mutex(void)")
}

// 0x7a9194 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_7a9194() -> ! {
    todo!("0x7a9194 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_do_get_mutex(void)")
}

// 0x7a9284 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")]
pub fn stub_7a9284() -> ! {
    todo!("0x7a9284 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")
}

// 0x7a92b0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")]
pub fn stub_7a92b0() -> ! {
    todo!("0x7a92b0 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")
}

// 0x7a9384 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
pub fn stub_7a9384() -> ! {
    todo!("0x7a9384 rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")
}

// 0x7a93b0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
pub fn stub_7a93b0() -> ! {
    todo!("0x7a93b0 rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")
}

// 0x7aac38 — __ZN3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")]
pub fn stub_7aac38() -> ! {
    todo!("0x7aac38 RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")
}

// 0x8922e8 — __ZN3RBX21PersonalServerService7getRankEPNS_7Network6PlayerEiN5boost8functionIFvSsEEES7_
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::PersonalServerService::getRank(RBX::Network::Player *,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_8922e8() -> ! {
    todo!("0x8922e8 RBX::PersonalServerService::getRank(RBX::Network::Player *,int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x892534 — __ZN3RBX21PersonalServerService7setRankEPNS_7Network6PlayerEiiN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::PersonalServerService::setRank(RBX::Network::Player *,int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_892534() -> ! {
    todo!("0x892534 RBX::PersonalServerService::setRank(RBX::Network::Player *,int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x8e61c8 — __ZN3RBX20ContextActionService27setupLocalPlayerConnectionsEPNS_7Network6PlayerE
// type: void __fastcall(int32_t **this, RBX::Network::Player *)
#[doc(alias = "RBX::ContextActionService::setupLocalPlayerConnections(RBX::Network::Player *)")]
pub fn stub_8e61c8() -> ! {
    todo!("0x8e61c8 RBX::ContextActionService::setupLocalPlayerConnections(RBX::Network::Player *)")
}

// 0x9038d0 — __ZNK3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(void)const")]
pub fn stub_9038d0() -> ! {
    todo!("0x9038d0 RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(void)const")
}

// 0x903c18 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_7Network7PlayersEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Network::Players>(void)")]
pub fn stub_903c18() -> ! {
    todo!("0x903c18 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Network::Players>(void)")
}

// 0x94f8a0 — __ZN3RBX7Network23TopNErrorsPhysicsSenderC1ERNS0_10ReplicatorE
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::TopNErrorsPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_94f8a0() -> ! {
    todo!("0x94f8a0 RBX::Network::TopNErrorsPhysicsSender::TopNErrorsPhysicsSender(RBX::Network::Replicator &)")
}

// 0x94f8ac — __ZN3RBX7Network23TopNErrorsPhysicsSenderC2ERNS0_10ReplicatorE
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::TopNErrorsPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_94f8ac() -> ! {
    todo!("0x94f8ac RBX::Network::TopNErrorsPhysicsSender::TopNErrorsPhysicsSender(RBX::Network::Replicator &)")
}

// 0x94ff68 — __ZN3RBX7Network23TopNErrorsPhysicsSenderD0Ev
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")]
pub fn stub_94ff68() -> ! {
    todo!("0x94ff68 RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")
}

// 0x950008 — __ZN3RBX7Network23TopNErrorsPhysicsSenderD1Ev
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")]
pub fn stub_950008() -> ! {
    todo!("0x950008 RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")
}

// 0x950014 — __ZN3RBX7Network23TopNErrorsPhysicsSenderD2Ev
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")]
pub fn stub_950014() -> ! {
    todo!("0x950014 RBX::Network::TopNErrorsPhysicsSender::~TopNErrorsPhysicsSender()")
}

// 0x9501c8 — __ZN3RBX7Network23TopNErrorsPhysicsSender4stepEv
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::step(void)")]
pub fn stub_9501c8() -> ! {
    todo!("0x9501c8 RBX::Network::TopNErrorsPhysicsSender::step(void)")
}

// 0x950fb4 — __ZN3RBX7Network23TopNErrorsPhysicsSender9addNuggetERNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::addNugget(RBX::PartInstance &)")]
pub fn stub_950fb4() -> ! {
    todo!("0x950fb4 RBX::Network::TopNErrorsPhysicsSender::addNugget(RBX::PartInstance &)")
}

// 0x9511c8 — __ZN3RBX7Network23TopNErrorsPhysicsSender16onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::onAddingAssembly(boost::shared_ptr<RBX::Instance>)")]
pub fn stub_9511c8() -> ! {
    todo!("0x9511c8 RBX::Network::TopNErrorsPhysicsSender::onAddingAssembly(boost::shared_ptr<RBX::Instance>)")
}

// 0x9514c4 — __ZN3RBX7Network23TopNErrorsPhysicsSender10addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::addNugget2(boost::shared_ptr<RBX::PartInstance>)")]
pub fn stub_9514c4() -> ! {
    todo!("0x9514c4 RBX::Network::TopNErrorsPhysicsSender::addNugget2(boost::shared_ptr<RBX::PartInstance>)")
}

// 0x952b38 — __ZN3RBX7Network23TopNErrorsPhysicsSender6Nugget12computeErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
// type: _DWORD __fastcall(RBX::Network::TopNErrorsPhysicsSender::Nugget *__hidden this, const G3D::CoordinateFrame *, const RBX::ModelInstance *, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
pub fn stub_952b38() -> ! {
    todo!("0x952b38 RBX::Network::TopNErrorsPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")
}

// 0x952d9c — __ZN3RBX7Network23TopNErrorsPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
pub fn stub_952d9c() -> ! {
    todo!("0x952d9c RBX::Network::TopNErrorsPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")
}

// 0x953b7c — __ZN3RBX7Network23TopNErrorsPhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::TopNErrorsPhysicsSender *this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::TopNErrorsPhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_953b7c() -> ! {
    todo!("0x953b7c RBX::Network::TopNErrorsPhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")
}

// 0x953e68 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23TopNErrorsPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_953e68() -> ! {
    todo!("0x953e68 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>)")
}

// 0x953edc — __ZNSt6vectorIPN3RBX7Network23TopNErrorsPhysicsSender6NuggetESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::TopNErrorsPhysicsSender::Nugget **,std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>>,RBX::Network::TopNErrorsPhysicsSender::Nugget * const&)")]
pub fn stub_953edc() -> ! {
    todo!("0x953edc std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::TopNErrorsPhysicsSender::Nugget **,std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>>,RBX::Network::TopNErrorsPhysicsSender::Nugget * const&)")
}

// 0x953fd4 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
// type: int __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
pub fn stub_953fd4() -> ! {
    todo!("0x953fd4 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")
}

// 0x9541dc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
pub fn stub_9541dc() -> ! {
    todo!("0x9541dc void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")
}

// 0x954450 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_954450() -> ! {
    todo!("0x954450 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")
}

// 0x9545f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_9545f8() -> ! {
    todo!("0x9545f8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")
}

// 0x9546ac — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)")]
pub fn stub_9546ac() -> ! {
    todo!("0x9546ac boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)")
}

// 0x954884 — __ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")]
pub fn stub_954884() -> ! {
    todo!("0x954884 boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")
}

// 0x954b00 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network23TopNErrorsPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_954b00() -> ! {
    todo!("0x954b00 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x954b5c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network23TopNErrorsPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_954b5c() -> ! {
    todo!("0x954b5c rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x954c68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network23TopNErrorsPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
pub fn stub_954c68() -> ! {
    todo!("0x954c68 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x954d84 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network23TopNErrorsPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
pub fn stub_954d84() -> ! {
    todo!("0x954d84 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x954ff0 — __ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")]
pub fn stub_954ff0() -> ! {
    todo!("0x954ff0 boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")
}

// 0x955268 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")]
pub fn stub_955268() -> ! {
    todo!("0x955268 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")
}

// 0x955a74 — __ZN3RBX10Reflection4Type12getSingletonINS_7Network12FilterResultEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::FilterResult>(void)")]
pub fn stub_955a74() -> ! {
    todo!("0x955a74 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::FilterResult>(void)")
}

// 0x955b80 — __ZN3RBX10Reflection4Type12getSingletonINS_7Network6Player14MembershipTypeEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Player::MembershipType>(void)")]
pub fn stub_955b80() -> ! {
    todo!("0x955b80 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Player::MembershipType>(void)")
}

// 0x955c8c — __ZN3RBX10Reflection4Type12getSingletonINS_7Network7Players14PlayerChatTypeEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Players::PlayerChatType>(void)")]
pub fn stub_955c8c() -> ! {
    todo!("0x955c8c RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Players::PlayerChatType>(void)")
}

// 0x955d98 — __ZN3RBX10Reflection4Type12getSingletonINS_7Network7Players10ChatOptionEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Players::ChatOption>(void)")]
pub fn stub_955d98() -> ! {
    todo!("0x955d98 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Network::Players::ChatOption>(void)")
}

// 0x955ea4 — __ZN3RBX7Network29isPlayerAuthenticationEnabledEv
// type: _DWORD __fastcall(RBX::Network *__hidden this)
#[doc(alias = "RBX::Network::isPlayerAuthenticationEnabled(void)")]
pub fn stub_955ea4() -> ! {
    todo!("0x955ea4 RBX::Network::isPlayerAuthenticationEnabled(void)")
}

// 0x956100 — __ZN3RBX7Network19initWithoutSecurityEv
// type: _DWORD __fastcall(RBX::Network *__hidden this)
#[doc(alias = "RBX::Network::initWithoutSecurity(void)")]
pub fn stub_956100() -> ! {
    todo!("0x956100 RBX::Network::initWithoutSecurity(void)")
}

// 0x9564ec — __ZN3RBX7Network22initWithPlayerSecurityEv
// type: _DWORD __fastcall(RBX::Network *__hidden this)
#[doc(alias = "RBX::Network::initWithPlayerSecurity(void)")]
pub fn stub_9564ec() -> ! {
    todo!("0x9564ec RBX::Network::initWithPlayerSecurity(void)")
}

// 0x95655c — __ZN3RBX7Network16isTrustedContentEPKc
// type: _DWORD __fastcall(RBX::Network *__hidden this, const char *)
#[doc(alias = "RBX::Network::isTrustedContent(char const*)")]
pub fn stub_95655c() -> ! {
    todo!("0x95655c RBX::Network::isTrustedContent(char const*)")
}

// 0x9573c0 — __ZN3RBX7Network12SafeInitFreeD1Ev
// type: void __fastcall(RBX::Network::SafeInitFree *__hidden this)
#[doc(alias = "RBX::Network::SafeInitFree::~SafeInitFree()")]
pub fn stub_9573c0() -> ! {
    todo!("0x9573c0 RBX::Network::SafeInitFree::~SafeInitFree()")
}

// 0x957584 — __ZN5boost6detail8function15functor_managerIPFNS_10shared_ptrIN3RBX7Network16ServerReplicatorEEEN6RakNet13SystemAddressEPNS5_6ServerEPNS4_15NetworkSettingsEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN ***, _WORD *, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::shared_ptr<RBX::Network::ServerReplicator> (*)(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_957584() -> ! {
    todo!("0x957584 boost::detail::function::functor_manager<boost::shared_ptr<RBX::Network::ServerReplicator> (*)(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x9575e0 — __ZN5boost6detail8function17function_invoker3IPFNS_10shared_ptrIN3RBX7Network16ServerReplicatorEEEN6RakNet13SystemAddressEPNS5_6ServerEPNS4_15NetworkSettingsEES7_S9_SB_SD_E6invokeERNS1_15function_bufferES9_SB_SD_
// type: int __fastcall(int, int (__fastcall **)(int, int, int, int, int, int, int, int), int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::function_invoker3<boost::shared_ptr<RBX::Network::ServerReplicator> (*)(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *),boost::shared_ptr<RBX::Network::ServerReplicator>,RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *>::invoke(boost::detail::function::function_buffer &,RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")]
pub fn stub_9575e0() -> ! {
    todo!("0x9575e0 boost::detail::function::function_invoker3<boost::shared_ptr<RBX::Network::ServerReplicator> (*)(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *),boost::shared_ptr<RBX::Network::ServerReplicator>,RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *>::invoke(boost::detail::function::function_buffer &,RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")
}

// 0x95760c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod> const>::initSingleton(void)")]
pub fn stub_95760c() -> ! {
    todo!("0x95760c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod> const>::initSingleton(void)")
}

// 0x9576f0 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_9576f0() -> ! {
    todo!("0x9576f0 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")
}

// 0x9576fc — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_9576fc() -> ! {
    todo!("0x9576fc RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")
}

// 0x957978 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_957978() -> ! {
    todo!("0x957978 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")
}

// 0x957a18 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::lookup(char const*)const")]
pub fn stub_957a18() -> ! {
    todo!("0x957a18 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::lookup(char const*)const")
}

// 0x957aa8 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_957aa8() -> ! {
    todo!("0x957aa8 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x957bac — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_957bac() -> ! {
    todo!("0x957bac RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x957bd4 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_957bd4() -> ! {
    todo!("0x957bd4 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(unsigned long,std::string &)const")
}

// 0x957d18 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")]
pub fn stub_957d18() -> ! {
    todo!("0x957d18 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")
}

// 0x957eb8 — __ZN3RBX10Reflection7VariantaSINS_15NetworkSettings20PhysicsReceiveMethodEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsReceiveMethod>(RBX::NetworkSettings::PhysicsReceiveMethod const&)")]
pub fn stub_957eb8() -> ! {
    todo!("0x957eb8 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsReceiveMethod>(RBX::NetworkSettings::PhysicsReceiveMethod const&)")
}

// 0x95806c — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings20PhysicsReceiveMethodEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsReceiveMethod>::construct_func(char const*,char *)")]
pub fn stub_95806c() -> ! {
    todo!("0x95806c rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsReceiveMethod>::construct_func(char const*,char *)")
}

// 0x958078 — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings20PhysicsReceiveMethodEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsReceiveMethod>::destruct_func(char *)")]
pub fn stub_958078() -> ! {
    todo!("0x958078 rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsReceiveMethod>::destruct_func(char *)")
}

// 0x95807c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToItem(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")]
pub fn stub_95807c() -> ! {
    todo!("0x95807c RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToItem(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")
}

// 0x958148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>> *)")]
pub fn stub_958148() -> ! {
    todo!("0x958148 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>> *)")
}

// 0x958170 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15NetworkSettings17PhysicsSendMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod> const>::initSingleton(void)")]
pub fn stub_958170() -> ! {
    todo!("0x958170 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod> const>::initSingleton(void)")
}

// 0x958254 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_958254() -> ! {
    todo!("0x958254 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")
}

// 0x958260 — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_958260() -> ! {
    todo!("0x958260 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")
}

// 0x9584dc — __ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_9584dc() -> ! {
    todo!("0x9584dc RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")
}

// 0x95857c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::lookup(char const*)const")]
pub fn stub_95857c() -> ! {
    todo!("0x95857c RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::lookup(char const*)const")
}

// 0x95860c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_95860c() -> ! {
    todo!("0x95860c RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x958710 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_958710() -> ! {
    todo!("0x958710 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x958738 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(unsigned long,std::string &)const")]
pub fn stub_958738() -> ! {
    todo!("0x958738 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(unsigned long,std::string &)const")
}

// 0x95887c — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(RBX::NetworkSettings::PhysicsSendMethod const&)const")]
pub fn stub_95887c() -> ! {
    todo!("0x95887c RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(RBX::NetworkSettings::PhysicsSendMethod const&)const")
}

// 0x958a1c — __ZN3RBX10Reflection7VariantaSINS_15NetworkSettings17PhysicsSendMethodEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsSendMethod>(RBX::NetworkSettings::PhysicsSendMethod const&)")]
pub fn stub_958a1c() -> ! {
    todo!("0x958a1c RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsSendMethod>(RBX::NetworkSettings::PhysicsSendMethod const&)")
}

// 0x958bd0 — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings17PhysicsSendMethodEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsSendMethod>::construct_func(char const*,char *)")]
pub fn stub_958bd0() -> ! {
    todo!("0x958bd0 rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsSendMethod>::construct_func(char const*,char *)")
}

// 0x958bdc — __ZN3rbx14implementation12typed_holderIN3RBX15NetworkSettings17PhysicsSendMethodEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsSendMethod>::destruct_func(char *)")]
pub fn stub_958bdc() -> ! {
    todo!("0x958bdc rbx::implementation::typed_holder<RBX::NetworkSettings::PhysicsSendMethod>::destruct_func(char *)")
}

// 0x958be0 — __ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToItem(RBX::NetworkSettings::PhysicsSendMethod const&)const")]
pub fn stub_958be0() -> ! {
    todo!("0x958be0 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToItem(RBX::NetworkSettings::PhysicsSendMethod const&)const")
}

// 0x958cac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>> *)")]
pub fn stub_958cac() -> ! {
    todo!("0x958cac std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>> *)")
}

// 0x958cd4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Network6Player8ChatModeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode> const>::initSingleton(void)")]
pub fn stub_958cd4() -> ! {
    todo!("0x958cd4 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode> const>::initSingleton(void)")
}

// 0x958db8 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_958db8() -> ! {
    todo!("0x958db8 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")
}

// 0x958dc4 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_958dc4() -> ! {
    todo!("0x958dc4 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")
}

// 0x959040 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_959040() -> ! {
    todo!("0x959040 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")
}

// 0x9590e0 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::lookup(char const*)const")]
pub fn stub_9590e0() -> ! {
    todo!("0x9590e0 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::lookup(char const*)const")
}

// 0x959170 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_959170() -> ! {
    todo!("0x959170 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x959274 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_959274() -> ! {
    todo!("0x959274 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x95929c — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(unsigned long,std::string &)const")]
pub fn stub_95929c() -> ! {
    todo!("0x95929c RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(unsigned long,std::string &)const")
}

// 0x9593e0 — __ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE15convertToStringERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(RBX::Network::Player::ChatMode const&)const")]
pub fn stub_9593e0() -> ! {
    todo!("0x9593e0 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(RBX::Network::Player::ChatMode const&)const")
}

// 0x959580 — __ZN3RBX10Reflection7VariantaSINS_7Network6Player8ChatModeEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::ChatMode>(RBX::Network::Player::ChatMode const&)")]
pub fn stub_959580() -> ! {
    todo!("0x959580 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::ChatMode>(RBX::Network::Player::ChatMode const&)")
}

