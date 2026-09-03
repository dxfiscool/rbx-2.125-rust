//! RBX::Reflection::Descriptor skeletons from `ida/export.json`.
//! Remaining batch — compile-only cutover points.

// --- remaining batch (150) from ida/export.json: RBX::Reflection not yet stubbed, sorted by ea ---

// 0x4a15b0 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
pub fn stub_0x4a15b0() {
    // IDA 0x4a15b0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x4a2ae8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TimerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TimerService,RBX::TimerService>(rbx_core::SharedPtr<RBX::TimerService> const*,RBX::TimerService *)const")]
pub fn stub_0x4a2ae8() {
    // IDA 0x4a2ae8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x4a38b8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x4a38b8() -> ! {
    todo!("0x4a38b8 __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x4a3aa8 — __ZN3RBX10Reflection9EventDescINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::~EventDesc()")]
pub fn stub_0x4a3aa8() {
    // IDA 0x4a3aa8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a3b5c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x4a3b5c() -> ! {
    todo!("0x4a3b5c __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

// 0x4a3cb0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x4a3cb0() -> ! {
    todo!("0x4a3cb0 __ZNK3RBX10Reflection13EventDescImplILi2ENS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

// 0x4a3e20 — __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Explosion,void ()(rbx_core::SharedPtr<RBX::Instance>,float),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,float)> RBX::Explosion::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x4a3e20() -> ! {
    todo!("0x4a3e20 __ZNK3RBX10Reflection13EventDescBaseINS_9ExplosionEFvN5boost10shared_ptrINS_8InstanceEEEfEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x4a3fac — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x4a3fac() -> ! {
    todo!("0x4a3fac __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKfNS4_IS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_")
}

// 0x4a40c8 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> const&,float const&)")]
pub fn stub_0x4a40c8() -> ! {
    todo!("0x4a40c8 __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEfEEvRKT_RKT0_")
}

// 0x4a442c — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x4a442c() -> ! {
    todo!("0x4a442c __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS2_10Reflection18GenericSlotWrapperERKS4_RKfEENS7_5list3INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_")
}

// 0x4a4524 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4a4524() -> ! {
    todo!("0x4a4524 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")
}

// 0x4a4540 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,float>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,float)")]
pub fn stub_0x4a4540() -> ! {
    todo!("0x4a4540 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSC_fE6invokeERNS1_15function_bufferESC_f")
}

// 0x4a4554 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x4a4554() -> ! {
    todo!("0x4a4554 __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x4a463c — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x4a463c() -> ! {
    todo!("0x4a463c __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x4a4720 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x4a4720() -> ! {
    todo!("0x4a4720 __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS4_10Reflection18GenericSlotWrapperERKS6_RKfEENS9_5list3INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

// 0x4a47f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,float>(rbx_core::SharedPtr<RBX::Instance> &,float &)")]
pub fn stub_0x4a47f4() -> ! {
    todo!("0x4a47f4 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKfEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS9_fEEvRT_RT0_")
}

// 0x4a4810 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,float const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x4a4810() -> ! {
    todo!("0x4a4810 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0x4a5834 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::EnumPropDescriptor<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>(char const*,char const*,RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a5834() -> ! {
    todo!("0x4a5834 __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4a59e8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::~EnumPropDescriptor()")]
pub fn stub_0x4a59e8() {
    // IDA 0x4a59e8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a5a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isReadOnly(void)const")]
pub fn stub_0x4a5a14() -> ! {
    todo!("0x4a5a14 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10isReadOnlyEv")
}

// 0x4a5a24 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::isWriteOnly(void)const")]
pub fn stub_0x4a5a24() -> ! {
    todo!("0x4a5a24 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11isWriteOnlyEv")
}

// 0x4a5a34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5a34() -> ! {
    todo!("0x4a5a34 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")
}

// 0x4a5a5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4a5a5c() -> ! {
    todo!("0x4a5a5c __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x4a5a80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4a5a80() -> ! {
    todo!("0x4a5a80 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x4a5bcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4a5bcc() -> ! {
    todo!("0x4a5bcc __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

// 0x4a5bf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::hasStringValue(void)const")]
pub fn stub_0x4a5bf4() -> ! {
    todo!("0x4a5bf4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14hasStringValueEv")
}

// 0x4a5bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5bf8() -> ! {
    todo!("0x4a5bf8 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x4a5c1c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4a5c1c() -> ! {
    todo!("0x4a5c1c __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x4a5c5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4a5c5c() -> ! {
    todo!("0x4a5c5c __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x4a5c7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4a5c7c() -> ! {
    todo!("0x4a5c7c __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x4a5ebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5ebc() -> ! {
    todo!("0x4a5ebc __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x4a5ed8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4a5ed8() -> ! {
    todo!("0x4a5ed8 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x4a5f0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5f0c() -> ! {
    todo!("0x4a5f0c __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x4a5f14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a5f14() -> ! {
    todo!("0x4a5f14 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x4a5f60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a5f60() -> ! {
    todo!("0x4a5f60 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x4a5f80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4a5f80() -> ! {
    todo!("0x4a5f80 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x4a5fb8 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToIndex(RBX::Explosion::ExplosionType)const")]
pub fn stub_0x4a5fb8(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x4a5fb8: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x4a6028 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a6028() -> ! {
    todo!("0x4a6028 __ZNK3RBX10Reflection18EnumPropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x4a606c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isReadOnly(void)const")]
pub fn stub_0x4a606c() -> ! {
    todo!("0x4a606c __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")
}

// 0x4a6070 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::isWriteOnly(void)const")]
pub fn stub_0x4a6070() -> ! {
    todo!("0x4a6070 __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")
}

// 0x4a6074 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a6074() -> ! {
    todo!("0x4a6074 __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4a6094 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,RBX::Explosion::ExplosionType>::GetSetImpl<RBX::Explosion::ExplosionType (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(RBX::Explosion::ExplosionType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Explosion::ExplosionType const&)const")]
pub fn stub_0x4a6094() -> ! {
    todo!("0x4a6094 __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionENS2_13ExplosionTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x4a60b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::initSingleton(void)")]
pub fn stub_0x4a60b8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0x4a60b8: Singleton<EnumDesc<T>>::initSingleton -- thunk to doGetSingleton (decompiled 0x4a60b8). Rust: forward to the singleton.
    crate::generated::stub_0x4b6a3c()
}

// 0x4a60bc — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,G3D::Vector3 RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a60bc() -> ! {
    todo!("0x4a60bc __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS9_MT_S3_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4a6250 — __ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x4a6250() {
    // IDA 0x4a6250: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a6280 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4a6280() -> ! {
    todo!("0x4a6280 __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x4a62b0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4a62b0() -> ! {
    todo!("0x4a62b0 __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

// 0x4a63c8 — __ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector3>::~TypedPropertyDescriptor()")]
pub fn stub_0x4a63c8() {
    // IDA 0x4a63c8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a63f4 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
pub fn stub_0x4a63f4() -> ! {
    todo!("0x4a63f4 __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv")
}

// 0x4a63f8 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
pub fn stub_0x4a63f8() -> ! {
    todo!("0x4a63f8 __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv")
}

// 0x4a63fc — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a63fc() -> ! {
    todo!("0x4a63fc __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4a6418 — __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Vector3,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x4a6418() -> ! {
    todo!("0x4a6418 __ZNK3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x4a64ac — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Explosion>(char const*,char const*,float RBX::Explosion::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a64ac() -> ! {
    todo!("0x4a64ac __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_9ExplosionEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4a6640 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x4a6640() {
    // IDA 0x4a6640: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a666c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isReadOnly(void)const")]
pub fn stub_0x4a666c() -> ! {
    todo!("0x4a666c __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE10isReadOnlyEv")
}

// 0x4a6670 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::isWriteOnly(void)const")]
pub fn stub_0x4a6670() -> ! {
    todo!("0x4a6670 __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE11isWriteOnlyEv")
}

// 0x4a6674 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a6674() -> ! {
    todo!("0x4a6674 __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4a6680 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Explosion>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x4a6680() -> ! {
    todo!("0x4a6680 __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_9ExplosionEE8setValueEPNS0_13DescribedBaseERKf")
}

// 0x4a66dc — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::PropDescriptor<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>(char const*,char const*,float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a66dc() -> ! {
    todo!("0x4a66dc __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4a67f0 — __ZN3RBX10Reflection14PropDescriptorINS_9ExplosionEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::~PropDescriptor()")]
pub fn stub_0x4a67f0() {
    // IDA 0x4a67f0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a681c — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isReadOnly(void)const")]
pub fn stub_0x4a681c() -> ! {
    todo!("0x4a681c __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")
}

// 0x4a6820 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::isWriteOnly(void)const")]
pub fn stub_0x4a6820() -> ! {
    todo!("0x4a6820 __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")
}

// 0x4a6824 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a6824() -> ! {
    todo!("0x4a6824 __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4a6844 — __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Explosion,float>::GetSetImpl<float (RBX::Explosion::*)(void)const,void (RBX::Explosion::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x4a6844() -> ! {
    todo!("0x4a6844 __ZNK3RBX10Reflection14PropDescriptorINS_9ExplosionEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")
}

// 0x4a7734 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
pub fn stub_0x4a7734() {
    // IDA 0x4a7734: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4a7f5c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ExtrudedPartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance>(rbx_core::SharedPtr<RBX::ExtrudedPartInstance> const*,RBX::ExtrudedPartInstance *)const")]
pub fn stub_0x4a7f5c() {
    // IDA 0x4a7f5c: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x4a88f0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::EnumPropDescriptor<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>(char const*,char const*,RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a88f0() -> ! {
    todo!("0x4a88f0 __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4a8aa4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumPropDescriptor()")]
pub fn stub_0x4a8aa4() {
    // IDA 0x4a8aa4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a8ad0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isReadOnly(void)const")]
pub fn stub_0x4a8ad0() -> ! {
    todo!("0x4a8ad0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10isReadOnlyEv")
}

// 0x4a8ae0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::isWriteOnly(void)const")]
pub fn stub_0x4a8ae0() -> ! {
    todo!("0x4a8ae0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11isWriteOnlyEv")
}

// 0x4a8af0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8af0() -> ! {
    todo!("0x4a8af0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11equalValuesEPKNS0_13DescribedBaseES7_")
}

// 0x4a8b18 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4a8b18() -> ! {
    todo!("0x4a8b18 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x4a8b3c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4a8b3c() -> ! {
    todo!("0x4a8b3c __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x4a8c88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4a8c88() -> ! {
    todo!("0x4a8c88 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

// 0x4a8cac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::hasStringValue(void)const")]
pub fn stub_0x4a8cac() -> ! {
    todo!("0x4a8cac __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14hasStringValueEv")
}

// 0x4a8cb0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8cb0() -> ! {
    todo!("0x4a8cb0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x4a8cd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4a8cd4() -> ! {
    todo!("0x4a8cd4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x4a8d14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4a8d14() -> ! {
    todo!("0x4a8d14 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x4a8d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4a8d34() -> ! {
    todo!("0x4a8d34 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x4a8f74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8f74() -> ! {
    todo!("0x4a8f74 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x4a8f90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4a8f90() -> ! {
    todo!("0x4a8f90 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x4a8fc4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a8fc4() -> ! {
    todo!("0x4a8fc4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x4a8fcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a8fcc() -> ! {
    todo!("0x4a8fcc __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x4a9018 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a9018() -> ! {
    todo!("0x4a9018 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x4a9038 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4a9038() -> ! {
    todo!("0x4a9038 __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x4a906c — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToIndex(RBX::ExtrudedPartInstance::VisualTrussStyle)const")]
pub fn stub_0x4a906c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0x4a906c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0x4a90dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4a90dc() -> ! {
    todo!("0x4a90dc __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x4a911c — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isReadOnly(void)const")]
pub fn stub_0x4a911c() -> ! {
    todo!("0x4a911c __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")
}

// 0x4a9120 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isWriteOnly(void)const")]
pub fn stub_0x4a9120() -> ! {
    todo!("0x4a9120 __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")
}

// 0x4a9124 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a9124() -> ! {
    todo!("0x4a9124 __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4a9144 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
pub fn stub_0x4a9144() -> ! {
    todo!("0x4a9144 __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x4a9728 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_0x4a9728() {
    // IDA 0x4a9728: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x4a9de0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4a9de0() -> ! {
    todo!("0x4a9de0 __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4a9f94 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
pub fn stub_0x4a9f94() {
    // IDA 0x4a9f94: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x4a9fc0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isReadOnly(void)const")]
pub fn stub_0x4a9fc0() -> ! {
    todo!("0x4a9fc0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10isReadOnlyEv")
}

// 0x4a9fd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isWriteOnly(void)const")]
pub fn stub_0x4a9fd0() -> ! {
    todo!("0x4a9fd0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11isWriteOnlyEv")
}

// 0x4a9fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4a9fe0() -> ! {
    todo!("0x4a9fe0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_")
}

// 0x4aa008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4aa008() -> ! {
    todo!("0x4aa008 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x4aa02c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4aa02c() -> ! {
    todo!("0x4aa02c __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x4aa178 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4aa178() -> ! {
    todo!("0x4aa178 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

// 0x4aa19c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::hasStringValue(void)const")]
pub fn stub_0x4aa19c() -> ! {
    todo!("0x4aa19c __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14hasStringValueEv")
}

// 0x4aa1a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa1a0() -> ! {
    todo!("0x4aa1a0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x4aa1c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4aa1c4() -> ! {
    todo!("0x4aa1c4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x4aa204 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4aa204() -> ! {
    todo!("0x4aa204 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x4aa224 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4aa224() -> ! {
    todo!("0x4aa224 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x4aa464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa464() -> ! {
    todo!("0x4aa464 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x4aa480 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4aa480() -> ! {
    todo!("0x4aa480 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x4aa4b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa4b4() -> ! {
    todo!("0x4aa4b4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x4aa4bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4aa4bc() -> ! {
    todo!("0x4aa4bc __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x4aa508 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa508() -> ! {
    todo!("0x4aa508 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x4aa528 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4aa528() -> ! {
    todo!("0x4aa528 __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x4aa55c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4aa55c() -> ! {
    todo!("0x4aa55c __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x4aa59c — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isReadOnly(void)const")]
pub fn stub_0x4aa59c() -> ! {
    todo!("0x4aa59c __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")
}

// 0x4aa5a0 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isWriteOnly(void)const")]
pub fn stub_0x4aa5a0() -> ! {
    todo!("0x4aa5a0 __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")
}

// 0x4aa5a4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4aa5a4() -> ! {
    todo!("0x4aa5a4 __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4aa5c4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
pub fn stub_0x4aa5c4() -> ! {
    todo!("0x4aa5c4 __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x4aab84 — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler14PriorityMethodEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::PriorityMethod>(void)")]
pub fn stub_0x4aab84() -> ! {
    todo!("0x4aab84 __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler14PriorityMethodEEERKS1_v")
}

// 0x4aabb8 — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler3Job17SleepAdjustMethodEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::Job::SleepAdjustMethod>(void)")]
pub fn stub_0x4aabb8() -> ! {
    todo!("0x4aabb8 __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler3Job17SleepAdjustMethodEEERKS1_v")
}

// 0x4aabec — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler16ThreadPoolConfigEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::ThreadPoolConfig>(void)")]
pub fn stub_0x4aabec() -> ! {
    todo!("0x4aabec __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler16ThreadPoolConfigEEERKS1_v")
}

// 0x4aac20 — __ZN3RBX10Reflection4Type12getSingletonINS_10Controller6ButtonEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Controller::Button>(void)")]
pub fn stub_0x4aac20() -> ! {
    todo!("0x4aac20 __ZN3RBX10Reflection4Type12getSingletonINS_10Controller6ButtonEEERKS1_v")
}

// 0x4aac54 — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject16TweenEasingStyleEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenEasingStyle>(void)")]
pub fn stub_0x4aac54() -> ! {
    todo!("0x4aac54 __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject16TweenEasingStyleEEERKS1_v")
}

// 0x4aac88 — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject11TweenStatusEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenStatus>(void)")]
pub fn stub_0x4aac88() -> ! {
    todo!("0x4aac88 __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject11TweenStatusEEERKS1_v")
}

// 0x4aacbc — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject20TweenEasingDirectionEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenEasingDirection>(void)")]
pub fn stub_0x4aacbc() -> ! {
    todo!("0x4aacbc __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject20TweenEasingDirectionEEERKS1_v")
}

// 0x4aacf0 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10XAlignmentEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::XAlignment>(void)")]
pub fn stub_0x4aacf0() -> ! {
    todo!("0x4aacf0 __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10XAlignmentEEERKS1_v")
}

// 0x4aad24 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10YAlignmentEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::YAlignment>(void)")]
pub fn stub_0x4aad24() -> ! {
    todo!("0x4aad24 __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10YAlignmentEEERKS1_v")
}

// 0x4aad58 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService8FontSizeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::FontSize>(void)")]
pub fn stub_0x4aad58() -> ! {
    todo!("0x4aad58 __ZN3RBX10Reflection4Type12getSingletonINS_11TextService8FontSizeEEERKS1_v")
}

// 0x4aad8c — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService4FontEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::Font>(void)")]
pub fn stub_0x4aad8c() -> ! {
    todo!("0x4aad8c __ZN3RBX10Reflection4Type12getSingletonINS_11TextService4FontEEERKS1_v")
}

// 0x4aadc0 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraType>(void)")]
pub fn stub_0x4aadc0() -> ! {
    todo!("0x4aadc0 __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraTypeEEERKS1_v")
}

// 0x4aadf4 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraModeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraMode>(void)")]
pub fn stub_0x4aadf4() -> ! {
    todo!("0x4aadf4 __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraModeEEERKS1_v")
}

// 0x4aae28 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera13CameraPanModeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraPanMode>(void)")]
pub fn stub_0x4aae28() -> ! {
    todo!("0x4aae28 __ZN3RBX10Reflection4Type12getSingletonINS_6Camera13CameraPanModeEEERKS1_v")
}

// 0x4aae5c — __ZN3RBX10Reflection4Type12getSingletonINS_16LegacyController9InputTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::LegacyController::InputType>(void)")]
pub fn stub_0x4aae5c() -> ! {
    todo!("0x4aae5c __ZN3RBX10Reflection4Type12getSingletonINS_16LegacyController9InputTypeEEERKS1_v")
}

// 0x4aae90 — __ZN3RBX10Reflection4Type12getSingletonINS_16DataModelArbiter16ConcurrencyModelEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModelArbiter::ConcurrencyModel>(void)")]
pub fn stub_0x4aae90() -> ! {
    todo!("0x4aae90 __ZN3RBX10Reflection4Type12getSingletonINS_16DataModelArbiter16ConcurrencyModelEEERKS1_v")
}

// 0x4aaec4 — __ZN3RBX10Reflection4Type12getSingletonINS_13DebugSettings14ErrorReportingEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DebugSettings::ErrorReporting>(void)")]
pub fn stub_0x4aaec4() -> ! {
    todo!("0x4aaec4 __ZN3RBX10Reflection4Type12getSingletonINS_13DebugSettings14ErrorReportingEEERKS1_v")
}

// 0x4aaef8 — __ZN3RBX10Reflection4Type12getSingletonINS_9EThrottle13EThrottleTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::EThrottle::EThrottleType>(void)")]
pub fn stub_0x4aaef8() -> ! {
    todo!("0x4aaef8 __ZN3RBX10Reflection4Type12getSingletonINS_9EThrottle13EThrottleTypeEEERKS1_v")
}

// 0x4aaf2c — __ZN3RBX10Reflection4Type12getSingletonINS_8NormalIdEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::NormalId>(void)")]
pub fn stub_0x4aaf2c() -> ! {
    todo!("0x4aaf2c __ZN3RBX10Reflection4Type12getSingletonINS_8NormalIdEEERKS1_v")
}

// 0x4aaf60 — __ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<G3D::Vector3::Axis>(void)")]
pub fn stub_0x4aaf60() -> ! {
    todo!("0x4aaf60 __ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v")
}

// 0x4aaf94 — __ZN3RBX10Reflection4Type12getSingletonINS_8Humanoid6StatusEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Humanoid::Status>(void)")]
pub fn stub_0x4aaf94() -> ! {
    todo!("0x4aaf94 __ZN3RBX10Reflection4Type12getSingletonINS_8Humanoid6StatusEEERKS1_v")
}

// 0x4aafc8 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel11CreatorTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::CreatorType>(void)")]
pub fn stub_0x4aafc8() -> ! {
    todo!("0x4aafc8 __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel11CreatorTypeEEERKS1_v")
}

// 0x4aaffc — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel5GenreEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::Genre>(void)")]
pub fn stub_0x4aaffc() -> ! {
    todo!("0x4aaffc __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel5GenreEEERKS1_v")
}

// 0x4ab030 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel16GearGenreSettingEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::GearGenreSetting>(void)")]
pub fn stub_0x4ab030() -> ! {
    todo!("0x4ab030 __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel16GearGenreSettingEEERKS1_v")
}

// 0x4ab064 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel8GearTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::GearType>(void)")]
pub fn stub_0x4ab064() -> ! {
    todo!("0x4ab064 __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel8GearTypeEEERKS1_v")
}

// 0x4ab098 — __ZN3RBX10Reflection4Type12getSingletonINS_8Instance10SaveFilterEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Instance::SaveFilter>(void)")]
pub fn stub_0x4ab098() -> ! {
    todo!("0x4ab098 __ZN3RBX10Reflection4Type12getSingletonINS_8Instance10SaveFilterEEERKS1_v")
}

// 0x4ab0cc — __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService12FriendStatusEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::FriendService::FriendStatus>(void)")]
pub fn stub_0x4ab0cc() -> ! {
    todo!("0x4ab0cc __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService12FriendStatusEEERKS1_v")
}

// 0x4ab100 — __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService15FriendEventTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::FriendService::FriendEventType>(void)")]
pub fn stub_0x4ab100() -> ! {
    todo!("0x4ab100 __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService15FriendEventTypeEEERKS1_v")
}

// 0x4ab134 — __ZN3RBX10Reflection4Type12getSingletonINS_18SkateboardPlatform9MoveStateEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SkateboardPlatform::MoveState>(void)")]
pub fn stub_0x4ab134() -> ! {
    todo!("0x4ab134 __ZN3RBX10Reflection4Type12getSingletonINS_18SkateboardPlatform9MoveStateEEERKS1_v")
}

// 0x4ab168 — __ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SoundType>(void)")]
pub fn stub_0x4ab168() -> ! {
    todo!("0x4ab168 __ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v")
}

// 0x4ab19c — __ZN3RBX10Reflection4Type12getSingletonINS_11SurfaceTypeEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SurfaceType>(void)")]
pub fn stub_0x4ab19c() -> ! {
    todo!("0x4ab19c __ZN3RBX10Reflection4Type12getSingletonINS_11SurfaceTypeEEERKS1_v")
}

// 0x4ab1d0 — __ZN3RBX10Reflection4Type12getSingletonINS_12PartInstance10FormFactorEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::PartInstance::FormFactor>(void)")]
pub fn stub_0x4ab1d0() -> ! {
    todo!("0x4ab1d0 __ZN3RBX10Reflection4Type12getSingletonINS_12PartInstance10FormFactorEEERKS1_v")
}

// 0x4ab204 — __ZN3RBX10Reflection4Type12getSingletonINS_16UserInputService14SwipeDirectionEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::UserInputService::SwipeDirection>(void)")]
pub fn stub_0x4ab204() -> ! {
    todo!("0x4ab204 __ZN3RBX10Reflection4Type12getSingletonINS_16UserInputService14SwipeDirectionEEERKS1_v")
}

// 0x4ab238 — __ZN3RBX10Reflection4Type12getSingletonINS_8MaterialEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Material>(void)")]
pub fn stub_0x4ab238() -> ! {
    todo!("0x4ab238 __ZN3RBX10Reflection4Type12getSingletonINS_8MaterialEEERKS1_v")
}
