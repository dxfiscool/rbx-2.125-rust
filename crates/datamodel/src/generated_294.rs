// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|RBX::Workspace|RBX::Part|RBX::Model|RBX::Humanoid|RBX::Script|RBX::Players|RBX::Lighting (EA-sorted asc, NOT in global /tmp/global_eas.txt)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xf47114..0xf5de34 | total filtered 13623, remaining 136 (36 after batch)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xf47114 — j___ZN3RBX12PlatformImplINS_17BasicPartInstanceEE21createPlatformMotor6DEPNS_8HumanoidE
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::createPlatformMotor6D(RBX::Humanoid *)")]
pub fn stub_0xf47114() -> ! {
    todo!("0xf47114 RBX::PlatformImpl<RBX::BasicPartInstance>::createPlatformMotor6D(RBX::Humanoid *)")
}

// 0xf471f4 — j___ZN3RBX9weak_fromINS_13ModelInstanceEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::Weak<RBX::ModelInstance> RBX::weak_from<RBX::ModelInstance>(RBX::ModelInstance*)")]
// was: boost::weak_ptr<RBX::ModelInstance> RBX::weak_from<RBX::ModelInstance>(RBX::ModelInstance*)
pub fn stub_0xf471f4() -> ! {
    todo!("0xf471f4 rbx_core::Weak<RBX::ModelInstance> RBX::weak_from<RBX::ModelInstance>(RBX::ModelInstance*)")
}

// 0xf472c4 — j___ZN5boost10shared_ptrIN3RBX13ModelInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance>(rbx_core::Weak<RBX::ModelInstance> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance>(boost::weak_ptr<RBX::ModelInstance> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0xf472c4() -> ! {
    todo!("0xf472c4 rbx_core::SharedPtr<RBX::ModelInstance>::shared_ptr<RBX::ModelInstance>(rbx_core::Weak<RBX::ModelInstance> const&,boost::detail::sp_nothrow_tag)")
}

// 0xf47324 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EC2ES7_S7_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::list2(boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>)")]
// was: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>::list2(boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>)
pub fn stub_0xf47324() -> ! {
    todo!("0xf47324 boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::list2(boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>)")
}

// 0xf47384 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ModelInstanceEEEEES7_EC2ES7_S7_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>)
pub fn stub_0xf47384() -> ! {
    todo!("0xf47384 boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>)")
}

// 0xf473a4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13ModelInstanceEEES4_S4_S4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list_av_2<rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>>::type> boost::bind<void,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>>(void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list_av_2<boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>>::type> boost::bind<void,boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>>(void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>)
pub fn stub_0xf473a4() -> ! {
    todo!("0xf473a4 boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list_av_2<rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>>::type> boost::bind<void,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>>(void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>)")
}

// 0xf473e4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ModelInstanceEEES8_ENS3_5list2INS3_5valueIS8_EESD_EEEEE7managerERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>,boost::_bi::value<boost::weak_ptr<RBX::ModelInstance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf473e4() -> ! {
    todo!("0xf473e4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>,boost::_bi::value<rbx_core::Weak<RBX::ModelInstance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf47414 — j___ZN5boost8weak_ptrIN3RBX13ModelInstanceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::Weak<RBX::ModelInstance>::weak_ptr<RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const&,boost::detail::sp_enable_if_convertible<RBX::ModelInstance,RBX::ModelInstance>::type)")]
// was: boost::weak_ptr<RBX::ModelInstance>::weak_ptr<RBX::ModelInstance>(boost::shared_ptr<RBX::ModelInstance> const&,boost::detail::sp_enable_if_convertible<RBX::ModelInstance,RBX::ModelInstance>::type)
pub fn stub_0xf47414() -> ! {
    todo!("0xf47414 rbx_core::Weak<RBX::ModelInstance>::weak_ptr<RBX::ModelInstance>(rbx_core::SharedPtr<RBX::ModelInstance> const&,boost::detail::sp_enable_if_convertible<RBX::ModelInstance,RBX::ModelInstance>::type)")
}

// 0xf48534 — j___ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_0xf48534() -> ! {
    todo!("0xf48534 RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0xf48544 — j___ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_0xf48544() -> ! {
    todo!("0xf48544 RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0xf48554 — j___ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_0xf48554() -> ! {
    todo!("0xf48554 RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0xf48564 — j___ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_0xf48564() -> ! {
    todo!("0xf48564 RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0xf48574 — j___ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_0xf48574() -> ! {
    todo!("0xf48574 RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0xf48584 — j___ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfEC2IMNS_12PartInstanceEKFfS1_EMS4_FvS1_fEEEPKcSA_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityENS_8Security11PermissionsE
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")]
pub fn stub_0xf48584() -> ! {
    todo!("0xf48584 RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::SurfacePropDescriptor<float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float)>(char const*,char const*,float (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,float),RBX::Reflection::PropertyDescriptor::Functionality,RBX::Security::Permissions)")
}

// 0xf48594 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf48594() -> ! {
    todo!("0xf48594 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf485a4 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf485a4() -> ! {
    todo!("0xf485a4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf485b4 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf485b4() -> ! {
    todo!("0xf485b4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf485c4 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf485c4() -> ! {
    todo!("0xf485c4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf485d4 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf485d4() -> ! {
    todo!("0xf485d4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf485e4 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf485e4() -> ! {
    todo!("0xf485e4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf485f4 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf485f4() -> ! {
    todo!("0xf485f4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf48604 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf48604() -> ! {
    todo!("0xf48604 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf48614 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf48614() -> ! {
    todo!("0xf48614 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf48624 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf48624() -> ! {
    todo!("0xf48624 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf48634 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEEC2IMNS_12PartInstanceEKFS2_S1_EMS5_FvS1_S2_EEEPKcSB_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf48634() -> ! {
    todo!("0xf48634 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::SurfaceEnumPropDescriptor<RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType)>(char const*,char const*,RBX::SurfaceType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::SurfaceType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf48644 — j___ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0xf48644() -> ! {
    todo!("0xf48644 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")
}

// 0xf49604 — j___ZN3RBX14TouchDebouncer5checkERKN5boost10shared_ptrINS_12PartInstanceEEENS_9TouchPair4TypeE
#[doc(alias = "RBX::TouchDebouncer::check(rbx_core::SharedPtr<RBX::PartInstance> const&,RBX::TouchPair::Type)")]
// was: RBX::TouchDebouncer::check(boost::shared_ptr<RBX::PartInstance> const&,RBX::TouchPair::Type)
pub fn stub_0xf49604() -> ! {
    todo!("0xf49604 RBX::TouchDebouncer::check(rbx_core::SharedPtr<RBX::PartInstance> const&,RBX::TouchPair::Type)")
}

// 0xf4aff4 — j___ZN3RBX10Reflection14PropDescriptorINS_11VehicleSeatEbEC2IMNS_8SeatImplINS_12PartInstanceEEEKFRKbvEMS7_FvS9_EEEPKcSF_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::PropDescriptor<bool const& (RBX::SeatImpl<RBX::PartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::PartInstance>::*)(bool const&)>(char const*,char const*,bool const& (RBX::SeatImpl<RBX::PartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::PartInstance>::*)(bool const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xf4aff4() -> ! {
    todo!("0xf4aff4 RBX::Reflection::PropDescriptor<RBX::VehicleSeat,bool>::PropDescriptor<bool const& (RBX::SeatImpl<RBX::PartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::PartInstance>::*)(bool const&)>(char const*,char const*,bool const& (RBX::SeatImpl<RBX::PartInstance>::*)(void)const,void (RBX::SeatImpl<RBX::PartInstance>::*)(bool const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf4b084 — j___ZN3RBX13ActionStationINS_12PartInstanceEEC2Ev
#[doc(alias = "RBX::ActionStation<RBX::PartInstance>::ActionStation(void)")]
pub fn stub_0xf4b084() -> ! {
    todo!("0xf4b084 RBX::ActionStation<RBX::PartInstance>::ActionStation(void)")
}

// 0xf4b094 — j___ZN3RBX13ActionStationINS_12PartInstanceEED0Ev
#[doc(alias = "RBX::ActionStation<RBX::PartInstance>::~ActionStation()")]
pub fn stub_0xf4b094() -> ! {
    todo!("0xf4b094 RBX::ActionStation<RBX::PartInstance>::~ActionStation()")
}

// 0xf4b124 — j___ZN3RBX8SeatImplINS_12PartInstanceEE11setDisabledERKb
#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::setDisabled(bool const&)")]
pub fn stub_0xf4b124() -> ! {
    todo!("0xf4b124 RBX::SeatImpl<RBX::PartInstance>::setDisabled(bool const&)")
}

// 0xf4b134 — j___ZN3RBX8SeatImplINS_12PartInstanceEE12findSeatWeldEv
#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::findSeatWeld(void)")]
pub fn stub_0xf4b134() -> ! {
    todo!("0xf4b134 RBX::SeatImpl<RBX::PartInstance>::findSeatWeld(void)")
}

// 0xf4b144 — j___ZN3RBX8SeatImplINS_12PartInstanceEE14createSeatWeldEPNS_8HumanoidE
#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::createSeatWeld(RBX::Humanoid *)")]
pub fn stub_0xf4b144() -> ! {
    todo!("0xf4b144 RBX::SeatImpl<RBX::PartInstance>::createSeatWeld(RBX::Humanoid *)")
}

// 0xf4b164 — j___ZN3RBX8SeatImplINS_12PartInstanceEE16humanoidFromWeldEPNS_4WeldE
#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::humanoidFromWeld(RBX::Weld *)")]
pub fn stub_0xf4b164() -> ! {
    todo!("0xf4b164 RBX::SeatImpl<RBX::PartInstance>::humanoidFromWeld(RBX::Weld *)")
}

// 0xf4b174 — j___ZN3RBX8SeatImplINS_12PartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0xf4b174() -> ! {
    todo!("0xf4b174 RBX::SeatImpl<RBX::PartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0xf4b184 — j___ZN3RBX8SeatImplINS_12PartInstanceEED2Ev
#[doc(alias = "RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")]
pub fn stub_0xf4b184() -> ! {
    todo!("0xf4b184 RBX::SeatImpl<RBX::PartInstance>::~SeatImpl()")
}

// 0xf4b1d4 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8SeatImplINSA_12PartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>> const&)")]
pub fn stub_0xf4b1d4() -> ! {
    todo!("0xf4b1d4 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>> const&)")
}

// 0xf4b234 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8SeatImplINS4_12PartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>::operator()(void)")]
pub fn stub_0xf4b234() -> ! {
    todo!("0xf4b234 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::SeatImpl<RBX::PartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::SeatImpl<RBX::PartInstance>*>>>::operator()(void)")
}

// 0xf4b5e4 — j___ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::Array(void)")]
pub fn stub_0xf4b5e4() -> ! {
    todo!("0xf4b5e4 G3D::Array<RBX::PartInstance *,10,32ul>::Array(void)")
}

// 0xf4b5f4 — j___ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::~Array()")]
pub fn stub_0xf4b5f4() -> ! {
    todo!("0xf4b5f4 G3D::Array<RBX::PartInstance *,10,32ul>::~Array()")
}

// 0xf4bd24 — j___ZN5boost10shared_ptrIN3RBX12PartInstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::operator=(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: boost::shared_ptr<RBX::PartInstance>::operator=(boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_0xf4bd24() -> ! {
    todo!("0xf4bd24 rbx_core::SharedPtr<RBX::PartInstance>::operator=(rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

// 0xf4c064 — j___ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_allocate(unsigned long)
pub fn stub_0xf4c064() -> ! {
    todo!("0xf4c064 std::_Vector_base<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_allocate(unsigned long)")
}

// 0xf4c094 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX12PartInstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::PartInstance> *,rbx_core::SharedPtr<RBX::PartInstance> *>(rbx_core::SharedPtr<RBX::PartInstance> *,rbx_core::SharedPtr<RBX::PartInstance> *,rbx_core::SharedPtr<RBX::PartInstance> *)")]
// was: boost::shared_ptr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::PartInstance> *,boost::shared_ptr<RBX::PartInstance> *>(boost::shared_ptr<RBX::PartInstance> *,boost::shared_ptr<RBX::PartInstance> *,boost::shared_ptr<RBX::PartInstance> *)
pub fn stub_0xf4c094() -> ! {
    todo!("0xf4c094 rbx_core::SharedPtr<RBX::PartInstance> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::PartInstance> *,rbx_core::SharedPtr<RBX::PartInstance> *>(rbx_core::SharedPtr<RBX::PartInstance> *,rbx_core::SharedPtr<RBX::PartInstance> *,rbx_core::SharedPtr<RBX::PartInstance> *)")
}

// 0xf4c0d4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::PartInstance>*,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>>,unsigned long,rbx_core::SharedPtr<RBX::PartInstance> const&)")]
// was: std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::PartInstance>*,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>>,unsigned long,boost::shared_ptr<RBX::PartInstance> const&)
pub fn stub_0xf4c0d4() -> ! {
    todo!("0xf4c0d4 std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::PartInstance>*,std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>>,unsigned long,rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

// 0xf4c0e4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE15_M_erase_at_endEPS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::PartInstance>*)")]
// was: std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_erase_at_end(boost::shared_ptr<RBX::PartInstance>*)
pub fn stub_0xf4c0e4() -> ! {
    todo!("0xf4c0e4 std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::PartInstance>*)")
}

// 0xf4c0f4 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE6resizeEmS4_
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::PartInstance>)")]
// was: std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::resize(unsigned long,boost::shared_ptr<RBX::PartInstance>)
pub fn stub_0xf4c0f4() -> ! {
    todo!("0xf4c0f4 std::vector<rbx_core::SharedPtr<RBX::PartInstance>,std::allocator<rbx_core::SharedPtr<RBX::PartInstance>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::PartInstance>)")
}

// 0xf4c154 — j___ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX12PartInstanceEEEmS4_EvT_T0_RKT1_St12__false_type
#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::PartInstance> *,unsigned long,rbx_core::SharedPtr<RBX::PartInstance>>(rbx_core::SharedPtr<RBX::PartInstance> *,unsigned long,rbx_core::SharedPtr<RBX::PartInstance> const&,std::__false_type)")]
// was: void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::PartInstance> *,unsigned long,boost::shared_ptr<RBX::PartInstance>>(boost::shared_ptr<RBX::PartInstance> *,unsigned long,boost::shared_ptr<RBX::PartInstance> const&,std::__false_type)
pub fn stub_0xf4c154() -> ! {
    todo!("0xf4c154 void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::PartInstance> *,unsigned long,rbx_core::SharedPtr<RBX::PartInstance>>(rbx_core::SharedPtr<RBX::PartInstance> *,unsigned long,rbx_core::SharedPtr<RBX::PartInstance> const&,std::__false_type)")
}

// 0xf520b4 — j___ZN3RBX9weak_fromINS_12PartInstanceEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::Weak<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)")]
// was: boost::weak_ptr<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)
pub fn stub_0xf520b4() -> ! {
    todo!("0xf520b4 rbx_core::Weak<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)")
}

// 0xf529a4 — j___ZN3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xf529a4() -> ! {
    todo!("0xf529a4 RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf52ad4 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_12PartInstanceEEEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::PartInstance>>(void)")]
pub fn stub_0xf52ad4() -> ! {
    todo!("0xf52ad4 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FilteredSelection<RBX::PartInstance>>(void)")
}

// 0xf52b04 — j___ZN3RBX17FilteredSelectionINS_12PartInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::onSelectionChanged(RBX::SelectionChanged const&)")]
pub fn stub_0xf52b04() -> ! {
    todo!("0xf52b04 RBX::FilteredSelection<RBX::PartInstance>::onSelectionChanged(RBX::SelectionChanged const&)")
}

// 0xf52b14 — j___ZN3RBX17FilteredSelectionINS_12PartInstanceEEC2Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::FilteredSelection(void)")]
pub fn stub_0xf52b14() -> ! {
    todo!("0xf52b14 RBX::FilteredSelection<RBX::PartInstance>::FilteredSelection(void)")
}

// 0xf52b24 — j___ZN3RBX17FilteredSelectionINS_12PartInstanceEED0Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
pub fn stub_0xf52b24() -> ! {
    todo!("0xf52b24 RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")
}

// 0xf52b34 — j___ZN3RBX17FilteredSelectionINS_12PartInstanceEED2Ev
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")]
pub fn stub_0xf52b34() -> ! {
    todo!("0xf52b34 RBX::FilteredSelection<RBX::PartInstance>::~FilteredSelection()")
}

// 0xf52b44 — j___ZN3RBX24shared_from_dynamic_castINS_12PartInstanceENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> RBX::shared_from_dynamic_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
// was: boost::shared_ptr<RBX::PartInstance> RBX::shared_from_dynamic_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
pub fn stub_0xf52b44() -> ! {
    todo!("0xf52b44 rbx_core::SharedPtr<RBX::PartInstance> RBX::shared_from_dynamic_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")
}

// 0xf52d84 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSEPS9_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot*)")]
pub fn stub_0xf52d84() -> ! {
    todo!("0xf52d84 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot*)")
}

// 0xf52d94 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSERKSA_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> const&)")]
pub fn stub_0xf52d94() -> ! {
    todo!("0xf52d94 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> const&)")
}

// 0xf52da4 — j___ZN5boost20dynamic_pointer_castIN3RBX12PartInstanceENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
// was: boost::shared_ptr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)
pub fn stub_0xf52da4() -> ! {
    todo!("0xf52da4 rbx_core::SharedPtr<RBX::PartInstance> boost::dynamic_pointer_cast<RBX::PartInstance,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")
}

// 0xf52de4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8HumanoidEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Humanoid>,boost::_bi::list1<boost::_bi::value<RBX::Humanoid*>>>::operator()(void)")]
pub fn stub_0xf52de4() -> ! {
    todo!("0xf52de4 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Humanoid>,boost::_bi::list1<boost::_bi::value<RBX::Humanoid*>>>::operator()(void)")
}

// 0xf52df4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_8Humanoid6StatusEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Humanoid::Status>(RBX::Humanoid::Status &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Humanoid::Status>(RBX::Humanoid::Status &)
pub fn stub_0xf52df4() -> ! {
    todo!("0xf52df4 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<RBX::Humanoid::Status>(RBX::Humanoid::Status &)")
}

// 0xf52e14 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_8Humanoid6StatusENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Humanoid::Status const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Humanoid::Status const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub fn stub_0xf52e14() -> ! {
    todo!("0xf52e14 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::Humanoid::Status const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")
}

// 0xf52e94 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8Humanoid6StatusEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf52e94() -> ! {
    todo!("0xf52e94 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf52ed4 — j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)")]
pub fn stub_0xf52ed4() -> ! {
    todo!("0xf52ed4 boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)")
}

// 0xf52ee4 — j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE5clearEv
#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::clear(void)")]
pub fn stub_0xf52ee4() -> ! {
    todo!("0xf52ee4 boost::function1<void,RBX::Humanoid::Status>::clear(void)")
}

// 0xf52ef4 — j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub fn stub_0xf52ef4() -> ! {
    todo!("0xf52ef4 void boost::function1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")
}

// 0xf52f74 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_8HumanoidENS2_13NameOcclusionEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Humanoid,RBX::Humanoid::NameOcclusion>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0xf52f74() -> ! {
    todo!("0xf52f74 RBX::Reflection::EnumPropDescriptor<RBX::Humanoid,RBX::Humanoid::NameOcclusion>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf52f84 — j___ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToIndex(RBX::Humanoid::NameOcclusion)const")]
pub fn stub_0xf52f84() -> ! {
    todo!("0xf52f84 RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToIndex(RBX::Humanoid::NameOcclusion)const")
}

// 0xf52f94 — j___ZNK3RBX13ModelInstance28findConstFirstModifierOfTypeINS_8HumanoidEEEPKT_v
#[doc(alias = "RBX::Humanoid const* RBX::ModelInstance::findConstFirstModifierOfType<RBX::Humanoid>(void)const")]
pub fn stub_0xf52f94() -> ! {
    todo!("0xf52f94 RBX::Humanoid const* RBX::ModelInstance::findConstFirstModifierOfType<RBX::Humanoid>(void)const")
}

// 0xf52fb4 — j___ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_12PartInstanceEEEEEPT_v
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PartInstance>>(void)const")]
pub fn stub_0xf52fb4() -> ! {
    todo!("0xf52fb4 RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::find<RBX::FilteredSelection<RBX::PartInstance>>(void)const")
}

// 0xf52fc4 — j___ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_12PartInstanceEEEEEPT_v
#[doc(alias = "RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PartInstance>>(void)const")]
pub fn stub_0xf52fc4() -> ! {
    todo!("0xf52fc4 RBX::FilteredSelection<RBX::PartInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PartInstance>>(void)const")
}

// 0xf53024 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17FilteredSelectionINS1_12PartInstanceEEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PartInstance>,RBX::FilteredSelection<RBX::PartInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PartInstance>> const*,RBX::FilteredSelection<RBX::PartInstance> *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PartInstance>,RBX::FilteredSelection<RBX::PartInstance>>(boost::shared_ptr<RBX::FilteredSelection<RBX::PartInstance>> const*,RBX::FilteredSelection<RBX::PartInstance> *)const
pub fn stub_0xf53024() -> ! {
    todo!("0xf53024 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FilteredSelection<RBX::PartInstance>,RBX::FilteredSelection<RBX::PartInstance>>(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::PartInstance>> const*,RBX::FilteredSelection<RBX::PartInstance> *)const")
}

// 0xf53054 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8HumanoidES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Humanoid,RBX::Humanoid>(rbx_core::SharedPtr<RBX::Humanoid> const*,RBX::Humanoid *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Humanoid,RBX::Humanoid>(boost::shared_ptr<RBX::Humanoid> const*,RBX::Humanoid *)const
pub fn stub_0xf53054() -> ! {
    todo!("0xf53054 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Humanoid,RBX::Humanoid>(rbx_core::SharedPtr<RBX::Humanoid> const*,RBX::Humanoid *)const")
}

// 0xf53074 — j___ZNK5boost6detail8function13basic_vtable1IvN3RBX8Humanoid6StatusEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf53074() -> ! {
    todo!("0xf53074 void boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf53084 — j___ZNK5boost6detail8function13basic_vtable1IvN3RBX8Humanoid6StatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf53084() -> ! {
    todo!("0xf53084 bool boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0xf53094 — j___ZNK5boost6detail8function13basic_vtable1IvN3RBX8Humanoid6StatusEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS3_10Reflection18GenericSlotWrapperERKS5_EENS8_5list2INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf53094() -> ! {
    todo!("0xf53094 bool boost::detail::function::basic_vtable1<void,RBX::Humanoid::Status>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,RBX::Humanoid::Status const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf530d4 — j___ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_
#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const")]
pub fn stub_0xf530d4() -> ! {
    todo!("0xf530d4 boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const")
}

// 0xf530f4 — j___ZNSt12_Vector_baseIN3RBX8Humanoid13NameOcclusionESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_allocate(unsigned long)")]
pub fn stub_0xf530f4() -> ! {
    todo!("0xf530f4 std::_Vector_base<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_allocate(unsigned long)")
}

// 0xf53104 — j___ZNSt12_Vector_baseIN3RBX8Humanoid6StatusESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_allocate(unsigned long)")]
pub fn stub_0xf53104() -> ! {
    todo!("0xf53104 std::_Vector_base<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_allocate(unsigned long)")
}

// 0xf53114 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid13NameOcclusionES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Humanoid::NameOcclusion * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *>(RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *)")]
pub fn stub_0xf53114() -> ! {
    todo!("0xf53114 RBX::Humanoid::NameOcclusion * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *>(RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *)")
}

// 0xf53124 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid6StatusES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Humanoid::Status * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::Status *,RBX::Humanoid::Status *>(RBX::Humanoid::Status *,RBX::Humanoid::Status *,RBX::Humanoid::Status *)")]
pub fn stub_0xf53124() -> ! {
    todo!("0xf53124 RBX::Humanoid::Status * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::Status *,RBX::Humanoid::Status *>(RBX::Humanoid::Status *,RBX::Humanoid::Status *,RBX::Humanoid::Status *)")
}

// 0xf53134 — j___ZNSt3mapIPKN3RBX4NameENS0_8Humanoid13NameOcclusionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Humanoid::NameOcclusion,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0xf53134() -> ! {
    todo!("0xf53134 std::map<RBX::Name const*,RBX::Humanoid::NameOcclusion,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::operator[](RBX::Name const* const&)")
}

// 0xf53144 — j___ZNSt3mapIPKN3RBX4NameENS0_8Humanoid6StatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Humanoid::Status,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0xf53144() -> ! {
    todo!("0xf53144 std::map<RBX::Name const*,RBX::Humanoid::Status,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::operator[](RBX::Name const* const&)")
}

// 0xf53154 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,RBX::Humanoid::NameOcclusion const&)")]
pub fn stub_0xf53154() -> ! {
    todo!("0xf53154 std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,RBX::Humanoid::NameOcclusion const&)")
}

// 0xf53164 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,unsigned long,RBX::Humanoid::NameOcclusion const&)")]
pub fn stub_0xf53164() -> ! {
    todo!("0xf53164 std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,unsigned long,RBX::Humanoid::NameOcclusion const&)")
}

// 0xf53174 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::resize(unsigned long,RBX::Humanoid::NameOcclusion)")]
pub fn stub_0xf53174() -> ! {
    todo!("0xf53174 std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::resize(unsigned long,RBX::Humanoid::NameOcclusion)")
}

// 0xf53184 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::push_back(RBX::Humanoid::NameOcclusion const&)")]
pub fn stub_0xf53184() -> ! {
    todo!("0xf53184 std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::push_back(RBX::Humanoid::NameOcclusion const&)")
}

// 0xf53194 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,RBX::Humanoid::Status const&)")]
pub fn stub_0xf53194() -> ! {
    todo!("0xf53194 std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,RBX::Humanoid::Status const&)")
}

// 0xf531a4 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,unsigned long,RBX::Humanoid::Status const&)")]
pub fn stub_0xf531a4() -> ! {
    todo!("0xf531a4 std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,unsigned long,RBX::Humanoid::Status const&)")
}

// 0xf531b4 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::resize(unsigned long,RBX::Humanoid::Status)")]
pub fn stub_0xf531b4() -> ! {
    todo!("0xf531b4 std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::resize(unsigned long,RBX::Humanoid::Status)")
}

// 0xf531c4 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::push_back(RBX::Humanoid::Status const&)")]
pub fn stub_0xf531c4() -> ! {
    todo!("0xf531c4 std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::push_back(RBX::Humanoid::Status const&)")
}

// 0xf531f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
pub fn stub_0xf531f4() -> ! {
    todo!("0xf531f4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")
}

// 0xf53204 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
pub fn stub_0xf53204() -> ! {
    todo!("0xf53204 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")
}

// 0xf53214 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
pub fn stub_0xf53214() -> ! {
    todo!("0xf53214 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")
}

// 0xf53224 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
pub fn stub_0xf53224() -> ! {
    todo!("0xf53224 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")
}

// 0xf53234 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
pub fn stub_0xf53234() -> ! {
    todo!("0xf53234 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")
}

// 0xf53244 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
pub fn stub_0xf53244() -> ! {
    todo!("0xf53244 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")
}

// 0xf56fe4 — j___ZN3RBX10Reflection17RefPropDescriptorINS_12TextureTrailENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::TextureTrail,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xf56fe4() -> ! {
    todo!("0xf56fe4 RBX::Reflection::RefPropDescriptor<RBX::TextureTrail,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::TextureTrail::*)(void)const,void (RBX::TextureTrail::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf57374 — j___ZN3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0xf57374() -> ! {
    todo!("0xf57374 RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf5de14 — j___ZN5boost9unordered6detail10copy_nodesISaINS1_8ptr_nodeISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEEEEE6createERKSB_
#[doc(alias = "boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>>::create(std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>> const&)")]
// was: boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>>::create(std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>> const&)
pub fn stub_0xf5de14() -> ! {
    todo!("0xf5de14 boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>>::create(std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>> const&)")
}

// 0xf5de34 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPN3RBX12PartInstanceENS_8weak_ptrIS6_EEEES7_SA_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeISB_EEEEEEEvNS0_15iterator_detail8iteratorISM_EERNS1_5tableISH_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>,RBX::PartInstance *,boost::weak_ptr<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,boost::weak_ptr<RBX::PartInstance>>>>> &)
pub fn stub_0xf5de34() -> ! {
    todo!("0xf5de34 void boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>,boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>,RBX::PartInstance *,rbx_core::Weak<RBX::PartInstance>,boost::hash<RBX::PartInstance *>,std::equal_to<RBX::PartInstance *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::PartInstance * const,rbx_core::Weak<RBX::PartInstance>>>>> &)")
}
