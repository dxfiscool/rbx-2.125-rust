// Auto-generated shard DG — next 100 RBX::Reflection stubs — EA-sorted ascending 0x850c..0xfce8 (remaining 7795) — starts 0x850c
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 8276->8376 covered, 7795 remaining)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;


// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev")]
pub fn stub_850c() -> ! {
    todo!("0x850c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev")]
pub fn stub_86d0() -> ! {
    todo!("0x86d0 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev")]
pub fn stub_88c4() -> ! {
    todo!("0x88c4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev")]
pub fn stub_8a88() -> ! {
    todo!("0x8a88 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev")]
pub fn stub_8c4c() -> ! {
    todo!("0x8c4c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev")]
pub fn stub_8e24() -> ! {
    todo!("0x8e24 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")
}

// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc")]
pub fn stub_9b48(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x9b48: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc")]
pub fn stub_9ea8(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x9ea8: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_")]
pub fn stub_a208(desc: &mut crate::enum_desc::EnumDesc, legacy_index: usize, name: &str, value: i32) {
    // IDA 0xa208: EnumDesc<T>::addLegacy -- grow legacy vector, map legacy name->value (decompiled 0x47cd20, model 0xa208). Delegates to the shared model.
    desc.add_legacy(legacy_index, name, value)
}

// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc")]
pub fn stub_a25c(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xa25c: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc")]
pub fn stub_a5bc(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xa5bc: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc")]
pub fn stub_a91c(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xa91c: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc")]
pub fn stub_ac7c(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xac7c: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc")]
pub fn stub_afdc(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xafdc: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev")]
pub fn stub_b340() {
    // IDA 0xb340: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev")]
pub fn stub_b368() {
    // IDA 0xb368: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev")]
pub fn stub_b390() {
    // IDA 0xb390: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev")]
pub fn stub_b3bc() {
    // IDA 0xb3bc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev")]
pub fn stub_b3f8() {
    // IDA 0xb3f8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev")]
pub fn stub_b420() {
    // IDA 0xb420: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev")]
pub fn stub_b448() {
    // IDA 0xb448: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev")]
pub fn stub_b478() {
    // IDA 0xb478: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev")]
pub fn stub_b4a8() {
    // IDA 0xb4a8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev")]
pub fn stub_b4d0() {
    // IDA 0xb4d0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_")]
pub fn stub_b76c() -> ! {
    todo!("0xb76c rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev")]
pub fn stub_b934() {
    // IDA 0xb934: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev")]
pub fn stub_b938() {
    // IDA 0xb938: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc")]
pub fn stub_b94c() -> ! {
    todo!("0xb94c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE")]
pub fn stub_b97c() -> ! {
    todo!("0xb97c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_b99c() -> ! {
    todo!("0xb99c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs")]
pub fn stub_b9f8() -> ! {
    todo!("0xb9f8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")
}

// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev")]
pub fn stub_bb3c() {
    // IDA 0xbb3c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev")]
pub fn stub_bb40() {
    // IDA 0xbb40: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc")]
pub fn stub_bb54() -> ! {
    todo!("0xbb54 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE")]
pub fn stub_bb84() -> ! {
    todo!("0xbb84 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_bba4() -> ! {
    todo!("0xbba4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs")]
pub fn stub_bc00() -> ! {
    todo!("0xbc00 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc")]
pub fn stub_bd5c() -> ! {
    todo!("0xbd5c RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE")]
pub fn stub_bd8c() -> ! {
    todo!("0xbd8c RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_bdac() -> ! {
    todo!("0xbdac RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs")]
pub fn stub_be08() -> ! {
    todo!("0xbe08 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")
}

// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev")]
pub fn stub_bf4c() {
    // IDA 0xbf4c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev")]
pub fn stub_bf50() {
    // IDA 0xbf50: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc")]
pub fn stub_bf64() -> ! {
    todo!("0xbf64 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE")]
pub fn stub_bf94() -> ! {
    todo!("0xbf94 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_bfb4() -> ! {
    todo!("0xbfb4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs")]
pub fn stub_c010() -> ! {
    todo!("0xc010 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")
}

// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev")]
pub fn stub_c154() {
    // IDA 0xc154: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev")]
pub fn stub_c158() {
    // IDA 0xc158: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc")]
pub fn stub_c16c() -> ! {
    todo!("0xc16c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE")]
pub fn stub_c19c() -> ! {
    todo!("0xc19c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_c1bc() -> ! {
    todo!("0xc1bc RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs")]
pub fn stub_c218() -> ! {
    todo!("0xc218 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")
}

// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev")]
pub fn stub_c35c() {
    // IDA 0xc35c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev")]
pub fn stub_c360() {
    // IDA 0xc360: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc")]
pub fn stub_c374() -> ! {
    todo!("0xc374 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE")]
pub fn stub_c3a4() -> ! {
    todo!("0xc3a4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_c3c4() -> ! {
    todo!("0xc3c4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs")]
pub fn stub_c420() -> ! {
    todo!("0xc420 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")
}

// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev")]
pub fn stub_c564() {
    // IDA 0xc564: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev")]
pub fn stub_c568() {
    // IDA 0xc568: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc")]
pub fn stub_c57c() -> ! {
    todo!("0xc57c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE")]
pub fn stub_c5ac() -> ! {
    todo!("0xc5ac RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_c5cc() -> ! {
    todo!("0xc5cc RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs")]
pub fn stub_c628() -> ! {
    todo!("0xc628 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_")]
pub fn stub_c76c() -> ! {
    todo!("0xc76c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_")]
pub fn stub_c9d8() -> ! {
    todo!("0xc9d8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_cc34() -> ! {
    todo!("0xcc34 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")
}

// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev")]
pub fn stub_ccb0() {
    // IDA 0xccb0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_")]
pub fn stub_cd4c() -> ! {
    todo!("0xcd4c RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_")]
pub fn stub_cfb8() -> ! {
    todo!("0xcfb8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_d174() -> ! {
    todo!("0xd174 RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")
}

// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev")]
pub fn stub_d1f0() {
    // IDA 0xd1f0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_")]
pub fn stub_d28c() -> ! {
    todo!("0xd28c RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_")]
pub fn stub_d4f8() -> ! {
    todo!("0xd4f8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_d6b4() -> ! {
    todo!("0xd6b4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")
}

// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev")]
pub fn stub_d730() {
    // IDA 0xd730: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_")]
pub fn stub_d7cc() -> ! {
    todo!("0xd7cc RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_")]
pub fn stub_da38() -> ! {
    todo!("0xda38 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_dbf4() -> ! {
    todo!("0xdbf4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")
}

// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev")]
pub fn stub_dc70() {
    // IDA 0xdc70: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_")]
pub fn stub_dd0c() -> ! {
    todo!("0xdd0c RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_")]
pub fn stub_df78() -> ! {
    todo!("0xdf78 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_e134() -> ! {
    todo!("0xe134 RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_")]
pub fn stub_e24c() -> ! {
    todo!("0xe24c RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_")]
pub fn stub_e4b8() -> ! {
    todo!("0xe4b8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_e674() -> ! {
    todo!("0xe674 RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")
}

// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev")]
pub fn stub_e6f0() {
    // IDA 0xe6f0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_")]
pub fn stub_e78c() -> ! {
    todo!("0xe78c RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_")]
pub fn stub_e9f8() -> ! {
    todo!("0xe9f8 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_ebb4() -> ! {
    todo!("0xebb4 RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")
}

// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev")]
pub fn stub_ec30() {
    // IDA 0xec30: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE")]
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)
pub fn stub_f574() -> ! {
    todo!("0xf574 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception")]
pub fn stub_f6dc() -> ! {
    todo!("0xf6dc rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")
}

// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_fb74() -> ! {
    todo!("0xfb74 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev")]
pub fn stub_fc88() {
    // IDA 0xfc88: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv")]
pub fn stub_fcb4() -> ! {
    todo!("0xfcb4 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv")]
pub fn stub_fcb8() -> ! {
    todo!("0xfcb8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")
}

// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_fcbc() -> ! {
    todo!("0xfcbc RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi")]
pub fn stub_fce8() -> ! {
    todo!("0xfce8 RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}
