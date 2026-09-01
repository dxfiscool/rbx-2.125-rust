//! rendering shard 328 — 100 stubs 0x550fa4..0x5a464c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 83280->83380 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35440 before -> 35540 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x550fa4 (lowest remaining 0x550fa4..0x5a464c, next lowest 0x5a47ac)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x550fa4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_550fa4() -> ! {
    todo!("0x550fa4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x55204c — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::EventDesc(rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_55204c() -> ! {
    todo!("0x55204c RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::EventDesc(rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x55223c — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev
pub fn stub_55223c() -> ! {
    todo!("0x55223c RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::~EventDesc()")
}

// 0x5522f0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_5522f0() -> ! {
    todo!("0x5522f0 RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x552444 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_552444() -> ! {
    todo!("0x552444 RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x552680 — __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_552680() -> ! {
    todo!("0x552680 RBX::Reflection::EventDescBase<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x55280c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
pub fn stub_55280c() -> ! {
    todo!("0x55280c boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x552928 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2ISsSsEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<std::string,std::string>(std::string const&,std::string const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute2ISsSsEEvRKT_RKT0_
pub fn stub_552928() -> ! {
    todo!("0x552928 void RBX::Reflection::GenericSlotWrapper::execute2<std::string,std::string>(std::string const&,std::string const&)")
}

// 0x552c88 — __ZN5boost9function2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: __ZN5boost9function2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_
pub fn stub_552c88() -> ! {
    todo!("0x552c88 void boost::function2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x552d80 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_552d80() -> ! {
    todo!("0x552d80 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x552d9c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEvSsSsE6invokeERNS1_15function_bufferESsSs
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,std::string>::invoke(boost::detail::function::function_buffer &,std::string,std::string)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEvSsSsE6invokeERNS1_15function_bufferESsSs
pub fn stub_552d9c() -> ! {
    todo!("0x552d9c boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,std::string>::invoke(boost::detail::function::function_buffer &,std::string,std::string)")
}

// 0x552da4 — __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_552da4() -> ! {
    todo!("0x552da4 bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x552e8c — __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_552e8c() -> ! {
    todo!("0x552e8c bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x552f70 — __ZNK5boost6detail8function13basic_vtable2IvSsSsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvSsSsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_552f70() -> ! {
    todo!("0x552f70 void boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x553044 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsS8_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSF_ILi2EEEEEEclISsSsEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<std::string,std::string>(std::string &,std::string &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsS8_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSF_ILi2EEEEEEclISsSsEEvRT_RT0_
pub fn stub_553044() -> ! {
    todo!("0x553044 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<std::string,std::string>(std::string &,std::string &)")
}

// 0x553060 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_553060() -> ! {
    todo!("0x553060 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x554210 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,int>(char const*,char const*,bool (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_554210() -> ! {
    todo!("0x554210 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,int>(char const*,char const*,bool (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x55431c — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
pub fn stub_55431c() -> ! {
    todo!("0x55431c RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isReadOnly(void)const")
}

// 0x554320 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
pub fn stub_554320() -> ! {
    todo!("0x554320 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::isWriteOnly(void)const")
}

// 0x554324 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_554324() -> ! {
    todo!("0x554324 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x554348 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_554348() -> ! {
    todo!("0x554348 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetImpl<bool (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x554468 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::PropDescriptor<double (RBX::GuiService::*)(void)const,int>(char const*,char const*,double (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_554468() -> ! {
    todo!("0x554468 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::PropDescriptor<double (RBX::GuiService::*)(void)const,int>(char const*,char const*,double (RBX::GuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x554574 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED0Ev
pub fn stub_554574() -> ! {
    todo!("0x554574 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor()")
}

// 0x5545a0 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE10isReadOnlyEv
pub fn stub_5545a0() -> ! {
    todo!("0x5545a0 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isReadOnly(void)const")
}

// 0x5545a4 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE11isWriteOnlyEv
pub fn stub_5545a4() -> ! {
    todo!("0x5545a4 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::isWriteOnly(void)const")
}

// 0x5545a8 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_5545a8() -> ! {
    todo!("0x5545a8 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5545c8 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEdE7GetImplIMS2_KFdvEE8setValueEPNS0_13DescribedBaseERKd
pub fn stub_5545c8() -> ! {
    todo!("0x5545c8 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::GetImpl<double (RBX::GuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x5546e8 — __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple>(void)")]
// was: __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEEEN5boost10shared_ptrIT_EEv
pub fn stub_5546e8() -> ! {
    todo!("0x5546e8 boost::shared_ptr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple>(void)")
}

// 0x554994 — __ZNSt8auto_ptrIN3RBX10Reflection5TupleEED2Ev
#[doc(alias = "std::auto_ptr<RBX::Reflection::Tuple>::~auto_ptr()")]
// was: __ZNSt8auto_ptrIN3RBX10Reflection5TupleEED2Ev
pub fn stub_554994() -> ! {
    todo!("0x554994 std::auto_ptr<RBX::Reflection::Tuple>::~auto_ptr()")
}

// 0x559474 — __ZN3RBX6Rocket13onGoalChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Rocket::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX6Rocket13onGoalChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_559474() -> ! {
    todo!("0x559474 RBX::Rocket::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x55947c — __ZN3RBX10Reflection13BoundFuncDescINS_6RocketEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Rocket,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_6RocketEFvvELi0EED1Ev
pub fn stub_55947c() -> ! {
    todo!("0x55947c RBX::Reflection::BoundFuncDesc<RBX::Rocket,void ()(void),0>::~BoundFuncDesc()")
}

// 0x5595a4 — __ZN3RBX12BodyPosition13onGoalChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::BodyPosition::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX12BodyPosition13onGoalChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_5595a4() -> ! {
    todo!("0x5595a4 RBX::BodyPosition::onGoalChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x57da48 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEEC2IMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEEC2IMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_57da48() -> ! {
    todo!("0x57da48 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57db5c — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED0Ev
pub fn stub_57db5c() -> ! {
    todo!("0x57db5c RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")
}

// 0x57db88 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_57db88() -> ! {
    todo!("0x57db88 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isReadOnly(void)const")
}

// 0x57db8c — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_57db8c() -> ! {
    todo!("0x57db8c RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::isWriteOnly(void)const")
}

// 0x57db90 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_57db90() -> ! {
    todo!("0x57db90 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x57dbc8 — __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_57dbc8() -> ! {
    todo!("0x57dbc8 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x57e80c — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEED1Ev
pub fn stub_57e80c() -> ! {
    todo!("0x57e80c RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")
}

// 0x57eefc — __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_57eefc() -> ! {
    todo!("0x57eefc __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x57ef84 — __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator6createEv
pub fn stub_57ef84() -> ! {
    todo!("0x57ef84 __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7Creator6createEv")
}

// 0x57f0c8 — __ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv
pub fn stub_57f0c8() -> ! {
    todo!("0x57f0c8 __ZN3RBX4Name13callDoDeclareILZNS_11sImageLabelEEEEvv")
}

// 0x57f0cc — __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v
pub fn stub_57f0cc() -> ! {
    todo!("0x57f0cc __ZN3RBX4Name9doDeclareILZNS_11sImageLabelEEEERKS0_v")
}

// 0x57f1ac — __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorC2Ev
pub fn stub_57f1ac() -> ! {
    todo!("0x57f1ac __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorC2Ev")
}

// 0x57f3f0 — __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE17static_getCreatorEv
pub fn stub_57f3f0() -> ! {
    todo!("0x57f3f0 __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE17static_getCreatorEv")
}

// 0x57f464 — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEEC2IMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEEC2IMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_57f464() -> ! {
    todo!("0x57f464 RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57f578 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_57f578() -> ! {
    todo!("0x57f578 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57f69c — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEED0Ev
pub fn stub_57f69c() -> ! {
    todo!("0x57f69c RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::~PropDescriptor()")
}

// 0x57f6c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10isReadOnlyEv
pub fn stub_57f6c8() -> ! {
    todo!("0x57f6c8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isReadOnly(void)const")
}

// 0x57f6d8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11isWriteOnlyEv
pub fn stub_57f6d8() -> ! {
    todo!("0x57f6d8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::isWriteOnly(void)const")
}

// 0x57f6e8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11equalValuesEPKNS0_13DescribedBaseES6_
pub fn stub_57f6e8() -> ! {
    todo!("0x57f6e8 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x57f894 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_57f894() -> ! {
    todo!("0x57f894 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x57f9c0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_57f9c0() -> ! {
    todo!("0x57f9c0 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x57fbbc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9copyValueEPKNS0_13DescribedBaseEPS4_
pub fn stub_57fbbc() -> ! {
    todo!("0x57fbbc RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x57fec0 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEED1Ev
pub fn stub_57fec0() -> ! {
    todo!("0x57fec0 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::~TypedPropertyDescriptor()")
}

// 0x57fee4 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEED0Ev
pub fn stub_57fee4() -> ! {
    todo!("0x57fee4 RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::~TypedPropertyDescriptor()")
}

// 0x57ff10 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_57ff10() -> ! {
    todo!("0x57ff10 RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::isReadOnly(void)const")
}

// 0x57ff14 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_57ff14() -> ! {
    todo!("0x57ff14 RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::isWriteOnly(void)const")
}

// 0x57ff18 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_57ff18() -> ! {
    todo!("0x57ff18 RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x57ff50 — __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10ImageLabelENS_9TextureIdEE10GetSetImplIMNS_13GuiImageMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_57ff50() -> ! {
    todo!("0x57ff50 RBX::Reflection::PropDescriptor<RBX::ImageLabel,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")
}

// 0x580490 — __GLOBAL__I_a_216
#[doc(alias = "global constructor keyed to_a_216")]
// was: __GLOBAL__I_a_216
pub fn stub_580490() -> ! {
    todo!("0x580490 global constructor keyed to_a_216")
}

// 0x5809a0 — __ZN3RBX13InsertService13getFreeDecalsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
#[doc(alias = "RBX::InsertService::getFreeDecals(std::string,int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService13getFreeDecalsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
pub fn stub_5809a0() -> ! {
    todo!("0x5809a0 RBX::InsertService::getFreeDecals(std::string,int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x580bf4 — __ZN3RBX13InsertService11getBaseSetsEN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
#[doc(alias = "RBX::InsertService::getBaseSets(boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService11getBaseSetsEN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
pub fn stub_580bf4() -> ! {
    todo!("0x580bf4 RBX::InsertService::getBaseSets(boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x580db4 — __ZN3RBX13InsertService11getUserSetsEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
#[doc(alias = "RBX::InsertService::getUserSets(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService11getUserSetsEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
pub fn stub_580db4() -> ! {
    todo!("0x580db4 RBX::InsertService::getUserSets(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x581004 — __ZN3RBX13InsertService13getCollectionEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
#[doc(alias = "RBX::InsertService::getCollection(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService13getCollectionEiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
pub fn stub_581004() -> ! {
    todo!("0x581004 RBX::InsertService::getCollection(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x581ac0 — __ZN3RBX13InsertService15dispatchRequestERKSsN5boost8functionIFvNS3_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS8_EEEEEEENS4_IFvSsEEE
#[doc(alias = "RBX::InsertService::dispatchRequest(std::string const&,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService15dispatchRequestERKSsN5boost8functionIFvNS3_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS8_EEEEEEENS4_IFvSsEEE
pub fn stub_581ac0() -> ! {
    todo!("0x581ac0 RBX::InsertService::dispatchRequest(std::string const&,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x584fa4 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS5_EEEEEEvNS2_8functionIFvS5_EEET_
#[doc(alias = "void RBX::Reflection::resume_adapter<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
// was: __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS5_EEEEEEvNS2_8functionIFvS5_EEET_
pub fn stub_584fa4() -> ! {
    todo!("0x584fa4 void RBX::Reflection::resume_adapter<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")
}

// 0x585b6c — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvSsELi1EED1Ev
pub fn stub_585b6c() -> ! {
    todo!("0x585b6c RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x585bac — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvfELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(float),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvfELi1EED1Ev
pub fn stub_585bac() -> ! {
    todo!("0x585bac RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(float),1>::~BoundFuncDesc()")
}

// 0x585bec — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EED1Ev
pub fn stub_585bec() -> ! {
    todo!("0x585bec RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::~BoundFuncDesc()")
}

// 0x585cd8 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EED1Ev
pub fn stub_585cd8() -> ! {
    todo!("0x585cd8 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::~BoundFuncDesc()")
}

// 0x587248 — __ZNK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv
pub fn stub_587248() -> ! {
    todo!("0x587248 __ZNK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv")
}

// 0x587304 — __ZThn32_NK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv
pub fn stub_587304() -> ! {
    todo!("0x587304 __ZThn32_NK3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E12getClassNameEv")
}

// 0x59e530 — __GLOBAL__I_a_217
#[doc(alias = "global constructor keyed to_a_217")]
// was: __GLOBAL__I_a_217
pub fn stub_59e530() -> ! {
    todo!("0x59e530 global constructor keyed to_a_217")
}

// 0x5a3394 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv
pub fn stub_5a3394() -> ! {
    todo!("0x5a3394 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv")
}

// 0x5a33bc — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv
pub fn stub_5a33bc() -> ! {
    todo!("0x5a33bc __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEE12getClassNameEv")
}

// 0x5a3488 — __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3488() -> ! {
    todo!("0x5a3488 __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3544 — __ZThn32_NK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3544() -> ! {
    todo!("0x5a3544 __ZThn32_NK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE12getClassNameEv")
}

// 0x5a36a4 — __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv
pub fn stub_5a36a4() -> ! {
    todo!("0x5a36a4 __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3760 — __ZThn32_NK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3760() -> ! {
    todo!("0x5a3760 __ZThn32_NK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE12getClassNameEv")
}

// 0x5a38c0 — __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv
pub fn stub_5a38c0() -> ! {
    todo!("0x5a38c0 __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv")
}

// 0x5a397c — __ZThn32_NK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv
pub fn stub_5a397c() -> ! {
    todo!("0x5a397c __ZThn32_NK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3adc — __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3adc() -> ! {
    todo!("0x5a3adc __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3b9c — __ZThn32_NK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3b9c() -> ! {
    todo!("0x5a3b9c __ZThn32_NK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3d00 — __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3d00() -> ! {
    todo!("0x5a3d00 __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3dc0 — __ZThn32_NK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3dc0() -> ! {
    todo!("0x5a3dc0 __ZThn32_NK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3f24 — __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3f24() -> ! {
    todo!("0x5a3f24 __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv")
}

// 0x5a3fe0 — __ZThn32_NK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv
pub fn stub_5a3fe0() -> ! {
    todo!("0x5a3fe0 __ZThn32_NK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE12getClassNameEv")
}

// 0x5a409c — __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5a409c() -> ! {
    todo!("0x5a409c __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5a40a0 — __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5a40a0() -> ! {
    todo!("0x5a40a0 __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5a40a4 — __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5a40a4() -> ! {
    todo!("0x5a40a4 __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5a40a8 — __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5a40a8() -> ! {
    todo!("0x5a40a8 __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5a40ac — __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5a40ac() -> ! {
    todo!("0x5a40ac __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5a40b0 — __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD1Ev
pub fn stub_5a40b0() -> ! {
    todo!("0x5a40b0 __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD1Ev")
}

// 0x5a4158 — __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv
pub fn stub_5a4158() -> ! {
    todo!("0x5a4158 __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv")
}

// 0x5a4214 — __ZThn32_NK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv
pub fn stub_5a4214() -> ! {
    todo!("0x5a4214 __ZThn32_NK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE12getClassNameEv")
}

// 0x5a4374 — __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv
pub fn stub_5a4374() -> ! {
    todo!("0x5a4374 __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv")
}

// 0x5a4430 — __ZThn32_NK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv
pub fn stub_5a4430() -> ! {
    todo!("0x5a4430 __ZThn32_NK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE12getClassNameEv")
}

// 0x5a4590 — __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv
pub fn stub_5a4590() -> ! {
    todo!("0x5a4590 __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv")
}

// 0x5a464c — __ZThn32_NK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv
pub fn stub_5a464c() -> ! {
    todo!("0x5a464c __ZThn32_NK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE12getClassNameEv")
}
