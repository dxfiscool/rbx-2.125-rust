//! rendering shard 386 — 100 stubs 0x569d5c..0x56d5c8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 41811->41911 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x569d5c..0x56d5c8 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x569d5c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::insert(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6insertEPNS5_4slotE
pub fn stub_569d5c() -> ! {
    todo!("0x569d5c rbx::signals::signal<void ()(RBX::NormalId,float)>::insert(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")
}

// 0x569f68 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_
pub fn stub_569f68() -> ! {
    todo!("0x569f68 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)")
}

// 0x569f8c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev
pub fn stub_569f8c() -> ! {
    todo!("0x569f8c rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x569fb8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev
pub fn stub_569fb8() -> ! {
    todo!("0x569fb8 rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x56a08c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot10disconnectEv
pub fn stub_56a08c() -> ! {
    todo!("0x56a08c rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::disconnect(void)")
}

// 0x56a19c — __ZNK3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot9connectedEv
pub fn stub_56a19c() -> ! {
    todo!("0x56a19c rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::connected(void)const")
}

// 0x56a1a8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
pub fn stub_56a1a8() -> ! {
    todo!("0x56a1a8 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")
}

// 0x56a1d0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
pub fn stub_56a1d0() -> ! {
    todo!("0x56a1d0 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")
}

// 0x56a1f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list2<RBX::NormalId&,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,boost::_bi::list2<RBX::NormalId&,float &> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_56a1f8() -> ! {
    todo!("0x56a1f8 void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list2<RBX::NormalId&,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,boost::_bi::list2<RBX::NormalId&,float &> &,int)")
}

// 0x56a224 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::remove(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE6removeEPNS5_4slotE
pub fn stub_56a224() -> ! {
    todo!("0x56a224 rbx::signals::signal<void ()(RBX::NormalId,float)>::remove(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot *)")
}

// 0x56a314 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot22safe_static_init_mutexEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot22safe_static_init_mutexEv
pub fn stub_56a314() -> ! {
    todo!("0x56a314 rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_init_mutex(void)")
}

// 0x56a318 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slot24safe_static_do_get_mutexEv
pub fn stub_56a318() -> ! {
    todo!("0x56a318 rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::safe_static_do_get_mutex(void)")
}

// 0x56a408 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD1Ev
pub fn stub_56a408() -> ! {
    todo!("0x56a408 rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::~slot()")
}

// 0x56a434 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotD0Ev
pub fn stub_56a434() -> ! {
    todo!("0x56a434 rbx::signals::signal<void ()(RBX::NormalId,float)>::slot::~slot()")
}

// 0x56a508 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev
pub fn stub_56a508() -> ! {
    todo!("0x56a508 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")
}

// 0x56a534 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev
pub fn stub_56a534() -> ! {
    todo!("0x56a534 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")
}

// 0x56a608 — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEfEE21connectSignalListenerEv
pub fn stub_56a608() -> ! {
    todo!("0x56a608 RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::connectSignalListener(void)")
}

// 0x56a60c — __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
pub fn stub_56a60c() -> ! {
    todo!("0x56a60c RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")
}

// 0x56a700 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
pub fn stub_56a700() -> ! {
    todo!("0x56a700 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::getSignalPtr(RBX::Reflection::EventSource *)")
}

// 0x56a768 — __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE25signalProducedIncrementedES2_
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE25signalProducedIncrementedES2_")]
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::signalProducedIncremented(RBX::NormalId)")]
// was: __ZN3RBX19EventReplicatorImplILi1ENS_7HandlesEFvNS_8NormalIdEEE25signalProducedIncrementedES2_
pub fn stub_56a768() -> ! {
    todo!("0x56a768 RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>::signalProducedIncremented(RBX::NormalId)")
}

// 0x56a77c — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_
pub fn stub_56a77c() -> ! {
    todo!("0x56a77c RBX::Reflection::RemoteEventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId)")
}

// 0x56a8c8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_56a8c8() -> ! {
    todo!("0x56a8c8 rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>> const&)")
}

// 0x56a93c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::insert(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE
pub fn stub_56a93c() -> ! {
    todo!("0x56a93c rbx::signals::signal<void ()(RBX::NormalId)>::insert(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")
}

// 0x56ab48 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_
pub fn stub_56ab48() -> ! {
    todo!("0x56ab48 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId)>::slot*)")
}

// 0x56ab6c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
pub fn stub_56ab6c() -> ! {
    todo!("0x56ab6c rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x56ab98 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
pub fn stub_56ab98() -> ! {
    todo!("0x56ab98 rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x56ac6c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot10disconnectEv
pub fn stub_56ac6c() -> ! {
    todo!("0x56ac6c rbx::signals::signal<void ()(RBX::NormalId)>::slot::disconnect(void)")
}

// 0x56ad7c — __ZNK3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot9connectedEv
pub fn stub_56ad7c() -> ! {
    todo!("0x56ad7c rbx::signals::signal<void ()(RBX::NormalId)>::slot::connected(void)const")
}

// 0x56ad88 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_56ad88() -> ! {
    todo!("0x56ad88 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")
}

// 0x56ad9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_56ad9c() -> ! {
    todo!("0x56ad9c `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")
}

// 0x56adb0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_56adb0() -> ! {
    todo!("0x56adb0 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")
}

// 0x56adc8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::remove(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE
pub fn stub_56adc8() -> ! {
    todo!("0x56adc8 rbx::signals::signal<void ()(RBX::NormalId)>::remove(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")
}

// 0x56aeb8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot22safe_static_init_mutexEv
pub fn stub_56aeb8() -> ! {
    todo!("0x56aeb8 rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_init_mutex(void)")
}

// 0x56aebc — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv
pub fn stub_56aebc() -> ! {
    todo!("0x56aebc rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_do_get_mutex(void)")
}

// 0x56afac — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD1Ev
pub fn stub_56afac() -> ! {
    todo!("0x56afac rbx::signals::signal<void ()(RBX::NormalId)>::slot::~slot()")
}

// 0x56afd8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotD0Ev
pub fn stub_56afd8() -> ! {
    todo!("0x56afd8 rbx::signals::signal<void ()(RBX::NormalId)>::slot::~slot()")
}

// 0x56b0ac — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
pub fn stub_56b0ac() -> ! {
    todo!("0x56b0ac rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")
}

// 0x56b0d8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
pub fn stub_56b0d8() -> ! {
    todo!("0x56b0d8 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,boost::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")
}

// 0x56b1ac — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv")]
#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")]
// was: __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE21connectSignalListenerEv
pub fn stub_56b1ac() -> ! {
    todo!("0x56b1ac RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::connectSignalListener(void)")
}

// 0x56b1b0 — __ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_56b1b0() -> ! {
    todo!("0x56b1b0 __ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x56b1b4 — __ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_56b1b4() -> ! {
    todo!("0x56b1b4 __ZN3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x56b254 — __ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_56b254() -> ! {
    todo!("0x56b254 __ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x56b25c — __ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_56b25c() -> ! {
    todo!("0x56b25c __ZThn32_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x56b300 — __ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_56b300() -> ! {
    todo!("0x56b300 __ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x56b308 — __ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_56b308() -> ! {
    todo!("0x56b308 __ZThn36_N3RBX10Reflection9DescribedINS_7HandlesELZNS_8sHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_8sHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x56b3ac — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_7HandlesEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_7HandlesEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Handles>(char const*,char const*,int RBX::Handles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_7HandlesEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_56b3ac() -> ! {
    todo!("0x56b3ac RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Handles>(char const*,char const*,int RBX::Handles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x56b53c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE10isReadOnlyEv
pub fn stub_56b53c() -> ! {
    todo!("0x56b53c RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::isReadOnly(void)const")
}

// 0x56b540 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE11isWriteOnlyEv
pub fn stub_56b540() -> ! {
    todo!("0x56b540 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::isWriteOnly(void)const")
}

// 0x56b544 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_56b544() -> ! {
    todo!("0x56b544 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x56b550 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8setValueEPNS0_13DescribedBaseERKi")]
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_7HandlesEE8setValueEPNS0_13DescribedBaseERKi
pub fn stub_56b550() -> ! {
    todo!("0x56b550 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Handles>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x56b5a0 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEED0Ev
pub fn stub_56b5a0() -> ! {
    todo!("0x56b5a0 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")
}

// 0x56b654 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_56b654() -> ! {
    todo!("0x56b654 RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x56b7b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE12isScriptableEv
pub fn stub_56b7b8() -> ! {
    todo!("0x56b7b8 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isScriptable(void)const")
}

// 0x56b7c0 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE11isBroadcastEv
pub fn stub_56b7c0() -> ! {
    todo!("0x56b7c0 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::isBroadcast(void)const")
}

// 0x56b7c8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
pub fn stub_56b7c8() -> ! {
    todo!("0x56b7c8 RBX::Reflection::EventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56b864 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_56b864() -> ! {
    todo!("0x56b864 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56b874 — __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_56b874() -> ! {
    todo!("0x56b874 RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x56b888 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdERKfNS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdERKfNS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdERKfNS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
pub fn stub_56b888() -> ! {
    todo!("0x56b888 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&,float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x56b9a4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_8NormalIdEfEEvRKT_RKT0_
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_8NormalIdEfEEvRKT_RKT0_")]
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::NormalId,float>(RBX::NormalId const&,float const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_8NormalIdEfEEvRKT_RKT0_
pub fn stub_56b9a4() -> ! {
    todo!("0x56b9a4 void RBX::Reflection::GenericSlotWrapper::execute2<RBX::NormalId,float>(RBX::NormalId const&,float const&)")
}

// 0x56bb0c — __ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv
// type: int(void)
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv")]
#[doc(alias = "boost::function2<void,RBX::NormalId,float>::clear(void)")]
// was: __ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv
pub fn stub_56bb0c() -> ! {
    todo!("0x56bb0c boost::function2<void,RBX::NormalId,float>::clear(void)")
}

// 0x56bb38 — __ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_56bb38() -> ! {
    todo!("0x56bb38 __ZN5boost8functionIFvN3RBX8NormalIdEfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0x56bc1c — __ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_56bc1c() -> ! {
    todo!("0x56bc1c __ZN5boost9function2IvN3RBX8NormalIdEfEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x56bd04 — __ZN5boost9function2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_")]
#[doc(alias = "void boost::function2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: __ZN5boost9function2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS2_RKfEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_
pub fn stub_56bd04() -> ! {
    todo!("0x56bd04 void boost::function2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x56bdfc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
pub fn stub_56bdfc() -> ! {
    todo!("0x56bdfc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x56be18 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSA_fE6invokeERNS1_15function_bufferESA_f
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSA_fE6invokeERNS1_15function_bufferESA_f")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::NormalId,float>::invoke(boost::detail::function::function_buffer &,RBX::NormalId,float)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSA_fE6invokeERNS1_15function_bufferESA_f
pub fn stub_56be18() -> ! {
    todo!("0x56be18 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::NormalId,float>::invoke(boost::detail::function::function_buffer &,RBX::NormalId,float)")
}

// 0x56be30 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_56be30() -> ! {
    todo!("0x56be30 bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x56bf18 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_56bf18() -> ! {
    todo!("0x56bf18 bool boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x56bffc — __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX8NormalIdEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_56bffc() -> ! {
    todo!("0x56bffc void boost::detail::function::basic_vtable2<void,RBX::NormalId,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x56c0d0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdERKfEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclIS7_fEEvRT_RT0_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdERKfEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclIS7_fEEvRT_RT0_")]
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::NormalId,float>(RBX::NormalId &,float &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8NormalIdERKfEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclIS7_fEEvRT_RT0_
pub fn stub_56c0d0() -> ! {
    todo!("0x56c0d0 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::NormalId,float>(RBX::NormalId &,float &)")
}

// 0x56c0ec — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdERKfEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_56c0ec() -> ! {
    todo!("0x56c0ec boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,float const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x56c244 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::function<void ()(RBX::NormalId,float)>>(boost::function<void ()(RBX::NormalId,float)> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_56c244() -> ! {
    todo!("0x56c244 rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::function<void ()(RBX::NormalId,float)>>(boost::function<void ()(RBX::NormalId,float)> const&)")
}

// 0x56c338 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>*>(boost::function<void ()(RBX::NormalId,float)> const&,rbx::signals::signal<void ()(RBX::NormalId,float)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
pub fn stub_56c338() -> ! {
    todo!("0x56c338 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>*>(boost::function<void ()(RBX::NormalId,float)> const&,rbx::signals::signal<void ()(RBX::NormalId,float)>*)")
}

// 0x56c434 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::function<void ()(RBX::NormalId,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_56c434() -> ! {
    todo!("0x56c434 rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::function<void ()(RBX::NormalId,float)>>::~callable_slot()")
}

// 0x56c544 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::function<void ()(RBX::NormalId,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_56c544() -> ! {
    todo!("0x56c544 rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::function<void ()(RBX::NormalId,float)>>::~callable_slot()")
}

// 0x56c674 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
pub fn stub_56c674() -> ! {
    todo!("0x56c674 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")
}

// 0x56c67c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
pub fn stub_56c67c() -> ! {
    todo!("0x56c67c `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")
}

// 0x56c684 — __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f
// type: int(void)
#[doc(alias = "__ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f")]
#[doc(alias = "boost::function2<void,RBX::NormalId,float>::operator()(RBX::NormalId,float)const")]
// was: __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f
pub fn stub_56c684() -> ! {
    todo!("0x56c684 boost::function2<void,RBX::NormalId,float>::operator()(RBX::NormalId,float)const")
}

// 0x56c750 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
pub fn stub_56c750() -> ! {
    todo!("0x56c750 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")
}

// 0x56c860 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
pub fn stub_56c860() -> ! {
    todo!("0x56c860 rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")
}

// 0x56c990 — __ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_
// type: int(void)
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_")]
#[doc(alias = "boost::function2<void,RBX::NormalId,float>::assign_to_own(boost::function2<void,RBX::NormalId,float> const&)")]
// was: __ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_
pub fn stub_56c990() -> ! {
    todo!("0x56c990 boost::function2<void,RBX::NormalId,float>::assign_to_own(boost::function2<void,RBX::NormalId,float> const&)")
}

// 0x56c9c0 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::EventDesc(rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_56c9c0() -> ! {
    todo!("0x56c9c0 RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::EventDesc(rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x56cbb0 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED1Ev
pub fn stub_56cbb0() -> ! {
    todo!("0x56cbb0 RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::~EventDesc()")
}

// 0x56cbd4 — __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_7HandlesEFvNS_8NormalIdEfEN3rbx13remote_signalIS4_EEMS2_S7_ED0Ev
pub fn stub_56cbd4() -> ! {
    todo!("0x56cbd4 RBX::Reflection::EventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>,rbx::remote_signal<void ()(RBX::NormalId,float)> RBX::Handles::*>::~EventDesc()")
}

// 0x56cc88 — __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED0Ev
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED0Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEED0Ev
pub fn stub_56cc88() -> ! {
    todo!("0x56cc88 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")
}

// 0x56cd3c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_56cd3c() -> ! {
    todo!("0x56cd3c RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x56cea0 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12isScriptableEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12isScriptableEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE12isScriptableEv
pub fn stub_56cea0() -> ! {
    todo!("0x56cea0 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isScriptable(void)const")
}

// 0x56cea8 — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE11isBroadcastEv
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE11isBroadcastEv
pub fn stub_56cea8() -> ! {
    todo!("0x56cea8 RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::isBroadcast(void)const")
}

// 0x56ceb0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
pub fn stub_56ceb0() -> ! {
    todo!("0x56ceb0 RBX::Reflection::EventDescImpl<1,RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56cf3c — __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_56cf3c() -> ! {
    todo!("0x56cf3c RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x56cf4c — __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_7HandlesEFvNS_8NormalIdEEN3rbx13remote_signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_56cf4c() -> ! {
    todo!("0x56cf4c RBX::Reflection::EventDescBase<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>,rbx::remote_signal<void ()(RBX::NormalId)> RBX::Handles::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x56cf60 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_")]
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8NormalIdENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISD_T0_T1_EENSB_9list_av_2IT2_T3_E4typeEEEMSG_FSD_SH_ESK_SL_
pub fn stub_56cf60() -> ! {
    todo!("0x56cf60 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::NormalId const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0x56d07c — __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_8NormalIdEEEvRKT_
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_8NormalIdEEEvRKT_")]
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<RBX::NormalId>(RBX::NormalId const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute1INS_8NormalIdEEEvRKT_
pub fn stub_56d07c() -> ! {
    todo!("0x56d07c void RBX::Reflection::GenericSlotWrapper::execute1<RBX::NormalId>(RBX::NormalId const&)")
}

// 0x56d1c0 — __ZN5boost9function1IvN3RBX8NormalIdEE5clearEv
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE5clearEv")]
#[doc(alias = "boost::function1<void,RBX::NormalId>::clear(void)")]
// was: __ZN5boost9function1IvN3RBX8NormalIdEE5clearEv
pub fn stub_56d1c0() -> ! {
    todo!("0x56d1c0 boost::function1<void,RBX::NormalId>::clear(void)")
}

// 0x56d1ec — __ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_56d1ec() -> ! {
    todo!("0x56d1ec __ZN5boost8functionIFvN3RBX8NormalIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x56d2d0 — __ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_56d2d0() -> ! {
    todo!("0x56d2d0 __ZN5boost9function1IvN3RBX8NormalIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x56d3b8 — __ZN5boost9function1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
pub fn stub_56d3b8() -> ! {
    todo!("0x56d3b8 void boost::function1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0x56d4b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_56d4b0() -> ! {
    todo!("0x56d4b0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x56d4cc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::NormalId>::invoke(boost::detail::function::function_buffer &,RBX::NormalId)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8NormalIdEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
pub fn stub_56d4cc() -> ! {
    todo!("0x56d4cc boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,RBX::NormalId>::invoke(boost::detail::function::function_buffer &,RBX::NormalId)")
}

// 0x56d4e0 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_56d4e0() -> ! {
    todo!("0x56d4e0 bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x56d5c8 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvN3RBX8NormalIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_56d5c8() -> ! {
    todo!("0x56d5c8 bool boost::detail::function::basic_vtable1<void,RBX::NormalId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::NormalId const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

