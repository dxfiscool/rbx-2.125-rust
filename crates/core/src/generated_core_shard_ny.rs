//! core shard ny — 120 core stubs EA-sorted asc gap filler not yet in core after nx (next uncovered).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 not yet in rbx_core (37397 uncovered before -> 37277 after, batch 0x4bb6b8..0x4c1750).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogPurpose&)const")]
// 0x4bb6b8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bb6b8() -> ! {
    todo!("0x4bb6b8 __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSingleton(void)")]
// 0x4bb75c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE13initSingletonEv
pub fn stub_0x4bb75c() -> ! {
    todo!("0x4bb75c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetSingleton(void)")]
// 0x4bb760 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv
pub fn stub_0x4bb760() -> ! {
    todo!("0x4bb760 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
// 0x4bb850 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED1Ev
pub fn stub_0x4bb850() -> ! {
    todo!("0x4bb850 __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
// 0x4bb854 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev
pub fn stub_0x4bb854() -> ! {
    todo!("0x4bb854 __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
// 0x4bba28 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED0Ev
pub fn stub_0x4bba28() -> ! {
    todo!("0x4bba28 __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*)const")]
// 0x4bbac8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupEPKc
pub fn stub_0x4bbac8() -> ! {
    todo!("0x4bbac8 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4bbaf8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupERKNS0_7VariantE
pub fn stub_0x4bbaf8() -> ! {
    todo!("0x4bbaf8 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4bbb18 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4bbb18() -> ! {
    todo!("0x4bbb18 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long,std::string &)const")]
// 0x4bbb74 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringEmRSs
pub fn stub_0x4bbb74() -> ! {
    todo!("0x4bbb74 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style const&)const")]
// 0x4bbcb8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_
pub fn stub_0x4bbcb8() -> ! {
    todo!("0x4bbcb8 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style const&)const")]
// 0x4bbf24 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_
pub fn stub_0x4bbf24() -> ! {
    todo!("0x4bbf24 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&,RBX::GuiButton::Style&)const")]
// 0x4bc0e0 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bc0e0() -> ! {
    todo!("0x4bc0e0 __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingleton(void)")]
// 0x4bc184 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE13initSingletonEv
pub fn stub_0x4bc184() -> ! {
    todo!("0x4bc184 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::doGetSingleton(void)")]
// 0x4bc188 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE14doGetSingletonEv
pub fn stub_0x4bc188() -> ! {
    todo!("0x4bc188 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()")]
// 0x4bc278 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED1Ev
pub fn stub_0x4bc278() -> ! {
    todo!("0x4bc278 __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()")]
// 0x4bc27c — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED2Ev
pub fn stub_0x4bc27c() -> ! {
    todo!("0x4bc27c __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()")]
// 0x4bc450 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED0Ev
pub fn stub_0x4bc450() -> ! {
    todo!("0x4bc450 __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(char const*)const")]
// 0x4bc4f0 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupEPKc
pub fn stub_0x4bc4f0() -> ! {
    todo!("0x4bc4f0 __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4bc520 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupERKNS0_7VariantE
pub fn stub_0x4bc520() -> ! {
    todo!("0x4bc520 __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4bc540 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4bc540() -> ! {
    todo!("0x4bc540 __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(unsigned long,std::string &)const")]
// 0x4bc59c — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringEmRSs
pub fn stub_0x4bc59c() -> ! {
    todo!("0x4bc59c __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(RBX::Frame::Style const&)const")]
// 0x4bc6e0 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringERKS3_
pub fn stub_0x4bc6e0() -> ! {
    todo!("0x4bc6e0 __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToItem(RBX::Frame::Style const&)const")]
// 0x4bc94c — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE13convertToItemERKS3_
pub fn stub_0x4bc94c() -> ! {
    todo!("0x4bc94c __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(RBX::Name const&,RBX::Frame::Style&)const")]
// 0x4bcb08 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bcb08() -> ! {
    todo!("0x4bcb08 __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::initSingleton(void)")]
// 0x4bcbac — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE13initSingletonEv
pub fn stub_0x4bcbac() -> ! {
    todo!("0x4bcbac __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)")]
// 0x4bcbb0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv
pub fn stub_0x4bcbb0() -> ! {
    todo!("0x4bcbb0 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// 0x4bcca0 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED1Ev
pub fn stub_0x4bcca0() -> ! {
    todo!("0x4bcca0 __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// 0x4bcca4 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev
pub fn stub_0x4bcca4() -> ! {
    todo!("0x4bcca4 __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// 0x4bce78 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED0Ev
pub fn stub_0x4bce78() -> ! {
    todo!("0x4bce78 __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::lookup(char const*)const")]
// 0x4bcf18 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE6lookupEPKc
pub fn stub_0x4bcf18() -> ! {
    todo!("0x4bcf18 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4bcf48 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE6lookupERKNS0_7VariantE
pub fn stub_0x4bcf48() -> ! {
    todo!("0x4bcf48 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4bcf68 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4bcf68() -> ! {
    todo!("0x4bcf68 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(unsigned long,std::string &)const")]
// 0x4bcfc4 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringEmRSs
pub fn stub_0x4bcfc4() -> ! {
    todo!("0x4bcfc4 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(RBX::GameBasicSettings::RenderQualitySetting const&)const")]
// 0x4bd108 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringERKS3_
pub fn stub_0x4bd108() -> ! {
    todo!("0x4bd108 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToItem(RBX::GameBasicSettings::RenderQualitySetting const&)const")]
// 0x4bd374 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE13convertToItemERKS3_
pub fn stub_0x4bd374() -> ! {
    todo!("0x4bd374 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(RBX::Name const&,RBX::GameBasicSettings::RenderQualitySetting&)const")]
// 0x4bd530 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bd530() -> ! {
    todo!("0x4bd530 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode> const>::initSingleton(void)")]
// 0x4bd5d4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings11ControlModeEEEE13initSingletonEv
pub fn stub_0x4bd5d4() -> ! {
    todo!("0x4bd5d4 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings11ControlModeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode> const>::doGetSingleton(void)")]
// 0x4bd5d8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings11ControlModeEEEE14doGetSingletonEv
pub fn stub_0x4bd5d8() -> ! {
    todo!("0x4bd5d8 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings11ControlModeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::~EnumDesc()")]
// 0x4bd6c8 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED1Ev
pub fn stub_0x4bd6c8() -> ! {
    todo!("0x4bd6c8 __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::~EnumDesc()")]
// 0x4bd6cc — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED2Ev
pub fn stub_0x4bd6cc() -> ! {
    todo!("0x4bd6cc __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::~EnumDesc()")]
// 0x4bd8a0 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED0Ev
pub fn stub_0x4bd8a0() -> ! {
    todo!("0x4bd8a0 __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::lookup(char const*)const")]
// 0x4bd940 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE6lookupEPKc
pub fn stub_0x4bd940() -> ! {
    todo!("0x4bd940 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4bd970 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE6lookupERKNS0_7VariantE
pub fn stub_0x4bd970() -> ! {
    todo!("0x4bd970 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4bd990 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4bd990() -> ! {
    todo!("0x4bd990 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::convertToString(unsigned long,std::string &)const")]
// 0x4bd9ec — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE15convertToStringEmRSs
pub fn stub_0x4bd9ec() -> ! {
    todo!("0x4bd9ec __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::convertToString(RBX::GameBasicSettings::ControlMode const&)const")]
// 0x4bdb30 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE15convertToStringERKS3_
pub fn stub_0x4bdb30() -> ! {
    todo!("0x4bdb30 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::convertToItem(RBX::GameBasicSettings::ControlMode const&)const")]
// 0x4bdd9c — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE13convertToItemERKS3_
pub fn stub_0x4bdd9c() -> ! {
    todo!("0x4bdd9c __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::ControlMode>::convertToValue(RBX::Name const&,RBX::GameBasicSettings::ControlMode&)const")]
// 0x4bdf58 — __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bdf58() -> ! {
    todo!("0x4bdf58 __ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting> const>::initSingleton(void)")]
// 0x4bdffc — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings13UploadSettingEEEE13initSingletonEv
pub fn stub_0x4bdffc() -> ! {
    todo!("0x4bdffc __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings13UploadSettingEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting> const>::doGetSingleton(void)")]
// 0x4be000 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings13UploadSettingEEEE14doGetSingletonEv
pub fn stub_0x4be000() -> ! {
    todo!("0x4be000 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings13UploadSettingEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::~EnumDesc()")]
// 0x4be0f0 — __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED1Ev
pub fn stub_0x4be0f0() -> ! {
    todo!("0x4be0f0 __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::~EnumDesc()")]
// 0x4be0f4 — __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED2Ev
pub fn stub_0x4be0f4() -> ! {
    todo!("0x4be0f4 __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::~EnumDesc()")]
// 0x4be2c8 — __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED0Ev
pub fn stub_0x4be2c8() -> ! {
    todo!("0x4be2c8 __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::lookup(char const*)const")]
// 0x4be368 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE6lookupEPKc
pub fn stub_0x4be368() -> ! {
    todo!("0x4be368 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4be398 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE6lookupERKNS0_7VariantE
pub fn stub_0x4be398() -> ! {
    todo!("0x4be398 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4be3b8 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4be3b8() -> ! {
    todo!("0x4be3b8 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::convertToString(unsigned long,std::string &)const")]
// 0x4be414 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE15convertToStringEmRSs
pub fn stub_0x4be414() -> ! {
    todo!("0x4be414 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::convertToString(RBX::GameSettings::UploadSetting const&)const")]
// 0x4be558 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE15convertToStringERKS3_
pub fn stub_0x4be558() -> ! {
    todo!("0x4be558 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::convertToItem(RBX::GameSettings::UploadSetting const&)const")]
// 0x4be7c4 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE13convertToItemERKS3_
pub fn stub_0x4be7c4() -> ! {
    todo!("0x4be7c4 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::UploadSetting>::convertToValue(RBX::Name const&,RBX::GameSettings::UploadSetting&)const")]
// 0x4be980 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4be980() -> ! {
    todo!("0x4be980 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality> const>::initSingleton(void)")]
// 0x4bea24 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings12VideoQualityEEEE13initSingletonEv
pub fn stub_0x4bea24() -> ! {
    todo!("0x4bea24 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings12VideoQualityEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality> const>::doGetSingleton(void)")]
// 0x4bea28 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings12VideoQualityEEEE14doGetSingletonEv
pub fn stub_0x4bea28() -> ! {
    todo!("0x4bea28 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings12VideoQualityEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::~EnumDesc()")]
// 0x4beb18 — __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED1Ev
pub fn stub_0x4beb18() -> ! {
    todo!("0x4beb18 __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::~EnumDesc()")]
// 0x4beb1c — __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED2Ev
pub fn stub_0x4beb1c() -> ! {
    todo!("0x4beb1c __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::~EnumDesc()")]
// 0x4becf0 — __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED0Ev
pub fn stub_0x4becf0() -> ! {
    todo!("0x4becf0 __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::lookup(char const*)const")]
// 0x4bed90 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE6lookupEPKc
pub fn stub_0x4bed90() -> ! {
    todo!("0x4bed90 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4bedc0 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE6lookupERKNS0_7VariantE
pub fn stub_0x4bedc0() -> ! {
    todo!("0x4bedc0 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4bede0 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4bede0() -> ! {
    todo!("0x4bede0 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::convertToString(unsigned long,std::string &)const")]
// 0x4bee3c — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE15convertToStringEmRSs
pub fn stub_0x4bee3c() -> ! {
    todo!("0x4bee3c __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::convertToString(RBX::GameSettings::VideoQuality const&)const")]
// 0x4bef80 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE15convertToStringERKS3_
pub fn stub_0x4bef80() -> ! {
    todo!("0x4bef80 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::convertToItem(RBX::GameSettings::VideoQuality const&)const")]
// 0x4bf1ec — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE13convertToItemERKS3_
pub fn stub_0x4bf1ec() -> ! {
    todo!("0x4bf1ec __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameSettings::VideoQuality>::convertToValue(RBX::Name const&,RBX::GameSettings::VideoQuality&)const")]
// 0x4bf3a8 — __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bf3a8() -> ! {
    todo!("0x4bf3a8 __ZNK3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart> const>::initSingleton(void)")]
// 0x4bf44c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE13initSingletonEv
pub fn stub_0x4bf44c() -> ! {
    todo!("0x4bf44c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart> const>::doGetSingleton(void)")]
// 0x4bf450 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv
pub fn stub_0x4bf450() -> ! {
    todo!("0x4bf450 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::~EnumDesc()")]
// 0x4bf540 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED1Ev
pub fn stub_0x4bf540() -> ! {
    todo!("0x4bf540 __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::~EnumDesc()")]
// 0x4bf544 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev
pub fn stub_0x4bf544() -> ! {
    todo!("0x4bf544 __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::~EnumDesc()")]
// 0x4bf718 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED0Ev
pub fn stub_0x4bf718() -> ! {
    todo!("0x4bf718 __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::lookup(char const*)const")]
// 0x4bf7b8 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE6lookupEPKc
pub fn stub_0x4bf7b8() -> ! {
    todo!("0x4bf7b8 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4bf7e8 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE6lookupERKNS0_7VariantE
pub fn stub_0x4bf7e8() -> ! {
    todo!("0x4bf7e8 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4bf808 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4bf808() -> ! {
    todo!("0x4bf808 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToString(unsigned long,std::string &)const")]
// 0x4bf864 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE15convertToStringEmRSs
pub fn stub_0x4bf864() -> ! {
    todo!("0x4bf864 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToString(RBX::CharacterMesh::BodyPart const&)const")]
// 0x4bf9a8 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE15convertToStringERKS3_
pub fn stub_0x4bf9a8() -> ! {
    todo!("0x4bf9a8 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToItem(RBX::CharacterMesh::BodyPart const&)const")]
// 0x4bfc14 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE13convertToItemERKS3_
pub fn stub_0x4bfc14() -> ! {
    todo!("0x4bfc14 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(RBX::Name const&,RBX::CharacterMesh::BodyPart&)const")]
// 0x4bfdd0 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4bfdd0() -> ! {
    todo!("0x4bfdd0 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType> const>::initSingleton(void)")]
// 0x4bfe74 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18MarketplaceService12CurrencyTypeEEEE13initSingletonEv
pub fn stub_0x4bfe74() -> ! {
    todo!("0x4bfe74 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18MarketplaceService12CurrencyTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::~EnumDesc()")]
// 0x4bfe78 — __ZN3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEED1Ev
pub fn stub_0x4bfe78() -> ! {
    todo!("0x4bfe78 __ZN3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToString(unsigned long,std::string &)const")]
// 0x4bfe7c — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE15convertToStringEmRSs
pub fn stub_0x4bfe7c() -> ! {
    todo!("0x4bfe7c __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToItem(RBX::MarketplaceService::CurrencyType const&)const")]
// 0x4bffcc — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE13convertToItemERKS3_
pub fn stub_0x4bffcc() -> ! {
    todo!("0x4bffcc __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToValue(RBX::Name const&,RBX::MarketplaceService::CurrencyType&)const")]
// 0x4c0098 — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c0098() -> ! {
    todo!("0x4c0098 __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::initSingleton(void)")]
// 0x4c0114 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE13initSingletonEv
pub fn stub_0x4c0114() -> ! {
    todo!("0x4c0114 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::doGetSingleton(void)")]
// 0x4c0118 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE14doGetSingletonEv
pub fn stub_0x4c0118() -> ! {
    todo!("0x4c0118 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// 0x4c0208 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED1Ev
pub fn stub_0x4c0208() -> ! {
    todo!("0x4c0208 __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// 0x4c020c — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED2Ev
pub fn stub_0x4c020c() -> ! {
    todo!("0x4c020c __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// 0x4c03e0 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED0Ev
pub fn stub_0x4c03e0() -> ! {
    todo!("0x4c03e0 __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(char const*)const")]
// 0x4c0480 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupEPKc
pub fn stub_0x4c0480() -> ! {
    todo!("0x4c0480 __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c04b0 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupERKNS0_7VariantE
pub fn stub_0x4c04b0() -> ! {
    todo!("0x4c04b0 __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c04d0 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c04d0() -> ! {
    todo!("0x4c04d0 __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(unsigned long,std::string &)const")]
// 0x4c052c — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringEmRSs
pub fn stub_0x4c052c() -> ! {
    todo!("0x4c052c __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(RBX::ChatService::ChatColor const&)const")]
// 0x4c0670 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringERKS3_
pub fn stub_0x4c0670() -> ! {
    todo!("0x4c0670 __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToItem(RBX::ChatService::ChatColor const&)const")]
// 0x4c08dc — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE13convertToItemERKS3_
pub fn stub_0x4c08dc() -> ! {
    todo!("0x4c08dc __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(RBX::Name const&,RBX::ChatService::ChatColor&)const")]
// 0x4c0a98 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueERKNS_4NameERS3_
pub fn stub_0x4c0a98() -> ! {
    todo!("0x4c0a98 __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueERKNS_4NameERS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::initSingleton(void)")]
// 0x4c0b3c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE13initSingletonEv
pub fn stub_0x4c0b3c() -> ! {
    todo!("0x4c0b3c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::doGetSingleton(void)")]
// 0x4c0b40 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE14doGetSingletonEv
pub fn stub_0x4c0b40() -> ! {
    todo!("0x4c0b40 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::initSingleton(void)")]
// 0x4c0c30 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE13initSingletonEv
pub fn stub_0x4c0c30() -> ! {
    todo!("0x4c0c30 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::doGetSingleton(void)")]
// 0x4c0c34 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE14doGetSingletonEv
pub fn stub_0x4c0c34() -> ! {
    todo!("0x4c0c34 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::initSingleton(void)")]
// 0x4c0d24 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE13initSingletonEv
pub fn stub_0x4c0d24() -> ! {
    todo!("0x4c0d24 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::doGetSingleton(void)")]
// 0x4c0d28 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE14doGetSingletonEv
pub fn stub_0x4c0d28() -> ! {
    todo!("0x4c0d28 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// 0x4c0e18 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED1Ev
pub fn stub_0x4c0e18() -> ! {
    todo!("0x4c0e18 __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// 0x4c0e1c — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED2Ev
pub fn stub_0x4c0e1c() -> ! {
    todo!("0x4c0e1c __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED2Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// 0x4c0ff0 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED0Ev
pub fn stub_0x4c0ff0() -> ! {
    todo!("0x4c0ff0 __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(char const*)const")]
// 0x4c1090 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupEPKc
pub fn stub_0x4c1090() -> ! {
    todo!("0x4c1090 __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupEPKc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(RBX::Reflection::Variant const&)const")]
// 0x4c10c0 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupERKNS0_7VariantE
pub fn stub_0x4c10c0() -> ! {
    todo!("0x4c10c0 __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// 0x4c10e0 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_0x4c10e0() -> ! {
    todo!("0x4c10e0 __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueEmRNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(unsigned long,std::string &)const")]
// 0x4c113c — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringEmRSs
pub fn stub_0x4c113c() -> ! {
    todo!("0x4c113c __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(RBX::SurfaceType const&)const")]
// 0x4c1280 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringERKS2_
pub fn stub_0x4c1280() -> ! {
    todo!("0x4c1280 __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringERKS2_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToItem(RBX::SurfaceType const&)const")]
// 0x4c14ec — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE13convertToItemERKS2_
pub fn stub_0x4c14ec() -> ! {
    todo!("0x4c14ec __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE13convertToItemERKS2_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(RBX::Name const&,RBX::SurfaceType&)const")]
// 0x4c16a8 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueERKNS_4NameERS2_
pub fn stub_0x4c16a8() -> ! {
    todo!("0x4c16a8 __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueERKNS_4NameERS2_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::initSingleton(void)")]
// 0x4c174c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE13initSingletonEv
pub fn stub_0x4c174c() -> ! {
    todo!("0x4c174c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::doGetSingleton(void)")]
// 0x4c1750 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv
pub fn stub_0x4c1750() -> ! {
    todo!("0x4c1750 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv")
}
