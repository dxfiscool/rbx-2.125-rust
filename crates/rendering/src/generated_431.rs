//! rendering shard 431 — 100 stubs 0x671ff8..0x675a54 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 46410->46510 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x671ff8..0x675a54 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x671ff8 — __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EED0Ev
// IDA 0x671ff8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_671ff8() {
}

// 0x6720ac — __ZNK3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TextBox,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_7TextBoxEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x6720ac: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6720ac() {
}

// 0x6720cc — __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::PropDescriptor<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>(char const*,char const*,bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_7TextBoxEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x6720cc: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6720cc() {
}

// 0x6721e0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x6721e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6721e0() {
}

// 0x6721e4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x6721e4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6721e4() {
}

// 0x6721e8 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x6721e8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6721e8() {
}

// 0x67220c — __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextBox,bool>::GetSetImpl<bool (RBX::TextBox::*)(void)const,void (RBX::TextBox::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_7TextBoxEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x67220c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67220c() {
}

// 0x672230 — __ZN3RBX7TextBoxD2Ev
// type: void __fastcall(RBX::TextBox *this, int, int, int)
#[doc(alias = "__ZN3RBX7TextBoxD2Ev")]
#[doc(alias = "RBX::TextBox::~TextBox()")]
// was: __ZN3RBX7TextBoxD2Ev
// IDA 0x672230: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_672230() {
}

// 0x672440 — __GLOBAL__I_a_270
// type: int()
#[doc(alias = "__GLOBAL__I_a_270")]
#[doc(alias = "global constructor keyed to_a_270")]
// was: __GLOBAL__I_a_270
// IDA 0x672440: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_672440() {
}

// 0x672d68 — __ZN3RBX13GuiTextButtonC2Ev
// type: RBX::GuiButton *__fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZN3RBX13GuiTextButtonC2Ev")]
#[doc(alias = "RBX::GuiTextButton::GuiTextButton(void)")]
// was: __ZN3RBX13GuiTextButtonC2Ev
// IDA 0x672d68: 243 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_672d68() {
}

// 0x67303c — __ZN3RBX13GuiTextButton7setTextESs
// type: void __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "__ZN3RBX13GuiTextButton7setTextESs")]
#[doc(alias = "RBX::GuiTextButton::setText(std::string)")]
// was: __ZN3RBX13GuiTextButton7setTextESs
// IDA 0x67303c: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67303c() {
}

// 0x6731f8 — __ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE")]
#[doc(alias = "RBX::GuiTextButton::setFontSize(RBX::TextService::FontSize)")]
// was: __ZN3RBX13GuiTextButton11setFontSizeENS_11TextService8FontSizeE
// IDA 0x6731f8: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6731f8() {
}

// 0x673230 — __ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE")]
#[doc(alias = "RBX::GuiTextButton::setFont(RBX::TextService::Font)")]
// was: __ZN3RBX13GuiTextButton7setFontENS_11TextService4FontE
// IDA 0x673230: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673230() {
}

// 0x673268 — __ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE")]
#[doc(alias = "RBX::GuiTextButton::setTextColor(RBX::BrickColor)")]
// was: __ZN3RBX13GuiTextButton12setTextColorENS_10BrickColorE
// IDA 0x673268: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673268() {
}

// 0x673308 — __ZN3RBX13GuiTextButton19setTextTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextTransparencyEf")]
#[doc(alias = "RBX::GuiTextButton::setTextTransparency(float)")]
// was: __ZN3RBX13GuiTextButton19setTextTransparencyEf
// IDA 0x673308: 11 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673308() {
}

// 0x673330 — __ZN3RBX13GuiTextButton11setTextWrapEb
// type: int __fastcall(RBX::GuiTextButton *this, int)
#[doc(alias = "__ZN3RBX13GuiTextButton11setTextWrapEb")]
#[doc(alias = "RBX::GuiTextButton::setTextWrap(bool)")]
// was: __ZN3RBX13GuiTextButton11setTextWrapEb
// IDA 0x673330: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673330() {
}

// 0x673370 — __ZN3RBX13GuiTextButton12setTextScaleEb
// type: int __fastcall(RBX::GuiTextButton *this, int)
#[doc(alias = "__ZN3RBX13GuiTextButton12setTextScaleEb")]
#[doc(alias = "RBX::GuiTextButton::setTextScale(bool)")]
// was: __ZN3RBX13GuiTextButton12setTextScaleEb
// IDA 0x673370: 28 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673370() {
}

// 0x6733c4 — __ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE")]
#[doc(alias = "RBX::GuiTextButton::setXAlignment(RBX::TextService::XAlignment)")]
// was: __ZN3RBX13GuiTextButton13setXAlignmentENS_11TextService10XAlignmentE
// IDA 0x6733c4: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6733c4() {
}

// 0x673404 — __ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "__ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE")]
#[doc(alias = "RBX::GuiTextButton::setYAlignment(RBX::TextService::YAlignment)")]
// was: __ZN3RBX13GuiTextButton13setYAlignmentENS_11TextService10YAlignmentE
// IDA 0x673404: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673404() {
}

// 0x673444 — __ZNK3RBX13GuiTextButton13getTextBoundsEv
// type: void __fastcall(RBX::GuiTextButton *this, unsigned int, bool)
#[doc(alias = "__ZNK3RBX13GuiTextButton13getTextBoundsEv")]
#[doc(alias = "RBX::GuiTextButton::getTextBounds(void)const")]
// was: __ZNK3RBX13GuiTextButton13getTextBoundsEv
// IDA 0x673444: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673444() {
}

// 0x6735d0 — __ZNK3RBX13GuiTextButton11getTextFitsEv
// type: int __fastcall(RBX::GuiTextButton *this, int, bool)
#[doc(alias = "__ZNK3RBX13GuiTextButton11getTextFitsEv")]
#[doc(alias = "RBX::GuiTextButton::getTextFits(void)const")]
// was: __ZNK3RBX13GuiTextButton11getTextFitsEv
// IDA 0x6735d0: 152 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6735d0() {
}

// 0x6737e8 — __ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf
// type: float *__fastcall(float *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf")]
#[doc(alias = "RBX::GuiTextButton::setTextStrokeTransparency(float)")]
// was: __ZN3RBX13GuiTextButton25setTextStrokeTransparencyEf
// IDA 0x6737e8: 11 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6737e8() {
}

// 0x673814 — __ZN3RBX13GuiTextButton14checkForResizeEv
// type: int __fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZN3RBX13GuiTextButton14checkForResizeEv")]
#[doc(alias = "RBX::GuiTextButton::checkForResize(void)")]
// was: __ZN3RBX13GuiTextButton14checkForResizeEv
// IDA 0x673814: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673814() {
}

// 0x673840 — __ZN3RBX13GuiTextButton21setTransparencyLegacyEf
// type: int __fastcall(RBX::GuiTextButton *this, float)
#[doc(alias = "__ZN3RBX13GuiTextButton21setTransparencyLegacyEf")]
#[doc(alias = "RBX::GuiTextButton::setTransparencyLegacy(float)")]
// was: __ZN3RBX13GuiTextButton21setTransparencyLegacyEf
// IDA 0x673840: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673840() {
}

// 0x673888 — __ZNK3RBX13GuiTextButton21getPersistentDataCostEv
// type: int __fastcall(RBX::GuiTextButton *this)
#[doc(alias = "__ZNK3RBX13GuiTextButton21getPersistentDataCostEv")]
#[doc(alias = "RBX::GuiTextButton::getPersistentDataCost(void)const")]
// was: __ZNK3RBX13GuiTextButton21getPersistentDataCostEv
// IDA 0x673888: 44 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673888() {
}

// 0x673b7c — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonESsED1Ev
// IDA 0x673b7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673b7c() {
}

// 0x673ba0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::FontSize>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService8FontSizeEED1Ev
// IDA 0x673ba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673ba0() {
}

// 0x673bc4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::Font>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService4FontEED1Ev
// IDA 0x673bc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673bc4() {
}

// 0x673be8 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::BrickColor>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_10BrickColorEED1Ev
// IDA 0x673be8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673be8() {
}

// 0x673c30 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEfED1Ev
// IDA 0x673c30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673c30() {
}

// 0x673c54 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED1Ev
// IDA 0x673c54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673c54() {
}

// 0x673c78 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED1Ev
// IDA 0x673c78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673c78() {
}

// 0x673c9c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED1Ev
// IDA 0x673c9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673c9c() {
}

// 0x673ce4 — __ZN3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZN3RBX13GuiTextButtonD1Ev")]
#[doc(alias = "RBX::GuiTextButton::~GuiTextButton()")]
// was: __ZN3RBX13GuiTextButtonD1Ev
// IDA 0x673ce4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673ce4() {
}

// 0x673cfc — __ZN3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZN3RBX13GuiTextButtonD0Ev")]
#[doc(alias = "RBX::GuiTextButton::~GuiTextButton()")]
// was: __ZN3RBX13GuiTextButtonD0Ev
// IDA 0x673cfc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673cfc() {
}

// 0x673da8 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// IDA 0x673da8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673da8() {
}

// 0x673db8 — __ZThn32_N3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
// was: __ZThn32_N3RBX13GuiTextButtonD1Ev
// IDA 0x673db8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673db8() {
}

// 0x673dd4 — __ZThn32_N3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX13GuiTextButtonD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
// was: __ZThn32_N3RBX13GuiTextButtonD0Ev
// IDA 0x673dd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673dd4() {
}

// 0x673e80 — __ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE12getClassNameEv
// IDA 0x673e80: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673e80() {
}

// 0x673e90 — __ZThn36_N3RBX13GuiTextButtonD1Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
// was: __ZThn36_N3RBX13GuiTextButtonD1Ev
// IDA 0x673e90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673e90() {
}

// 0x673eac — __ZThn36_N3RBX13GuiTextButtonD0Ev
// type: void __fastcall(RBX::GuiTextButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX13GuiTextButtonD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiTextButton::~GuiTextButton()")]
// was: __ZThn36_N3RBX13GuiTextButtonD0Ev
// IDA 0x673eac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673eac() {
}

// 0x673f58 — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD1Ev
// IDA 0x673f58: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_673f58() {
}

// 0x673f5c — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorD2Ev
// IDA 0x673f5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_673f5c() {
}

// 0x673ff8 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x673ff8: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_673ff8() {
}

// 0x674080 — __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7Creator6createEv
// IDA 0x674080: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674080() {
}

// 0x6741c4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13GuiTextButtonEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int *)
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_13GuiTextButtonEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiTextButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiTextButton>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13GuiTextButtonEEEN5boost10shared_ptrIT_EEv
// IDA 0x6741c4: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6741c4() {
}

// 0x674278 — __ZN5boost10shared_ptrIN3RBX13GuiTextButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13GuiTextButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiTextButton>::shared_ptr<RBX::GuiTextButton,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13GuiTextButtonEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x674278: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674278() {
}

// 0x674340 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiTextButton,RBX::GuiTextButton>(rbx_core::SharedPtr<RBX::GuiTextButton> const*,RBX::GuiTextButton *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13GuiTextButtonES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x674340: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674340() {
}

// 0x674428 — __ZN5boost6detail12shared_countC2IPN3RBX13GuiTextButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13GuiTextButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13GuiTextButtonENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x674428: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674428() {
}

// 0x674530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x674530: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_674530() {
}

// 0x674534 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x674534: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_674534() {
}

// 0x674538 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x674538: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674538() {
}

// 0x674558 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x674558: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674558() {
}

// 0x674570 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiTextButton *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13GuiTextButtonENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x674570: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674570() {
}

// 0x674574 — __ZN3RBX4Name13callDoDeclareILZNS_14sGuiTextButtonEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sGuiTextButtonEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sGuiTextButtonEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sGuiTextButtonEEEEvv
// IDA 0x674574: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_674574() {
}

// 0x674578 — __ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v
// IDA 0x674578: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674578() {
}

// 0x674658 — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE7CreatorC2Ev
// IDA 0x674658: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674658() {
}

// 0x67489c — __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13GuiTextButtonENS_9GuiButtonELZNS_14sGuiTextButtonEENS_8InstanceEE17static_getCreatorEv
// IDA 0x67489c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67489c() {
}

// 0x674910 — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::PropDescriptor<bool (RBX::GuiTextButton::*)(void)const,int>(char const*,char const*,bool (RBX::GuiTextButton::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x674910: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674910() {
}

// 0x674a1c — __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbED0Ev
// IDA 0x674a1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_674a1c() {
}

// 0x674a48 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
// IDA 0x674a48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674a48() {
}

// 0x674a4c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
// IDA 0x674a4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674a4c() {
}

// 0x674a50 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x674a50: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674a50() {
}

// 0x674a74 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: void __noreturn()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,bool>::GetImpl<bool (RBX::GuiTextButton::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x674a74: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674a74() {
}

// 0x674e1c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::EnumPropDescriptor<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>(char const*,char const*,RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x674e1c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674e1c() {
}

// 0x674fd0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEED0Ev
// IDA 0x674fd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_674fd0() {
}

// 0x674ffc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10isReadOnlyEv
// IDA 0x674ffc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_674ffc() {
}

// 0x67500c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11isWriteOnlyEv
// IDA 0x67500c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67500c() {
}

// 0x67501c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x67501c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67501c() {
}

// 0x675044 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x675044: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675044() {
}

// 0x675068 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x675068: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675068() {
}

// 0x6751b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x6751b4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6751b4() {
}

// 0x6751d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14hasStringValueEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14hasStringValueEv
// IDA 0x6751d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6751d8() {
}

// 0x6751dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6751dc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6751dc() {
}

// 0x675200 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x675200: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675200() {
}

// 0x675240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x675240: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675240() {
}

// 0x675260 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x675260: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675260() {
}

// 0x6754a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x6754a0: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6754a0() {
}

// 0x6754bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x6754bc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6754bc() {
}

// 0x6754f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x6754f0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6754f0() {
}

// 0x6754f8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x6754f8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6754f8() {
}

// 0x675544 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x675544: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675544() {
}

// 0x675564 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x675564: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675564() {
}

// 0x675598 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x675598: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675598() {
}

// 0x6755d8 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x6755d8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6755d8() {
}

// 0x6755dc — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x6755dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6755dc() {
}

// 0x6755e0 — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x6755e0: 16 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6755e0() {
}

// 0x67560c — __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiTextButton,RBX::TextService::YAlignment>::GetSetImpl<RBX::TextService::YAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::YAlignment)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextService::YAlignment const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13GuiTextButtonENS_11TextService10YAlignmentEE10GetSetImplIMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x67560c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67560c() {
}

// 0x675630 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::EnumPropDescriptor<RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment)>(char const*,char const*,RBX::TextService::XAlignment (RBX::GuiTextMixin::*)(void)const,void (RBX::GuiTextButton::*)(RBX::TextService::XAlignment),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEEC2IMNS_12GuiTextMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x675630: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675630() {
}

// 0x6757e4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEED0Ev
// IDA 0x6757e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6757e4() {
}

// 0x675810 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10isReadOnlyEv
// IDA 0x675810: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675810() {
}

// 0x675820 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11isWriteOnlyEv
// IDA 0x675820: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675820() {
}

// 0x675830 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// type: bool __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x675830: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675830() {
}

// 0x675858 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x675858: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675858() {
}

// 0x67587c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x67587c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_67587c() {
}

// 0x6759c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x6759c8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6759c8() {
}

// 0x6759ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14hasStringValueEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14hasStringValueEv
// IDA 0x6759ec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6759ec() {
}

// 0x6759f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x6759f0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6759f0() {
}

// 0x675a14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x675a14: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675a14() {
}

// 0x675a54 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GuiTextButton,RBX::TextService::XAlignment>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13GuiTextButtonENS_11TextService10XAlignmentEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x675a54: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_675a54() {
}
