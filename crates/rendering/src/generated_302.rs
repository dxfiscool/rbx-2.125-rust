//! rendering shard 302 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 32740->32840 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32740 before -> 32840 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x431ad4 (lowest remaining 0x431af8..0x438d9c, next lowest 0x438dbc)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x431af8 — __ZN3RBX9DataModel15getIsGameLoadedEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getIsGameLoaded(void)")]
// was: __ZN3RBX9DataModel15getIsGameLoadedEv
pub fn stub_431af8() -> ! {
    todo!("0x431af8 RBX::DataModel::getIsGameLoaded(void)")
}

// 0x431b00 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED1Ev
pub fn stub_431b00() -> ! {
    todo!("0x431b00 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc()")
}

// 0x431b48 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::addPair(RBX::DataModel::CreatorType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE7addPairES3_PKc
pub fn stub_431b48() -> ! {
    todo!("0x431b48 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::addPair(RBX::DataModel::CreatorType,char const*)")
}

// 0x431ea8 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel11CreatorTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::CreatorType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::CreatorType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel11CreatorTypeEEERT_v
pub fn stub_431ea8() -> ! {
    todo!("0x431ea8 RBX::DataModel::CreatorType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::CreatorType>(void)")
}

// 0x432094 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::addPair(RBX::DataModel::Genre,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEE7addPairES3_PKc
pub fn stub_432094() -> ! {
    todo!("0x432094 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::addPair(RBX::DataModel::Genre,char const*)")
}

// 0x4323f4 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel5GenreEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::Genre & RBX::Reflection::Variant::genericConvert<RBX::DataModel::Genre>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel5GenreEEERT_v
pub fn stub_4323f4() -> ! {
    todo!("0x4323f4 RBX::DataModel::Genre & RBX::Reflection::Variant::genericConvert<RBX::DataModel::Genre>(void)")
}

// 0x4325e0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::addPair(RBX::DataModel::GearGenreSetting,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE7addPairES3_PKc
pub fn stub_4325e0() -> ! {
    todo!("0x4325e0 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::addPair(RBX::DataModel::GearGenreSetting,char const*)")
}

// 0x432940 — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel16GearGenreSettingEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::GearGenreSetting & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearGenreSetting>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel16GearGenreSettingEEERT_v
pub fn stub_432940() -> ! {
    todo!("0x432940 RBX::DataModel::GearGenreSetting & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearGenreSetting>(void)")
}

// 0x432b2c — __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::addPair(RBX::DataModel::GearType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel8GearTypeEE7addPairES3_PKc
pub fn stub_432b2c() -> ! {
    todo!("0x432b2c RBX::Reflection::EnumDesc<RBX::DataModel::GearType>::addPair(RBX::DataModel::GearType,char const*)")
}

// 0x432e8c — __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel8GearTypeEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::GearType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_9DataModel8GearTypeEEERT_v
pub fn stub_432e8c() -> ! {
    todo!("0x432e8c RBX::DataModel::GearType & RBX::Reflection::Variant::genericConvert<RBX::DataModel::GearType>(void)")
}

// 0x433078 — __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::addPair(RBX::Instance::SaveFilter,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_8Instance10SaveFilterEE7addPairES3_PKc
pub fn stub_433078() -> ! {
    todo!("0x433078 RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter>::addPair(RBX::Instance::SaveFilter,char const*)")
}

// 0x4333d8 — __ZN3RBX10Reflection7Variant14genericConvertINS_8Instance10SaveFilterEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Instance::SaveFilter & RBX::Reflection::Variant::genericConvert<RBX::Instance::SaveFilter>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_8Instance10SaveFilterEEERT_v
pub fn stub_4333d8() -> ! {
    todo!("0x4333d8 RBX::Instance::SaveFilter & RBX::Reflection::Variant::genericConvert<RBX::Instance::SaveFilter>(void)")
}

// 0x4335c4 — __ZN3RBX15ServiceProvider4findINS_5VisitEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider4findINS_5VisitEEEPT_PKNS_8InstanceE
pub fn stub_4335c4() -> ! {
    todo!("0x4335c4 RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(RBX::Instance const*)")
}

// 0x4335e0 — __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::GenericJob>::operator=(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX9DataModel10GenericJobEEaSERKS4_
pub fn stub_4335e0() -> ! {
    todo!("0x4335e0 boost::shared_ptr<RBX::DataModel::GenericJob>::operator=(boost::shared_ptr<RBX::DataModel::GenericJob> const&)")
}

// 0x433618 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9DataModelEPNS_4VerbEPS4_EEN5boost10shared_ptrIT_EET0_T1_
// type: void __fastcall(int, RBX::Verb *, RBX::DataModel *)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel> RBX::Creatable<RBX::Instance>::create<RBX::DataModel,RBX::Verb *,RBX::DataModel*>(RBX::Verb *,RBX::DataModel*)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9DataModelEPNS_4VerbEPS4_EEN5boost10shared_ptrIT_EET0_T1_
pub fn stub_433618() -> ! {
    todo!("0x433618 boost::shared_ptr<RBX::DataModel> RBX::Creatable<RBX::Instance>::create<RBX::DataModel,RBX::Verb *,RBX::DataModel*>(RBX::Verb *,RBX::DataModel*)")
}

// 0x4336d8 — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// was: __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEE
pub fn stub_4336d8() -> ! {
    todo!("0x4336d8 RBX::TaskScheduler::removeBlocking(boost::shared_ptr<RBX::TaskScheduler::Job>)")
}

// 0x4337d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9WorkspaceEPNS_9DataModelEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace> RBX::Creatable<RBX::Instance>::create<RBX::Workspace,RBX::DataModel *>(RBX::DataModel *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9WorkspaceEPNS_9DataModelEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4337d0() -> ! {
    todo!("0x4337d0 boost::shared_ptr<RBX::Workspace> RBX::Creatable<RBX::Instance>::create<RBX::Workspace,RBX::DataModel *>(RBX::DataModel *)")
}

// 0x43388c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7GuiRootEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot> RBX::Creatable<RBX::Instance>::create<RBX::GuiRoot>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_7GuiRootEEEN5boost10shared_ptrIT_EEv
pub fn stub_43388c() -> ! {
    todo!("0x43388c boost::shared_ptr<RBX::GuiRoot> RBX::Creatable<RBX::Instance>::create<RBX::GuiRoot>(void)")
}

// 0x433940 — __ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ContentFilter * RBX::ServiceProvider::create<RBX::ContentFilter>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v
pub fn stub_433940() -> ! {
    todo!("0x433940 RBX::ContentFilter * RBX::ServiceProvider::create<RBX::ContentFilter>(void)const")
}

// 0x433b08 — __ZNK3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_v
pub fn stub_433b08() -> ! {
    todo!("0x433b08 RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(void)const")
}

// 0x433cd0 — __ZNK3RBX15ServiceProvider6createINS_10GuiServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::create<RBX::GuiService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_10GuiServiceEEEPT_v
pub fn stub_433cd0() -> ! {
    todo!("0x433cd0 RBX::GuiService * RBX::ServiceProvider::create<RBX::GuiService>(void)const")
}

// 0x433e98 — __ZNK3RBX15ServiceProvider6createINS_11ChatServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ChatService * RBX::ServiceProvider::create<RBX::ChatService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_11ChatServiceEEEPT_v
pub fn stub_433e98() -> ! {
    todo!("0x433e98 RBX::ChatService * RBX::ServiceProvider::create<RBX::ChatService>(void)const")
}

// 0x434078 — __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack>::operator=(rbx_core::SharedPtr<RBX::LocalBackpack> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEaSERKS3_
pub fn stub_434078() -> ! {
    todo!("0x434078 boost::shared_ptr<RBX::LocalBackpack>::operator=(boost::shared_ptr<RBX::LocalBackpack> const&)")
}

// 0x4340b0 — __ZN3RBX11shared_fromINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack> RBX::shared_from<RBX::LocalBackpack>(RBX::LocalBackpack*)")]
// was: __ZN3RBX11shared_fromINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_4340b0() -> ! {
    todo!("0x4340b0 boost::shared_ptr<RBX::LocalBackpack> RBX::shared_from<RBX::LocalBackpack>(RBX::LocalBackpack*)")
}

// 0x434198 — __ZNK3RBX15ServiceProvider6createINS_13LocalBackpackEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::create<RBX::LocalBackpack>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13LocalBackpackEEEPT_v
pub fn stub_434198() -> ! {
    todo!("0x434198 RBX::LocalBackpack * RBX::ServiceProvider::create<RBX::LocalBackpack>(void)const")
}

// 0x434360 — __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD>::operator=(rbx_core::SharedPtr<RBX::PlayerHUD> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEaSERKS3_
pub fn stub_434360() -> ! {
    todo!("0x434360 boost::shared_ptr<RBX::PlayerHUD>::operator=(boost::shared_ptr<RBX::PlayerHUD> const&)")
}

// 0x434398 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerHUDEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD> RBX::Creatable<RBX::Instance>::create<RBX::PlayerHUD>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerHUDEEEN5boost10shared_ptrIT_EEv
pub fn stub_434398() -> ! {
    todo!("0x434398 boost::shared_ptr<RBX::PlayerHUD> RBX::Creatable<RBX::Instance>::create<RBX::PlayerHUD>(void)")
}

// 0x434448 — __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService>::operator=(rbx_core::SharedPtr<RBX::StarterPackService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEaSERKS3_
pub fn stub_434448() -> ! {
    todo!("0x434448 boost::shared_ptr<RBX::StarterPackService>::operator=(boost::shared_ptr<RBX::StarterPackService> const&)")
}

// 0x434480 — __ZN3RBX11shared_fromINS_18StarterPackServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService> RBX::shared_from<RBX::StarterPackService>(RBX::StarterPackService*)")]
// was: __ZN3RBX11shared_fromINS_18StarterPackServiceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_434480() -> ! {
    todo!("0x434480 boost::shared_ptr<RBX::StarterPackService> RBX::shared_from<RBX::StarterPackService>(RBX::StarterPackService*)")
}

// 0x434568 — __ZNK3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::StarterPackService * RBX::ServiceProvider::create<RBX::StarterPackService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_v
pub fn stub_434568() -> ! {
    todo!("0x434568 RBX::StarterPackService * RBX::ServiceProvider::create<RBX::StarterPackService>(void)const")
}

// 0x434730 — __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService>::operator=(rbx_core::SharedPtr<RBX::StarterGuiService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEaSERKS3_
pub fn stub_434730() -> ! {
    todo!("0x434730 boost::shared_ptr<RBX::StarterGuiService>::operator=(boost::shared_ptr<RBX::StarterGuiService> const&)")
}

// 0x434768 — __ZN3RBX11shared_fromINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService> RBX::shared_from<RBX::StarterGuiService>(RBX::StarterGuiService*)")]
// was: __ZN3RBX11shared_fromINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_434768() -> ! {
    todo!("0x434768 boost::shared_ptr<RBX::StarterGuiService> RBX::shared_from<RBX::StarterGuiService>(RBX::StarterGuiService*)")
}

// 0x434850 — __ZNK3RBX15ServiceProvider6createINS_17StarterGuiServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::StarterGuiService * RBX::ServiceProvider::create<RBX::StarterGuiService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_17StarterGuiServiceEEEPT_v
pub fn stub_434850() -> ! {
    todo!("0x434850 RBX::StarterGuiService * RBX::ServiceProvider::create<RBX::StarterGuiService>(void)const")
}

// 0x434a18 — __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService>::operator=(rbx_core::SharedPtr<RBX::CoreGuiService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX14CoreGuiServiceEEaSERKS3_
pub fn stub_434a18() -> ! {
    todo!("0x434a18 boost::shared_ptr<RBX::CoreGuiService>::operator=(boost::shared_ptr<RBX::CoreGuiService> const&)")
}

// 0x434a50 — __ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService> RBX::shared_from<RBX::CoreGuiService>(RBX::CoreGuiService*)")]
// was: __ZN3RBX11shared_fromINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_434a50() -> ! {
    todo!("0x434a50 boost::shared_ptr<RBX::CoreGuiService> RBX::shared_from<RBX::CoreGuiService>(RBX::CoreGuiService*)")
}

// 0x434b38 — __ZNK3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::create<RBX::CoreGuiService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_14CoreGuiServiceEEEPT_v
pub fn stub_434b38() -> ! {
    todo!("0x434b38 RBX::CoreGuiService * RBX::ServiceProvider::create<RBX::CoreGuiService>(void)const")
}

// 0x434d00 — __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v
pub fn stub_434d00() -> ! {
    todo!("0x434d00 RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")
}

// 0x434edc — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_9DataModelES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_9DataModelES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_434edc() -> ! {
    todo!("0x434edc rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>> const&)")
}

// 0x434f50 — __ZNK3RBX15ServiceProvider6createINS_13JointsServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::JointsService * RBX::ServiceProvider::create<RBX::JointsService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13JointsServiceEEEPT_v
pub fn stub_434f50() -> ! {
    todo!("0x434f50 RBX::JointsService * RBX::ServiceProvider::create<RBX::JointsService>(void)const")
}

// 0x435118 — __ZNK3RBX15ServiceProvider6createINS_17CollectionServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CollectionService * RBX::ServiceProvider::create<RBX::CollectionService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_17CollectionServiceEEEPT_v
pub fn stub_435118() -> ! {
    todo!("0x435118 RBX::CollectionService * RBX::ServiceProvider::create<RBX::CollectionService>(void)const")
}

// 0x4352e0 — __ZNK3RBX15ServiceProvider6createINS_14PhysicsServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::create<RBX::PhysicsService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_14PhysicsServiceEEEPT_v
pub fn stub_4352e0() -> ! {
    todo!("0x4352e0 RBX::PhysicsService * RBX::ServiceProvider::create<RBX::PhysicsService>(void)const")
}

// 0x4354a8 — __ZNK3RBX15ServiceProvider6createINS_12BadgeServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::create<RBX::BadgeService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_12BadgeServiceEEEPT_v
pub fn stub_4354a8() -> ! {
    todo!("0x4354a8 RBX::BadgeService * RBX::ServiceProvider::create<RBX::BadgeService>(void)const")
}

// 0x435684 — __ZNK3RBX15ServiceProvider6createINS_15GeometryServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::create<RBX::GeometryService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_15GeometryServiceEEEPT_v
pub fn stub_435684() -> ! {
    todo!("0x435684 RBX::GeometryService * RBX::ServiceProvider::create<RBX::GeometryService>(void)const")
}

// 0x43584c — __ZNK3RBX15ServiceProvider6createINS_13FriendServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FriendService * RBX::ServiceProvider::create<RBX::FriendService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13FriendServiceEEEPT_v
pub fn stub_43584c() -> ! {
    todo!("0x43584c RBX::FriendService * RBX::ServiceProvider::create<RBX::FriendService>(void)const")
}

// 0x435bf0 — __ZNK3RBX15ServiceProvider6createINS_13InsertServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::InsertService * RBX::ServiceProvider::create<RBX::InsertService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13InsertServiceEEEPT_v
pub fn stub_435bf0() -> ! {
    todo!("0x435bf0 RBX::InsertService * RBX::ServiceProvider::create<RBX::InsertService>(void)const")
}

// 0x435dcc — __ZNK3RBX15ServiceProvider6createINS_13SocialServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::create<RBX::SocialService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13SocialServiceEEEPT_v
pub fn stub_435dcc() -> ! {
    todo!("0x435dcc RBX::SocialService * RBX::ServiceProvider::create<RBX::SocialService>(void)const")
}

// 0x435f94 — __ZNK3RBX15ServiceProvider6createINS_15GamePassServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GamePassService * RBX::ServiceProvider::create<RBX::GamePassService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_15GamePassServiceEEEPT_v
pub fn stub_435f94() -> ! {
    todo!("0x435f94 RBX::GamePassService * RBX::ServiceProvider::create<RBX::GamePassService>(void)const")
}

// 0x43615c — __ZNK3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_v
pub fn stub_43615c() -> ! {
    todo!("0x43615c RBX::DebrisService * RBX::ServiceProvider::create<RBX::DebrisService>(void)const")
}

// 0x436324 — __ZNK3RBX15ServiceProvider6createINS_14CookiesServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CookiesService * RBX::ServiceProvider::create<RBX::CookiesService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_14CookiesServiceEEEPT_v
pub fn stub_436324() -> ! {
    todo!("0x436324 RBX::CookiesService * RBX::ServiceProvider::create<RBX::CookiesService>(void)const")
}

// 0x4364ec — __ZNK3RBX15ServiceProvider6createINS_15TeleportServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TeleportService * RBX::ServiceProvider::create<RBX::TeleportService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_15TeleportServiceEEEPT_v
pub fn stub_4364ec() -> ! {
    todo!("0x4364ec RBX::TeleportService * RBX::ServiceProvider::create<RBX::TeleportService>(void)const")
}

// 0x4366b4 — __ZNK3RBX15ServiceProvider6createINS_21PersonalServerServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::PersonalServerService * RBX::ServiceProvider::create<RBX::PersonalServerService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_21PersonalServerServiceEEEPT_v
pub fn stub_4366b4() -> ! {
    todo!("0x4366b4 RBX::PersonalServerService * RBX::ServiceProvider::create<RBX::PersonalServerService>(void)const")
}

// 0x43687c — __ZNK3RBX15ServiceProvider6createINS_9FWServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::FWService * RBX::ServiceProvider::create<RBX::FWService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_9FWServiceEEEPT_v
pub fn stub_43687c() -> ! {
    todo!("0x43687c RBX::FWService * RBX::ServiceProvider::create<RBX::FWService>(void)const")
}

// 0x436a48 — __ZNK3RBX15ServiceProvider6createINS_20ContextActionServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ContextActionService * RBX::ServiceProvider::create<RBX::ContextActionService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_20ContextActionServiceEEEPT_v
pub fn stub_436a48() -> ! {
    todo!("0x436a48 RBX::ContextActionService * RBX::ServiceProvider::create<RBX::ContextActionService>(void)const")
}

// 0x436c10 — __ZNK3RBX15ServiceProvider6createINS_13ScriptServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::create<RBX::ScriptService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13ScriptServiceEEEPT_v
pub fn stub_436c10() -> ! {
    todo!("0x436c10 RBX::ScriptService * RBX::ServiceProvider::create<RBX::ScriptService>(void)const")
}

// 0x436dd8 — __ZNK3RBX15ServiceProvider6createINS_12AssetServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::AssetService * RBX::ServiceProvider::create<RBX::AssetService>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_12AssetServiceEEEPT_v
pub fn stub_436dd8() -> ! {
    todo!("0x436dd8 RBX::AssetService * RBX::ServiceProvider::create<RBX::AssetService>(void)const")
}

// 0x436fa0 — __ZN3rbx22timestamped_safe_queueIN5boost8functionIFvPN3RBX9DataModelEEEEE4pushERKS7_
// type: void __fastcall(int)
#[doc(alias = "rbx::timestamped_safe_queue<boost::function<void ()(RBX::DataModel *)>>::push(boost::function<void ()(RBX::DataModel *)> const&)")]
// was: __ZN3rbx22timestamped_safe_queueIN5boost8functionIFvPN3RBX9DataModelEEEEE4pushERKS7_
pub fn stub_436fa0() -> ! {
    todo!("0x436fa0 rbx::timestamped_safe_queue<boost::function<void ()(RBX::DataModel *)>>::push(boost::function<void ()(RBX::DataModel *)> const&)")
}

// 0x437060 — __ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFvSsEEES6_NS_3argILi1EEENS7_ILi2EEES6_S6_EENS_3_bi6bind_tIT_PFSC_T0_T1_T2_T3_ENSA_9list_av_4IT4_T5_T6_T7_E4typeEEESI_SK_SL_SM_SN_
// type: void __fastcall(_DWORD *, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFvSsEEES6_NS_3argILi1EEENS7_ILi2EEES6_S6_EENS_3_bi6bind_tIT_PFSC_T0_T1_T2_T3_ENSA_9list_av_4IT4_T5_T6_T7_E4typeEEESI_SK_SL_SM_SN_
pub fn stub_437060() -> ! {
    todo!("0x437060 boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>::type> boost::bind<void,std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>,boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>>(void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::arg<1>,boost::arg<2>,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x437214 — __ZN5boost4bindISsRKSsSsEENS_3_bi6bind_tIT_PFS5_T0_ENS3_9list_av_1IT1_E4typeEEES8_SA_
// type: void __fastcall(_DWORD *, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list_av_1<std::string>::type> boost::bind<std::string,std::string const&,std::string>(std::string (*)(std::string const&),std::string)")]
// was: __ZN5boost4bindISsRKSsSsEENS_3_bi6bind_tIT_PFS5_T0_ENS3_9list_av_1IT1_E4typeEEES8_SA_
pub fn stub_437214() -> ! {
    todo!("0x437214 boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list_av_1<std::string>::type> boost::bind<std::string,std::string const&,std::string>(std::string (*)(std::string const&),std::string)")
}

// 0x4373bc — __ZN5boost4bindISsRKSsS2_SsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_ENS3_9list_av_2IT2_T3_E4typeEEES9_SB_SC_
// type: void __fastcall(int, int, std::string *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list_av_2<std::string,std::string>::type> boost::bind<std::string,std::string const&,std::string const&,std::string,std::string>(std::string (*)(std::string const&,std::string const&),std::string,std::string)")]
// was: __ZN5boost4bindISsRKSsS2_SsSsEENS_3_bi6bind_tIT_PFS5_T0_T1_ENS3_9list_av_2IT2_T3_E4typeEEES9_SB_SC_
pub fn stub_4373bc() -> ! {
    todo!("0x4373bc boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list_av_2<std::string,std::string>::type> boost::bind<std::string,std::string const&,std::string const&,std::string,std::string>(std::string (*)(std::string const&,std::string const&),std::string,std::string)")
}

// 0x437650 — __ZN3RBX9Workspace22getCurrentMouseCommandEv
// type: int __fastcall(RBX::Workspace *this)
#[doc(alias = "RBX::Workspace::getCurrentMouseCommand(void)")]
// was: __ZN3RBX9Workspace22getCurrentMouseCommandEv
pub fn stub_437650() -> ! {
    todo!("0x437650 RBX::Workspace::getCurrentMouseCommand(void)")
}

// 0x4376b0 — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7UIEventEEEclES5_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UIEvent const&)>::operator()(RBX::UIEvent const&)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7UIEventEEEclES5_
pub fn stub_4376b0() -> ! {
    todo!("0x4376b0 rbx::signals::signal_with_args<1,void ()(RBX::UIEvent const&)>::operator()(RBX::UIEvent const&)")
}

// 0x4377f8 — __ZNK3RBX8Instance22countDescendantsOfTypeIS0_EEiv
// type: int __fastcall(const shared_count *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::Instance>(void)const")]
// was: __ZNK3RBX8Instance22countDescendantsOfTypeIS0_EEiv
pub fn stub_4377f8() -> ! {
    todo!("0x4377f8 int RBX::Instance::countDescendantsOfType<RBX::Instance>(void)const")
}

// 0x437914 — __ZNK3RBX8Instance22countDescendantsOfTypeINS_12PartInstanceEEEiv
// type: int __fastcall(const shared_count *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::PartInstance>(void)const")]
// was: __ZNK3RBX8Instance22countDescendantsOfTypeINS_12PartInstanceEEEiv
pub fn stub_437914() -> ! {
    todo!("0x437914 int RBX::Instance::countDescendantsOfType<RBX::PartInstance>(void)const")
}

// 0x437a30 — __ZNK3RBX8Instance22countDescendantsOfTypeINS_10BaseScriptEEEiv
// type: int __fastcall(const shared_count *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::BaseScript>(void)const")]
// was: __ZNK3RBX8Instance22countDescendantsOfTypeINS_10BaseScriptEEEiv
pub fn stub_437a30() -> ! {
    todo!("0x437a30 int RBX::Instance::countDescendantsOfType<RBX::BaseScript>(void)const")
}

// 0x437b50 — __ZN3RBX32shared_from_polymorphic_downcastINS_5Stats4ItemENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::Item> RBX::shared_from_polymorphic_downcast<RBX::Stats::Item,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
// was: __ZN3RBX32shared_from_polymorphic_downcastINS_5Stats4ItemENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
pub fn stub_437b50() -> ! {
    todo!("0x437b50 boost::shared_ptr<RBX::Stats::Item> RBX::shared_from_polymorphic_downcast<RBX::Stats::Item,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")
}

// 0x437c38 — __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE3strEv
// type: void __fastcall(std::string *, int *)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::str(void)const")]
// was: __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE3strEv
pub fn stub_437c38() -> ! {
    todo!("0x437c38 boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::str(void)const")
}

// 0x437e68 — __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv
// type: __int64 __fastcall(int)
#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::rate(void)const")]
// was: __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv
pub fn stub_437e68() -> ! {
    todo!("0x437e68 RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::rate(void)const")
}

// 0x437ec8 — __ZNK3RBX6Kernel9numBodiesEv
// type: int __fastcall(RBX::Kernel *this)
#[doc(alias = "RBX::Kernel::numBodies(void)const")]
// was: __ZNK3RBX6Kernel9numBodiesEv
pub fn stub_437ec8() -> ! {
    todo!("0x437ec8 RBX::Kernel::numBodies(void)const")
}

// 0x437ef0 — __ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void)")]
// was: __ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv
pub fn stub_437ef0() -> ! {
    todo!("0x437ef0 boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void)")
}

// 0x438048 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvPFvPNS4_9DataModelES8_PSA_INS4_10Reflection7VariantESaISJ_EEENSE_5list3INSE_5valueISH_EENS2_3argILi1EEENSQ_ISM_EEEEEEET0_T_SY_SX_
// type: int __fastcall(int, int, int, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvPFvPNS4_9DataModelES8_PSA_INS4_10Reflection7VariantESaISJ_EEENSE_5list3INSE_5valueISH_EENS2_3argILi1EEENSQ_ISM_EEEEEEET0_T_SY_SX_
pub fn stub_438048() -> ! {
    todo!("0x438048 boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>),boost::_bi::list3<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::vector*<RBX::Reflection::Variant,std::allocator<RBX::Reflection>>>>>)")
}

// 0x43809c — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvPFvPNS4_9DataModelES8_RSsdPdENSE_5list5INSE_5valueISH_EENS2_3argILi1EEENSN_ISsEENSN_IdEENSN_ISJ_EEEEEEET0_T_SX_SW_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvPFvPNS4_9DataModelES8_RSsdPdENSE_5list5INSE_5valueISH_EENS2_3argILi1EEENSN_ISsEENSN_IdEENSN_ISJ_EEEEEEET0_T_SX_SW_
pub fn stub_43809c() -> ! {
    todo!("0x43809c boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list5<boost::_bi::value<RBX::DataModel *>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<double>,boost::_bi::value<double *>>>)")
}

// 0x4380fc — __ZN5boost4bindIvPN3RBX9DataModelENS_10shared_ptrIKNS1_13TaskScheduler3JobEEERSsdPdS3_NS_3argILi1EEESsdSA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_T4_ENSD_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESM_SO_SP_SQ_SR_SS_
// type: void __fastcall(int, int, int, std::string *, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list_av_5<RBX::DataModel *,boost::arg<1>,std::string,double,double *>::type> boost::bind<void,RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *,RBX::DataModel *,boost::arg<1>,std::string,double,double *>(void (*)(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *),RBX::DataModel *,boost::arg<1>,std::string,double,double *)")]
// was: __ZN5boost4bindIvPN3RBX9DataModelENS_10shared_ptrIKNS1_13TaskScheduler3JobEEERSsdPdS3_NS_3argILi1EEESsdSA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_T4_ENSD_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESM_SO_SP_SQ_SR_SS_
pub fn stub_4380fc() -> ! {
    todo!("0x4380fc boost::_bi::bind_t<void,void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),boost::_bi::list_av_5<RBX::DataModel *,boost::arg<1>,std::string,double,double *>::type> boost::bind<void,RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *,RBX::DataModel *,boost::arg<1>,std::string,double,double *>(void (*)(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *),RBX::DataModel *,boost::arg<1>,std::string,double,double *)")
}

// 0x4382c8 — __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEEclES6_SA_
// type: void __fastcall(_DWORD *, int, int, const void *)
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3rbx7signals16signal_with_argsILi2EFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEEclES6_SA_
pub fn stub_4382c8() -> ! {
    todo!("0x4382c8 rbx::signals::signal_with_args<2,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}

// 0x4384b4 — __ZN3rbx7signals16signal_with_argsILi1EFvRKSsEEclES3_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(std::string const&)>::operator()(std::string const&)")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvRKSsEEclES3_
pub fn stub_4384b4() -> ! {
    todo!("0x4384b4 rbx::signals::signal_with_args<1,void ()(std::string const&)>::operator()(std::string const&)")
}

// 0x4385f8 — __ZNK3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::find<RBX::CoreGuiService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_v
pub fn stub_4385f8() -> ! {
    todo!("0x4385f8 RBX::CoreGuiService * RBX::ServiceProvider::find<RBX::CoreGuiService>(void)const")
}

// 0x43876c — __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv
pub fn stub_43876c() -> ! {
    todo!("0x43876c __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")
}

// 0x438798 — __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv
pub fn stub_438798() -> ! {
    todo!("0x438798 __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")
}

// 0x4387c4 — __ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7CreatorD1Ev
pub fn stub_4387c4() -> ! {
    todo!("0x4387c4 __ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7CreatorD1Ev")
}

// 0x4387c8 — __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7CreatorD1Ev
pub fn stub_4387c8() -> ! {
    todo!("0x4387c8 __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7CreatorD1Ev")
}

// 0x4387d0 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD1Ev
pub fn stub_4387d0() -> ! {
    todo!("0x4387d0 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD1Ev")
}

// 0x4387d8 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev
pub fn stub_4387d8() -> ! {
    todo!("0x4387d8 __ZN3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E7CreatorD1Ev")
}

// 0x4387e0 — __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD1Ev
pub fn stub_4387e0() -> ! {
    todo!("0x4387e0 __ZN3RBX14FactoryProductINS_13ServerStorageENS_8InstanceELZNS_14sServerStorageEES2_E7CreatorD1Ev")
}

// 0x4387e8 — __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD1Ev
pub fn stub_4387e8() -> ! {
    todo!("0x4387e8 __ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD1Ev")
}

// 0x4387ec — __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorD1Ev
pub fn stub_4387ec() -> ! {
    todo!("0x4387ec __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorD1Ev")
}

// 0x4387f0 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED1Ev
pub fn stub_4387f0() -> ! {
    todo!("0x4387f0 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")
}

// 0x4387f4 — __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEED0Ev
pub fn stub_4387f4() -> ! {
    todo!("0x4387f4 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::~EnumDesc()")
}

// 0x438894 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupEPKc
pub fn stub_438894() -> ! {
    todo!("0x438894 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(char const*)const")
}

// 0x4388c4 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE6lookupERKNS0_7VariantE
pub fn stub_4388c4() -> ! {
    todo!("0x4388c4 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4388e4 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_4388e4() -> ! {
    todo!("0x4388e4 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x438918 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel11CreatorTypeEE15convertToStringEmRSs
pub fn stub_438918() -> ! {
    todo!("0x438918 RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType>::convertToString(unsigned long,std::string &)const")
}

// 0x438a5c — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED1Ev
pub fn stub_438a5c() -> ! {
    todo!("0x438a5c RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")
}

// 0x438a60 — __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel5GenreEED0Ev
pub fn stub_438a60() -> ! {
    todo!("0x438a60 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::~EnumDesc()")
}

// 0x438b00 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupEPKc
pub fn stub_438b00() -> ! {
    todo!("0x438b00 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(char const*)const")
}

// 0x438b30 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE6lookupERKNS0_7VariantE
pub fn stub_438b30() -> ! {
    todo!("0x438b30 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x438b50 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToValueEmRNS0_7VariantE
pub fn stub_438b50() -> ! {
    todo!("0x438b50 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x438b84 — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE15convertToStringEmRSs
pub fn stub_438b84() -> ! {
    todo!("0x438b84 RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToString(unsigned long,std::string &)const")
}

// 0x438cc8 — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED1Ev
pub fn stub_438cc8() -> ! {
    todo!("0x438cc8 RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")
}

// 0x438ccc — __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEED0Ev
pub fn stub_438ccc() -> ! {
    todo!("0x438ccc RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::~EnumDesc()")
}

// 0x438d6c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupEPKc
pub fn stub_438d6c() -> ! {
    todo!("0x438d6c RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(char const*)const")
}

// 0x438d9c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE6lookupERKNS0_7VariantE
pub fn stub_438d9c() -> ! {
    todo!("0x438d9c RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::lookup(RBX::Reflection::Variant const&)const")
}
