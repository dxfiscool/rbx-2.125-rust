// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x392380..0x3deaac | total filtered 10215, remaining 2668 after batch (global dedup), 4166 dm remaining
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0x392380 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_392380() -> ! {
    todo!("0x392380 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x392398 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_392398() -> ! {
    todo!("0x392398 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x392804 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_392804() -> ! {
    todo!("0x392804 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x392830 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_392830() -> ! {
    todo!("0x392830 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x392904 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_392904() -> ! {
    todo!("0x392904 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x392920 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_392920() -> ! {
    todo!("0x392920 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x39293c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX12AccoutrementEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_39293c() -> ! {
    todo!("0x39293c void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0x392a14 — __ZNK5boost4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Accoutrement*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Accoutrement*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_392a14() -> ! {
    todo!("0x392a14 boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Accoutrement*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x392afc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_392afc() -> ! {
    todo!("0x392afc rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x392b28 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_392b28() -> ! {
    todo!("0x392b28 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x392bfc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_392bfc() -> ! {
    todo!("0x392bfc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x392c5c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_392c5c() -> ! {
    todo!("0x392c5c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x392c78 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_12AccoutrementEEEPKT_v
#[doc(alias = "RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")]
pub fn stub_392c78() -> ! {
    todo!("0x392c78 RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")
}

// 0x396c40 — __ZNK3RBX9Animation19getKeyframeSequenceEPKNS_8InstanceE
#[doc(alias = "RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")]
pub fn stub_396c40() -> ! {
    todo!("0x396c40 RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")
}

// 0x396e44 — __ZN3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")]
pub fn stub_396e44() -> ! {
    todo!("0x396e44 RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")
}

// 0x3970bc — __ZNK3RBX9Animation12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Animation::askSetParent(RBX::Instance const*)const")]
pub fn stub_3970bc() -> ! {
    todo!("0x3970bc RBX::Animation::askSetParent(RBX::Instance const*)const")
}

// 0x39bdb4 — __ZN3RBX19AnimationTrackState28triggerKeyframeReachedSignalERKN5boost10shared_ptrINS_8InstanceEEEdd
#[doc(alias = "RBX::AnimationTrackState::triggerKeyframeReachedSignal(rbx_core::SharedPtr<RBX::Instance> const&,double,double)")]
// was: RBX::AnimationTrackState::triggerKeyframeReachedSignal(boost::shared_ptr<RBX::Instance> const&,double,double)
pub fn stub_39bdb4() -> ! {
    todo!("0x39bdb4 RBX::AnimationTrackState::triggerKeyframeReachedSignal(rbx_core::SharedPtr<RBX::Instance> const&,double,double)")
}

// 0x3a395c — __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Animator::loadAnimation(boost::shared_ptr<RBX::Instance>)
pub fn stub_3a395c() -> ! {
    todo!("0x3a395c RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3a3d44 — __ZN3RBX8AnimatorC1EPNS_8InstanceE
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
pub fn stub_3a3d44() -> ! {
    todo!("0x3a3d44 RBX::Animator::Animator(RBX::Instance *)")
}

// 0x3a3d48 — __ZN3RBX8AnimatorC2EPNS_8InstanceE
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
pub fn stub_3a3d48() -> ! {
    todo!("0x3a3d48 RBX::Animator::Animator(RBX::Instance *)")
}

// 0x3a46a0 — __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Animator::appendAnimatableJointsRec(boost::shared_ptr<RBX::Instance>)
pub fn stub_3a46a0() -> ! {
    todo!("0x3a46a0 RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3a4ea0 — __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
pub fn stub_3a4ea0() -> ! {
    todo!("0x3a4ea0 RBX::Animator::askAddChild(RBX::Instance const*)const")
}

// 0x3a5218 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")]
// was: boost::shared_ptr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>)
pub fn stub_3a5218() -> ! {
    todo!("0x3a5218 rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")
}

// 0x3a5380 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")]
// was: boost::shared_ptr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>)
pub fn stub_3a5380() -> ! {
    todo!("0x3a5380 rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")
}

// 0x3a55fc — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const
pub fn stub_3a55fc() -> ! {
    todo!("0x3a55fc void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")
}

// 0x3a5880 — __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
pub fn stub_3a5880() -> ! {
    todo!("0x3a5880 RBX::Animator::askSetParent(RBX::Instance const*)const")
}

// 0x3a5cd4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_3a5cd4() -> ! {
    todo!("0x3a5cd4 void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x3a5dac — __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Animator*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_3a5dac() -> ! {
    todo!("0x3a5dac boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x3a61fc — __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3a61fc() -> ! {
    todo!("0x3a61fc rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a63ac — __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3a63ac() -> ! {
    todo!("0x3a63ac boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a64b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3a64b4() -> ! {
    todo!("0x3a64b4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3a64b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3a64b8() -> ! {
    todo!("0x3a64b8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3a64bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3a64bc() -> ! {
    todo!("0x3a64bc boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3a64dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3a64dc() -> ! {
    todo!("0x3a64dc boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3a64f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3a64f4() -> ! {
    todo!("0x3a64f4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3a64f8 — __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3a64f8() -> ! {
    todo!("0x3a64f8 rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a66a8 — __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3a66a8() -> ! {
    todo!("0x3a66a8 boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a67b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3a67b0() -> ! {
    todo!("0x3a67b0 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3a67b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3a67b4() -> ! {
    todo!("0x3a67b4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3a67b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3a67b8() -> ! {
    todo!("0x3a67b8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3a67d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3a67d8() -> ! {
    todo!("0x3a67d8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3a67f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3a67f0() -> ! {
    todo!("0x3a67f0 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3a8a58 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
// was: boost::shared_ptr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)
pub fn stub_3a8a58() -> ! {
    todo!("0x3a8a58 rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")
}

// 0x3a8b0c — __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3a8b0c() -> ! {
    todo!("0x3a8b0c rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a8cbc — __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3a8cbc() -> ! {
    todo!("0x3a8cbc boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a8dc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3a8dc4() -> ! {
    todo!("0x3a8dc4 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3a8dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3a8dc8() -> ! {
    todo!("0x3a8dc8 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3a8dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3a8dcc() -> ! {
    todo!("0x3a8dcc boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3a8dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3a8dec() -> ! {
    todo!("0x3a8dec boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3a8e04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3a8e04() -> ! {
    todo!("0x3a8e04 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3b16ac — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8BackpackEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Backpack> RBX::Creatable<RBX::Instance>::create<RBX::Backpack>(void)")]
// was: boost::shared_ptr<RBX::Backpack> RBX::Creatable<RBX::Instance>::create<RBX::Backpack>(void)
pub fn stub_3b16ac() -> ! {
    todo!("0x3b16ac rbx_core::SharedPtr<RBX::Backpack> RBX::Creatable<RBX::Instance>::create<RBX::Backpack>(void)")
}

// 0x3b175c — __ZN5boost10shared_ptrIN3RBX8BackpackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Backpack>::shared_ptr<RBX::Backpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Backpack>::shared_ptr<RBX::Backpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3b175c() -> ! {
    todo!("0x3b175c rbx_core::SharedPtr<RBX::Backpack>::shared_ptr<RBX::Backpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3b1824 — __ZN5boost6detail12shared_countC2IPN3RBX8BackpackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3b1824() -> ! {
    todo!("0x3b1824 boost::detail::shared_count::shared_count<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3bffc0 — __ZN3RBX12BillboardGui19setPlayerToHideFromEPNS_8InstanceE
#[doc(alias = "RBX::BillboardGui::setPlayerToHideFrom(RBX::Instance *)")]
pub fn stub_3bffc0() -> ! {
    todo!("0x3bffc0 RBX::BillboardGui::setPlayerToHideFrom(RBX::Instance *)")
}

// 0x3c0434 — __ZNK3RBX12BillboardGui12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::BillboardGui::askSetParent(RBX::Instance const*)const")]
pub fn stub_3c0434() -> ! {
    todo!("0x3c0434 RBX::BillboardGui::askSetParent(RBX::Instance const*)const")
}

// 0x3c39b4 — __ZN3RBX6Camera16setCameraSubjectEPNS_8InstanceE
#[doc(alias = "RBX::Camera::setCameraSubject(RBX::Instance *)")]
pub fn stub_3c39b4() -> ! {
    todo!("0x3c39b4 RBX::Camera::setCameraSubject(RBX::Instance *)")
}

// 0x3c4e90 — __ZNK3RBX6Camera12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Camera::askSetParent(RBX::Instance const*)const")]
pub fn stub_3c4e90() -> ! {
    todo!("0x3c4e90 RBX::Camera::askSetParent(RBX::Instance const*)const")
}

// 0x3c9c64 — __ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(RBX::Instance const*)")]
pub fn stub_3c9c64() -> ! {
    todo!("0x3c9c64 RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(RBX::Instance const*)")
}

// 0x3d43a4 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKcPKNS_8InstanceE
#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*,RBX::Instance const*)")]
pub fn stub_3d43a4() -> ! {
    todo!("0x3d43a4 RBX::ChangeHistoryService::requestWaypoint(char const*,RBX::Instance const*)")
}

// 0x3d4700 — __ZN3RBX20ChangeHistoryService26reportMissedPhysicsChangesEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ChangeHistoryService::reportMissedPhysicsChanges(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ChangeHistoryService::reportMissedPhysicsChanges(boost::shared_ptr<RBX::Instance>)
pub fn stub_3d4700() -> ! {
    todo!("0x3d4700 RBX::ChangeHistoryService::reportMissedPhysicsChanges(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3d5444 — __ZN3RBX20ChangeHistoryService11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ChangeHistoryService::onItemAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ChangeHistoryService::onItemAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_3d5444() -> ! {
    todo!("0x3d5444 RBX::ChangeHistoryService::onItemAdded(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3d576c — __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
#[doc(alias = "RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")]
pub fn stub_3d576c() -> ! {
    todo!("0x3d576c RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")
}

// 0x3d59c0 — __ZN3RBX20ChangeHistoryService13onItemRemovedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ChangeHistoryService::onItemRemoved(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ChangeHistoryService::onItemRemoved(boost::shared_ptr<RBX::Instance>)
pub fn stub_3d59c0() -> ! {
    todo!("0x3d59c0 RBX::ChangeHistoryService::onItemRemoved(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3d6ba0 — __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")]
pub fn stub_3d6ba0() -> ! {
    todo!("0x3d6ba0 RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")
}

// 0x3d6fc4 — __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")]
pub fn stub_3d6fc4() -> ! {
    todo!("0x3d6fc4 RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")
}

// 0x3d6fe0 — __ZN3RBX11shared_fromINS_9WorkspaceEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)")]
// was: boost::shared_ptr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)
pub fn stub_3d6fe0() -> ! {
    todo!("0x3d6fe0 rbx_core::SharedPtr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)")
}

// 0x3d72f0 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_20ChangeHistoryServiceENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)const
pub fn stub_3d72f0() -> ! {
    todo!("0x3d72f0 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)const")
}

// 0x3d73f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)
pub fn stub_3d73f8() -> ! {
    todo!("0x3d73f8 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")
}

// 0x3d7558 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI22ChangeHistoryStatsItemEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<ChangeHistoryStatsItem>(rbx_core::SharedPtr<ChangeHistoryStatsItem> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<ChangeHistoryStatsItem>(boost::shared_ptr<ChangeHistoryStatsItem> const&)
pub fn stub_3d7558() -> ! {
    todo!("0x3d7558 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<ChangeHistoryStatsItem>(rbx_core::SharedPtr<ChangeHistoryStatsItem> const&)")
}

// 0x3d76f0 — __ZN3RBX20ChangeHistoryService8Waypoint7getItemEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::getItem(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::ChangeHistoryService::Waypoint::getItem(boost::shared_ptr<RBX::Instance>)
pub fn stub_3d76f0() -> ! {
    todo!("0x3d76f0 RBX::ChangeHistoryService::Waypoint::getItem(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3d82b0 — __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")]
pub fn stub_3d82b0() -> ! {
    todo!("0x3d82b0 RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")
}

// 0x3d82c8 — __ZN3RBX9Selection12setSelectionISt14_List_iteratorIN5boost10shared_ptrINS_8InstanceEEEEEEvT_S8_
#[doc(alias = "void RBX::Selection::setSelection<std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>)")]
// was: void RBX::Selection::setSelection<std::_List_iterator<boost::shared_ptr<RBX::Instance>>>(std::_List_iterator<boost::shared_ptr<RBX::Instance>>,std::_List_iterator<boost::shared_ptr<RBX::Instance>>)
pub fn stub_3d82c8() -> ! {
    todo!("0x3d82c8 void RBX::Selection::setSelection<std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>)")
}

// 0x3d87a4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_3d87a4() -> ! {
    todo!("0x3d87a4 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0x3d887c — __ZNK5boost4_mfi3mf1IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_3d887c() -> ! {
    todo!("0x3d887c boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x3d8964 — __ZNSt4listIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_create_nodeERKS4_
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: std::list<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_create_node(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_3d8964() -> ! {
    todo!("0x3d8964 std::list<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x3d8a48 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14find_node_implIS6_SB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::find_node_impl<RBX::Instance *,std::equal_to<RBX::Instance *>>(unsigned long,RBX::Instance * const&,std::equal_to<RBX::Instance *> const&)const")]
pub fn stub_3d8a48() -> ! {
    todo!("0x3d8a48 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::find_node_impl<RBX::Instance *,std::equal_to<RBX::Instance *>>(unsigned long,RBX::Instance * const&,std::equal_to<RBX::Instance *> const&)const")
}

// 0x3d8ab4 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance *>>(RBX::Instance * const&,boost::unordered::detail::emplace_args1<RBX::Instance *> const&)")]
pub fn stub_3d8ab4() -> ! {
    todo!("0x3d8ab4 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance *>>(RBX::Instance * const&,boost::unordered::detail::emplace_args1<RBX::Instance *> const&)")
}

// 0x3d8c44 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")]
pub fn stub_3d8c44() -> ! {
    todo!("0x3d8c44 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")
}

// 0x3d8c98 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")]
pub fn stub_3d8c98() -> ! {
    todo!("0x3d8c98 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")
}

// 0x3d8dc0 — __ZNK5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_3d8dc0() -> ! {
    todo!("0x3d8dc0 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::min_buckets_for_size(unsigned long)const")
}

// 0x3d8e50 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::rehash_impl(unsigned long)")]
pub fn stub_3d8e50() -> ! {
    todo!("0x3d8e50 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::rehash_impl(unsigned long)")
}

// 0x3d8e7c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISC_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_3d8e7c() -> ! {
    todo!("0x3d8e7c boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x3d8ed0 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPN3RBX8InstanceEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>>>::construct(void)")]
pub fn stub_3d8ed0() -> ! {
    todo!("0x3d8ed0 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>>>::construct(void)")
}

// 0x3d8f08 — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(rbx_core::SharedPtr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>>>(boost::shared_ptr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_3d8f08() -> ! {
    todo!("0x3d8f08 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(rbx_core::SharedPtr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")
}

// 0x3d9090 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE20construct_with_valueINS1_13emplace_args1IS7_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_3d9090() -> ! {
    todo!("0x3d9090 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")
}

// 0x3d90bc — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::reserve_for_insert(unsigned long)
pub fn stub_3d90bc() -> ! {
    todo!("0x3d90bc boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::reserve_for_insert(unsigned long)")
}

// 0x3d910c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEED2Ev
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::~node_constructor()")]
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::~node_constructor()
pub fn stub_3d910c() -> ! {
    todo!("0x3d910c boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::~node_constructor()")
}

// 0x3d9138 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::create_buckets(unsigned long)
pub fn stub_3d9138() -> ! {
    todo!("0x3d9138 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::create_buckets(unsigned long)")
}

// 0x3d9260 — __ZNK5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const
pub fn stub_3d9260() -> ! {
    todo!("0x3d9260 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const")
}

// 0x3d92f0 — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::rehash_impl(unsigned long)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::rehash_impl(unsigned long)
pub fn stub_3d92f0() -> ! {
    todo!("0x3d92f0 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::rehash_impl(unsigned long)")
}

// 0x3d931c — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_3d931c() -> ! {
    todo!("0x3d931c boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x3d9374 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct(void)")]
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::construct(void)
pub fn stub_3d9374() -> ! {
    todo!("0x3d9374 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct(void)")
}

// 0x3d93b8 — __ZNK5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::find_node_impl<rbx_core::SharedPtr<RBX::Instance>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>(unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&)const")]
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::find_node_impl<boost::shared_ptr<RBX::Instance>,std::equal_to<boost::shared_ptr<RBX::Instance>>>(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::equal_to<boost::shared_ptr<RBX::Instance>> const&)const
pub fn stub_3d93b8() -> ! {
    todo!("0x3d93b8 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::find_node_impl<rbx_core::SharedPtr<RBX::Instance>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>(unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&)const")
}

// 0x3d9424 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::delete_buckets(void)")]
pub fn stub_3d9424() -> ! {
    todo!("0x3d9424 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::delete_buckets(void)")
}

// 0x3d9470 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::table(unsigned long,boost::hash<RBX::Instance *> const&,std::equal_to<RBX::Instance *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>> const&)")]
pub fn stub_3d9470() -> ! {
    todo!("0x3d9470 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::table(unsigned long,boost::hash<RBX::Instance *> const&,std::equal_to<RBX::Instance *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>> const&)")
}

// 0x3d94dc — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_buckets(void)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::delete_buckets(void)
pub fn stub_3d94dc() -> ! {
    todo!("0x3d94dc boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_buckets(void)")
}

// 0x3d9514 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_3d9514() -> ! {
    todo!("0x3d9514 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x3d9544 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::table(unsigned long,boost::hash<rbx_core::SharedPtr<RBX::Instance>> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::table(unsigned long,boost::hash<boost::shared_ptr<RBX::Instance>> const&,std::equal_to<boost::shared_ptr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>> const&)
pub fn stub_3d9544() -> ! {
    todo!("0x3d9544 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::table(unsigned long,boost::hash<rbx_core::SharedPtr<RBX::Instance>> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> const&)")
}

// 0x3da220 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot*)
pub fn stub_3da220() -> ! {
    todo!("0x3da220 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot*)")
}

// 0x3da244 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_3da244() -> ! {
    todo!("0x3da244 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3da270 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_3da270() -> ! {
    todo!("0x3da270 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x3da348 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_3da348() -> ! {
    todo!("0x3da348 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3da364 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_3da364() -> ! {
    todo!("0x3da364 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x3da380 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
pub fn stub_3da380() -> ! {
    todo!("0x3da380 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")
}

// 0x3da45c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_3da45c() -> ! {
    todo!("0x3da45c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x3da488 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_20ChangeHistoryServiceES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_3da488() -> ! {
    todo!("0x3da488 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")
}

// 0x3da560 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20ChangeHistoryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3da560() -> ! {
    todo!("0x3da560 boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3dc308 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE8_M_clearEv
#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_clear(void)")]
// was: std::_List_base<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_clear(void)
pub fn stub_3dc308() -> ! {
    todo!("0x3dc308 std::_List_base<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_clear(void)")
}

// 0x3dd564 — __ZNSt3mapIPN3RBX8InstanceEjSt4lessIS2_ESaISt4pairIKS2_jEEEixERS6_
#[doc(alias = "std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")]
pub fn stub_3dd564() -> ! {
    todo!("0x3dd564 std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")
}

// 0x3dd5bc — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")]
pub fn stub_3dd5bc() -> ! {
    todo!("0x3dd5bc std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")
}

// 0x3dd670 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")]
pub fn stub_3dd670() -> ! {
    todo!("0x3dd670 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")
}

// 0x3dd6c8 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")]
pub fn stub_3dd6c8() -> ! {
    todo!("0x3dd6c8 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")
}

// 0x3ddc30 — __ZN3RBX9CreatableINS_8InstanceEE6createI22ChangeHistoryStatsItemEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)")]
// was: boost::shared_ptr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)
pub fn stub_3ddc30() -> ! {
    todo!("0x3ddc30 rbx_core::SharedPtr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)")
}

// 0x3de7b4 — __ZN5boost10shared_ptrI22ChangeHistoryStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3de7b4() -> ! {
    todo!("0x3de7b4 rbx_core::SharedPtr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3de964 — __ZN5boost6detail12shared_countC2IP22ChangeHistoryStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3de964() -> ! {
    todo!("0x3de964 boost::detail::shared_count::shared_count<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3dea6c — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3dea6c() -> ! {
    todo!("0x3dea6c boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3dea70 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_3dea70() -> ! {
    todo!("0x3dea70 boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3dea74 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_3dea74() -> ! {
    todo!("0x3dea74 boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3dea94 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_3dea94() -> ! {
    todo!("0x3dea94 boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3deaac — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_3deaac() -> ! {
    todo!("0x3deaac boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}