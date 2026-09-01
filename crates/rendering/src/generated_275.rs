//! rendering shard 275 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 29870->29970 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29870 before -> 29970 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x39e5a8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_
// type: int *__fastcall(int *, int)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_
pub fn stub_39e5a8() -> ! {
    todo!("0x39e5a8 rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")
}

// 0x39e5cc — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED1Ev
pub fn stub_39e5cc() -> ! {
    todo!("0x39e5cc rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")
}

// 0x39e5f8 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED0Ev
pub fn stub_39e5f8() -> ! {
    todo!("0x39e5f8 rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")
}

// 0x39e6cc — __ZN3rbx7signals6signalIFvffffEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvffffEE4slot10disconnectEv
pub fn stub_39e6cc() -> ! {
    todo!("0x39e6cc rbx::signals::signal<void ()(float,float,float,float)>::slot::disconnect(void)")
}

// 0x39e7dc — __ZNK3rbx7signals6signalIFvffffEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvffffEE4slot9connectedEv
pub fn stub_39e7dc() -> ! {
    todo!("0x39e7dc rbx::signals::signal<void ()(float,float,float,float)>::slot::connected(void)const")
}

// 0x39e7e8 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
// type: int __fastcall(int, int, int, int, float)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
pub fn stub_39e7e8() -> ! {
    todo!("0x39e7e8 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}

// 0x39e824 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
// type: int __fastcall(int, int, int, int, float)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
pub fn stub_39e824() -> ! {
    todo!("0x39e824 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}

// 0x39e860 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_39e860() -> ! {
    todo!("0x39e860 void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")
}

// 0x39e8b0 — __ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE
pub fn stub_39e8b0() -> ! {
    todo!("0x39e8b0 rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")
}

// 0x39e9a0 — __ZN3rbx7signals6signalIFvffffEE4slot22safe_static_init_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvffffEE4slot22safe_static_init_mutexEv
pub fn stub_39e9a0() -> ! {
    todo!("0x39e9a0 rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_init_mutex(void)")
}

// 0x39e9a4 — __ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv
pub fn stub_39e9a4() -> ! {
    todo!("0x39e9a4 rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")
}

// 0x39ea94 — __ZN3rbx7signals6signalIFvffffEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE4slotD1Ev
pub fn stub_39ea94() -> ! {
    todo!("0x39ea94 rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")
}

// 0x39eac0 — __ZN3rbx7signals6signalIFvffffEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE4slotD0Ev
pub fn stub_39eac0() -> ! {
    todo!("0x39eac0 rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")
}

// 0x39eb94 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED1Ev
pub fn stub_39eb94() -> ! {
    todo!("0x39eb94 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")
}

// 0x39ebc0 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED0Ev
pub fn stub_39ebc0() -> ! {
    todo!("0x39ebc0 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")
}

// 0x39ec94 — __ZN3rbx13remote_signalIFvfffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvfffEEC2Ev
pub fn stub_39ec94() -> ! {
    todo!("0x39ec94 rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")
}

// 0x39edf0 — __ZN3rbx7signals6signalIFvfffEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvfffEE13disconnectAllEv
pub fn stub_39edf0() -> ! {
    todo!("0x39edf0 rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")
}

// 0x39ef68 — __ZN3rbx13remote_signalIFvffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvffEEC2Ev
pub fn stub_39ef68() -> ! {
    todo!("0x39ef68 rbx::remote_signal<void ()(float,float)>::remote_signal(void)")
}

// 0x39f0c4 — __ZN3rbx13remote_signalIFvffffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvffffEEC2Ev
pub fn stub_39f0c4() -> ! {
    todo!("0x39f0c4 rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")
}

// 0x39f220 — __ZN3rbx7signals6signalIFvffffEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvffffEE13disconnectAllEv
pub fn stub_39f220() -> ! {
    todo!("0x39f220 rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")
}

// 0x39f398 — __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_39f398() -> ! {
    todo!("0x39f398 __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x39f39c — __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_39f39c() -> ! {
    todo!("0x39f39c __ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x39f43c — __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_39f43c() -> ! {
    todo!("0x39f43c __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x39f444 — __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_39f444() -> ! {
    todo!("0x39f444 __ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x39f4e8 — __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_39f4e8() -> ! {
    todo!("0x39f4e8 __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x39f4f0 — __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_39f4f0() -> ! {
    todo!("0x39f4f0 __ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x39f594 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEED0Ev
pub fn stub_39f594() -> ! {
    todo!("0x39f594 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}

// 0x39f648 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_39f648() -> ! {
    todo!("0x39f648 RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x39f7ac — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
pub fn stub_39f7ac() -> ! {
    todo!("0x39f7ac RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")
}

// 0x39f7b4 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
pub fn stub_39f7b4() -> ! {
    todo!("0x39f7b4 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")
}

// 0x39f7bc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_39f7bc() -> ! {
    todo!("0x39f7bc RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39f960 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
pub fn stub_39f960() -> ! {
    todo!("0x39f960 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39f970 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_39f970() -> ! {
    todo!("0x39f970 RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x39f984 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_39f984() -> ! {
    todo!("0x39f984 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x39fb08 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
pub fn stub_39fb08() -> ! {
    todo!("0x39fb08 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x39fb2c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
pub fn stub_39fb2c() -> ! {
    todo!("0x39fb2c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x39fbe0 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEED0Ev
pub fn stub_39fbe0() -> ! {
    todo!("0x39fbe0 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc()")
}

// 0x39fc94 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_39fc94() -> ! {
    todo!("0x39fc94 RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x39fdf8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE12isScriptableEv
pub fn stub_39fdf8() -> ! {
    todo!("0x39fdf8 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isScriptable(void)const")
}

// 0x39fe00 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
pub fn stub_39fe00() -> ! {
    todo!("0x39fe00 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isBroadcast(void)const")
}

// 0x39fe08 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_39fe08() -> ! {
    todo!("0x39fe08 RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39feb0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
pub fn stub_39feb0() -> ! {
    todo!("0x39feb0 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x39fec0 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_39fec0() -> ! {
    todo!("0x39fec0 RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x39fed4 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfS5_S5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISE_T0_T1_T2_T3_EENSC_9list_av_4IT4_T5_T6_T7_E4typeEEEMSH_FSE_SI_SJ_SK_ESN_SO_SP_SQ_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfS5_S5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISE_T0_T1_T2_T3_EENSC_9list_av_4IT4_T5_T6_T7_E4typeEEEMSH_FSE_SI_SJ_SK_ESN_SO_SP_SQ_
pub fn stub_39fed4() -> ! {
    todo!("0x39fed4 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x39fff0 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IfffEEvRKT_RKT0_RKT1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<float,float,float>(float const&,float const&,float const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute3IfffEEvRKT_RKT0_RKT1_
pub fn stub_39fff0() -> ! {
    todo!("0x39fff0 void RBX::Reflection::GenericSlotWrapper::execute3<float,float,float>(float const&,float const&,float const&)")
}

// 0x3a017c — __ZN5boost9function3IvfffE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function3<void,float,float,float>::clear(void)")]
// was: __ZN5boost9function3IvfffE5clearEv
pub fn stub_3a017c() -> ! {
    todo!("0x3a017c boost::function3<void,float,float,float>::clear(void)")
}

// 0x3a01a8 — __ZN5boost8functionIFvfffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvfffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvfffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_3a01a8() -> ! {
    todo!("0x3a01a8 __ZN5boost8functionIFvfffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x3a028c — __ZN5boost9function3IvfffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvfffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvfffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_3a028c() -> ! {
    todo!("0x3a028c __ZN5boost9function3IvfffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x3a0374 — __ZN5boost9function3IvfffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: __ZN5boost9function3IvfffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEEvT_
pub fn stub_3a0374() -> ! {
    todo!("0x3a0374 void boost::function3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x3a046c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
pub fn stub_3a046c() -> ! {
    todo!("0x3a046c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3a0488 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEvfffE6invokeERNS1_15function_bufferEfff
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,float,float,float>::invoke(boost::detail::function::function_buffer &,float,float,float)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEvfffE6invokeERNS1_15function_bufferEfff
pub fn stub_3a0488() -> ! {
    todo!("0x3a0488 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,float,float,float>::invoke(boost::detail::function::function_buffer &,float,float,float)")
}

// 0x3a04b0 — __ZNK5boost6detail8function13basic_vtable3IvfffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvfffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_3a04b0() -> ! {
    todo!("0x3a04b0 bool boost::detail::function::basic_vtable3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x3a0598 — __ZNK5boost6detail8function13basic_vtable3IvfffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvfffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_3a0598() -> ! {
    todo!("0x3a0598 bool boost::detail::function::basic_vtable3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x3a067c — __ZNK5boost6detail8function13basic_vtable3IvfffE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,float,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvfffE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_3a067c() -> ! {
    todo!("0x3a067c void boost::detail::function::basic_vtable3<void,float,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x3a0750 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKfSI_SI_EENS0_5list3IRfSL_SL_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int *)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&> &,boost::_bi::list3<float &,float &,float &> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKfSI_SI_EENS0_5list3IRfSL_SL_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3a0750() -> ! {
    todo!("0x3a0750 void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&> &,boost::_bi::list3<float &,float &,float &> &,int)")
}

// 0x3a0778 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_3a0778() -> ! {
    todo!("0x3a0778 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x3a08d0 — __ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")]
// was: __ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_3a08d0() -> ! {
    todo!("0x3a08d0 rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")
}

// 0x3a09c4 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_3a09c4() -> ! {
    todo!("0x3a09c4 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")
}

// 0x3a0ac0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_3a0ac0() -> ! {
    todo!("0x3a0ac0 rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")
}

// 0x3a0bd0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_3a0bd0() -> ! {
    todo!("0x3a0bd0 rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")
}

// 0x3a0d00 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
pub fn stub_3a0d00() -> ! {
    todo!("0x3a0d00 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")
}

// 0x3a0d08 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
pub fn stub_3a0d08() -> ! {
    todo!("0x3a0d08 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")
}

// 0x3a0d10 — __ZNK5boost9function3IvfffEclEfff
// type: void __fastcall(_DWORD *, int, int, int)
#[doc(alias = "boost::function3<void,float,float,float>::operator()(float,float,float)const")]
// was: __ZNK5boost9function3IvfffEclEfff
pub fn stub_3a0d10() -> ! {
    todo!("0x3a0d10 boost::function3<void,float,float,float>::operator()(float,float,float)const")
}

// 0x3a0dec — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_3a0dec() -> ! {
    todo!("0x3a0dec rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")
}

// 0x3a0efc — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_3a0efc() -> ! {
    todo!("0x3a0efc rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")
}

// 0x3a102c — __ZN5boost9function3IvfffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")]
// was: __ZN5boost9function3IvfffE13assign_to_ownERKS1_
pub fn stub_3a102c() -> ! {
    todo!("0x3a102c boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")
}

// 0x3a105c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3a105c() -> ! {
    todo!("0x3a105c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3a12b8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
pub fn stub_3a12b8() -> ! {
    todo!("0x3a12b8 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a12dc — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvfffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
pub fn stub_3a12dc() -> ! {
    todo!("0x3a12dc RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a1390 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEED0Ev
pub fn stub_3a1390() -> ! {
    todo!("0x3a1390 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc()")
}

// 0x3a1444 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_3a1444() -> ! {
    todo!("0x3a1444 RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3a15a8 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE12isScriptableEv
pub fn stub_3a15a8() -> ! {
    todo!("0x3a15a8 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isScriptable(void)const")
}

// 0x3a15b0 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
pub fn stub_3a15b0() -> ! {
    todo!("0x3a15b0 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isBroadcast(void)const")
}

// 0x3a15b8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_3a15b8() -> ! {
    todo!("0x3a15b8 RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1654 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
pub fn stub_3a1654() -> ! {
    todo!("0x3a1654 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1664 — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_3a1664() -> ! {
    todo!("0x3a1664 RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3a1678 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3a1678() -> ! {
    todo!("0x3a1678 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3a1868 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
pub fn stub_3a1868() -> ! {
    todo!("0x3a1868 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a188c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
pub fn stub_3a188c() -> ! {
    todo!("0x3a188c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}

// 0x3a1940 — __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEED0Ev
pub fn stub_3a1940() -> ! {
    todo!("0x3a1940 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc()")
}

// 0x3a19f4 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_3a19f4() -> ! {
    todo!("0x3a19f4 RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3a1b58 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE12isScriptableEv
pub fn stub_3a1b58() -> ! {
    todo!("0x3a1b58 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isScriptable(void)const")
}

// 0x3a1b60 — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE11isBroadcastEv
pub fn stub_3a1b60() -> ! {
    todo!("0x3a1b60 RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isBroadcast(void)const")
}

// 0x3a1b68 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi4ENS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_3a1b68() -> ! {
    todo!("0x3a1b68 RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1c2c — __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
pub fn stub_3a1c2c() -> ! {
    todo!("0x3a1c2c RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3a1c3c — __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_3a1c3c() -> ! {
    todo!("0x3a1c3c RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3a1c50 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfS5_S5_S5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS8_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISF_T0_T1_T2_T3_T4_EENSD_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSI_FSF_SJ_SK_SL_SM_ESP_SQ_SR_SS_ST_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&,float const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfS5_S5_S5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS8_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISF_T0_T1_T2_T3_T4_EENSD_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSI_FSF_SJ_SK_SL_SM_ESP_SQ_SR_SS_ST_
pub fn stub_3a1c50() -> ! {
    todo!("0x3a1c50 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0x3a1d6c — __ZN3RBX10Reflection18GenericSlotWrapper8execute4IffffEEvRKT_RKT0_RKT1_RKT2_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<float,float,float,float>(float const&,float const&,float const&,float const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute4IffffEEvRKT_RKT0_RKT1_RKT2_
pub fn stub_3a1d6c() -> ! {
    todo!("0x3a1d6c void RBX::Reflection::GenericSlotWrapper::execute4<float,float,float,float>(float const&,float const&,float const&,float const&)")
}

// 0x3a1f18 — __ZN5boost9function4IvffffE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function4<void,float,float,float,float>::clear(void)")]
// was: __ZN5boost9function4IvffffE5clearEv
pub fn stub_3a1f18() -> ! {
    todo!("0x3a1f18 boost::function4<void,float,float,float,float>::clear(void)")
}

// 0x3a1f44 — __ZN5boost8functionIFvffffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_SC_EENS4_5list5INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEENSJ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvffffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_SC_EENS4_5list5INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEENSJ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvffffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_SC_EENS4_5list5INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEENSJ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_3a1f44() -> ! {
    todo!("0x3a1f44 __ZN5boost8functionIFvffffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_SC_EENS4_5list5INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEENSJ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x3a2028 — __ZN5boost9function4IvffffEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function4IvffffEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function4IvffffEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_3a2028() -> ! {
    todo!("0x3a2028 __ZN5boost9function4IvffffEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x3a2110 — __ZN5boost9function4IvffffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
// was: __ZN5boost9function4IvffffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEEvT_
pub fn stub_3a2110() -> ! {
    todo!("0x3a2110 void boost::function4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0x3a2208 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_3a2208() -> ! {
    todo!("0x3a2208 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3a2224 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEvffffE6invokeERNS1_15function_bufferEffff
// type: int __fastcall(int *, int, int, int, float)
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,float,float,float,float>::invoke(boost::detail::function::function_buffer &,float,float,float,float)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEvffffE6invokeERNS1_15function_bufferEffff
pub fn stub_3a2224() -> ! {
    todo!("0x3a2224 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,float,float,float,float>::invoke(boost::detail::function::function_buffer &,float,float,float,float)")
}

// 0x3a225c — __ZNK5boost6detail8function13basic_vtable4IvffffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_SD_EENS5_5list5INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable4IvffffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_SD_EENS5_5list5INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_3a225c() -> ! {
    todo!("0x3a225c bool boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")
}

// 0x3a2344 — __ZNK5boost6detail8function13basic_vtable4IvffffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_SD_EENS5_5list5INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable4IvffffE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_SD_EENS5_5list5INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_3a2344() -> ! {
    todo!("0x3a2344 bool boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x3a2428 — __ZNK5boost6detail8function13basic_vtable4IvffffE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_SD_EENS5_5list5INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable4IvffffE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSD_SD_SD_EENS5_5list5INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_3a2428() -> ! {
    todo!("0x3a2428 void boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x3a24fc — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKfSJ_SJ_SJ_EENS0_5list4IRfSM_SM_SM_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKfSJ_SJ_SJ_EENS0_5list4IRfSM_SM_SM_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3a24fc() -> ! {
    todo!("0x3a24fc void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")
}

// 0x3a252c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_3a252c() -> ! {
    todo!("0x3a252c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x3a2684 — __ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")]
// was: __ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_3a2684() -> ! {
    todo!("0x3a2684 rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")
}
