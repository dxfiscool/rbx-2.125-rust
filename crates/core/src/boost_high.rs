//! boost_high — 100 boost stubs high EA >=0x5C0000 (shard 2, EA-sorted).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 100 uncovered >=0x5C0000.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.

#[doc(
    alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)"
)]
// 0x5c001c — __ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)
pub fn stub_5c001c() -> ! {
    todo!("0x5c001c __ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_")
}

#[doc(
    alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)"
)]
// 0x5c0288 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// was: boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)
pub fn stub_5c0288() -> ! {
    todo!("0x5c0288 __ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")
}

#[doc(
    alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)"
)]
// 0x5c049c — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// was: boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)
pub fn stub_5c049c() -> ! {
    todo!("0x5c049c __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")
}

#[doc(
    alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)"
)]
// 0x5c05b4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// was: boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)
pub fn stub_5c05b4() -> ! {
    todo!("0x5c05b4 __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")
}

#[doc(
    alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE"
)]
// 0x5c06d0 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_5c06d0() -> ! {
    todo!("0x5c06d0 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(
    alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE"
)]
// 0x5c0830 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_5c0830() -> ! {
    todo!("0x5c0830 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(
    alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)"
)]
// 0x5c0994 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)
pub fn stub_5c0994() -> ! {
    todo!("0x5c0994 __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_")
}

#[doc(
    alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)"
)]
// 0x5c0b0c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_5c0b0c() -> ! {
    todo!("0x5c0b0c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(
    alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)"
)]
// 0x5c0b28 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_5c0b28() -> ! {
    todo!("0x5c0b28 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_")
}

#[doc(
    alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const"
)]
// 0x5c0b44 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const
pub fn stub_5c0b44() -> ! {
    todo!("0x5c0b44 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(
    alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const"
)]
// 0x5c0ca8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_5c0ca8() -> ! {
    todo!("0x5c0ca8 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(
    alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const"
)]
// 0x5c0e08 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_5c0e08() -> ! {
    todo!("0x5c0e08 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(
    alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)"
)]
// 0x5c0f1c — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_5c0f1c() -> ! {
    todo!("0x5c0f1c __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(
    alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)"
)]
// 0x5c102c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_5c102c() -> ! {
    todo!("0x5c102c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(rbx_core::WeakPtr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)"
)]
// 0x5c11ec — __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(rbx_core::WeakPtr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)
pub fn stub_5c11ec() -> ! {
    todo!("0x5c11ec __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)"
)]
// 0x5c1268 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)
pub fn stub_5c1268() -> ! {
    todo!("0x5c1268 __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(
    alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>> *)"
)]
// 0x5c12e8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>> *)
pub fn stub_5c12e8() -> ! {
    todo!("0x5c12e8 __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(
    alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>*)"
)]
// 0x5c1318 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>*)
pub fn stub_5c1318() -> ! {
    todo!("0x5c1318 __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_")
}

#[doc(alias = "RBX::Lighting::setTime(boost::posix_time::time_duration const&)")]
// 0x5c2874 — __ZN3RBX8Lighting7setTimeERKN5boost10posix_time13time_durationE
// was: RBX::Lighting::setTime(boost::posix_time::time_duration const&)
pub fn stub_5c2874() -> ! {
    todo!("0x5c2874 __ZN3RBX8Lighting7setTimeERKN5boost10posix_time13time_durationE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)")]
// 0x5c2eb4 — __ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_
// was: rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)
pub fn stub_5c2eb4() -> ! {
    todo!("0x5c2eb4 __ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)")]
// 0x5c2eec — __ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_
// was: rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)
pub fn stub_5c2eec() -> ! {
    todo!("0x5c2eec __ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(
    alias = "RBX::Reflection::EventDescImpl<1,RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const"
)]
// 0x5c4cac — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5c4cac() -> ! {
    todo!("0x5c4cac __ZNK3RBX10Reflection13EventDescImplILi1ENS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(
    alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> boost::posix_time::to_simple_string_type<char>(boost::posix_time::time_duration)"
)]
// 0x5c501c — __ZN5boost10posix_time21to_simple_string_typeIcEESbIT_St11char_traitsIS2_ESaIS2_EENS0_13time_durationE
// was: std::basic_string<char,std::char_traits<char>,std::allocator<char>> boost::posix_time::to_simple_string_type<char>(boost::posix_time::time_duration)
pub fn stub_5c501c() -> ! {
    todo!("0x5c501c __ZN5boost10posix_time21to_simple_string_typeIcEESbIT_St11char_traitsIS2_ESaIS2_EENS0_13time_durationE")
}

#[doc(
    alias = "boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&)const"
)]
// 0x5c5354 — __ZNK5boost9date_time11int_adapterIxE7compareERKS2_
// was: boost::date_time::int_adapter<long long>::compare(boost::date_time::int_adapter<long long> const&)const
pub fn stub_5c5354() -> ! {
    todo!("0x5c5354 __ZNK5boost9date_time11int_adapterIxE7compareERKS2_")
}

#[doc(
    alias = "boost::posix_time::time_duration boost::date_time::str_from_delimited_time_duration<boost::posix_time::time_duration,char>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)"
)]
// 0x5c549c — __ZN5boost9date_time32str_from_delimited_time_durationINS_10posix_time13time_durationEcEET_RKSbIT0_St11char_traitsIS5_ESaIS5_EE
// was: boost::posix_time::time_duration boost::date_time::str_from_delimited_time_duration<boost::posix_time::time_duration,char>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)
pub fn stub_5c549c() -> ! {
    todo!("0x5c549c __ZN5boost9date_time32str_from_delimited_time_durationINS_10posix_time13time_durationEcEET_RKSbIT0_St11char_traitsIS5_ESaIS5_EE")
}

#[doc(alias = "boost::date_time::int_adapter<long long>::operator*(int)const")]
// 0x5c5d80 — __ZNK5boost9date_time11int_adapterIxEmlEi
// was: boost::date_time::int_adapter<long long>::operator*(int)const
pub fn stub_5c5d80() -> ! {
    todo!("0x5c5d80 __ZNK5boost9date_time11int_adapterIxEmlEi")
}

#[doc(alias = "boost::date_time::int_adapter<long long>::mult_div_specials(int const&)const")]
// 0x5c5de4 — __ZNK5boost9date_time11int_adapterIxE17mult_div_specialsERKi
// was: boost::date_time::int_adapter<long long>::mult_div_specials(int const&)const
pub fn stub_5c5de4() -> ! {
    todo!("0x5c5de4 __ZNK5boost9date_time11int_adapterIxE17mult_div_specialsERKi")
}

#[doc(alias = "boost::char_separator<char,std::char_traits<char>>::is_kept(char)const")]
// 0x5c5e9c — __ZNK5boost14char_separatorIcSt11char_traitsIcEE7is_keptEc
// was: boost::char_separator<char,std::char_traits<char>>::is_kept(char)const
pub fn stub_5c5e9c() -> ! {
    todo!("0x5c5e9c __ZNK5boost14char_separatorIcSt11char_traitsIcEE7is_keptEc")
}

#[doc(
    alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::ispunct(char)"
)]
// 0x5c5ecc — __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7ispunctEc
// was: boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::ispunct(char)
pub fn stub_5c5ecc() -> ! {
    todo!("0x5c5ecc __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7ispunctEc")
}

#[doc(
    alias = "boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::isspace(char)"
)]
// 0x5c5efc — __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7isspaceEc
// was: boost::tokenizer_detail::traits_extension_details<std::char_traits<char>,1>::isspace(char)
pub fn stub_5c5efc() -> ! {
    todo!("0x5c5efc __ZN5boost16tokenizer_detail24traits_extension_detailsISt11char_traitsIcELi1EE7isspaceEc")
}

#[doc(
    alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long long>(long long &)"
)]
// 0x5c5f2c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIxEEbRT_
// was: bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long long>(long long &)
pub fn stub_5c5f2c() -> ! {
    todo!("0x5c5f2c __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIxEEbRT_")
}

#[doc(
    alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long long,char>(unsigned long long &,char const*,char const*)"
)]
// 0x5c5fb8 — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEycEEbRT0_PKT1_S8_
// was: bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long long,char>(unsigned long long &,char const*,char const*)
pub fn stub_5c5fb8() -> ! {
    todo!("0x5c5fb8 __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEycEEbRT0_PKT1_S8_")
}

#[doc(
    alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned short>(unsigned short &)"
)]
// 0x5c641c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedItEEbRT_
// was: bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned short>(unsigned short &)
pub fn stub_5c641c() -> ! {
    todo!("0x5c641c __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedItEEbRT_")
}

#[doc(
    alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned short,char>(unsigned short &,char const*,char const*)"
)]
// 0x5c6460 — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEtcEEbRT0_PKT1_S8_
// was: bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned short,char>(unsigned short &,char const*,char const*)
pub fn stub_5c6460() -> ! {
    todo!("0x5c6460 __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEtcEEbRT0_PKT1_S8_")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)"
)]
// 0x5c9708 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv
// was: rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)
pub fn stub_5c9708() -> ! {
    todo!("0x5c9708 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)"
)]
// 0x5c9f04 — __ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5c9f04() -> ! {
    todo!("0x5c9f04 __ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hint,RBX::Hint>(rbx_core::SharedPtr<RBX::Hint> const*,RBX::Hint *)const"
)]
// 0x5c9fcc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HintES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hint,RBX::Hint>(rbx_core::SharedPtr<RBX::Hint> const*,RBX::Hint *)const
pub fn stub_5c9fcc() -> ! {
    todo!("0x5c9fcc __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4HintES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(
    alias = "boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)"
)]
// 0x5ca0b4 — __ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5ca0b4() -> ! {
    todo!("0x5ca0b4 __ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()"
)]
// 0x5ca1bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_5ca1bc() -> ! {
    todo!("0x5ca1bc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()"
)]
// 0x5ca1c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_5ca1c0() -> ! {
    todo!("0x5ca1c0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)"
)]
// 0x5ca1c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_5ca1c4() -> ! {
    todo!("0x5ca1c4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)"
)]
// 0x5ca1e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_5ca1e4() -> ! {
    todo!("0x5ca1e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)"
)]
// 0x5ca1fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_5ca1fc() -> ! {
    todo!("0x5ca1fc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4HintENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)"
)]
// 0x5ca804 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv
// was: rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)
pub fn stub_5ca804() -> ! {
    todo!(
        "0x5ca804 __ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv"
    )
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)"
)]
// 0x5ca8b4 — __ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5ca8b4() -> ! {
    todo!("0x5ca8b4 __ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Message,RBX::Message>(rbx_core::SharedPtr<RBX::Message> const*,RBX::Message *)const"
)]
// 0x5ca97c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7MessageES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Message,RBX::Message>(rbx_core::SharedPtr<RBX::Message> const*,RBX::Message *)const
pub fn stub_5ca97c() -> ! {
    todo!("0x5ca97c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7MessageES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(
    alias = "boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)"
)]
// 0x5caa64 — __ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5caa64() -> ! {
    todo!("0x5caa64 __ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()"
)]
// 0x5cab6c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_5cab6c() -> ! {
    todo!("0x5cab6c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()"
)]
// 0x5cab70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_5cab70() -> ! {
    todo!("0x5cab70 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)"
)]
// 0x5cab74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_5cab74() -> ! {
    todo!("0x5cab74 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)"
)]
// 0x5cab94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_5cab94() -> ! {
    todo!("0x5cab94 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)"
)]
// 0x5cabac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_5cabac() -> ! {
    todo!("0x5cabac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7MessageENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Translate(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*)")]
// 0x5cc74c — __ZN3RBXL9TranslateEN5boost10shared_ptrINS_8InstanceEEEPKN3G3D7Vector3E
// was: RBX::Translate(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*)
pub fn stub_5cc74c() -> ! {
    todo!("0x5cc74c __ZN3RBXL9TranslateEN5boost10shared_ptrINS_8InstanceEEEPKN3G3D7Vector3E")
}

#[doc(
    alias = "RBX::rotateModelPart(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*)"
)]
// 0x5cc7ec — __ZN3RBXL15rotateModelPartEN5boost10shared_ptrINS_8InstanceEEEPKN3G3D15CoordinateFrameEPKNS4_7Vector3E
// was: RBX::rotateModelPart(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*)
pub fn stub_5cc7ec() -> ! {
    todo!("0x5cc7ec __ZN3RBXL15rotateModelPartEN5boost10shared_ptrINS_8InstanceEEEPKN3G3D15CoordinateFrameEPKNS4_7Vector3E")
}

#[doc(
    alias = "RBX::ModelInstance::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)"
)]
// 0x5ccac0 — __ZN3RBX13ModelInstance20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::ModelInstance::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)
pub fn stub_5ccac0() -> ! {
    todo!("0x5ccac0 __ZN3RBX13ModelInstance20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(
    alias = "RBX::VisitModelDescendants(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *)"
)]
// 0x5cce3c — __ZN3RBX21VisitModelDescendantsEN5boost10shared_ptrINS_8InstanceEEEPPNS_12PartInstanceEPf
// was: RBX::VisitModelDescendants(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *)
pub fn stub_5cce3c() -> ! {
    todo!("0x5cce3c __ZN3RBX21VisitModelDescendantsEN5boost10shared_ptrINS_8InstanceEEEPPNS_12PartInstanceEPf")
}

#[doc(alias = "RBX::makeJ(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5ccebc — __ZN3RBXL5makeJEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::makeJ(rbx_core::SharedPtr<RBX::Instance>)
pub fn stub_5ccebc() -> ! {
    todo!("0x5ccebc __ZN3RBXL5makeJEN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::breakJ(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5ccf3c — __ZN3RBXL6breakJEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::breakJ(rbx_core::SharedPtr<RBX::Instance>)
pub fn stub_5ccf3c() -> ! {
    todo!("0x5ccf3c __ZN3RBXL6breakJEN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(alias = "RBX::unionPartExtentsWorld(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &)")]
// 0x5cd44c — __ZN3RBXL21unionPartExtentsWorldEN5boost10shared_ptrINS_8InstanceEEERNS_7ExtentsE
// was: RBX::unionPartExtentsWorld(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &)
pub fn stub_5cd44c() -> ! {
    todo!("0x5cd44c __ZN3RBXL21unionPartExtentsWorldEN5boost10shared_ptrINS_8InstanceEEERNS_7ExtentsE")
}

#[doc(
    alias = "RBX::unionPartExtentsLocal(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&)"
)]
// 0x5cd668 — __ZN3RBXL21unionPartExtentsLocalEN5boost10shared_ptrINS_8InstanceEEERNS_7ExtentsERKN3G3D15CoordinateFrameE
// was: RBX::unionPartExtentsLocal(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&)
pub fn stub_5cd668() -> ! {
    todo!("0x5cd668 __ZN3RBXL21unionPartExtentsLocalEN5boost10shared_ptrINS_8InstanceEEERNS_7ExtentsERKN3G3D15CoordinateFrameE")
}

#[doc(
    alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const"
)]
// 0x5cdc0c — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D7Vector3EENS3_5list2INS2_3argILi1EEENS3_5valueIPS8_EEEEEEEEvRKT_
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const
pub fn stub_5cdc0c() -> ! {
    todo!("0x5cdc0c __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D7Vector3EENS3_5list2INS2_3argILi1EEENS3_5valueIPS8_EEEEEEEEvRKT_")
}

#[doc(
    alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const"
)]
// 0x5cdd14 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D15CoordinateFrameEPKNS7_7Vector3EENS3_5list3INS2_3argILi1EEENS3_5valueIPS8_EENSJ_IPSB_EEEEEEEEvRKT_
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const
pub fn stub_5cdd14() -> ! {
    todo!("0x5cdd14 __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D15CoordinateFrameEPKNS7_7Vector3EENS3_5list3INS2_3argILi1EEENS3_5valueIPS8_EENSJ_IPSB_EEEEEEEEvRKT_")
}

#[doc(
    alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const"
)]
// 0x5cde48 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const
pub fn stub_5cde48() -> ! {
    todo!("0x5cde48 __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_")
}

#[doc(
    alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const"
)]
// 0x5cdf50 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const
pub fn stub_5cdf50() -> ! {
    todo!("0x5cdf50 __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_")
}

#[doc(
    alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const"
)]
// 0x5ce058 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsERKN3G3D15CoordinateFrameEENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS7_EENSI_ISB_EEEEEEEEvRKT_
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const
pub fn stub_5ce058() -> ! {
    todo!("0x5ce058 __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsERKN3G3D15CoordinateFrameEENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS7_EENSI_ISB_EEEEEEEEvRKT_")
}

#[doc(
    alias = "void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)"
)]
// 0x5ce160 — __ZN5boost3_bi5list3INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEENS4_IKN3G3D15CoordinateFrameEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_RSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)
pub fn stub_5ce160() -> ! {
    todo!("0x5ce160 __ZN5boost3_bi5list3INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEENS4_IKN3G3D15CoordinateFrameEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_RSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(
    alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)"
)]
// 0x5ce23c — __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)
pub fn stub_5ce23c() -> ! {
    todo!("0x5ce23c __ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(
    alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)"
)]
// 0x5ce310 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)
pub fn stub_5ce310() -> ! {
    todo!("0x5ce310 __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(
    alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)"
)]
// 0x5ce574 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3G3D15CoordinateFrameEEENS4_IPNS5_7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_PKS9_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)
pub fn stub_5ce574() -> ! {
    todo!("0x5ce574 __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3G3D15CoordinateFrameEEENS4_IPNS5_7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_PKS9_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(
    alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)"
)]
// 0x5ce650 — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3G3D7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_ENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)
pub fn stub_5ce650() -> ! {
    todo!("0x5ce650 __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3G3D7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_ENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(
    alias = "RBX::Reflection::EventDescImpl<1,RBX::Mouse,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Mouse::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const"
)]
// 0x5d3fe4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_5MouseEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<1,RBX::Mouse,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Mouse::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5d3fe4() -> ! {
    todo!("0x5d3fe4 __ZNK3RBX10Reflection13EventDescImplILi1ENS_5MouseEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(
    alias = "RBX::Reflection::EventDescImpl<0,RBX::Mouse,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Mouse::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const"
)]
// 0x5d4528 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_5MouseEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::Mouse,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Mouse::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5d4528() -> ! {
    todo!("0x5d4528 __ZNK3RBX10Reflection13EventDescImplILi0ENS_5MouseEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)"
)]
// 0x5d7738 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4PART5WedgeEEEN5boost10shared_ptrIT_EEv
// was: rbx_core::SharedPtr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)
pub fn stub_5d7738() -> ! {
    todo!("0x5d7738 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4PART5WedgeEEEN5boost10shared_ptrIT_EEv")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)"
)]
// 0x5d77ec — __ZN5boost10shared_ptrIN3RBX4PART5WedgeEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: rbx_core::SharedPtr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5d77ec() -> ! {
    todo!("0x5d77ec __ZN5boost10shared_ptrIN3RBX4PART5WedgeEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(
    alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PART::Wedge,RBX::PART::Wedge>(rbx_core::SharedPtr<RBX::PART::Wedge> const*,RBX::PART::Wedge *)const"
)]
// 0x5d78b4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4PART5WedgeES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PART::Wedge,RBX::PART::Wedge>(rbx_core::SharedPtr<RBX::PART::Wedge> const*,RBX::PART::Wedge *)const
pub fn stub_5d78b4() -> ! {
    todo!("0x5d78b4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4PART5WedgeES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(
    alias = "boost::detail::shared_count::shared_count<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)"
)]
// 0x5d799c — __ZN5boost6detail12shared_countC2IPN3RBX4PART5WedgeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5d799c() -> ! {
    todo!("0x5d799c __ZN5boost6detail12shared_countC2IPN3RBX4PART5WedgeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()"
)]
// 0x5d7aa4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_5d7aa4() -> ! {
    todo!("0x5d7aa4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()"
)]
// 0x5d7aa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_5d7aa8() -> ! {
    todo!("0x5d7aa8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)"
)]
// 0x5d7aac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_5d7aac() -> ! {
    todo!("0x5d7aac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)"
)]
// 0x5d7acc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_5d7acc() -> ! {
    todo!("0x5d7acc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(
    alias = "boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)"
)]
// 0x5d7ae4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_5d7ae4() -> ! {
    todo!("0x5d7ae4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PART5WedgeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(
    alias = "RBX::PartInstance::primitivesToParts(G3D::Array<RBX::Primitive *,10,32ul> const&,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> &)"
)]
// 0x5dae40 — __ZN3RBX12PartInstance17primitivesToPartsERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERSt6vectorIN5boost10shared_ptrIS0_EESaISB_EE
// was: RBX::PartInstance::primitivesToParts(G3D::Array<RBX::Primitive *,10,32ul> const&,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>> &)
pub fn stub_5dae40() -> ! {
    todo!("0x5dae40 __ZN3RBX12PartInstance17primitivesToPartsERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERSt6vectorIN5boost10shared_ptrIS0_EESaISB_EE")
}

#[doc(
    alias = "RBX::PartInstance::findParts(RBX::Instance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> &)"
)]
// 0x5daf54 — __ZN3RBX12PartInstance9findPartsEPNS_8InstanceERSt6vectorIN5boost8weak_ptrIS0_EESaIS6_EE
// was: RBX::PartInstance::findParts(RBX::Instance *,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> &)
pub fn stub_5daf54() -> ! {
    todo!("0x5daf54 __ZN3RBX12PartInstance9findPartsEPNS_8InstanceERSt6vectorIN5boost8weak_ptrIS0_EESaIS6_EE")
}

#[doc(alias = "RBX::PartInstance::nonNullInWorkspace(rbx_core::SharedPtr<RBX::PartInstance>)")]
// 0x5db0c8 — __ZN3RBX12PartInstance18nonNullInWorkspaceEN5boost10shared_ptrIS0_EE
// was: RBX::PartInstance::nonNullInWorkspace(rbx_core::SharedPtr<RBX::PartInstance>)
pub fn stub_5db0c8() -> ! {
    todo!("0x5db0c8 __ZN3RBX12PartInstance18nonNullInWorkspaceEN5boost10shared_ptrIS0_EE")
}

#[doc(alias = "RBX::PartInstance::reportUntouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// 0x5dbb84 — __ZN3RBX12PartInstance13reportUntouchERKN5boost10shared_ptrIS0_EE
// was: RBX::PartInstance::reportUntouch(rbx_core::SharedPtr<RBX::PartInstance> const&)
pub fn stub_5dbb84() -> ! {
    todo!("0x5dbb84 __ZN3RBX12PartInstance13reportUntouchERKN5boost10shared_ptrIS0_EE")
}

#[doc(
    alias = "RBX::PartInstance::getConnectedPartsRecursiveImpl(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::unordered_set<RBX::PartInstance*,boost::hash<RBX::PartInstance*>,std::equal_to<RBX::PartInstance*>,std::allocator<RBX::PartInstance*>> &)const"
)]
// 0x5de610 — __ZNK3RBX12PartInstance30getConnectedPartsRecursiveImplERN5boost10shared_ptrISt6vectorINS2_INS_8InstanceEEESaIS5_EEEERNS1_9unordered13unordered_setIPS0_NS1_4hashISC_EESt8equal_toISC_ESaISC_EEE
// was: RBX::PartInstance::getConnectedPartsRecursiveImpl(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::unordered_set<RBX::PartInstance*,boost::hash<RBX::PartInstance*>,std::equal_to<RBX::PartInstance*>,std::allocator<RBX::PartInstance*>> &)const
pub fn stub_5de610() -> ! {
    todo!("0x5de610 __ZNK3RBX12PartInstance30getConnectedPartsRecursiveImplERN5boost10shared_ptrISt6vectorINS2_INS_8InstanceEEESaIS5_EEEERNS1_9unordered13unordered_setIPS0_NS1_4hashISC_EESt8equal_toISC_ESaISC_EEE")
}

#[doc(
    alias = "RBX::getConnectedPartsVisitor(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &)"
)]
// 0x5de7fc — __ZN3RBXL24getConnectedPartsVisitorEPNS_9PrimitiveERN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// was: RBX::getConnectedPartsVisitor(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &)
pub fn stub_5de7fc() -> ! {
    todo!("0x5de7fc __ZN3RBXL24getConnectedPartsVisitorEPNS_9PrimitiveERN5boost10shared_ptrISt6vectorINS3_INS_8InstanceEEESaIS6_EEEE")
}

#[doc(alias = "RBX::PartInstance::reportTouch(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// 0x5e01dc — __ZN3RBX12PartInstance11reportTouchERKN5boost10shared_ptrIS0_EE
// was: RBX::PartInstance::reportTouch(rbx_core::SharedPtr<RBX::PartInstance> const&)
pub fn stub_5e01dc() -> ! {
    todo!("0x5e01dc __ZN3RBX12PartInstance11reportTouchERKN5boost10shared_ptrIS0_EE")
}

#[doc(
    alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()"
)]
// 0x5e0ce0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()
pub fn stub_5e0ce0() -> ! {
    todo!("0x5e0ce0 __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED1Ev")
}

#[doc(
    alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()"
)]
// 0x5e0eec — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED1Ev
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()
pub fn stub_5e0eec() -> ! {
    todo!("0x5e0eec __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED1Ev")
}

#[doc(
    alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()"
)]
// 0x5e0f1c — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED1Ev
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::~EventDesc()
pub fn stub_5e0f1c() -> ! {
    todo!("0x5e0f1c __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEED1Ev")
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)"
)]
// 0x5e0ff8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
// was: rbx_core::SharedPtr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)
pub fn stub_5e0ff8() -> ! {
    todo!("0x5e0ff8 __ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv")
}

#[doc(
    alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::push_back(rbx_core::SharedPtr<RBX::PartInstance> const&)"
)]
// 0x5e15c0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
// was: std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::push_back(rbx_core::SharedPtr<RBX::PartInstance> const&)
pub fn stub_5e15c0() -> ! {
    todo!(
        "0x5e15c0 __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_"
    )
}

#[doc(
    alias = "rbx_core::SharedPtr<RBX::PartInstance> RBX::shared_from<RBX::PartInstance>(RBX::PartInstance*)"
)]
// 0x5e1610 — __ZN3RBX11shared_fromINS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS4_
// was: rbx_core::SharedPtr<RBX::PartInstance> RBX::shared_from<RBX::PartInstance>(RBX::PartInstance*)
pub fn stub_5e1610() -> ! {
    todo!("0x5e1610 __ZN3RBX11shared_fromINS_12PartInstanceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(
    alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::push_back(rbx_core::WeakPtr<RBX::PartInstance> const&)"
)]
// 0x5e1780 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_
// was: std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::push_back(rbx_core::WeakPtr<RBX::PartInstance> const&)
pub fn stub_5e1780() -> ! {
    todo!("0x5e1780 __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EE9push_backERKS4_")
}

#[doc(
    alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::push_back(rbx_core::SharedPtr<RBX::Instance> const&)"
)]
// 0x5e1f50 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_
// was: std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::push_back(rbx_core::SharedPtr<RBX::Instance> const&)
pub fn stub_5e1f50() -> ! {
    todo!("0x5e1f50 __ZNSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE9push_backERKS4_")
}

#[doc(
    alias = "void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)"
)]
// 0x5e1fa0 — __ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
// was: void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)
pub fn stub_5e1fa0() -> ! {
    todo!("0x5e1fa0 __ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_")
}

#[doc(
    alias = "boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list_av_2<boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)"
)]
// 0x5e20e4 — __ZN5boost4bindIvPN3RBX9PrimitiveERNS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
// was: boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list_av_2<boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::type> boost::bind<void,RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::arg<1>,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)
pub fn stub_5e20e4() -> ! {
    todo!("0x5e20e4 __ZN5boost4bindIvPN3RBX9PrimitiveERNS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEENS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_")
}

#[doc(alias = "RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// 0x5e2888 — __ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)
pub fn stub_5e2888() -> ! {
    todo!("0x5e2888 __ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE")
}

#[doc(
    alias = "void boost::throw_exception<rbx::bad_placement_any_cast>(rbx::bad_placement_any_cast const&)"
)]
// 0x5e3fd0 — __ZN5boost15throw_exceptionIN3rbx22bad_placement_any_castEEEvRKT_
// was: void boost::throw_exception<rbx::bad_placement_any_cast>(rbx::bad_placement_any_cast const&)
pub fn stub_5e3fd0() -> ! {
    todo!("0x5e3fd0 __ZN5boost15throw_exceptionIN3rbx22bad_placement_any_castEEEvRKT_")
}

// -- next 100 core boost high shard (watchdog refill) --
#[doc(alias = "RBX::Tool::setTimerCallback(rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x681fd8 — __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE
// was: RBX::Tool::setTimerCallback(rbx_core::WeakPtr<RBX::Network::Player>)
pub fn stub_681fd8() -> ! {
    todo!("0x681fd8 __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE")
}

#[doc(alias = "RBX::Tool::moveOtherToolsToBackpack(rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x682190 — __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE
// was: RBX::Tool::moveOtherToolsToBackpack(rbx_core::WeakPtr<RBX::Network::Player>)
pub fn stub_682190() -> ! {
    todo!("0x682190 __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)")]
// 0x682e2c — __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>,RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::WeakPtr<RBX::Network::Player>),RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)
pub fn stub_682e2c() -> ! {
    todo!("0x682e2c __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
// 0x683ee0 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: rbx_core::SharedPtr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(rbx_core::WeakPtr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)
pub fn stub_683ee0() -> ! {
    todo!("0x683ee0 __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x683f5c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
pub fn stub_683f5c() -> ! {
    todo!("0x683f5c __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x684044 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// was: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_684044() -> ! {
    todo!("0x684044 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)")]
// 0x684130 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>)
pub fn stub_684130() -> ! {
    todo!("0x684130 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x68422c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_68422c() -> ! {
    todo!("0x68422c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x684248 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_684248() -> ! {
    todo!("0x684248 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
// 0x684260 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const
pub fn stub_684260() -> ! {
    todo!("0x684260 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x68434c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_68434c() -> ! {
    todo!("0x68434c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x684434 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_684434() -> ! {
    todo!("0x684434 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
// 0x68450c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>> &,boost::_bi::list0 &,int)
pub fn stub_68450c() -> ! {
    todo!("0x68450c __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const")]
// 0x6845e0 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
// was: boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>::operator()(RBX::Tool*,rbx_core::WeakPtr<RBX::Network::Player>)const
pub fn stub_6845e0() -> ! {
    todo!("0x6845e0 __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x6846c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::WeakPtr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_6846c8() -> ! {
    todo!("0x6846c8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
// 0x684824 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
// was: boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)
pub fn stub_684824() -> ! {
    todo!("0x684824 __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_")
}

#[doc(alias = "boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace const*>>>::operator()(void)")]
// 0x6dea0c — __ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv
// was: boost::_bi::bind_t<double,boost::_mfi::cmf0<double,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace const*>>>::operator()(void)
pub fn stub_6dea0c() -> ! {
    todo!("0x6dea0c __ZN5boost3_bi6bind_tIdNS_4_mfi4cmf0IdN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPKS5_EEEEEclEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(lua_State *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>> const&)")]
// 0x773188 — __ZN3rbx7signals6signalIFvP9lua_StateEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(lua_State *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>> const&)
pub fn stub_773188() -> ! {
    todo!("0x773188 __ZN3rbx7signals6signalIFvP9lua_StateEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "bool RBX::Scripting::ScriptDebugger::withPausedThread<bool>(boost::function<bool ()(lua_State *,lua_Debug *)>)")]
// 0x773814 — __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIbEET_N5boost8functionIFS3_P9lua_StateP9lua_DebugEEE
// was: bool RBX::Scripting::ScriptDebugger::withPausedThread<bool>(boost::function<bool ()(lua_State *,lua_Debug *)>)
pub fn stub_773814() -> ! {
    todo!("0x773814 __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIbEET_N5boost8functionIFS3_P9lua_StateP9lua_DebugEEE")
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
// 0x777400 — __ZNK5boost9function2IbP9lua_StateP9lua_DebugEclES2_S4_
// was: boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const
pub fn stub_777400() -> ! {
    todo!("0x777400 __ZNK5boost9function2IbP9lua_StateP9lua_DebugEclES2_S4_")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_")]
// 0x7774cc — __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_
// was: __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_
pub fn stub_7774cc() -> ! {
    todo!("0x7774cc __ZN5boost8functionIFvP9lua_StateP9lua_DebugEEaSINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerES2_S4_NS0_IFbS2_S4_EEERbRNS_10shared_ptrISsEEEENS8_5list6INS8_5valueIPSE_EENS_3argILi1EEENSQ_ILi2EEENSN_ISG_EENS_17reference_wrapperIbEENSU_ISJ_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS6_E4typeES10_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)")]
// 0x7775e4 — __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS5_S7_EEERbRNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSG_ILi2EEESA_NS_17reference_wrapperIbEENSJ_ISD_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISO_T0_T1_T2_T3_T4_T5_EENSM_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSR_FSO_SS_ST_SU_SV_SW_ESZ_S10_S11_S12_S13_S14_
// was: boost::_bi::bind_t<void,boost::_mfi::mf5<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &>,boost::_bi::list_av_6<RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>::type> boost::bind<void,RBX::Scripting::ScriptDebugger,lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &,RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>>(void (RBX::Scripting::ScriptDebugger::*)(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool &,rbx_core::SharedPtr<std::string> &),RBX::Scripting::ScriptDebugger*,boost::arg<1>,boost::arg<2>,boost::function<bool ()(lua_State *,lua_Debug *)>,boost::reference_wrapper<bool>,boost::reference_wrapper<rbx_core::SharedPtr<std::string>>)
pub fn stub_7775e4() -> ! {
    todo!("0x7775e4 __ZN5boost4bindIvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFbS5_S7_EEERbRNS_10shared_ptrISsEEPS3_NS_3argILi1EEENSG_ILi2EEESA_NS_17reference_wrapperIbEENSJ_ISD_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf5ISO_T0_T1_T2_T3_T4_T5_EENSM_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEEMSR_FSO_SS_ST_SU_SV_SW_ESZ_S10_S11_S12_S13_S14_")
}

#[doc(alias = "void RBX::Scripting::ScriptDebugger::withPausedThreadHook<bool>(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool&,rbx_core::SharedPtr<std::string> &)")]
// 0x777708 — __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIbEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S4_S6_EEERS9_RNS7_10shared_ptrISsEE
// was: void RBX::Scripting::ScriptDebugger::withPausedThreadHook<bool>(lua_State *,lua_Debug *,boost::function<bool ()(lua_State *,lua_Debug *)>,bool&,rbx_core::SharedPtr<std::string> &)
pub fn stub_777708() -> ! {
    todo!("0x777708 __ZN3RBX9Scripting14ScriptDebugger20withPausedThreadHookIbEEvP9lua_StateP9lua_DebugN5boost8functionIFT_S4_S6_EEERS9_RNS7_10shared_ptrISsEE")
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&)")]
// 0x777a6c — __ZN5boost9function2IbP9lua_StateP9lua_DebugE13assign_to_ownERKS5_
// was: boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&)
pub fn stub_777a6c() -> ! {
    todo!("0x777a6c __ZN5boost9function2IbP9lua_StateP9lua_DebugE13assign_to_ownERKS5_")
}

#[doc(alias = "RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,std::string const&,float,bool)")]
// 0x79d5a8 — __ZN3RBX14PlayerChatLineC2ENS_8ChatLine8ChatTypeEN5boost10shared_ptrINS_7Network6PlayerEEERKSsfb
// was: RBX::PlayerChatLine::PlayerChatLine(RBX::ChatLine::ChatType,rbx_core::SharedPtr<RBX::Network::Player>,std::string const&,float,bool)
pub fn stub_79d5a8() -> ! {
    todo!("0x79d5a8 __ZN3RBX14PlayerChatLineC2ENS_8ChatLine8ChatTypeEN5boost10shared_ptrINS_7Network6PlayerEEERKSsfb")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
// 0x7a3bbc — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)
pub fn stub_7a3bbc() -> ! {
    todo!("0x7a3bbc __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")]
// 0x7a8d40 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_
// was: rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)
pub fn stub_7a8d40() -> ! {
    todo!("0x7a8d40 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7a8e5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_7a8e5c() -> ! {
    todo!("0x7a8e5c __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7a8e88 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_7a8e88() -> ! {
    todo!("0x7a8e88 __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
// 0x7a9078 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)
pub fn stub_7a9078() -> ! {
    todo!("0x7a9078 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
// 0x7a9080 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)
pub fn stub_7a9080() -> ! {
    todo!("0x7a9080 __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")]
// 0x7a9088 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)
pub fn stub_7a9088() -> ! {
    todo!("0x7a9088 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
// 0x7a9384 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()
pub fn stub_7a9384() -> ! {
    todo!("0x7a9384 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
// 0x7a93b0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()
pub fn stub_7a93b0() -> ! {
    todo!("0x7a93b0 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()")]
// 0x7c64f4 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED1Ev
// was: rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()
pub fn stub_7c64f4() -> ! {
    todo!("0x7c64f4 __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()")]
// 0x7c6604 — __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED0Ev
// was: rbx::signals::signal<void ()(RBX::Humanoid::Status)>::callable_slot<boost::function<void ()(RBX::Humanoid::Status)>>::~callable_slot()
pub fn stub_7c6604() -> ! {
    todo!("0x7c6604 __ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)")]
// 0x7c6850 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)
pub fn stub_7c6850() -> ! {
    todo!("0x7c6850 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)")]
// 0x7c6858 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::call(RBX::Humanoid::Status)
pub fn stub_7c6858() -> ! {
    todo!("0x7c6858 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const")]
// 0x7c6860 — __ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_
// was: boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const
pub fn stub_7c6860() -> ! {
    todo!("0x7c6860 __ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()")]
// 0x7c6b08 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()
pub fn stub_7c6b08() -> ! {
    todo!("0x7c6b08 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()")]
// 0x7c6c18 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()
pub fn stub_7c6c18() -> ! {
    todo!("0x7c6c18 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)")]
// 0x7c6e48 — __ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_
// was: boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)
pub fn stub_7c6e48() -> ! {
    todo!("0x7c6e48 __ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function1<void,float>::clear(void)")]
// 0x7c8ca0 — __ZN5boost9function1IvfE5clearEv
// was: boost::function1<void,float>::clear(void)
pub fn stub_7c8ca0() -> ! {
    todo!("0x7c8ca0 __ZN5boost9function1IvfE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::function<void ()(float)>>(boost::function<void ()(float)> const&)")]
// 0x7c93d0 — __ZN3rbx7signals6signalIFvfEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::function<void ()(float)>>(boost::function<void ()(float)> const&)
pub fn stub_7c93d0() -> ! {
    todo!("0x7c93d0 __ZN3rbx7signals6signalIFvfEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::callable<rbx::signals::signal<void ()(float)>*>(boost::function<void ()(float)> const&,rbx::signals::signal<void ()(float)>*)")]
// 0x7c94c4 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
// was: rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::callable<rbx::signals::signal<void ()(float)>*>(boost::function<void ()(float)> const&,rbx::signals::signal<void ()(float)>*)
pub fn stub_7c94c4() -> ! {
    todo!("0x7c94c4 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::function<void ()(float)>>::~callable_slot()")]
// 0x7c95c0 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED1Ev
// was: rbx::signals::signal<void ()(float)>::callable_slot<boost::function<void ()(float)>>::~callable_slot()
pub fn stub_7c95c0() -> ! {
    todo!("0x7c95c0 __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::function<void ()(float)>>::~callable_slot()")]
// 0x7c96d0 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED0Ev
// was: rbx::signals::signal<void ()(float)>::callable_slot<boost::function<void ()(float)>>::~callable_slot()
pub fn stub_7c96d0() -> ! {
    todo!("0x7c96d0 __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::call(float)")]
// 0x7c9800 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf
// was: rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::call(float)
pub fn stub_7c9800() -> ! {
    todo!("0x7c9800 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::call(float)")]
// 0x7c9808 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::call(float)
pub fn stub_7c9808() -> ! {
    todo!("0x7c9808 __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf")
}

#[doc(alias = "boost::function1<void,float>::operator()(float)const")]
// 0x7c9810 — __ZNK5boost9function1IvfEclEf
// was: boost::function1<void,float>::operator()(float)const
pub fn stub_7c9810() -> ! {
    todo!("0x7c9810 __ZNK5boost9function1IvfEclEf")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::~callable()")]
// 0x7c98d8 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::~callable()
pub fn stub_7c98d8() -> ! {
    todo!("0x7c98d8 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::~callable()")]
// 0x7c99e8 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::~callable()
pub fn stub_7c99e8() -> ! {
    todo!("0x7c99e8 __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev")
}

#[doc(alias = "boost::function1<void,float>::assign_to_own(boost::function1<void,float> const&)")]
// 0x7c9b18 — __ZN5boost9function1IvfE13assign_to_ownERKS1_
// was: boost::function1<void,float>::assign_to_own(boost::function1<void,float> const&)
pub fn stub_7c9b18() -> ! {
    todo!("0x7c9b18 __ZN5boost9function1IvfE13assign_to_ownERKS1_")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::simulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)")]
// 0x7cf624 — __ZN3RBX5HUMAN13HumanoidState8simulateERN5boost10shared_ptrIS1_EEf
// was: RBX::HUMAN::HumanoidState::simulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)
pub fn stub_7cf624() -> ! {
    todo!("0x7cf624 __ZN3RBX5HUMAN13HumanoidState8simulateERN5boost10shared_ptrIS1_EEf")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::doSimulatorStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)")]
// 0x7cf76c — __ZN3RBX5HUMAN13HumanoidState21doSimulatorStateTableERN5boost10shared_ptrIS1_EEf
// was: RBX::HUMAN::HumanoidState::doSimulatorStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,float)
pub fn stub_7cf76c() -> ! {
    todo!("0x7cf76c __ZN3RBX5HUMAN13HumanoidState21doSimulatorStateTableERN5boost10shared_ptrIS1_EEf")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::noSimulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &)")]
// 0x7cf838 — __ZN3RBX5HUMAN13HumanoidState10noSimulateERN5boost10shared_ptrIS1_EE
// was: RBX::HUMAN::HumanoidState::noSimulate(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &)
pub fn stub_7cf838() -> ! {
    todo!("0x7cf838 __ZN3RBX5HUMAN13HumanoidState10noSimulateERN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::doSlaveStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)")]
// 0x7cf8cc — __ZN3RBX5HUMAN13HumanoidState17doSlaveStateTableERN5boost10shared_ptrIS1_EENS0_9StateTypeE
// was: RBX::HUMAN::HumanoidState::doSlaveStateTable(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)
pub fn stub_7cf8cc() -> ! {
    todo!("0x7cf8cc __ZN3RBX5HUMAN13HumanoidState17doSlaveStateTableERN5boost10shared_ptrIS1_EENS0_9StateTypeE")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::changeState(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)")]
// 0x7cf96c — __ZN3RBX5HUMAN13HumanoidState11changeStateERN5boost10shared_ptrIS1_EENS0_9StateTypeE
// was: RBX::HUMAN::HumanoidState::changeState(rbx_core::SharedPtr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)
pub fn stub_7cf96c() -> ! {
    todo!("0x7cf96c __ZN3RBX5HUMAN13HumanoidState11changeStateERN5boost10shared_ptrIS1_EENS0_9StateTypeE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>> const&)")]
// 0x7d14d4 — __ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>> const&)
pub fn stub_7d14d4() -> ! {
    todo!("0x7d14d4 __ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7d1800 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_7d1800() -> ! {
    todo!("0x7d1800 __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()")]
// 0x7d182c — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_7d182c() -> ! {
    todo!("0x7d182c __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0x7d1900 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
pub fn stub_7d1900() -> ! {
    todo!("0x7d1900 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
// 0x7d1924 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
// was: `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
pub fn stub_7d1924() -> ! {
    todo!("0x7d1924 __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)")]
// 0x7d1948 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)
pub fn stub_7d1948() -> ! {
    todo!("0x7d1948 __ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()")]
// 0x7d1968 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()
pub fn stub_7d1968() -> ! {
    todo!("0x7d1968 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()")]
// 0x7d1994 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()
pub fn stub_7d1994() -> ! {
    todo!("0x7d1994 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")
}

#[doc(alias = "RBX::TextService::registerTypesetter(RBX::TextService::Font,rbx_core::SharedPtr<RBX::Typesetter>)")]
// 0x7d8e48 — __ZN3RBX11TextService18registerTypesetterENS0_4FontEN5boost10shared_ptrINS_10TypesetterEEE
// was: RBX::TextService::registerTypesetter(RBX::TextService::Font,rbx_core::SharedPtr<RBX::Typesetter>)
pub fn stub_7d8e48() -> ! {
    todo!("0x7d8e48 __ZN3RBX11TextService18registerTypesetterENS0_4FontEN5boost10shared_ptrINS_10TypesetterEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Typesetter>::operator=(rbx_core::SharedPtr<RBX::Typesetter> const&)")]
// 0x7d9cb8 — __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_
// was: rbx_core::SharedPtr<RBX::Typesetter>::operator=(rbx_core::SharedPtr<RBX::Typesetter> const&)
pub fn stub_7d9cb8() -> ! {
    todo!("0x7d9cb8 __ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_")
}

#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::filtering_stream(void)")]
// 0x7ddd4c — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEC1Ev
// was: boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::filtering_stream(void)
pub fn stub_7ddd4c() -> ! {
    todo!("0x7ddd4c __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEC1Ev")
}

#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7dde5c — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
// was: boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7dde5c() -> ! {
    todo!("0x7dde5c __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)")]
// 0x7ddf24 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)
pub fn stub_7ddf24() -> ! {
    todo!("0x7ddf24 __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)")]
// 0x7de024 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)
pub fn stub_7de024() -> ! {
    todo!("0x7de024 __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_")
}

#[doc(alias = "void boost::iostreams::close<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,std::_Ios_Openmode)")]
// 0x7de0ec — __ZN5boost9iostreams5closeINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEvRT_St13_Ios_Openmode
// was: void boost::iostreams::close<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,std::_Ios_Openmode)
pub fn stub_7de0ec() -> ! {
    todo!("0x7de0ec __ZN5boost9iostreams5closeINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pop(void)")]
// 0x7de110 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pop(void)
pub fn stub_7de110() -> ! {
    todo!("0x7de110 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::close(void)")]
// 0x7de180 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::close(void)
pub fn stub_7de180() -> ! {
    todo!("0x7de180 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)")]
// 0x7de348 — __ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)
pub fn stub_7de348() -> ! {
    todo!("0x7de348 __ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)")]
// 0x7de490 — __ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)
pub fn stub_7de490() -> ! {
    todo!("0x7de490 __ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7de5b8 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED1Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7de5b8() -> ! {
    todo!("0x7de5b8 __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED1Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7de5bc — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7de5bc() -> ! {
    todo!("0x7de5bc __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7de728 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED0Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7de728() -> ! {
    todo!("0x7de728 __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
// 0x7de7c8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_7de7c8() -> ! {
    todo!("0x7de7c8 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E5imbueERKSt6locale")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7de7f4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7de7f4() -> ! {
    todo!("0x7de7f4 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x7de80c — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_7de80c() -> ! {
    todo!("0x7de80c __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
// 0x7de840 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_7de840() -> ! {
    todo!("0x7de840 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
// 0x7de900 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_7de900() -> ! {
    todo!("0x7de900 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9underflowEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
// 0x7de958 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_7de958() -> ! {
    todo!("0x7de958 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9pbackfailEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
// 0x7dea70 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_7dea70() -> ! {
    todo!("0x7dea70 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8overflowEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7deadc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7deadc() -> ! {
    todo!("0x7deadc __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8set_nextEPNS1_16linked_streambufIcS7_EE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
// 0x7deae0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_7deae0() -> ! {
    todo!("0x7deae0 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10close_implESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
// 0x7deb04 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_7deb04() -> ! {
    todo!("0x7deb04 __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10auto_closeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
// 0x7deb10 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_7deb10() -> ! {
    todo!("0x7deb10 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14set_auto_closeEb")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
// 0x7deb24 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_7deb24() -> ! {
    todo!("0x7deb24 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E11strict_syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
// 0x7debec — __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_7debec() -> ! {
    todo!("0x7debec __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_typeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
// 0x7debfc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)
pub fn stub_7debfc() -> ! {
    todo!("0x7debfc __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_implEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
// 0x7dec00 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)
pub fn stub_7dec00() -> ! {
    todo!("0x7dec00 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_get_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
// 0x7dec0c — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)
pub fn stub_7dec0c() -> ! {
    todo!("0x7dec0c __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_put_areaEv")
}

#[doc(alias = "void boost::throw_exception<std::ios_base::failure>(std::ios_base::failure const&)")]
// 0x7dec30 — __ZN5boost15throw_exceptionINSt8ios_base7failureEEEvRKT_
// was: void boost::throw_exception<std::ios_base::failure>(std::ios_base::failure const&)
pub fn stub_7dec30() -> ! {
    todo!("0x7dec30 __ZN5boost15throw_exceptionINSt8ios_base7failureEEEvRKT_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
// 0x7ded0c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_7ded0c() -> ! {
    todo!("0x7ded0c __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
// 0x7ded1c — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7ded1c() -> ! {
    todo!("0x7ded1c __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
// 0x7ded20 — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7ded20() -> ! {
    todo!("0x7ded20 __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev")
}
