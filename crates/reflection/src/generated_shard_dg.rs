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
pub fn stub_850c() -> crate::enum_desc::EnumDesc {
    // IDA 0x850c: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "AASamples", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("AASamples")
}

// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev")]
pub fn stub_86d0() -> crate::enum_desc::EnumDesc {
    // IDA 0x86d0: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "GraphicsMode", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("GraphicsMode")
}

// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev")]
pub fn stub_88c4() -> crate::enum_desc::EnumDesc {
    // IDA 0x88c4: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "FramerateManagerMode", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("FramerateManagerMode")
}

// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev")]
pub fn stub_8a88() -> crate::enum_desc::EnumDesc {
    // IDA 0x8a88: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "Antialiasing", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Antialiasing")
}

// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev")]
pub fn stub_8c4c() -> crate::enum_desc::EnumDesc {
    // IDA 0x8c4c: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "Shadow", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Shadow")
}

// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev")]
pub fn stub_8e24() -> crate::enum_desc::EnumDesc {
    // IDA 0x8e24: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "QualityLevel", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("QualityLevel")
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
pub fn stub_b94c(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xb94c: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE")]
pub fn stub_b97c(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xb97c: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_b99c() {
    // IDA 0xb99c: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs")]
pub fn stub_b9f8(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xb9f8: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
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
pub fn stub_bb54(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xbb54: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE")]
pub fn stub_bb84(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xbb84: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_bba4() {
    // IDA 0xbba4: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs")]
pub fn stub_bc00(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xbc00: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc")]
pub fn stub_bd5c(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xbd5c: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE")]
pub fn stub_bd8c(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xbd8c: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_bdac() {
    // IDA 0xbdac: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs")]
pub fn stub_be08(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xbe08: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
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
pub fn stub_bf64(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xbf64: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE")]
pub fn stub_bf94(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xbf94: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_bfb4() {
    // IDA 0xbfb4: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs")]
pub fn stub_c010(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc010: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
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
pub fn stub_c16c(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xc16c: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE")]
pub fn stub_c19c(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xc19c: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_c1bc() {
    // IDA 0xc1bc: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs")]
pub fn stub_c218(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc218: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
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
pub fn stub_c374(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xc374: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE")]
pub fn stub_c3a4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xc3a4: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_c3c4() {
    // IDA 0xc3c4: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs")]
pub fn stub_c420(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc420: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
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
pub fn stub_c57c(desc: &crate::enum_desc::EnumDesc, name: &str) -> usize {
    // IDA 0xc57c: EnumDesc<T>::lookup(char const*) -- Name::lookup intern, search name_to_value then legacy_names; hit: return convertToItem(value); miss: return 0 (decompiled 0x957a18).
    desc.lookup_value(name).and_then(|v| usize::try_from(v).ok()).and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE")]
pub fn stub_c5ac(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xc5ac: EnumDesc<T>::lookup(Variant) -- rbx::any_cast<T> the payload, then convertToItem (decompiled 0xb97c). Variant is unmodeled in this crate; the caller passes the already-cast enum value, and this is convertToItem exactly.
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_c5cc() {
    // IDA 0xc5cc: EnumDesc<T>::convertToValue(index, Variant&) -- writes the converted value into a Variant out-param; Variant is unmodeled in this crate: cutover no-op. See the (desc, name, &mut i32) sibling for the lookup semantics.
}

// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs")]
pub fn stub_c628(desc: &crate::enum_desc::EnumDesc, index: usize, out: &mut String) -> bool {
    // IDA 0xc628: EnumDesc<T>::convertToString(index, string&) -- if index < items.size(): out = items[index].name, return true; else return false, out untouched (decompiled 0x957bd4).
    if let Some(item) = desc.items.get(index) {
        *out = item.name.clone();
        true
    } else {
        false
    }
}

// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_")]
pub fn stub_c76c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xc76c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_")]
pub fn stub_c9d8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xc9d8: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_cc34(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xcc34: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
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
pub fn stub_cd4c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xcd4c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_")]
pub fn stub_cfb8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xcfb8: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_d174(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xd174: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
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
pub fn stub_d28c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xd28c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_")]
pub fn stub_d4f8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xd4f8: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_d6b4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xd6b4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
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
pub fn stub_d7cc(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xd7cc: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_")]
pub fn stub_da38(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xda38: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_dbf4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xdbf4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
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
pub fn stub_dd0c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xdd0c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_")]
pub fn stub_df78(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xdf78: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_e134(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xe134: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_")]
pub fn stub_e24c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xe24c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_")]
pub fn stub_e4b8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xe4b8: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_e674(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xe674: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
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
pub fn stub_e78c(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xe78c: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_")]
pub fn stub_e9f8(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xe9f8: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_ebb4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xebb4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
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
pub fn stub_fcb4() -> bool {
    // IDA 0xfcb4: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv")]
pub fn stub_fcb8() -> bool {
    // IDA 0xfcb8: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
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
