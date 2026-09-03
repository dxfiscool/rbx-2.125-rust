//! core shard jb — 120 core stubs EA-sorted, 0x850c..0x100ac (EA-sorted asc global gap filler next 120 not yet in core, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) EA-sorted asc not in crates/core — next 120 uncovered (53535 remaining before -> 53415 after, 0x850c..0x100ac).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::EnumDesc(void)")]
// 0x850c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x850c() {
    // IDA 0x850c: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::EnumDesc(void)")]
// 0x86d0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x86d0() {
    // IDA 0x86d0: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::EnumDesc(void)")]
// 0x88c4 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x88c4() {
    // IDA 0x88c4: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::EnumDesc(void)")]
// 0x8a88 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x8a88() {
    // IDA 0x8a88: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::EnumDesc(void)")]
// 0x8c4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x8c4c() {
    // IDA 0x8c4c: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::EnumDesc(void)")]
// 0x8e24 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0x8e24() {
    // IDA 0x8e24: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::EnumDesc(void)")]
// 0x9100 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEEC2Ev
// type: RBX::Reflection::EnumDescriptor *__fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0x9100() {
    // IDA 0x9100: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
// 0x9b48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0x9b48() {
    // IDA 0x9b48: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
// 0x9ea8 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0x9ea8() {
    // IDA 0x9ea8: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
// 0xa208 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
// type: _DWORD *__fastcall(int, unsigned int, int, int)
pub fn stub_0xa208() {
    // IDA 0xa208: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
// 0xa25c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0xa25c() {
    // IDA 0xa25c: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
// 0xa5bc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0xa5bc() {
    // IDA 0xa5bc: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
// 0xa91c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0xa91c() {
    // IDA 0xa91c: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
// 0xac7c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0xac7c() {
    // IDA 0xac7c: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
// 0xafdc — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_0xafdc() {
    // IDA 0xafdc: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::~EnumPropDescriptor()")]
// 0xb340 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb340() {
    // IDA 0xb340: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::~EnumPropDescriptor()")]
// 0xb368 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb368() {
    // IDA 0xb368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::~EnumPropDescriptor()")]
// 0xb390 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb390() {
    // IDA 0xb390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::~PropDescriptor()")]
// 0xb3bc — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb3bc() {
    // IDA 0xb3bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::~EnumPropDescriptor()")]
// 0xb3f8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb3f8() {
    // IDA 0xb3f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::~EnumPropDescriptor()")]
// 0xb420 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb420() {
    // IDA 0xb420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::~EnumPropDescriptor()")]
// 0xb448 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb448() {
    // IDA 0xb448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
// 0xb478 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb478() {
    // IDA 0xb478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
// 0xb4a8 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb4a8() {
    // IDA 0xb4a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
// 0xb4d0 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xb4d0() {
    // IDA 0xb4d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xb934 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED1Ev
// type: int()
pub fn stub_0xb934() {
    // IDA 0xb934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xb938 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED0Ev
// type: int __fastcall(int)
pub fn stub_0xb938() {
    // IDA 0xb938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(char const*)const")]
// 0xb94c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xb94c() {
    // IDA 0xb94c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::lookup(RBX::Reflection::Variant const&)const")]
// 0xb97c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xb97c() {
    // IDA 0xb97c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xb99c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xb99c() {
    // IDA 0xb99c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(unsigned long,std::string &)const")]
// 0xb9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xb9f8() {
    // IDA 0xb9f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xbb3c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED1Ev
// type: int()
pub fn stub_0xbb3c() {
    // IDA 0xbb3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xbb40 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED0Ev
// type: int __fastcall(int)
pub fn stub_0xbb40() {
    // IDA 0xbb40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(char const*)const")]
// 0xbb54 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xbb54() {
    // IDA 0xbb54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbb84 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xbb84() {
    // IDA 0xbb84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbba4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xbba4() {
    // IDA 0xbba4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(unsigned long,std::string &)const")]
// 0xbc00 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xbc00() {
    // IDA 0xbc00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xbd44 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED1Ev
// type: int()
pub fn stub_0xbd44() {
    // IDA 0xbd44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xbd48 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED0Ev
// type: int __fastcall(int)
pub fn stub_0xbd48() {
    // IDA 0xbd48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(char const*)const")]
// 0xbd5c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xbd5c() {
    // IDA 0xbd5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbd8c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xbd8c() {
    // IDA 0xbd8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbdac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xbdac() {
    // IDA 0xbdac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(unsigned long,std::string &)const")]
// 0xbe08 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xbe08() {
    // IDA 0xbe08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xbf4c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED1Ev
// type: int()
pub fn stub_0xbf4c() {
    // IDA 0xbf4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xbf50 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED0Ev
// type: int __fastcall(int)
pub fn stub_0xbf50() {
    // IDA 0xbf50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(char const*)const")]
// 0xbf64 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xbf64() {
    // IDA 0xbf64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xbf94 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xbf94() {
    // IDA 0xbf94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xbfb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xbfb4() {
    // IDA 0xbfb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(unsigned long,std::string &)const")]
// 0xc010 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xc010() {
    // IDA 0xc010: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
// 0xc154 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED1Ev
// type: int()
pub fn stub_0xc154() {
    // IDA 0xc154: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
// 0xc158 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED0Ev
// type: int __fastcall(int)
pub fn stub_0xc158() {
    // IDA 0xc158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(char const*)const")]
// 0xc16c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xc16c() {
    // IDA 0xc16c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0xc19c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xc19c() {
    // IDA 0xc19c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xc1bc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xc1bc() {
    // IDA 0xc1bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(unsigned long,std::string &)const")]
// 0xc218 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xc218() {
    // IDA 0xc218: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
// 0xc35c — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED1Ev
// type: int()
pub fn stub_0xc35c() {
    // IDA 0xc35c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
// 0xc360 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED0Ev
// type: int __fastcall(int)
pub fn stub_0xc360() {
    // IDA 0xc360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(char const*)const")]
// 0xc374 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xc374() {
    // IDA 0xc374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::lookup(RBX::Reflection::Variant const&)const")]
// 0xc3a4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xc3a4() {
    // IDA 0xc3a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xc3c4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xc3c4() {
    // IDA 0xc3c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(unsigned long,std::string &)const")]
// 0xc420 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xc420() {
    // IDA 0xc420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
// 0xc564 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED1Ev
// type: int()
pub fn stub_0xc564() {
    // IDA 0xc564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
// 0xc568 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(int)
pub fn stub_0xc568() {
    // IDA 0xc568: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(char const*)const")]
// 0xc57c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0xc57c() {
    // IDA 0xc57c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::lookup(RBX::Reflection::Variant const&)const")]
// 0xc5ac — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0xc5ac() {
    // IDA 0xc5ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0xc5cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueEmRNS0_7VariantE
pub fn stub_0xc5cc() {
    // IDA 0xc5cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(unsigned long,std::string &)const")]
// 0xc628 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0xc628() {
    // IDA 0xc628: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
// 0xc76c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xc76c() {
    // IDA 0xc76c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
// 0xc9d8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xc9d8() {
    // IDA 0xc9d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
// 0xcc34 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xcc34() {
    // IDA 0xcc34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
// 0xccb0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xccb0() {
    // IDA 0xccb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
// 0xcd4c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xcd4c() {
    // IDA 0xcd4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
// 0xcfb8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xcfb8() {
    // IDA 0xcfb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
// 0xd174 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xd174() {
    // IDA 0xd174: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
// 0xd1f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xd1f0() {
    // IDA 0xd1f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
// 0xd28c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xd28c() {
    // IDA 0xd28c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
// 0xd4f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xd4f8() {
    // IDA 0xd4f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
// 0xd6b4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xd6b4() {
    // IDA 0xd6b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
// 0xd730 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xd730() {
    // IDA 0xd730: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
// 0xd7cc — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xd7cc() {
    // IDA 0xd7cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
// 0xda38 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xda38() {
    // IDA 0xda38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
// 0xdbf4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xdbf4() {
    // IDA 0xdbf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
// 0xdc70 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xdc70() {
    // IDA 0xdc70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
// 0xdd0c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xdd0c() {
    // IDA 0xdd0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
// 0xdf78 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xdf78() {
    // IDA 0xdf78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
// 0xe134 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xe134() {
    // IDA 0xe134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
// 0xe1b0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xe1b0() {
    // IDA 0xe1b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
// 0xe24c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xe24c() {
    // IDA 0xe24c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
// 0xe4b8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xe4b8() {
    // IDA 0xe4b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
// 0xe674 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xe674() {
    // IDA 0xe674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
// 0xe6f0 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xe6f0() {
    // IDA 0xe6f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
// 0xe78c — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xe78c() {
    // IDA 0xe78c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
// 0xe9f8 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0xe9f8() {
    // IDA 0xe9f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
// 0xebb4 — __ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0xebb4() {
    // IDA 0xebb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
// 0xec30 — __ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0xec30() {
    // IDA 0xec30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev")]
// 0xf2bc — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_0xf2bc() {
    // IDA 0xf2bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv")]
// 0xf500 — __ZN3RBX14FactoryProductI19CRenderSettingsItemNS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_0xf500() {
    // IDA 0xf500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// 0xfa00 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_0xfa00() {
    // IDA 0xfa00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0xfb1c — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0xfb1c() {
    // IDA 0xfb1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0xfb20 — __ZN3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(RBX::Instance *)
pub fn stub_0xfb20() {
    // IDA 0xfb20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0xfb34 — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0xfb34() {
    // IDA 0xfb34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0xfb3c — __ZThn32_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
pub fn stub_0xfb3c() {
    // IDA 0xfb3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0xfb54 — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0xfb54() {
    // IDA 0xfb54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0xfb5c — __ZThn36_N3RBX10Reflection9DescribedI19CRenderSettingsItemLZ15sRenderSettingsENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZ15sRenderSettingsENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: int __fastcall(int)
pub fn stub_0xfb5c() {
    // IDA 0xfb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0xfb74 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xfb74() {
    // IDA 0xfb74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::~PropDescriptor()")]
// 0xfc88 — __ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_0xfc88() {
    // IDA 0xfc88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isReadOnly(void)const")]
// 0xfcb4 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE10isReadOnlyEv
// type: int()
pub fn stub_0xfcb4() {
    // IDA 0xfcb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::isWriteOnly(void)const")]
// 0xfcb8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE11isWriteOnlyEv
// type: int()
pub fn stub_0xfcb8() {
    // IDA 0xfcb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0xfcbc — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_0xfcbc() {
    // IDA 0xfcbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::GetSetImpl<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// 0xfce8 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiE10GetSetImplIMNS_15CRenderSettingsEKFjvEMS2_FvjEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0xfce8() {
    // IDA 0xfce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0xfd0c — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, int)
pub fn stub_0xfd0c() {
    // IDA 0xfd0c: render-settings accessor owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::~BoundFuncDesc()")]
// 0xfe04 — __ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_0xfe04() {
    // IDA 0xfe04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0xfe30 — __ZNK3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_0xfe30() {
    // IDA 0xfe30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
// 0xfe54 — __ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
// type: int __fastcall(int, int (__fastcall *)(_DWORD), int, _DWORD *)
pub fn stub_0xfe54() {
    // IDA 0xfe54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0xfe84 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xfe84() {
    // IDA 0xfe84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::~EnumPropDescriptor()")]
// 0x10038 — __ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_0x10038() {
    // IDA 0x10038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isReadOnly(void)const")]
// 0x10064 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10isReadOnlyEv
// type: int __fastcall(int)
pub fn stub_0x10064() {
    // IDA 0x10064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::isWriteOnly(void)const")]
// 0x10074 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11isWriteOnlyEv
// type: int __fastcall(int)
pub fn stub_0x10074() {
    // IDA 0x10074: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x10084 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
pub fn stub_0x10084() {
    // IDA 0x10084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x100ac — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0x100ac() {
    // IDA 0x100ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
