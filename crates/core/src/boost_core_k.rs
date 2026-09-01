//! boost_core_k — 100 boost stubs (EA-ordered, next uncovered after boost_core_j up to 0x5be070).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 100 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.


#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x58e990 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_58e990() -> ! {
    todo!("0x58e990 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x58fe3c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_58fe3c() -> ! {
    todo!("0x58fe3c __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5917cc — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<3,RBX::InsertService,void ()(std::string,int,int),rbx::remote_signal<void ()(std::string,int,int)>,rbx::remote_signal<void ()(std::string,int,int)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5917cc() -> ! {
    todo!("0x5917cc __ZNK3RBX10Reflection13EventDescImplILi3ENS_13InsertServiceEFvSsiiEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "__ZN5boost8functionIFvSsiiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSE_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x591cc0 — __ZN5boost8functionIFvSsiiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSE_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_591cc0() -> ! {
    todo!("0x591cc0 __ZN5boost8functionIFvSsiiEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSE_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,int,int>::invoke(boost::detail::function::function_buffer &,std::string,int,int)")]
// 0x591da4 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSD_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEvSsiiE6invokeERNS1_15function_bufferESsii
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,int,int>::invoke(boost::detail::function::function_buffer &,std::string,int,int)
pub fn stub_591da4() -> ! {
    todo!("0x591da4 __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSD_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEvSsiiE6invokeERNS1_15function_bufferESsii")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// 0x591dcc — __ZNK5boost6detail8function13basic_vtable3IvSsiiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSF_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable3<void,std::string,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const
pub fn stub_591dcc() -> ! {
    todo!("0x591dcc __ZNK5boost6detail8function13basic_vtable3IvSsiiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSF_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x591eb4 — __ZNK5boost6detail8function13basic_vtable3IvSsiiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSF_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable3<void,std::string,int,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_591eb4() -> ! {
    todo!("0x591eb4 __ZNK5boost6detail8function13basic_vtable3IvSsiiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSF_EENS5_5list4INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&> &,boost::_bi::list3<std::string &,int &,int &> &,int)")]
// 0x591f98 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsRKiSK_EENS0_5list3IRSsRiSO_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&> &,boost::_bi::list3<std::string &,int &,int &> &,int)
pub fn stub_591f98() -> ! {
    todo!("0x591f98 __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKSsRKiSK_EENS0_5list3IRSsRiSO_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x591fc0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSD_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,int const&,int const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_591fc0() -> ! {
    todo!("0x591fc0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKiSD_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::function<void ()(std::string,int,int)>>(boost::function<void ()(std::string,int,int)> const&)")]
// 0x592118 — __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_592118() -> ! {
    todo!("0x592118 __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::function<void ()(std::string,int,int)>>::~callable_slot()")]
// 0x59220c — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_59220c() -> ! {
    todo!("0x59220c __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::function<void ()(std::string,int,int)>>::~callable_slot()")]
// 0x59231c — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_59231c() -> ! {
    todo!("0x59231c __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x59244c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::call(std::string,int,int)
pub fn stub_59244c() -> ! {
    todo!("0x59244c __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii")
}

#[doc(alias = "boost::function3<void,std::string,int,int>::operator()(std::string,int,int)const")]
// 0x592454 — __ZNK5boost9function3IvSsiiEclESsii
pub fn stub_592454() -> ! {
    todo!("0x592454 __ZNK5boost9function3IvSsiiEclESsii")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::~callable()")]
// 0x5925b0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_5925b0() -> ! {
    todo!("0x5925b0 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::~callable()")]
// 0x5926c0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_5926c0() -> ! {
    todo!("0x5926c0 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")
}

#[doc(alias = "boost::function3<void,std::string,int,int>::assign_to_own(boost::function3<void,std::string,int,int> const&)")]
// 0x5927f0 — __ZN5boost9function3IvSsiiE13assign_to_ownERKS1_
pub fn stub_5927f0() -> ! {
    todo!("0x5927f0 __ZN5boost9function3IvSsiiE13assign_to_ownERKS1_")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,RBX::ContentId),rbx::remote_signal<void ()(std::string,RBX::ContentId)>,rbx::remote_signal<void ()(std::string,RBX::ContentId)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x592c08 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsNS_9ContentIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,RBX::ContentId),rbx::remote_signal<void ()(std::string,RBX::ContentId)>,rbx::remote_signal<void ()(std::string,RBX::ContentId)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_592c08() -> ! {
    todo!("0x592c08 __ZNK3RBX10Reflection13EventDescImplILi2ENS_13InsertServiceEFvSsNS_9ContentIdEEN3rbx13remote_signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> &)")]
// 0x59324c — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(std::string,RBX::ContentId)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> &)
pub fn stub_59324c() -> ! {
    todo!("0x59324c __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,RBX::ContentId const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// 0x593594 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS1_9ContentIdENS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,RBX::ContentId const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_593594() -> ! {
    todo!("0x593594 __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS1_9ContentIdENS_10shared_ptrIS3_EENS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_")
}

#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::clear(void)")]
// 0x593818 — __ZN5boost9function2IvSsN3RBX9ContentIdEE5clearEv
pub fn stub_593818() -> ! {
    todo!("0x593818 __ZN5boost9function2IvSsN3RBX9ContentIdEE5clearEv")
}

#[doc(alias = "__ZN5boost8functionIFvSsN3RBX9ContentIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x593844 — __ZN5boost8functionIFvSsN3RBX9ContentIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_593844() -> ! {
    todo!("0x593844 __ZN5boost8functionIFvSsN3RBX9ContentIdEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvSsN3RBX9ContentIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// 0x593928 — __ZN5boost9function2IvSsN3RBX9ContentIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_593928() -> ! {
    todo!("0x593928 __ZN5boost9function2IvSsN3RBX9ContentIdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,std::string,RBX::ContentId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x593a10 — __ZN5boost9function2IvSsN3RBX9ContentIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,std::string,RBX::ContentId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_593a10() -> ! {
    todo!("0x593a10 __ZN5boost9function2IvSsN3RBX9ContentIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSL_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x593b08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS7_9ContentIdEEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_593b08() -> ! {
    todo!("0x593b08 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS7_9ContentIdEEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,RBX::ContentId>::invoke(boost::detail::function::function_buffer &,std::string,RBX::ContentId)")]
// 0x593b24 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS7_9ContentIdEEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSsSC_E6invokeERNS1_15function_bufferESsSC_
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,RBX::ContentId>::invoke(boost::detail::function::function_buffer &,std::string,RBX::ContentId)
pub fn stub_593b24() -> ! {
    todo!("0x593b24 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS7_9ContentIdEEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEvSsSC_E6invokeERNS1_15function_bufferESsSC_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,RBX::ContentId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x593b2c — __ZNK5boost6detail8function13basic_vtable2IvSsN3RBX9ContentIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string,RBX::ContentId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_593b2c() -> ! {
    todo!("0x593b2c __ZNK5boost6detail8function13basic_vtable2IvSsN3RBX9ContentIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,RBX::ContentId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x593c14 — __ZNK5boost6detail8function13basic_vtable2IvSsN3RBX9ContentIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string,RBX::ContentId>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_593c14() -> ! {
    todo!("0x593c14 __ZNK5boost6detail8function13basic_vtable2IvSsN3RBX9ContentIdEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string,RBX::ContentId>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x593cf8 — __ZNK5boost6detail8function13basic_vtable2IvSsN3RBX9ContentIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string,RBX::ContentId>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_593cf8() -> ! {
    todo!("0x593cf8 __ZNK5boost6detail8function13basic_vtable2IvSsN3RBX9ContentIdEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<std::string,RBX::ContentId>(std::string &,RBX::ContentId &)")]
// 0x593dcc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS4_9ContentIdEEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclISsS9_EEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<std::string,RBX::ContentId>(std::string &,RBX::ContentId &)
pub fn stub_593dcc() -> ! {
    todo!("0x593dcc __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS4_9ContentIdEEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSI_ILi2EEEEEEclISsS9_EEvRT_RT0_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x593de8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS7_9ContentIdEEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,RBX::ContentId const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_593de8() -> ! {
    todo!("0x593de8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS7_9ContentIdEEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSL_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::function<void ()(std::string,RBX::ContentId)>>(boost::function<void ()(std::string,RBX::ContentId)> const&)")]
// 0x593f40 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
pub fn stub_593f40() -> ! {
    todo!("0x593f40 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>*>(boost::function<void ()(std::string,RBX::ContentId)> const&,rbx::signals::signal<void ()(std::string,RBX::ContentId)>*)")]
// 0x594034 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
pub fn stub_594034() -> ! {
    todo!("0x594034 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::function<void ()(std::string,RBX::ContentId)>>::~callable_slot()")]
// 0x594130 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_594130() -> ! {
    todo!("0x594130 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::function<void ()(std::string,RBX::ContentId)>>::~callable_slot()")]
// 0x594240 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_594240() -> ! {
    todo!("0x594240 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x594370 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
pub fn stub_594370() -> ! {
    todo!("0x594370 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x594518 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)
pub fn stub_594518() -> ! {
    todo!("0x594518 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")
}

#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::operator()(std::string,RBX::ContentId)const")]
// 0x594520 — __ZNK5boost9function2IvSsN3RBX9ContentIdEEclESsS2_
pub fn stub_594520() -> ! {
    todo!("0x594520 __ZNK5boost9function2IvSsN3RBX9ContentIdEEclESsS2_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x594708 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
pub fn stub_594708() -> ! {
    todo!("0x594708 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x594818 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
pub fn stub_594818() -> ! {
    todo!("0x594818 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")
}

#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::assign_to_own(boost::function2<void,std::string,RBX::ContentId> const&)")]
// 0x594948 — __ZN5boost9function2IvSsN3RBX9ContentIdEE13assign_to_ownERKS3_
pub fn stub_594948() -> ! {
    todo!("0x594948 __ZN5boost9function2IvSsN3RBX9ContentIdEE13assign_to_ownERKS3_")
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS_10shared_ptrINS1_8InstanceEEEEEESB_ENS7_5list2INS7_5valueISD_EENSH_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x595aac — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS_10shared_ptrINS1_8InstanceEEEEEESB_ENS7_5list2INS7_5valueISD_EENSH_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_595aac() -> ! {
    todo!("0x595aac __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS_10shared_ptrINS1_8InstanceEEEEEESB_ENS7_5list2INS7_5valueISD_EENSH_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS_10shared_ptrINS1_8InstanceEEEEEESB_ENS6_5list2INS6_5valueISD_EENSH_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x595c00 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS_10shared_ptrINS1_8InstanceEEEEEESB_ENS6_5list2INS6_5valueISD_EENSH_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_595c00() -> ! {
    todo!("0x595c00 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS_10shared_ptrINS1_8InstanceEEEEEESB_ENS6_5list2INS6_5valueISD_EENSH_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::clear(void)")]
// 0x59e0f8 — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE5clearEv
// was: boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::clear(void)
pub fn stub_59e0f8() -> ! {
    todo!("0x59e0f8 __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE5clearEv")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualGlue,RBX::ManualGlue>(rbx_core::SharedPtr<RBX::ManualGlue> const*,RBX::ManualGlue *)const")]
// 0x5a5a98 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualGlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualGlue,RBX::ManualGlue>(boost::shared_ptr<RBX::ManualGlue> const*,RBX::ManualGlue *)const
pub fn stub_5a5a98() -> ! {
    todo!("0x5a5a98 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualGlueES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualWeld,RBX::ManualWeld>(rbx_core::SharedPtr<RBX::ManualWeld> const*,RBX::ManualWeld *)const")]
// 0x5a6448 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualWeldES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualWeld,RBX::ManualWeld>(boost::shared_ptr<RBX::ManualWeld> const*,RBX::ManualWeld *)const
pub fn stub_5a6448() -> ! {
    todo!("0x5a6448 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualWeldES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Motor,RBX::Motor>(rbx_core::SharedPtr<RBX::Motor> const*,RBX::Motor *)const")]
// 0x5a7dac — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MotorES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Motor,RBX::Motor>(boost::shared_ptr<RBX::Motor> const*,RBX::Motor *)const
pub fn stub_5a7dac() -> ! {
    todo!("0x5a7dac __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MotorES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::IAdornableCollector>::operator=(rbx_core::SharedPtr<RBX::IAdornableCollector> const&)")]
// 0x5aca74 — __ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEaSERKS3_
// was: boost::shared_ptr<RBX::IAdornableCollector>::operator=(boost::shared_ptr<RBX::IAdornableCollector> const&)
pub fn stub_5aca74() -> ! {
    todo!("0x5aca74 __ZN5boost10shared_ptrIN3RBX19IAdornableCollectorEEaSERKS3_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Joint *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>> const&)")]
// 0x5acaac — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_5acaac() -> ! {
    todo!("0x5acaac __ZN3rbx7signals6signalIFvPN3RBX5JointEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateV,RBX::RotateV>(rbx_core::SharedPtr<RBX::RotateV> const*,RBX::RotateV *)const")]
// 0x5ad770 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateV,RBX::RotateV>(boost::shared_ptr<RBX::RotateV> const*,RBX::RotateV *)const
pub fn stub_5ad770() -> ! {
    todo!("0x5ad770 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotateVES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateP,RBX::RotateP>(rbx_core::SharedPtr<RBX::RotateP> const*,RBX::RotateP *)const")]
// 0x5ae0bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RotateP,RBX::RotateP>(boost::shared_ptr<RBX::RotateP> const*,RBX::RotateP *)const
pub fn stub_5ae0bc() -> ! {
    todo!("0x5ae0bc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7RotatePES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rotate,RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const*,RBX::Rotate *)const")]
// 0x5aea08 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rotate,RBX::Rotate>(boost::shared_ptr<RBX::Rotate> const*,RBX::Rotate *)const
pub fn stub_5aea08() -> ! {
    todo!("0x5aea08 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Glue,RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const*,RBX::Glue *)const")]
// 0x5af354 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Glue,RBX::Glue>(boost::shared_ptr<RBX::Glue> const*,RBX::Glue *)const
pub fn stub_5af354() -> ! {
    todo!("0x5af354 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Snap,RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const*,RBX::Snap *)const")]
// 0x5afca0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Snap,RBX::Snap>(boost::shared_ptr<RBX::Snap> const*,RBX::Snap *)const
pub fn stub_5afca0() -> ! {
    todo!("0x5afca0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")]
// 0x5b0434 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)
pub fn stub_5b0434() -> ! {
    todo!("0x5b0434 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")]
// 0x5b0458 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)
pub fn stub_5b0458() -> ! {
    todo!("0x5b0458 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
// 0x5b0578 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
pub fn stub_5b0578() -> ! {
    todo!("0x5b0578 __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
// 0x5b05a4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
pub fn stub_5b05a4() -> ! {
    todo!("0x5b05a4 __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
// 0x5b0794 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_5b0794() -> ! {
    todo!("0x5b0794 __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
// 0x5b07a8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)
pub fn stub_5b07a8() -> ! {
    todo!("0x5b07a8 __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>::operator()<RBX::Joint *>(RBX::Joint * &)")]
// 0x5b07bc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_5b07bc() -> ! {
    todo!("0x5b07bc __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
// 0x5b0ab8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
pub fn stub_5b0ab8() -> ! {
    todo!("0x5b0ab8 __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
// 0x5b0ae4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
pub fn stub_5b0ae4() -> ! {
    todo!("0x5b0ae4 __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Keyframe,RBX::Keyframe>(rbx_core::SharedPtr<RBX::Keyframe> const*,RBX::Keyframe *)const")]
// 0x5b26d0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Keyframe,RBX::Keyframe>(boost::shared_ptr<RBX::Keyframe> const*,RBX::Keyframe *)const
pub fn stub_5b26d0() -> ! {
    todo!("0x5b26d0 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "RBX::AsyncKeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")]
// 0x5bb1ec — __ZN3RBXL25AsyncKeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEE
// was: RBX::AsyncKeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>)
pub fn stub_5bb1ec() -> ! {
    todo!("0x5bb1ec __ZN3RBXL25AsyncKeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEE")
}

#[doc(alias = "RBX::KeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,bool)")]
// 0x5bb30c — __ZN3RBXL20KeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEEb
// was: RBX::KeyframeLoaderHelper(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>,bool)
pub fn stub_5bb30c() -> ! {
    todo!("0x5bb30c __ZN3RBXL20KeyframeLoaderHelperENS_14AsyncHttpQueue13RequestResultEPSiN5boost8weak_ptrINS_24KeyframeSequenceProviderEEENS4_INS_16KeyframeSequenceEEEb")
}

#[doc(alias = "RBX::CopyKeyframeSequenceData(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")]
// 0x5bb8cc — __ZN3RBXL24CopyKeyframeSequenceDataEN5boost8weak_ptrINS_16KeyframeSequenceEEENS0_10shared_ptrIS2_EE
// was: RBX::CopyKeyframeSequenceData(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>)
pub fn stub_5bb8cc() -> ! {
    todo!("0x5bb8cc __ZN3RBXL24CopyKeyframeSequenceDataEN5boost8weak_ptrINS_16KeyframeSequenceEEENS0_10shared_ptrIS2_EE")
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::operator[](std::string const&)")]
// 0x5bbadc — __ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// was: std::map<std::string,boost::shared_ptr<RBX::KeyframeSequence>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::KeyframeSequence>>>>::operator[](std::string const&)
pub fn stub_5bbadc() -> ! {
    todo!("0x5bbadc __ZNSt3mapISsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::operator=(rbx_core::SharedPtr<RBX::KeyframeSequence> const&)")]
// 0x5bbcf8 — __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_
// was: boost::shared_ptr<RBX::KeyframeSequence>::operator=(boost::shared_ptr<RBX::KeyframeSequence> const&)
pub fn stub_5bbcf8() -> ! {
    todo!("0x5bbcf8 __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEaSERKS3_")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")]
// 0x5bbf14 — __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
// was: RBX::SizeEnforcedLRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::insert(std::string const&,boost::shared_ptr<RBX::KeyframeSequence> const&,unsigned long)
pub fn stub_5bbf14() -> ! {
    todo!("0x5bbf14 __ZN3RBX20SizeEnforcedLRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)")]
// 0x5bbf48 — __ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::KeyframeSequenceProvider> RBX::weak_from<RBX::KeyframeSequenceProvider>(RBX::KeyframeSequenceProvider*)
pub fn stub_5bbf48() -> ! {
    todo!("0x5bbf48 __ZN3RBX9weak_fromINS_24KeyframeSequenceProviderEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>::type> boost::bind<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>>(void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>)")]
// 0x5bc150 — __ZN5boost4bindIvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS1_24KeyframeSequenceProviderEEENS5_INS1_16KeyframeSequenceEEENS_3argILi1EEENSA_ILi2EEES7_S9_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
// was: boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>>::type> boost::bind<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>,boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>>(void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>)
pub fn stub_5bc150() -> ! {
    todo!("0x5bc150 __ZN5boost4bindIvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS1_24KeyframeSequenceProviderEEENS5_INS1_16KeyframeSequenceEEENS_3argILi1EEENSA_ILi2EEES7_S9_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::~LRUCache()")]
// 0x5bcb68 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::~LRUCache()
pub fn stub_5bcb68() -> ! {
    todo!("0x5bcb68 __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEED2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::resize(unsigned long)")]
// 0x5bcc68 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6resizeEm
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::resize(unsigned long)
pub fn stub_5bcc68() -> ! {
    todo!("0x5bcc68 __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6resizeEm")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::insert(std::string const&,rbx_core::SharedPtr<RBX::KeyframeSequence> const&,unsigned long)")]
// 0x5bcca0 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::insert(std::string const&,boost::shared_ptr<RBX::KeyframeSequence> const&,unsigned long)
pub fn stub_5bcca0() -> ! {
    todo!("0x5bcca0 __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6insertERKSsRKS4_m")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
// 0x5bd1c8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> const&)
pub fn stub_5bd1c8() -> ! {
    todo!("0x5bd1c8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERS5_RKT_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> const&)")]
// 0x5bd368 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> const&)
pub fn stub_5bd368() -> ! {
    todo!("0x5bd368 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0x5bd38c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
pub fn stub_5bd38c() -> ! {
    todo!("0x5bd38c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()")]
// 0x5bd3dc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::~node_constructor()
pub fn stub_5bd3dc() -> ! {
    todo!("0x5bd3dc __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0x5bd3f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
pub fn stub_5bd3f8() -> ! {
    todo!("0x5bd3f8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0x5bd520 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const
pub fn stub_5bd520() -> ! {
    todo!("0x5bd520 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0x5bd5b0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)
pub fn stub_5bd5b0() -> ! {
    todo!("0x5bd5b0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x5bd5dc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)
pub fn stub_5bd5dc() -> ! {
    todo!("0x5bd5dc __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISK_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>>>::construct(void)")]
// 0x5bd634 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>>>::construct(void)
pub fn stub_5bd634() -> ! {
    todo!("0x5bd634 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x5bd670 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
pub fn stub_5bd670() -> ! {
    todo!("0x5bd670 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSJ_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEmRKT_RKT0_")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>> const&)")]
// 0x5bd6dc — __ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_
// was: std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>::pair(std::string const&,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>> const&)
pub fn stub_5bd6dc() -> ! {
    todo!("0x5bd6dc __ZNSt4pairISsS_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEC2ERKSsRKS5_")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>> const&)")]
// 0x5bd7a4 — __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_
// was: std::list<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>> const&)
pub fn stub_5bd7a4() -> ! {
    todo!("0x5bd7a4 __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE14_M_create_nodeERKS7_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>> *)")]
// 0x5bd8b8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>> *)
pub fn stub_5bd8b8() -> ! {
    todo!("0x5bd8b8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x5bd914 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_5bd914() -> ! {
    todo!("0x5bd914 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0x5bd940 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)
pub fn stub_5bd940() -> ! {
    todo!("0x5bd940 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>*)")]
// 0x5bd980 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>::destroy(std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>*)
pub fn stub_5bd980() -> ! {
    todo!("0x5bd980 __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEEE7destroyEPS8_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x5bda24 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
pub fn stub_5bda24() -> ! {
    todo!("0x5bda24 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "RBX::LRUCache<std::string,rbx_core::SharedPtr<RBX::KeyframeSequence>>::remove(std::string const&)")]
// 0x5bda64 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs
// was: RBX::LRUCache<std::string,boost::shared_ptr<RBX::KeyframeSequence>>::remove(std::string const&)
pub fn stub_5bda64() -> ! {
    todo!("0x5bda64 __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_16KeyframeSequenceEEEE6removeERKSs")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::_M_clear(void)")]
// 0x5bdab8 — __ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv
// was: std::_List_base<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>::_M_clear(void)
pub fn stub_5bdab8() -> ! {
    todo!("0x5bdab8 __ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEESaIS7_EE8_M_clearEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x5bdae0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
pub fn stub_5bdae0() -> ! {
    todo!("0x5bdae0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// 0x5bdb18 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)
pub fn stub_5bdb18() -> ! {
    todo!("0x5bdb18 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX16KeyframeSequenceEEEEEEEESsSD_NS_4hashISsEESt8equal_toISsEEEE5clearEv")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// 0x5bdc30 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
pub fn stub_5bdc30() -> ! {
    todo!("0x5bdc30 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSD_INS1_16KeyframeSequenceEEEENSB_5list4INS_3argILi1EEENSL_ILi2EEENSB_5valueISF_EENSO_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0x5bdd90 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
pub fn stub_5bdd90() -> ! {
    todo!("0x5bdd90 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>)")]
// 0x5bdef8 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEEvT_
// was: void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>)
pub fn stub_5bdef8() -> ! {
    todo!("0x5bdef8 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvS3_S4_NS_8weak_ptrINS1_24KeyframeSequenceProviderEEENSC_INS1_16KeyframeSequenceEEEENSA_5list4INS_3argILi1EEENSK_ILi2EEENSA_5valueISE_EENSN_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>,rbx_core::WeakPtr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x5be070 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::weak_ptr<RBX::KeyframeSequenceProvider>,boost::weak_ptr<RBX::KeyframeSequence>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequenceProvider>>,boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_5be070() -> ! {
    todo!("0x5be070 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_8weak_ptrINS5_24KeyframeSequenceProviderEEENS9_INS5_16KeyframeSequenceEEEENS3_5list4INS_3argILi1EEENSH_ILi2EEENS3_5valueISB_EENSK_ISD_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}
