//! rendering shard 368 — 100 stubs 0x506ef8..0x50ba28 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 40060->40160 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x506ef8..0x50ba28 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x506ef8 — __ZThn36_N3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x506ef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_506ef8() {
}


// 0x506f00 — __ZThn36_N3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x506f00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_506f00() {
}


// 0x507098 — __ZN3RBX17FilterDescendentsD0Ev
// type: void __fastcall(RBX::FilterDescendents *__hidden this)
#[doc(alias = "RBX::FilterDescendents::~FilterDescendents()")]
// was: __ZN3RBX17FilterDescendentsD0Ev
// IDA 0x507098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_507098() {
}


// 0x507150 — __GLOBAL__I_a_202
#[doc(alias = "global constructor keyed to_a_202")]
// was: __GLOBAL__I_a_202
// IDA 0x507150: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_507150() {
}


// 0x5073c0 — __ZN3RBX22GlobalAdvancedSettings12getFVariableESs
#[doc(alias = "RBX::GlobalAdvancedSettings::getFVariable(std::string)")]
// was: __ZN3RBX22GlobalAdvancedSettings12getFVariableESs
// IDA 0x5073c0: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5073c0() {
}


// 0x5075a4 — __ZN3RBX22GlobalAdvancedSettings8getFFlagESs
#[doc(alias = "RBX::GlobalAdvancedSettings::getFFlag(std::string)")]
// was: __ZN3RBX22GlobalAdvancedSettings8getFFlagESs
// IDA 0x5075a4: 217 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5075a4() {
}


// 0x507808 — __ZN3RBX8SettingsC2ERKSs
// type: _DWORD __fastcall(RBX::Settings *__hidden this, const std::string *)
#[doc(alias = "RBX::Settings::Settings(std::string const&)")]
// was: __ZN3RBX8SettingsC2ERKSs
// IDA 0x507808: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_507808() {
}


// 0x507970 — __ZN3RBX8Settings25InvalidDescendentDetector7invalidEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Settings::InvalidDescendentDetector *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Settings::InvalidDescendentDetector::invalid(RBX::Instance const*)")]
// was: __ZN3RBX8Settings25InvalidDescendentDetector7invalidEPKNS_8InstanceE
// IDA 0x507970: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_507970() {
}


// 0x5079f8 — __ZN3RBX8Settings9loadStateERKSs
// type: _DWORD __fastcall(RBX::Settings *__hidden this, const std::string *)
#[doc(alias = "RBX::Settings::loadState(std::string const&)")]
// was: __ZN3RBX8Settings9loadStateERKSs
// IDA 0x5079f8: 348 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5079f8() {
}


// 0x507da4 — __ZNK3RBX8Settings19verifyAddDescendantEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::Settings *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Settings::verifyAddDescendant(RBX::Instance const*,RBX::Instance const*)const")]
// was: __ZNK3RBX8Settings19verifyAddDescendantEPKNS_8InstanceES3_
// IDA 0x507da4: 83 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_507da4() {
}


// 0x507eac — __Z21initAdvancedSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "initAdvancedSingleton(void)")]
// was: __Z21initAdvancedSingletonv
// IDA 0x507eac: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_507eac() {
}


// 0x507fdc — __ZL19doAdvancedSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "doAdvancedSingleton(void)")]
// was: __ZL19doAdvancedSingletonv
// IDA 0x507fdc: 85 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_507fdc() {
}


// 0x5080ec — __ZN3RBX22GlobalAdvancedSettings9singletonEv
// type: _DWORD __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "RBX::GlobalAdvancedSettings::singleton(void)")]
// was: __ZN3RBX22GlobalAdvancedSettings9singletonEv
// IDA 0x5080ec: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5080ec() {
}


// 0x508114 — __ZN3RBX22GlobalAdvancedSettingsC2Ev
// type: _DWORD __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "RBX::GlobalAdvancedSettings::GlobalAdvancedSettings(void)")]
// was: __ZN3RBX22GlobalAdvancedSettingsC2Ev
// IDA 0x508114: 323 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508114() {
}


// 0x50849c — __ZN3RBX22GlobalAdvancedSettingsD0Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZN3RBX22GlobalAdvancedSettingsD0Ev
// IDA 0x50849c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50849c() {
}


// 0x50853c — __ZN3RBX22GlobalAdvancedSettingsD1Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZN3RBX22GlobalAdvancedSettingsD1Ev
// IDA 0x50853c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_50853c() {
}


// 0x508540 — __ZThn32_N3RBX22GlobalAdvancedSettingsD0Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZThn32_N3RBX22GlobalAdvancedSettingsD0Ev
// IDA 0x508540: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508540() {
}


// 0x508548 — __ZThn36_N3RBX22GlobalAdvancedSettingsD0Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZThn36_N3RBX22GlobalAdvancedSettingsD0Ev
// IDA 0x508548: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508548() {
}


// 0x508550 — __ZN3RBX22GlobalAdvancedSettingsD2Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZN3RBX22GlobalAdvancedSettingsD2Ev
// IDA 0x508550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508550() {
}


// 0x5086d0 — __ZThn32_N3RBX22GlobalAdvancedSettingsD1Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZThn32_N3RBX22GlobalAdvancedSettingsD1Ev
// IDA 0x5086d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5086d0() {
}


// 0x5086d8 — __ZThn36_N3RBX22GlobalAdvancedSettingsD1Ev
// type: void __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// was: __ZThn36_N3RBX22GlobalAdvancedSettingsD1Ev
// IDA 0x5086d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5086d8() {
}


// 0x5086e0 — __Z18initBasicSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "initBasicSingleton(void)")]
// was: __Z18initBasicSingletonv
// IDA 0x5086e0: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5086e0() {
}


// 0x508810 — __ZL16doBasicSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "doBasicSingleton(void)")]
// was: __ZL16doBasicSingletonv
// IDA 0x508810: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508810() {
}


// 0x508914 — __ZN3RBX19GlobalBasicSettings9singletonEv
// type: _DWORD __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "RBX::GlobalBasicSettings::singleton(void)")]
// was: __ZN3RBX19GlobalBasicSettings9singletonEv
// IDA 0x508914: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508914() {
}


// 0x50893c — __ZN3RBX19GlobalBasicSettings5resetEv
// type: _DWORD __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "RBX::GlobalBasicSettings::reset(void)")]
// was: __ZN3RBX19GlobalBasicSettings5resetEv
// IDA 0x50893c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50893c() {
}


// 0x508958 — __ZN3RBX19GlobalBasicSettingsC2Ev
// type: _DWORD __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "RBX::GlobalBasicSettings::GlobalBasicSettings(void)")]
// was: __ZN3RBX19GlobalBasicSettingsC2Ev
// IDA 0x508958: 319 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508958() {
}


// 0x508cd4 — __ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "resetChild(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZL10resetChildN5boost10shared_ptrIN3RBX8InstanceEEE
// IDA 0x508cd4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508cd4() {
}


// 0x508d14 — __ZN3RBX22GlobalAdvancedSettings13getFVariablesEv
// type: _DWORD __fastcall(RBX::GlobalAdvancedSettings *__hidden this)
#[doc(alias = "RBX::GlobalAdvancedSettings::getFVariables(void)")]
// was: __ZN3RBX22GlobalAdvancedSettings13getFVariablesEv
// IDA 0x508d14: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508d14() {
}


// 0x508df4 — __ZL5visitRKSsS0_Pv
// type: _DWORD __fastcall(const std::string *, const std::string *, void *)
#[doc(alias = "visit(std::string const&,std::string const&,void *)")]
// was: __ZL5visitRKSsS0_Pv
// IDA 0x508df4: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508df4() {
}


// 0x508e18 — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,std::string ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFSsSsELi1EED1Ev
// IDA 0x508e18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508e18() {
}


// 0x508e7c — __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalAdvancedSettings,bool ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_22GlobalAdvancedSettingsEFbSsELi1EED1Ev
// IDA 0x508e7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508e7c() {
}


// 0x508ebc — __ZN3RBX11MergeBinder11resolveRefsEv
// type: _DWORD __fastcall(RBX::MergeBinder *__hidden this)
#[doc(alias = "RBX::MergeBinder::resolveRefs(void)")]
// was: __ZN3RBX11MergeBinder11resolveRefsEv
// IDA 0x508ebc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508ebc() {
}


// 0x508ef4 — __ZN3RBX11MergeBinderD1Ev
// type: void __fastcall(RBX::MergeBinder *__hidden this)
#[doc(alias = "RBX::MergeBinder::~MergeBinder()")]
// was: __ZN3RBX11MergeBinderD1Ev
// IDA 0x508ef4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_508ef4() {
}


// 0x508f18 — __ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<RBX::Settings::InvalidDescendentDetector>(RBX::Settings::InvalidDescendentDetector const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsINS_8Settings25InvalidDescendentDetectorEEEvRKT_
// IDA 0x508f18: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_508f18() {
}


// 0x509048 — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::~shared_ptr()")]
// was: __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev
// IDA 0x509048: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509048() {
}


// 0x50905c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::~shared_ptr()")]
// was: __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev
// IDA 0x50905c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50905c() {
}


// 0x509070 — __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED1Ev
// IDA 0x509070: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509070() {
}


// 0x509094 — __ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<void (*)(rbx_core::SharedPtr<RBX::Instance>)>(void (*)(rbx_core::SharedPtr<RBX::Instance>) const&)const")]
// was: __ZNK3RBX8Instance13visitChildrenIPFvN5boost10shared_ptrIS0_EEEEEvRKT_
// IDA 0x509094: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509094() {
}


// 0x5091cc — __ZN3RBX8SettingsD1Ev
// type: void __fastcall(RBX::Settings *__hidden this)
#[doc(alias = "RBX::Settings::~Settings()")]
// was: __ZN3RBX8SettingsD1Ev
// IDA 0x5091cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5091cc() {
}


// 0x509208 — __ZN3RBX8SettingsD0Ev
// type: void __fastcall(RBX::Settings *__hidden this)
#[doc(alias = "RBX::Settings::~Settings()")]
// was: __ZN3RBX8SettingsD0Ev
// IDA 0x509208: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509208() {
}


// 0x5092dc — __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
// IDA 0x5092dc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5092dc() {
}


// 0x509308 — __ZThn32_N3RBX8SettingsD1Ev
// type: void __fastcall(RBX::Settings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings()")]
// was: __ZThn32_N3RBX8SettingsD1Ev
// IDA 0x509308: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509308() {
}


// 0x509344 — __ZThn32_N3RBX8SettingsD0Ev
// type: void __fastcall(RBX::Settings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings()")]
// was: __ZThn32_N3RBX8SettingsD0Ev
// IDA 0x509344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509344() {
}


// 0x509418 — __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEE12getClassNameEv
// IDA 0x509418: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509418() {
}


// 0x509440 — __ZThn36_N3RBX8SettingsD1Ev
// type: void __fastcall(RBX::Settings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings()")]
// was: __ZThn36_N3RBX8SettingsD1Ev
// IDA 0x509440: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509440() {
}


// 0x50947c — __ZThn36_N3RBX8SettingsD0Ev
// type: void __fastcall(RBX::Settings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings()")]
// was: __ZThn36_N3RBX8SettingsD0Ev
// IDA 0x50947c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50947c() {
}


// 0x509554 — __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
// IDA 0x509554: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509554() {
}


// 0x50957c — __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEE12getClassNameEv
// IDA 0x50957c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50957c() {
}


// 0x5095a4 — __ZN3RBX19GlobalBasicSettingsD1Ev
// type: void __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "RBX::GlobalBasicSettings::~GlobalBasicSettings()")]
// was: __ZN3RBX19GlobalBasicSettingsD1Ev
// IDA 0x5095a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5095a4() {
}


// 0x5096f0 — __ZN3RBX19GlobalBasicSettingsD0Ev
// type: void __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "RBX::GlobalBasicSettings::~GlobalBasicSettings()")]
// was: __ZN3RBX19GlobalBasicSettingsD0Ev
// IDA 0x5096f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5096f0() {
}


// 0x509850 — __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
// IDA 0x509850: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509850() {
}


// 0x509878 — __ZThn32_N3RBX19GlobalBasicSettingsD1Ev
// type: void __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()")]
// was: __ZThn32_N3RBX19GlobalBasicSettingsD1Ev
// IDA 0x509878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509878() {
}


// 0x5099d0 — __ZThn32_N3RBX19GlobalBasicSettingsD0Ev
// type: void __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()")]
// was: __ZThn32_N3RBX19GlobalBasicSettingsD0Ev
// IDA 0x5099d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5099d0() {
}


// 0x509b40 — __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEE12getClassNameEv
// IDA 0x509b40: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509b40() {
}


// 0x509b68 — __ZThn36_N3RBX19GlobalBasicSettingsD1Ev
// type: void __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()")]
// was: __ZThn36_N3RBX19GlobalBasicSettingsD1Ev
// IDA 0x509b68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509b68() {
}


// 0x509cc8 — __ZThn36_N3RBX19GlobalBasicSettingsD0Ev
// type: void __fastcall(RBX::GlobalBasicSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GlobalBasicSettings::~GlobalBasicSettings()")]
// was: __ZThn36_N3RBX19GlobalBasicSettingsD0Ev
// IDA 0x509cc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_509cc8() {
}


// 0x509e3c — __ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_20sGlobalBasicSettingsEEEEvv
// IDA 0x509e3c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_509e3c() {
}


// 0x509e40 — __ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v
// IDA 0x509e40: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509e40() {
}


// 0x509f20 — __ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_23sGlobalAdvancedSettingsEEEEvv
// IDA 0x509f20: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_509f20() {
}


// 0x509f24 — __ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v
// IDA 0x509f24: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_509f24() {
}


// 0x50a004 — __ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sSettingsEEEEvv
// IDA 0x50a004: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_50a004() {
}


// 0x50a008 — __ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v
// IDA 0x50a008: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a008() {
}


// 0x50a29c — __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_")]
// was: __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_
// IDA 0x50a29c: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a29c() {
}


// 0x50a490 — __ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x50a490: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a490() {
}


// 0x50a4cc — __ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x50a4cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a4cc() {
}


// 0x50a59c — __ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x50a59c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a59c() {
}


// 0x50a5d8 — __ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x50a5d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a5d8() {
}


// 0x50a6ac — __ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x50a6ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a6ac() {
}


// 0x50a6e8 — __ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_19GlobalBasicSettingsENS_8SettingsELZNS_20sGlobalBasicSettingsEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x50a6e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a6e8() {
}


// 0x50a7bc — __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x50a7bc: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50a7bc() {
}


// 0x50a8d8 — __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x50a8d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a8d8() {
}


// 0x50a914 — __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x50a914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a914() {
}


// 0x50a9e4 — __ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x50a9e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50a9e4() {
}


// 0x50aa20 — __ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x50aa20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50aa20() {
}


// 0x50aaf4 — __ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x50aaf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50aaf4() {
}


// 0x50ab30 — __ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19GlobalBasicSettingsELZNS_20sGlobalBasicSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x50ab30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50ab30() {
}


// 0x50ac04 — __ZN3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_8SettingsELZNS_9sSettingsEENS_17NonFactoryProductINS_15ServiceProviderELZNS_9sSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x50ac04: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50ac04() {
}


// 0x50ad20 — __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// was: __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
// IDA 0x50ad20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50ad20() {
}


// 0x50ad5c — __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// was: __ZN3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
// IDA 0x50ad5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50ad5c() {
}


// 0x50ae2c — __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// was: __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
// IDA 0x50ae2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50ae2c() {
}


// 0x50ae68 — __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// was: __ZThn32_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
// IDA 0x50ae68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50ae68() {
}


// 0x50af3c — __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev")]
// was: __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED1Ev
// IDA 0x50af3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50af3c() {
}


// 0x50af78 — __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev")]
// was: __ZThn36_N3RBX17NonFactoryProductINS_8SettingsELZNS_20sGlobalBasicSettingsEEED0Ev
// IDA 0x50af78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50af78() {
}


// 0x50b04c — __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::BoundFuncDesc(void (RBX::GlobalBasicSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x50b04c: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b04c() {
}


// 0x50b150 — __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EED0Ev
// IDA 0x50b150: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_50b150() {
}


// 0x50b204 — __ZNK3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GlobalBasicSettings,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_19GlobalBasicSettingsEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x50b204: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b204() {
}


// 0x50b228 — __ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::find<RBX::Selection>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v
// IDA 0x50b228: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b228() {
}


// 0x50b39c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection> RBX::Creatable<RBX::Instance>::create<RBX::Selection>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SelectionEEEN5boost10shared_ptrIT_EEv
// IDA 0x50b39c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b39c() {
}


// 0x50b44c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Selection>(rbx_core::SharedPtr<RBX::Selection> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9SelectionEEERS3_RKNS0_IT_EE
// IDA 0x50b44c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b44c() {
}


// 0x50b480 — __ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v
// IDA 0x50b480: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b480() {
}


// 0x50b4c8 — __ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v
// IDA 0x50b4c8: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b4c8() {
}


// 0x50b5b0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Selection>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv
// IDA 0x50b5b0: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b5b0() {
}


// 0x50b688 — __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Selection>::shared_ptr<RBX::Selection,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9SelectionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x50b688: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b688() {
}


// 0x50b838 — __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9SelectionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x50b838: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b838() {
}


// 0x50b940 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x50b940: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_50b940() {
}


// 0x50b948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x50b948: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b948() {
}


// 0x50b968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x50b968: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b968() {
}


// 0x50b980 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Selection *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SelectionENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x50b980: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b980() {
}


// 0x50b988 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sSelectionEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sSelectionEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_10sSelectionEEE15isNullClassNameEv
// IDA 0x50b988: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50b988() {
}


// 0x50ba28 — __ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_")]
// was: __ZN3RBX10Reflection9DescribedINS_22GlobalAdvancedSettingsELZNS_23sGlobalAdvancedSettingsEENS_17NonFactoryProductINS_8SettingsELZNS_23sGlobalAdvancedSettingsEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2ISsEET_
// IDA 0x50ba28: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_50ba28() {
}

