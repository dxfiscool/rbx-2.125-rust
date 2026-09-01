//! rendering shard 423 — 100 stubs 0x6536dc..0x6591a8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 45510->45610 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x6536dc..0x6591a8 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x6536dc — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info")]
// was: __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
pub fn stub_6536dc() -> ! {
    todo!("0x6536dc boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")
}

// 0x6536e0 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
// type: 
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv")]
// was: __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
pub fn stub_6536e0() -> ! {
    todo!("0x6536e0 boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")
}

// 0x6536e8 — __ZN5boost6detail8function15functor_managerIPFvPSsPSt9exceptionEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
// type: 
#[doc(alias = "__ZN5boost6detail8function15functor_managerIPFvPSsPSt9exceptionEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE")]
// was: __ZN5boost6detail8function15functor_managerIPFvPSsPSt9exceptionEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<void (*)(std::string *,std::exception *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_6536e8() -> ! {
    todo!("0x6536e8 boost::detail::function::functor_manager<void (*)(std::string *,std::exception *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x653744 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_653744() -> ! {
    todo!("0x653744 boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x653838 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEENS5_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEENS5_7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIRKN5boost10shared_ptrIKNS1_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEENS5_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_653838() -> ! {
    todo!("0x653838 boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x653928 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX5Stats10JsonWriterEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairISsNS3_10Reflection7VariantEEEENS0_5list1IRKSE_IKSsSG_EEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX5Stats10JsonWriterEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairISsNS3_10Reflection7VariantEEEENS0_5list1IRKSE_IKSsSG_EEEEEvNS0_4typeIvEERT_RT0_i")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX5Stats10JsonWriterEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairISsNS3_10Reflection7VariantEEEENS0_5list1IRKSE_IKSsSG_EEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Stats::JsonWriter *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Stats::JsonWriter,std::pair<std::string,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Stats::JsonWriter,std::pair<std::string,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>> &,int)")]
pub fn stub_653928() -> ! {
    todo!("0x653928 void boost::_bi::list2<boost::_bi::value<RBX::Stats::JsonWriter *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Stats::JsonWriter,std::pair<std::string,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Stats::JsonWriter,std::pair<std::string,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair const&<std::string const,RBX::Reflection::Variant>> &,int)")
}

// 0x653b2c — __ZNSt4pairISsN3RBX10Reflection7VariantEEC2IKSsS2_EERKS_IT_T0_E
// type: 
#[doc(alias = "__ZNSt4pairISsN3RBX10Reflection7VariantEEC2IKSsS2_EERKS_IT_T0_E")]
// was: __ZNSt4pairISsN3RBX10Reflection7VariantEEC2IKSsS2_EERKS_IT_T0_E
#[doc(alias = "std::pair<std::string,RBX::Reflection::Variant>::pair<std::string const,RBX::Reflection::Variant>(std::pair const&<std::string const,RBX::Reflection::Variant>)")]
pub fn stub_653b2c() -> ! {
    todo!("0x653b2c std::pair<std::string,RBX::Reflection::Variant>::pair<std::string const,RBX::Reflection::Variant>(std::pair const&<std::string const,RBX::Reflection::Variant>)")
}

// 0x653bf0 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EEC2EMS3_FdvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EEC2EMS3_FdvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EEC2EMS3_FdvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,double ()(void),0>::BoundFuncDesc(double (RBX::Stats::Item::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_653bf0() -> ! {
    todo!("0x653bf0 RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,double ()(void),0>::BoundFuncDesc(double (RBX::Stats::Item::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x653cf4 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,double ()(void),0>::~BoundFuncDesc()")]
pub fn stub_653cf4() -> ! {
    todo!("0x653cf4 RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,double ()(void),0>::~BoundFuncDesc()")
}

// 0x653da8 — __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFdvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,double ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_653da8() -> ! {
    todo!("0x653da8 RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,double ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x653dcc — __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FdvEdE4callEPS3_S5_RNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FdvEdE4callEPS3_S5_RNS0_7VariantE")]
// was: __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FdvEdE4callEPS3_S5_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Stats::Item,double (RBX::Stats::Item::*)(void),double>::call(RBX::Stats::Item*,double (RBX::Stats::Item::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_653dcc() -> ! {
    todo!("0x653dcc RBX::Reflection::Call0Helper<RBX::Stats::Item,double (RBX::Stats::Item::*)(void),double>::call(RBX::Stats::Item*,double (RBX::Stats::Item::*)(void),RBX::Reflection::Variant &)")
}

// 0x653e08 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EEC2EMS3_FSsvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EEC2EMS3_FSsvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EEC2EMS3_FSsvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,std::string ()(void),0>::BoundFuncDesc(std::string (RBX::Stats::Item::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_653e08() -> ! {
    todo!("0x653e08 RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,std::string ()(void),0>::BoundFuncDesc(std::string (RBX::Stats::Item::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x653f0c — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,std::string ()(void),0>::~BoundFuncDesc()")]
pub fn stub_653f0c() -> ! {
    todo!("0x653f0c RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,std::string ()(void),0>::~BoundFuncDesc()")
}

// 0x653fc0 — __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats4ItemEFSsvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,std::string ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_653fc0() -> ! {
    todo!("0x653fc0 RBX::Reflection::BoundFuncDesc<RBX::Stats::Item,std::string ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x653fe4 — __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FSsvESsE4callEPS3_S5_RNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FSsvESsE4callEPS3_S5_RNS0_7VariantE")]
// was: __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FSsvESsE4callEPS3_S5_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Stats::Item,std::string (RBX::Stats::Item::*)(void),std::string>::call(RBX::Stats::Item*,std::string (RBX::Stats::Item::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_653fe4() -> ! {
    todo!("0x653fe4 RBX::Reflection::Call0Helper<RBX::Stats::Item,std::string (RBX::Stats::Item::*)(void),std::string>::call(RBX::Stats::Item*,std::string (RBX::Stats::Item::*)(void),RBX::Reflection::Variant &)")
}

// 0x654124 — __ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_5Stats12StatsServiceEEEPKcS8_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_5Stats12StatsServiceEEEPKcS8_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EEC2INS_5Stats12StatsServiceEEEPKcS8_MT_dNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Stats::StatsService>(char const*,char const*,double RBX::Stats::StatsService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_654124() -> ! {
    todo!("0x654124 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Stats::StatsService>(char const*,char const*,double RBX::Stats::StatsService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6542b8 — __ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EED0Ev")]
// was: __ZN3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_6542b8() -> ! {
    todo!("0x6542b8 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x6542e8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_6542e8() -> ! {
    todo!("0x6542e8 RBX::Reflection::TypedPropertyDescriptor<double>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x654440 — __ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// was: __ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "double const& rbx::any_cast<double const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_654440() -> ! {
    todo!("0x654440 double const& rbx::any_cast<double const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x654528 — __ZN3RBX10Reflection23TypedPropertyDescriptorIdED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIdED1Ev")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorIdED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::~TypedPropertyDescriptor()")]
pub fn stub_654528() -> ! {
    todo!("0x654528 RBX::Reflection::TypedPropertyDescriptor<double>::~TypedPropertyDescriptor()")
}

// 0x654550 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isReadOnly(void)const")]
pub fn stub_654550() -> ! {
    todo!("0x654550 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isReadOnly(void)const")
}

// 0x654554 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isWriteOnly(void)const")]
pub fn stub_654554() -> ! {
    todo!("0x654554 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isWriteOnly(void)const")
}

// 0x654558 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_654558() -> ! {
    todo!("0x654558 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x654568 — __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8setValueEPNS0_13DescribedBaseERKd
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8setValueEPNS0_13DescribedBaseERKd")]
// was: __ZNK3RBX10Reflection9BoundPropIdLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
pub fn stub_654568() -> ! {
    todo!("0x654568 RBX::Reflection::BoundProp<double,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::setValue(RBX::Reflection::DescribedBase *,double const&)const")
}

// 0x6545c4 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_5Stats12StatsServiceEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_5Stats12StatsServiceEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// was: __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2INS_5Stats12StatsServiceEEEPKcS8_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Stats::StatsService>(char const*,char const*,std::string  RBX::Stats::StatsService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_6545c4() -> ! {
    todo!("0x6545c4 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Stats::StatsService>(char const*,char const*,std::string  RBX::Stats::StatsService::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x654758 — __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EED0Ev")]
// was: __ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_654758() -> ! {
    todo!("0x654758 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x654788 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE10isReadOnlyEv")]
// was: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isReadOnly(void)const")]
pub fn stub_654788() -> ! {
    todo!("0x654788 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isReadOnly(void)const")
}

// 0x65478c — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE11isWriteOnlyEv")]
// was: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isWriteOnly(void)const")]
pub fn stub_65478c() -> ! {
    todo!("0x65478c RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::isWriteOnly(void)const")
}

// 0x654790 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8getValueEPKNS0_13DescribedBaseE")]
// was: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_654790() -> ! {
    todo!("0x654790 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6547a8 — __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8setValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8setValueEPNS0_13DescribedBaseERKSs")]
// was: __ZNK3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EE15BoundPropGetSetINS_5Stats12StatsServiceEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_6547a8() -> ! {
    todo!("0x6547a8 RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Stats::StatsService>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x654810 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EEC2EMS3_FvSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EEC2EMS3_FvSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EEC2EMS3_FvSsEPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_654810() -> ! {
    todo!("0x654810 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x654988 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_654988() -> ! {
    todo!("0x654988 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x6549b8 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_6549b8() -> ! {
    todo!("0x6549b8 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x654a84 — __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_654a84() -> ! {
    todo!("0x654a84 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x654bc0 — __ZN3RBX10Reflection11Call1HelperINS_5Stats12StatsServiceEMS3_FvSsESsvE4callEPS3_S5_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_5Stats12StatsServiceEMS3_FvSsESsvE4callEPS3_S5_RNS0_7VariantERKSs")]
// was: __ZN3RBX10Reflection11Call1HelperINS_5Stats12StatsServiceEMS3_FvSsESsvE4callEPS3_S5_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Stats::StatsService,void (RBX::Stats::StatsService::*)(std::string),std::string,void>::call(RBX::Stats::StatsService*,void (RBX::Stats::StatsService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_654bc0() -> ! {
    todo!("0x654bc0 RBX::Reflection::Call1Helper<RBX::Stats::StatsService,void (RBX::Stats::StatsService::*)(std::string),std::string,void>::call(RBX::Stats::StatsService*,void (RBX::Stats::StatsService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x654cf0 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EEC2EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(void),0>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_654cf0() -> ! {
    todo!("0x654cf0 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(void),0>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x654df4 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_654df4() -> ! {
    todo!("0x654df4 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(void),0>::~BoundFuncDesc()")
}

// 0x654ea8 — __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_654ea8() -> ! {
    todo!("0x654ea8 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x654ec8 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EEC2EMS3_FvbEPKcS9_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EEC2EMS3_FvbEPKcS9_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EEC2EMS3_FvbEPKcS9_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_654ec8() -> ! {
    todo!("0x654ec8 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x655074 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_655074() -> ! {
    todo!("0x655074 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x6550a4 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_6550a4() -> ! {
    todo!("0x6550a4 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x655178 — __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_655178() -> ! {
    todo!("0x655178 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6551ac — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EEC2EMS3_FvSsSJ_EPKcSP_SP_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EEC2EMS3_FvSsSJ_EPKcSP_SP_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EEC2EMS3_FvSsSJ_EPKcSP_SP_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_6551ac() -> ! {
    todo!("0x6551ac RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::BoundFuncDesc(void (RBX::Stats::StatsService::*)(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x655394 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EE16declareSignatureEPKcS8_SN_S8_
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EE16declareSignatureEPKcS8_SN_S8_")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EE16declareSignatureEPKcS8_SN_S8_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_655394() -> ! {
    todo!("0x655394 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x6553e0 — __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::~BoundFuncDesc()")]
pub fn stub_6553e0() -> ! {
    todo!("0x6553e0 RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::~BoundFuncDesc()")
}

// 0x65550c — __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_5Stats12StatsServiceEFvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_65550c() -> ! {
    todo!("0x65550c RBX::Reflection::BoundFuncDesc<RBX::Stats::StatsService,void ()(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x655690 — __ZN3RBX10Reflection11Call2HelperINS_5Stats12StatsServiceEMS3_FvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEESsSJ_vE4callEPS3_SL_RS8_RSE_RKSJ_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_5Stats12StatsServiceEMS3_FvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEESsSJ_vE4callEPS3_SL_RS8_RSE_RKSJ_")]
// was: __ZN3RBX10Reflection11Call2HelperINS_5Stats12StatsServiceEMS3_FvSsN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEESsSJ_vE4callEPS3_SL_RS8_RSE_RKSJ_
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Stats::StatsService,void (RBX::Stats::StatsService::*)(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,void>::call(RBX::Stats::StatsService*,void (RBX::Stats::StatsService::*)(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,std::string const&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_655690() -> ! {
    todo!("0x655690 RBX::Reflection::Call2Helper<RBX::Stats::StatsService,void (RBX::Stats::StatsService::*)(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,void>::call(RBX::Stats::StatsService*,void (RBX::Stats::StatsService::*)(std::string,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),RBX::Reflection::Variant&,std::string const&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")
}

// 0x655818 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrISJ_EEPNS3_10disable_ifINS3_7is_sameISJ_NS4_IKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_655818() -> ! {
    todo!("0x655818 boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Reflection::ArgHelper::getArg<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&,boost::disable_if<boost::is_same<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x6559dc — __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EEC2EMS2_FS7_iEPKcSD_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EEC2EMS2_FS7_iEPKcSD_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EEC2EMS2_FS7_iEPKcSD_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_6559dc() -> ! {
    todo!("0x6559dc RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x655b88 — __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_655b88() -> ! {
    todo!("0x655b88 RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x655bb8 — __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::~BoundFuncDesc()")]
pub fn stub_655bb8() -> ! {
    todo!("0x655bb8 RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::~BoundFuncDesc()")
}

// 0x655c8c — __ZNK3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEiELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_655c8c() -> ! {
    todo!("0x655c8c RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x655ccc — __ZN3RBX10Reflection11Call1HelperI13ProfilingItemMS2_FN5boost10shared_ptrIKNS0_5TupleEEEiEiS7_E4callEPS2_S9_RNS0_7VariantERKi
// type: 
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperI13ProfilingItemMS2_FN5boost10shared_ptrIKNS0_5TupleEEEiEiS7_E4callEPS2_S9_RNS0_7VariantERKi")]
// was: __ZN3RBX10Reflection11Call1HelperI13ProfilingItemMS2_FN5boost10shared_ptrIKNS0_5TupleEEEiEiS7_E4callEPS2_S9_RNS0_7VariantERKi
#[doc(alias = "RBX::Reflection::Call1Helper<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(int),int,boost::shared_ptr<RBX::Reflection::Tuple const>>::call(ProfilingItem*,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(int),RBX::Reflection::Variant &,int const&)")]
pub fn stub_655ccc() -> ! {
    todo!("0x655ccc RBX::Reflection::Call1Helper<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(int),int,boost::shared_ptr<RBX::Reflection::Tuple const>>::call(ProfilingItem*,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(int),RBX::Reflection::Variant &,int const&)")
}

// 0x655db8 — __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EEC2EMS2_FS7_dEPKcSD_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, double, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EEC2EMS2_FS7_dEPKcSD_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EEC2EMS2_FS7_dEPKcSD_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(double),char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_655db8() -> ! {
    todo!("0x655db8 RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(double),char const*,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x655f68 — __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EE16declareSignatureEPKcNS0_7VariantE")]
// was: __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_655f68() -> ! {
    todo!("0x655f68 RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x655f98 — __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EED0Ev")]
// was: __ZN3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::~BoundFuncDesc()")]
pub fn stub_655f98() -> ! {
    todo!("0x655f98 RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::~BoundFuncDesc()")
}

// 0x65606c — __ZNK3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// was: __ZNK3RBX10Reflection13BoundFuncDescI13ProfilingItemFN5boost10shared_ptrIKNS0_5TupleEEEdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_65606c() -> ! {
    todo!("0x65606c RBX::Reflection::BoundFuncDesc<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x6560b8 — __ZN3RBX10Reflection11Call1HelperI13ProfilingItemMS2_FN5boost10shared_ptrIKNS0_5TupleEEEdEdS7_E4callEPS2_S9_RNS0_7VariantERKd
// type: 
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperI13ProfilingItemMS2_FN5boost10shared_ptrIKNS0_5TupleEEEdEdS7_E4callEPS2_S9_RNS0_7VariantERKd")]
// was: __ZN3RBX10Reflection11Call1HelperI13ProfilingItemMS2_FN5boost10shared_ptrIKNS0_5TupleEEEdEdS7_E4callEPS2_S9_RNS0_7VariantERKd
#[doc(alias = "RBX::Reflection::Call1Helper<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(double),double,boost::shared_ptr<RBX::Reflection::Tuple const>>::call(ProfilingItem*,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(double),RBX::Reflection::Variant &,double const&)")]
pub fn stub_6560b8() -> ! {
    todo!("0x6560b8 RBX::Reflection::Call1Helper<ProfilingItem,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(double),double,boost::shared_ptr<RBX::Reflection::Tuple const>>::call(ProfilingItem*,boost::shared_ptr<RBX::Reflection::Tuple const> (ProfilingItem::*)(double),RBX::Reflection::Variant &,double const&)")
}

// 0x6561ac — __ZN3RBX5Stats12StatsServiceD2Ev
// type: void __fastcall(RBX::Stats::StatsService *__hidden this)
#[doc(alias = "__ZN3RBX5Stats12StatsServiceD2Ev")]
// was: __ZN3RBX5Stats12StatsServiceD2Ev
#[doc(alias = "RBX::Stats::StatsService::~StatsService()")]
pub fn stub_6561ac() -> ! {
    todo!("0x6561ac RBX::Stats::StatsService::~StatsService()")
}

// 0x6562e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_6562e4() -> ! {
    todo!("0x6562e4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x65631c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_65631c() -> ! {
    todo!("0x65631c boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x65634c — __ZN5boost4bindIvNS_10shared_ptrIN3RBX9DataModelEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// type: int(void)
#[doc(alias = "__ZN5boost4bindIvNS_10shared_ptrIN3RBX9DataModelEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")]
// was: __ZN5boost4bindIvNS_10shared_ptrIN3RBX9DataModelEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::DataModel>,std::string>::type> boost::bind<void,boost::shared_ptr<RBX::DataModel>,std::string,boost::shared_ptr<RBX::DataModel>,std::string>(void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::shared_ptr<RBX::DataModel>,std::string)")]
pub fn stub_65634c() -> ! {
    todo!("0x65634c boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::DataModel>,std::string>::type> boost::bind<void,boost::shared_ptr<RBX::DataModel>,std::string,boost::shared_ptr<RBX::DataModel>,std::string>(void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::shared_ptr<RBX::DataModel>,std::string)")
}

// 0x656598 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEC2ES7_S8_")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>)")]
pub fn stub_656598() -> ! {
    todo!("0x656598 boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>)")
}

// 0x656704 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEC2ES7_S8_")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>)")]
pub fn stub_656704() -> ! {
    todo!("0x656704 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>)")
}

// 0x65680c — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
pub fn stub_65680c() -> ! {
    todo!("0x65680c __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS7_5list2INS7_5valueISA_EENSE_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")
}

// 0x656994 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
pub fn stub_656994() -> ! {
    todo!("0x656994 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

// 0x656b20 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEEvT_
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEEvT_")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS2_EESsENS6_5list2INS6_5valueIS9_EENSD_ISsEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>)")]
pub fn stub_656b20() -> ! {
    todo!("0x656b20 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>)")
}

// 0x656cb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// type: 
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_656cb8() -> ! {
    todo!("0x656cb8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x656cd4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPS7_E6invokeERNS1_15function_bufferESH_
// type: 
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPS7_E6invokeERNS1_15function_bufferESH_")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPS7_E6invokeERNS1_15function_bufferESH_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
pub fn stub_656cd4() -> ! {
    todo!("0x656cd4 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")
}

// 0x656cf0 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_656cf0() -> ! {
    todo!("0x656cf0 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0x656e78 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_656e78() -> ! {
    todo!("0x656e78 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x656ffc — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIS4_EESsENS8_5list2INS8_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_656ffc() -> ! {
    todo!("0x656ffc void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x657104 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX9DataModelEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::DataModel>,std::string) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
pub fn stub_657104() -> ! {
    todo!("0x657104 void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::DataModel>,std::string) &,boost::_bi::list1<RBX::DataModel*&> &,int)")
}

// 0x657270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX9DataModelEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_657270() -> ! {
    todo!("0x657270 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::DataModel>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::DataModel>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x65793c — __ZN3RBX10StudioTool10setEnabledEb
// type: _DWORD __fastcall(RBX::StudioTool *__hidden this, bool)
#[doc(alias = "__ZN3RBX10StudioTool10setEnabledEb")]
// was: __ZN3RBX10StudioTool10setEnabledEb
#[doc(alias = "RBX::StudioTool::setEnabled(bool)")]
pub fn stub_65793c() -> ! {
    todo!("0x65793c RBX::StudioTool::setEnabled(bool)")
}

// 0x65795c — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
// was: __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()")]
pub fn stub_65795c() -> ! {
    todo!("0x65795c RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()")
}

// 0x6579a4 — __ZNK3RBX10StudioTool10getEnabledEv
// type: _DWORD __fastcall(RBX::StudioTool *__hidden this)
#[doc(alias = "__ZNK3RBX10StudioTool10getEnabledEv")]
// was: __ZNK3RBX10StudioTool10getEnabledEv
#[doc(alias = "RBX::StudioTool::getEnabled(void)const")]
pub fn stub_6579a4() -> ! {
    todo!("0x6579a4 RBX::StudioTool::getEnabled(void)const")
}

// 0x657ea0 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_657ea0() -> ! {
    todo!("0x657ea0 RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x658024 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev")]
// was: __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()")]
pub fn stub_658024() -> ! {
    todo!("0x658024 RBX::Reflection::EventDesc<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::~EventDesc()")
}

// 0x6580d8 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_6580d8() -> ! {
    todo!("0x6580d8 RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x65822c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_65822c() -> ! {
    todo!("0x65822c RBX::Reflection::EventDescImpl<1,RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x65838c — __ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_65838c() -> ! {
    todo!("0x65838c RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x6589e8 — __ZN3RBX7SurfaceC1EPNS_12PartInstanceENS_8NormalIdE
// type: 
#[doc(alias = "__ZN3RBX7SurfaceC1EPNS_12PartInstanceENS_8NormalIdE")]
// was: __ZN3RBX7SurfaceC1EPNS_12PartInstanceENS_8NormalIdE
#[doc(alias = "RBX::Surface::Surface(RBX::PartInstance *,RBX::NormalId)")]
pub fn stub_6589e8() -> ! {
    todo!("0x6589e8 RBX::Surface::Surface(RBX::PartInstance *,RBX::NormalId)")
}

// 0x6589f4 — __ZN3RBX7SurfaceC1Ev
// type: _DWORD __fastcall(RBX::Surface *__hidden this)
#[doc(alias = "__ZN3RBX7SurfaceC1Ev")]
// was: __ZN3RBX7SurfaceC1Ev
#[doc(alias = "RBX::Surface::Surface(void)")]
pub fn stub_6589f4() -> ! {
    todo!("0x6589f4 RBX::Surface::Surface(void)")
}

// 0x658a00 — __ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE
// type: 
#[doc(alias = "__ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE")]
// was: __ZN3RBX7Surface14setSurfaceTypeENS_11SurfaceTypeE
#[doc(alias = "RBX::Surface::setSurfaceType(RBX::SurfaceType)")]
pub fn stub_658a00() -> ! {
    todo!("0x658a00 RBX::Surface::setSurfaceType(RBX::SurfaceType)")
}

// 0x658a0c — __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE
// type: 
#[doc(alias = "__ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE")]
// was: __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE
#[doc(alias = "RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)")]
pub fn stub_658a0c() -> ! {
    todo!("0x658a0c RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)")
}

// 0x658a18 — __ZN3RBX7Surface9setParamAEf
// type: _DWORD __fastcall(RBX::Surface *__hidden this, float)
#[doc(alias = "__ZN3RBX7Surface9setParamAEf")]
// was: __ZN3RBX7Surface9setParamAEf
#[doc(alias = "RBX::Surface::setParamA(float)")]
pub fn stub_658a18() -> ! {
    todo!("0x658a18 RBX::Surface::setParamA(float)")
}

// 0x658a24 — __ZN3RBX7Surface9setParamBEf
// type: _DWORD __fastcall(RBX::Surface *__hidden this, float)
#[doc(alias = "__ZN3RBX7Surface9setParamBEf")]
// was: __ZN3RBX7Surface9setParamBEf
#[doc(alias = "RBX::Surface::setParamB(float)")]
pub fn stub_658a24() -> ! {
    todo!("0x658a24 RBX::Surface::setParamB(float)")
}

// 0x658a30 — __ZN3RBX7Surface4flatEv
// type: 
#[doc(alias = "__ZN3RBX7Surface4flatEv")]
// was: __ZN3RBX7Surface4flatEv
#[doc(alias = "RBX::Surface::flat(void)")]
pub fn stub_658a30() -> ! {
    todo!("0x658a30 RBX::Surface::flat(void)")
}

// 0x658b6c — __ZN3RBX7Surface26registerSurfaceDescriptorsEv
// type: _DWORD __fastcall(RBX::Surface *__hidden this)
#[doc(alias = "__ZN3RBX7Surface26registerSurfaceDescriptorsEv")]
// was: __ZN3RBX7Surface26registerSurfaceDescriptorsEv
#[doc(alias = "RBX::Surface::registerSurfaceDescriptors(void)")]
pub fn stub_658b6c() -> ! {
    todo!("0x658b6c RBX::Surface::registerSurfaceDescriptors(void)")
}

// 0x658be0 — __ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE
// type: 
#[doc(alias = "__ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE")]
// was: __ZN3RBX7Surface20getSurfaceTypeStaticENS_8NormalIdE
#[doc(alias = "RBX::Surface::getSurfaceTypeStatic(RBX::NormalId)")]
pub fn stub_658be0() -> ! {
    todo!("0x658be0 RBX::Surface::getSurfaceTypeStatic(RBX::NormalId)")
}

// 0x658c70 — __ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE
// type: 
#[doc(alias = "__ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE")]
// was: __ZN3RBX7Surface21getSurfaceInputStaticENS_8NormalIdE
#[doc(alias = "RBX::Surface::getSurfaceInputStatic(RBX::NormalId)")]
pub fn stub_658c70() -> ! {
    todo!("0x658c70 RBX::Surface::getSurfaceInputStatic(RBX::NormalId)")
}

// 0x658d00 — __ZN3RBX7Surface15getParamAStaticENS_8NormalIdE
// type: 
#[doc(alias = "__ZN3RBX7Surface15getParamAStaticENS_8NormalIdE")]
// was: __ZN3RBX7Surface15getParamAStaticENS_8NormalIdE
#[doc(alias = "RBX::Surface::getParamAStatic(RBX::NormalId)")]
pub fn stub_658d00() -> ! {
    todo!("0x658d00 RBX::Surface::getParamAStatic(RBX::NormalId)")
}

// 0x658d90 — __ZN3RBX7Surface15getParamBStaticENS_8NormalIdE
// type: 
#[doc(alias = "__ZN3RBX7Surface15getParamBStaticENS_8NormalIdE")]
// was: __ZN3RBX7Surface15getParamBStaticENS_8NormalIdE
#[doc(alias = "RBX::Surface::getParamBStatic(RBX::NormalId)")]
pub fn stub_658d90() -> ! {
    todo!("0x658d90 RBX::Surface::getParamBStatic(RBX::NormalId)")
}

// 0x6590f4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev
// type: 
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev")]
// was: __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_11SurfaceTypeEED1Ev
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_6590f4() -> ! {
    todo!("0x6590f4 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")
}

// 0x659118 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev
// type: 
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev")]
// was: __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_659118() -> ! {
    todo!("0x659118 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")
}

// 0x65913c — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev
// type: 
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev")]
// was: __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE1EfED1Ev
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()")]
pub fn stub_65913c() -> ! {
    todo!("0x65913c RBX::SurfacePropDescriptor<(RBX::NormalId)1,float>::~SurfacePropDescriptor()")
}

// 0x659160 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev
// type: 
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev")]
// was: __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_11SurfaceTypeEED1Ev
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_659160() -> ! {
    todo!("0x659160 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::SurfaceType>::~SurfaceEnumPropDescriptor()")
}

// 0x659184 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev
// type: 
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev")]
// was: __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_659184() -> ! {
    todo!("0x659184 RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")
}

// 0x6591a8 — __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev
// type: 
#[doc(alias = "__ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev")]
// was: __ZN3RBX21SurfacePropDescriptorILNS_8NormalIdE4EfED1Ev
#[doc(alias = "RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()")]
pub fn stub_6591a8() -> ! {
    todo!("0x6591a8 RBX::SurfacePropDescriptor<(RBX::NormalId)4,float>::~SurfacePropDescriptor()")
}
