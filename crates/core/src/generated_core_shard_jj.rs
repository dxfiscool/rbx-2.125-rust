//! core shard jj — 150 stubs EA-sorted, 0x14220..0x35e88 (EA-sorted asc global gap filler next 150 uncovered, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) EA-sorted asc not in crates/core/src via grep -r stub_0x crates/core/src --include=*.rs — next 150 uncovered (74131 remaining before -> 73981 after, 0x14220..0x35e88).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x14220 — __ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
pub fn stub_0x14220() -> ! {
    todo!("0x14220 RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")]
// 0x14260 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
pub fn stub_0x14260() -> ! {
    todo!("0x14260 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isReadOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")]
// 0x14264 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
pub fn stub_0x14264() -> ! {
    todo!("0x14264 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::isWriteOnly(void)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x14268 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_0x14268() -> ! {
    todo!("0x14268 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")]
// 0x14294 — __ZNK3RBX10Reflection14PropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE10GetSetImplIMS3_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_0x14294() -> ! {
    todo!("0x14294 RBX::Reflection::PropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::GetSetImpl<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::CRenderSettings::GraphicsMode const&)const")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")]
// 0x16548 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE13initSingletonEv
pub fn stub_0x16548() -> ! {
    todo!("0x16548 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
// 0x1654c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x1654c() -> ! {
    todo!("0x1654c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")]
// 0x1663c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE13initSingletonEv
pub fn stub_0x1663c() -> ! {
    todo!("0x1663c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
// 0x16640 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x16640() -> ! {
    todo!("0x16640 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")]
// 0x16730 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE13initSingletonEv
pub fn stub_0x16730() -> ! {
    todo!("0x16730 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
// 0x16734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x16734() -> ! {
    todo!("0x16734 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")]
// 0x16824 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE13initSingletonEv
pub fn stub_0x16824() -> ! {
    todo!("0x16824 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
// 0x16828 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x16828() -> ! {
    todo!("0x16828 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")]
// 0x16918 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE13initSingletonEv
pub fn stub_0x16918() -> ! {
    todo!("0x16918 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
// 0x1691c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x1691c() -> ! {
    todo!("0x1691c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")]
// 0x16a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE13initSingletonEv
pub fn stub_0x16a0c() -> ! {
    todo!("0x16a0c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
// 0x16a10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x16a10() -> ! {
    todo!("0x16a10 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")]
// 0x16b00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE13initSingletonEv
pub fn stub_0x16b00() -> ! {
    todo!("0x16b00 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::initSingleton(void)")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
// 0x16b04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x16b04() -> ! {
    todo!("0x16b04 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")
}

#[doc(alias = "global constructor keyed to_a")]
// 0x16e4c — __GLOBAL__I_a
pub fn stub_0x16e4c() -> ! {
    todo!("0x16e4c global constructor keyed to_a")
}

#[doc(alias = "RBX::DataModel::serverSave(void)")]
// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
// type: void __fastcall(RBX::DataModel *this)
pub fn stub_0x179e8() -> ! {
    todo!("0x179e8 RBX::DataModel::serverSave(void)")
}

#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
// type: void()
pub fn stub_0x179f0() -> ! {
    todo!("0x179f0 RBX::DataModel::internalSave(RBX::ContentId)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
pub fn stub_0x17aac() -> ! {
    todo!("0x17aac rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
pub fn stub_0x17b80() -> ! {
    todo!("0x17b80 rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")
}

#[doc(alias = "global constructor keyed to_a_0")]
// 0x17c58 — __GLOBAL__I_a_0
pub fn stub_0x17c58() -> ! {
    todo!("0x17c58 global constructor keyed to_a_0")
}

#[doc(alias = "+[Appirater sharedInstance]")]
// 0x17f80 — +[Appirater sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x17f80() -> ! {
    todo!("0x17f80 +[Appirater sharedInstance]")
}

#[doc(alias = "___27+[Appirater sharedInstance]_block_invoke")]
// 0x17fe4 — ___27+[Appirater sharedInstance]_block_invoke
pub fn stub_0x17fe4() -> ! {
    todo!("0x17fe4 ___27+[Appirater sharedInstance]_block_invoke")
}

#[doc(alias = "___copy_helper_block_")]
// 0x18094 — ___copy_helper_block_
pub fn stub_0x18094() -> ! {
    todo!("0x18094 ___copy_helper_block_")
}

#[doc(alias = "___destroy_helper_block_")]
// 0x180a0 — ___destroy_helper_block_
// type: void __fastcall(int)
pub fn stub_0x180a0() -> ! {
    todo!("0x180a0 ___destroy_helper_block_")
}

#[doc(alias = "___copy_helper_block_125")]
// 0x18bc8 — ___copy_helper_block_125
pub fn stub_0x18bc8() -> ! {
    todo!("0x18bc8 ___copy_helper_block_125")
}

#[doc(alias = "___destroy_helper_block_126")]
// 0x18bd4 — ___destroy_helper_block_126
pub fn stub_0x18bd4() -> ! {
    todo!("0x18bd4 ___destroy_helper_block_126")
}

#[doc(alias = "___copy_helper_block_130")]
// 0x18c8c — ___copy_helper_block_130
pub fn stub_0x18c8c() -> ! {
    todo!("0x18c8c ___copy_helper_block_130")
}

#[doc(alias = "___destroy_helper_block_131")]
// 0x18c98 — ___destroy_helper_block_131
pub fn stub_0x18c98() -> ! {
    todo!("0x18c98 ___destroy_helper_block_131")
}

#[doc(alias = "topMostController(void)")]
// 0x1a124 — __Z17topMostControllerv
// type: _DWORD __fastcall()
pub fn stub_0x1a124() -> ! {
    todo!("0x1a124 topMostController(void)")
}

#[doc(alias = "global constructor keyed to_a_1")]
// 0x1a5d0 — __GLOBAL__I_a_1
pub fn stub_0x1a5d0() -> ! {
    todo!("0x1a5d0 global constructor keyed to_a_1")
}

#[doc(alias = "_main")]
// 0x1a768 — _main
// type: int __fastcall(int argc, const char **argv, const char **envp)
pub fn stub_0x1a768() -> ! {
    todo!("0x1a768 _main")
}

#[doc(alias = "global constructor keyed to_a_2")]
// 0x1a7d4 — __GLOBAL__I_a_2
pub fn stub_0x1a7d4() -> ! {
    todo!("0x1a7d4 global constructor keyed to_a_2")
}

#[doc(alias = "___copy_helper_block__0")]
// 0x1ae78 — ___copy_helper_block__0
// type: void __fastcall(int, const void **)
pub fn stub_0x1ae78() -> ! {
    todo!("0x1ae78 ___copy_helper_block__0")
}

#[doc(alias = "___destroy_helper_block__0")]
// 0x1aea8 — ___destroy_helper_block__0
pub fn stub_0x1aea8() -> ! {
    todo!("0x1aea8 ___destroy_helper_block__0")
}

#[doc(alias = "___copy_helper_block_66")]
// 0x1b11c — ___copy_helper_block_66
pub fn stub_0x1b11c() -> ! {
    todo!("0x1b11c ___copy_helper_block_66")
}

#[doc(alias = "___destroy_helper_block_67")]
// 0x1b14c — ___destroy_helper_block_67
pub fn stub_0x1b14c() -> ! {
    todo!("0x1b14c ___destroy_helper_block_67")
}

#[doc(alias = "global constructor keyed to_a_3")]
// 0x1b308 — __GLOBAL__I_a_3
pub fn stub_0x1b308() -> ! {
    todo!("0x1b308 global constructor keyed to_a_3")
}

#[doc(alias = "___copy_helper_block__1")]
// 0x1bb88 — ___copy_helper_block__1
pub fn stub_0x1bb88() -> ! {
    todo!("0x1bb88 ___copy_helper_block__1")
}

#[doc(alias = "___destroy_helper_block__1")]
// 0x1bb94 — ___destroy_helper_block__1
pub fn stub_0x1bb94() -> ! {
    todo!("0x1bb94 ___destroy_helper_block__1")
}

#[doc(alias = "___copy_helper_block_80")]
// 0x1bb9c — ___copy_helper_block_80
pub fn stub_0x1bb9c() -> ! {
    todo!("0x1bb9c ___copy_helper_block_80")
}

#[doc(alias = "___destroy_helper_block_81")]
// 0x1bba8 — ___destroy_helper_block_81
pub fn stub_0x1bba8() -> ! {
    todo!("0x1bba8 ___destroy_helper_block_81")
}

#[doc(alias = "___copy_helper_block_224")]
// 0x1c5f4 — ___copy_helper_block_224
pub fn stub_0x1c5f4() -> ! {
    todo!("0x1c5f4 ___copy_helper_block_224")
}

#[doc(alias = "___destroy_helper_block_225")]
// 0x1c600 — ___destroy_helper_block_225
pub fn stub_0x1c600() -> ! {
    todo!("0x1c600 ___destroy_helper_block_225")
}

#[doc(alias = "___copy_helper_block_246")]
// 0x1c734 — ___copy_helper_block_246
pub fn stub_0x1c734() -> ! {
    todo!("0x1c734 ___copy_helper_block_246")
}

#[doc(alias = "___destroy_helper_block_247")]
// 0x1c740 — ___destroy_helper_block_247
pub fn stub_0x1c740() -> ! {
    todo!("0x1c740 ___destroy_helper_block_247")
}

#[doc(alias = "___copy_helper_block_261")]
// 0x1c874 — ___copy_helper_block_261
pub fn stub_0x1c874() -> ! {
    todo!("0x1c874 ___copy_helper_block_261")
}

#[doc(alias = "___destroy_helper_block_262")]
// 0x1c880 — ___destroy_helper_block_262
pub fn stub_0x1c880() -> ! {
    todo!("0x1c880 ___destroy_helper_block_262")
}

#[doc(alias = "global constructor keyed to_a_4")]
// 0x1d870 — __GLOBAL__I_a_4
pub fn stub_0x1d870() -> ! {
    todo!("0x1d870 global constructor keyed to_a_4")
}

#[doc(alias = "+[LoginViewController sharedInstance]")]
// 0x1da5c — +[LoginViewController sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x1da5c() -> ! {
    todo!("0x1da5c +[LoginViewController sharedInstance]")
}

#[doc(alias = "___copy_helper_block__2")]
// 0x1e2d8 — ___copy_helper_block__2
pub fn stub_0x1e2d8() -> ! {
    todo!("0x1e2d8 ___copy_helper_block__2")
}

#[doc(alias = "___destroy_helper_block__2")]
// 0x1e2e4 — ___destroy_helper_block__2
pub fn stub_0x1e2e4() -> ! {
    todo!("0x1e2e4 ___destroy_helper_block__2")
}

#[doc(alias = "___copy_helper_block_226")]
// 0x1eb08 — ___copy_helper_block_226
// type: void __fastcall(int, const void **)
pub fn stub_0x1eb08() -> ! {
    todo!("0x1eb08 ___copy_helper_block_226")
}

#[doc(alias = "___destroy_helper_block_227")]
// 0x1eb38 — ___destroy_helper_block_227
pub fn stub_0x1eb38() -> ! {
    todo!("0x1eb38 ___destroy_helper_block_227")
}

#[doc(alias = "___copy_helper_block_234")]
// 0x1ec44 — ___copy_helper_block_234
pub fn stub_0x1ec44() -> ! {
    todo!("0x1ec44 ___copy_helper_block_234")
}

#[doc(alias = "___destroy_helper_block_235")]
// 0x1ec68 — ___destroy_helper_block_235
pub fn stub_0x1ec68() -> ! {
    todo!("0x1ec68 ___destroy_helper_block_235")
}

#[doc(alias = "___copy_helper_block_242")]
// 0x1ed30 — ___copy_helper_block_242
pub fn stub_0x1ed30() -> ! {
    todo!("0x1ed30 ___copy_helper_block_242")
}

#[doc(alias = "___destroy_helper_block_243")]
// 0x1ed3c — ___destroy_helper_block_243
pub fn stub_0x1ed3c() -> ! {
    todo!("0x1ed3c ___destroy_helper_block_243")
}

#[doc(alias = "___copy_helper_block_252")]
// 0x1ee84 — ___copy_helper_block_252
pub fn stub_0x1ee84() -> ! {
    todo!("0x1ee84 ___copy_helper_block_252")
}

#[doc(alias = "___destroy_helper_block_253")]
// 0x1ee90 — ___destroy_helper_block_253
pub fn stub_0x1ee90() -> ! {
    todo!("0x1ee90 ___destroy_helper_block_253")
}

#[doc(alias = "___copy_helper_block_257")]
// 0x1ee98 — ___copy_helper_block_257
pub fn stub_0x1ee98() -> ! {
    todo!("0x1ee98 ___copy_helper_block_257")
}

#[doc(alias = "___destroy_helper_block_258")]
// 0x1eea4 — ___destroy_helper_block_258
pub fn stub_0x1eea4() -> ! {
    todo!("0x1eea4 ___destroy_helper_block_258")
}

#[doc(alias = "___copy_helper_block_260")]
// 0x1efdc — ___copy_helper_block_260
// type: void __fastcall(int, int)
pub fn stub_0x1efdc() -> ! {
    todo!("0x1efdc ___copy_helper_block_260")
}

#[doc(alias = "___destroy_helper_block_261")]
// 0x1efe8 — ___destroy_helper_block_261
pub fn stub_0x1efe8() -> ! {
    todo!("0x1efe8 ___destroy_helper_block_261")
}

#[doc(alias = "___copy_helper_block_263")]
// 0x1eff0 — ___copy_helper_block_263
pub fn stub_0x1eff0() -> ! {
    todo!("0x1eff0 ___copy_helper_block_263")
}

#[doc(alias = "___destroy_helper_block_264")]
// 0x1effc — ___destroy_helper_block_264
pub fn stub_0x1effc() -> ! {
    todo!("0x1effc ___destroy_helper_block_264")
}

#[doc(alias = "___copy_helper_block_300")]
// 0x1f480 — ___copy_helper_block_300
pub fn stub_0x1f480() -> ! {
    todo!("0x1f480 ___copy_helper_block_300")
}

#[doc(alias = "___destroy_helper_block_301")]
// 0x1f48c — ___destroy_helper_block_301
pub fn stub_0x1f48c() -> ! {
    todo!("0x1f48c ___destroy_helper_block_301")
}

#[doc(alias = "___copy_helper_block_305")]
// 0x1f494 — ___copy_helper_block_305
pub fn stub_0x1f494() -> ! {
    todo!("0x1f494 ___copy_helper_block_305")
}

#[doc(alias = "___destroy_helper_block_306")]
// 0x1f4a0 — ___destroy_helper_block_306
pub fn stub_0x1f4a0() -> ! {
    todo!("0x1f4a0 ___destroy_helper_block_306")
}

#[doc(alias = "___copy_helper_block_308")]
// 0x1f660 — ___copy_helper_block_308
// type: void __fastcall(int, int)
pub fn stub_0x1f660() -> ! {
    todo!("0x1f660 ___copy_helper_block_308")
}

#[doc(alias = "___destroy_helper_block_309")]
// 0x1f66c — ___destroy_helper_block_309
pub fn stub_0x1f66c() -> ! {
    todo!("0x1f66c ___destroy_helper_block_309")
}

#[doc(alias = "___copy_helper_block_314")]
// 0x1f688 — ___copy_helper_block_314
pub fn stub_0x1f688() -> ! {
    todo!("0x1f688 ___copy_helper_block_314")
}

#[doc(alias = "___destroy_helper_block_315")]
// 0x1f694 — ___destroy_helper_block_315
pub fn stub_0x1f694() -> ! {
    todo!("0x1f694 ___destroy_helper_block_315")
}

#[doc(alias = "___copy_helper_block_320")]
// 0x1f69c — ___copy_helper_block_320
pub fn stub_0x1f69c() -> ! {
    todo!("0x1f69c ___copy_helper_block_320")
}

#[doc(alias = "___destroy_helper_block_321")]
// 0x1f6a8 — ___destroy_helper_block_321
pub fn stub_0x1f6a8() -> ! {
    todo!("0x1f6a8 ___destroy_helper_block_321")
}

#[doc(alias = "___copy_helper_block_323")]
// 0x1f82c — ___copy_helper_block_323
pub fn stub_0x1f82c() -> ! {
    todo!("0x1f82c ___copy_helper_block_323")
}

#[doc(alias = "___destroy_helper_block_324")]
// 0x1f838 — ___destroy_helper_block_324
pub fn stub_0x1f838() -> ! {
    todo!("0x1f838 ___destroy_helper_block_324")
}

#[doc(alias = "___copy_helper_block_339")]
// 0x1fa44 — ___copy_helper_block_339
pub fn stub_0x1fa44() -> ! {
    todo!("0x1fa44 ___copy_helper_block_339")
}

#[doc(alias = "___destroy_helper_block_340")]
// 0x1fa50 — ___destroy_helper_block_340
pub fn stub_0x1fa50() -> ! {
    todo!("0x1fa50 ___destroy_helper_block_340")
}

#[doc(alias = "___copy_helper_block_356")]
// 0x1fc90 — ___copy_helper_block_356
// type: void __fastcall(int, int)
pub fn stub_0x1fc90() -> ! {
    todo!("0x1fc90 ___copy_helper_block_356")
}

#[doc(alias = "___destroy_helper_block_357")]
// 0x1fc9c — ___destroy_helper_block_357
pub fn stub_0x1fc9c() -> ! {
    todo!("0x1fc9c ___destroy_helper_block_357")
}

#[doc(alias = "___copy_helper_block_359")]
// 0x1fca4 — ___copy_helper_block_359
pub fn stub_0x1fca4() -> ! {
    todo!("0x1fca4 ___copy_helper_block_359")
}

#[doc(alias = "___destroy_helper_block_360")]
// 0x1fcc8 — ___destroy_helper_block_360
pub fn stub_0x1fcc8() -> ! {
    todo!("0x1fcc8 ___destroy_helper_block_360")
}

#[doc(alias = "___copy_helper_block_364")]
// 0x1fce4 — ___copy_helper_block_364
pub fn stub_0x1fce4() -> ! {
    todo!("0x1fce4 ___copy_helper_block_364")
}

#[doc(alias = "___destroy_helper_block_365")]
// 0x1fd08 — ___destroy_helper_block_365
pub fn stub_0x1fd08() -> ! {
    todo!("0x1fd08 ___destroy_helper_block_365")
}

#[doc(alias = "___copy_helper_block_367")]
// 0x1fd24 — ___copy_helper_block_367
pub fn stub_0x1fd24() -> ! {
    todo!("0x1fd24 ___copy_helper_block_367")
}

#[doc(alias = "___destroy_helper_block_368")]
// 0x1fd30 — ___destroy_helper_block_368
pub fn stub_0x1fd30() -> ! {
    todo!("0x1fd30 ___destroy_helper_block_368")
}

#[doc(alias = "global constructor keyed to_a_5")]
// 0x202d0 — __GLOBAL__I_a_5
pub fn stub_0x202d0() -> ! {
    todo!("0x202d0 global constructor keyed to_a_5")
}

#[doc(alias = "___copy_helper_block__3")]
// 0x20f08 — ___copy_helper_block__3
pub fn stub_0x20f08() -> ! {
    todo!("0x20f08 ___copy_helper_block__3")
}

#[doc(alias = "___destroy_helper_block__3")]
// 0x20f14 — ___destroy_helper_block__3
pub fn stub_0x20f14() -> ! {
    todo!("0x20f14 ___destroy_helper_block__3")
}

#[doc(alias = "___copy_helper_block_132")]
// 0x21adc — ___copy_helper_block_132
pub fn stub_0x21adc() -> ! {
    todo!("0x21adc ___copy_helper_block_132")
}

#[doc(alias = "___destroy_helper_block_133")]
// 0x21ae8 — ___destroy_helper_block_133
pub fn stub_0x21ae8() -> ! {
    todo!("0x21ae8 ___destroy_helper_block_133")
}

#[doc(alias = "___copy_helper_block_142")]
// 0x21b10 — ___copy_helper_block_142
pub fn stub_0x21b10() -> ! {
    todo!("0x21b10 ___copy_helper_block_142")
}

#[doc(alias = "___destroy_helper_block_143")]
// 0x21b1c — ___destroy_helper_block_143
pub fn stub_0x21b1c() -> ! {
    todo!("0x21b1c ___destroy_helper_block_143")
}

#[doc(alias = "global constructor keyed to_a_6")]
// 0x21c18 — __GLOBAL__I_a_6
pub fn stub_0x21c18() -> ! {
    todo!("0x21c18 global constructor keyed to_a_6")
}

#[doc(alias = "global constructor keyed to_a_7")]
// 0x24540 — __GLOBAL__I_a_7
pub fn stub_0x24540() -> ! {
    todo!("0x24540 global constructor keyed to_a_7")
}

#[doc(alias = "+[PlaceLauncher sharedInstance]")]
// 0x24974 — +[PlaceLauncher sharedInstance]
// type: id __cdecl(id, SEL)
pub fn stub_0x24974() -> ! {
    todo!("0x24974 +[PlaceLauncher sharedInstance]")
}

#[doc(alias = "___31+[PlaceLauncher sharedInstance]_block_invoke")]
// 0x249d0 — ___31+[PlaceLauncher sharedInstance]_block_invoke
// type: id __fastcall(int)
pub fn stub_0x249d0() -> ! {
    todo!("0x249d0 ___31+[PlaceLauncher sharedInstance]_block_invoke")
}

#[doc(alias = "___copy_helper_block__4")]
// 0x24a04 — ___copy_helper_block__4
pub fn stub_0x24a04() -> ! {
    todo!("0x24a04 ___copy_helper_block__4")
}

#[doc(alias = "___destroy_helper_block__4")]
// 0x24a10 — ___destroy_helper_block__4
pub fn stub_0x24a10() -> ! {
    todo!("0x24a10 ___destroy_helper_block__4")
}

#[doc(alias = "___copy_helper_block_98")]
// 0x253cc — ___copy_helper_block_98
pub fn stub_0x253cc() -> ! {
    todo!("0x253cc ___copy_helper_block_98")
}

#[doc(alias = "___destroy_helper_block_99")]
// 0x253d8 — ___destroy_helper_block_99
pub fn stub_0x253d8() -> ! {
    todo!("0x253d8 ___destroy_helper_block_99")
}

#[doc(alias = "___copy_helper_block_191")]
// 0x298a0 — ___copy_helper_block_191
pub fn stub_0x298a0() -> ! {
    todo!("0x298a0 ___copy_helper_block_191")
}

#[doc(alias = "___destroy_helper_block_192")]
// 0x298c4 — ___destroy_helper_block_192
pub fn stub_0x298c4() -> ! {
    todo!("0x298c4 ___destroy_helper_block_192")
}

#[doc(alias = "___copy_helper_block_217")]
// 0x29c34 — ___copy_helper_block_217
pub fn stub_0x29c34() -> ! {
    todo!("0x29c34 ___copy_helper_block_217")
}

#[doc(alias = "___destroy_helper_block_218")]
// 0x29c58 — ___destroy_helper_block_218
pub fn stub_0x29c58() -> ! {
    todo!("0x29c58 ___destroy_helper_block_218")
}

#[doc(alias = "___copy_helper_block_232")]
// 0x29c88 — ___copy_helper_block_232
pub fn stub_0x29c88() -> ! {
    todo!("0x29c88 ___copy_helper_block_232")
}

#[doc(alias = "___destroy_helper_block_233")]
// 0x29c94 — ___destroy_helper_block_233
pub fn stub_0x29c94() -> ! {
    todo!("0x29c94 ___destroy_helper_block_233")
}

#[doc(alias = "___copy_helper_block_243")]
// 0x2a988 — ___copy_helper_block_243
pub fn stub_0x2a988() -> ! {
    todo!("0x2a988 ___copy_helper_block_243")
}

#[doc(alias = "___destroy_helper_block_244")]
// 0x2a994 — ___destroy_helper_block_244
pub fn stub_0x2a994() -> ! {
    todo!("0x2a994 ___destroy_helper_block_244")
}

#[doc(alias = "___copy_helper_block_247")]
// 0x2acec — ___copy_helper_block_247
// type: void __fastcall(_DWORD *, const shared_count *)
pub fn stub_0x2acec() -> ! {
    todo!("0x2acec ___copy_helper_block_247")
}

#[doc(alias = "___destroy_helper_block_248")]
// 0x2ada4 — ___destroy_helper_block_248
pub fn stub_0x2ada4() -> ! {
    todo!("0x2ada4 ___destroy_helper_block_248")
}

#[doc(alias = "___copy_helper_block_425")]
// 0x2ba00 — ___copy_helper_block_425
pub fn stub_0x2ba00() -> ! {
    todo!("0x2ba00 ___copy_helper_block_425")
}

#[doc(alias = "___destroy_helper_block_426")]
// 0x2ba0c — ___destroy_helper_block_426
pub fn stub_0x2ba0c() -> ! {
    todo!("0x2ba0c ___destroy_helper_block_426")
}

#[doc(alias = "___copy_helper_block_429")]
// 0x2ba40 — ___copy_helper_block_429
pub fn stub_0x2ba40() -> ! {
    todo!("0x2ba40 ___copy_helper_block_429")
}

#[doc(alias = "___destroy_helper_block_430")]
// 0x2ba4c — ___destroy_helper_block_430
pub fn stub_0x2ba4c() -> ! {
    todo!("0x2ba4c ___destroy_helper_block_430")
}

#[doc(alias = "____ZL15presentGameViewv_block_invoke")]
// 0x2c138 — ____ZL15presentGameViewv_block_invoke
// type: void __cdecl(id)
pub fn stub_0x2c138() -> ! {
    todo!("0x2c138 ____ZL15presentGameViewv_block_invoke")
}

#[doc(alias = "____ZL15presentGameViewv_block_invoke_2")]
// 0x2c1f8 — ____ZL15presentGameViewv_block_invoke_2
// type: id __fastcall(int)
pub fn stub_0x2c1f8() -> ! {
    todo!("0x2c1f8 ____ZL15presentGameViewv_block_invoke_2")
}

#[doc(alias = "___copy_helper_block_499")]
// 0x2c210 — ___copy_helper_block_499
pub fn stub_0x2c210() -> ! {
    todo!("0x2c210 ___copy_helper_block_499")
}

#[doc(alias = "___destroy_helper_block_500")]
// 0x2c21c — ___destroy_helper_block_500
pub fn stub_0x2c21c() -> ! {
    todo!("0x2c21c ___destroy_helper_block_500")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
// 0x2c5b0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0x2c5b0() -> ! {
    todo!("0x2c5b0 __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x2d370 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x2d370() -> ! {
    todo!("0x2d370 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x2d458 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x2d458() -> ! {
    todo!("0x2d458 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")]
// 0x317e4 — __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x317e4() -> ! {
    todo!("0x317e4 __ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv")]
// 0x31828 — __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv
pub fn stub_0x31828() -> ! {
    todo!("0x31828 __ZN3RBX4Name13callDoDeclareILZNS_13sLoginServiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")]
// 0x3182c — __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
pub fn stub_0x3182c() -> ! {
    todo!("0x3182c __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")]
// 0x31c30 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv
// type: int(void)
pub fn stub_0x31c30() -> ! {
    todo!("0x31c30 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv")]
// 0x32408 — __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv
pub fn stub_0x32408() -> ! {
    todo!("0x32408 __ZN3RBX4Name13callDoDeclareILZNS_11sGuiServiceEEEEvv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x32410 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
pub fn stub_0x32410() -> ! {
    todo!("0x32410 __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")]
// 0x3247c — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
pub fn stub_0x3247c() -> ! {
    todo!("0x3247c __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x324fc — __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int(void)
pub fn stub_0x324fc() -> ! {
    todo!("0x324fc rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TaskSchedulerSettings,RBX::TaskSchedulerSettings>(rbx_core::SharedPtr<RBX::TaskSchedulerSettings> const*,RBX::TaskSchedulerSettings *)const")]
// 0x32520 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21TaskSchedulerSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0x32520() -> ! {
    todo!("0x32520 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TaskSchedulerSettings,RBX::TaskSchedulerSettings>(rbx_core::SharedPtr<RBX::TaskSchedulerSettings> const*,RBX::TaskSchedulerSettings *)const")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x325fc — __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x325fc() -> ! {
    todo!("0x325fc boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x326fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_0x326fc() -> ! {
    todo!("0x326fc boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x32700 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_0x32700() -> ! {
    todo!("0x32700 boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
// 0x32720 — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0x32720() -> ! {
    todo!("0x32720 __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv")]
// 0x32764 — __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv
pub fn stub_0x32764() -> ! {
    todo!("0x32764 __ZN3RBX4Name13callDoDeclareILZNS_22sTaskSchedulerSettingsEEEEvv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")]
// 0x32768 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
// type: int(void)
pub fn stub_0x32768() -> ! {
    todo!("0x32768 __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
pub fn stub_0x33454() -> ! {
    todo!("0x33454 boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
pub fn stub_0x3346c() -> ! {
    todo!("0x3346c boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

#[doc(alias = "global constructor keyed to_a_8")]
// 0x355c8 — __GLOBAL__I_a_8
pub fn stub_0x355c8() -> ! {
    todo!("0x355c8 global constructor keyed to_a_8")
}

#[doc(alias = "_ReachabilityCallback")]
// 0x358ec — _ReachabilityCallback
// type: id __fastcall(int, int, int)
pub fn stub_0x358ec() -> ! {
    todo!("0x358ec _ReachabilityCallback")
}

#[doc(alias = "_PrintReachabilityFlags")]
// 0x35bd0 — _PrintReachabilityFlags
pub fn stub_0x35bd0() -> ! {
    todo!("0x35bd0 _PrintReachabilityFlags")
}

#[doc(alias = "___copy_helper_block__5")]
// 0x35e7c — ___copy_helper_block__5
pub fn stub_0x35e7c() -> ! {
    todo!("0x35e7c ___copy_helper_block__5")
}

#[doc(alias = "___destroy_helper_block__5")]
// 0x35e88 — ___destroy_helper_block__5
pub fn stub_0x35e88() -> ! {
    todo!("0x35e88 ___destroy_helper_block__5")
}

