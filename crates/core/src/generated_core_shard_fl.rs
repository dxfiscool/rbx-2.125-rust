//! core shard FL — 100 core stubs EA-sorted, 0xf31544..0xf324f4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FK 0xf31544 gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf31534.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
// 0xf31544 — j___ZN3rbx13remote_signalIFvfffEED2Ev
pub fn stub_f31544() -> ! {
    todo!("0xf31544 j___ZN3rbx13remote_signalIFvfffEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")]
// 0xf31554 — j___ZN3rbx13remote_signalIFvffffEEC2Ev
pub fn stub_f31554() -> ! {
    todo!("0xf31554 j___ZN3rbx13remote_signalIFvffffEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
// 0xf31564 — j___ZN3rbx13remote_signalIFvffffEED2Ev
pub fn stub_f31564() -> ! {
    todo!("0xf31564 j___ZN3rbx13remote_signalIFvffffEED2Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
// 0xf31574 — j___ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
pub fn stub_f31574() -> ! {
    todo!("0xf31574 j___ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff")
}

#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
// 0xf31584 — j___ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
pub fn stub_f31584() -> ! {
    todo!("0xf31584 j___ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0xf31594 — j___ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_f31594() -> ! {
    todo!("0xf31594 j___ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")]
// 0xf315a4 — j___ZN3rbx7signals6signalIFvfffEE13disconnectAllEv
pub fn stub_f315a4() -> ! {
    todo!("0xf315a4 j___ZN3rbx7signals6signalIFvfffEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
// 0xf315b4 — j___ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
pub fn stub_f315b4() -> ! {
    todo!("0xf315b4 j___ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")]
// 0xf315c4 — j___ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> &)
pub fn stub_f315c4() -> ! {
    todo!("0xf315c4 j___ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")]
// 0xf315d4 — j___ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv
pub fn stub_f315d4() -> ! {
    todo!("0xf315d4 j___ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// 0xf315e4 — j___ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
pub fn stub_f315e4() -> ! {
    todo!("0xf315e4 j___ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// 0xf315f4 — j___ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE
pub fn stub_f315f4() -> ! {
    todo!("0xf315f4 j___ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0xf31604 — j___ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_f31604() -> ! {
    todo!("0xf31604 j___ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")]
// 0xf31614 — j___ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f31614() -> ! {
    todo!("0xf31614 j___ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
// 0xf31624 — j___ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
pub fn stub_f31624() -> ! {
    todo!("0xf31624 j___ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")]
// 0xf31634 — j___ZN3rbx7signals6signalIFvffffEE13disconnectAllEv
pub fn stub_f31634() -> ! {
    todo!("0xf31634 j___ZN3rbx7signals6signalIFvffffEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
// 0xf31644 — j___ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
pub fn stub_f31644() -> ! {
    todo!("0xf31644 j___ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")]
// 0xf31654 — j___ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(float,float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)
pub fn stub_f31654() -> ! {
    todo!("0xf31654 j___ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")]
// 0xf31664 — j___ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv
pub fn stub_f31664() -> ! {
    todo!("0xf31664 j___ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
// 0xf31674 — j___ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE
pub fn stub_f31674() -> ! {
    todo!("0xf31674 j___ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
// 0xf31684 — j___ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE
pub fn stub_f31684() -> ! {
    todo!("0xf31684 j___ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")]
// 0xf31694 — j___ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_
pub fn stub_f31694() -> ! {
    todo!("0xf31694 j___ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")]
// 0xf316a4 — j___ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f316a4() -> ! {
    todo!("0xf316a4 j___ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
// 0xf316b4 — j___ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
pub fn stub_f316b4() -> ! {
    todo!("0xf316b4 j___ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")]
// 0xf316c4 — j___ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
pub fn stub_f316c4() -> ! {
    todo!("0xf316c4 j___ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")]
// 0xf316d4 — j___ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_
pub fn stub_f316d4() -> ! {
    todo!("0xf316d4 j___ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator const>::shared_ptr<RBX::Animator const>(rbx_core::WeakPtr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")]
// 0xf316e4 — j___ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::Animator const>::shared_ptr<RBX::Animator const>(boost::weak_ptr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f316e4() -> ! {
    todo!("0xf316e4 j___ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")]
// 0xf316f4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)
pub fn stub_f316f4() -> ! {
    todo!("0xf316f4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")]
// 0xf31704 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)
pub fn stub_f31704() -> ! {
    todo!("0xf31704 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")]
// 0xf31714 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)
pub fn stub_f31714() -> ! {
    todo!("0xf31714 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")]
// 0xf31724 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)
pub fn stub_f31724() -> ! {
    todo!("0xf31724 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")]
// 0xf31734 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f31734() -> ! {
    todo!("0xf31734 j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")]
// 0xf31754 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f31754() -> ! {
    todo!("0xf31754 j___ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
// 0xf31774 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f31774() -> ! {
    todo!("0xf31774 j___ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")]
// 0xf317e4 — j___ZN5boost9function3IvfffE13assign_to_ownERKS1_
pub fn stub_f317e4() -> ! {
    todo!("0xf317e4 j___ZN5boost9function3IvfffE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function3<void,float,float,float>::clear(void)")]
// 0xf317f4 — j___ZN5boost9function3IvfffE5clearEv
pub fn stub_f317f4() -> ! {
    todo!("0xf317f4 j___ZN5boost9function3IvfffE5clearEv")
}

#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
// 0xf31824 — j___ZN5boost9function4IvffffE13assign_to_ownERKS1_
pub fn stub_f31824() -> ! {
    todo!("0xf31824 j___ZN5boost9function4IvffffE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function4<void,float,float,float,float>::clear(void)")]
// 0xf31834 — j___ZN5boost9function4IvffffE5clearEv
pub fn stub_f31834() -> ! {
    todo!("0xf31834 j___ZN5boost9function4IvffffE5clearEv")
}

#[doc(alias = "boost::function3<void,float,float,float>::operator()(float,float,float)const")]
// 0xf318d4 — j___ZNK5boost9function3IvfffEclEfff
pub fn stub_f318d4() -> ! {
    todo!("0xf318d4 j___ZNK5boost9function3IvfffEclEfff")
}

#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
// 0xf318e4 — j___ZNK5boost9function4IvffffEclEffff
pub fn stub_f318e4() -> ! {
    todo!("0xf318e4 j___ZNK5boost9function4IvffffEclEffff")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
// 0xf31924 — j___ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)
pub fn stub_f31924() -> ! {
    todo!("0xf31924 j___ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
// 0xf31964 — j___ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
pub fn stub_f31964() -> ! {
    todo!("0xf31964 j___ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&> &,int)")]
// 0xf319b4 — j___ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&> &,int)
pub fn stub_f319b4() -> ! {
    todo!("0xf319b4 j___ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")]
// 0xf31a44 — j___ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
// was: boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const
pub fn stub_f31a44() -> ! {
    todo!("0xf31a44 j___ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_")
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_clear(void)")]
// 0xf31a54 — j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_clear(void)
pub fn stub_f31a54() -> ! {
    todo!("0xf31a54 j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv")
}

#[doc(alias = "std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")]
// 0xf31a64 — j___ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
pub fn stub_f31a64() -> ! {
    todo!("0xf31a64 j___ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")]
// 0xf31a74 — j___ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
pub fn stub_f31a74() -> ! {
    todo!("0xf31a74 j___ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
// 0xf31a84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
pub fn stub_f31a84() -> ! {
    todo!("0xf31a84 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_create_node(rbx_core::SharedPtr<RBX::AnimationTrackState> const&)")]
// 0xf31a94 — j___ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_create_node(boost::shared_ptr<RBX::AnimationTrackState> const&)
pub fn stub_f31a94() -> ! {
    todo!("0xf31a94 j___ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>)")]
// 0xf31aa4 — j___ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// was: std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>)
pub fn stub_f31aa4() -> ! {
    todo!("0xf31aa4 j___ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")
}

#[doc(alias = "RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")]
// 0xf31ab4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
pub fn stub_f31ab4() -> ! {
    todo!("0xf31ab4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_")
}

#[doc(alias = "RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
// 0xf31ac4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
pub fn stub_f31ac4() -> ! {
    todo!("0xf31ac4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")]
// 0xf31ad4 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f31ad4() -> ! {
    todo!("0xf31ad4 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")]
// 0xf31ae4 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
pub fn stub_f31ae4() -> ! {
    todo!("0xf31ae4 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")]
// 0xf31af4 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
pub fn stub_f31af4() -> ! {
    todo!("0xf31af4 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")]
// 0xf31b04 — j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
pub fn stub_f31b04() -> ! {
    todo!("0xf31b04 j___ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_")
}

#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")]
// 0xf31b14 — j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f31b14() -> ! {
    todo!("0xf31b14 j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")]
// 0xf31b24 — j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
pub fn stub_f31b24() -> ! {
    todo!("0xf31b24 j___ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")]
// 0xf31b34 — j___ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
// was: boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)
pub fn stub_f31b34() -> ! {
    todo!("0xf31b34 j___ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_")
}

#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
// 0xf31b44 — j___ZN3RBX10ArcHandlesD2Ev
pub fn stub_f31b44() -> ! {
    todo!("0xf31b44 j___ZN3RBX10ArcHandlesD2Ev")
}

#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
// 0xf31be4 — j___ZN3RBX11HandlesBaseD2Ev
pub fn stub_f31be4() -> ! {
    todo!("0xf31be4 j___ZN3RBX11HandlesBaseD2Ev")
}

#[doc(alias = "RBX::PartAdornment::~PartAdornment()")]
// 0xf31bf4 — j___ZN3RBX13PartAdornmentD1Ev
pub fn stub_f31bf4() -> ! {
    todo!("0xf31bf4 j___ZN3RBX13PartAdornmentD1Ev")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// 0xf31ee4 — j___ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
// was: void boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)
pub fn stub_f31ee4() -> ! {
    todo!("0xf31ee4 j___ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// 0xf31ef4 — j___ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)
pub fn stub_f31ef4() -> ! {
    todo!("0xf31ef4 j___ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// 0xf31fc4 — j___ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
pub fn stub_f31fc4() -> ! {
    todo!("0xf31fc4 j___ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_")
}

#[doc(alias = "RBX::BadgeService::~BadgeService()")]
// 0xf32284 — j___ZN3RBX12BadgeServiceD0Ev
pub fn stub_f32284() -> ! {
    todo!("0xf32284 j___ZN3RBX12BadgeServiceD0Ev")
}

#[doc(alias = "RBX::BadgeService::~BadgeService()")]
// 0xf32294 — j___ZN3RBX12BadgeServiceD2Ev
pub fn stub_f32294() -> ! {
    todo!("0xf32294 j___ZN3RBX12BadgeServiceD2Ev")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::BadgeService> RBX::weak_from<RBX::BadgeService>(RBX::BadgeService*)")]
// 0xf322a4 — j___ZN3RBX9weak_fromINS_12BadgeServiceEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::BadgeService> RBX::weak_from<RBX::BadgeService>(RBX::BadgeService*)
pub fn stub_f322a4() -> ! {
    todo!("0xf322a4 j___ZN3RBX9weak_fromINS_12BadgeServiceEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::remote_signal(void)")]
// 0xf322b4 — j___ZN3rbx13remote_signalIFvSsEEC2Ev
pub fn stub_f322b4() -> ! {
    todo!("0xf322b4 j___ZN3rbx13remote_signalIFvSsEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
// 0xf322c4 — j___ZN3rbx13remote_signalIFvSsEED2Ev
pub fn stub_f322c4() -> ! {
    todo!("0xf322c4 j___ZN3rbx13remote_signalIFvSsEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(rbx_core::WeakPtr<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)")]
// 0xf322d4 — j___ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(boost::weak_ptr<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f322d4() -> ! {
    todo!("0xf322d4 j___ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf322e4 — j___ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// was: boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list6(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_f322e4() -> ! {
    todo!("0xf322e4 j___ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_")
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf322f4 — j___ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
pub fn stub_f322f4() -> ! {
    todo!("0xf322f4 j___ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf32304 — j___ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// was: boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_f32304() -> ! {
    todo!("0xf32304 j___ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_")
}

#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf32314 — j___ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
pub fn stub_f32314() -> ! {
    todo!("0xf32314 j___ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>)")]
// 0xf32324 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>)
pub fn stub_f32324() -> ! {
    todo!("0xf32324 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEEEC2ES7_S8_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>)")]
// 0xf32334 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>)
pub fn stub_f32334() -> ! {
    todo!("0xf32334 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)")]
// 0xf32344 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)
pub fn stub_f32344() -> ! {
    todo!("0xf32344 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// 0xf32354 — j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)
pub fn stub_f32354() -> ! {
    todo!("0xf32354 j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)")]
// 0xf32364 — j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)
pub fn stub_f32364() -> ! {
    todo!("0xf32364 j___ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// 0xf32374 — j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_SA_SB_SF_
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)
pub fn stub_f32374() -> ! {
    todo!("0xf32374 j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_SA_SB_SF_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// 0xf32384 — j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)
pub fn stub_f32384() -> ! {
    todo!("0xf32384 j___ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// 0xf32394 — j___ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// was: boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)
pub fn stub_f32394() -> ! {
    todo!("0xf32394 j___ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf323a4 — j___ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// was: boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_f323a4() -> ! {
    todo!("0xf323a4 j___ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// 0xf323b4 — j___ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_
// was: boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)
pub fn stub_f323b4() -> ! {
    todo!("0xf323b4 j___ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_")
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// 0xf323c4 — j___ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// was: boost::_bi::storage7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::storage7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)
pub fn stub_f323c4() -> ! {
    todo!("0xf323c4 j___ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_")
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0xf323d4 — j___ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// was: boost::_bi::storage7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_f323d4() -> ! {
    todo!("0xf323d4 j___ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf323e4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENS8_IFvSsEEES4_iNS_3argILi1EEENSD_ILi2EEESA_SC_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_T5_ENSG_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESQ_SS_ST_SU_SV_SW_SX_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_6<boost::weak_ptr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,boost::weak_ptr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::weak_ptr<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_f323e4() -> ! {
    todo!("0xf323e4 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENS8_IFvSsEEES4_iNS_3argILi1EEENSD_ILi2EEESA_SC_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_T5_ENSG_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESQ_SS_ST_SU_SV_SW_SX_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_7<rbx_core::WeakPtr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::WeakPtr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::WeakPtr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf323f4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENS8_IFvSsEEES4_iiNS_3argILi1EEENSD_ILi2EEESA_SC_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_T5_T6_ENSG_9list_av_7IT7_T8_T9_T10_T11_T12_T13_E4typeEEESR_ST_SU_SV_SW_SX_SY_SZ_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_7<boost::weak_ptr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,boost::weak_ptr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::weak_ptr<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
pub fn stub_f323f4() -> ! {
    todo!("0xf323f4 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENS8_IFvSsEEES4_iiNS_3argILi1EEENSD_ILi2EEESA_SC_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_T5_T6_ENSG_9list_av_7IT7_T8_T9_T10_T11_T12_T13_E4typeEEESR_ST_SU_SV_SW_SX_SY_SZ_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf32404 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f32404() -> ! {
    todo!("0xf32404 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf32414 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f32414() -> ! {
    todo!("0xf32414 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)")]
// 0xf32444 — j___ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)
pub fn stub_f32444() -> ! {
    todo!("0xf32444 j___ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0xf32454 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)
pub fn stub_f32454() -> ! {
    todo!("0xf32454 j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0xf32464 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)
pub fn stub_f32464() -> ! {
    todo!("0xf32464 j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf324a4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f324a4() -> ! {
    todo!("0xf324a4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf324b4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f324b4() -> ! {
    todo!("0xf324b4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf324c4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f324c4() -> ! {
    todo!("0xf324c4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf324d4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f324d4() -> ! {
    todo!("0xf324d4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf324e4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f324e4() -> ! {
    todo!("0xf324e4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf324f4 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f324f4() -> ! {
    todo!("0xf324f4 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}
