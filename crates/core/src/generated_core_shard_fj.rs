//! core shard FJ — 100 core stubs EA-sorted, 0xf2f764..0xf30b04 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FI 0xf2f764 gap).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf2f764.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::implementation::typed_holder<double>::singleton(void)")]
// 0xf2f764 — j___ZN3rbx14implementation12typed_holderIdE9singletonEv
pub fn stub_f2f764() -> ! {
    todo!("0xf2f764 j___ZN3rbx14implementation12typed_holderIdE9singletonEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::RunTransition)>::operator()(RBX::RunTransition)")]
// 0xf2f774 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_
pub fn stub_f2f774() -> ! {
    todo!("0xf2f774 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Stepped const&)>::operator()(RBX::Stepped const&)")]
// 0xf2f784 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_
pub fn stub_f2f784() -> ! {
    todo!("0xf2f784 j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Heartbeat const&)>::operator()(RBX::Heartbeat const&)")]
// 0xf2f794 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_
pub fn stub_f2f794() -> ! {
    todo!("0xf2f794 j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(double)>::operator()(double)")]
// 0xf2f7a4 — j___ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd
pub fn stub_f2f7a4() -> ! {
    todo!("0xf2f7a4 j___ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(double,double)>::operator()(double,double)")]
// 0xf2f7c4 — j___ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd
pub fn stub_f2f7c4() -> ! {
    todo!("0xf2f7c4 j___ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")]
// 0xf2f7d4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv
pub fn stub_f2f7d4() -> ! {
    todo!("0xf2f7d4 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> &)")]
// 0xf2f7e4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::RunTransition)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> &)
pub fn stub_f2f7e4() -> ! {
    todo!("0xf2f7e4 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::on_error(std::exception &)")]
// 0xf2f7f4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception
pub fn stub_f2f7f4() -> ! {
    todo!("0xf2f7f4 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::disconnectAll(void)")]
// 0xf2f834 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv
pub fn stub_f2f834() -> ! {
    todo!("0xf2f834 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")]
// 0xf2f844 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv
pub fn stub_f2f844() -> ! {
    todo!("0xf2f844 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)")]
// 0xf2f854 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Stepped const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)
pub fn stub_f2f854() -> ! {
    todo!("0xf2f854 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")]
// 0xf2f864 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception
pub fn stub_f2f864() -> ! {
    todo!("0xf2f864 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::disconnectAll(void)")]
// 0xf2f874 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv
pub fn stub_f2f874() -> ! {
    todo!("0xf2f874 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)")]
// 0xf2f884 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)
pub fn stub_f2f884() -> ! {
    todo!("0xf2f884 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")]
// 0xf2f894 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception
pub fn stub_f2f894() -> ! {
    todo!("0xf2f894 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::disconnectAll(void)")]
// 0xf2f8a4 — j___ZN3rbx7signals6signalIFvdEE13disconnectAllEv
pub fn stub_f2f8a4() -> ! {
    todo!("0xf2f8a4 j___ZN3rbx7signals6signalIFvdEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")]
// 0xf2f8b4 — j___ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv
pub fn stub_f2f8b4() -> ! {
    todo!("0xf2f8b4 j___ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot> &)")]
// 0xf2f8c4 — j___ZN3rbx7signals6signalIFvdEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> &)
pub fn stub_f2f8c4() -> ! {
    todo!("0xf2f8c4 j___ZN3rbx7signals6signalIFvdEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")]
// 0xf2f8d4 — j___ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv
pub fn stub_f2f8d4() -> ! {
    todo!("0xf2f8d4 j___ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")]
// 0xf2f8e4 — j___ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE
pub fn stub_f2f8e4() -> ! {
    todo!("0xf2f8e4 j___ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")]
// 0xf2f8f4 — j___ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE
pub fn stub_f2f8f4() -> ! {
    todo!("0xf2f8f4 j___ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double)>::connect<boost::function<void ()(double)>>(boost::function<void ()(double)> const&)")]
// 0xf2f904 — j___ZN3rbx7signals6signalIFvdEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f2f904() -> ! {
    todo!("0xf2f904 j___ZN3rbx7signals6signalIFvdEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::on_error(std::exception &)")]
// 0xf2f914 — j___ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception
pub fn stub_f2f914() -> ! {
    todo!("0xf2f914 j___ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::disconnectAll(void)")]
// 0xf2f924 — j___ZN3rbx7signals6signalIFvddEE13disconnectAllEv
pub fn stub_f2f924() -> ! {
    todo!("0xf2f924 j___ZN3rbx7signals6signalIFvddEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")]
// 0xf2f934 — j___ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv
pub fn stub_f2f934() -> ! {
    todo!("0xf2f934 j___ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot> &)")]
// 0xf2f944 — j___ZN3rbx7signals6signalIFvddEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(double,double)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> &)
pub fn stub_f2f944() -> ! {
    todo!("0xf2f944 j___ZN3rbx7signals6signalIFvddEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_do_get_mutex(void)")]
// 0xf2f954 — j___ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv
pub fn stub_f2f954() -> ! {
    todo!("0xf2f954 j___ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::insert(rbx::signals::signal<void ()(double,double)>::slot *)")]
// 0xf2f964 — j___ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE
pub fn stub_f2f964() -> ! {
    todo!("0xf2f964 j___ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::remove(rbx::signals::signal<void ()(double,double)>::slot *)")]
// 0xf2f974 — j___ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE
pub fn stub_f2f974() -> ! {
    todo!("0xf2f974 j___ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double,double)>::connect<boost::function<void ()(double,double)>>(boost::function<void ()(double,double)> const&)")]
// 0xf2f984 — j___ZN3rbx7signals6signalIFvddEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f2f984() -> ! {
    todo!("0xf2f984 j___ZN3rbx7signals6signalIFvddEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")]
// 0xf2f994 — j___ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception
pub fn stub_f2f994() -> ! {
    todo!("0xf2f994 j___ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::callable<rbx::signals::signal<void ()(double)>*>(boost::function<void ()(double)> const&,rbx::signals::signal<void ()(double)>*)")]
// 0xf2f9a4 — j___ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
pub fn stub_f2f9a4() -> ! {
    todo!("0xf2f9a4 j___ZN3rbx8callableINS_7signals6signalIFvdEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::callable<rbx::signals::signal<void ()(double,double)>*>(boost::function<void ()(double,double)> const&,rbx::signals::signal<void ()(double,double)>*)")]
// 0xf2f9b4 — j___ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_f2f9b4() -> ! {
    todo!("0xf2f9b4 j___ZN3rbx8callableINS_7signals6signalIFvddEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
// 0xf2f9c4 — j___ZN5boost10shared_ptrIN3RBX10PhysicsJobEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)
pub fn stub_f2f9c4() -> ! {
    todo!("0xf2f9c4 j___ZN5boost10shared_ptrIN3RBX10PhysicsJobEEC2IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsJob>::operator=(rbx_core::SharedPtr<RBX::PhysicsJob> const&)")]
// 0xf2f9d4 — j___ZN5boost10shared_ptrIN3RBX10PhysicsJobEEaSERKS3_
// was: boost::shared_ptr<RBX::PhysicsJob>::operator=(boost::shared_ptr<RBX::PhysicsJob> const&)
pub fn stub_f2f9d4() -> ! {
    todo!("0xf2f9d4 j___ZN5boost10shared_ptrIN3RBX10PhysicsJobEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService>(rbx_core::WeakPtr<RBX::RunService> const&,boost::detail::sp_nothrow_tag)")]
// 0xf2f9e4 — j___ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService>(boost::weak_ptr<RBX::RunService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f2f9e4() -> ! {
    todo!("0xf2f9e4 j___ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
// 0xf2f9f4 — j___ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)
pub fn stub_f2f9f4() -> ! {
    todo!("0xf2f9f4 j___ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEC2IS2_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HeartbeatTask>::operator=(rbx_core::SharedPtr<RBX::HeartbeatTask> const&)")]
// 0xf2fa04 — j___ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEaSERKS3_
// was: boost::shared_ptr<RBX::HeartbeatTask>::operator=(boost::shared_ptr<RBX::HeartbeatTask> const&)
pub fn stub_f2fa04() -> ! {
    todo!("0xf2fa04 j___ZN5boost10shared_ptrIN3RBX13HeartbeatTaskEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> const&)")]
// 0xf2fa24 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSERKS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> const&)
pub fn stub_f2fa24() -> ! {
    todo!("0xf2fa24 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)")]
// 0xf2fa44 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSERKSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)
pub fn stub_f2fa44() -> ! {
    todo!("0xf2fa44 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)")]
// 0xf2fa54 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)
pub fn stub_f2fa54() -> ! {
    todo!("0xf2fa54 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot> const&)")]
// 0xf2fa64 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double)>::slot> const&)
pub fn stub_f2fa64() -> ! {
    todo!("0xf2fa64 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvdEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx::signals::signal<void ()(double,double)>::slot*)")]
// 0xf2fa74 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx::signals::signal<void ()(double,double)>::slot*)
pub fn stub_f2fa74() -> ! {
    todo!("0xf2fa74 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot> const&)")]
// 0xf2fa84 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(double,double)>::slot> const&)
pub fn stub_f2fa84() -> ! {
    todo!("0xf2fa84 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvddEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::throw_exception<boost::bad_weak_ptr>(boost::bad_weak_ptr const&)")]
// 0xf2fa94 — j___ZN5boost15throw_exceptionINS_12bad_weak_ptrEEEvRKT_
pub fn stub_f2fa94() -> ! {
    todo!("0xf2fa94 j___ZN5boost15throw_exceptionINS_12bad_weak_ptrEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast> const&)")]
// 0xf2faa4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS5_
pub fn stub_f2faa4() -> ! {
    todo!("0xf2faa4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS5_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_tag)")]
// 0xf2fab4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_f2fab4() -> ! {
    todo!("0xf2fab4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_tag)")]
// 0xf2fac4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_f2fac4() -> ! {
    todo!("0xf2fac4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_function_call> const&)")]
// 0xf2fad4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS4_
pub fn stub_f2fad4() -> ! {
    todo!("0xf2fad4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS4_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_tag)")]
// 0xf2fae4 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_f2fae4() -> ! {
    todo!("0xf2fae4 j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_17bad_function_callEEEEC1ERKS5_NS5_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
// 0xf2faf4 — j___ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev
pub fn stub_f2faf4() -> ! {
    todo!("0xf2faf4 j___ZN5boost16exception_detail19error_info_injectorIN3rbx22bad_placement_any_castEED2Ev")
}

#[doc(alias = "boost::bad_function_call::bad_function_call(void)")]
// 0xf2fb04 — j___ZN5boost17bad_function_callC2Ev
pub fn stub_f2fb04() -> ! {
    todo!("0xf2fb04 j___ZN5boost17bad_function_callC2Ev")
}

#[doc(alias = "void rbx_core::SharedPtr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0xf2fb24 — j___ZN5boost21intrusive_ptr_add_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_f2fb24() -> ! {
    todo!("0xf2fb24 j___ZN5boost21intrusive_ptr_add_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE")
}

#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::shared_count const&)")]
// 0xf2fb94 — j___ZN5boost6detail10weak_countC1ERKNS0_12shared_countE
pub fn stub_f2fb94() -> ! {
    todo!("0xf2fb94 j___ZN5boost6detail10weak_countC1ERKNS0_12shared_countE")
}

#[doc(alias = "boost::detail::weak_count::operator=(boost::detail::shared_count const&)")]
// 0xf2fba4 — j___ZN5boost6detail10weak_countaSERKNS0_12shared_countE
pub fn stub_f2fba4() -> ! {
    todo!("0xf2fba4 j___ZN5boost6detail10weak_countaSERKNS0_12shared_countE")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
// 0xf2fbb4 — j___ZN5boost6detail12shared_countC2IN3RBX10PhysicsJobEEEPT_
pub fn stub_f2fbb4() -> ! {
    todo!("0xf2fbb4 j___ZN5boost6detail12shared_countC2IN3RBX10PhysicsJobEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
// 0xf2fbc4 — j___ZN5boost6detail12shared_countC2IN3RBX13HeartbeatTaskEEEPT_
pub fn stub_f2fbc4() -> ! {
    todo!("0xf2fbc4 j___ZN5boost6detail12shared_countC2IN3RBX13HeartbeatTaskEEEPT_")
}

#[doc(alias = "boost::function1<void,double>::assign_to_own(boost::function1<void,double> const&)")]
// 0xf2fc24 — j___ZN5boost9function1IvdE13assign_to_ownERKS1_
pub fn stub_f2fc24() -> ! {
    todo!("0xf2fc24 j___ZN5boost9function1IvdE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function1<void,double>::clear(void)")]
// 0xf2fc34 — j___ZN5boost9function1IvdE5clearEv
pub fn stub_f2fc34() -> ! {
    todo!("0xf2fc34 j___ZN5boost9function1IvdE5clearEv")
}

#[doc(alias = "boost::function2<void,double,double>::assign_to_own(boost::function2<void,double,double> const&)")]
// 0xf2fc64 — j___ZN5boost9function2IvddE13assign_to_ownERKS1_
pub fn stub_f2fc64() -> ! {
    todo!("0xf2fc64 j___ZN5boost9function2IvddE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function2<void,double,double>::clear(void)")]
// 0xf2fc74 — j___ZN5boost9function2IvddE5clearEv
pub fn stub_f2fc74() -> ! {
    todo!("0xf2fc74 j___ZN5boost9function2IvddE5clearEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone(void)const")]
// 0xf2fcb4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv
pub fn stub_f2fcb4() -> ! {
    todo!("0xf2fcb4 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorIN3rbx22bad_placement_any_castEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const")]
// 0xf2fcc4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE7rethrowEv
pub fn stub_f2fcc4() -> ! {
    todo!("0xf2fcc4 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE7rethrowEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(rbx_core::SharedPtr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const")]
// 0xf2fcd4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10PhysicsJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(boost::shared_ptr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const
pub fn stub_f2fcd4() -> ! {
    todo!("0xf2fcd4 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10PhysicsJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(rbx_core::SharedPtr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const")]
// 0xf2fce4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_13HeartbeatTaskES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(boost::shared_ptr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const
pub fn stub_f2fce4() -> ! {
    todo!("0xf2fce4 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_13HeartbeatTaskES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::function1<void,double>::operator()(double)const")]
// 0xf2fd54 — j___ZNK5boost9function1IvdEclEd
pub fn stub_f2fd54() -> ! {
    todo!("0xf2fd54 j___ZNK5boost9function1IvdEclEd")
}

#[doc(alias = "boost::function2<void,double,double>::operator()(double,double)const")]
// 0xf2fd64 — j___ZNK5boost9function2IvddEclEdd
pub fn stub_f2fd64() -> ! {
    todo!("0xf2fd64 j___ZNK5boost9function2IvddEclEdd")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
// 0xf2fe54 — j___ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
pub fn stub_f2fe54() -> ! {
    todo!("0xf2fe54 j___ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
// 0xf2fe64 — j___ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
pub fn stub_f2fe64() -> ! {
    todo!("0xf2fe64 j___ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
// 0xf2fe74 — j___ZN3RBX8LRUCacheISsSsE6removeERKSs
pub fn stub_f2fe74() -> ! {
    todo!("0xf2fe74 j___ZN3RBX8LRUCacheISsSsE6removeERKSs")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
// 0xf2fe84 — j___ZN3RBX8LRUCacheISsSsEC2Ev
pub fn stub_f2fe84() -> ! {
    todo!("0xf2fe84 j___ZN3RBX8LRUCacheISsSsEC2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
// 0xf2fe94 — j___ZN3RBX8LRUCacheISsSsED2Ev
pub fn stub_f2fe94() -> ! {
    todo!("0xf2fe94 j___ZN3RBX8LRUCacheISsSsED2Ev")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")]
// 0xf30054 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
pub fn stub_f30054() -> ! {
    todo!("0xf30054 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0xf30064 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_f30064() -> ! {
    todo!("0xf30064 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
// 0xf30074 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_
pub fn stub_f30074() -> ! {
    todo!("0xf30074 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf30084 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
pub fn stub_f30084() -> ! {
    todo!("0xf30084 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
// 0xf300c4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_
pub fn stub_f300c4() -> ! {
    todo!("0xf300c4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")]
// 0xf300d4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv
pub fn stub_f300d4() -> ! {
    todo!("0xf300d4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")]
// 0xf300e4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev
pub fn stub_f300e4() -> ! {
    todo!("0xf300e4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0xf30164 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_f30164() -> ! {
    todo!("0xf30164 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf30174 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_f30174() -> ! {
    todo!("0xf30174 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf30184 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_f30184() -> ! {
    todo!("0xf30184 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf30194 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_f30194() -> ! {
    todo!("0xf30194 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf301a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_f301a4() -> ! {
    todo!("0xf301a4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// 0xf301b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_f301b4() -> ! {
    todo!("0xf301b4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
// 0xf301c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
pub fn stub_f301c4() -> ! {
    todo!("0xf301c4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
// 0xf301d4 — j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
pub fn stub_f301d4() -> ! {
    todo!("0xf301d4 j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf30274 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
pub fn stub_f30274() -> ! {
    todo!("0xf30274 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0xf302a4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_f302a4() -> ! {
    todo!("0xf302a4 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf302b4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_f302b4() -> ! {
    todo!("0xf302b4 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
// 0xf302d4 — j___ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
pub fn stub_f302d4() -> ! {
    todo!("0xf302d4 j___ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
// 0xf302f4 — j___ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_f302f4() -> ! {
    todo!("0xf302f4 j___ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
// 0xf30304 — j___ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
pub fn stub_f30304() -> ! {
    todo!("0xf30304 j___ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
// 0xf30514 — j___ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
pub fn stub_f30514() -> ! {
    todo!("0xf30514 j___ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
// 0xf30ac4 — j___ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
pub fn stub_f30ac4() -> ! {
    todo!("0xf30ac4 j___ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
// 0xf30ad4 — j___ZN3RBX12SpanningNode8getDepthEPS0_
pub fn stub_f30ad4() -> ! {
    todo!("0xf30ad4 j___ZN3RBX12SpanningNode8getDepthEPS0_")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
// 0xf30ae4 — j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f30ae4() -> ! {
    todo!("0xf30ae4 j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
// 0xf30af4 — j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f30af4() -> ! {
    todo!("0xf30af4 j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
// 0xf30b04 — j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f30b04() -> ! {
    todo!("0xf30b04 j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

