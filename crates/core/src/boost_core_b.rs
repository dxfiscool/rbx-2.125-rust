//! core-B2: 100 boost stubs — filtered boost:: namespace.
//! Continuation of EA-ordered boost stubs (0x461efc..0x46c0bc) so `cargo check` stays green.
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 100 after 4630 already covered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; signatures use `rbx_core::SharedPtr` not `boost::`.


#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::Genre>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::Genre &,boost::enable_if<boost::is_enum<RBX::DataModel::Genre>,void>::type *)")]
// 0x461efc — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel5GenreEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::Genre>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::Genre &,boost::enable_if<boost::is_enum<RBX::DataModel::Genre>,void>::type *)
pub fn stub_461efc() -> ! {
    todo!("0x461efc __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel5GenreEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::DataModel::CreatorType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::CreatorType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::CreatorType> const&,boost::disable_if<boost::is_same<RBX::DataModel::CreatorType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x46229c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel11CreatorTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::DataModel::CreatorType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::CreatorType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::CreatorType> const&,boost::disable_if<boost::is_same<RBX::DataModel::CreatorType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_46229c() -> ! {
    todo!("0x46229c __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel11CreatorTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::DataModel::CreatorType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::CreatorType &,boost::enable_if<boost::is_enum<RBX::DataModel::CreatorType>,void>::type *)")]
// 0x462430 — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9DataModel11CreatorTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<2,RBX::DataModel::CreatorType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::CreatorType &,boost::enable_if<boost::is_enum<RBX::DataModel::CreatorType>,void>::type *)
pub fn stub_462430() -> ! {
    todo!("0x462430 __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9DataModel11CreatorTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x462800 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_462800() -> ! {
    todo!("0x462800 __ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::BoundCallbackDesc<RBX::DataModel>(char const*,boost::function<bool ()(void)> RBX::DataModel::*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x463220 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEEC2INS_9DataModelEEEPKcMT_N5boost8functionIS2_EENS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundCallbackDesc<bool ()(void)>::BoundCallbackDesc<RBX::DataModel>(char const*,boost::function<bool ()(void)> RBX::DataModel::*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_463220() -> ! {
    todo!("0x463220 __ZN3RBX10Reflection17BoundCallbackDescIFbvEEC2INS_9DataModelEEEPKcMT_N5boost8functionIS2_EENS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const")]
// 0x4635f0 — __ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE
// was: RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)const
pub fn stub_4635f0() -> ! {
    todo!("0x4635f0 __ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE")
}

#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")]
// 0x4637f0 — __ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_
// was: boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>(bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
pub fn stub_4637f0() -> ! {
    todo!("0x4637f0 __ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_")
}

#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)")]
// 0x463908 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE
// was: RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
pub fn stub_463908() -> ! {
    todo!("0x463908 __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE")
}

#[doc(alias = "boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)")]
// 0x463a5c — __ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_
// was: boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::Tuple>)
pub fn stub_463a5c() -> ! {
    todo!("0x463a5c __ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_")
}

#[doc(alias = "boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(boost::shared_ptr<RBX::Reflection::Tuple>)")]
// 0x463b98 — __ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE
// was: boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(boost::shared_ptr<RBX::Reflection::Tuple>)
pub fn stub_463b98() -> ! {
    todo!("0x463b98 __ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE")
}

#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// 0x463ce8 — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)
pub fn stub_463ce8() -> ! {
    todo!("0x463ce8 __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
// 0x463dc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()
pub fn stub_463dc0() -> ! {
    todo!("0x463dc0 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")]
// 0x463dc8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)
pub fn stub_463dc8() -> ! {
    todo!("0x463dc8 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")]
// 0x463e70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)
pub fn stub_463e70() -> ! {
    todo!("0x463e70 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>)")]
// 0x463e74 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>)
pub fn stub_463e74() -> ! {
    todo!("0x463e74 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_")
}

#[doc(alias = "__ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x463f54 — __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_463f54() -> ! {
    todo!("0x463f54 __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x464030 — __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_464030() -> ! {
    todo!("0x464030 __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>)")]
// 0x464110 — __ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_
// was: void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>)
pub fn stub_464110() -> ! {
    todo!("0x464110 __ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x464200 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_464200() -> ! {
    todo!("0x464200 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")]
// 0x46421c — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE
// was: boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)
pub fn stub_46421c() -> ! {
    todo!("0x46421c __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")]
// 0x464230 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const
pub fn stub_464230() -> ! {
    todo!("0x464230 __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x464310 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_464310() -> ! {
    todo!("0x464310 __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)")]
// 0x464408 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l
// was: bool boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)
pub fn stub_464408() -> ! {
    todo!("0x464408 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l")
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4644d8 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4644d8() -> ! {
    todo!("0x4644d8 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::setCallback(RBX::Reflection::DescribedBase *,boost::function<bool ()(void)> const&)const")]
// 0x464790 — __ZNK3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEE11setCallbackEPNS0_13DescribedBaseERKN5boost8functionIS2_EE
// was: RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::setCallback(RBX::Reflection::DescribedBase *,boost::function<bool ()(void)> const&)const
pub fn stub_464790() -> ! {
    todo!("0x464790 __ZNK3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEE11setCallbackEPNS0_13DescribedBaseERKN5boost8functionIS2_EE")
}

#[doc(alias = "boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)")]
// 0x4647cc — __ZN5boost8functionIFbvEEaSERKS2_
// was: boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)
pub fn stub_4647cc() -> ! {
    todo!("0x4647cc __ZN5boost8functionIFbvEEaSERKS2_")
}

#[doc(alias = "boost::function0<bool>::swap(boost::function0<bool>&)")]
// 0x464890 — __ZN5boost9function0IbE4swapERS1_
// was: boost::function0<bool>::swap(boost::function0<bool>&)
pub fn stub_464890() -> ! {
    todo!("0x464890 __ZN5boost9function0IbE4swapERS1_")
}

#[doc(alias = "boost::function0<bool>::move_assign(boost::function0<bool>&)")]
// 0x46496c — __ZN5boost9function0IbE11move_assignERS1_
// was: boost::function0<bool>::move_assign(boost::function0<bool>&)
pub fn stub_46496c() -> ! {
    todo!("0x46496c __ZN5boost9function0IbE11move_assignERS1_")
}

#[doc(alias = "boost::function0<bool>::clear(void)")]
// 0x464a70 — __ZN5boost9function0IbE5clearEv
// was: boost::function0<bool>::clear(void)
pub fn stub_464a70() -> ! {
    todo!("0x464a70 __ZN5boost9function0IbE5clearEv")
}

#[doc(alias = "boost::function0<bool>::assign_to_own(boost::function0<bool> const&)")]
// 0x464a9c — __ZN5boost9function0IbE13assign_to_ownERKS1_
// was: boost::function0<bool>::assign_to_own(boost::function0<bool> const&)
pub fn stub_464a9c() -> ! {
    todo!("0x464a9c __ZN5boost9function0IbE13assign_to_ownERKS1_")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::DataModel::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x464cd0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EEC2EMS2_FvN5boost8functionIFvbEEENS6_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::DataModel::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_464cd0() -> ! {
    todo!("0x464cd0 __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EEC2EMS2_FvN5boost8functionIFvbEEENS6_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x464e88 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_464e88() -> ! {
    todo!("0x464e88 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// 0x465010 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEEbS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)
pub fn stub_465010() -> ! {
    todo!("0x465010 __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEEbS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")
}

#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x465110 — __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_465110() -> ! {
    todo!("0x465110 __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4651e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4651e8() -> ! {
    todo!("0x4651e8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x465208 — __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_465208() -> ! {
    todo!("0x465208 __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x4652e0 — __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_4652e0() -> ! {
    todo!("0x4652e0 __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x4653b0 — __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_4653b0() -> ! {
    todo!("0x4653b0 __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x466388 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_466388() -> ! {
    todo!("0x466388 __ZN3RBX10Reflection9ArgHelper6getArgISsLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x4665dc — __ZN3RBX10Reflection9ArgHelper6getArgISsLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_4665dc() -> ! {
    todo!("0x4665dc __ZN3RBX10Reflection9ArgHelper6getArgISsLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x466830 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_466830() -> ! {
    todo!("0x466830 __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
// 0x466934 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_466934() -> ! {
    todo!("0x466934 __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x4669e8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_4669e8() -> ! {
    todo!("0x4669e8 __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::DataModel*,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),RBX::Reflection::Variant&)")]
// 0x466a0c — __ZN3RBX10Reflection11Call0HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
// was: RBX::Reflection::Call0Helper<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::DataModel*,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),RBX::Reflection::Variant&)
pub fn stub_466a0c() -> ! {
    todo!("0x466a0c __ZN3RBX10Reflection11Call0HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x467938 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EEC2EMS2_FvSsSsN5boost8functionIFvSsEEES8_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_467938() -> ! {
    todo!("0x467938 __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EEC2EMS2_FvSsSsN5boost8functionIFvSsEEES8_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x467c20 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_467c20() -> ! {
    todo!("0x467c20 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// 0x467ed0 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEESsS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)
pub fn stub_467ed0() -> ! {
    todo!("0x467ed0 __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEESsS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")
}

#[doc(alias = "__ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x467fcc — __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_467fcc() -> ! {
    todo!("0x467fcc __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvPFvNS0_IFvN3RBX10Reflection7VariantEEEESsENS4_5list2INS4_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x4680a0 — __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_4680a0() -> ! {
    todo!("0x4680a0 __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
// 0x468174 — __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)
pub fn stub_468174() -> ! {
    todo!("0x468174 __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x468258 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_468258() -> ! {
    todo!("0x468258 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
// 0x468274 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)
pub fn stub_468274() -> ! {
    todo!("0x468274 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x46828c — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_46828c() -> ! {
    todo!("0x46828c __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x468364 — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_468364() -> ! {
    todo!("0x468364 __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x468434 — __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_468434() -> ! {
    todo!("0x468434 __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string) &,boost::_bi::list1<std::string &> &,int)")]
// 0x4684f8 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string) &,boost::_bi::list1<std::string &> &,int)
pub fn stub_4684f8() -> ! {
    todo!("0x4684f8 __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x468658 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_468658() -> ! {
    todo!("0x468658 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x4687a0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_4687a0() -> ! {
    todo!("0x4687a0 __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EEC2EMS2_FvSsN5boost8functionIFvSsEEES8_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x468a14 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_468a14() -> ! {
    todo!("0x468a14 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Instance::SaveFilter,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x469d74 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EEC2EMS2_FvS9_NS3_8functionIFvS7_EEENSC_IFvSsEEEEPKcSK_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Instance::SaveFilter,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_469d74() -> ! {
    todo!("0x469d74 __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EEC2EMS2_FvS9_NS3_8functionIFvS7_EEENSC_IFvSsEEEEPKcSK_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x469f20 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_469f20() -> ! {
    todo!("0x469f20 __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
// 0x469f50 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED0Ev
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()
pub fn stub_469f50() -> ! {
    todo!("0x469f50 __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// 0x46a024 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSH_IFvSsEEE
// was: RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const
pub fn stub_46a024() -> ! {
    todo!("0x46a024 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSH_IFvSsEEE")
}

#[doc(alias = "RBX::Instance::SaveFilter RBX::Reflection::ArgHelper::getArg<RBX::Instance::SaveFilter,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Instance::SaveFilter> const&,boost::disable_if<boost::is_same<RBX::Instance::SaveFilter,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x46a1c4 — __ZN3RBX10Reflection9ArgHelper6getArgINS_8Instance10SaveFilterELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::Instance::SaveFilter RBX::Reflection::ArgHelper::getArg<RBX::Instance::SaveFilter,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Instance::SaveFilter> const&,boost::disable_if<boost::is_same<RBX::Instance::SaveFilter,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_46a1c4() -> ! {
    todo!("0x46a1c4 __ZN3RBX10Reflection9ArgHelper6getArgINS_8Instance10SaveFilterELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// 0x46a354 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS3_5TupleEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)
pub fn stub_46a354() -> ! {
    todo!("0x46a354 __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS3_5TupleEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x46a454 — __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_46a454() -> ! {
    todo!("0x46a454 __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS3_7VariantEEEES6_ENSA_5list2INSA_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x46a528 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_46a528() -> ! {
    todo!("0x46a528 __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
// 0x46a5fc — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)
pub fn stub_46a5fc() -> ! {
    todo!("0x46a5fc __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x46a6e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_46a6e0() -> ! {
    todo!("0x46a6e0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Reflection::Tuple const>)")]
// 0x46a6fc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Reflection::Tuple const>)
pub fn stub_46a6fc() -> ! {
    todo!("0x46a6fc __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x46a714 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_46a714() -> ! {
    todo!("0x46a714 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x46a7ec — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_46a7ec() -> ! {
    todo!("0x46a7ec __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x46a8bc — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_46a8bc() -> ! {
    todo!("0x46a8bc __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>) &,boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&> &,int)")]
// 0x46a980 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKNS5_5TupleEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>) &,boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&> &,int)
pub fn stub_46a980() -> ! {
    todo!("0x46a980 __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKNS5_5TupleEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x46aa8c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_46aa8c() -> ! {
    todo!("0x46aa8c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Instance::SaveFilter &,boost::enable_if<boost::is_enum<RBX::Instance::SaveFilter>,void>::type *)")]
// 0x46abd4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8Instance10SaveFilterEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Instance::SaveFilter>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Instance::SaveFilter &,boost::enable_if<boost::is_enum<RBX::Instance::SaveFilter>,void>::type *)
pub fn stub_46abd4() -> ! {
    todo!("0x46abd4 __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8Instance10SaveFilterEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x46ac28 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_46ac28() -> ! {
    todo!("0x46ac28 __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EEC2EMS2_FSB_SC_EPKcSI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x46ada0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_46ada0() -> ! {
    todo!("0x46ada0 __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// 0x46add0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()
pub fn stub_46add0() -> ! {
    todo!("0x46add0 __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x46ae9c — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_46ae9c() -> ! {
    todo!("0x46ae9c __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::DataModel*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
// 0x46afdc — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_
// was: RBX::Reflection::Call1Helper<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::ContentId,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::DataModel*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::DataModel::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)
pub fn stub_46afdc() -> ! {
    todo!("0x46afdc __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEESC_SB_E4callEPS2_SE_RNS0_7VariantERKSC_")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x46b164 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_46b164() -> ! {
    todo!("0x46b164 __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")]
// 0x46b354 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED0Ev
// was: RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()
pub fn stub_46b354() -> ! {
    todo!("0x46b354 __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED0Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x46b408 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_46b408() -> ! {
    todo!("0x46b408 __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// 0x46b55c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
// was: RBX::Reflection::EventDescImpl<2,RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub fn stub_46b55c() -> ! {
    todo!("0x46b55c __ZNK3RBX10Reflection13EventDescImplILi2ENS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// 0x46b6cc — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E13disconnectAllEPNS0_11EventSourceE
// was: RBX::Reflection::EventDescBase<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const
pub fn stub_46b6cc() -> ! {
    todo!("0x46b6cc __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_E13disconnectAllEPNS0_11EventSourceE")
}

#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)")]
// 0x46b6e0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13disconnectAllEv
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::disconnectAll(void)
pub fn stub_46b6e0() -> ! {
    todo!("0x46b6e0 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13disconnectAllEv")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// 0x46b858 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKPKNS2_18PropertyDescriptorENS4_IS3_EENS_3argILi1EEENSF_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISK_T0_T1_T2_EENSI_9list_av_3IT3_T4_T5_E4typeEEEMSN_FSK_SO_SP_ESS_ST_SU_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_46b858() -> ! {
    todo!("0x46b858 __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKPKNS2_18PropertyDescriptorENS4_IS3_EENS_3argILi1EEENSF_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISK_T0_T1_T2_EENSI_9list_av_3IT3_T4_T5_E4typeEEEMSN_FSK_SO_SP_ESS_ST_SU_")
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&)")]
// 0x46b974 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEEvRKT_RKT0_
// was: void RBX::Reflection::GenericSlotWrapper::execute2<boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&)
pub fn stub_46b974() -> ! {
    todo!("0x46b974 __ZN3RBX10Reflection18GenericSlotWrapper8execute2IN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEEvRKT_RKT0_")
}

#[doc(alias = "boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::clear(void)")]
// 0x46badc — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE5clearEv
// was: boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::clear(void)
pub fn stub_46badc() -> ! {
    todo!("0x46badc __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE5clearEv")
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
// 0x46bb0c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
pub fn stub_46bb0c() -> ! {
    todo!("0x46bb0c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSC_5list3INSC_5valueINS1_ISG_EEEENS_3argILi1EEENSQ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x46bbf0 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
pub fn stub_46bbf0() -> ! {
    todo!("0x46bbf0 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x46bcd8 — __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_46bcd8() -> ! {
    todo!("0x46bcd8 __ZN5boost9function2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS2_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS5_18GenericSlotWrapperERKS4_RKS8_EENSB_5list3INSB_5valueINS1_ISF_EEEENS_3argILi1EEENSP_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x46bdd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_46bdd0() -> ! {
    todo!("0x46bdd0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// 0x46bdec — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEvSC_SH_E6invokeERNS1_15function_bufferESC_SH_
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)
pub fn stub_46bdec() -> ! {
    todo!("0x46bdec __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEvSC_SH_E6invokeERNS1_15function_bufferESC_SH_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x46be00 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_46be00() -> ! {
    todo!("0x46be00 __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x46bee8 — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_46bee8() -> ! {
    todo!("0x46bee8 __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x46bfcc — __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_46bfcc() -> ! {
    todo!("0x46bfcc __ZNK5boost6detail8function13basic_vtable2IvNS_10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS7_18GenericSlotWrapperERKS6_RKSA_EENSD_5list3INSD_5valueINS3_ISH_EEEENS_3argILi1EEENSR_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(boost::shared_ptr<RBX::Instance> &,RBX::Reflection::PropertyDescriptor const* &)")]
// 0x46c0a0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKPKNS5_18PropertyDescriptorEEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSM_ILi2EEEEEEclIS9_SE_EEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>(boost::shared_ptr<RBX::Instance> &,RBX::Reflection::PropertyDescriptor const* &)
pub fn stub_46c0a0() -> ! {
    todo!("0x46c0a0 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEERKPKNS5_18PropertyDescriptorEEENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSM_ILi2EEEEEEclIS9_SE_EEvRT_RT0_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x46c0bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,RBX::Reflection::PropertyDescriptor const* const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_46c0bc() -> ! {
    todo!("0x46c0bc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKPKNS8_18PropertyDescriptorEEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSP_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}
