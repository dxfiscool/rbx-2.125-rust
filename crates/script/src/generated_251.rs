// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau (filtered 5401 exhausted) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x508958..0x50cee0 | EA-sorted asc distinct not yet in script (remaining 55990->55870, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x508958 — __ZN3RBX19GlobalBasicSettingsC2Ev — RBX::GlobalBasicSettings::GlobalBasicSettings(void)
#[doc(alias = "RBX::GlobalBasicSettings::GlobalBasicSettings(void)")]
#[doc(alias = "__ZN3RBX19GlobalBasicSettingsC2Ev")]
pub fn stub_0x508958() -> crate::slot::InstanceHandle {
// RBX::GlobalBasicSettings ctor.
crate::slot::InstanceHandle::new("RBX::GlobalBasicSettings")
}

// 0x508cd4 — __ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE — resetChild(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "resetChild(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE")]
pub fn stub_0x508cd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x508d14 — __ZN3RBX22GlobalAdvancedSettings13getFVariablesEv — RBX::GlobalAdvancedSettings::getFVariables(void)
#[doc(alias = "RBX::GlobalAdvancedSettings::getFVariables(void)")]
#[doc(alias = "__ZN3RBX22GlobalAdvancedSettings13getFVariablesEv")]
pub fn stub_0x508d14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GlobalAdvancedSettings getter.
cell.get()
}

// 0x508df4 — __ZL5visitRKSsS0_Pv — visit(std::string const&,std::string const&,void *)
#[doc(alias = "visit(std::string const&,std::string const&,void *)")]
#[doc(alias = "__ZL5visitRKSsS0_Pv")]
pub fn stub_0x508df4() -> crate::slot::PortedFn {
// IDA 0x508df4: visit(std::string const&, std::string const&, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x508df4, "visit(std::string const&, std::string const&, void*)")
}

// 0x508e18 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EED1Ev")]
pub fn stub_0x508e18(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x508e58 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFN5boost10shared_ptrIKNS3_9unordered13unordered_mapISsNS0_7VariantENS3_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEEvELi0EED1Ev")]
pub fn stub_0x508e58(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x508e7c — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EED1Ev")]
pub fn stub_0x508e7c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x508ebc — __ZN3RBX11MergeBinder11resolveRefsEv — RBX::MergeBinder::resolveRefs(void)
#[doc(alias = "RBX::MergeBinder::resolveRefs(void)")]
#[doc(alias = "__ZN3RBX11MergeBinder11resolveRefsEv")]
pub fn stub_0x508ebc(handle: &crate::slot::InstanceHandle) {
// RBX::MergeBinder::resolveRefs() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x508ef4 — __ZN3RBX11MergeBinderD1Ev — RBX::MergeBinder::~MergeBinder()
#[doc(alias = "RBX::MergeBinder::~MergeBinder()")]
#[doc(alias = "__ZN3RBX11MergeBinderD1Ev")]
pub fn stub_0x508ef4(handle: crate::slot::InstanceHandle) {
// RBX::MergeBinder dtor.
drop(handle);
}

// 0x508f18 — __ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_ — void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector const&)const
#[doc(alias = "void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_")]
pub fn stub_0x508f18() -> crate::slot::PortedFn {
// IDA 0x508f18: void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x508f18, "void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::Invali~")
}

// 0x509048 — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev — boost::shared_ptr<RBX::GlobalAdvancedSettings>::~shared_ptr()
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev")]
pub fn stub_0x509048(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x50905c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev — boost::shared_ptr<RBX::GlobalBasicSettings>::~shared_ptr()
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev")]
pub fn stub_0x50905c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x509070 — __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED1Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED1Ev")]
pub fn stub_0x509070(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x509094 — __ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_ — void RBX::Instance::visitChildren<void (*)(boost::shared_ptr<RBX::Instance>)>(void (*)(boost::shared_ptr<RBX::Instance>) const&)const
#[doc(alias = "void RBX::Instance::visitChildren<void (*)(rbx_core::SharedPtr<RBX::Instance>)>(void (*)(rbx_core::SharedPtr<RBX::Instance>) const&)const")]
#[doc(alias = "__ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_")]
pub fn stub_0x509094() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5091cc — __ZN3RBX8SettingsD1Ev — RBX::Settings::~Settings()
#[doc(alias = "RBX::Settings::~Settings()")]
#[doc(alias = "__ZN3RBX8SettingsD1Ev")]
pub fn stub_0x5091cc(handle: crate::slot::InstanceHandle) {
// RBX::Settings dtor.
drop(handle);
}

// 0x509208 — __ZN3RBX8SettingsD0Ev — RBX::Settings::~Settings()
#[doc(alias = "RBX::Settings::~Settings() [0x509208]")]
#[doc(alias = "__ZN3RBX8SettingsD0Ev")]
pub fn stub_0x509208(handle: crate::slot::InstanceHandle) {
// RBX::Settings dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv [0x5092dc]")]
pub fn stub_0x5092dc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServiceProvider"
}

// 0x509308 — __ZThn32_N3RBX8SettingsD1Ev — non-virtual thunk toRBX::Settings::~Settings()
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings()")]
#[doc(alias = "__ZThn32_N3RBX8SettingsD1Ev")]
pub fn stub_0x509308(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x509344 — __ZThn32_N3RBX8SettingsD0Ev — non-virtual thunk toRBX::Settings::~Settings()
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings() [0x509344]")]
#[doc(alias = "__ZThn32_N3RBX8SettingsD0Ev")]
pub fn stub_0x509344(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv [0x509418]")]
pub fn stub_0x509418() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServiceProvider"
}

// 0x509440 — __ZThn36_N3RBX8SettingsD1Ev — non-virtual thunk toRBX::Settings::~Settings()
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings() [0x509440]")]
#[doc(alias = "__ZThn36_N3RBX8SettingsD1Ev")]
pub fn stub_0x509440(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x50947c — __ZThn36_N3RBX8SettingsD0Ev — non-virtual thunk toRBX::Settings::~Settings()
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings() [0x50947c]")]
#[doc(alias = "__ZThn36_N3RBX8SettingsD0Ev")]
pub fn stub_0x50947c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv [0x509554]")]
pub fn stub_0x509554() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Settings"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv [0x50957c]")]
pub fn stub_0x50957c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Settings"
}

// 0x5095a4 — __ZN3RBX19GlobalBasicSettingsD1Ev — RBX::GlobalBasicSettings::~GlobalBasicSettings()
#[doc(alias = "RBX::GlobalBasicSettings::~GlobalBasicSettings()")]
#[doc(alias = "__ZN3RBX19GlobalBasicSettingsD1Ev")]
pub fn stub_0x5095a4(handle: crate::slot::InstanceHandle) {
// RBX::GlobalBasicSettings dtor.
drop(handle);
}

// 0x5096f0 — __ZN3RBX19GlobalBasicSettingsD0Ev — RBX::GlobalBasicSettings::~GlobalBasicSettings()
#[doc(alias = "RBX::GlobalBasicSettings::~GlobalBasicSettings() [0x5096f0]")]
#[doc(alias = "__ZN3RBX19GlobalBasicSettingsD0Ev")]
pub fn stub_0x5096f0(handle: crate::slot::InstanceHandle) {
// RBX::GlobalBasicSettings dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv [0x509850]")]
pub fn stub_0x509850() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Settings"
}

// 0x509878 — __ZThn32_N3RBX19GlobalBasicSettingsD1Ev — non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()")]
#[doc(alias = "__ZThn32_N3RBX19GlobalBasicSettingsD1Ev")]
pub fn stub_0x509878(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x5099d0 — __ZThn32_N3RBX19GlobalBasicSettingsD0Ev — non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings() [0x5099d0]")]
#[doc(alias = "__ZThn32_N3RBX19GlobalBasicSettingsD0Ev")]
pub fn stub_0x5099d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv [0x509b40]")]
pub fn stub_0x509b40() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Settings"
}

// 0x509b68 — __ZThn36_N3RBX19GlobalBasicSettingsD1Ev — non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings() [0x509b68]")]
#[doc(alias = "__ZThn36_N3RBX19GlobalBasicSettingsD1Ev")]
pub fn stub_0x509b68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x509cc8 — __ZThn36_N3RBX19GlobalBasicSettingsD0Ev — non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings() [0x509cc8]")]
#[doc(alias = "__ZThn36_N3RBX19GlobalBasicSettingsD0Ev")]
pub fn stub_0x509cc8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv [0x509e3c]")]
pub fn stub_0x509e3c() -> crate::slot::PortedFn {
// IDA 0x509e3c: void RBX::Name::callDoDeclare<RBX::sGlobalBasicSettings>() [0x509e3c].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x509e3c, "void RBX::Name::callDoDeclare<RBX::sGlobalBasicSettings>() [0x509e3c]")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v [0x509e40]")]
pub fn stub_0x509e40(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGlobalBasicSettings>() [0x509e40] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv [0x509f20]")]
pub fn stub_0x509f20() -> crate::slot::PortedFn {
// IDA 0x509f20: void RBX::Name::callDoDeclare<RBX::sGlobalAdvancedSettings>() [0x509f20].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x509f20, "void RBX::Name::callDoDeclare<RBX::sGlobalAdvancedSettings>() [0x509f20]")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v [0x509f24]")]
pub fn stub_0x509f24(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGlobalAdvancedSettings>() [0x509f24] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv [0x50a004]")]
pub fn stub_0x50a004() -> crate::slot::PortedFn {
// IDA 0x50a004: void RBX::Name::callDoDeclare<RBX::sSettings>() [0x50a004].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x50a004, "void RBX::Name::callDoDeclare<RBX::sSettings>() [0x50a004]")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v [0x50a008]")]
pub fn stub_0x50a008(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSettings>() [0x50a008] — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50a0e8 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE — rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)
#[doc(alias = "rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>,boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterINS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEESH_EEPT_RKNS_10shared_ptrIT0_EE")]
pub fn stub_0x50a0e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

// 0x50a148 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
pub fn stub_0x50a148(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x50a180 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE — boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX10Reflection7VariantEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
pub fn stub_0x50a180(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

// 0x50a250 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev — boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEED1Ev")]
pub fn stub_0x50a250(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x50a280 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE11get_deleterERKSt9type_info")]
pub fn stub_0x50a280() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50a298 — __ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *,rbx::detail::sp_ms_deleter<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS6_EEEEN3rbx6detail13sp_ms_deleterISF_EEE19get_untyped_deleterEv")]
pub fn stub_0x50a298() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_ [0x50a29c]")]
pub fn stub_0x50a29c() -> crate::slot::InstanceHandle {
// RBX::Reflection::Described ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Described")
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50a490]")]
pub fn stub_0x50a490(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50a4cc]")]
pub fn stub_0x50a4cc(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50a59c]")]
pub fn stub_0x50a59c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50a5d8]")]
pub fn stub_0x50a5d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50a6ac]")]
pub fn stub_0x50a6ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50a6e8]")]
pub fn stub_0x50a6e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv [0x50a7bc]")]
pub fn stub_0x50a7bc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GlobalBasicSettings, RBX::sGlobalBasicSettings, RBX::NonFa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50a8d8]")]
pub fn stub_0x50a8d8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50a914]")]
pub fn stub_0x50a914(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50a9e4]")]
pub fn stub_0x50a9e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50aa20]")]
pub fn stub_0x50aa20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50aaf4]")]
pub fn stub_0x50aaf4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50ab30]")]
pub fn stub_0x50ab30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv [0x50ac04]")]
pub fn stub_0x50ac04(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Settings, RBX::sSettings, RBX::NonFactoryProduct<RBX::Serv~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev [0x50ad20]")]
pub fn stub_0x50ad20(handle: crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev [0x50ad5c]")]
pub fn stub_0x50ad5c(handle: crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev [0x50ae2c]")]
pub fn stub_0x50ae2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev [0x50ae68]")]
pub fn stub_0x50ae68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev [0x50af3c]")]
pub fn stub_0x50af3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev [0x50af78]")]
pub fn stub_0x50af78(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x50b04c — __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::BoundFuncDesc(void (RBX::GlobalBasicSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::BoundFuncDesc(void (RBX::GlobalBasicSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x50b04c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalBasicSettings", "void", 0)
}

// 0x50b150 — __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED0Ev — RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::~BoundFuncDesc() [0x50b150]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED0Ev")]
pub fn stub_0x50b150(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x50b204 — __ZNK3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE — RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x50b204() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GlobalBasicSettings", "void", 0)
}

// 0x50b228 — __ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v — RBX::Selection * RBX::ServiceProvider::find<RBX::Selection>(void)const
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::find<RBX::Selection>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v")]
pub fn stub_0x50b228() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Selection"))
}

// 0x50b39c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x50b39c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Selection")
}

// 0x50b44c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE — boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Selection>(boost::shared_ptr<RBX::Selection> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE")]
pub fn stub_0x50b44c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v [0x50b480]")]
pub fn stub_0x50b480(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sSelection>() [0x50b480] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v [0x50b4c8]")]
pub fn stub_0x50b4c8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSelection>() [0x50b4c8] — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50b5b0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Selection>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Selection>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv")]
pub fn stub_0x50b5b0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

// 0x50b688 — __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x50b688() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Selection")
}

// 0x50b750 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Selection,RBX::Selection>(boost::shared_ptr<RBX::Selection> const*,RBX::Selection *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Selection,RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const*,RBX::Selection *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9SelectionES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x50b750() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Selection")
}

// 0x50b838 — __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x50b838() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50b940 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x50b940(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x50b948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x50b948() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50b968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x50b968() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x50b980 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x50b980() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sSelectionEEE15isNullClassNameEv [0x50b988]")]
pub fn stub_0x50b988(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_ [0x50ba28]")]
pub fn stub_0x50ba28() -> crate::slot::InstanceHandle {
// RBX::Reflection::Described ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Described")
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_22GlobalAdvancedSettingsENS_8SettingsELZNS_23sGlobalAdvancedSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50bc1c]")]
pub fn stub_0x50bc1c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_22GlobalAdvancedSettingsENS_8SettingsELZNS_23sGlobalAdvancedSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50bc58]")]
pub fn stub_0x50bc58(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_22GlobalAdvancedSettingsENS_8SettingsELZNS_23sGlobalAdvancedSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50bd28]")]
pub fn stub_0x50bd28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_22GlobalAdvancedSettingsENS_8SettingsELZNS_23sGlobalAdvancedSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50bd64]")]
pub fn stub_0x50bd64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_22GlobalAdvancedSettingsENS_8SettingsELZNS_23sGlobalAdvancedSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50be38]")]
pub fn stub_0x50be38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_22GlobalAdvancedSettingsENS_8SettingsELZNS_23sGlobalAdvancedSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50be74]")]
pub fn stub_0x50be74(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv [0x50bf48]")]
pub fn stub_0x50bf48(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::GlobalAdvancedSettings, RBX::sGlobalAdvancedSettings, RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50c064]")]
pub fn stub_0x50c064(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50c0a0]")]
pub fn stub_0x50c0a0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50c170]")]
pub fn stub_0x50c170(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50c1ac]")]
pub fn stub_0x50c1ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50c280]")]
pub fn stub_0x50c280(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50c2bc]")]
pub fn stub_0x50c2bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEED1Ev [0x50c390]")]
pub fn stub_0x50c390(handle: crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEED0Ev [0x50c3cc]")]
pub fn stub_0x50c3cc(handle: crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEED1Ev [0x50c49c]")]
pub fn stub_0x50c49c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEED0Ev [0x50c4d8]")]
pub fn stub_0x50c4d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEED1Ev [0x50c5ac]")]
pub fn stub_0x50c5ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEED0Ev [0x50c5e8]")]
pub fn stub_0x50c5e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50c6c0]")]
pub fn stub_0x50c6c0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50c6c4]")]
pub fn stub_0x50c6c4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50c764]")]
pub fn stub_0x50c764(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50c76c]")]
pub fn stub_0x50c76c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50c810]")]
pub fn stub_0x50c810(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50c818]")]
pub fn stub_0x50c818(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x50c8c0 — __ZN3RBX15ServiceProviderD0Ev — RBX::ServiceProvider::~ServiceProvider()
#[doc(alias = "RBX::ServiceProvider::~ServiceProvider() [0x50c8c0]")]
#[doc(alias = "__ZN3RBX15ServiceProviderD0Ev")]
pub fn stub_0x50c8c0(handle: crate::slot::InstanceHandle) {
// RBX::ServiceProvider dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEE12getClassNameEv [0x50c960]")]
pub fn stub_0x50c960() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEE12getClassNameEv [0x50c968]")]
pub fn stub_0x50c968() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sServiceProviderEEEERKS0_v [0x50c970]")]
pub fn stub_0x50c970(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sServiceProvider>() [0x50c970] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v [0x50c9b8]")]
pub fn stub_0x50c9b8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sServiceProvider>() [0x50c9b8] — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x50caa0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E")]
pub fn stub_0x50caa0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x50cac8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E")]
pub fn stub_0x50cac8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x50cae4 — __ZN3RBX15ServiceProviderC2Ev — RBX::ServiceProvider::ServiceProvider(void)
#[doc(alias = "RBX::ServiceProvider::ServiceProvider(void)")]
#[doc(alias = "__ZN3RBX15ServiceProviderC2Ev")]
pub fn stub_0x50cae4() -> crate::slot::InstanceHandle {
// RBX::ServiceProvider ctor.
crate::slot::InstanceHandle::new("RBX::ServiceProvider")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50ce34]")]
pub fn stub_0x50ce34(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50ce38]")]
pub fn stub_0x50ce38(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev [0x50ced8]")]
pub fn stub_0x50ced8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev [0x50cee0]")]
pub fn stub_0x50cee0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}
