// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xf54d54..0xf58354 | total filtered 10215, remaining 688 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xf54d54 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)
pub fn stub_0xf54d54() -> ! {
    todo!("0xf54d54 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)")
}

// 0xf54d64 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot> &)
pub fn stub_0xf54d64() -> ! {
    todo!("0xf54d64 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")
}

// 0xf54d74 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0xf54d74() -> ! {
    todo!("0xf54d74 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")
}

// 0xf54d84 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot *)
pub fn stub_0xf54d84() -> ! {
    todo!("0xf54d84 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")
}

// 0xf54d94 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot *)
pub fn stub_0xf54d94() -> ! {
    todo!("0xf54d94 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")
}

// 0xf54da4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_11TestServiceESsS6_iEENSA_5list4INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,boost::shared_ptr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,boost::shared_ptr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)
pub fn stub_0xf54da4() -> ! {
    todo!("0xf54da4 rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0xf54db4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::connect<boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>>(boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)> const&)
pub fn stub_0xf54db4() -> ! {
    todo!("0xf54db4 rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")
}

// 0xf54dc4 — j___ZN3rbx7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::on_error(std::exception &)
pub fn stub_0xf54dc4() -> ! {
    todo!("0xf54dc4 rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")
}

// 0xf54dd4 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::disconnectAll(void)
pub fn stub_0xf54dd4() -> ! {
    todo!("0xf54dd4 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::disconnectAll(void)")
}

// 0xf54de4 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)
pub fn stub_0xf54de4() -> ! {
    todo!("0xf54de4 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::safe_static_do_get_mutex(void)")
}

// 0xf54df4 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot> &)
pub fn stub_0xf54df4() -> ! {
    todo!("0xf54df4 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> &)")
}

// 0xf54e04 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0xf54e04() -> ! {
    todo!("0xf54e04 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot::safe_static_do_get_mutex(void)")
}

// 0xf54e14 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot *)
pub fn stub_0xf54e14() -> ! {
    todo!("0xf54e14 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::insert(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")
}

// 0xf54e24 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot *)
pub fn stub_0xf54e24() -> ! {
    todo!("0xf54e24 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::remove(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot *)")
}

// 0xf54e34 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf4IvNS4_11TestServiceEbSsS6_iEENSA_5list5INSA_5valueIPSE_EENS2_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSK_ILi4EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,boost::shared_ptr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,boost::shared_ptr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)
pub fn stub_0xf54e34() -> ! {
    todo!("0xf54e34 rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list5<boost::_bi::value<RBX::TestService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")
}

// 0xf54e44 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::connect<boost::function<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>>(boost::function<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)> const&)
pub fn stub_0xf54e44() -> ! {
    todo!("0xf54e44 rbx::signals::connection rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::connect<boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&)")
}

// 0xf54e54 — j___ZN3rbx7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::on_error(std::exception &)
pub fn stub_0xf54e54() -> ! {
    todo!("0xf54e54 rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::on_error(std::exception &)")
}

// 0xf54e64 — j___ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_E4callESsS7_i
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>,3,void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::call(std::string,boost::shared_ptr<RBX::Instance>,int)
pub fn stub_0xf54e64() -> ! {
    todo!("0xf54e64 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0xf54e74 — j___ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>,3,void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>*>(boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>*)
pub fn stub_0xf54e74() -> ! {
    todo!("0xf54e74 rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,3,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")
}

// 0xf54e84 — j___ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_E4callEbSsS7_i
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")]
// was: rbx::callable<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>,4,void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::call(bool,std::string,boost::shared_ptr<RBX::Instance>,int)
pub fn stub_0xf54e84() -> ! {
    todo!("0xf54e84 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::call(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)")
}

// 0xf54e94 — j___ZN3rbx8callableINS_7signals6signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEE4slotENS3_8functionIS8_EELi4ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>,4,void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>*>(boost::function<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>*)
pub fn stub_0xf54e94() -> ! {
    todo!("0xf54e94 rbx::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot,boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,4,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::callable<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*>(boost::function<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> const&,rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>*)")
}

// 0xf54eb4 — j___ZN5boost10shared_ptrIN3RBX14FunctionalTestEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::FunctionalTest>::shared_ptr<RBX::FunctionalTest,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::FunctionalTest>::shared_ptr<RBX::FunctionalTest,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf54eb4() -> ! {
    todo!("0xf54eb4 rbx_core::SharedPtr<RBX::FunctionalTest>::shared_ptr<RBX::FunctionalTest,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf54ec4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot*)
pub fn stub_0xf54ec4() -> ! {
    todo!("0xf54ec4 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot*)")
}

// 0xf54ed4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>::slot> const&)
pub fn stub_0xf54ed4() -> ! {
    todo!("0xf54ed4 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> const&)")
}

// 0xf54ee4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot*)
pub fn stub_0xf54ee4() -> ! {
    todo!("0xf54ee4 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot*)")
}

// 0xf54ef4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>::slot> const&)
pub fn stub_0xf54ef4() -> ! {
    todo!("0xf54ef4 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::slot> const&)")
}

// 0xf54f04 — j___ZN5boost20dynamic_pointer_castIN3RBX6ScriptENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xf54f04() -> ! {
    todo!("0xf54f04 rbx_core::SharedPtr<RBX::Script> boost::dynamic_pointer_cast<RBX::Script,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf54f24 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::TestService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::TestService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::TestService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::TestService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_0xf54f24() -> ! {
    todo!("0xf54f24 void boost::_bi::list2<boost::_bi::value<RBX::TestService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf54f54 — j___ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsRKNS3_INS4_8InstanceEEERKiEENS0_5list3IRSsRSK_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list3<std::string &,boost::shared_ptr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&> &,boost::_bi::list3<std::string &,boost::shared_ptr<RBX::Instance>&,int &> &,int)
pub fn stub_0xf54f54() -> ! {
    todo!("0xf54f54 void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0xf54f64 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsNS_10shared_ptrINS3_8InstanceEEEiEENS0_5list3IRSsRSH_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::TestService,std::string,boost::shared_ptr<RBX::Instance>,int>,boost::_bi::list3<std::string &,boost::shared_ptr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::TestService,std::string,boost::shared_ptr<RBX::Instance>,int> &,boost::_bi::list3<std::string &,boost::shared_ptr<RBX::Instance>&,int &> &,int)
pub fn stub_0xf54f64() -> ! {
    todo!("0xf54f64 void boost::_bi::list4<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int> &,boost::_bi::list3<std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0xf54f74 — j___ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEclINS_4_mfi3mf4IvS6_RKbRKSsRKNS3_INS4_8InstanceEEERKiEENS0_5list4IRbRSsRSN_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<bool &,std::string &,boost::shared_ptr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&> &,boost::_bi::list4<bool &,std::string &,boost::shared_ptr<RBX::Instance>&,int &> &,int)
pub fn stub_0xf54f74() -> ! {
    todo!("0xf54f74 void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&> &,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0xf54f84 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX11TestServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_bSsNS_10shared_ptrINS3_8InstanceEEEiEENS0_5list4IRbRSsRSI_RiEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int> &,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::TestService,bool,std::string,boost::shared_ptr<RBX::Instance>,int>,boost::_bi::list4<bool &,std::string &,boost::shared_ptr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,boost::shared_ptr<RBX::Instance>,int> &,boost::_bi::list4<bool &,std::string &,boost::shared_ptr<RBX::Instance>&,int &> &,int)
pub fn stub_0xf54f84() -> ! {
    todo!("0xf54f84 void boost::_bi::list5<boost::_bi::value<RBX::TestService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int> &,boost::_bi::list4<bool &,std::string &,rbx_core::SharedPtr<RBX::Instance>&,int &> &,int)")
}

// 0xf55034 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS1_8InstanceEEERKiNS6_IS3_EENS_3argILi1EEENSE_ILi2EEENSE_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISK_T0_T1_T2_T3_EENSI_9list_av_4IT4_T5_T6_T7_E4typeEEEMSN_FSK_SO_SP_SQ_EST_SU_SV_SW_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_0xf55034() -> ! {
    todo!("0xf55034 boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0xf55044 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS1_8InstanceEEERKiNS8_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEENSG_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISN_T0_T1_T2_T3_T4_EENSL_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSQ_FSN_SR_SS_ST_SU_ESX_SY_SZ_S10_S11_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_0xf55044() -> ! {
    todo!("0xf55044 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0xf55084 — j___ZN5boost6detail12shared_countC2IPN3RBX14FunctionalTestENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0xf55084() -> ! {
    todo!("0xf55084 boost::detail::shared_count::shared_count<RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FunctionalTest *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf550b4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf550b4() -> ! {
    todo!("0xf550b4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf550c4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf550c4() -> ! {
    todo!("0xf550c4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf55194 — j___ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE13assign_to_ownERKS5_
#[doc(alias = "boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")]
// was: boost::function3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to_own(boost::function3<void,std::string,boost::shared_ptr<RBX::Instance>,int> const&)
pub fn stub_0xf55194() -> ! {
    todo!("0xf55194 boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")
}

// 0xf551a4 — j___ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE5clearEv
#[doc(alias = "boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")]
// was: boost::function3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::clear(void)
pub fn stub_0xf551a4() -> ! {
    todo!("0xf551a4 boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")
}

// 0xf551b4 — j___ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsRKS4_RKiEENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: void boost::function3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)
pub fn stub_0xf551b4() -> ! {
    todo!("0xf551b4 void boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0xf551f4 — j___ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE13assign_to_ownERKS5_
#[doc(alias = "boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")]
// was: boost::function4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to_own(boost::function4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int> const&)
pub fn stub_0xf551f4() -> ! {
    todo!("0xf551f4 boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to_own(boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int> const&)")
}

// 0xf55204 — j___ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE5clearEv
#[doc(alias = "boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")]
// was: boost::function4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::clear(void)
pub fn stub_0xf55204() -> ! {
    todo!("0xf55204 boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")
}

// 0xf55214 — j___ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKbRKSsRKS4_RKiEENS7_5list5INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEEEvT_
#[doc(alias = "void boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
// was: void boost::function4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)
pub fn stub_0xf55214() -> ! {
    todo!("0xf55214 void boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")
}

// 0xf55234 — j___ZNK3RBX10Reflection13EventDescImplILi3ENS_11TestServiceEFvSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_SsS6_i
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
// was: RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,boost::shared_ptr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,std::string,boost::shared_ptr<RBX::Instance>,int)const
pub fn stub_0xf55234() -> ! {
    todo!("0xf55234 RBX::Reflection::EventDescImpl<3,RBX::TestService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0xf55244 — j___ZNK3RBX10Reflection13EventDescImplILi4ENS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_bSsS6_i
#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
// was: RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,bool,std::string,boost::shared_ptr<RBX::Instance>,int)const
pub fn stub_0xf55244() -> ! {
    todo!("0xf55244 RBX::Reflection::EventDescImpl<4,RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>,rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)> RBX::TestService::*>::fireEvent(RBX::TestService*,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0xf55274 — j___ZNK5boost4_mfi3mf1IvN3RBX11TestServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::TestService*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::TestService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::TestService*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0xf55274() -> ! {
    todo!("0xf55274 boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::TestService*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xf55284 — j___ZNK5boost4_mfi3mf3IvN3RBX11TestServiceESsNS_10shared_ptrINS2_8InstanceEEEiEclEPS3_SsS6_i
#[doc(alias = "boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
// was: boost::_mfi::mf3<void,RBX::TestService,std::string,boost::shared_ptr<RBX::Instance>,int>::operator()(RBX::TestService*,std::string,boost::shared_ptr<RBX::Instance>,int)const
pub fn stub_0xf55284() -> ! {
    todo!("0xf55284 boost::_mfi::mf3<void,RBX::TestService,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(RBX::TestService*,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0xf55294 — j___ZNK5boost4_mfi3mf4IvN3RBX11TestServiceEbSsNS_10shared_ptrINS2_8InstanceEEEiEclEPS3_bSsS6_i
#[doc(alias = "boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(RBX::TestService*,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
// was: boost::_mfi::mf4<void,RBX::TestService,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::operator()(RBX::TestService*,bool,std::string,boost::shared_ptr<RBX::Instance>,int)const
pub fn stub_0xf55294() -> ! {
    todo!("0xf55294 boost::_mfi::mf4<void,RBX::TestService,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(RBX::TestService*,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0xf55314 — j___ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf55314() -> ! {
    todo!("0xf55314 void boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf55324 — j___ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf55324() -> ! {
    todo!("0xf55324 bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0xf55334 — j___ZNK5boost6detail8function13basic_vtable3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKSsRKS6_RKiEENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf55334() -> ! {
    todo!("0xf55334 bool boost::detail::function::basic_vtable3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf55374 — j___ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf55374() -> ! {
    todo!("0xf55374 void boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf55384 — j___ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf55384() -> ! {
    todo!("0xf55384 bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")
}

// 0xf55394 — j___ZNK5boost6detail8function13basic_vtable4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvNS4_10Reflection18GenericSlotWrapperERKbRKSsRKS6_RKiEENS9_5list5INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,boost::shared_ptr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf55394() -> ! {
    todo!("0xf55394 bool boost::detail::function::basic_vtable4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,bool const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,int const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf553a4 — j___ZNK5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiEclESsS4_i
#[doc(alias = "boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
// was: boost::function3<void,std::string,boost::shared_ptr<RBX::Instance>,int>::operator()(std::string,boost::shared_ptr<RBX::Instance>,int)const
pub fn stub_0xf553a4() -> ! {
    todo!("0xf553a4 boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0xf553b4 — j___ZNK5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiEclEbSsS4_i
#[doc(alias = "boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")]
// was: boost::function4<void,bool,std::string,boost::shared_ptr<RBX::Instance>,int>::operator()(bool,std::string,boost::shared_ptr<RBX::Instance>,int)const
pub fn stub_0xf553b4() -> ! {
    todo!("0xf553b4 boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::operator()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)const")
}

// 0xf55484 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_11TestServiceES6_EENSD_5list2INSD_5valueIPSH_EENS2_3argILi1EEEEEEEET0_T_SS_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>)
pub fn stub_0xf55484() -> ! {
    todo!("0xf55484 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>)")
}

// 0xf56164 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13FriendServiceEEEEENS2_IiEENS2_INS_10shared_ptrISt3mapIiNS5_12FriendStatusESt4lessIiESaISt4pairIKiSB_EEEEEEEEclIPFvS6_iSJ_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::operator()<void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0xf56164() -> ! {
    todo!("0xf56164 void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::operator()<void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0xf56364 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS9_12FriendStatusESt4lessIiESaISt4pairIKiSD_EEEEEENS6_5list3INS6_5valueISA_EENSP_IiEENSP_ISL_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>)
pub fn stub_0xf56364() -> ! {
    todo!("0xf56364 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>)")
}

// 0xf56474 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13FriendServiceEEEiNS_10shared_ptrISt3mapIiNSB_12FriendStatusESt4lessIiESaISt4pairIKiSF_EEEEEENS8_5list3INS8_5valueISC_EENSR_IiEENSR_ISN_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf56474() -> ! {
    todo!("0xf56474 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf56484 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13FriendServiceEEEiNS_10shared_ptrISt3mapIiNSB_12FriendStatusESt4lessIiESaISt4pairIKiSF_EEEEEENS8_5list3INS8_5valueISC_EENSR_IiEENSR_ISN_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf56484() -> ! {
    todo!("0xf56484 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf56494 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13FriendServiceEEEiNS_10shared_ptrISt3mapIiNSB_12FriendStatusESt4lessIiESaISt4pairIKiSF_EEEEEENS8_5list3INS8_5valueISC_EENSR_IiEENSR_ISN_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf56494() -> ! {
    todo!("0xf56494 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::FriendService>>,boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf56e24 — j___ZN3RBX9CreatableINS_8InstanceEE6createIN4FLog19FastLogSettingsItemEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<FLog::FastLogSettingsItem> RBX::Creatable<RBX::Instance>::create<FLog::FastLogSettingsItem>(void)")]
// was: boost::shared_ptr<FLog::FastLogSettingsItem> RBX::Creatable<RBX::Instance>::create<FLog::FastLogSettingsItem>(void)
pub fn stub_0xf56e24() -> ! {
    todo!("0xf56e24 rbx_core::SharedPtr<FLog::FastLogSettingsItem> RBX::Creatable<RBX::Instance>::create<FLog::FastLogSettingsItem>(void)")
}

// 0xf56e44 — j___ZN5boost10shared_ptrIN4FLog19FastLogSettingsItemEEC2IS2_N3RBX9CreatableINS5_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<FLog::FastLogSettingsItem>::shared_ptr<FLog::FastLogSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<FLog::FastLogSettingsItem>::shared_ptr<FLog::FastLogSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf56e44() -> ! {
    todo!("0xf56e44 rbx_core::SharedPtr<FLog::FastLogSettingsItem>::shared_ptr<FLog::FastLogSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf56e54 — j___ZN5boost6detail12shared_countC2IPN4FLog19FastLogSettingsItemEN3RBX9CreatableINS6_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0xf56e54() -> ! {
    todo!("0xf56e54 boost::detail::shared_count::shared_count<FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(FLog::FastLogSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf56e84 — j___ZN3RBX10Reflection11Call0HelperINS_11CustomEventEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::CustomEvent,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::CustomEvent*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::CustomEvent,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::CustomEvent*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),RBX::Reflection::Variant &)
pub fn stub_0xf56e84() -> ! {
    todo!("0xf56e84 RBX::Reflection::Call0Helper<RBX::CustomEvent,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::CustomEvent*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),RBX::Reflection::Variant &)")
}

// 0xf56e94 — j___ZN3RBX10Reflection13BoundFuncDescINS_11CustomEventEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CustomEvent,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::CustomEvent,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf56e94() -> ! {
    todo!("0xf56e94 RBX::Reflection::BoundFuncDesc<RBX::CustomEvent,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::CustomEvent::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf56ed4 — j___ZN3RBX10Reflection9EventDescINS_11CustomEventEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CustomEvent,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEvent::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::CustomEvent,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CustomEvent::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CustomEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf56ed4() -> ! {
    todo!("0xf56ed4 RBX::Reflection::EventDesc<RBX::CustomEvent,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEvent::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf56f04 — j___ZN3RBX10Reflection17RefPropDescriptorINS_19CustomEventReceiverENS_8InstanceEEC2IMS2_KFKPS3_vEMS2_FvS6_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::CustomEventReceiver,RBX::Instance>::RefPropDescriptor<RBX::Instance* const (RBX::CustomEventReceiver::*)(void)const,void (RBX::CustomEventReceiver::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* const (RBX::CustomEventReceiver::*)(void)const,void (RBX::CustomEventReceiver::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xf56f04() -> ! {
    todo!("0xf56f04 RBX::Reflection::RefPropDescriptor<RBX::CustomEventReceiver,RBX::Instance>::RefPropDescriptor<RBX::Instance* const (RBX::CustomEventReceiver::*)(void)const,void (RBX::CustomEventReceiver::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* const (RBX::CustomEventReceiver::*)(void)const,void (RBX::CustomEventReceiver::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf56f14 — j___ZN3RBX10Reflection9EventDescINS_19CustomEventReceiverEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CustomEventReceiver,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEventReceiver::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEventReceiver::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::CustomEventReceiver,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CustomEventReceiver::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CustomEventReceiver::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf56f14() -> ! {
    todo!("0xf56f14 RBX::Reflection::EventDesc<RBX::CustomEventReceiver,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEventReceiver::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CustomEventReceiver::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf56f94 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_19CustomEventReceiverENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::CustomEventReceiver,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0xf56f94() -> ! {
    todo!("0xf56f94 RBX::Reflection::RefPropDescriptor<RBX::CustomEventReceiver,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf57044 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12TextureTrailEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::TextureTrail> RBX::Creatable<RBX::Instance>::create<RBX::TextureTrail>(void)")]
// was: boost::shared_ptr<RBX::TextureTrail> RBX::Creatable<RBX::Instance>::create<RBX::TextureTrail>(void)
pub fn stub_0xf57044() -> ! {
    todo!("0xf57044 rbx_core::SharedPtr<RBX::TextureTrail> RBX::Creatable<RBX::Instance>::create<RBX::TextureTrail>(void)")
}

// 0xf57054 — j___ZN5boost10shared_ptrIN3RBX12TextureTrailEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TextureTrail>::shared_ptr<RBX::TextureTrail,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::TextureTrail>::shared_ptr<RBX::TextureTrail,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf57054() -> ! {
    todo!("0xf57054 rbx_core::SharedPtr<RBX::TextureTrail>::shared_ptr<RBX::TextureTrail,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf57064 — j___ZN5boost6detail12shared_countC2IPN3RBX12TextureTrailENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0xf57064() -> ! {
    todo!("0xf57064 boost::detail::shared_count::shared_count<RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextureTrail *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf57074 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_12TextureTrailENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::TextureTrail,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0xf57074() -> ! {
    todo!("0xf57074 RBX::Reflection::RefPropDescriptor<RBX::TextureTrail,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf573d4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_9FloorWireEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::FloorWire> RBX::Creatable<RBX::Instance>::create<RBX::FloorWire>(void)")]
// was: boost::shared_ptr<RBX::FloorWire> RBX::Creatable<RBX::Instance>::create<RBX::FloorWire>(void)
pub fn stub_0xf573d4() -> ! {
    todo!("0xf573d4 rbx_core::SharedPtr<RBX::FloorWire> RBX::Creatable<RBX::Instance>::create<RBX::FloorWire>(void)")
}

// 0xf573e4 — j___ZN5boost10shared_ptrIN3RBX9FloorWireEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::FloorWire>::shared_ptr<RBX::FloorWire,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::FloorWire>::shared_ptr<RBX::FloorWire,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf573e4() -> ! {
    todo!("0xf573e4 rbx_core::SharedPtr<RBX::FloorWire>::shared_ptr<RBX::FloorWire,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf573f4 — j___ZN5boost6detail12shared_countC2IPN3RBX9FloorWireENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0xf573f4() -> ! {
    todo!("0xf573f4 boost::detail::shared_count::shared_count<RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FloorWire *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf57404 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0xf57404() -> ! {
    todo!("0xf57404 RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf580d4 — j___ZN3RBX10Reflection11Call0HelperINS_13PluginManagerEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),rbx_core::SharedPtr<RBX::Instance>>::call(RBX::PluginManager*,rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::PluginManager,boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),boost::shared_ptr<RBX::Instance>>::call(RBX::PluginManager*,boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),RBX::Reflection::Variant &)
pub fn stub_0xf580d4() -> ! {
    todo!("0xf580d4 RBX::Reflection::Call0Helper<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),rbx_core::SharedPtr<RBX::Instance>>::call(RBX::PluginManager*,rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),RBX::Reflection::Variant &)")
}

// 0xf580e4 — j___ZN3RBX10Reflection11Call0HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Plugin*,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::Plugin,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),boost::shared_ptr<RBX::Instance>>::call(RBX::Plugin*,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),RBX::Reflection::Variant &)
pub fn stub_0xf580e4() -> ! {
    todo!("0xf580e4 RBX::Reflection::Call0Helper<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Plugin*,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),RBX::Reflection::Variant &)")
}

// 0xf580f4 — j___ZN3RBX10Reflection11Call1HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEESsESsS6_E4callEPS2_S8_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Plugin*,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Plugin,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Plugin*,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),RBX::Reflection::Variant &,std::string const&)
pub fn stub_0xf580f4() -> ! {
    todo!("0xf580f4 RBX::Reflection::Call1Helper<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Plugin*,rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0xf58104 — j___ZN3RBX10Reflection11Call3HelperINS_7ToolbarEMS2_FN5boost10shared_ptrINS_8InstanceEEESsSsSsESsSsSsS6_E4callEPS2_S8_RNS0_7VariantERKSsSE_SE_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Toolbar,rbx_core::SharedPtr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),std::string,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Toolbar*,rbx_core::SharedPtr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&)")]
// was: RBX::Reflection::Call3Helper<RBX::Toolbar,boost::shared_ptr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),std::string,std::string,std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Toolbar*,boost::shared_ptr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&)
pub fn stub_0xf58104() -> ! {
    todo!("0xf58104 RBX::Reflection::Call3Helper<RBX::Toolbar,rbx_core::SharedPtr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),std::string,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Toolbar*,rbx_core::SharedPtr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&)")
}

// 0xf58114 — j___ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf58114() -> ! {
    todo!("0xf58114 RBX::Reflection::BoundFuncDesc<RBX::PluginManager,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::PluginManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf58144 — j___ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_0xf58144() -> ! {
    todo!("0xf58144 RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf58154 — j___ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS2_FS6_SsEPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf58154() -> ! {
    todo!("0xf58154 RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf58164 — j___ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf58164() -> ! {
    todo!("0xf58164 RBX::Reflection::BoundFuncDesc<RBX::Plugin,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Plugin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf58194 — j___ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,rbx_core::SharedPtr<RBX::Instance> ()(std::string,std::string,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_0xf58194() -> ! {
    todo!("0xf58194 RBX::Reflection::BoundFuncDesc<RBX::Toolbar,rbx_core::SharedPtr<RBX::Instance> ()(std::string,std::string,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0xf581a4 — j___ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EEC2EMS2_FS6_SsSsSsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,rbx_core::SharedPtr<RBX::Instance> ()(std::string,std::string,std::string),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xf581a4() -> ! {
    todo!("0xf581a4 RBX::Reflection::BoundFuncDesc<RBX::Toolbar,rbx_core::SharedPtr<RBX::Instance> ()(std::string,std::string,std::string),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf58294 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_11PluginMouseEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PluginMouse> RBX::Creatable<RBX::Instance>::create<RBX::PluginMouse>(void)")]
// was: boost::shared_ptr<RBX::PluginMouse> RBX::Creatable<RBX::Instance>::create<RBX::PluginMouse>(void)
pub fn stub_0xf58294() -> ! {
    todo!("0xf58294 rbx_core::SharedPtr<RBX::PluginMouse> RBX::Creatable<RBX::Instance>::create<RBX::PluginMouse>(void)")
}

// 0xf582a4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_13PluginManagerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PluginManager> RBX::Creatable<RBX::Instance>::create<RBX::PluginManager>(void)")]
// was: boost::shared_ptr<RBX::PluginManager> RBX::Creatable<RBX::Instance>::create<RBX::PluginManager>(void)
pub fn stub_0xf582a4() -> ! {
    todo!("0xf582a4 rbx_core::SharedPtr<RBX::PluginManager> RBX::Creatable<RBX::Instance>::create<RBX::PluginManager>(void)")
}

// 0xf582b4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_6ButtonEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Button> RBX::Creatable<RBX::Instance>::create<RBX::Button>(void)")]
// was: boost::shared_ptr<RBX::Button> RBX::Creatable<RBX::Instance>::create<RBX::Button>(void)
pub fn stub_0xf582b4() -> ! {
    todo!("0xf582b4 rbx_core::SharedPtr<RBX::Button> RBX::Creatable<RBX::Instance>::create<RBX::Button>(void)")
}

// 0xf582c4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_6PluginEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Plugin> RBX::Creatable<RBX::Instance>::create<RBX::Plugin>(void)")]
// was: boost::shared_ptr<RBX::Plugin> RBX::Creatable<RBX::Instance>::create<RBX::Plugin>(void)
pub fn stub_0xf582c4() -> ! {
    todo!("0xf582c4 rbx_core::SharedPtr<RBX::Plugin> RBX::Creatable<RBX::Instance>::create<RBX::Plugin>(void)")
}

// 0xf582d4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7ToolbarEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Toolbar> RBX::Creatable<RBX::Instance>::create<RBX::Toolbar>(void)")]
// was: boost::shared_ptr<RBX::Toolbar> RBX::Creatable<RBX::Instance>::create<RBX::Toolbar>(void)
pub fn stub_0xf582d4() -> ! {
    todo!("0xf582d4 rbx_core::SharedPtr<RBX::Toolbar> RBX::Creatable<RBX::Instance>::create<RBX::Toolbar>(void)")
}

// 0xf582e4 — j___ZN5boost10shared_ptrIN3RBX11PluginMouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PluginMouse>::shared_ptr<RBX::PluginMouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::PluginMouse>::shared_ptr<RBX::PluginMouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf582e4() -> ! {
    todo!("0xf582e4 rbx_core::SharedPtr<RBX::PluginMouse>::shared_ptr<RBX::PluginMouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginMouse *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf582f4 — j___ZN5boost10shared_ptrIN3RBX13PluginManagerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PluginManager>::shared_ptr<RBX::PluginManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::PluginManager>::shared_ptr<RBX::PluginManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf582f4() -> ! {
    todo!("0xf582f4 rbx_core::SharedPtr<RBX::PluginManager>::shared_ptr<RBX::PluginManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf58304 — j___ZN5boost10shared_ptrIN3RBX6ButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Button>::shared_ptr<RBX::Button,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Button>::shared_ptr<RBX::Button,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf58304() -> ! {
    todo!("0xf58304 rbx_core::SharedPtr<RBX::Button>::shared_ptr<RBX::Button,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Button *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf58324 — j___ZN5boost10shared_ptrIN3RBX6PluginEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Plugin>::shared_ptr<RBX::Plugin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Plugin>::shared_ptr<RBX::Plugin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf58324() -> ! {
    todo!("0xf58324 rbx_core::SharedPtr<RBX::Plugin>::shared_ptr<RBX::Plugin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Plugin *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf58334 — j___ZN5boost10shared_ptrIN3RBX7ToolbarEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Toolbar>::shared_ptr<RBX::Toolbar,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Toolbar>::shared_ptr<RBX::Toolbar,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_0xf58334() -> ! {
    todo!("0xf58334 rbx_core::SharedPtr<RBX::Toolbar>::shared_ptr<RBX::Toolbar,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Toolbar *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf58354 — j___ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11PluginMouseEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PluginMouse>(rbx_core::SharedPtr<RBX::PluginMouse> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PluginMouse>(boost::shared_ptr<RBX::PluginMouse> const&)
pub fn stub_0xf58354() -> ! {
    todo!("0xf58354 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PluginMouse>(rbx_core::SharedPtr<RBX::PluginMouse> const&)")
}
