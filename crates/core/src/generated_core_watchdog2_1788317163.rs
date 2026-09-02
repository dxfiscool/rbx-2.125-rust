//! core watchdog2 1788317163 — 100 core stubs EA-sorted asc second shard after first core shard 1788317087.
//! Source: ida/export.json (85545 funcs) global EA asc not yet covered — next 100 uncovered after first shard (offset 100) 0x84c924..0xf1ef8c to avoid duplicate.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x84c924 — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbvELi0EEC2EMS2_FbvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(void),0>::BoundFuncDesc(bool (RBX::GameBasicSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_84c924() -> ! {
    todo!("0x84c924")
}

// 0x84ca28 — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(void),0>::~BoundFuncDesc()")]
pub fn stub_84ca28() -> ! {
    todo!("0x84ca28")
}

// 0x84cadc — __ZNK3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_84cadc() -> ! {
    todo!("0x84cadc")
}

// 0x84cb00 — __ZN3RBX10Reflection11Call0HelperINS_17GameBasicSettingsEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::GameBasicSettings,bool (RBX::GameBasicSettings::*)(void),bool>::call(RBX::GameBasicSettings*,bool (RBX::GameBasicSettings::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_84cb00() -> ! {
    todo!("0x84cb00")
}

// 0x84cb30 — __ZN3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,bool>::PropDescriptor<bool (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(bool)>(char const*,char const*,bool (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_84cb30() -> ! {
    todo!("0x84cb30")
}

// 0x84cc44 — __ZN3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,bool>::~PropDescriptor()")]
pub fn stub_84cc44() -> ! {
    todo!("0x84cc44")
}

// 0x84cc70 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,bool>::GetSetImpl<bool (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(bool)>::isReadOnly(void)const")]
pub fn stub_84cc70() -> ! {
    todo!("0x84cc70")
}

// 0x84cc74 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,bool>::GetSetImpl<bool (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_84cc74() -> ! {
    todo!("0x84cc74")
}

// 0x84cc78 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,bool>::GetSetImpl<bool (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_84cc78() -> ! {
    todo!("0x84cc78")
}

// 0x84cc9c — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,bool>::GetSetImpl<bool (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_84cc9c() -> ! {
    todo!("0x84cc9c")
}

// 0x84ccc0 — __ZN3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,std::string>::PropDescriptor<std::string (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(std::string)>(char const*,char const*,std::string (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_84ccc0() -> ! {
    todo!("0x84ccc0")
}

// 0x84cdd4 — __ZN3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,std::string>::~PropDescriptor()")]
pub fn stub_84cdd4() -> ! {
    todo!("0x84cdd4")
}

// 0x84ce00 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,std::string>::GetSetImpl<std::string (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(std::string)>::isReadOnly(void)const")]
pub fn stub_84ce00() -> ! {
    todo!("0x84ce00")
}

// 0x84ce04 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,std::string>::GetSetImpl<std::string (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(std::string)>::isWriteOnly(void)const")]
pub fn stub_84ce04() -> ! {
    todo!("0x84ce04")
}

// 0x84ce08 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,std::string>::GetSetImpl<std::string (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_84ce08() -> ! {
    todo!("0x84ce08")
}

// 0x84ce30 — __ZNK3RBX10Reflection14PropDescriptorINS_17GameBasicSettingsESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GameBasicSettings,std::string>::GetSetImpl<std::string (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_84ce30() -> ! {
    todo!("0x84ce30")
}

// 0x84cf74 — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFvSsbELi2EEC2EMS2_FvSsbEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,void ()(std::string,bool),2>::BoundFuncDesc(void (RBX::GameBasicSettings::*)(std::string,bool),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_84cf74() -> ! {
    todo!("0x84cf74")
}

// 0x84d13c — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFvSsbELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,void ()(std::string,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_84d13c() -> ! {
    todo!("0x84d13c")
}

// 0x84d188 — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFvSsbELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,void ()(std::string,bool),2>::~BoundFuncDesc()")]
pub fn stub_84d188() -> ! {
    todo!("0x84d188")
}

// 0x84d264 — __ZNK3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFvSsbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,void ()(std::string,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_84d264() -> ! {
    todo!("0x84d264")
}

// 0x84d3bc — __ZN3RBX10Reflection11Call2HelperINS_17GameBasicSettingsEMS2_FvSsbESsbvE4callEPS2_S4_RNS0_7VariantERKSsRKb
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::GameBasicSettings,void (RBX::GameBasicSettings::*)(std::string,bool),std::string,bool,void>::call(RBX::GameBasicSettings*,void (RBX::GameBasicSettings::*)(std::string,bool),RBX::Reflection::Variant &,std::string const&,bool const&)")]
pub fn stub_84d3bc() -> ! {
    todo!("0x84d3bc")
}

// 0x84d4f4 — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::GameBasicSettings::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_84d4f4() -> ! {
    todo!("0x84d4f4")
}

// 0x84d66c — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_84d66c() -> ! {
    todo!("0x84d66c")
}

// 0x84d69c — __ZN3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_84d69c() -> ! {
    todo!("0x84d69c")
}

// 0x84d768 — __ZNK3RBX10Reflection13BoundFuncDescINS_17GameBasicSettingsEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GameBasicSettings,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_84d768() -> ! {
    todo!("0x84d768")
}

// 0x84d8a8 — __ZN3RBX10Reflection11Call1HelperINS_17GameBasicSettingsEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GameBasicSettings,bool (RBX::GameBasicSettings::*)(std::string),std::string,bool>::call(RBX::GameBasicSettings*,bool (RBX::GameBasicSettings::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_84d8a8() -> ! {
    todo!("0x84d8a8")
}

// 0x9eaa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9eaa18() -> ! {
    todo!("0x9eaa18")
}

// 0x9eaa30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9eaa30() -> ! {
    todo!("0x9eaa30")
}

// 0x9eb598 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIdEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9eb598() -> ! {
    todo!("0x9eb598")
}

// 0x9eb5b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIdEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<double> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9eb5b0() -> ! {
    todo!("0x9eb5b0")
}

// 0x9ebad0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIbEENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9ebad0() -> ! {
    todo!("0x9ebad0")
}

// 0x9ebae8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIbEENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9ebae8() -> ! {
    todo!("0x9ebae8")
}

// 0x9ebd70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIyEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned long long> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9ebd70() -> ! {
    todo!("0x9ebd70")
}

// 0x9ebd78 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIyEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned long long> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_9ebd78() -> ! {
    todo!("0x9ebd78")
}

// 0xac60a0 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbRKiNS_10shared_ptrIS3_EENS_3argILi1EEENSA_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(bool const&,int const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_ac60a0() -> ! {
    todo!("0xac60a0")
}

// 0xac650c — __ZN3RBX10Reflection18GenericSlotWrapper8execute2IbiEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<bool,int>(bool const&,int const&)")]
pub fn stub_ac650c() -> ! {
    todo!("0xac650c")
}

// 0xac6c74 — __ZN5boost9function2IvbiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKbRKiEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
#[doc(alias = "void boost::function2<void,bool,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_ac6c74() -> ! {
    todo!("0xac6c74")
}

// 0xac70ec — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKbRKiEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_ac70ec() -> ! {
    todo!("0xac70ec")
}

// 0xac7110 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKbRKiEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEvbiE6invokeERNS1_15function_bufferEbi
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,bool,int>::invoke(boost::detail::function::function_buffer &,bool,int)")]
pub fn stub_ac7110() -> ! {
    todo!("0xac7110")
}

// 0xac713c — __ZNK5boost6detail8function13basic_vtable2IvbiE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKbRKiEENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,bool,int>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_ac713c() -> ! {
    todo!("0xac713c")
}

// 0xac7424 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKbRKiEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,bool const&,int const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_ac7424() -> ! {
    todo!("0xac7424")
}

// 0xf1c680 — _plcrash_async_macho_next_command_type
#[doc(alias = "_plcrash_async_macho_next_command_type")]
pub fn stub_f1c680() -> ! {
    todo!("0xf1c680")
}

// 0xf1c6a8 — _plcrash_async_macho_next_command
#[doc(alias = "_plcrash_async_macho_next_command")]
pub fn stub_f1c6a8() -> ! {
    todo!("0xf1c6a8")
}

// 0xf1c738 — _plcrash_async_macho_find_command
#[doc(alias = "_plcrash_async_macho_find_command")]
pub fn stub_f1c738() -> ! {
    todo!("0xf1c738")
}

// 0xf1c77c — _plcrash_async_macho_find_segment_cmd
#[doc(alias = "_plcrash_async_macho_find_segment_cmd")]
pub fn stub_f1c77c() -> ! {
    todo!("0xf1c77c")
}

// 0xf1c7c8 — _plcrash_async_macho_map_segment
#[doc(alias = "_plcrash_async_macho_map_segment")]
pub fn stub_f1c7c8() -> ! {
    todo!("0xf1c7c8")
}

// 0xf1c870 — _plcrash_async_macho_map_section
#[doc(alias = "_plcrash_async_macho_map_section")]
pub fn stub_f1c870() -> ! {
    todo!("0xf1c870")
}

// 0xf1c960 — _plcrash_async_macho_find_symbol
#[doc(alias = "_plcrash_async_macho_find_symbol")]
pub fn stub_f1c960() -> ! {
    todo!("0xf1c960")
}

// 0xf1cb78 — _plcrash_async_macho_find_symtab_symbol
#[doc(alias = "_plcrash_async_macho_find_symtab_symbol")]
pub fn stub_f1cb78() -> ! {
    todo!("0xf1cb78")
}

// 0xf1cc6c — _plcrash_async_macho_mapped_segment_free
#[doc(alias = "_plcrash_async_macho_mapped_segment_free")]
pub fn stub_f1cc6c() -> ! {
    todo!("0xf1cc6c")
}

// 0xf1cc78 — _plcrash_nasync_macho_free
#[doc(alias = "_plcrash_nasync_macho_free")]
pub fn stub_f1cc78() -> ! {
    todo!("0xf1cc78")
}

// 0xf1ccac — _plcrash_async_mobject_init
#[doc(alias = "_plcrash_async_mobject_init")]
pub fn stub_f1ccac() -> ! {
    todo!("0xf1ccac")
}

// 0xf1cd7c — _plcrash_async_mobject_verify_local_pointer
#[doc(alias = "_plcrash_async_mobject_verify_local_pointer")]
pub fn stub_f1cd7c() -> ! {
    todo!("0xf1cd7c")
}

// 0xf1cda4 — _plcrash_async_mobject_remap_address
#[doc(alias = "_plcrash_async_mobject_remap_address")]
pub fn stub_f1cda4() -> ! {
    todo!("0xf1cda4")
}

// 0xf1cdd4 — _plcrash_async_mobject_free
#[doc(alias = "_plcrash_async_mobject_free")]
pub fn stub_f1cdd4() -> ! {
    todo!("0xf1cdd4")
}

// 0xf1cdf8 — _plcrash_async_objc_cache_init
#[doc(alias = "_plcrash_async_objc_cache_init")]
pub fn stub_f1cdf8() -> ! {
    todo!("0xf1cdf8")
}

// 0xf1ce14 — _plcrash_async_objc_cache_free
#[doc(alias = "_plcrash_async_objc_cache_free")]
pub fn stub_f1ce14() -> ! {
    todo!("0xf1ce14")
}

// 0xf1ce40 — _free_mapped_sections
#[doc(alias = "_free_mapped_sections")]
pub fn stub_f1ce40() -> ! {
    todo!("0xf1ce40")
}

// 0xf1ce84 — _plcrash_async_objc_find_method
#[doc(alias = "_plcrash_async_objc_find_method")]
pub fn stub_f1ce84() -> ! {
    todo!("0xf1ce84")
}

// 0xf1ced4 — _plcrash_async_objc_parse
#[doc(alias = "_plcrash_async_objc_parse")]
pub fn stub_f1ced4() -> ! {
    todo!("0xf1ced4")
}

// 0xf1d1cc — _pl_async_objc_find_method_search_callback
#[doc(alias = "_pl_async_objc_find_method_search_callback")]
pub fn stub_f1d1cc() -> ! {
    todo!("0xf1d1cc")
}

// 0xf1d1e0 — _pl_async_objc_find_method_call_callback
#[doc(alias = "_pl_async_objc_find_method_call_callback")]
pub fn stub_f1d1e0() -> ! {
    todo!("0xf1d1e0")
}

// 0xf1d200 — _pl_async_objc_parse_objc2_class
#[doc(alias = "_pl_async_objc_parse_objc2_class")]
pub fn stub_f1d200() -> ! {
    todo!("0xf1d200")
}

// 0xf1d4ac — _pl_async_parse_obj1_class
#[doc(alias = "_pl_async_parse_obj1_class")]
pub fn stub_f1d4ac() -> ! {
    todo!("0xf1d4ac")
}

// 0xf1d5b0 — _plcrash_async_symbol_cache_init
#[doc(alias = "_plcrash_async_symbol_cache_init")]
pub fn stub_f1d5b0() -> ! {
    todo!("0xf1d5b0")
}

// 0xf1d5bc — _plcrash_async_symbol_cache_free
#[doc(alias = "_plcrash_async_symbol_cache_free")]
pub fn stub_f1d5bc() -> ! {
    todo!("0xf1d5bc")
}

// 0xf1d5c8 — _plcrash_async_find_symbol
#[doc(alias = "_plcrash_async_find_symbol")]
pub fn stub_f1d5c8() -> ! {
    todo!("0xf1d5c8")
}

// 0xf1d65c — _macho_symbol_callback
#[doc(alias = "_macho_symbol_callback")]
pub fn stub_f1d65c() -> ! {
    todo!("0xf1d65c")
}

// 0xf1d6a4 — _objc_symbol_callback
#[doc(alias = "_objc_symbol_callback")]
pub fn stub_f1d6a4() -> ! {
    todo!("0xf1d6a4")
}

// 0xf1d7a4 — _plcrash_async_macho_string_init
#[doc(alias = "_plcrash_async_macho_string_init")]
pub fn stub_f1d7a4() -> ! {
    todo!("0xf1d7a4")
}

// 0xf1d7b8 — _plcrash_async_macho_string_get_length
#[doc(alias = "_plcrash_async_macho_string_get_length")]
pub fn stub_f1d7b8() -> ! {
    todo!("0xf1d7b8")
}

// 0xf1d7d0 — _plcrash_async_macho_string_read
#[doc(alias = "_plcrash_async_macho_string_read")]
pub fn stub_f1d7d0() -> ! {
    todo!("0xf1d7d0")
}

// 0xf1d848 — _plcrash_async_macho_string_get_pointer
#[doc(alias = "_plcrash_async_macho_string_get_pointer")]
pub fn stub_f1d848() -> ! {
    todo!("0xf1d848")
}

// 0xf1d878 — _plcrash_async_macho_string_free
#[doc(alias = "_plcrash_async_macho_string_free")]
pub fn stub_f1d878() -> ! {
    todo!("0xf1d878")
}

// 0xf1de4c — _exception_server_thread
#[doc(alias = "_exception_server_thread")]
pub fn stub_f1de4c() -> ! {
    todo!("0xf1de4c")
}

// 0xf1e5c0 — _set_exception_ports
#[doc(alias = "_set_exception_ports")]
pub fn stub_f1e5c0() -> ! {
    todo!("0xf1e5c0")
}

// 0xf1e648 — _plcrash_async_allocator_new
#[doc(alias = "_plcrash_async_allocator_new")]
pub fn stub_f1e648() -> ! {
    todo!("0xf1e648")
}

// 0xf1e750 — _plcrash_async_allocator_alloc
#[doc(alias = "_plcrash_async_allocator_alloc")]
pub fn stub_f1e750() -> ! {
    todo!("0xf1e750")
}

// 0xf1e9d0 — _add_image_hook_ARC
#[doc(alias = "_add_image_hook_ARC")]
pub fn stub_f1e9d0() -> ! {
    todo!("0xf1e9d0")
}

// 0xf1e9e8 — ___arclite_NSArray_objectAtIndexedSubscript
#[doc(alias = "___arclite_NSArray_objectAtIndexedSubscript")]
pub fn stub_f1e9e8() -> ! {
    todo!("0xf1e9e8")
}

// 0xf1ea00 — ___arclite_NSMutableArray_setObject_atIndexedSubscript
#[doc(alias = "___arclite_NSMutableArray_setObject_atIndexedSubscript")]
pub fn stub_f1ea00() -> ! {
    todo!("0xf1ea00")
}

// 0xf1ea50 — ___arclite_NSDictionary_objectForKeyedSubscript
#[doc(alias = "___arclite_NSDictionary_objectForKeyedSubscript")]
pub fn stub_f1ea50() -> ! {
    todo!("0xf1ea50")
}

// 0xf1ea68 — ___arclite_NSMutableDictionary__setObject_forKeyedSubscript
#[doc(alias = "___arclite_NSMutableDictionary__setObject_forKeyedSubscript")]
pub fn stub_f1ea68() -> ! {
    todo!("0xf1ea68")
}

// 0xf1ea80 — ___arclite_NSOrderedSet_objectAtIndexedSubscript
#[doc(alias = "___arclite_NSOrderedSet_objectAtIndexedSubscript")]
pub fn stub_f1ea80() -> ! {
    todo!("0xf1ea80")
}

// 0xf1ea98 — ___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript
#[doc(alias = "___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript")]
pub fn stub_f1ea98() -> ! {
    todo!("0xf1ea98")
}

// 0xf1eab0 — ___arclite_objc_autoreleasePoolPop
#[doc(alias = "___arclite_objc_autoreleasePoolPop")]
pub fn stub_f1eab0() -> ! {
    todo!("0xf1eab0")
}

// 0xf1eac8 — _patch_lazy_pointers
#[doc(alias = "_patch_lazy_pointers")]
pub fn stub_f1eac8() -> ! {
    todo!("0xf1eac8")
}

// 0xf1ec64 — ___arclite_objc_autoreleasePoolPush
#[doc(alias = "___arclite_objc_autoreleasePoolPush")]
pub fn stub_f1ec64() -> ! {
    todo!("0xf1ec64")
}

// 0xf1ec88 — ___arclite_object_setInstanceVariable
#[doc(alias = "___arclite_object_setInstanceVariable")]
pub fn stub_f1ec88() -> ! {
    todo!("0xf1ec88")
}

// 0xf1ecc4 — ___arclite_object_setIvar
#[doc(alias = "___arclite_object_setIvar")]
pub fn stub_f1ecc4() -> ! {
    todo!("0xf1ecc4")
}

// 0xf1edac — ___arclite_object_copy
#[doc(alias = "___arclite_object_copy")]
pub fn stub_f1edac() -> ! {
    todo!("0xf1edac")
}

// 0xf1eeb8 — ___arclite_objc_retain
#[doc(alias = "___arclite_objc_retain")]
pub fn stub_f1eeb8() -> ! {
    todo!("0xf1eeb8")
}

// 0xf1eed0 — ___arclite_objc_retainBlock
#[doc(alias = "___arclite_objc_retainBlock")]
pub fn stub_f1eed0() -> ! {
    todo!("0xf1eed0")
}

// 0xf1eedc — ___arclite_objc_release
#[doc(alias = "___arclite_objc_release")]
pub fn stub_f1eedc() -> ! {
    todo!("0xf1eedc")
}

// 0xf1eef4 — ___arclite_objc_autorelease
#[doc(alias = "___arclite_objc_autorelease")]
pub fn stub_f1eef4() -> ! {
    todo!("0xf1eef4")
}

// 0xf1ef0c — ___arclite_objc_retainAutorelease
#[doc(alias = "___arclite_objc_retainAutorelease")]
pub fn stub_f1ef0c() -> ! {
    todo!("0xf1ef0c")
}

// 0xf1ef34 — ___arclite_objc_autoreleaseReturnValue
#[doc(alias = "___arclite_objc_autoreleaseReturnValue")]
pub fn stub_f1ef34() -> ! {
    todo!("0xf1ef34")
}

// 0xf1ef4c — ___arclite_objc_retainAutoreleaseReturnValue
#[doc(alias = "___arclite_objc_retainAutoreleaseReturnValue")]
pub fn stub_f1ef4c() -> ! {
    todo!("0xf1ef4c")
}

// 0xf1ef74 — ___arclite_objc_retainAutoreleasedReturnValue
#[doc(alias = "___arclite_objc_retainAutoreleasedReturnValue")]
pub fn stub_f1ef74() -> ! {
    todo!("0xf1ef74")
}

// 0xf1ef8c — ___arclite_objc_storeStrong
#[doc(alias = "___arclite_objc_storeStrong")]
pub fn stub_f1ef8c() -> ! {
    todo!("0xf1ef8c")
}

