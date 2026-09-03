// Auto-generated shard BJ — next 100 RBX::Reflection stubs — EA-sorted ascending
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 12258 prior, next 100 after 0xf28ba4)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;

// 0xf28ba4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v
#[doc(alias = "RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_11HttpService15HttpContentTypeEEERT_v")]
pub fn stub_f28ba4() -> ! {
    todo!("0xf28ba4 RBX::HttpService::HttpContentType & RBX::Reflection::Variant::genericConvert<RBX::HttpService::HttpContentType>(void)")
}

// 0xf28bb4 — j___ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::addPair(RBX::HttpService::HttpContentType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE7addPairES3_PKc")]
pub fn stub_f28bb4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf28bb4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf28bc4 — j___ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f28bc4() -> ! {
    todo!("0xf28bc4 boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf28bd4 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgINS_11HttpService15HttpContentTypeELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f28bd4() -> ! {
    todo!("0xf28bd4 RBX::HttpService::HttpContentType RBX::Reflection::ArgHelper::getArg<RBX::HttpService::HttpContentType,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::HttpService::HttpContentType> const&,boost::disable_if<boost::is_same<RBX::HttpService::HttpContentType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf28be4 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11HttpService15HttpContentTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_f28be4() -> ! {
    todo!("0xf28be4 bool RBX::Reflection::ArgHelper::try_enum<3,RBX::HttpService::HttpContentType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::HttpService::HttpContentType &,boost::enable_if<boost::is_enum<RBX::HttpService::HttpContentType>,void>::type *)")
}

// 0xf28bf4 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11HttpServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11HttpServiceEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28bf4() -> ! {
    todo!("0xf28bf4 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HttpService>(char const*,char const*,bool RBX::HttpService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28ca4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11HttpServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpService,RBX::HttpService>(rbx_core::SharedPtr<RBX::HttpService> const*,RBX::HttpService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11HttpServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f28ca4() {
    // IDA 0xf28ca4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf28d54 — j___ZN3RBX10Reflection14PropDescriptorINS_10PointLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PointLight,float>::PropDescriptor<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>(char const*,char const*,float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10PointLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28d54() -> ! {
    todo!("0xf28d54 RBX::Reflection::PropDescriptor<RBX::PointLight,float>::PropDescriptor<float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float)>(char const*,char const*,float (RBX::PointLight::*)(void)const,void (RBX::PointLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d64 — j___ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28d64() -> ! {
    todo!("0xf28d64 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d74 — j___ZN3RBX10Reflection14PropDescriptorINS_5LightEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,bool>::PropDescriptor<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>(char const*,char const*,bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5LightEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28d74() -> ! {
    todo!("0xf28d74 RBX::Reflection::PropDescriptor<RBX::Light,bool>::PropDescriptor<bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool)>(char const*,char const*,bool (RBX::Light::*)(void)const,void (RBX::Light::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d84 — j___ZN3RBX10Reflection14PropDescriptorINS_5LightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,float>::PropDescriptor<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>(char const*,char const*,float (RBX::Light::*)(void)const,void (RBX::Light::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5LightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28d84() -> ! {
    todo!("0xf28d84 RBX::Reflection::PropDescriptor<RBX::Light,float>::PropDescriptor<float (RBX::Light::*)(void)const,void (RBX::Light::*)(float)>(char const*,char const*,float (RBX::Light::*)(void)const,void (RBX::Light::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28d94 — j___ZN3RBX10Reflection14PropDescriptorINS_9SpotLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::PropDescriptor<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>(char const*,char const*,float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_9SpotLightEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28d94() -> ! {
    todo!("0xf28d94 RBX::Reflection::PropDescriptor<RBX::SpotLight,float>::PropDescriptor<float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float)>(char const*,char const*,float (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28da4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f28da4() -> ! {
    todo!("0xf28da4 RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::SpotLight::*)(void)const,void (RBX::SpotLight::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf28ea4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_9SpotLightENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f28ea4() -> ! {
    todo!("0xf28ea4 RBX::Reflection::EnumPropDescriptor<RBX::SpotLight,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf28eb4 — j___ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToIndexES2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::convertToIndex(RBX::NormalId)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToIndexES2_")]
pub fn stub_f28eb4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf28eb4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf28ee4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10PointLightES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PointLight,RBX::PointLight>(rbx_core::SharedPtr<RBX::PointLight> const*,RBX::PointLight *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10PointLightES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f28ee4() {
    // IDA 0xf28ee4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf28ef4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SpotLightES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpotLight,RBX::SpotLight>(rbx_core::SharedPtr<RBX::SpotLight> const*,RBX::SpotLight *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SpotLightES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f28ef4() {
    // IDA 0xf28ef4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf28f04 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10declareSubEPS2_S4_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declareSub(RBX::Reflection::EventDescriptor*,RBX::Reflection::EventDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10declareSubEPS2_S4_")]
pub fn stub_f28f04() -> ! {
    todo!("0xf28f04 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declareSub(RBX::Reflection::EventDescriptor*,RBX::Reflection::EventDescriptor*)")
}

// 0xf28f14 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::staticData(void)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE10staticDataEv")]
pub fn stub_f28f14() -> ! {
    todo!("0xf28f14 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::staticData(void)")
}

// 0xf28f24 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE7declareEPS2_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declare(RBX::Reflection::EventDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE7declareEPS2_")]
pub fn stub_f28f24() -> ! {
    todo!("0xf28f24 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::declare(RBX::Reflection::EventDescriptor*)")
}

// 0xf28f74 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")]
pub fn stub_f28f74() -> ! {
    todo!("0xf28f74 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf28f94 — j___ZNSt10_List_baseIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE8_M_clearEv
#[doc(alias = "std::_List_base<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_clear(void)")]
#[doc(alias = "j___ZNSt10_List_baseIN3RBX10Reflection19SignatureDescriptor4ItemESaIS3_EE8_M_clearEv")]
pub fn stub_f28f94() -> ! {
    todo!("0xf28f94 std::_List_base<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_clear(void)")
}

// 0xf28fa4 — j___ZNSt6vectorIPN3RBX10Reflection15EventDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX10Reflection15EventDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_f28fa4() -> ! {
    todo!("0xf28fa4 std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")
}

// 0xf28fb4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE7declareEPS2_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declare(RBX::Reflection::FunctionDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE7declareEPS2_")]
pub fn stub_f28fb4() -> ! {
    todo!("0xf28fb4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declare(RBX::Reflection::FunctionDescriptor*)")
}

// 0xf28fc4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10declareSubEPS2_S4_")]
pub fn stub_f28fc4() -> ! {
    todo!("0xf28fc4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declareSub(RBX::Reflection::YieldFunctionDescriptor*,RBX::Reflection::YieldFunctionDescriptor*)")
}

// 0xf28fd4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE10staticDataEv")]
pub fn stub_f28fd4() -> ! {
    todo!("0xf28fd4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::staticData(void)")
}

// 0xf28fe4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE7declareEPS2_")]
pub fn stub_f28fe4() -> ! {
    todo!("0xf28fe4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::declare(RBX::Reflection::YieldFunctionDescriptor*)")
}

// 0xf29014 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")]
pub fn stub_f29014() -> ! {
    todo!("0xf29014 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf29024 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")]
pub fn stub_f29024() -> ! {
    todo!("0xf29024 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf29034 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")]
pub fn stub_f29034() -> ! {
    todo!("0xf29034 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")
}

// 0xf29044 — j___ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX10Reflection23YieldFunctionDescriptorESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
pub fn stub_f29044() -> ! {
    todo!("0xf29044 std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::YieldFunctionDescriptor **,std::vector<RBX::Reflection::YieldFunctionDescriptor *,std::allocator<RBX::Reflection::YieldFunctionDescriptor *>>>,RBX::Reflection::YieldFunctionDescriptor * const&)")
}

// 0xf29054 — j___ZN3RBX10Reflection10DescriptorC2EPKcNS1_10AttributesE
#[doc(alias = "RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection10DescriptorC2EPKcNS1_10AttributesE")]
pub fn stub_f29054() -> ! {
    todo!("0xf29054 RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf29064 — j___ZN3RBX10Reflection15ClassDescriptorD2Ev
#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
#[doc(alias = "j___ZN3RBX10Reflection15ClassDescriptorD2Ev")]
pub fn stub_f29064() {
    // IDA 0xf29064: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf29074 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE12mergeMembersEPKS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> const*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE12mergeMembersEPKS3_")]
pub fn stub_f29074() -> ! {
    todo!("0xf29074 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> const*)")
}

// 0xf29084 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEEC2EPS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEEC2EPS3_")]
pub fn stub_f29084() -> ! {
    todo!("0xf29084 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")
}

// 0xf29094 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10declareSubEPS2_S4_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10declareSubEPS2_S4_")]
pub fn stub_f29094() -> ! {
    todo!("0xf29094 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")
}

// 0xf290a4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE10staticDataEv")]
pub fn stub_f290a4() -> ! {
    todo!("0xf290a4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")
}

// 0xf290b4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE12mergeMembersEPKS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE12mergeMembersEPKS3_")]
pub fn stub_f290b4() -> ! {
    todo!("0xf290b4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")
}

// 0xf290c4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE7declareEPS2_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE7declareEPS2_")]
pub fn stub_f290c4() -> ! {
    todo!("0xf290c4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")
}

// 0xf290d4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEEC2EPS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEEC2EPS3_")]
pub fn stub_f290d4() -> ! {
    todo!("0xf290d4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")
}

// 0xf290e4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10declareSubEPS2_S4_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declareSub(RBX::Reflection::FunctionDescriptor*,RBX::Reflection::FunctionDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10declareSubEPS2_S4_")]
pub fn stub_f290e4() -> ! {
    todo!("0xf290e4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declareSub(RBX::Reflection::FunctionDescriptor*,RBX::Reflection::FunctionDescriptor*)")
}

// 0xf290f4 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10staticDataEv
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::staticData(void)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE10staticDataEv")]
pub fn stub_f290f4() -> ! {
    todo!("0xf290f4 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::staticData(void)")
}

// 0xf29104 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE12mergeMembersEPKS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> const*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE12mergeMembersEPKS3_")]
pub fn stub_f29104() -> ! {
    todo!("0xf29104 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> const*)")
}

// 0xf29114 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEEC2EPS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEEC2EPS3_")]
pub fn stub_f29114() -> ! {
    todo!("0xf29114 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")
}

// 0xf29124 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10declareSubEPS2_S4_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declareSub(RBX::Reflection::PropertyDescriptor*,RBX::Reflection::PropertyDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10declareSubEPS2_S4_")]
pub fn stub_f29124() -> ! {
    todo!("0xf29124 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declareSub(RBX::Reflection::PropertyDescriptor*,RBX::Reflection::PropertyDescriptor*)")
}

// 0xf29134 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10staticDataEv
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::staticData(void)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE10staticDataEv")]
pub fn stub_f29134() -> ! {
    todo!("0xf29134 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::staticData(void)")
}

// 0xf29144 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE12mergeMembersEPKS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> const*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE12mergeMembersEPKS3_")]
pub fn stub_f29144() -> ! {
    todo!("0xf29144 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> const*)")
}

// 0xf29154 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE7declareEPS2_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declare(RBX::Reflection::PropertyDescriptor*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEE7declareEPS2_")]
pub fn stub_f29154() -> ! {
    todo!("0xf29154 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declare(RBX::Reflection::PropertyDescriptor*)")
}

// 0xf29164 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEEC2EPS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_18PropertyDescriptorEEC2EPS3_")]
pub fn stub_f29164() -> ! {
    todo!("0xf29164 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")
}

// 0xf29174 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE12mergeMembersEPKS3_")]
pub fn stub_f29174() -> ! {
    todo!("0xf29174 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor> const*)")
}

// 0xf29184 — j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")]
#[doc(alias = "j___ZN3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEEC2EPS3_")]
pub fn stub_f29184() -> ! {
    todo!("0xf29184 RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>*)")
}

// 0xf291f4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")]
pub fn stub_f291f4() -> ! {
    todo!("0xf291f4 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")
}

// 0xf29204 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
pub fn stub_f29204() -> ! {
    todo!("0xf29204 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf29214 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")]
pub fn stub_f29214() -> ! {
    todo!("0xf29214 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")
}

// 0xf29224 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
pub fn stub_f29224() -> ! {
    todo!("0xf29224 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf29234 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")]
pub fn stub_f29234() -> ! {
    todo!("0xf29234 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf29244 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")]
pub fn stub_f29244() -> ! {
    todo!("0xf29244 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")
}

// 0xf29254 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
pub fn stub_f29254() -> ! {
    todo!("0xf29254 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf29264 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")]
pub fn stub_f29264() -> ! {
    todo!("0xf29264 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")
}

// 0xf29274 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
pub fn stub_f29274() -> ! {
    todo!("0xf29274 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf29284 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEixERS7_")]
pub fn stub_f29284() -> ! {
    todo!("0xf29284 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")
}

// 0xf29294 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE11rehash_implEm")]
pub fn stub_f29294() -> ! {
    todo!("0xf29294 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")
}

// 0xf292a4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE")]
pub fn stub_f292a4() -> ! {
    todo!("0xf292a4 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0xf292b4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEEEEE9constructEv")]
pub fn stub_f292b4() -> ! {
    todo!("0xf292b4 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>>::construct(void)")
}

// 0xf292c4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEEEEE9constructEv")]
pub fn stub_f292c4() -> ! {
    todo!("0xf292c4 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)")
}

// 0xf292d4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEEEEE9constructEv")]
pub fn stub_f292d4() -> ! {
    todo!("0xf292d4 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>>>::construct(void)")
}

// 0xf292e4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEEEEE9constructEv")]
pub fn stub_f292e4() -> ! {
    todo!("0xf292e4 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>>>::construct(void)")
}

// 0xf292f4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEEEEE9constructEv")]
pub fn stub_f292f4() -> ! {
    todo!("0xf292f4 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>>>::construct(void)")
}

// 0xf29304 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")]
pub fn stub_f29304() -> ! {
    todo!("0xf29304 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")
}

// 0xf29314 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")]
pub fn stub_f29314() -> ! {
    todo!("0xf29314 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")
}

// 0xf29324 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")]
pub fn stub_f29324() -> ! {
    todo!("0xf29324 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")
}

// 0xf29334 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")]
pub fn stub_f29334() -> ! {
    todo!("0xf29334 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> const&)")
}

// 0xf29344 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")]
pub fn stub_f29344() -> ! {
    todo!("0xf29344 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")
}

// 0xf29354 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")]
pub fn stub_f29354() -> ! {
    todo!("0xf29354 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")
}

// 0xf29364 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")]
pub fn stub_f29364() -> ! {
    todo!("0xf29364 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)")
}

// 0xf29374 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")]
pub fn stub_f29374() -> ! {
    todo!("0xf29374 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")
}

// 0xf29384 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")]
pub fn stub_f29384() -> ! {
    todo!("0xf29384 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")
}

// 0xf29394 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")]
pub fn stub_f29394() -> ! {
    todo!("0xf29394 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> const&)")
}

// 0xf293a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")]
pub fn stub_f293a4() -> ! {
    todo!("0xf293a4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")
}

// 0xf293b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")]
pub fn stub_f293b4() -> ! {
    todo!("0xf293b4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")
}

// 0xf293c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")]
pub fn stub_f293c4() -> ! {
    todo!("0xf293c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> const&)")
}

// 0xf293d4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14create_bucketsEm")]
pub fn stub_f293d4() -> ! {
    todo!("0xf293d4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")
}

// 0xf293e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")]
pub fn stub_f293e4() -> ! {
    todo!("0xf293e4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")
}

// 0xf293f4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE18reserve_for_insertEm")]
pub fn stub_f293f4() -> ! {
    todo!("0xf293f4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")
}

// 0xf29404 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEEC2EmRKSE_RKSF_RKSaINS1_8ptr_nodeISC_EEE")]
pub fn stub_f29404() -> ! {
    todo!("0xf29404 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> const&)")
}

// 0xf29414 — j___ZNK3RBX10Reflection19StringHashPredicateclEPKc
#[doc(alias = "RBX::Reflection::StringHashPredicate::operator()(char const*)const")]
#[doc(alias = "j___ZNK3RBX10Reflection19StringHashPredicateclEPKc")]
pub fn stub_f29414() -> ! {
    todo!("0xf29414 RBX::Reflection::StringHashPredicate::operator()(char const*)const")
}

// 0xf29434 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")]
pub fn stub_f29434() -> ! {
    todo!("0xf29434 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")
}

// 0xf29444 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")]
pub fn stub_f29444() -> ! {
    todo!("0xf29444 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")
}

// 0xf29454 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")]
pub fn stub_f29454() -> ! {
    todo!("0xf29454 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")
}

// 0xf29464 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")]
pub fn stub_f29464() -> ! {
    todo!("0xf29464 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")
}

// 0xf29474 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14find_node_implIS6_SF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEmRKT_RKT0_")]
pub fn stub_f29474() -> ! {
    todo!("0xf29474 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")
}

// 0xf29484 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection15EventDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")]
pub fn stub_f29484() -> ! {
    todo!("0xf29484 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")
}

// 0xf29494 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")]
pub fn stub_f29494() -> ! {
    todo!("0xf29494 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")
}

// 0xf294a4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18FunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")]
pub fn stub_f294a4() -> ! {
    todo!("0xf294a4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")
}

// 0xf294b4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18PropertyDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")]
pub fn stub_f294b4() -> ! {
    todo!("0xf294b4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")
}

// 0xf294c4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection23YieldFunctionDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE20min_buckets_for_sizeEm")]
pub fn stub_f294c4() -> ! {
    todo!("0xf294c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::YieldFunctionDescriptor *>>,char const*,RBX::Reflection::YieldFunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")
}

// 0xf294d4 — j___ZNSt12_Vector_baseIPN3RBX10Reflection15ClassDescriptorESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10Reflection15ClassDescriptorESaIS3_EE11_M_allocateEm")]
pub fn stub_f294d4() -> ! {
    todo!("0xf294d4 std::_Vector_base<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_allocate(unsigned long)")
}

// 0xf294e4 — j___ZNSt12_Vector_baseIPN3RBX10Reflection15EventDescriptorESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10Reflection15EventDescriptorESaIS3_EE11_M_allocateEm")]
pub fn stub_f294e4() -> ! {
    todo!("0xf294e4 std::_Vector_base<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::_M_allocate(unsigned long)")
}

// 0xf294f4 — j___ZNSt12_Vector_baseIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10Reflection18CallbackDescriptorESaIS3_EE11_M_allocateEm")]
pub fn stub_f294f4() -> ! {
    todo!("0xf294f4 std::_Vector_base<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_allocate(unsigned long)")
}

// 0xf29504 — j___ZNSt12_Vector_baseIPN3RBX10Reflection18FunctionDescriptorESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10Reflection18FunctionDescriptorESaIS3_EE11_M_allocateEm")]
pub fn stub_f29504() -> ! {
    todo!("0xf29504 std::_Vector_base<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::_M_allocate(unsigned long)")
}
