// Auto-generated shard H — next 120 RBX::Reflection stubs — EA-sorted after 0x673cc0
// Source: ida/export.json filtered demangled contains RBX::Reflection (16171 total, 7293 stubbed -> 7413 total, showing next 120)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr
#![allow(unused_imports)]
use rbx_core::SharedPtr;

// 0x674340 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiTextButton,RBX::GuiTextButton>(boost::shared_ptr<RBX::GuiTextButton> const*,RBX::GuiTextButton *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiTextButton,RBX::GuiTextButton>(rbx_core::SharedPtr<RBX::GuiTextButton> const*,RBX::GuiTextButton *)const")]
pub fn stub_0x674340() {
    // IDA 0x674340: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0x674910 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x674910() -> ! {
    todo!("0x674910 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x674a1c — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()")]
pub fn stub_0x674a1c() {
    // IDA 0x674a1c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x674a48 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x674a48() -> ! {
    todo!("0x674a48 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")
}

// 0x674a4c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x674a4c() -> ! {
    todo!("0x674a4c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")
}

// 0x674a50 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x674a50() -> ! {
    todo!("0x674a50 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x674a74 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x674a74() -> ! {
    todo!("0x674a74 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x674b94 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x674b94() -> ! {
    todo!("0x674b94 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x674ca0 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_0x674ca0() {
    // IDA 0x674ca0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x674ccc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x674ccc() -> ! {
    todo!("0x674ccc RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")
}

// 0x674cd0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x674cd0() -> ! {
    todo!("0x674cd0 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")
}

// 0x674cd4 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x674cd4() -> ! {
    todo!("0x674cd4 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x674cfc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
pub fn stub_0x674cfc() -> ! {
    todo!("0x674cfc RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")
}

// 0x674e1c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x674e1c() -> ! {
    todo!("0x674e1c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x674fd0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
pub fn stub_0x674fd0() {
    // IDA 0x674fd0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x674ffc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isReadOnly(void)const")]
pub fn stub_0x674ffc() -> ! {
    todo!("0x674ffc RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isReadOnly(void)const")
}

// 0x67500c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isWriteOnly(void)const")]
pub fn stub_0x67500c() -> ! {
    todo!("0x67500c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isWriteOnly(void)const")
}

// 0x67501c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x67501c() -> ! {
    todo!("0x67501c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x675044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x675044() -> ! {
    todo!("0x675044 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x675068 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x675068() -> ! {
    todo!("0x675068 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x6751b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x6751b4() -> ! {
    todo!("0x6751b4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x6751d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::hasStringValue(void)const")]
pub fn stub_0x6751d8() -> ! {
    todo!("0x6751d8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::hasStringValue(void)const")
}

// 0x6751dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6751dc() -> ! {
    todo!("0x6751dc RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x675200 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x675200() -> ! {
    todo!("0x675200 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x675240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x675240() -> ! {
    todo!("0x675240 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x675260 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x675260() -> ! {
    todo!("0x675260 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x6754a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6754a0() -> ! {
    todo!("0x6754a0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6754bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x6754bc() -> ! {
    todo!("0x6754bc RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x6754f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6754f0() -> ! {
    todo!("0x6754f0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6754f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x6754f8() -> ! {
    todo!("0x6754f8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x675544 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675544() -> ! {
    todo!("0x675544 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x675564 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x675564() -> ! {
    todo!("0x675564 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x675598 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x675598() -> ! {
    todo!("0x675598 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x6755d8 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")]
pub fn stub_0x6755d8() -> ! {
    todo!("0x6755d8 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")
}

// 0x6755dc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")]
pub fn stub_0x6755dc() -> ! {
    todo!("0x6755dc RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")
}

// 0x6755e0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6755e0() -> ! {
    todo!("0x6755e0 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x67560c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")]
pub fn stub_0x67560c() -> ! {
    todo!("0x67560c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")
}

// 0x675630 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x675630() -> ! {
    todo!("0x675630 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6757e4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
pub fn stub_0x6757e4() {
    // IDA 0x6757e4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x675810 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isReadOnly(void)const")]
pub fn stub_0x675810() -> ! {
    todo!("0x675810 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isReadOnly(void)const")
}

// 0x675820 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isWriteOnly(void)const")]
pub fn stub_0x675820() -> ! {
    todo!("0x675820 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isWriteOnly(void)const")
}

// 0x675830 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675830() -> ! {
    todo!("0x675830 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x675858 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x675858() -> ! {
    todo!("0x675858 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x67587c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x67587c() -> ! {
    todo!("0x67587c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x6759c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x6759c8() -> ! {
    todo!("0x6759c8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x6759ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::hasStringValue(void)const")]
pub fn stub_0x6759ec() -> ! {
    todo!("0x6759ec RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::hasStringValue(void)const")
}

// 0x6759f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6759f0() -> ! {
    todo!("0x6759f0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x675a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x675a14() -> ! {
    todo!("0x675a14 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x675a54 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x675a54() -> ! {
    todo!("0x675a54 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x675a74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x675a74() -> ! {
    todo!("0x675a74 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x675cb4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675cb4() -> ! {
    todo!("0x675cb4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x675cd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x675cd0() -> ! {
    todo!("0x675cd0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x675d04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675d04() -> ! {
    todo!("0x675d04 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x675d0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x675d0c() -> ! {
    todo!("0x675d0c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x675d58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675d58() -> ! {
    todo!("0x675d58 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x675d78 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x675d78() -> ! {
    todo!("0x675d78 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x675dac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x675dac() -> ! {
    todo!("0x675dac RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x675dec — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const")]
pub fn stub_0x675dec() -> ! {
    todo!("0x675dec RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const")
}

// 0x675df0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const")]
pub fn stub_0x675df0() -> ! {
    todo!("0x675df0 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const")
}

// 0x675df4 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675df4() -> ! {
    todo!("0x675df4 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x675e20 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")]
pub fn stub_0x675e20() -> ! {
    todo!("0x675e20 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")
}

// 0x675e44 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x675e44() -> ! {
    todo!("0x675e44 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x675f58 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x675f58() -> ! {
    todo!("0x675f58 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isReadOnly(void)const")
}

// 0x675f5c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x675f5c() -> ! {
    todo!("0x675f5c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isWriteOnly(void)const")
}

// 0x675f60 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x675f60() -> ! {
    todo!("0x675f60 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x675f94 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x675f94() -> ! {
    todo!("0x675f94 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x675fb8 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x675fb8() -> ! {
    todo!("0x675fb8 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x6760cc — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::~PropDescriptor()")]
pub fn stub_0x6760cc() {
    // IDA 0x6760cc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x6760f8 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isReadOnly(void)const")]
pub fn stub_0x6760f8() -> ! {
    todo!("0x6760f8 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isReadOnly(void)const")
}

// 0x6760fc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isWriteOnly(void)const")]
pub fn stub_0x6760fc() -> ! {
    todo!("0x6760fc RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isWriteOnly(void)const")
}

// 0x676100 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676100() -> ! {
    todo!("0x676100 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x67612c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x67612c() -> ! {
    todo!("0x67612c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x676150 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x676150() -> ! {
    todo!("0x676150 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x676264 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::~PropDescriptor()")]
pub fn stub_0x676264() {
    // IDA 0x676264: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x676290 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isReadOnly(void)const")]
pub fn stub_0x676290() -> ! {
    todo!("0x676290 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isReadOnly(void)const")
}

// 0x676294 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isWriteOnly(void)const")]
pub fn stub_0x676294() -> ! {
    todo!("0x676294 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isWriteOnly(void)const")
}

// 0x676298 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676298() -> ! {
    todo!("0x676298 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6762d0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
pub fn stub_0x6762d0() -> ! {
    todo!("0x6762d0 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")
}

// 0x67630c — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x67630c() -> ! {
    todo!("0x67630c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x676420 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::~PropDescriptor()")]
pub fn stub_0x676420() {
    // IDA 0x676420: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x67644c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isReadOnly(void)const")]
pub fn stub_0x67644c() -> ! {
    todo!("0x67644c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isReadOnly(void)const")
}

// 0x676450 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
pub fn stub_0x676450() -> ! {
    todo!("0x676450 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isWriteOnly(void)const")
}

// 0x676454 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676454() -> ! {
    todo!("0x676454 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x67648c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
pub fn stub_0x67648c() -> ! {
    todo!("0x67648c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")
}

// 0x6764b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x6764b0() -> ! {
    todo!("0x6764b0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x676664 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::~EnumPropDescriptor()")]
pub fn stub_0x676664() {
    // IDA 0x676664: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x676690 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isReadOnly(void)const")]
pub fn stub_0x676690() -> ! {
    todo!("0x676690 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isReadOnly(void)const")
}

// 0x6766a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isWriteOnly(void)const")]
pub fn stub_0x6766a0() -> ! {
    todo!("0x6766a0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isWriteOnly(void)const")
}

// 0x6766b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x6766b0() -> ! {
    todo!("0x6766b0 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x6766d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x6766d8() -> ! {
    todo!("0x6766d8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x6766fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x6766fc() -> ! {
    todo!("0x6766fc RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x676848 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x676848() -> ! {
    todo!("0x676848 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x67686c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::hasStringValue(void)const")]
pub fn stub_0x67686c() -> ! {
    todo!("0x67686c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::hasStringValue(void)const")
}

// 0x676870 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676870() -> ! {
    todo!("0x676870 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x676894 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x676894() -> ! {
    todo!("0x676894 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x6768d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x6768d4() -> ! {
    todo!("0x6768d4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x6768f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x6768f4() -> ! {
    todo!("0x6768f4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x676b34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676b34() -> ! {
    todo!("0x676b34 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x676b50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x676b50() -> ! {
    todo!("0x676b50 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0x676b84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676b84() -> ! {
    todo!("0x676b84 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x676b8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x676b8c() -> ! {
    todo!("0x676b8c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x676bd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676bd8() -> ! {
    todo!("0x676bd8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0x676bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x676bf8() -> ! {
    todo!("0x676bf8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0x676c2c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x676c2c() -> ! {
    todo!("0x676c2c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0x676c6c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::isReadOnly(void)const")]
pub fn stub_0x676c6c() -> ! {
    todo!("0x676c6c RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::isReadOnly(void)const")
}

// 0x676c70 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::isWriteOnly(void)const")]
pub fn stub_0x676c70() -> ! {
    todo!("0x676c70 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::isWriteOnly(void)const")
}

// 0x676c74 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676c74() -> ! {
    todo!("0x676c74 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x676ca0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const")]
pub fn stub_0x676ca0() -> ! {
    todo!("0x676ca0 RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::Font const&)const")
}

// 0x676cc4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x676cc4() -> ! {
    todo!("0x676cc4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::EnumPropDescriptor<RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize)>(char const*,char const*,RBX::TextService::FontSize (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::FontSize),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x676e78 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
pub fn stub_0x676e78() {
    // IDA 0x676e78: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x676ea4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::isReadOnly(void)const")]
pub fn stub_0x676ea4() -> ! {
    todo!("0x676ea4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::isReadOnly(void)const")
}

// 0x676eb4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::isWriteOnly(void)const")]
pub fn stub_0x676eb4() -> ! {
    todo!("0x676eb4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::isWriteOnly(void)const")
}

// 0x676ec4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x676ec4() -> ! {
    todo!("0x676ec4 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x676eec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x676eec() -> ! {
    todo!("0x676eec RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x676f10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x676f10() -> ! {
    todo!("0x676f10 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x67705c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x67705c() -> ! {
    todo!("0x67705c RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x677080 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::hasStringValue(void)const")]
pub fn stub_0x677080() -> ! {
    todo!("0x677080 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::hasStringValue(void)const")
}

// 0x677084 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x677084() -> ! {
    todo!("0x677084 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x6770a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x6770a8() -> ! {
    todo!("0x6770a8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x6770e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x6770e8() -> ! {
    todo!("0x6770e8 RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}
