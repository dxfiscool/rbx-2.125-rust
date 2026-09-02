//! rendering shard rend_wd_watchdog13 — 120 stubs 0x8068b4..0x80c4e8 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x8068b0
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x8068b4 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_8068b4() -> ! {
    todo!("0x8068b4 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")
}

// 0x8069a4 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD1Ev")]
pub fn stub_8069a4() -> ! {
    todo!("0x8069a4 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")
}

// 0x8069d0 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD0Ev")]
pub fn stub_8069d0() -> ! {
    todo!("0x8069d0 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")
}

// 0x806aa4 — __ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf4IvNS5_11TestServiceEbSsS7_iEENSB_5list5INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEELi4ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf4IvNS5_11TestServiceEbSsS7_iEENSB_5list5INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEELi4ES8_ED1Ev")]
pub fn stub_806aa4() -> ! {
    todo!("0x806aa4 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x806ad0 — __ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf4IvNS5_11TestServiceEbSsS7_iEENSB_5list5INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEELi4ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf4IvNS5_11TestServiceEbSsS7_iEENSB_5list5INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEELi4ES8_ED0Ev")]
pub fn stub_806ad0() -> ! {
    todo!("0x806ad0 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x806ba4 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_806ba4() -> ! {
    todo!("0x806ba4 rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0x806c18 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6insertEPNS8_4slotE")]
pub fn stub_806c18() -> ! {
    todo!("0x806c18 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")
}

// 0x806e24 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSEPSA_")]
pub fn stub_806e24() -> ! {
    todo!("0x806e24 rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot*)")
}

// 0x806e48 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSERKSB_")]
pub fn stub_806e48() -> ! {
    todo!("0x806e48 rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> const&)")
}

// 0x806e6c — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE22safe_static_init_mutexEv")]
pub fn stub_806e6c() -> ! {
    todo!("0x806e6c rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_init_mutex(void)")
}

// 0x806e70 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE24safe_static_do_get_mutexEv")]
pub fn stub_806e70() -> ! {
    todo!("0x806e70 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)")
}

// 0x806f68 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED1Ev")]
pub fn stub_806f68() -> ! {
    todo!("0x806f68 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x806f94 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEED0Ev")]
pub fn stub_806f94() -> ! {
    todo!("0x806f94 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x807068 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot10disconnectEv")]
pub fn stub_807068() -> ! {
    todo!("0x807068 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::disconnect(void)")
}

// 0x807178 — __ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot9connectedEv")]
pub fn stub_807178() -> ! {
    todo!("0x807178 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::connected(void)const")
}

// 0x807184 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callESsS7_i
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callESsS7_i")]
pub fn stub_807184() -> ! {
    todo!("0x807184 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x8071a8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callESsS7_i
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_E4callESsS7_i")]
pub fn stub_8071a8() -> ! {
    todo!("0x8071a8 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x8071cc — __ZN5boost3_bi5list4INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsNS_10shared_ptrINS3_8InstanceEEEiEENS0_5list3IRSsRSH_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsNS_10shared_ptrINS3_8InstanceEEEiEENS0_5list3IRSsRSH_RiEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_8071cc() -> ! {
    todo!("0x8071cc void boost::_bi::list4<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0x807348 — __ZNK5boost4_mfi3mf3IvN3RBX11TestServiceESsNS_10shared_ptrINS2_8InstanceEEEiEclEPS3_SsS6_i
#[doc(alias = "boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf3IvN3RBX11TestServiceESsNS_10shared_ptrINS2_8InstanceEEEiEclEPS3_SsS6_i")]
pub fn stub_807348() -> ! {
    todo!("0x807348 boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0x8074d4 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6removeEPNS8_4slotE")]
pub fn stub_8074d4() -> ! {
    todo!("0x8074d4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")
}

// 0x8075c4 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot22safe_static_init_mutexEv")]
pub fn stub_8075c4() -> ! {
    todo!("0x8075c4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_init_mutex(void)")
}

// 0x8075c8 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_8075c8() -> ! {
    todo!("0x8075c8 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")
}

// 0x8076b8 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD1Ev")]
pub fn stub_8076b8() -> ! {
    todo!("0x8076b8 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")
}

// 0x8076e4 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotD0Ev")]
pub fn stub_8076e4() -> ! {
    todo!("0x8076e4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::~slot()")
}

// 0x8077b8 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED1Ev")]
pub fn stub_8077b8() -> ! {
    todo!("0x8077b8 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x8077e4 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_11TestServiceESsS7_iEENSB_5list4INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEELi3ES8_ED0Ev")]
pub fn stub_8077e4() -> ! {
    todo!("0x8077e4 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~callable()")
}

// 0x8078b8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
pub fn stub_8078b8() -> ! {
    todo!("0x8078b8 boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x807918 — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEESsSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>,std::string,std::string const&>::invoke(boost::detail::function::function_buffer &,std::string const&)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEESsSA_E6invokeERNS1_15function_bufferESA_")]
pub fn stub_807918() -> ! {
    todo!("0x807918 boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>,std::string,std::string const&>::invoke(boost::detail::function::function_buffer &,std::string const&)")
}

// 0x807924 — __ZN5boost3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclISsEESsRKT_
#[doc(alias = "std::string boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>::operator()<std::string>(std::string const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclISsEESsRKT_")]
pub fn stub_807924() -> ! {
    todo!("0x807924 std::string boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>::operator()<std::string>(std::string const&)")
}

// 0x807944 — __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_807944() -> ! {
    todo!("0x807944 __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENSA_5list6INSA_5valueINS3_ISE_EEEENSH_IiEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSL_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0x807a2c — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_807a2c() -> ! {
    todo!("0x807a2c __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0x807b18 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_
#[doc(alias = "void boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
#[doc(alias = "__ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS4_11TestServiceEiS2_S2_S6_iEENS9_5list6INS9_5valueINS3_ISD_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEEvT_")]
pub fn stub_807b18() -> ! {
    todo!("0x807b18 void boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0x807c14 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
pub fn stub_807c14() -> ! {
    todo!("0x807c14 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x807c30 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEvSA_SA_SD_iE6invokeERNS1_15function_bufferESA_SA_SD_i
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::invoke(boost::detail::function::function_buffer &,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEvSA_SA_SD_iE6invokeERNS1_15function_bufferESA_SA_SD_i")]
pub fn stub_807c30() -> ! {
    todo!("0x807c30 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::invoke(boost::detail::function::function_buffer &,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)")
}

// 0x807c60 — __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_807c60() -> ! {
    todo!("0x807c60 bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")
}

// 0x807d4c — __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_807d4c() -> ! {
    todo!("0x807d4c bool boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x807e34 — __ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvPKcS4_NS_10shared_ptrIN3RBX10BaseScriptEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf5IvNS6_11TestServiceEiS4_S4_S8_iEENSB_5list6INSB_5valueINS5_ISF_EEEENSI_IiEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSM_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_807e34() -> ! {
    todo!("0x807e34 void boost::detail::function::basic_vtable4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x807f0c — __ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf5IvS5_iPKcSJ_NS3_INS4_10BaseScriptEEEiEENS0_5list4IRSJ_SO_RSL_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> &,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf5IvS5_iPKcSJ_NS3_INS4_10BaseScriptEEEiEENS0_5list4IRSJ_SO_RSL_RiEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_807f0c() -> ! {
    todo!("0x807f0c void boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> &,boost::_bi::list4<char const*&,char const*&,rbx_core::SharedPtr<RBX::BaseScript>&,int &> &,int)")
}

// 0x80800c — __ZNK5boost4_mfi3mf5IvN3RBX11TestServiceEiPKcS5_NS_10shared_ptrINS2_10BaseScriptEEEiE4callINS6_IS3_EEiS5_S5_S8_iEEvRT_PKvRT0_RT1_RT2_RT3_RT4_
#[doc(alias = "void boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::call<rbx_core::SharedPtr<RBX::TestService>,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>(rbx_core::SharedPtr<RBX::TestService> &,void const*,int &,char const* &,char const* &,rbx_core::SharedPtr<RBX::BaseScript> &,int &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf5IvN3RBX11TestServiceEiPKcS5_NS_10shared_ptrINS2_10BaseScriptEEEiE4callINS6_IS3_EEiS5_S5_S8_iEEvRT_PKvRT0_RT1_RT2_RT3_RT4_")]
pub fn stub_80800c() -> ! {
    todo!("0x80800c void boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::call<rbx_core::SharedPtr<RBX::TestService>,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>(rbx_core::SharedPtr<RBX::TestService> &,void const*,int &,char const* &,char const* &,rbx_core::SharedPtr<RBX::BaseScript> &,int &)const")
}

// 0x80811c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX11TestServiceEiPKcSA_NS_10shared_ptrINS7_10BaseScriptEEEiEENS3_5list6INS3_5valueINSB_IS8_EEEENSG_IiEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_80811c() -> ! {
    todo!("0x80811c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::TestService,int,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>,boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x808278 — __ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_")]
pub fn stub_808278() -> ! {
    todo!("0x808278 boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0x808350 — __ZN5boost3_bi8storage6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::storage6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_")]
pub fn stub_808350() -> ! {
    todo!("0x808350 boost::_bi::storage6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::storage6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0x808428 — __ZN5boost3_bi8storage5INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_SA_SB_SC_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage5(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_SA_SB_SC_")]
pub fn stub_808428() -> ! {
    todo!("0x808428 boost::_bi::storage5<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage5(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x808500 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_")]
pub fn stub_808500() -> ! {
    todo!("0x808500 boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")
}

// 0x8085d8 — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_")]
pub fn stub_8085d8() -> ! {
    todo!("0x8085d8 boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>)")
}

// 0x8086b0 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_")]
pub fn stub_8086b0() -> ! {
    todo!("0x8086b0 boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")
}

// 0x808798 — __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENSA_5list2INSA_5valueINS1_ISE_EEEENSH_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENSA_5list2INSA_5valueINS1_ISE_EEEENSH_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENSA_5list2INSA_5valueINS1_ISE_EEEENSH_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_808798() -> ! {
    todo!("0x808798 __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENSA_5list2INSA_5valueINS1_ISE_EEEENSH_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x808880 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENS9_5list2INS9_5valueINS1_ISD_EEEENSG_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENS9_5list2INS9_5valueINS1_ISD_EEEENSG_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENS9_5list2INS9_5valueINS1_ISD_EEEENSG_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_808880() -> ! {
    todo!("0x808880 __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENS9_5list2INS9_5valueINS1_ISD_EEEENSG_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x80896c — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENS9_5list2INS9_5valueINS1_ISD_EEEENSG_IiEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_11TestServiceEiEENS9_5list2INS9_5valueINS1_ISD_EEEENSG_IiEEEEEEEEvT_")]
pub fn stub_80896c() -> ! {
    todo!("0x80896c void boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>)")
}

// 0x808a68 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")]
pub fn stub_808a68() -> ! {
    todo!("0x808a68 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x808a84 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEvNSC_IKNS7_10Reflection5TupleEEEE6invokeERNS1_15function_bufferESL_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEvNSC_IKNS7_10Reflection5TupleEEEE6invokeERNS1_15function_bufferESL_")]
pub fn stub_808a84() -> ! {
    todo!("0x808a84 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")
}

// 0x808a8c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_11TestServiceEiEENSB_5list2INSB_5valueINS3_ISF_EEEENSI_IiEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_11TestServiceEiEENSB_5list2INSB_5valueINS3_ISF_EEEENSI_IiEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_808a8c() -> ! {
    todo!("0x808a8c bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")
}

// 0x808b78 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_11TestServiceEiEENSB_5list2INSB_5valueINS3_ISF_EEEENSI_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_11TestServiceEiEENSB_5list2INSB_5valueINS3_ISF_EEEENSI_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_808b78() -> ! {
    todo!("0x808b78 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x808c60 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_11TestServiceEiEENSB_5list2INSB_5valueINS3_ISF_EEEENSI_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_11TestServiceEiEENSB_5list2INSB_5valueINS3_ISF_EEEENSI_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_808c60() -> ! {
    todo!("0x808c60 void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x808d38 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS0_5list2INS0_5valueINS_10shared_ptrIS5_EEEENS8_IiEEEEEclINS9_IKNS4_10Reflection5TupleEEEEEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>::operator()<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS0_5list2INS0_5valueINS_10shared_ptrIS5_EEEENS8_IiEEEEEclINS9_IKNS4_10Reflection5TupleEEEEEvRT_")]
pub fn stub_808d38() -> ! {
    todo!("0x808d38 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>::operator()<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(rbx_core::SharedPtr<RBX::Reflection::Tuple const> &)")
}

// 0x808d50 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_808d50() -> ! {
    todo!("0x808d50 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x808eac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_")]
pub fn stub_808eac() -> ! {
    todo!("0x808eac boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")
}

// 0x808f84 — __ZN5boost20dynamic_pointer_castIN3RBX6ScriptENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost20dynamic_pointer_castIN3RBX6ScriptENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE")]
pub fn stub_808f84() -> ! {
    todo!("0x808f84 rbx_core::SharedPtr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x808fcc — __ZN5boost3_bi5list2INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::TestService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_808fcc() -> ! {
    todo!("0x808fcc void boost::_bi::list2<boost::_bi::value<RBX::TestService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x8090a4 — __ZNK5boost4_mfi3mf1IvN3RBX11TestServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::TestService*,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX11TestServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")]
pub fn stub_8090a4() -> ! {
    todo!("0x8090a4 boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::TestService*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x80918c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_80918c() -> ! {
    todo!("0x80918c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")
}

// 0x809280 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEEvT_")]
pub fn stub_809280() -> ! {
    todo!("0x809280 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>)")
}

// 0x809384 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_809384() -> ! {
    todo!("0x809384 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x8093a0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_8093a0() -> ! {
    todo!("0x8093a0 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x8093b8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_8093b8() -> ! {
    todo!("0x8093b8 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const")
}

// 0x8094ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_8094ac() -> ! {
    todo!("0x8094ac bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x80959c — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_80959c() -> ! {
    todo!("0x80959c void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x809680 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_809680() -> ! {
    todo!("0x809680 void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)")
}

// 0x8096a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_8096a4() -> ! {
    todo!("0x8096a4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x80980c — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")]
pub fn stub_80980c() -> ! {
    todo!("0x80980c boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")
}

// 0x8098ec — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")]
pub fn stub_8098ec() -> ! {
    todo!("0x8098ec boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")
}

// 0x8099d0 — __ZN5boost9function1IvdE5dummy7nonnullEv
#[doc(alias = "boost::function1<void,double>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function1IvdE5dummy7nonnullEv")]
pub fn stub_8099d0() -> ! {
    todo!("0x8099d0 boost::function1<void,double>::dummy::nonnull(void)")
}

// 0x8099d4 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_bSsS6_i
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_bSsS6_i")]
pub fn stub_8099d4() -> ! {
    todo!("0x8099d4 RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0x809b4c — __ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceEbSsS6_i
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::replicateEvent(RBX::Reflection::EventSource *,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceEbSsS6_i")]
pub fn stub_809b4c() -> ! {
    todo!("0x809b4c RBX::Reflection::RemoteEventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::replicateEvent(RBX::Reflection::EventSource *,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x809d04 — __ZN3rbx7signals16signal_with_argsILi4EFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEclEbSsS6_i
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::operator()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi4EFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEclEbSsS6_i")]
pub fn stub_809d04() -> ! {
    todo!("0x809d04 rbx::signals::signal_with_args<4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::operator()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x809f40 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")]
pub fn stub_809f40() -> ! {
    todo!("0x809f40 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")
}

// 0x80a0a0 — __ZN3rbx7signals16signal_with_argsILi4EFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8fireItemEPNS0_6signalIS7_E4slotEbSsS6_i
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::fireItem(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi4EFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8fireItemEPNS0_6signalIS7_E4slotEbSsS6_i")]
pub fn stub_80a0a0() -> ! {
    todo!("0x80a0a0 rbx::signals::signal_with_args<4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::fireItem(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80a224 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8on_errorERSt9exception")]
pub fn stub_80a224() -> ! {
    todo!("0x80a224 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")
}

// 0x80a24c — __ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE5dummy7nonnullEv
#[doc(alias = "boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE5dummy7nonnullEv")]
pub fn stub_80a24c() -> ! {
    todo!("0x80a24c boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::dummy::nonnull(void)")
}

// 0x80a250 — __ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE5dummy7nonnullEv
#[doc(alias = "boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE5dummy7nonnullEv")]
pub fn stub_80a250() -> ! {
    todo!("0x80a250 boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::dummy::nonnull(void)")
}

// 0x80a254 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_i
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_i")]
pub fn stub_80a254() -> ! {
    todo!("0x80a254 RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0x80a3c8 — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_i
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceESsS6_i")]
pub fn stub_80a3c8() -> ! {
    todo!("0x80a3c8 RBX::Reflection::RemoteEventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::replicateEvent(RBX::Reflection::EventSource *,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80a55c — __ZN3rbx7signals16signal_with_argsILi3EFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEclESsS6_i
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEclESsS6_i")]
pub fn stub_80a55c() -> ! {
    todo!("0x80a55c rbx::signals::signal_with_args<3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80a790 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")]
pub fn stub_80a790() -> ! {
    todo!("0x80a790 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")
}

// 0x80a8f0 — __ZN3rbx7signals16signal_with_argsILi3EFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8fireItemEPNS0_6signalIS7_E4slotESsS6_i
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::fireItem(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8fireItemEPNS0_6signalIS7_E4slotESsS6_i")]
pub fn stub_80a8f0() -> ! {
    todo!("0x80a8f0 rbx::signals::signal_with_args<3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::fireItem(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80aa70 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8on_errorERSt9exception")]
pub fn stub_80aa70() -> ! {
    todo!("0x80aa70 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")
}

// 0x80aa98 — __ZN3rbx13remote_signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEC2Ev")]
pub fn stub_80aa98() -> ! {
    todo!("0x80aa98 rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remote_signal(void)")
}

// 0x80abf4 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13disconnectAllEv")]
pub fn stub_80abf4() -> ! {
    todo!("0x80abf4 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::disconnectAll(void)")
}

// 0x80ad6c — __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEEC2Ev")]
pub fn stub_80ad6c() -> ! {
    todo!("0x80ad6c rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remote_signal(void)")
}

// 0x80aec8 — __ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13disconnectAllEv")]
pub fn stub_80aec8() -> ! {
    todo!("0x80aec8 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::disconnectAll(void)")
}

// 0x80b040 — __ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_80b040() -> ! {
    todo!("0x80b040 __ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x80b044 — __ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_80b044() -> ! {
    todo!("0x80b044 __ZN3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x80b0e4 — __ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_80b0e4() -> ! {
    todo!("0x80b0e4 __ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x80b0ec — __ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_80b0ec() -> ! {
    todo!("0x80b0ec __ZThn32_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x80b190 — __ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_80b190() -> ! {
    todo!("0x80b190 __ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x80b198 — __ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_80b198() -> ! {
    todo!("0x80b198 __ZThn36_N3RBX10Reflection9DescribedINS_11TestServiceELZNS_12sTestServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sTestServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x80b23c — __ZN3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEED0Ev")]
pub fn stub_80b23c() -> ! {
    todo!("0x80b23c RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~RemoteEventDesc()")
}

// 0x80b2f0 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_80b2f0() -> ! {
    todo!("0x80b2f0 RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x80b454 — __ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE12isScriptableEv")]
pub fn stub_80b454() -> ! {
    todo!("0x80b454 RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isScriptable(void)const")
}

// 0x80b45c — __ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE11isBroadcastEv")]
pub fn stub_80b45c() -> ! {
    todo!("0x80b45c RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::isBroadcast(void)const")
}

// 0x80b464 — __ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
pub fn stub_80b464() -> ! {
    todo!("0x80b464 RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x80b694 — __ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE")]
pub fn stub_80b694() -> ! {
    todo!("0x80b694 RBX::Reflection::RemoteEventDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x80b6a4 — __ZNK3RBX10Reflection13EventDescBaseINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_80b6a4() -> ! {
    todo!("0x80b6a4 RBX::Reflection::EventDescBase<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x80b6b8 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS1_8InstanceEEERKiNS8_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEENSG_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISN_T0_T1_T2_T3_T4_EENSL_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSQ_FSN_SR_SS_ST_SU_ESX_SY_SZ_S10_S11_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS1_8InstanceEEERKiNS8_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEENSG_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISN_T0_T1_T2_T3_T4_EENSL_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSQ_FSN_SR_SS_ST_SU_ESX_SY_SZ_S10_S11_")]
pub fn stub_80b6b8() -> ! {
    todo!("0x80b6b8 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0x80b7d4 — __ZN3RBX10Reflection18GenericSlotWrapper8execute4IbSsN5boost10shared_ptrINS_8InstanceEEEiEEvRKT_RKT0_RKT1_RKT2_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>(bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute4IbSsN5boost10shared_ptrINS_8InstanceEEEiEEvRKT_RKT0_RKT1_RKT2_")]
pub fn stub_80b7d4() -> ! {
    todo!("0x80b7d4 void RBX::Reflection::GenericSlotWrapper::execute4<bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>(bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&)")
}

// 0x80b980 — __ZN5boost8functionIFvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS8_5list5INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS8_5list5INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS8_5list5INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")]
pub fn stub_80b980() -> ! {
    todo!("0x80b980 __ZN5boost8functionIFvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS8_5list5INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")
}

// 0x80ba64 — __ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
pub fn stub_80ba64() -> ! {
    todo!("0x80ba64 __ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")
}

// 0x80bb4c — __ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEEvT_
#[doc(alias = "void boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
#[doc(alias = "__ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEEvT_")]
pub fn stub_80bb4c() -> ! {
    todo!("0x80bb4c void boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0x80bc44 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE6manageERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeE")]
pub fn stub_80bc44() -> ! {
    todo!("0x80bc44 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x80bc60 — __ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEvbSsSG_iE6invokeERNS1_15function_bufferEbSsSG_i
#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::invoke(boost::detail::function::function_buffer &,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker4INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEvbSsSG_iE6invokeERNS1_15function_bufferEbSsSG_i")]
pub fn stub_80bc60() -> ! {
    todo!("0x80bc60 boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::invoke(boost::detail::function::function_buffer &,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0x80bc90 — __ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_80bc90() -> ! {
    todo!("0x80bc90 bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")
}

// 0x80bd78 — __ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_80bd78() -> ! {
    todo!("0x80bd78 bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x80be5c — __ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_80be5c() -> ! {
    todo!("0x80be5c void boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x80bf30 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKbRKSsRKNS3_INS4_8InstanceEEERKiEENS0_5list4IRbRSsRSN_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKbRKSsRKNS3_INS4_8InstanceEEERKiEENS0_5list4IRbRSsRSN_RiEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_80bf30() -> ! {
    todo!("0x80bf30 void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0x80bf60 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_80bf60() -> ! {
    todo!("0x80bf60 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x80c0b8 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_")]
pub fn stub_80c0b8() -> ! {
    todo!("0x80c0b8 rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")
}

// 0x80c1ac — __ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_EC2IPS9_EERKSC_T_")]
pub fn stub_80c1ac() -> ! {
    todo!("0x80c1ac rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")
}

// 0x80c2a8 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED1Ev")]
pub fn stub_80c2a8() -> ! {
    todo!("0x80c2a8 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")
}

// 0x80c3b8 — __ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13callable_slotINS2_8functionIS7_EEED0Ev")]
pub fn stub_80c3b8() -> ! {
    todo!("0x80c3b8 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable_slot<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>::~callable_slot()")
}

// 0x80c4e8 — __ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_E4callEbSsS7_i
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_E4callEbSsS7_i")]
pub fn stub_80c4e8() -> ! {
    todo!("0x80c4e8 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}
