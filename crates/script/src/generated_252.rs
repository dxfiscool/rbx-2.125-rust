// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau (filtered 5401 exhausted) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x50cf84..0x5131e8 | EA-sorted asc distinct not yet in script (remaining 55870->55750, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50cf84]")]
pub fn stub_0x50cf84(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50cf8c]")]
pub fn stub_0x50cf8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x50d030 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::GlobalAdvancedSettings::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::GlobalAdvancedSettings::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x50d030() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalAdvancedSettings", "bool", 1)
}

// 0x50d1a8 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x50d1a8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalAdvancedSettings", "bool", 1)
}

// 0x50d1d8 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EED0Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::~BoundFuncDesc() [0x50d1d8]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EED0Ev")]
pub fn stub_0x50d1d8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x50d2a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x50d2a4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalAdvancedSettings", "bool", 1)
}

// 0x50d3e4 — __ZN3RBX10Reflection11Call1HelperINS_22GlobalAdvancedSettingsEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs — RBX::Reflection::Call1Helper<RBX::GlobalAdvancedSettings,bool (RBX::GlobalAdvancedSettings::*)(std::string),std::string,bool>::call(RBX::GlobalAdvancedSettings*,bool (RBX::GlobalAdvancedSettings::*)(std::string),RBX::Reflection::Variant &,std::string const&)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GlobalAdvancedSettings,bool (RBX::GlobalAdvancedSettings::*)(std::string),std::string,bool>::call(RBX::GlobalAdvancedSettings*,bool (RBX::GlobalAdvancedSettings::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_22GlobalAdvancedSettingsEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_0x50d3e4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::GlobalAdvancedSettings, bool (RBX::GlobalAdvancedSetting~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50d538 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EEC2EMS2_FSI_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x50d538() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

// 0x50d63c — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc() [0x50d63c]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED0Ev")]
pub fn stub_0x50d63c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x50d6f0 — __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x50d6f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

// 0x50d714 — __ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_ — RBX::Reflection::Call0Helper<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::GlobalAdvancedSettings*,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),RBX::Reflection::Variant&)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::call(RBX::GlobalAdvancedSettings*,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> (RBX::GlobalAdvancedSettings::*)(void),RBX::Reflection::Variant&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_22GlobalAdvancedSettingsEMS2_FN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvESI_E4callEPS2_SK_RS7_")]
pub fn stub_0x50d714() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

// 0x50d7fc — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EEC2EMS2_FSsSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::BoundFuncDesc(std::string (RBX::GlobalAdvancedSettings::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::BoundFuncDesc(std::string (RBX::GlobalAdvancedSettings::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EEC2EMS2_FSsSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x50d7fc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalAdvancedSettings", "std::string", 1)
}

// 0x50d974 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EE16declareSignatureEPKcNS0_7VariantE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x50d974() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalAdvancedSettings", "std::string", 1)
}

// 0x50d9a4 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EED0Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::~BoundFuncDesc() [0x50d9a4]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EED0Ev")]
pub fn stub_0x50d9a4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x50da70 — __ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x50da70() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalAdvancedSettings", "std::string", 1)
}

// 0x50dbb0 — __ZN3RBX10Reflection11Call1HelperINS_22GlobalAdvancedSettingsEMS2_FSsSsESsSsE4callEPS2_S4_RNS0_7VariantERKSs — RBX::Reflection::Call1Helper<RBX::GlobalAdvancedSettings,std::string (RBX::GlobalAdvancedSettings::*)(std::string),std::string,std::string>::call(RBX::GlobalAdvancedSettings*,std::string (RBX::GlobalAdvancedSettings::*)(std::string),RBX::Reflection::Variant &,std::string const&)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GlobalAdvancedSettings,std::string (RBX::GlobalAdvancedSettings::*)(std::string),std::string,std::string>::call(RBX::GlobalAdvancedSettings*,std::string (RBX::GlobalAdvancedSettings::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_22GlobalAdvancedSettingsEMS2_FSsSsESsSsE4callEPS2_S4_RNS0_7VariantERKSs")]
pub fn stub_0x50dbb0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::GlobalAdvancedSettings, std::string (RBX::GlobalAdvanced~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50dd7c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalBasicSettings>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19GlobalBasicSettingsEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x50dd7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlobalBasicSettings")
}

// 0x50de2c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::shared_ptr<RBX::GlobalBasicSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x50de2c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlobalBasicSettings")
}

// 0x50def4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalBasicSettings,RBX::GlobalBasicSettings>(boost::shared_ptr<RBX::GlobalBasicSettings> const*,RBX::GlobalBasicSettings *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalBasicSettings,RBX::GlobalBasicSettings>(rbx_core::SharedPtr<RBX::GlobalBasicSettings> const*,RBX::GlobalBasicSettings *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19GlobalBasicSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x50def4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlobalBasicSettings")
}

// 0x50dfdc — __ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX19GlobalBasicSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x50dfdc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e0e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x50e0e4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x50e0e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x50e0e8]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x50e0e8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x50e0ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x50e0ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e10c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x50e10c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e124 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalBasicSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19GlobalBasicSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x50e124() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e12c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings> RBX::Creatable<RBX::Instance>::create<RBX::GlobalAdvancedSettings>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_22GlobalAdvancedSettingsEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x50e12c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettings")
}

// 0x50e1dc — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::shared_ptr<RBX::GlobalAdvancedSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x50e1dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettings")
}

// 0x50e2a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalAdvancedSettings,RBX::GlobalAdvancedSettings>(boost::shared_ptr<RBX::GlobalAdvancedSettings> const*,RBX::GlobalAdvancedSettings *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GlobalAdvancedSettings,RBX::GlobalAdvancedSettings>(rbx_core::SharedPtr<RBX::GlobalAdvancedSettings> const*,RBX::GlobalAdvancedSettings *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22GlobalAdvancedSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x50e2a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettings")
}

// 0x50e38c — __ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX22GlobalAdvancedSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x50e38c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e494 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x50e494(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x50e498 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x50e498]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x50e498(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x50e49c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x50e49c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e4bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x50e4bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e4d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlobalAdvancedSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX22GlobalAdvancedSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x50e4d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50e4d8 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E — std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_0x50e4d8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x50e500 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E — std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,int>,std::_Select1st<std::pair<RBX::InstanceHandle const,int>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,int>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_iESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
pub fn stub_0x50e500() -> crate::slot::PortedFn {
// IDA 0x50e500: std::_Rb_tree<RBX::InstanceHandle, std::pair<RBX::InstanceHandle const, int>, std::_Select1st<std::pair<RBX::InstanceHan~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x50e500, "std::_Rb_tree<RBX::InstanceHandle, std::pair<RBX::InstanceHandle const, int>, std::_Select1st<std::p~")
}

// 0x50e520 — __ZN3RBX11MergeBinder10announceIDEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseE — RBX::MergeBinder::announceID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)
#[doc(alias = "RBX::MergeBinder::announceID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)")]
#[doc(alias = "__ZN3RBX11MergeBinder10announceIDEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseE")]
pub fn stub_0x50e520(handle: &crate::slot::InstanceHandle) {
// RBX::MergeBinder::announceID(XmlNameValuePair const*, RBX::Reflection::DescribedBase*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50e528 — __ZN3RBX11MergeBinder13announceIDREFEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseEPKNS_6IIDREFE — RBX::MergeBinder::announceIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)
#[doc(alias = "RBX::MergeBinder::announceIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)")]
#[doc(alias = "__ZN3RBX11MergeBinder13announceIDREFEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseEPKNS_6IIDREFE")]
pub fn stub_0x50e528(handle: &crate::slot::InstanceHandle) {
// RBX::MergeBinder::announceIDREF(XmlNameValuePair const*, RBX::Reflection::DescribedBase*, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50e58c — __ZN3RBX11MergeBinderD0Ev — RBX::MergeBinder::~MergeBinder()
#[doc(alias = "RBX::MergeBinder::~MergeBinder() [0x50e58c]")]
#[doc(alias = "__ZN3RBX11MergeBinderD0Ev")]
pub fn stub_0x50e58c(handle: crate::slot::InstanceHandle) {
// RBX::MergeBinder dtor.
drop(handle);
}

// 0x50e640 — __ZN3RBX11MergeBinder9processIDEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseE — RBX::MergeBinder::processID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)
#[doc(alias = "RBX::MergeBinder::processID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)")]
#[doc(alias = "__ZN3RBX11MergeBinder9processIDEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseE")]
pub fn stub_0x50e640(handle: &crate::slot::InstanceHandle) {
// RBX::MergeBinder::processID(XmlNameValuePair const*, RBX::Reflection::DescribedBase*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50e768 — __ZN3RBX11MergeBinder12processIDREFEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseEPKNS_6IIDREFE — RBX::MergeBinder::processIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)
#[doc(alias = "RBX::MergeBinder::processIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)")]
#[doc(alias = "__ZN3RBX11MergeBinder12processIDREFEPK16XmlNameValuePairPNS_10Reflection13DescribedBaseEPKNS_6IIDREFE")]
pub fn stub_0x50e768(handle: &crate::slot::InstanceHandle) {
// RBX::MergeBinder::processIDREF(XmlNameValuePair const*, RBX::Reflection::DescribedBase*, R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50e8d0 — __ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE9push_backERKS2_ — std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::push_back(RBX::MergeBinder::IDREFItem const&)
#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::push_back(RBX::MergeBinder::IDREFItem const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE9push_backERKS2_")]
pub fn stub_0x50e8d0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x50e92c — __ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MergeBinder::IDREFItem*,std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>>,RBX::MergeBinder::IDREFItem const&)
#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MergeBinder::IDREFItem*,std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>>,RBX::MergeBinder::IDREFItem const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x50e92c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x50ed44 — __ZNSt12_Vector_baseIN3RBX11MergeBinder9IDREFItemESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_allocate(unsigned long)
#[doc(alias = "std::_Vector_base<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX11MergeBinder9IDREFItemESaIS2_EE11_M_allocateEm")]
pub fn stub_0x50ed44() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x50ed5c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11MergeBinder9IDREFItemES6_EET0_T_S8_S7_ — RBX::MergeBinder::IDREFItem * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *>(RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *)
#[doc(alias = "RBX::MergeBinder::IDREFItem * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *>(RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11MergeBinder9IDREFItemES6_EET0_T_S8_S7_")]
pub fn stub_0x50ed5c(handle: &crate::slot::InstanceHandle) {
// RBX::MergeBinder::IDREFItem* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50edb8 — __ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_ — boost::shared_ptr<RBX::Reflection::DescribedBase> RBX::shared_from<RBX::Reflection::DescribedBase>(RBX::Reflection::DescribedBase*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase> RBX::shared_from<RBX::Reflection::DescribedBase>(RBX::Reflection::DescribedBase*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPS5_")]
pub fn stub_0x50edb8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::DescribedBase")
}

// 0x50ef24 — __ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EED2Ev — std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::~vector()
#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EED2Ev")]
pub fn stub_0x50ef24(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

// 0x50eff0 — __ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE15_M_erase_at_endEPS2_ — std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_erase_at_end(RBX::MergeBinder::IDREFItem*)
#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_erase_at_end(RBX::MergeBinder::IDREFItem*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE15_M_erase_at_endEPS2_")]
pub fn stub_0x50eff0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

// 0x50f020 — __GLOBAL__I_a_203 — global constructor keyed to_a_203
#[doc(alias = "global constructor keyed to_a_203")]
#[doc(alias = "__GLOBAL__I_a_203")]
pub fn stub_0x50f020() -> crate::slot::PortedFn {
// IDA 0x50f020: __GLOBAL__I_a_203.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x50f020, "__GLOBAL__I_a_203")
}

// 0x50f330 — __ZN3RBX7GuiBaseC2EPKc — RBX::GuiBase::GuiBase(char const*)
#[doc(alias = "RBX::GuiBase::GuiBase(char const*)")]
#[doc(alias = "__ZN3RBX7GuiBaseC2EPKc")]
pub fn stub_0x50f330() -> crate::slot::InstanceHandle {
// RBX::GuiBase ctor.
crate::slot::InstanceHandle::new("RBX::GuiBase")
}

// 0x50f494 — __ZN3RBX7GuiBaseD1Ev — RBX::GuiBase::~GuiBase()
#[doc(alias = "RBX::GuiBase::~GuiBase()")]
#[doc(alias = "__ZN3RBX7GuiBaseD1Ev")]
pub fn stub_0x50f494(handle: crate::slot::InstanceHandle) {
// RBX::GuiBase dtor.
drop(handle);
}

// 0x50f550 — __ZN3RBX7GuiBaseD0Ev — RBX::GuiBase::~GuiBase()
#[doc(alias = "RBX::GuiBase::~GuiBase() [0x50f550]")]
#[doc(alias = "__ZN3RBX7GuiBaseD0Ev")]
pub fn stub_0x50f550(handle: crate::slot::InstanceHandle) {
// RBX::GuiBase dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEE12getClassNameEv [0x50f61c]")]
pub fn stub_0x50f61c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

// 0x50f644 — __ZThn32_N3RBX7GuiBaseD1Ev — non-virtual thunk toRBX::GuiBase::~GuiBase()
#[doc(alias = "non-virtual thunk toRBX::GuiBase::~GuiBase()")]
#[doc(alias = "__ZThn32_N3RBX7GuiBaseD1Ev")]
pub fn stub_0x50f644(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x50f6fc — __ZThn32_N3RBX7GuiBaseD0Ev — non-virtual thunk toRBX::GuiBase::~GuiBase()
#[doc(alias = "non-virtual thunk toRBX::GuiBase::~GuiBase() [0x50f6fc]")]
#[doc(alias = "__ZThn32_N3RBX7GuiBaseD0Ev")]
pub fn stub_0x50f6fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEE12getClassNameEv [0x50f7cc]")]
pub fn stub_0x50f7cc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

// 0x50f7f4 — __ZThn36_N3RBX7GuiBaseD1Ev — non-virtual thunk toRBX::GuiBase::~GuiBase()
#[doc(alias = "non-virtual thunk toRBX::GuiBase::~GuiBase() [0x50f7f4]")]
#[doc(alias = "__ZThn36_N3RBX7GuiBaseD1Ev")]
pub fn stub_0x50f7f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x50f8ac — __ZThn36_N3RBX7GuiBaseD0Ev — non-virtual thunk toRBX::GuiBase::~GuiBase()
#[doc(alias = "non-virtual thunk toRBX::GuiBase::~GuiBase() [0x50f8ac]")]
#[doc(alias = "__ZThn36_N3RBX7GuiBaseD0Ev")]
pub fn stub_0x50f8ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sGuiBaseEEEEvv [0x50f97c]")]
pub fn stub_0x50f97c() -> crate::slot::PortedFn {
// IDA 0x50f97c: void RBX::Name::callDoDeclare<RBX::sGuiBase>() [0x50f97c].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x50f97c, "void RBX::Name::callDoDeclare<RBX::sGuiBase>() [0x50f97c]")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sGuiBaseEEEERKS0_v [0x50f980]")]
pub fn stub_0x50f980(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGuiBase>() [0x50f980] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50fa60]")]
pub fn stub_0x50fa60(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50fa64]")]
pub fn stub_0x50fa64(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50fb04]")]
pub fn stub_0x50fb04(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50fb0c]")]
pub fn stub_0x50fb0c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50fbb0]")]
pub fn stub_0x50fbb0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7GuiBaseELZNS_8sGuiBaseEENS_17NonFactoryProductINS_8InstanceELZNS_8sGuiBaseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50fbb8]")]
pub fn stub_0x50fbb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x50fc5c — __GLOBAL__I_a_204 — global constructor keyed to_a_204
#[doc(alias = "global constructor keyed to_a_204")]
#[doc(alias = "__GLOBAL__I_a_204")]
pub fn stub_0x50fc5c() -> crate::slot::PortedFn {
// IDA 0x50fc5c: __GLOBAL__I_a_204.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x50fc5c, "__GLOBAL__I_a_204")
}

// 0x50fdf4 — __ZN3RBX9GuiBase3d8setColorENS_10BrickColorE — RBX::GuiBase3d::setColor(RBX::BrickColor)
#[doc(alias = "RBX::GuiBase3d::setColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX9GuiBase3d8setColorENS_10BrickColorE")]
pub fn stub_0x50fdf4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::GuiBase3d setter.
cell.set(value)
}

// 0x50fe10 — __ZN3RBX9GuiBase3d15setTransparencyEf — RBX::GuiBase3d::setTransparency(float)
#[doc(alias = "RBX::GuiBase3d::setTransparency(float)")]
#[doc(alias = "__ZN3RBX9GuiBase3d15setTransparencyEf")]
pub fn stub_0x50fe10(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::GuiBase3d setter.
cell.set(value)
}

// 0x50fe38 — __ZN3RBX9GuiBase3d10setVisibleEb — RBX::GuiBase3d::setVisible(bool)
#[doc(alias = "RBX::GuiBase3d::setVisible(bool)")]
#[doc(alias = "__ZN3RBX9GuiBase3d10setVisibleEb")]
pub fn stub_0x50fe38(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::GuiBase3d setter.
cell.set(value)
}

// 0x50fe6c — __ZN3RBX9GuiBase3dC2EPKc — RBX::GuiBase3d::GuiBase3d(char const*)
#[doc(alias = "RBX::GuiBase3d::GuiBase3d(char const*)")]
#[doc(alias = "__ZN3RBX9GuiBase3dC2EPKc")]
pub fn stub_0x50fe6c() -> crate::slot::InstanceHandle {
// RBX::GuiBase3d ctor.
crate::slot::InstanceHandle::new("RBX::GuiBase3d")
}

// 0x510000 — __ZNK3RBX9GuiBase3d8getColorEv — RBX::GuiBase3d::getColor(void)const
#[doc(alias = "RBX::GuiBase3d::getColor(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase3d8getColorEv")]
pub fn stub_0x510000(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiBase3d getter.
cell.get()
}

// 0x510008 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEED1Ev — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEED1Ev")]
pub fn stub_0x510008(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x51002c — __ZNK3RBX9GuiBase3d15getTransparencyEv — RBX::GuiBase3d::getTransparency(void)const
#[doc(alias = "RBX::GuiBase3d::getTransparency(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase3d15getTransparencyEv")]
pub fn stub_0x51002c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiBase3d getter.
cell.get()
}

// 0x510030 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfED1Ev — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfED1Ev")]
pub fn stub_0x510030(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510054 — __ZNK3RBX9GuiBase3d10getVisibleEv — RBX::GuiBase3d::getVisible(void)const
#[doc(alias = "RBX::GuiBase3d::getVisible(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase3d10getVisibleEv")]
pub fn stub_0x510054(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiBase3d getter.
cell.get()
}

// 0x51005c — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbED1Ev — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbED1Ev")]
pub fn stub_0x51005c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510080 — __ZN3RBX9GuiBase3dD1Ev — RBX::GuiBase3d::~GuiBase3d()
#[doc(alias = "RBX::GuiBase3d::~GuiBase3d()")]
#[doc(alias = "__ZN3RBX9GuiBase3dD1Ev")]
pub fn stub_0x510080(handle: crate::slot::InstanceHandle) {
// RBX::GuiBase3d dtor.
drop(handle);
}

// 0x51013c — __ZN3RBX9GuiBase3dD0Ev — RBX::GuiBase3d::~GuiBase3d()
#[doc(alias = "RBX::GuiBase3d::~GuiBase3d() [0x51013c]")]
#[doc(alias = "__ZN3RBX9GuiBase3dD0Ev")]
pub fn stub_0x51013c(handle: crate::slot::InstanceHandle) {
// RBX::GuiBase3d dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEE12getClassNameEv [0x510208]")]
pub fn stub_0x510208() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase"
}

// 0x510230 — __ZThn32_N3RBX9GuiBase3dD1Ev — non-virtual thunk toRBX::GuiBase3d::~GuiBase3d()
#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::~GuiBase3d()")]
#[doc(alias = "__ZThn32_N3RBX9GuiBase3dD1Ev")]
pub fn stub_0x510230(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5102e8 — __ZThn32_N3RBX9GuiBase3dD0Ev — non-virtual thunk toRBX::GuiBase3d::~GuiBase3d()
#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::~GuiBase3d() [0x5102e8]")]
#[doc(alias = "__ZThn32_N3RBX9GuiBase3dD0Ev")]
pub fn stub_0x5102e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEE12getClassNameEv [0x5103b8]")]
pub fn stub_0x5103b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase"
}

// 0x5103e0 — __ZThn36_N3RBX9GuiBase3dD1Ev — non-virtual thunk toRBX::GuiBase3d::~GuiBase3d()
#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::~GuiBase3d() [0x5103e0]")]
#[doc(alias = "__ZThn36_N3RBX9GuiBase3dD1Ev")]
pub fn stub_0x5103e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x510498 — __ZThn36_N3RBX9GuiBase3dD0Ev — non-virtual thunk toRBX::GuiBase3d::~GuiBase3d()
#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::~GuiBase3d() [0x510498]")]
#[doc(alias = "__ZThn36_N3RBX9GuiBase3dD0Ev")]
pub fn stub_0x510498(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sGuiBase3dEEEEvv [0x510568]")]
pub fn stub_0x510568() -> crate::slot::PortedFn {
// IDA 0x510568: void RBX::Name::callDoDeclare<RBX::sGuiBase3d>() [0x510568].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x510568, "void RBX::Name::callDoDeclare<RBX::sGuiBase3d>() [0x510568]")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sGuiBase3dEEEERKS0_v [0x51056c]")]
pub fn stub_0x51056c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGuiBase3d>() [0x51056c] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiBase3dELZNS_10sGuiBase3dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x51064c]")]
pub fn stub_0x51064c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiBase3dELZNS_10sGuiBase3dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x510708]")]
pub fn stub_0x510708(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiBase3dELZNS_10sGuiBase3dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x5107d4]")]
pub fn stub_0x5107d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiBase3dELZNS_10sGuiBase3dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x51088c]")]
pub fn stub_0x51088c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiBase3dELZNS_10sGuiBase3dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x51095c]")]
pub fn stub_0x51095c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiBase3dELZNS_10sGuiBase3dEENS_17NonFactoryProductINS_7GuiBaseELZNS_10sGuiBase3dEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x510a14]")]
pub fn stub_0x510a14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x510ae4 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::PropDescriptor<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>(char const*,char const*,bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::PropDescriptor<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>(char const*,char const*,bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x510ae4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510bf8 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbED0Ev — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::~PropDescriptor() [0x510bf8]")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbED0Ev")]
pub fn stub_0x510bf8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510c24 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_0x510c24(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510c28 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_0x510c28(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510c2c — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x510c2c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510c50 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,bool>::GetSetImpl<bool (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x510c50(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510c74 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::PropDescriptor<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>(char const*,char const*,float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::PropDescriptor<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>(char const*,char const*,float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x510c74(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510d88 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfED0Ev — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::~PropDescriptor() [0x510d88]")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfED0Ev")]
pub fn stub_0x510d88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510db4 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
pub fn stub_0x510db4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510db8 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
pub fn stub_0x510db8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510dbc — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x510dbc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510ddc — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,float>::GetSetImpl<float (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_0x510ddc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510e00 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x510e00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510f14 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEED0Ev — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::~PropDescriptor() [0x510f14]")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEED0Ev")]
pub fn stub_0x510f14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x510f40 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::isReadOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x510f40(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510f44 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::isWriteOnly(void)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x510f44(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510f48 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x510f48(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510f70 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_ — RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase3d,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiBase3d::*)(void)const,void (RBX::GuiBase3d::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9GuiBase3dENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x510f70(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x510f94 — __GLOBAL__I_a_205 — global constructor keyed to_a_205
#[doc(alias = "global constructor keyed to_a_205")]
#[doc(alias = "__GLOBAL__I_a_205")]
pub fn stub_0x510f94() -> crate::slot::PortedFn {
// IDA 0x510f94: __GLOBAL__I_a_205.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x510f94, "__GLOBAL__I_a_205")
}

// 0x511244 — __ZN3RBX22GetCustomStatsFilenameEv — RBX::GetCustomStatsFilename(void)
#[doc(alias = "RBX::GetCustomStatsFilename(void)")]
#[doc(alias = "__ZN3RBX22GetCustomStatsFilenameEv")]
pub fn stub_0x511244() -> crate::slot::PortedFn {
// IDA 0x511244: RBX::GetCustomStatsFilename().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x511244, "RBX::GetCustomStatsFilename()")
}

// 0x511390 — __ZN3RBX18CustomStatsGuiJSON14DefaultHandlerERKSsS2_ — RBX::CustomStatsGuiJSON::DefaultHandler(std::string const&,std::string const&)
#[doc(alias = "RBX::CustomStatsGuiJSON::DefaultHandler(std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX18CustomStatsGuiJSON14DefaultHandlerERKSsS2_")]
pub fn stub_0x511390(handle: &crate::slot::InstanceHandle) {
// RBX::CustomStatsGuiJSON::DefaultHandler(std::string const&, std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x511d68 — __ZN3RBX18CustomStatsGuiJSON9WriteFileEv — RBX::CustomStatsGuiJSON::WriteFile(void)
#[doc(alias = "RBX::CustomStatsGuiJSON::WriteFile(void)")]
#[doc(alias = "__ZN3RBX18CustomStatsGuiJSON9WriteFileEv")]
pub fn stub_0x511d68(handle: &crate::slot::InstanceHandle) {
// RBX::CustomStatsGuiJSON::WriteFile() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x512280 — __ZN3RBX10GuiBuilder15getDebugDisplayEv — RBX::GuiBuilder::getDebugDisplay(void)
#[doc(alias = "RBX::GuiBuilder::getDebugDisplay(void)")]
#[doc(alias = "__ZN3RBX10GuiBuilder15getDebugDisplayEv")]
pub fn stub_0x512280(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiBuilder getter.
cell.get()
}

// 0x512290 — __ZN3RBX10GuiBuilder15setDebugDisplayENS0_7DisplayE — RBX::GuiBuilder::setDebugDisplay(RBX::GuiBuilder::Display)
#[doc(alias = "RBX::GuiBuilder::setDebugDisplay(RBX::GuiBuilder::Display)")]
#[doc(alias = "__ZN3RBX10GuiBuilder15setDebugDisplayENS0_7DisplayE")]
pub fn stub_0x512290(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::GuiBuilder setter.
cell.set(value)
}

// 0x5122a0 — __ZN3RBX10GuiBuilder7getVerbERKSs — RBX::GuiBuilder::getVerb(std::string const&)
#[doc(alias = "RBX::GuiBuilder::getVerb(std::string const&)")]
#[doc(alias = "__ZN3RBX10GuiBuilder7getVerbERKSs")]
pub fn stub_0x5122a0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GuiBuilder getter.
cell.get()
}

// 0x512308 — __ZN3RBX10GuiBuilder8buildGuiEPNS_5AdornEPNS_9WorkspaceEb — RBX::GuiBuilder::buildGui(RBX::Adorn *,RBX::Workspace *,bool)
#[doc(alias = "RBX::GuiBuilder::buildGui(RBX::Adorn *,RBX::Workspace *,bool)")]
#[doc(alias = "__ZN3RBX10GuiBuilder8buildGuiEPNS_5AdornEPNS_9WorkspaceEb")]
pub fn stub_0x512308(handle: &crate::slot::InstanceHandle) {
// RBX::GuiBuilder::buildGui(RBX::Adorn*, RBX::Workspace*, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5131e8 — __ZN3RBX10GuiBuilder14buildStatsHud1Ev — RBX::GuiBuilder::buildStatsHud1(void)
#[doc(alias = "RBX::GuiBuilder::buildStatsHud1(void)")]
#[doc(alias = "__ZN3RBX10GuiBuilder14buildStatsHud1Ev")]
pub fn stub_0x5131e8(handle: &crate::slot::InstanceHandle) {
// RBX::GuiBuilder::buildStatsHud1() — engine-side; linkage preserved via the alias.
let _ = handle;
}
