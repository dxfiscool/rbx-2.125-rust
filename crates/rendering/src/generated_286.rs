//! rendering shard 286 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15586/15586 complete, 31140->31240 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31140 before -> 31240 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 100 after 0x3eecec

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3eedd4 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEEvT_
pub fn stub_3eedd4() -> ! {
    todo!("0x3eedd4 void boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x3eeecc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsRKNS7_11ChatService9ChatColorEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsRKNS7_11ChatService9ChatColorEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
pub fn stub_3eeecc() -> ! {
    todo!("0x3eeecc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3eeee8 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsRKNS7_11ChatService9ChatColorEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEvSC_SsSI_E6invokeERNS1_15function_bufferESC_SsSI_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsRKNS7_11ChatService9ChatColorEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEvSC_SsSI_E6invokeERNS1_15function_bufferESC_SsSI_
pub fn stub_3eeee8() -> ! {
    todo!("0x3eeee8 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")
}

// 0x3eef0c — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsRKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsRKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_3eef0c() -> ! {
    todo!("0x3eef0c bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x3eeff4 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsRKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsRKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_3eeff4() -> ! {
    todo!("0x3eeff4 bool boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x3ef0d8 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsRKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsRKS8_EENSB_5list4INSB_5valueINS3_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_3ef0d8() -> ! {
    todo!("0x3ef0d8 void boost::detail::function::basic_vtable3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x3ef1ac — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsRKNS4_11ChatService9ChatColorEEENS0_5list3IRSI_RSsRSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int *)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsRKNS4_11ChatService9ChatColorEEENS0_5list3IRSI_RSsRSO_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3ef1ac() -> ! {
    todo!("0x3ef1ac void boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&> &,int)")
}

// 0x3ef1d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsRKNS7_11ChatService9ChatColorEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsRKNS7_11ChatService9ChatColorEEENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_3ef1d4() -> ! {
    todo!("0x3ef1d4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x3ef32c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE7connectINS2_8functionIS9_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE7connectINS2_8functionIS9_EEEENS0_10connectionERKT_
pub fn stub_3ef32c() -> ! {
    todo!("0x3ef32c rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> const&)")
}

// 0x3ef420 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_EC2IPSB_EERKSE_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_EC2IPSB_EERKSE_T_
pub fn stub_3ef420() -> ! {
    todo!("0x3ef420 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>*)")
}

// 0x3ef51c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_8functionIS9_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_8functionIS9_EEED1Ev
pub fn stub_3ef51c() -> ! {
    todo!("0x3ef51c rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~callable_slot()")
}

// 0x3ef62c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_8functionIS9_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_8functionIS9_EEED0Ev
pub fn stub_3ef62c() -> ! {
    todo!("0x3ef62c rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~callable_slot()")
}

// 0x3ef75c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_E4callES7_SsS9_
// type: void __fastcall(int, const shared_count *, const std::string *, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_E4callES7_SsS9_
pub fn stub_3ef75c() -> ! {
    todo!("0x3ef75c rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")
}

// 0x3ef8c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_E4callES7_SsS9_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_E4callES7_SsS9_
pub fn stub_3ef8c8() -> ! {
    todo!("0x3ef8c8 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")
}

// 0x3ef8d0 — __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEclES4_SsS6_
// type: void __fastcall(_DWORD *, const shared_count *, const std::string *, int)
#[doc(alias = "boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::operator()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)const")]
// was: __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEclES4_SsS6_
pub fn stub_3ef8d0() -> ! {
    todo!("0x3ef8d0 boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::operator()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)const")
}

// 0x3efa78 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_ED1Ev
pub fn stub_3efa78() -> ! {
    todo!("0x3efa78 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")
}

// 0x3efb88 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_8functionISA_EELi3ESA_ED0Ev
pub fn stub_3efb88() -> ! {
    todo!("0x3efb88 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")
}

// 0x3efcb8 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEE13assign_to_ownERKS7_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to_own(boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor> const&)")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEE13assign_to_ownERKS7_
pub fn stub_3efcb8() -> ! {
    todo!("0x3efcb8 boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to_own(boost::function3<void,boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor> const&)")
}

// 0x3efce8 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3efce8() -> ! {
    todo!("0x3efce8 RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3eff44 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev
pub fn stub_3eff44() -> ! {
    todo!("0x3eff44 RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")
}

// 0x3eff68 — __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev
pub fn stub_3eff68() -> ! {
    todo!("0x3eff68 RBX::Reflection::EventDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")
}

// 0x3f001c — __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EEC2EMS2_FvS6_SsS7_EPKcSD_SD_SD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::BoundFuncDesc(void (RBX::ChatService::*)(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),char const*,char const*,char const*,char const*,RBX::ChatService::ChatColor,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EEC2EMS2_FvS6_SsS7_EPKcSD_SD_SD_S7_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3f001c() -> ! {
    todo!("0x3f001c RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::BoundFuncDesc(void (RBX::ChatService::*)(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),char const*,char const*,char const*,char const*,RBX::ChatService::ChatColor,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3f0290 — __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_
pub fn stub_3f0290() -> ! {
    todo!("0x3f0290 RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x3f02f8 — __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EED0Ev
pub fn stub_3f02f8() -> ! {
    todo!("0x3f02f8 RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::~BoundFuncDesc()")
}

// 0x3f042c — __ZNK3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3f042c() -> ! {
    todo!("0x3f042c RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x3f05d0 — __ZN3RBX10Reflection11Call3HelperINS_11ChatServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEES6_SsS7_vE4callEPS2_S9_RNS0_7VariantERKS6_RKSsRKS7_
// type: void __fastcall(int, char *, int, int, const shared_count *, const std::string *, _DWORD *)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::ChatService,void (RBX::ChatService::*)(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor,void>::call(RBX::ChatService*,void (RBX::ChatService::*)(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&)")]
// was: __ZN3RBX10Reflection11Call3HelperINS_11ChatServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEES6_SsS7_vE4callEPS2_S9_RNS0_7VariantERKS6_RKSsRKS7_
pub fn stub_3f05d0() -> ! {
    todo!("0x3f05d0 RBX::Reflection::Call3Helper<RBX::ChatService,void (RBX::ChatService::*)(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor,void>::call(RBX::ChatService*,void (RBX::ChatService::*)(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&)")
}

// 0x3f0760 — __ZN3RBX10Reflection9ArgHelper6getArgINS_11ChatService9ChatColorELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::ChatService::ChatColor RBX::Reflection::ArgHelper::getArg<RBX::ChatService::ChatColor,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::ChatService::ChatColor> const&,boost::disable_if<boost::is_same<RBX::ChatService::ChatColor,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_11ChatService9ChatColorELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_3f0760() -> ! {
    todo!("0x3f0760 RBX::ChatService::ChatColor RBX::Reflection::ArgHelper::getArg<RBX::ChatService::ChatColor,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::ChatService::ChatColor> const&,boost::disable_if<boost::is_same<RBX::ChatService::ChatColor,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x3f08f4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11ChatService9ChatColorEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int __fastcall(int, _DWORD *, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::ChatService::ChatColor>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::ChatService::ChatColor &,boost::enable_if<boost::is_enum<RBX::ChatService::ChatColor>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11ChatService9ChatColorEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_3f08f4() -> ! {
    todo!("0x3f08f4 bool RBX::Reflection::ArgHelper::try_enum<3,RBX::ChatService::ChatColor>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::ChatService::ChatColor &,boost::enable_if<boost::is_enum<RBX::ChatService::ChatColor>,void>::type *)")
}

// 0x3f0948 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEED2Ev
pub fn stub_3f0948() -> ! {
    todo!("0x3f0948 rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~remote_signal()")
}

// 0x3f0a94 — __GLOBAL__I_a_170
// type: 
#[doc(alias = "global constructor keyed to_a_170")]
// was: __GLOBAL__I_a_170
pub fn stub_3f0a94() -> ! {
    todo!("0x3f0a94 global constructor keyed to_a_170")
}

// 0x3f0e0c — __ZN3RBX13ClickDetectorC1Ev
// type: int __fastcall(RBX::ClickDetector *this)
#[doc(alias = "RBX::ClickDetector::ClickDetector(void)")]
// was: __ZN3RBX13ClickDetectorC1Ev
pub fn stub_3f0e0c() -> ! {
    todo!("0x3f0e0c RBX::ClickDetector::ClickDetector(void)")
}

// 0x3f0e10 — __ZN3RBX13ClickDetectorC2Ev
// type: RBX::Instance *__fastcall(RBX::ClickDetector *this)
#[doc(alias = "RBX::ClickDetector::ClickDetector(void)")]
// was: __ZN3RBX13ClickDetectorC2Ev
pub fn stub_3f0e10() -> ! {
    todo!("0x3f0e10 RBX::ClickDetector::ClickDetector(void)")
}

// 0x3f1114 — __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, float, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector14fireMouseClickEfPNS_7Network6PlayerE
pub fn stub_3f1114() -> ! {
    todo!("0x3f1114 RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")
}

// 0x3f1234 — __ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE
// type: int __fastcall(int *, float, int, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isClickable(boost::shared_ptr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector11isClickableEN5boost10shared_ptrINS_12PartInstanceEEEfbPNS_7Network6PlayerE
pub fn stub_3f1234() -> ! {
    todo!("0x3f1234 RBX::ClickDetector::isClickable(boost::shared_ptr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")
}

// 0x3f12e0 — __ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *, int *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::updateLastHoverPart(boost::shared_ptr<RBX::Instance>,RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector19updateLastHoverPartEN5boost10shared_ptrINS_8InstanceEEEPNS_7Network6PlayerE
pub fn stub_3f12e0() -> ! {
    todo!("0x3f12e0 RBX::ClickDetector::updateLastHoverPart(boost::shared_ptr<RBX::Instance>,RBX::Network::Player *)")
}

// 0x3f130c — __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector14fireMouseHoverEPNS_7Network6PlayerE
pub fn stub_3f130c() -> ! {
    todo!("0x3f130c RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")
}

// 0x3f1410 — __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
// type: void __fastcall(RBX::ClickDetector *this, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector19fireMouseHoverLeaveEPNS_7Network6PlayerE
pub fn stub_3f1410() -> ! {
    todo!("0x3f1410 RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")
}

// 0x3f154c — __ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE
// type: void __fastcall(int *, RBX::Network::Player *, int, int)
#[doc(alias = "RBX::ClickDetector::stopHover(boost::shared_ptr<RBX::PartInstance>,RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector9stopHoverEN5boost10shared_ptrINS_12PartInstanceEEEPNS_7Network6PlayerE
pub fn stub_3f154c() -> ! {
    todo!("0x3f154c RBX::ClickDetector::stopHover(boost::shared_ptr<RBX::PartInstance>,RBX::Network::Player *)")
}

// 0x3f15b8 — __ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE
// type: int __fastcall(RBX::ClickDetector *this, RBX::PartInstance *, float, RBX::Network::Player *, RBX::Network::Player *)
#[doc(alias = "RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")]
// was: __ZN3RBX13ClickDetector9isHoveredEPNS_12PartInstanceEfbPNS_7Network6PlayerE
pub fn stub_3f15b8() -> ! {
    todo!("0x3f15b8 RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")
}

// 0x3f17f8 — __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED1Ev
pub fn stub_3f17f8() -> ! {
    todo!("0x3f17f8 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x3f181c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12PartInstanceEEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PartInstance>(boost::shared_ptr<RBX::PartInstance> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12PartInstanceEEERS3_RKNS0_IT_EE
pub fn stub_3f181c() -> ! {
    todo!("0x3f181c boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PartInstance>(boost::shared_ptr<RBX::PartInstance> const&)")
}

// 0x3f1850 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_
// type: void __fastcall(int, int, const shared_count *, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::ClickDetector*,boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE21fireAndReplicateEventEPS2_S6_
pub fn stub_3f1850() -> ! {
    todo!("0x3f1850 RBX::Reflection::RemoteEventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::ClickDetector*,boost::shared_ptr<RBX::Instance>)")
}

// 0x3f1984 — __ZN3RBX11shared_fromINS_13ClickDetectorEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::ClickDetector> RBX::shared_from<RBX::ClickDetector>(RBX::ClickDetector*)")]
// was: __ZN3RBX11shared_fromINS_13ClickDetectorEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_3f1984() -> ! {
    todo!("0x3f1984 boost::shared_ptr<RBX::ClickDetector> RBX::shared_from<RBX::ClickDetector>(RBX::ClickDetector*)")
}

// 0x3f1af4 — __ZN3RBX13ClickDetectorD1Ev
// type: void __fastcall(RBX::ClickDetector *__hidden this)
#[doc(alias = "RBX::ClickDetector::~ClickDetector()")]
// was: __ZN3RBX13ClickDetectorD1Ev
pub fn stub_3f1af4() -> ! {
    todo!("0x3f1af4 RBX::ClickDetector::~ClickDetector()")
}

// 0x3f1af8 — __ZN3RBX13ClickDetectorD0Ev
// type: void __fastcall(RBX::ClickDetector *__hidden this)
#[doc(alias = "RBX::ClickDetector::~ClickDetector()")]
// was: __ZN3RBX13ClickDetectorD0Ev
pub fn stub_3f1af8() -> ! {
    todo!("0x3f1af8 RBX::ClickDetector::~ClickDetector()")
}

// 0x3f1b98 — __ZNK3RBX13ClickDetector11askAddChildEPKNS_8InstanceE
// type: int __fastcall(RBX::ClickDetector *this, const RBX::Instance *)
#[doc(alias = "RBX::ClickDetector::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX13ClickDetector11askAddChildEPKNS_8InstanceE
pub fn stub_3f1b98() -> ! {
    todo!("0x3f1b98 RBX::ClickDetector::askAddChild(RBX::Instance const*)const")
}

// 0x3f1b9c — __ZNK3RBX13ClickDetector12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ClickDetector *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::ClickDetector::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX13ClickDetector12askSetParentEPKNS_8InstanceE
pub fn stub_3f1b9c() -> ! {
    todo!("0x3f1b9c RBX::ClickDetector::askSetParent(RBX::Instance const*)const")
}

// 0x3f1bf0 — __ZNK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv
pub fn stub_3f1bf0() -> ! {
    todo!("0x3f1bf0 __ZNK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv")
}

// 0x3f1c04 — __ZThn32_N3RBX13ClickDetectorD1Ev
// type: void __fastcall(RBX::ClickDetector *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ClickDetector::~ClickDetector()")]
// was: __ZThn32_N3RBX13ClickDetectorD1Ev
pub fn stub_3f1c04() -> ! {
    todo!("0x3f1c04 non-virtual thunk toRBX::ClickDetector::~ClickDetector()")
}

// 0x3f1c0c — __ZThn32_N3RBX13ClickDetectorD0Ev
// type: void __fastcall(RBX::ClickDetector *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ClickDetector::~ClickDetector()")]
// was: __ZThn32_N3RBX13ClickDetectorD0Ev
pub fn stub_3f1c0c() -> ! {
    todo!("0x3f1c0c non-virtual thunk toRBX::ClickDetector::~ClickDetector()")
}

// 0x3f1c14 — __ZThn32_NK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv
pub fn stub_3f1c14() -> ! {
    todo!("0x3f1c14 __ZThn32_NK3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E12getClassNameEv")
}

// 0x3f1c24 — __ZThn36_N3RBX13ClickDetectorD1Ev
// type: void __fastcall(RBX::ClickDetector *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ClickDetector::~ClickDetector()")]
// was: __ZThn36_N3RBX13ClickDetectorD1Ev
pub fn stub_3f1c24() -> ! {
    todo!("0x3f1c24 non-virtual thunk toRBX::ClickDetector::~ClickDetector()")
}

// 0x3f1c2c — __ZThn36_N3RBX13ClickDetectorD0Ev
// type: void __fastcall(RBX::ClickDetector *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ClickDetector::~ClickDetector()")]
// was: __ZThn36_N3RBX13ClickDetectorD0Ev
pub fn stub_3f1c2c() -> ! {
    todo!("0x3f1c2c non-virtual thunk toRBX::ClickDetector::~ClickDetector()")
}

// 0x3f1c38 — __ZN3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E17static_getCreatorEv
pub fn stub_3f1c38() -> ! {
    todo!("0x3f1c38 __ZN3RBX14FactoryProductINS_13ClickDetectorENS_8InstanceELZNS_14sClickDetectorEES2_E17static_getCreatorEv")
}

// 0x3f1cac — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_13ClickDetectorEEEPKT_v
// type: _UNKNOWN **__fastcall(int)
#[doc(alias = "RBX::ClickDetector const* RBX::Instance::findConstFirstChildOfType<RBX::ClickDetector>(void)const")]
// was: __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_13ClickDetectorEEEPKT_v
pub fn stub_3f1cac() -> ! {
    todo!("0x3f1cac RBX::ClickDetector const* RBX::Instance::findConstFirstChildOfType<RBX::ClickDetector>(void)const")
}

// 0x3f1d14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_
// type: void __fastcall(int, int, const shared_count *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::ClickDetector*,boost::shared_ptr<RBX::Instance>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPS2_S6_
pub fn stub_3f1d14() -> ! {
    todo!("0x3f1d14 RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::ClickDetector*,boost::shared_ptr<RBX::Instance>)const")
}

// 0x3f1de8 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE14replicateEventEPNS0_11EventSourceES6_
pub fn stub_3f1de8() -> ! {
    todo!("0x3f1de8 RBX::Reflection::RemoteEventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Instance>)")
}

// 0x3f1f34 — __ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3f1f34() -> ! {
    todo!("0x3f1f34 __ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3f1f38 — __ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3f1f38() -> ! {
    todo!("0x3f1f38 __ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3f1fd8 — __ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3f1fd8() -> ! {
    todo!("0x3f1fd8 __ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3f1fe0 — __ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3f1fe0() -> ! {
    todo!("0x3f1fe0 __ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3f2084 — __ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3f2084() -> ! {
    todo!("0x3f2084 __ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3f208c — __ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3f208c() -> ! {
    todo!("0x3f208c __ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3f2130 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_13ClickDetectorEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ClickDetector>(char const*,char const*,float RBX::ClickDetector::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_13ClickDetectorEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3f2130() -> ! {
    todo!("0x3f2130 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ClickDetector>(char const*,char const*,float RBX::ClickDetector::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3f22c0 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE10isReadOnlyEv
pub fn stub_3f22c0() -> ! {
    todo!("0x3f22c0 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::isReadOnly(void)const")
}

// 0x3f22c4 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE11isWriteOnlyEv
pub fn stub_3f22c4() -> ! {
    todo!("0x3f22c4 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::isWriteOnly(void)const")
}

// 0x3f22c8 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_3f22c8() -> ! {
    todo!("0x3f22c8 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3f22d4 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE8setValueEPNS0_13DescribedBaseERKf
// type: float *__fastcall(int, int, float *)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ClickDetectorEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_3f22d4() -> ! {
    todo!("0x3f22d4 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x3f2330 — __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEED0Ev
pub fn stub_3f2330() -> ! {
    todo!("0x3f2330 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::~RemoteEventDesc()")
}

// 0x3f23e4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
pub fn stub_3f23e4() -> ! {
    todo!("0x3f23e4 RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x3f2548 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE12isScriptableEv
pub fn stub_3f2548() -> ! {
    todo!("0x3f2548 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::isScriptable(void)const")
}

// 0x3f2550 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE11isBroadcastEv
pub fn stub_3f2550() -> ! {
    todo!("0x3f2550 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::isBroadcast(void)const")
}

// 0x3f2558 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
pub fn stub_3f2558() -> ! {
    todo!("0x3f2558 RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3f26b8 — __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
pub fn stub_3f26b8() -> ! {
    todo!("0x3f26b8 RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x3f26c8 — __ZNK3RBX10Reflection13EventDescBaseINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_3f26c8() -> ! {
    todo!("0x3f26c8 RBX::Reflection::EventDescBase<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x3f26dc — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3f26dc() -> ! {
    todo!("0x3f26dc RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::EventDesc(rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3f2860 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED1Ev
pub fn stub_3f2860() -> ! {
    todo!("0x3f2860 RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")
}

// 0x3f2884 — __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_13ClickDetectorEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx13remote_signalIS7_EEMS2_SA_ED0Ev
pub fn stub_3f2884() -> ! {
    todo!("0x3f2884 RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(boost::shared_ptr<RBX::Instance>),rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::remote_signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")
}

// 0x3f2938 — __ZN3RBX13ClickDetectorD2Ev
// type: void __fastcall(RBX::ClickDetector *this, int, int, int)
#[doc(alias = "RBX::ClickDetector::~ClickDetector()")]
// was: __ZN3RBX13ClickDetectorD2Ev
pub fn stub_3f2938() -> ! {
    todo!("0x3f2938 RBX::ClickDetector::~ClickDetector()")
}

// 0x3f2ab4 — __GLOBAL__I_a_171
// type: 
#[doc(alias = "global constructor keyed to_a_171")]
// was: __GLOBAL__I_a_171
pub fn stub_3f2ab4() -> ! {
    todo!("0x3f2ab4 global constructor keyed to_a_171")
}

// 0x3f2f00 — __ZN3RBX17CollectionService13getCollectionESs
// type: int __fastcall(shared_count *, int, std::string *)
#[doc(alias = "RBX::CollectionService::getCollection(std::string)")]
// was: __ZN3RBX17CollectionService13getCollectionESs
pub fn stub_3f2f00() -> ! {
    todo!("0x3f2f00 RBX::CollectionService::getCollection(std::string)")
}

// 0x3f2f38 — __ZN3RBX17CollectionServiceC1Ev
// type: int __fastcall(RBX::CollectionService *this)
#[doc(alias = "RBX::CollectionService::CollectionService(void)")]
// was: __ZN3RBX17CollectionServiceC1Ev
pub fn stub_3f2f38() -> ! {
    todo!("0x3f2f38 RBX::CollectionService::CollectionService(void)")
}

// 0x3f2f3c — __ZN3RBX17CollectionServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::CollectionService *this)
#[doc(alias = "RBX::CollectionService::CollectionService(void)")]
// was: __ZN3RBX17CollectionServiceC2Ev
pub fn stub_3f2f3c() -> ! {
    todo!("0x3f2f3c RBX::CollectionService::CollectionService(void)")
}

// 0x3f3178 — __ZN3RBX17CollectionService14removeInstanceEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CollectionService::removeInstance(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX17CollectionService14removeInstanceEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3f3178() -> ! {
    todo!("0x3f3178 RBX::CollectionService::removeInstance(boost::shared_ptr<RBX::Instance>)")
}

// 0x3f3394 — __ZN3RBX17CollectionService11addInstanceEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CollectionService::addInstance(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX17CollectionService11addInstanceEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3f3394() -> ! {
    todo!("0x3f3394 RBX::CollectionService::addInstance(boost::shared_ptr<RBX::Instance>)")
}

// 0x3f351c — __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_17CollectionServiceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEESsELi1EED1Ev
pub fn stub_3f351c() -> ! {
    todo!("0x3f351c RBX::Reflection::BoundFuncDesc<RBX::CollectionService,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")
}

// 0x3f355c — __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_17CollectionServiceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
pub fn stub_3f355c() -> ! {
    todo!("0x3f355c RBX::Reflection::EventDesc<RBX::CollectionService,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")
}

// 0x3f3580 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS1_INS2_8InstanceEEESaIS6_EEEEEESt4lessISsESaISt4pairIKSsSA_EEEixERSE_
// type: uint32_t *__fastcall(int, std::string *)
#[doc(alias = "std::map<std::string,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS1_INS2_8InstanceEEESaIS6_EEEEEESt4lessISsESaISt4pairIKSsSA_EEEixERSE_
pub fn stub_3f3580() -> ! {
    todo!("0x3f3580 std::map<std::string,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::operator[](std::string const&)")
}

// 0x3f379c — __ZN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS0_INS1_8InstanceEEESaIS5_EEEEE5resetIS8_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::reset<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> *)")]
// was: __ZN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS0_INS1_8InstanceEEESaIS5_EEEEE5resetIS8_EEvPT_
pub fn stub_3f379c() -> ! {
    todo!("0x3f379c void boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::reset<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> *)")
}

// 0x3f37c8 — __ZN3RBX17CollectionServiceD1Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "RBX::CollectionService::~CollectionService()")]
// was: __ZN3RBX17CollectionServiceD1Ev
pub fn stub_3f37c8() -> ! {
    todo!("0x3f37c8 RBX::CollectionService::~CollectionService()")
}

// 0x3f37cc — __ZN3RBX17CollectionServiceD0Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "RBX::CollectionService::~CollectionService()")]
// was: __ZN3RBX17CollectionServiceD0Ev
pub fn stub_3f37cc() -> ! {
    todo!("0x3f37cc RBX::CollectionService::~CollectionService()")
}

// 0x3f386c — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv
pub fn stub_3f386c() -> ! {
    todo!("0x3f386c __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv")
}

// 0x3f3894 — __ZThn32_N3RBX17CollectionServiceD1Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CollectionService::~CollectionService()")]
// was: __ZThn32_N3RBX17CollectionServiceD1Ev
pub fn stub_3f3894() -> ! {
    todo!("0x3f3894 non-virtual thunk toRBX::CollectionService::~CollectionService()")
}

// 0x3f389c — __ZThn32_N3RBX17CollectionServiceD0Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CollectionService::~CollectionService()")]
// was: __ZThn32_N3RBX17CollectionServiceD0Ev
pub fn stub_3f389c() -> ! {
    todo!("0x3f389c non-virtual thunk toRBX::CollectionService::~CollectionService()")
}

// 0x3f3940 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv
pub fn stub_3f3940() -> ! {
    todo!("0x3f3940 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv")
}

// 0x3f3968 — __ZThn36_N3RBX17CollectionServiceD1Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CollectionService::~CollectionService()")]
// was: __ZThn36_N3RBX17CollectionServiceD1Ev
pub fn stub_3f3968() -> ! {
    todo!("0x3f3968 non-virtual thunk toRBX::CollectionService::~CollectionService()")
}

// 0x3f3970 — __ZThn36_N3RBX17CollectionServiceD0Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CollectionService::~CollectionService()")]
// was: __ZThn36_N3RBX17CollectionServiceD0Ev
pub fn stub_3f3970() -> ! {
    todo!("0x3f3970 non-virtual thunk toRBX::CollectionService::~CollectionService()")
}

// 0x3f3a14 — __ZN3RBX17CollectionServiceD2Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
#[doc(alias = "RBX::CollectionService::~CollectionService()")]
// was: __ZN3RBX17CollectionServiceD2Ev
pub fn stub_3f3a14() -> ! {
    todo!("0x3f3a14 RBX::CollectionService::~CollectionService()")
}

// 0x3f3c08 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE8_M_eraseEPSt13_Rb_tree_nodeISD_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS3_INS4_8InstanceEEESaIS8_EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE8_M_eraseEPSt13_Rb_tree_nodeISD_E
pub fn stub_3f3c08() -> ! {
    todo!("0x3f3c08 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>> *)")
}

// 0x3f3c38 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS4_INS5_8InstanceEEESaIS9_EEEEEEEE7destroyEPSE_
// type: void __fastcall(int, std::string *)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX17copy_on_write_ptrISt6vectorINS4_INS5_8InstanceEEESaIS9_EEEEEEEE7destroyEPSE_
pub fn stub_3f3c38() -> ! {
    todo!("0x3f3c38 __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>*)")
}
