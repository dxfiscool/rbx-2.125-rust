//! core wd_watchdog — 120 core stubs EA-sorted asc gap filler not yet in core (global fallback).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 uncovered foundation/global distinct not yet in crates/core/src (37157 uncovered before -> 37037 after, batch 0x4c1840..0x4c7cd0).
//! Filter: foundation exhausted (32689/32689 stubbed) — fallback to global gap filler EA-sorted asc distinct.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// 0x4c1840 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED1Ev
pub fn stub_0x4c1840() -> ! {
    todo!("0x4c1840 __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// 0x4c1844 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev
pub fn stub_0x4c1844() -> ! {
    todo!("0x4c1844 __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// 0x4c1a18 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED0Ev
pub fn stub_0x4c1a18() -> ! {
    todo!("0x4c1a18 __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(char const*)const")]
// 0x4c1ab8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupEPKc
pub fn stub_0x4c1ab8() -> ! {
    todo!("0x4c1ab8 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c1ae8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupERKNS0_7VariantE
pub fn stub_0x4c1ae8() -> ! {
    todo!("0x4c1ae8 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c1b08 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c1b08() -> ! {
    todo!("0x4c1b08 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(unsigned long,std::string &)const")]
// 0x4c1b64 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringEmRSs
pub fn stub_0x4c1b64() -> ! {
    todo!("0x4c1b64 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(RBX::SpecialShape::MeshType const&)const")]
// 0x4c1ca8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c1ca8() -> ! {
    todo!("0x4c1ca8 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToItem(RBX::SpecialShape::MeshType const&)const")]
// 0x4c1f14 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE13convertToItemERKS3_
pub fn stub_0x4c1f14() -> ! {
    todo!("0x4c1f14 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(RBX::Name const&,RBX::SpecialShape::MeshType&)const")]
// 0x4c20d0 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c20d0() -> ! {
    todo!("0x4c20d0 __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)")]
// 0x4c2174 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE13initSingletonEv
pub fn stub_0x4c2174() -> ! {
    todo!("0x4c2174 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")]
// 0x4c2178 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv
pub fn stub_0x4c2178() -> ! {
    todo!("0x4c2178 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// 0x4c2268 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED1Ev
pub fn stub_0x4c2268() -> ! {
    todo!("0x4c2268 __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// 0x4c226c — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev
pub fn stub_0x4c226c() -> ! {
    todo!("0x4c226c __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// 0x4c2440 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED0Ev
pub fn stub_0x4c2440() -> ! {
    todo!("0x4c2440 __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const")]
// 0x4c24e0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupEPKc
pub fn stub_0x4c24e0() -> ! {
    todo!("0x4c24e0 __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c2510 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupERKNS0_7VariantE
pub fn stub_0x4c2510() -> ! {
    todo!("0x4c2510 __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c2530 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c2530() -> ! {
    todo!("0x4c2530 __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const")]
// 0x4c258c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringEmRSs
pub fn stub_0x4c258c() -> ! {
    todo!("0x4c258c __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")]
// 0x4c26d0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c26d0() -> ! {
    todo!("0x4c26d0 __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringERKS2_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")]
// 0x4c293c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE13convertToItemERKS2_
pub fn stub_0x4c293c() -> ! {
    todo!("0x4c293c __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE13convertToItemERKS2_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")]
// 0x4c2af8 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueERKNS_4NameERS2_
pub fn stub_0x4c2af8() -> ! {
    todo!("0x4c2af8 __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueERKNS_4NameERS2_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::initSingleton(void)")]
// 0x4c2b9c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE13initSingletonEv
pub fn stub_0x4c2b9c() -> ! {
    todo!("0x4c2b9c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::doGetSingleton(void)")]
// 0x4c2ba0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE14doGetSingletonEv
pub fn stub_0x4c2ba0() -> ! {
    todo!("0x4c2ba0 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// 0x4c2c90 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED1Ev
pub fn stub_0x4c2c90() -> ! {
    todo!("0x4c2c90 __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// 0x4c2c94 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED2Ev
pub fn stub_0x4c2c94() -> ! {
    todo!("0x4c2c94 __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// 0x4c2e68 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED0Ev
// type: void __fastcall(void *, int, int, int)
pub fn stub_0x4c2e68() -> ! {
    todo!("0x4c2e68 __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(char const*)const")]
// 0x4c2f08 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupEPKc
pub fn stub_0x4c2f08() -> ! {
    todo!("0x4c2f08 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c2f38 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupERKNS0_7VariantE
pub fn stub_0x4c2f38() -> ! {
    todo!("0x4c2f38 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c2f58 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c2f58() -> ! {
    todo!("0x4c2f58 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToString(unsigned long,std::string &)const")]
// 0x4c2fb4 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE15convertToStringEmRSs
pub fn stub_0x4c2fb4() -> ! {
    todo!("0x4c2fb4 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToString(RBX::SkateboardPlatform::MoveState const&)const")]
// 0x4c30f8 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c30f8() -> ! {
    todo!("0x4c30f8 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToItem(RBX::SkateboardPlatform::MoveState const&)const")]
// 0x4c3364 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE13convertToItemERKS3_
pub fn stub_0x4c3364() -> ! {
    todo!("0x4c3364 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(RBX::Name const&,RBX::SkateboardPlatform::MoveState&)const")]
// 0x4c3520 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c3520() -> ! {
    todo!("0x4c3520 __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle> const>::initSingleton(void)")]
// 0x4c35c4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Handles11VisualStyleEEEE13initSingletonEv
pub fn stub_0x4c35c4() -> ! {
    todo!("0x4c35c4 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Handles11VisualStyleEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle> const>::doGetSingleton(void)")]
// 0x4c35c8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Handles11VisualStyleEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x4c35c8() -> ! {
    todo!("0x4c35c8 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Handles11VisualStyleEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::~EnumDesc()")]
// 0x4c36b8 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED1Ev
pub fn stub_0x4c36b8() -> ! {
    todo!("0x4c36b8 __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::~EnumDesc()")]
// 0x4c36bc — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED2Ev
pub fn stub_0x4c36bc() -> ! {
    todo!("0x4c36bc __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::~EnumDesc()")]
// 0x4c3890 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED0Ev
pub fn stub_0x4c3890() -> ! {
    todo!("0x4c3890 __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::lookup(char const*)const")]
// 0x4c3930 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE6lookupEPKc
pub fn stub_0x4c3930() -> ! {
    todo!("0x4c3930 __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c3960 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE6lookupERKNS0_7VariantE
pub fn stub_0x4c3960() -> ! {
    todo!("0x4c3960 __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c3980 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c3980() -> ! {
    todo!("0x4c3980 __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToString(unsigned long,std::string &)const")]
// 0x4c39dc — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE15convertToStringEmRSs
pub fn stub_0x4c39dc() -> ! {
    todo!("0x4c39dc __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToString(RBX::Handles::VisualStyle const&)const")]
// 0x4c3b20 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c3b20() -> ! {
    todo!("0x4c3b20 __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToItem(RBX::Handles::VisualStyle const&)const")]
// 0x4c3d8c — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE13convertToItemERKS3_
pub fn stub_0x4c3d8c() -> ! {
    todo!("0x4c3d8c __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToValue(RBX::Name const&,RBX::Handles::VisualStyle&)const")]
// 0x4c3f48 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c3f48() -> ! {
    todo!("0x4c3f48 __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> const>::initSingleton(void)")]
// 0x4c3fec — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE13initSingletonEv
pub fn stub_0x4c3fec() -> ! {
    todo!("0x4c3fec __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> const>::doGetSingleton(void)")]
// 0x4c3ff0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE14doGetSingletonEv
pub fn stub_0x4c3ff0() -> ! {
    todo!("0x4c3ff0 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()")]
// 0x4c40e0 — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED1Ev
pub fn stub_0x4c40e0() -> ! {
    todo!("0x4c40e0 __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()")]
// 0x4c40e4 — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED2Ev
pub fn stub_0x4c40e4() -> ! {
    todo!("0x4c40e4 __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()")]
// 0x4c42b8 — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED0Ev
pub fn stub_0x4c42b8() -> ! {
    todo!("0x4c42b8 __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(char const*)const")]
// 0x4c4358 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupEPKc
pub fn stub_0x4c4358() -> ! {
    todo!("0x4c4358 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c4388 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupERKNS0_7VariantE
pub fn stub_0x4c4388() -> ! {
    todo!("0x4c4388 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c43a8 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c43a8() -> ! {
    todo!("0x4c43a8 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(unsigned long,std::string &)const")]
// 0x4c4404 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringEmRSs
pub fn stub_0x4c4404() -> ! {
    todo!("0x4c4404 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(RBX::FriendService::FriendEventType const&)const")]
// 0x4c4548 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c4548() -> ! {
    todo!("0x4c4548 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToItem(RBX::FriendService::FriendEventType const&)const")]
// 0x4c47a4 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE13convertToItemERKS3_
pub fn stub_0x4c47a4() -> ! {
    todo!("0x4c47a4 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(RBX::Name const&,RBX::FriendService::FriendEventType&)const")]
// 0x4c4960 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c4960() -> ! {
    todo!("0x4c4960 __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> const>::initSingleton(void)")]
// 0x4c4a04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE13initSingletonEv
pub fn stub_0x4c4a04() -> ! {
    todo!("0x4c4a04 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> const>::doGetSingleton(void)")]
// 0x4c4a08 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE14doGetSingletonEv
pub fn stub_0x4c4a08() -> ! {
    todo!("0x4c4a08 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()")]
// 0x4c4af8 — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED1Ev
pub fn stub_0x4c4af8() -> ! {
    todo!("0x4c4af8 __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()")]
// 0x4c4afc — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED2Ev
pub fn stub_0x4c4afc() -> ! {
    todo!("0x4c4afc __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()")]
// 0x4c4cd0 — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED0Ev
pub fn stub_0x4c4cd0() -> ! {
    todo!("0x4c4cd0 __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(char const*)const")]
// 0x4c4d70 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupEPKc
pub fn stub_0x4c4d70() -> ! {
    todo!("0x4c4d70 __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c4da0 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupERKNS0_7VariantE
pub fn stub_0x4c4da0() -> ! {
    todo!("0x4c4da0 __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c4dc0 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c4dc0() -> ! {
    todo!("0x4c4dc0 __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(unsigned long,std::string &)const")]
// 0x4c4e1c — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringEmRSs
pub fn stub_0x4c4e1c() -> ! {
    todo!("0x4c4e1c __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(RBX::FriendService::FriendStatus const&)const")]
// 0x4c4f60 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c4f60() -> ! {
    todo!("0x4c4f60 __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToItem(RBX::FriendService::FriendStatus const&)const")]
// 0x4c51cc — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE13convertToItemERKS3_
pub fn stub_0x4c51cc() -> ! {
    todo!("0x4c51cc __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(RBX::Name const&,RBX::FriendService::FriendStatus&)const")]
// 0x4c5388 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c5388() -> ! {
    todo!("0x4c5388 __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> const>::initSingleton(void)")]
// 0x4c542c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE13initSingletonEv
pub fn stub_0x4c542c() -> ! {
    todo!("0x4c542c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> const>::doGetSingleton(void)")]
// 0x4c5430 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE14doGetSingletonEv
pub fn stub_0x4c5430() -> ! {
    todo!("0x4c5430 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()")]
// 0x4c5520 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED1Ev
pub fn stub_0x4c5520() -> ! {
    todo!("0x4c5520 __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()")]
// 0x4c5524 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED2Ev
pub fn stub_0x4c5524() -> ! {
    todo!("0x4c5524 __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()")]
// 0x4c56f8 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED0Ev
pub fn stub_0x4c56f8() -> ! {
    todo!("0x4c56f8 __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(char const*)const")]
// 0x4c5798 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupEPKc
pub fn stub_0x4c5798() -> ! {
    todo!("0x4c5798 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c57c8 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupERKNS0_7VariantE
pub fn stub_0x4c57c8() -> ! {
    todo!("0x4c57c8 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c57e8 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c57e8() -> ! {
    todo!("0x4c57e8 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(unsigned long,std::string &)const")]
// 0x4c5844 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringEmRSs
pub fn stub_0x4c5844() -> ! {
    todo!("0x4c5844 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(RBX::PyramidInstance::NumSidesEnum const&)const")]
// 0x4c5988 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c5988() -> ! {
    todo!("0x4c5988 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToItem(RBX::PyramidInstance::NumSidesEnum const&)const")]
// 0x4c5bf4 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE13convertToItemERKS3_
pub fn stub_0x4c5bf4() -> ! {
    todo!("0x4c5bf4 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(RBX::Name const&,RBX::PyramidInstance::NumSidesEnum&)const")]
// 0x4c5db0 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c5db0() -> ! {
    todo!("0x4c5db0 __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> const>::initSingleton(void)")]
// 0x4c5e54 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE13initSingletonEv
pub fn stub_0x4c5e54() -> ! {
    todo!("0x4c5e54 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> const>::doGetSingleton(void)")]
// 0x4c5e58 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE14doGetSingletonEv
pub fn stub_0x4c5e58() -> ! {
    todo!("0x4c5e58 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()")]
// 0x4c5f48 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED1Ev
pub fn stub_0x4c5f48() -> ! {
    todo!("0x4c5f48 __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()")]
// 0x4c5f4c — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED2Ev
pub fn stub_0x4c5f4c() -> ! {
    todo!("0x4c5f4c __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()")]
// 0x4c6120 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED0Ev
pub fn stub_0x4c6120() -> ! {
    todo!("0x4c6120 __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(char const*)const")]
// 0x4c61c0 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupEPKc
pub fn stub_0x4c61c0() -> ! {
    todo!("0x4c61c0 __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c61f0 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupERKNS0_7VariantE
pub fn stub_0x4c61f0() -> ! {
    todo!("0x4c61f0 __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c6210 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c6210() -> ! {
    todo!("0x4c6210 __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(unsigned long,std::string &)const")]
// 0x4c626c — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringEmRSs
pub fn stub_0x4c626c() -> ! {
    todo!("0x4c626c __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(RBX::PrismInstance::NumSidesEnum const&)const")]
// 0x4c63b0 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c63b0() -> ! {
    todo!("0x4c63b0 __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToItem(RBX::PrismInstance::NumSidesEnum const&)const")]
// 0x4c661c — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE13convertToItemERKS3_
pub fn stub_0x4c661c() -> ! {
    todo!("0x4c661c __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(RBX::Name const&,RBX::PrismInstance::NumSidesEnum&)const")]
// 0x4c67d8 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c67d8() -> ! {
    todo!("0x4c67d8 __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle> const>::initSingleton(void)")]
// 0x4c687c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE13initSingletonEv
pub fn stub_0x4c687c() -> ! {
    todo!("0x4c687c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle> const>::doGetSingleton(void)")]
// 0x4c6880 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE14doGetSingletonEv
pub fn stub_0x4c6880() -> ! {
    todo!("0x4c6880 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
// 0x4c6970 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED1Ev
pub fn stub_0x4c6970() -> ! {
    todo!("0x4c6970 __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
// 0x4c6974 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev
pub fn stub_0x4c6974() -> ! {
    todo!("0x4c6974 __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
// 0x4c6b48 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED0Ev
pub fn stub_0x4c6b48() -> ! {
    todo!("0x4c6b48 __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(char const*)const")]
// 0x4c6be8 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupEPKc
pub fn stub_0x4c6be8() -> ! {
    todo!("0x4c6be8 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c6c18 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupERKNS0_7VariantE
pub fn stub_0x4c6c18() -> ! {
    todo!("0x4c6c18 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c6c38 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c6c38() -> ! {
    todo!("0x4c6c38 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(unsigned long,std::string &)const")]
// 0x4c6c94 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringEmRSs
pub fn stub_0x4c6c94() -> ! {
    todo!("0x4c6c94 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
// 0x4c6dd8 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c6dd8() -> ! {
    todo!("0x4c6dd8 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToItem(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
// 0x4c7044 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE13convertToItemERKS3_
pub fn stub_0x4c7044() -> ! {
    todo!("0x4c7044 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(RBX::Name const&,RBX::ExtrudedPartInstance::VisualTrussStyle&)const")]
// 0x4c7200 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c7200() -> ! {
    todo!("0x4c7200 __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::initSingleton(void)")]
// 0x4c72a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE13initSingletonEv
pub fn stub_0x4c72a4() -> ! {
    todo!("0x4c72a4 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::doGetSingleton(void)")]
// 0x4c72a8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x4c72a8() -> ! {
    todo!("0x4c72a8 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
// 0x4c7398 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED1Ev
pub fn stub_0x4c7398() -> ! {
    todo!("0x4c7398 __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
// 0x4c739c — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0x4c739c() -> ! {
    todo!("0x4c739c __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
// 0x4c7570 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_0x4c7570() -> ! {
    todo!("0x4c7570 __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(char const*)const")]
// 0x4c7610 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
pub fn stub_0x4c7610() -> ! {
    todo!("0x4c7610 __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c7640 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
pub fn stub_0x4c7640() -> ! {
    todo!("0x4c7640 __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c7660 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c7660() -> ! {
    todo!("0x4c7660 __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(unsigned long,std::string &)const")]
// 0x4c76bc — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_0x4c76bc() -> ! {
    todo!("0x4c76bc __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(RBX::PersonalServerService::PrivilegeType const&)const")]
// 0x4c7800 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x4c7800() -> ! {
    todo!("0x4c7800 __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToItem(RBX::PersonalServerService::PrivilegeType const&)const")]
// 0x4c7a6c — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_0x4c7a6c() -> ! {
    todo!("0x4c7a6c __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(RBX::Name const&,RBX::PersonalServerService::PrivilegeType&)const")]
// 0x4c7c28 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
pub fn stub_0x4c7c28() -> ! {
    todo!("0x4c7c28 __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SocialService::StuffType> const>::initSingleton(void)")]
// 0x4c7ccc — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE13initSingletonEv
// type: int()
pub fn stub_0x4c7ccc() -> ! {
    todo!("0x4c7ccc __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SocialService::StuffType> const>::doGetSingleton(void)")]
// 0x4c7cd0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_0x4c7cd0() -> ! {
    todo!("0x4c7cd0 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE14doGetSingletonEv")
}
