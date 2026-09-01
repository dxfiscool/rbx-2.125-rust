//! rendering shard 313 — 120 stubs 0x467b4c..0x46df08 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34080->34200 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34080 before -> 34200 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 after 0x467b4c (lowest remaining 0x467b4c..0x46df08, next lowest 0x46dff8)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x467b4c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED0Ev
pub fn stub_467b4c() -> ! {
    todo!("0x467b4c RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::~BoundYieldFuncDesc()")
}

// 0x467c20 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
pub fn stub_467c20() -> ! {
    todo!("0x467c20 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x467ed0 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEESsS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// type: void
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// was: __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEESsS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
pub fn stub_467ed0() -> ! {
    todo!("0x467ed0 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")
}

// 0x467fcc — __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: void
#[doc(alias = "__ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_467fcc() -> ! {
    todo!("0x467fcc __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

// 0x4680a0 — __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: void
#[doc(alias = "__ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_4680a0() -> ! {
    todo!("0x4680a0 __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

// 0x468174 — __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEEvT_
// type: void
#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEEvT_
pub fn stub_468174() -> ! {
    todo!("0x468174 void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")
}

// 0x468258 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// type: void
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_468258() -> ! {
    todo!("0x468258 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x468274 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// type: void
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
pub fn stub_468274() -> ! {
    todo!("0x468274 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")
}

// 0x46828c — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_46828c() -> ! {
    todo!("0x46828c bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x468364 — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_468364() -> ! {
    todo!("0x468364 bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x468434 — __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void
#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_468434() -> ! {
    todo!("0x468434 void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x4684f8 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string) &,boost::_bi::list1<std::string &> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_4684f8() -> ! {
    todo!("0x4684f8 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string) &,boost::_bi::list1<std::string &> &,int)")
}

// 0x468658 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_468658() -> ! {
    todo!("0x468658 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x4687a0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_4687a0() -> ! {
    todo!("0x4687a0 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x468918 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_468918() -> ! {
    todo!("0x468918 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x468948 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED0Ev
pub fn stub_468948() -> ! {
    todo!("0x468948 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")
}

// 0x468a14 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
pub fn stub_468a14() -> ! {
    todo!("0x468a14 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x468c38 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::BoundFuncDesc(void (RBX::DataModel::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_468c38() -> ! {
    todo!("0x468c38 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::BoundFuncDesc(void (RBX::DataModel::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x468db0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_468db0() -> ! {
    todo!("0x468db0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x468de0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED0Ev
pub fn stub_468de0() -> ! {
    todo!("0x468de0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x468eac — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_468eac() -> ! {
    todo!("0x468eac RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x468fe8 — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string),std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
pub fn stub_468fe8() -> ! {
    todo!("0x468fe8 RBX::Reflection::Call1Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string),std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x469118 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EEC2EMS2_FbvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::BoundFuncDesc(bool (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EEC2EMS2_FbvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_469118() -> ! {
    todo!("0x469118 RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::BoundFuncDesc(bool (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x46921c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EED0Ev
pub fn stub_46921c() -> ! {
    todo!("0x46921c RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc()")
}

// 0x4692d0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_4692d0() -> ! {
    todo!("0x4692d0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x4692f4 — __ZN3RBX10Reflection11Call0HelperINS_9DataModelEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DataModel,bool (RBX::DataModel::*)(void),bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_9DataModelEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE
pub fn stub_4692f4() -> ! {
    todo!("0x4692f4 RBX::Reflection::Call0Helper<RBX::DataModel,bool (RBX::DataModel::*)(void),bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(void),RBX::Reflection::Variant &)")
}

// 0x469324 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::BoundFuncDesc(void (RBX::DataModel::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_469324() -> ! {
    todo!("0x469324 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::BoundFuncDesc(void (RBX::DataModel::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x46949c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_46949c() -> ! {
    todo!("0x46949c RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x4694cc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EED0Ev
pub fn stub_4694cc() -> ! {
    todo!("0x4694cc RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x4695a0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_4695a0() -> ! {
    todo!("0x4695a0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x4695d8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_4695d8() -> ! {
    todo!("0x4695d8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::BoundFuncDesc(void (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x469750 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_469750() -> ! {
    todo!("0x469750 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x469780 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED0Ev
pub fn stub_469780() -> ! {
    todo!("0x469780 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x46984c — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_46984c() -> ! {
    todo!("0x46984c RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x469988 — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FvNS_9ContentIdEES3_vE4callEPS2_S5_RNS0_7VariantERKS3_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,void (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FvNS_9ContentIdEES3_vE4callEPS2_S5_RNS0_7VariantERKS3_
pub fn stub_469988() -> ! {
    todo!("0x469988 RBX::Reflection::Call1Helper<RBX::DataModel,void (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")
}

// 0x469ac4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::BoundFuncDesc(void (RBX::DataModel::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_469ac4() -> ! {
    todo!("0x469ac4 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::BoundFuncDesc(void (RBX::DataModel::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x469c3c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_469c3c() -> ! {
    todo!("0x469c3c RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x469c6c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EED0Ev
pub fn stub_469c6c() -> ! {
    todo!("0x469c6c RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc()")
}

// 0x469d40 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_469d40() -> ! {
    todo!("0x469d40 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x469d74 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EEC2EMS2_FvS9_NS3_8functionIFvS7_EEENSC_IFvSsEEEEPKcSK_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Instance::SaveFilter,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EEC2EMS2_FvS9_NS3_8functionIFvS7_EEENSC_IFvSsEEEEPKcSK_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_469d74() -> ! {
    todo!("0x469d74 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Instance::SaveFilter,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x469f20 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_469f20() -> ! {
    todo!("0x469f20 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x469f50 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED0Ev
pub fn stub_469f50() -> ! {
    todo!("0x469f50 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")
}

// 0x46a024 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSH_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSH_IFvSsEEE
pub fn stub_46a024() -> ! {
    todo!("0x46a024 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x46a1c4 — __ZN3RBX10Reflection9ArgHelper6getArgINS_8Instance10SaveFilterELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "RBX::Instance::SaveFilter RBX::Reflection::ArgHelper::getArg<RBX::Instance::SaveFilter,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Instance::SaveFilter> const&,boost::disable_if<boost::is_same<RBX::Instance::SaveFilter,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_8Instance10SaveFilterELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_46a1c4() -> ! {
    todo!("0x46a1c4 RBX::Instance::SaveFilter RBX::Reflection::ArgHelper::getArg<RBX::Instance::SaveFilter,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Instance::SaveFilter> const&,boost::disable_if<boost::is_same<RBX::Instance::SaveFilter,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x46a354 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS3_5TupleEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
// type: void
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// was: __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS3_5TupleEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
pub fn stub_46a354() -> ! {
    todo!("0x46a354 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")
}

// 0x46a454 — __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: void
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_46a454() -> ! {
    todo!("0x46a454 __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x46a528 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: void
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_46a528() -> ! {
    todo!("0x46a528 __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x46a5fc — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEEvT_
// type: void
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEEvT_
pub fn stub_46a5fc() -> ! {
    todo!("0x46a5fc void boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")
}

// 0x46a6e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// type: void
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
pub fn stub_46a6e0() -> ! {
    todo!("0x46a6e0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46a6fc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
pub fn stub_46a6fc() -> ! {
    todo!("0x46a6fc boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x46a714 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_46a714() -> ! {
    todo!("0x46a714 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x46a7ec — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: void
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_46a7ec() -> ! {
    todo!("0x46a7ec bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x46a8bc — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_46a8bc() -> ! {
    todo!("0x46a8bc void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x46a980 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKNS5_5TupleEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKNS5_5TupleEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_46a980() -> ! {
    todo!("0x46a980 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>) &,boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&> &,int)")
}

// 0x46aa8c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_46aa8c() -> ! {
    todo!("0x46aa8c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x46abd4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8Instance10SaveFilterEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Instance::SaveFilter &,boost::enable_if<boost::is_enum<RBX::Instance::SaveFilter>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8Instance10SaveFilterEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_46abd4() -> ! {
    todo!("0x46abd4 bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Instance::SaveFilter &,boost::enable_if<boost::is_enum<RBX::Instance::SaveFilter>,void>::type *)")
}

// 0x46ac28 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_46ac28() -> ! {
    todo!("0x46ac28 RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x46ada0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_46ada0() -> ! {
    todo!("0x46ada0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x46add0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev
pub fn stub_46add0() -> ! {
    todo!("0x46add0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x46ae9c — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_46ae9c() -> ! {
    todo!("0x46ae9c RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x46afdc — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::DataModel*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_
pub fn stub_46afdc() -> ! {
    todo!("0x46afdc RBX::Reflection::Call1Helper<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::DataModel*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")
}

// 0x46b164 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: void
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_46b164() -> ! {
    todo!("0x46b164 RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x46b354 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED0Ev
// type: void
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED0Ev
pub fn stub_46b354() -> ! {
    todo!("0x46b354 RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")
}

// 0x46b408 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
pub fn stub_46b408() -> ! {
    todo!("0x46b408 RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x46b55c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
pub fn stub_46b55c() -> ! {
    todo!("0x46b55c RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x46b6cc — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E13disconnectAllEPNS0_11EventSourceE
// type: void
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_46b6cc() -> ! {
    todo!("0x46b6cc RBX::Reflection::EventDescBase<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x46b6e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13disconnectAllEv
pub fn stub_46b6e0() -> ! {
    todo!("0x46b6e0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)")
}

// 0x46b858 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKPKNS2_18PropertyDescriptorENS4_IS3_EENS_3argILi1EEENSF_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISK_T0_T1_T2_EENSI_9list_av_3IT3_T4_T5_E4typeEEEMSN_FSK_SO_SP_ESS_ST_SU_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKPKNS2_18PropertyDescriptorENS4_IS3_EENS_3argILi1EEENSF_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISK_T0_T1_T2_EENSI_9list_av_3IT3_T4_T5_E4typeEEEMSN_FSK_SO_SP_ESS_ST_SU_
pub fn stub_46b858() -> ! {
    todo!("0x46b858 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x46b974 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEEvRKT_RKT0_
// type: void
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEEvRKT_RKT0_
pub fn stub_46b974() -> ! {
    todo!("0x46b974 void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&)")
}

// 0x46badc — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE5clearEv
// type: int(void)
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE5clearEv
pub fn stub_46badc() -> ! {
    todo!("0x46badc boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::clear(void)")
}

// 0x46bb0c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
pub fn stub_46bb0c() -> ! {
    todo!("0x46bb0c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")
}

// 0x46bbf0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
pub fn stub_46bbf0() -> ! {
    todo!("0x46bbf0 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

// 0x46bcd8 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
pub fn stub_46bcd8() -> ! {
    todo!("0x46bcd8 void boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x46bdd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: void
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
pub fn stub_46bdd0() -> ! {
    todo!("0x46bdd0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46bdec — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEvSC_SH_E6invokeERNS1_15function_bufferESC_SH_
// type: void
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEvSC_SH_E6invokeERNS1_15function_bufferESC_SH_
pub fn stub_46bdec() -> ! {
    todo!("0x46bdec boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x46be00 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_46be00() -> ! {
    todo!("0x46be00 bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x46bee8 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_46bee8() -> ! {
    todo!("0x46bee8 bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x46bfcc — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void
#[doc(alias = "void boost::detail::function::basic_vtable2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_46bfcc() -> ! {
    todo!("0x46bfcc void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x46c0a0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKPKNS5_18PropertyDescriptorEEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSM_ILi2EEEEEEclIS9_SE_EEvRT_RT0_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(rbx_core::SharedPtr<RBX::Instance> &,RBX::Reflection::PropertyDescriptor const* &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKPKNS5_18PropertyDescriptorEEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSM_ILi2EEEEEEclIS9_SE_EEvRT_RT0_
pub fn stub_46c0a0() -> ! {
    todo!("0x46c0a0 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(boost::shared_ptr<RBX::Instance> &,RBX::Reflection::PropertyDescriptor const* &)")
}

// 0x46c0bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_46c0bc() -> ! {
    todo!("0x46c0bc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x46c214 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE7connectINS2_8functionISB_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE7connectINS2_8functionISB_EEEENS0_10connectionERKT_
pub fn stub_46c214() -> ! {
    todo!("0x46c214 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> const&)")
}

// 0x46c308 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE6insertEPNSC_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE6insertEPNSC_4slotE
pub fn stub_46c308() -> ! {
    todo!("0x46c308 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot *)")
}

// 0x46c514 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotEEaSEPSE_
// type: int(void)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotEEaSEPSE_
pub fn stub_46c514() -> ! {
    todo!("0x46c514 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot*)")
}

// 0x46c538 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_EC2IPSD_EERKSG_T_
// type: void
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_EC2IPSD_EERKSG_T_
pub fn stub_46c538() -> ! {
    todo!("0x46c538 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>*>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>*)")
}

// 0x46c634 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_8functionISB_EEED1Ev
// type: void
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_8functionISB_EEED1Ev
pub fn stub_46c634() -> ! {
    todo!("0x46c634 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")
}

// 0x46c744 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_8functionISB_EEED0Ev
// type: void
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_8functionISB_EEED0Ev
pub fn stub_46c744() -> ! {
    todo!("0x46c744 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>>::~callable_slot()")
}

// 0x46c878 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_E4callES7_SB_
// type: void
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_E4callES7_SB_
pub fn stub_46c878() -> ! {
    todo!("0x46c878 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x46c950 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_E4callES7_SB_
// type: void
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_E4callES7_SB_
pub fn stub_46c950() -> ! {
    todo!("0x46c950 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x46c958 — __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEclES4_S8_
// type: int(void)
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)const")]
// was: __ZNK5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEclES4_S8_
pub fn stub_46c958() -> ! {
    todo!("0x46c958 boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)const")
}

// 0x46ca70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot24safe_static_do_get_mutexEv
// type: void
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot24safe_static_do_get_mutexEv
pub fn stub_46ca70() -> ! {
    todo!("0x46ca70 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_do_get_mutex(void)")
}

// 0x46cb60 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_ED1Ev
// type: void
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_ED1Ev
pub fn stub_46cb60() -> ! {
    todo!("0x46cb60 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x46cc70 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_ED0Ev
// type: void
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS5_10Reflection18PropertyDescriptorEEE4slotENS3_8functionISC_EELi2ESC_ED0Ev
pub fn stub_46cc70() -> ! {
    todo!("0x46cc70 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0x46cda0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slotD1Ev
// type: void
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slotD1Ev
pub fn stub_46cda0() -> ! {
    todo!("0x46cda0 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")
}

// 0x46cdd0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE13assign_to_ownERKS9_
// type: int(void)
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to_own(boost::function2<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*> const&)")]
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE13assign_to_ownERKS9_
pub fn stub_46cdd0() -> ! {
    todo!("0x46cdd0 boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to_own(boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*> const&)")
}

// 0x46ce00 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::BoundFuncDesc(void (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_46ce00() -> ! {
    todo!("0x46ce00 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::BoundFuncDesc(void (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x46cf04 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EED0Ev
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EED0Ev
pub fn stub_46cf04() -> ! {
    todo!("0x46cf04 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc()")
}

// 0x46cfb8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_46cfb8() -> ! {
    todo!("0x46cfb8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x46cfd8 — __ZN5boost6detail8function15functor_managerIPFvPN3RBX9DataModelEEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
// type: void
#[doc(alias = "boost::detail::function::functor_manager<void (*)(RBX::DataModel *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerIPFvPN3RBX9DataModelEEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
pub fn stub_46cfd8() -> ! {
    todo!("0x46cfd8 boost::detail::function::functor_manager<void (*)(RBX::DataModel *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46d034 — __ZN5boost6detail8function22void_function_invoker1IPFvPN3RBX9DataModelEEvS5_E6invokeERNS1_15function_bufferES5_
// type: void
#[doc(alias = "boost::detail::function::void_function_invoker1<void (*)(RBX::DataModel *),void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function22void_function_invoker1IPFvPN3RBX9DataModelEEvS5_E6invokeERNS1_15function_bufferES5_
pub fn stub_46d034() -> ! {
    todo!("0x46d034 boost::detail::function::void_function_invoker1<void (*)(RBX::DataModel *),void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x46d040 — __ZN5boost15circular_bufferIdSaIdEE9push_backERKd
// type: int(void)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::push_back(double const&)")]
// was: __ZN5boost15circular_bufferIdSaIdEE9push_backERKd
pub fn stub_46d040() -> ! {
    todo!("0x46d040 boost::circular_buffer<double,std::allocator<double>>::push_back(double const&)")
}

// 0x46d098 — __ZN5boost14singleton_poolIN3RBX15BallBallContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// was: __ZN5boost14singleton_poolIN3RBX15BallBallContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_46d098() -> ! {
    todo!("0x46d098 boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x46d0e4 — __ZNK3RBX13WindowAverageIddE4iterINS_22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE9GTCounterEEEvRT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter>(RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter &)const")]
// was: __ZNK3RBX13WindowAverageIddE4iterINS_22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE9GTCounterEEEvRT_
pub fn stub_46d0e4() -> ! {
    todo!("0x46d0e4 void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter>(RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter &)const")
}

// 0x46d128 — __ZNK3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE8getStatsEm
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
// was: __ZNK3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE8getStatsEm
pub fn stub_46d128() -> ! {
    todo!("0x46d128 RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")
}

// 0x46d1b0 — __ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm
// type: int(void)
#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
// was: __ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm
pub fn stub_46d1b0() -> ! {
    todo!("0x46d1b0 RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")
}

// 0x46d208 — __ZNK3RBX13WindowAverageIddE4iterINS_25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE4FSumEEEvRT_
// type: int(void)
#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum>(RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum &)const")]
// was: __ZNK3RBX13WindowAverageIddE4iterINS_25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE4FSumEEEvRT_
pub fn stub_46d208() -> ! {
    todo!("0x46d208 void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum>(RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum &)const")
}

// 0x46d248 — __ZN3RBX9DataModel10LegacyLock14ImplementationC2EPS0_N5boost10shared_ptrINS0_10GenericJobEEENS5_INS_6Limits7CounterEEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, RBX::Limits::Counter::Activator *, void *, int, int, int, int)
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::Implementation(RBX::DataModel*,rbx_core::SharedPtr<RBX::DataModel::GenericJob>,rbx_core::SharedPtr<RBX::Limits::Counter>)")]
// was: __ZN3RBX9DataModel10LegacyLock14ImplementationC2EPS0_N5boost10shared_ptrINS0_10GenericJobEEENS5_INS_6Limits7CounterEEE
pub fn stub_46d248() -> ! {
    todo!("0x46d248 RBX::DataModel::LegacyLock::Implementation::Implementation(RBX::DataModel*,boost::shared_ptr<RBX::DataModel::GenericJob>,boost::shared_ptr<RBX::Limits::Counter>)")
}

// 0x46d698 — __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEE14pop_if_presentERS8_
// type: void
#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::pop_if_present(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>&)")]
// was: __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEE14pop_if_presentERS8_
pub fn stub_46d698() -> ! {
    todo!("0x46d698 rbx::safe_queue<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>::pop_if_present(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>&)")
}

// 0x46d778 — __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEE5resetIS5_EEvPT_
// type: int(void)
#[doc(alias = "void rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>::reset<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")]
// was: __ZN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEE5resetIS5_EEvPT_
pub fn stub_46d778() -> ! {
    todo!("0x46d778 void boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>::reset<RBX::DataModel::LegacyLock::Implementation::Events>(RBX::DataModel::LegacyLock::Implementation::Events *)")
}

// 0x46d7a4 — __ZN5boost4bindIvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEES7_EENS_3_bi6bind_tIT_PFSA_T0_ENS8_9list_av_1IT1_E4typeEEESD_SF_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>(void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>)")]
// was: __ZN5boost4bindIvNS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEES7_EENS_3_bi6bind_tIT_PFSA_T0_ENS8_9list_av_1IT1_E4typeEEESD_SF_
pub fn stub_46d7a4() -> ! {
    todo!("0x46d7a4 boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list_av_1<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>::type> boost::bind<void,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>(void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>)")
}

// 0x46d8bc — __ZN3RBX9DataModel10LegacyLock14Implementation4taskEN5boost10shared_ptrINS2_6EventsEEE
// type: void
#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::task(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>)")]
// was: __ZN3RBX9DataModel10LegacyLock14Implementation4taskEN5boost10shared_ptrINS2_6EventsEEE
pub fn stub_46d8bc() -> ! {
    todo!("0x46d8bc RBX::DataModel::LegacyLock::Implementation::task(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>)")
}

// 0x46d908 — __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEE4pushERKS8_
// type: void
#[doc(alias = "rbx::safe_queue<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>::push(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
// was: __ZN3rbx10safe_queueIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEE4pushERKS8_
pub fn stub_46d908() -> ! {
    todo!("0x46d908 rbx::safe_queue<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>::push(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")
}

// 0x46d9cc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE9push_backERKS7_
// type: int(void)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::push_back(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE9push_backERKS7_
pub fn stub_46d9cc() -> ! {
    todo!("0x46d9cc std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::push_back(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")
}

// 0x46da0c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE16_M_push_back_auxERKS7_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, int)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_push_back_aux(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE16_M_push_back_auxERKS7_
pub fn stub_46da0c() -> ! {
    todo!("0x46da0c std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_push_back_aux(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events> const&)")
}

// 0x46db60 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE22_M_reserve_map_at_backEm
// type: int(void)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE22_M_reserve_map_at_backEm
pub fn stub_46db60() -> ! {
    todo!("0x46db60 std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_reserve_map_at_back(unsigned long)")
}

// 0x46db7c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE17_M_reallocate_mapEmb
pub fn stub_46db7c() -> ! {
    todo!("0x46db7c std::deque<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_reallocate_map(unsigned long,bool)")
}

// 0x46dc54 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEESaIS7_EE15_M_allocate_mapEm
pub fn stub_46dc54() -> ! {
    todo!("0x46dc54 std::_Deque_base<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>,std::allocator<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::_M_allocate_map(unsigned long)")
}

// 0x46dc6c — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEEEEC2ESA_
// type: void
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX9DataModel10LegacyLock14Implementation6EventsEEEEEEC2ESA_
pub fn stub_46dc6c() -> ! {
    todo!("0x46dc6c boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>)")
}

// 0x46dd4c — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS7_5list1INS7_5valueISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS7_5list1INS7_5valueISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS7_5list1INS7_5valueISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_46dd4c() -> ! {
    todo!("0x46dd4c __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS7_5list1INS7_5valueISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x46de28 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS6_5list1INS6_5valueISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS6_5list1INS6_5valueISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS6_5list1INS6_5valueISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_46de28() -> ! {
    todo!("0x46de28 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS6_5list1INS6_5valueISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

// 0x46df08 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS6_5list1INS6_5valueISC_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::DataModel::LegacyLock::Implementation::Events>>>>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS2_10LegacyLock14Implementation6EventsEEEENS6_5list1INS6_5valueISC_EEEEEEEEvT_
pub fn stub_46df08() -> ! {
    todo!("0x46df08 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::DataModel::LegacyLock::Implementation::Events>>>>)")
}
