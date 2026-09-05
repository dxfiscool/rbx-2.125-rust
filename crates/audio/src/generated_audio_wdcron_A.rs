//! audio generated_audio_wdcron_A — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Soundscape exhausted, global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x62c19c
//! Range 0x674a50..0x676c6c | existing 37323 -> 37423 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

use rbx_core::SharedPtr;
use crate::generated::flog_asserts;
use crate::generated_134::{XmlIntSlot, XmlReadValue};
use crate::generated_audio_wd_watchdog18::{
    GuiButtonBoolProp, GuiButtonBoolSlot, GuiButtonColorProp, GuiButtonColorSlot, GuiButtonFloatProp,
    GuiButtonFloatSlot, GuiButtonXAlignProp, GuiButtonYAlignProp, GuiTextButtonState, XAlignmentVariant,
    YAlignmentVariant, XALIGNMENT_ITEMS, YALIGNMENT_ITEMS, xalignment_index, xalignment_name,
    yalignment_index, yalignment_name,
};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


// 0x674a50 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_674a50(state: &GuiTextButtonState, slot: GuiButtonBoolSlot) -> bool {
    // IDA 0x674a50 (`PropDescriptor<GuiTextButton,
    // bool>::GetImpl::getValue`): dispatches the stored getter
    // member-pointer over the object (0x674a52-0x674a72, host: the
    // `slot` selects the `GuiTextButtonState` bool). Same shape as
    // the `TextBox` twin at 0x66c2d4.
    state.bool_slot(slot)
}

// 0x674a74 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_674a74() -> ! {
    // IDA 0x674a74 (`PropDescriptor<GuiTextButton,
    // bool>::GetImpl::setValue`): `__noreturn`, unconditionally
    // throws `std::runtime_error("can't set value")` — the impl is
    // getter-only. Host: panic.
    panic!("can't set value")
}

// 0x674b94 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,G3D::Vector2 (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EEC2IMS2_KFS4_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_674b94() {
    // IDA 0x674b94 (`PropDescriptor<GuiTextButton,
    // Vector2>::PropDescriptor`): same generic shape as the `TextBox`
    // Vector2 C2 at 0x66c418 (member pair + typed identity). No
    // `Vector2`-returning `GuiTextButton` member is identified in
    // this range, so only the registry half exists: carrier no-op.
}

// 0x674ca0 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EED0Ev")]
pub fn stub_674ca0() {
    // IDA 0x674ca0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674ccc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE10isReadOnlyEv")]
pub fn stub_674ccc() -> bool {
    // IDA 0x674ccc (`PropDescriptor<GuiTextButton,
    // Vector2>::GetImpl::isReadOnly`): returns constant 1 — the impl
    // throws in `setValue`. Same shape as the `TextBox` twin at
    // 0x66c638.
    true
}

// 0x674cd0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE11isWriteOnlyEv")]
pub fn stub_674cd0() -> bool {
    // IDA 0x674cd0 (`PropDescriptor<GuiTextButton,
    // Vector2>::GetImpl::isWriteOnly`): returns constant 0. Same
    // shape as the `TextBox` twin at 0x66c63c.
    false
}

// 0x674cd4 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_674cd4(value: [f32; 2]) -> [f32; 2] {
    // IDA 0x674cd4 (`PropDescriptor<GuiTextButton,
    // Vector2>::GetImpl::getValue`): dispatches the stored getter
    // member-pointer. No `Vector2`-returning `GuiTextButton` member
    // is identified in this range (same gap as 0x674b94): host
    // passes the read edge through.
    value
}

// 0x674cfc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Vector2>::GetImpl<G3D::Vector2 (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D7Vector2EE7GetImplIMS2_KFS4_vEE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_674cfc() -> ! {
    // IDA 0x674cfc (`PropDescriptor<GuiTextButton,
    // Vector2>::GetImpl::setValue`): `__noreturn`, unconditionally
    // throws — the impl is getter-only. Host: panic.
    panic!("can't set value")
}

// 0x674e1c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_674e1c(name: &str, category: &str, attributes: u32, permissions: u32) -> GuiButtonYAlignProp {
    // IDA 0x674e1c (`EnumPropDescriptor<GuiTextButton, YAlignment>`
    // ctor): the `GuiTextButton` `classDescriptor` call, the
    // `EnumDesc<YAlignment>` singleton once-init and the
    // `PropertyDescriptor` base init with name/category/
    // attributes/permissions plus the impl holding the
    // getter/setter member-pointer pair. The pair folds into the
    // `y_alignment` field. Same shape as the `TextBox` twin at
    // 0x66c788.
    GuiButtonYAlignProp::new(name, category, attributes, permissions)
}

// 0x674fd0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED0Ev")]
pub fn stub_674fd0() {
    // IDA 0x674fd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x674ffc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10isReadOnlyEv")]
pub fn stub_674ffc() -> bool {
    // IDA 0x674ffc (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::isReadOnly`): delegates to the inner `GetSet` at
    // +44 — always readable.
    false
}

// 0x67500c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11isWriteOnlyEv")]
pub fn stub_67500c() -> bool {
    // IDA 0x67500c (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::isWriteOnly`): delegates to the inner `GetSet`
    // at +44 — always writable.
    false
}

// 0x67501c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_67501c(first: &GuiTextButtonState, second: &GuiTextButtonState) -> bool {
    // IDA 0x67501c (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::equalValues`): reads the inner value for both
    // instances via the +44 `GetSet` and compares. Host: compare
    // the alignments.
    first.y_alignment == second.y_alignment
}

// 0x675044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_675044(state: &GuiTextButtonState) -> YAlignmentVariant {
    // IDA 0x675044 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::getVariant`): reads the inner value, tags it
    // with the plain-`int` singleton and placement-moves it in.
    // Host: the `YAlignment` tag.
    YAlignmentVariant::YAlignment(state.y_alignment)
}

// 0x675068 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_675068(state: &mut GuiTextButtonState, variant: &YAlignmentVariant) {
    // IDA 0x675068 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::setVariant`): an int-typed variant runs
    // `any_cast<int>`; anything else runs `Variant::convert<int>`
    // (throws on failure); then the +72 setter. Host:
    // convert-or-throw, then store.
    let value = match *variant {
        YAlignmentVariant::YAlignment(value) => value,
        _ => panic!("Unable to convert variant to int (IDA 0x675068)"),
    };
    state.y_alignment = value;
}

// 0x6751b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_6751b4(first: &GuiTextButtonState, second: &mut GuiTextButtonState) {
    // IDA 0x6751b4 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::copyValue`): inner `getValue` on the source then
    // inner `setValue` on the target. Host: copy the alignment.
    second.y_alignment = first.y_alignment;
}

// 0x6751d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14hasStringValueEv")]
pub fn stub_6751d8() -> bool {
    // IDA 0x6751d8 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::hasStringValue`): returns 1 — always stringable.
    true
}

// 0x6751dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_6751dc(state: &GuiTextButtonState) -> String {
    // IDA 0x6751dc (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::getStringValue`): inner value via the +44
    // `GetSet` plus `EnumDesc::convertToString`. Host: the grounded
    // item name.
    yalignment_name(state.y_alignment).to_owned()
}

// 0x675200 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_675200(state: &mut GuiTextButtonState, name: &str) -> bool {
    // IDA 0x675200 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::setStringValue`): `Name::lookup` +
    // `EnumDesc::convertToValue`; on a hit the inner `setValue`
    // runs and 1 returns, else 0. Host: table position decides.
    match YALIGNMENT_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.y_alignment = YALIGNMENT_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x675240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_675240(state: &GuiTextButtonState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x675240 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::writeValue`): inner `getValue`, `clearValue`,
    // int tag `5` at +16, value at +20, returns 5.
    out.value_type = 5;
    out.int_value = state.y_alignment as i32;
    5
}

// 0x675260 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_675260(state: &mut GuiTextButtonState, xml: &XmlReadValue) {
    // IDA 0x675260 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::readValue`): xsi:nil early-out; an int pair runs
    // `setIntValue` (index→value with -1 rejection) and returns on
    // success; a string pair runs lookup + convert + inner set, a
    // miss running the +64 reset hook before asserting (folds away);
    // anything else hits `ReleaseAssert(false)`
    // (Reflection.h:359, host seam). The `enumToItem` map is dense
    // identity, so the int path reads the table.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if *value >= 0 {
                if let Some((_, align)) = YALIGNMENT_ITEMS.get(*value as usize) {
                    state.y_alignment = *align;
                    return;
                }
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x675260)");
            }
        }
        XmlReadValue::Text(text) => {
            if stub_675200(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x675260)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x675260)");
            }
        }
    }
}

// 0x6754a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_6754a0(state: &GuiTextButtonState) -> i32 {
    // IDA 0x6754a0 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::getIndexValue`): inner `getValue` +
    // `EnumDesc::convertToIndex`. Host: the item index of the live
    // value.
    yalignment_index(state.y_alignment)
}

// 0x6754bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_6754bc(state: &mut GuiTextButtonState, index: u32) -> bool {
    // IDA 0x6754bc (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::setIndexValue`): bounds-checks the index against
    // the item count, stores `items[index]` through the inner
    // `setValue` and returns 1, else 0.
    match YALIGNMENT_ITEMS.get(index as usize) {
        Some((_, align)) => {
            state.y_alignment = *align;
            true
        }
        None => false,
    }
}

// 0x6754f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_6754f0(state: &GuiTextButtonState) -> u32 {
    // IDA 0x6754f0 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::getEnumValue`): the inner `getValue` straight
    // through.
    state.y_alignment
}

// 0x6754f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_6754f8(state: &mut GuiTextButtonState, value: u32) -> bool {
    // IDA 0x6754f8 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::setEnumValue`): `find_if` with `equalValue` over
    // the items; on a hit the inner `setValue` runs and 1 returns,
    // else 0. Host: membership decides.
    if YALIGNMENT_ITEMS.iter().any(|(_, v)| *v == value) {
        state.y_alignment = value;
        true
    } else {
        false
    }
}

// 0x675544 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_675544(state: &GuiTextButtonState) -> i32 {
    // IDA 0x675544 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::getEnumItem`): inner `getValue` +
    // `EnumDesc::convertToItem`. Host: the item position of the
    // live value (-1 when missing).
    yalignment_index(state.y_alignment)
}

// 0x675564 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_675564(state: &mut GuiTextButtonState, name: &str) -> bool {
    // IDA 0x675564 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::setStringValue` over `Name`): `convertToValue`
    // on the name; on a hit the inner `setValue` runs and 1
    // returns, else 0. Same string edge as 0x675200 — host forwards
    // into that twin (`Name` folds into `&str`).
    stub_675200(state, name)
}

// 0x675598 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_675598(state: &mut GuiTextButtonState, index: i32) -> bool {
    // IDA 0x675598 (`EnumPropDescriptor<GuiTextButton,
    // YAlignment>::setIntValue`): rejects negative indices,
    // bounds-checks against the item count and rejects `-1`-valued
    // items, then stores through the inner `setValue` and returns 1,
    // else 0. Table values are non-negative by type, so the `-1`
    // check folds away.
    if index >= 0 {
        if let Some((_, align)) = YALIGNMENT_ITEMS.get(index as usize) {
            state.y_alignment = *align;
            return true;
        }
    }
    false
}

// 0x6755d8 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_6755d8() -> bool {
    // IDA 0x6755d8 (`GetSetImpl<GuiTextButton YAlignment>::isReadOnly`):
    // returns constant 0.
    false
}

// 0x6755dc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_6755dc() -> bool {
    // IDA 0x6755dc (`GetSetImpl<GuiTextButton YAlignment>::isWriteOnly`):
    // returns constant 0.
    false
}

// 0x6755e0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_6755e0(state: &GuiTextButtonState) -> u32 {
    // IDA 0x6755e0 (`GetSetImpl<GuiTextButton YAlignment>::getValue`):
    // the member-pointer resolve (null described reads at offset 0
    // with the +800 `Instance`-to-mixin adjust, 0x6755e8-0x6755f0;
    // virtual when the low bit is set) tail-calling the getter. The
    // member is `getYAlignment`; the pointer folds into the field.
    state.y_alignment
}

// 0x67560c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_67560c(state: &mut GuiTextButtonState, value: u32) {
    // IDA 0x67560c (`GetSetImpl<GuiTextButton YAlignment>::setValue`):
    // the member-pointer resolve over +12/+16 tail-calling the
    // setter with the input word. The member is `setYAlignment`;
    // the pointer folds into the field (its raises fold into the
    // store).
    state.y_alignment = value;
}

// 0x675630 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_675630(name: &str, category: &str, attributes: u32, permissions: u32) -> GuiButtonXAlignProp {
    // IDA 0x675630 (`EnumPropDescriptor<GuiTextButton, XAlignment>`
    // ctor): the `GuiTextButton` `classDescriptor` call, the
    // `EnumDesc<XAlignment>` singleton once-init and the
    // `PropertyDescriptor` base init with name/category/
    // attributes/permissions plus the impl holding the
    // getter/setter member-pointer pair. The pair folds into the
    // `x_alignment` field. Same shape as the `TextBox` twin at
    // 0x66da0c.
    GuiButtonXAlignProp::new(name, category, attributes, permissions)
}

// 0x6757e4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED0Ev")]
pub fn stub_6757e4() {
    // IDA 0x6757e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x675810 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10isReadOnlyEv")]
pub fn stub_675810() -> bool {
    // IDA 0x675810 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::isReadOnly`): delegates to the inner `GetSet` at
    // +44 — always readable.
    false
}

// 0x675820 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11isWriteOnlyEv")]
pub fn stub_675820() -> bool {
    // IDA 0x675820 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::isWriteOnly`): delegates to the inner `GetSet`
    // at +44 — always writable.
    false
}

// 0x675830 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_675830(first: &GuiTextButtonState, second: &GuiTextButtonState) -> bool {
    // IDA 0x675830 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::equalValues`): reads the inner value for both
    // instances via the +44 `GetSet` and compares. Host: compare
    // the alignments.
    first.x_alignment == second.x_alignment
}

// 0x675858 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_675858(state: &GuiTextButtonState) -> XAlignmentVariant {
    // IDA 0x675858 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::getVariant`): reads the inner value, tags it
    // with the plain-`int` singleton and placement-moves it in.
    // Host: the `XAlignment` tag.
    XAlignmentVariant::XAlignment(state.x_alignment)
}

// 0x67587c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_67587c(state: &mut GuiTextButtonState, variant: &XAlignmentVariant) {
    // IDA 0x67587c (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::setVariant`): an int-typed variant runs
    // `any_cast<int>`; anything else runs `Variant::convert<int>`
    // (throws on failure); then the +72 setter. Host:
    // convert-or-throw, then store.
    let value = match *variant {
        XAlignmentVariant::XAlignment(value) => value,
        _ => panic!("Unable to convert variant to int (IDA 0x67587c)"),
    };
    state.x_alignment = value;
}

// 0x6759c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_6759c8(first: &GuiTextButtonState, second: &mut GuiTextButtonState) {
    // IDA 0x6759c8 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::copyValue`): inner `getValue` on the source then
    // inner `setValue` on the target. Host: copy the alignment.
    second.x_alignment = first.x_alignment;
}

// 0x6759ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14hasStringValueEv")]
pub fn stub_6759ec() -> bool {
    // IDA 0x6759ec (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::hasStringValue`): returns 1 — always stringable.
    true
}

// 0x6759f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_6759f0(state: &GuiTextButtonState) -> String {
    // IDA 0x6759f0 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::getStringValue`): inner value via the +44
    // `GetSet` plus `EnumDesc::convertToString`. Host: the grounded
    // item name.
    xalignment_name(state.x_alignment).to_owned()
}

// 0x675a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_675a14(state: &mut GuiTextButtonState, name: &str) -> bool {
    // IDA 0x675a14 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::setStringValue`): `Name::lookup` +
    // `EnumDesc::convertToValue`; on a hit the inner `setValue`
    // runs and 1 returns, else 0. Host: table position decides.
    match XALIGNMENT_ITEMS.iter().position(|(n, _)| *n == name) {
        Some(index) => {
            state.x_alignment = XALIGNMENT_ITEMS[index].1;
            true
        }
        None => false,
    }
}

// 0x675a54 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_675a54(state: &GuiTextButtonState, out: &mut XmlIntSlot) -> i32 {
    // IDA 0x675a54 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::writeValue`): inner `getValue`, `clearValue`,
    // int tag `5` at +16, value at +20, returns 5.
    out.value_type = 5;
    out.int_value = state.x_alignment as i32;
    5
}

// 0x675a74 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_675a74(state: &mut GuiTextButtonState, xml: &XmlReadValue) {
    // IDA 0x675a74 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::readValue`): xsi:nil early-out; an int pair runs
    // `setIntValue` (index→value with -1 rejection) and returns on
    // success; a string pair runs lookup + convert + inner set, a
    // miss running the +64 reset hook before asserting (folds away);
    // anything else hits `ReleaseAssert(false)`
    // (Reflection.h:359, host seam). The `enumToItem` map is dense
    // identity, so the int path reads the table.
    match xml {
        XmlReadValue::Nil => {}
        XmlReadValue::Int(value) => {
            if *value >= 0 {
                if let Some((_, align)) = XALIGNMENT_ITEMS.get(*value as usize) {
                    state.x_alignment = *align;
                    return;
                }
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x675a74)");
            }
        }
        XmlReadValue::Text(text) => {
            if stub_675a14(state, text) {
                return;
            }
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x675a74)");
            }
        }
        XmlReadValue::Other => {
            if flog_asserts() {
                panic!("false file: include/Reflection/Reflection.h line: 359 (IDA 0x675a74)");
            }
        }
    }
}

// 0x675cb4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_675cb4(state: &GuiTextButtonState) -> i32 {
    // IDA 0x675cb4 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::getIndexValue`): inner `getValue` +
    // `EnumDesc::convertToIndex`. Host: the item index of the live
    // value.
    xalignment_index(state.x_alignment)
}

// 0x675cd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_675cd0(state: &mut GuiTextButtonState, index: u32) -> bool {
    // IDA 0x675cd0 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::setIndexValue`): bounds-checks the index against
    // the item count, stores `items[index]` through the inner
    // `setValue` and returns 1, else 0.
    match XALIGNMENT_ITEMS.get(index as usize) {
        Some((_, align)) => {
            state.x_alignment = *align;
            true
        }
        None => false,
    }
}

// 0x675d04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_675d04(state: &GuiTextButtonState) -> u32 {
    // IDA 0x675d04 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::getEnumValue`): the inner `getValue` straight
    // through.
    state.x_alignment
}

// 0x675d0c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_675d0c(state: &mut GuiTextButtonState, value: u32) -> bool {
    // IDA 0x675d0c (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::setEnumValue`): `find_if` with `equalValue` over
    // the items; on a hit the inner `setValue` runs and 1 returns,
    // else 0. Host: membership decides.
    if XALIGNMENT_ITEMS.iter().any(|(_, v)| *v == value) {
        state.x_alignment = value;
        true
    } else {
        false
    }
}

// 0x675d58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_675d58(state: &GuiTextButtonState) -> i32 {
    // IDA 0x675d58 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::getEnumItem`): inner `getValue` +
    // `EnumDesc::convertToItem`. Host: the item position of the
    // live value (-1 when missing).
    xalignment_index(state.x_alignment)
}

// 0x675d78 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_675d78(state: &mut GuiTextButtonState, name: &str) -> bool {
    // IDA 0x675d78 (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::setStringValue` over `Name`): `convertToValue`
    // on the name; on a hit the inner `setValue` runs and 1
    // returns, else 0. Same string edge as 0x675a14 — host forwards
    // into that twin (`Name` folds into `&str`).
    stub_675a14(state, name)
}

// 0x675dac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_675dac(state: &mut GuiTextButtonState, index: i32) -> bool {
    // IDA 0x675dac (`EnumPropDescriptor<GuiTextButton,
    // XAlignment>::setIntValue`): rejects negative indices,
    // bounds-checks against the item count and rejects `-1`-valued
    // items, then stores through the inner `setValue` and returns 1,
    // else 0. Table values are non-negative by type, so the `-1`
    // check folds away.
    if index >= 0 {
        if let Some((_, align)) = XALIGNMENT_ITEMS.get(index as usize) {
            state.x_alignment = *align;
            return true;
        }
    }
    false
}

// 0x675dec — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_675dec() -> bool {
    // IDA 0x675dec (`GetSetImpl<GuiTextButton XAlignment>::isReadOnly`):
    // returns constant 0.
    false
}

// 0x675df0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_675df0() -> bool {
    // IDA 0x675df0 (`GetSetImpl<GuiTextButton XAlignment>::isWriteOnly`):
    // returns constant 0.
    false
}

// 0x675df4 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_675df4(state: &GuiTextButtonState) -> u32 {
    // IDA 0x675df4 (`GetSetImpl<GuiTextButton XAlignment>::getValue`):
    // the member-pointer resolve (null described reads at offset 0
    // with the +800 `Instance`-to-mixin adjust; virtual when the low
    // bit is set) tail-calling the getter. The member is
    // `getXAlignment`; the pointer folds into the field.
    state.x_alignment
}

// 0x675e20 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::GetSetImpl<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::XAlignment const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_675e20(state: &mut GuiTextButtonState, value: u32) {
    // IDA 0x675e20 (`GetSetImpl<GuiTextButton XAlignment>::setValue`):
    // the member-pointer resolve over +12/+16 tail-calling the
    // setter with the input word. The member is `setXAlignment`;
    // the pointer folds into the field (its raises fold into the
    // store).
    state.x_alignment = value;
}

// 0x675e44 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>(char const*,char const*,bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMNS_12GuiTextMixinEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_675e44(name: &str, category: &str, attributes: u32, permissions: u32) -> GuiButtonBoolProp {
    // IDA 0x675e44 (`PropDescriptor<GuiTextButton, bool>::PropDescriptor`
    // with get+set pair over the mixin getter): builds the
    // `GetSetImpl` member-pair cell plus the typed descriptor
    // identity with name/category/attributes/permissions. The pair
    // folds into the caller's `GuiButtonBoolSlot`. Host: the
    // identity half (same shape as the `TextBox` twin at 0x6720cc).
    GuiButtonBoolProp::new(name, category, attributes, permissions)
}

// 0x675f58 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_675f58() -> bool {
    // IDA 0x675f58 (`GetSetImpl<GuiTextButton bool>::isReadOnly`):
    // returns constant 0.
    false
}

// 0x675f5c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_675f5c() -> bool {
    // IDA 0x675f5c (`GetSetImpl<GuiTextButton bool>::isWriteOnly`):
    // returns constant 0.
    false
}

// 0x675f60 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_675f60(state: &GuiTextButtonState, slot: GuiButtonBoolSlot) -> bool {
    // IDA 0x675f60 (`GetSetImpl<GuiTextButton bool>::getValue`):
    // dispatches the stored mixin getter member-pointer over the
    // object (host: the `slot` selects the `GuiTextButtonState`
    // bool).
    state.bool_slot(slot)
}

// 0x675f94 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetSetImpl<bool (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE10GetSetImplIMNS_12GuiTextMixinEKFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_675f94(state: &mut GuiTextButtonState, slot: GuiButtonBoolSlot, value: bool) -> bool {
    // IDA 0x675f94 (`GetSetImpl<GuiTextButton bool>::setValue`):
    // dispatches the stored setter member-pointer with the input
    // word (host: the `slot` selects the member setter twin —
    // `setTextWrap`/`setTextScale` at 0x673330/0x673370, whose
    // raises fold into the changed flag).
    match slot {
        GuiButtonBoolSlot::TextWrap => crate::generated_audio_wd_watchdog_Y::stub_673330(state, value),
        GuiButtonBoolSlot::TextScaled => crate::generated_audio_wd_watchdog_Y::stub_673370(state, value),
    }
}

// 0x675fb8 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::PropDescriptor<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>(char const*,char const*,float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfEC2IMNS_12GuiTextMixinEKFfvEMS2_FvfEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_675fb8(name: &str, category: &str, attributes: u32, permissions: u32) -> GuiButtonFloatProp {
    // IDA 0x675fb8 (`PropDescriptor<GuiTextButton,
    // float>::PropDescriptor`): builds the `GetSetImpl` member-pair
    // cell plus the typed descriptor identity with name/category/
    // attributes/permissions. The pair folds into the caller's
    // `GuiButtonFloatSlot`. Host: the identity half.
    GuiButtonFloatProp::new(name, category, attributes, permissions)
}

// 0x6760cc — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED0Ev")]
pub fn stub_6760cc() {
    // IDA 0x6760cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6760f8 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE10isReadOnlyEv")]
pub fn stub_6760f8() -> bool {
    // IDA 0x6760f8 (`GetSetImpl<GuiTextButton float>::isReadOnly`):
    // returns constant 0.
    false
}

// 0x6760fc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE11isWriteOnlyEv")]
pub fn stub_6760fc() -> bool {
    // IDA 0x6760fc (`GetSetImpl<GuiTextButton float>::isWriteOnly`):
    // returns constant 0.
    false
}

// 0x676100 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_676100(state: &GuiTextButtonState, slot: GuiButtonFloatSlot) -> f32 {
    // IDA 0x676100 (`GetSetImpl<GuiTextButton float>::getValue`):
    // dispatches the stored mixin getter member-pointer over the
    // object (host: the `slot` selects the `GuiTextButtonState`
    // float).
    state.float_slot(slot)
}

// 0x67612c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::GetSetImpl<float (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfE10GetSetImplIMNS_12GuiTextMixinEKFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_67612c(state: &mut GuiTextButtonState, slot: GuiButtonFloatSlot, value: f32) -> bool {
    // IDA 0x67612c (`GetSetImpl<GuiTextButton float>::setValue`):
    // dispatches the stored setter member-pointer with the input
    // word (host: the `slot` selects the member setter twin —
    // `setTextTransparency`/`setTextStrokeTransparency` at
    // 0x673308/0x6737e8, whose raises fold into the changed flag).
    match slot {
        GuiButtonFloatSlot::TextTransparency => crate::generated_audio_wd_watchdog_Y::stub_673308(state, value),
        GuiButtonFloatSlot::TextStrokeTransparency => crate::generated_audio_wd_watchdog_Y::stub_6737e8(state, value),
    }
}

// 0x676150 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_676150(name: &str, category: &str, attributes: u32, permissions: u32) -> GuiButtonColorProp {
    // IDA 0x676150 (`PropDescriptor<GuiTextButton,
    // Color3>::PropDescriptor`): builds the `GetSetImpl` member-pair
    // cell plus the typed descriptor identity with name/category/
    // attributes/permissions. The pair folds into the caller's
    // `GuiButtonColorSlot`. Host: the identity half.
    GuiButtonColorProp::new(name, category, attributes, permissions)
}

// 0x676264 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EED0Ev")]
pub fn stub_676264() {
    // IDA 0x676264: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x676290 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_676290() -> bool {
    // IDA 0x676290 (`GetSetImpl<GuiTextButton Color3>::isReadOnly`):
    // returns constant 0.
    false
}

// 0x676294 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_676294() -> ! {
    todo!("0x676294 __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")
}

// 0x676298 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_676298() -> ! {
    todo!("0x676298 __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x6762d0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_6762d0() -> ! {
    todo!("0x6762d0 __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEN3G3D6Color3EE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")
}

// 0x67630c — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_67630c() -> ! {
    todo!("0x67630c __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEEC2IMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x676420 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::~PropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED0Ev")]
pub fn stub_676420() {
    // IDA 0x676420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67644c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_67644c() -> ! {
    todo!("0x67644c __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE10isReadOnlyEv")
}

// 0x676450 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_676450() -> ! {
    todo!("0x676450 __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE11isWriteOnlyEv")
}

// 0x676454 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_676454() -> ! {
    todo!("0x676454 __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x67648c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_67648c() -> ! {
    todo!("0x67648c __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEE10GetSetImplIMNS_12GuiTextMixinEKFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x6764b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::EnumPropDescriptor<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>(char const*,char const*,RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_6764b0() -> ! {
    todo!("0x6764b0 __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x676664 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED0Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::~EnumPropDescriptor()
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED0Ev")]
pub fn stub_676664() {
    // IDA 0x676664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x676690 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10isReadOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isReadOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10isReadOnlyEv")]
pub fn stub_676690() -> ! {
    todo!("0x676690 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10isReadOnlyEv")
}

// 0x6766a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11isWriteOnlyEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isWriteOnly(void)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11isWriteOnlyEv")]
pub fn stub_6766a0() -> ! {
    todo!("0x6766a0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11isWriteOnlyEv")
}

// 0x6766b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_")]
pub fn stub_6766b0() -> ! {
    todo!("0x6766b0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11equalValuesEPKNS0_13DescribedBaseES8_")
}

// 0x6766d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_6766d8() -> ! {
    todo!("0x6766d8 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

// 0x6766fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_6766fc() -> ! {
    todo!("0x6766fc __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

// 0x676848 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
pub fn stub_676848() -> ! {
    todo!("0x676848 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9copyValueEPKNS0_13DescribedBaseEPS6_")
}

// 0x67686c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14hasStringValueEv
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::hasStringValue(void)const
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14hasStringValueEv")]
pub fn stub_67686c() -> ! {
    todo!("0x67686c __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14hasStringValueEv")
}

// 0x676870 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_676870() -> ! {
    todo!("0x676870 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14getStringValueEPKNS0_13DescribedBaseE")
}

// 0x676894 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_676894() -> ! {
    todo!("0x676894 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

// 0x6768d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_6768d4() -> ! {
    todo!("0x6768d4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

// 0x6768f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_6768f4() -> ! {
    todo!("0x6768f4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x676b34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_676b34() -> ! {
    todo!("0x676b34 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13getIndexValueEPKNS0_13DescribedBaseE")
}

// 0x676b50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub fn stub_676b50() -> ! {
    todo!("0x676b50 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE13setIndexValueEPNS0_13DescribedBaseEm")
}

// 0x676b84 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub fn stub_676b84() -> ! {
    todo!("0x676b84 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12getEnumValueEPKNS0_13DescribedBaseE")
}

// 0x676b8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub fn stub_676b8c() -> ! {
    todo!("0x676b8c __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE12setEnumValueEPNS0_13DescribedBaseEi")
}

// 0x676bd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub fn stub_676bd8() -> ! {
    todo!("0x676bd8 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11getEnumItemEPKNS0_13DescribedBaseE")
}

// 0x676bf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub fn stub_676bf8() -> ! {
    todo!("0x676bf8 __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

// 0x676c2c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_676c2c() -> ! {
    todo!("0x676c2c __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE11setIntValueEPNS0_13DescribedBaseEi")
}

// 0x676c6c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::isReadOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::GetSetImpl<RBX::TextService::Font (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::Font)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_676c6c() -> ! {
    todo!("0x676c6c __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService4FontEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")
}
