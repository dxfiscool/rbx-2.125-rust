//! core shard CI — 100 core stubs EA-sorted, next uncovered after CH 0x6536c4 (strict RBX|boost|std|rbx earliest gap 0x6536c8).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
// 0x6536c8 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEED0Ev
pub fn stub_6536c8() -> ! {
    todo!("0x6536c8 __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")]
// 0x6536cc — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE7disposeEv
pub fn stub_6536cc() -> ! {
    todo!("0x6536cc __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
// 0x6536dc — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
pub fn stub_6536dc() -> ! {
    todo!("0x6536dc __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
// 0x6536e0 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
pub fn stub_6536e0() -> ! {
    todo!("0x6536e0 __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<void (*)(std::string *,std::exception *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x6536e8 — __ZN5boost6detail8function15functor_managerIPFvPSsPSt9exceptionEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
pub fn stub_6536e8() -> ! {
    todo!("0x6536e8 __ZN5boost6detail8function15functor_managerIPFvPSsPSt9exceptionEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "double const& rbx::any_cast<double const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x654440 — __ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_654440() -> ! {
    todo!("0x654440 __ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Stats::StatsService::~StatsService()")]
// 0x6561ac — __ZN3RBX5Stats12StatsServiceD2Ev
pub fn stub_6561ac() -> ! {
    todo!("0x6561ac __ZN3RBX5Stats12StatsServiceD2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x6562e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_6562e4() -> ! {
    todo!("0x6562e4 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x65631c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_65631c() -> ! {
    todo!("0x65631c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "RBX::StudioTool::setEnabled(bool)")]
// 0x65793c — __ZN3RBX10StudioTool10setEnabledEb
pub fn stub_65793c() -> ! {
    todo!("0x65793c __ZN3RBX10StudioTool10setEnabledEb")
}

#[doc(alias = "RBX::StudioTool::getEnabled(void)const")]
// 0x6579a4 — __ZNK3RBX10StudioTool10getEnabledEv
pub fn stub_6579a4() -> ! {
    todo!("0x6579a4 __ZNK3RBX10StudioTool10getEnabledEv")
}

#[doc(alias = "RBX::Surface::Surface(void)")]
// 0x6589f4 — __ZN3RBX7SurfaceC1Ev
pub fn stub_6589f4() -> ! {
    todo!("0x6589f4 __ZN3RBX7SurfaceC1Ev")
}

#[doc(alias = "RBX::Surface::setSurfaceType(RBX::SurfaceType)")]
// 0x658a00 — __ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE
pub fn stub_658a00() -> ! {
    todo!("0x658a00 __ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE")
}

#[doc(alias = "RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)")]
// 0x658a0c — __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE
pub fn stub_658a0c() -> ! {
    todo!("0x658a0c __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE")
}

#[doc(alias = "RBX::Surface::setParamA(float)")]
// 0x658a18 — __ZN3RBX7Surface9setParamAEf
pub fn stub_658a18() -> ! {
    todo!("0x658a18 __ZN3RBX7Surface9setParamAEf")
}

#[doc(alias = "RBX::Surface::setParamB(float)")]
// 0x658a24 — __ZN3RBX7Surface9setParamBEf
pub fn stub_658a24() -> ! {
    todo!("0x658a24 __ZN3RBX7Surface9setParamBEf")
}

#[doc(alias = "RBX::Surface::flat(void)")]
// 0x658a30 — __ZN3RBX7Surface4flatEv
pub fn stub_658a30() -> ! {
    todo!("0x658a30 __ZN3RBX7Surface4flatEv")
}

#[doc(alias = "RBX::Surface::registerSurfaceDescriptors(void)")]
// 0x658b6c — __ZN3RBX7Surface26registerSurfaceDescriptorsEv
pub fn stub_658b6c() -> ! {
    todo!("0x658b6c __ZN3RBX7Surface26registerSurfaceDescriptorsEv")
}

#[doc(alias = "RBX::Surface::getSurfaceTypeStatic(RBX::NormalId)")]
// 0x658be0 — __ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE
pub fn stub_658be0() -> ! {
    todo!("0x658be0 __ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE")
}

#[doc(alias = "RBX::Surface::getSurfaceInputStatic(RBX::NormalId)")]
// 0x658c70 — __ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE
pub fn stub_658c70() -> ! {
    todo!("0x658c70 __ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE")
}

#[doc(alias = "RBX::Surface::getParamAStatic(RBX::NormalId)")]
// 0x658d00 — __ZN3RBX7Surface15getParamAStaticENS_8NormalIdE
pub fn stub_658d00() -> ! {
    todo!("0x658d00 __ZN3RBX7Surface15getParamAStaticENS_8NormalIdE")
}

#[doc(alias = "RBX::Surface::getParamBStatic(RBX::NormalId)")]
// 0x658d90 — __ZN3RBX7Surface15getParamBStaticENS_8NormalIdE
pub fn stub_658d90() -> ! {
    todo!("0x658d90 __ZN3RBX7Surface15getParamBStaticENS_8NormalIdE")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x6590f4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev
pub fn stub_6590f4() -> ! {
    todo!("0x6590f4 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x659118 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev
pub fn stub_659118() -> ! {
    todo!("0x659118 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()")]
// 0x65913c — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev
pub fn stub_65913c() -> ! {
    todo!("0x65913c __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x659160 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev
pub fn stub_659160() -> ! {
    todo!("0x659160 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x659184 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev
pub fn stub_659184() -> ! {
    todo!("0x659184 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()")]
// 0x6591a8 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev
pub fn stub_6591a8() -> ! {
    todo!("0x6591a8 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x6591cc — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED1Ev
pub fn stub_6591cc() -> ! {
    todo!("0x6591cc __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x6591f0 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev
pub fn stub_6591f0() -> ! {
    todo!("0x6591f0 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::~SurfacePropDescriptor()")]
// 0x659214 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED1Ev
pub fn stub_659214() -> ! {
    todo!("0x659214 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x659238 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED1Ev
pub fn stub_659238() -> ! {
    todo!("0x659238 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x65925c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev
pub fn stub_65925c() -> ! {
    todo!("0x65925c __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::~SurfacePropDescriptor()")]
// 0x659280 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED1Ev
pub fn stub_659280() -> ! {
    todo!("0x659280 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x6592a4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED1Ev
pub fn stub_6592a4() -> ! {
    todo!("0x6592a4 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x6592c8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev
pub fn stub_6592c8() -> ! {
    todo!("0x6592c8 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::~SurfacePropDescriptor()")]
// 0x6592ec — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED1Ev
pub fn stub_6592ec() -> ! {
    todo!("0x6592ec __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x659310 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED1Ev
pub fn stub_659310() -> ! {
    todo!("0x659310 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x659334 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev
pub fn stub_659334() -> ! {
    todo!("0x659334 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::~SurfacePropDescriptor()")]
// 0x659358 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED1Ev
pub fn stub_659358() -> ! {
    todo!("0x659358 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED1Ev")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)2,float>::~SurfacePropDescriptor()")]
// 0x659490 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED0Ev
pub fn stub_659490() -> ! {
    todo!("0x659490 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE2EfED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x6595b4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev
pub fn stub_6595b4() -> ! {
    todo!("0x6595b4 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isReadOnly(void)const")]
// 0x6595e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv
pub fn stub_6595e0() -> ! {
    todo!("0x6595e0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
// 0x6595f0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
pub fn stub_6595f0() -> ! {
    todo!("0x6595f0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::hasStringValue(void)const")]
// 0x6597cc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv
pub fn stub_6597cc() -> ! {
    todo!("0x6597cc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x659e30 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED0Ev
pub fn stub_659e30() -> ! {
    todo!("0x659e30 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isReadOnly(void)const")]
// 0x659e5c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10isReadOnlyEv
pub fn stub_659e5c() -> ! {
    todo!("0x659e5c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::isWriteOnly(void)const")]
// 0x659e6c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11isWriteOnlyEv
pub fn stub_659e6c() -> ! {
    todo!("0x659e6c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::hasStringValue(void)const")]
// 0x65a048 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14hasStringValueEv
pub fn stub_65a048() -> ! {
    todo!("0x65a048 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)5,float>::~SurfacePropDescriptor()")]
// 0x65a714 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED0Ev
pub fn stub_65a714() -> ! {
    todo!("0x65a714 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE5EfED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x65a838 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev
pub fn stub_65a838() -> ! {
    todo!("0x65a838 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isReadOnly(void)const")]
// 0x65a864 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv
pub fn stub_65a864() -> ! {
    todo!("0x65a864 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
// 0x65a874 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
pub fn stub_65a874() -> ! {
    todo!("0x65a874 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::hasStringValue(void)const")]
// 0x65aa50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv
pub fn stub_65aa50() -> ! {
    todo!("0x65aa50 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x65b044 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED0Ev
pub fn stub_65b044() -> ! {
    todo!("0x65b044 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::isReadOnly(void)const")]
// 0x65b070 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10isReadOnlyEv
pub fn stub_65b070() -> ! {
    todo!("0x65b070 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::isWriteOnly(void)const")]
// 0x65b080 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11isWriteOnlyEv
pub fn stub_65b080() -> ! {
    todo!("0x65b080 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::hasStringValue(void)const")]
// 0x65b25c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14hasStringValueEv
pub fn stub_65b25c() -> ! {
    todo!("0x65b25c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)0,float>::~SurfacePropDescriptor()")]
// 0x65b8b8 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED0Ev
pub fn stub_65b8b8() -> ! {
    todo!("0x65b8b8 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE0EfED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x65b9dc — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED0Ev
pub fn stub_65b9dc() -> ! {
    todo!("0x65b9dc __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::isReadOnly(void)const")]
// 0x65ba08 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10isReadOnlyEv
pub fn stub_65ba08() -> ! {
    todo!("0x65ba08 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
// 0x65ba18 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
pub fn stub_65ba18() -> ! {
    todo!("0x65ba18 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::hasStringValue(void)const")]
// 0x65bbf4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14hasStringValueEv
pub fn stub_65bbf4() -> ! {
    todo!("0x65bbf4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x65c1e8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED0Ev
pub fn stub_65c1e8() -> ! {
    todo!("0x65c1e8 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::isReadOnly(void)const")]
// 0x65c214 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10isReadOnlyEv
pub fn stub_65c214() -> ! {
    todo!("0x65c214 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::isWriteOnly(void)const")]
// 0x65c224 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11isWriteOnlyEv
pub fn stub_65c224() -> ! {
    todo!("0x65c224 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::SurfaceType>::hasStringValue(void)const")]
// 0x65c400 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14hasStringValueEv
pub fn stub_65c400() -> ! {
    todo!("0x65c400 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_11SurfaceTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)3,float>::~SurfacePropDescriptor()")]
// 0x65ca5c — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED0Ev
pub fn stub_65ca5c() -> ! {
    todo!("0x65ca5c __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE3EfED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x65cb80 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED0Ev
pub fn stub_65cb80() -> ! {
    todo!("0x65cb80 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::isReadOnly(void)const")]
// 0x65cbac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10isReadOnlyEv
pub fn stub_65cbac() -> ! {
    todo!("0x65cbac __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
// 0x65cbbc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
pub fn stub_65cbbc() -> ! {
    todo!("0x65cbbc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::hasStringValue(void)const")]
// 0x65cd98 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14hasStringValueEv
pub fn stub_65cd98() -> ! {
    todo!("0x65cd98 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x65d38c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED0Ev
pub fn stub_65d38c() -> ! {
    todo!("0x65d38c __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::isReadOnly(void)const")]
// 0x65d3b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10isReadOnlyEv
pub fn stub_65d3b8() -> ! {
    todo!("0x65d3b8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::isWriteOnly(void)const")]
// 0x65d3c8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11isWriteOnlyEv
pub fn stub_65d3c8() -> ! {
    todo!("0x65d3c8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::SurfaceType>::hasStringValue(void)const")]
// 0x65d5a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14hasStringValueEv
pub fn stub_65d5a4() -> ! {
    todo!("0x65d5a4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_11SurfaceTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()")]
// 0x65dc00 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED0Ev
pub fn stub_65dc00() -> ! {
    todo!("0x65dc00 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x65dd24 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED0Ev
pub fn stub_65dd24() -> ! {
    todo!("0x65dd24 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::isReadOnly(void)const")]
// 0x65dd50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10isReadOnlyEv
pub fn stub_65dd50() -> ! {
    todo!("0x65dd50 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
// 0x65dd60 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
pub fn stub_65dd60() -> ! {
    todo!("0x65dd60 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::hasStringValue(void)const")]
// 0x65df3c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14hasStringValueEv
pub fn stub_65df3c() -> ! {
    todo!("0x65df3c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x65e530 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED0Ev
pub fn stub_65e530() -> ! {
    todo!("0x65e530 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::isReadOnly(void)const")]
// 0x65e55c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10isReadOnlyEv
pub fn stub_65e55c() -> ! {
    todo!("0x65e55c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::isWriteOnly(void)const")]
// 0x65e56c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11isWriteOnlyEv
pub fn stub_65e56c() -> ! {
    todo!("0x65e56c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::hasStringValue(void)const")]
// 0x65e748 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14hasStringValueEv
pub fn stub_65e748() -> ! {
    todo!("0x65e748 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()")]
// 0x65eda4 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED0Ev
pub fn stub_65eda4() -> ! {
    todo!("0x65eda4 __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
// 0x65eec8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED0Ev
pub fn stub_65eec8() -> ! {
    todo!("0x65eec8 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::isReadOnly(void)const")]
// 0x65eef4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10isReadOnlyEv
pub fn stub_65eef4() -> ! {
    todo!("0x65eef4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
// 0x65ef04 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11isWriteOnlyEv
pub fn stub_65ef04() -> ! {
    todo!("0x65ef04 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::hasStringValue(void)const")]
// 0x65f0e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14hasStringValueEv
pub fn stub_65f0e0() -> ! {
    todo!("0x65f0e0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
// 0x65f6d4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED0Ev
pub fn stub_65f6d4() -> ! {
    todo!("0x65f6d4 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::isReadOnly(void)const")]
// 0x65f700 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10isReadOnlyEv
pub fn stub_65f700() -> ! {
    todo!("0x65f700 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::isWriteOnly(void)const")]
// 0x65f710 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11isWriteOnlyEv
pub fn stub_65f710() -> ! {
    todo!("0x65f710 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::hasStringValue(void)const")]
// 0x65f8ec — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14hasStringValueEv
pub fn stub_65f8ec() -> ! {
    todo!("0x65f8ec __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::LegacyController::InputType * rbx::any_cast<RBX::LegacyController::InputType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x65fe34 — __ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_65fe34() -> ! {
    todo!("0x65fe34 __ZN3rbx8any_castIN3RBX16LegacyController9InputTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::SurfaceType * rbx::any_cast<RBX::SurfaceType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x65fe8c — __ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_65fe8c() -> ! {
    todo!("0x65fe8c __ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::SurfaceSelection::setSurface(RBX::NormalId)")]
// 0x660890 — __ZN3RBX16SurfaceSelection10setSurfaceENS_8NormalIdE
pub fn stub_660890() -> ! {
    todo!("0x660890 __ZN3RBX16SurfaceSelection10setSurfaceENS_8NormalIdE")
}

#[doc(alias = "RBX::SurfaceSelection::SurfaceSelection(void)")]
// 0x6608b0 — __ZN3RBX16SurfaceSelectionC2Ev
pub fn stub_6608b0() -> ! {
    todo!("0x6608b0 __ZN3RBX16SurfaceSelectionC2Ev")
}

#[doc(alias = "RBX::SurfaceSelection::render3dAdorn(RBX::Adorn *)")]
// 0x660ac0 — __ZN3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE
pub fn stub_660ac0() -> ! {
    todo!("0x660ac0 __ZN3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::SurfaceSelection::render3dAdorn(RBX::Adorn *)")]
// 0x660bd8 — __ZThn96_N3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE
pub fn stub_660bd8() -> ! {
    todo!("0x660bd8 __ZThn96_N3RBX16SurfaceSelection13render3dAdornEPNS_5AdornE")
}

