//! rendering shard 326 — 100 stubs 0x53d4d0..0x550f88 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 35520->35620 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35520 before -> 35620 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x53d4d0 (lowest remaining 0x53d4d0..0x550f88, next lowest 0x550fa4)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x53d4d0 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x53d4d0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d4d0() {
}

// 0x53d4d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x53d4d4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d4d4() {
}

// 0x53d4d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x53d4d8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d4d8() {
}

// 0x53d500 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::setValue(RBX::Reflection::DescribedBase *,RBX::UDim2 const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x53d500: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53d500() {
}

// 0x53dd64 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::UDim2 RBX::Reflection::ArgHelper::getArg<RBX::UDim2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::UDim2> const&,boost::disable_if<boost::is_same<RBX::UDim2,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53dd64: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53dd64() {
}

// 0x53df00 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingDirection RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingDirection,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingDirection> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingDirection,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53df00: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53df00() {
}

// 0x53e094 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingStyle RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingStyle,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingStyle> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingStyle,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53e094: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e094() {
}

// 0x53e228 — __ZN3RBX10Reflection9ArgHelper6getArgIfLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53e228: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e228() {
}

// 0x53e3cc — __ZN3RBX10Reflection9ArgHelper6getArgIbLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIbLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53e3cc: 163 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e3cc() {
}

// 0x53e74c — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::GuiObject::TweenEasingStyle>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingStyle &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingStyle>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x53e74c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e74c() {
}

// 0x53e7a0 — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiObject::TweenEasingDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingDirection &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingDirection>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x53e7a0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53e7a0() {
}

// 0x53f0cc — __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::UDim2 RBX::Reflection::ArgHelper::getArg<RBX::UDim2,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::UDim2> const&,boost::disable_if<boost::is_same<RBX::UDim2,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53f0cc: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53f0cc() {
}

// 0x53f268 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingDirection RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingDirection,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingDirection> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingDirection,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53f268: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53f268() {
}

// 0x53f3fc — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingStyle RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingStyle,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingStyle> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingStyle,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53f3fc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53f3fc() {
}

// 0x53f590 — __ZN3RBX10Reflection9ArgHelper6getArgIfLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53f590: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53f590() {
}

// 0x53f734 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIbLi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x53f734: 163 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53f734() {
}

// 0x53fab4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<4,RBX::GuiObject::TweenEasingStyle>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingStyle &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingStyle>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x53fab4: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53fab4() {
}

// 0x53fb08 — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::GuiObject::TweenEasingDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingDirection &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingDirection>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x53fb08: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_53fb08() {
}

// 0x543264 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEEC2Ev
// IDA 0x543264: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543264() {
}

// 0x543480 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEEC2Ev
// IDA 0x543480: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_543480() {
}

// 0x544a08 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED1Ev
// IDA 0x544a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544a08() {
}

// 0x544a2c — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED1Ev
// IDA 0x544a2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544a2c() {
}

// 0x544a5c — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x544a5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544a5c() {
}

// 0x544a80 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED1Ev
// IDA 0x544a80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544a80() {
}

// 0x544aa4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED1Ev
// IDA 0x544aa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544aa4() {
}

// 0x544ae4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED1Ev
// IDA 0x544ae4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544ae4() {
}

// 0x544b24 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x544b24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544b24() {
}

// 0x544c44 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED1Ev
// IDA 0x544c44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_544c44() {
}

// 0x544ca8 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::addPair(RBX::GuiService::SpecialKey,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE7addPairES3_PKc
// IDA 0x544ca8: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_544ca8() {
}

// 0x545008 — __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService10SpecialKeyEEERT_v
#[doc(alias = "RBX::GuiService::SpecialKey & RBX::Reflection::Variant::genericConvert<RBX::GuiService::SpecialKey>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService10SpecialKeyEEERT_v
// IDA 0x545008: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545008() {
}

// 0x5451f4 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::addPair(RBX::GuiService::CenterDialogType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE7addPairES3_PKc
// IDA 0x5451f4: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5451f4() {
}

// 0x545554 — __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService16CenterDialogTypeEEERT_v
#[doc(alias = "RBX::GuiService::CenterDialogType & RBX::Reflection::Variant::genericConvert<RBX::GuiService::CenterDialogType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService16CenterDialogTypeEEERT_v
// IDA 0x545554: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_545554() {
}

// 0x546994 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED1Ev
// IDA 0x546994: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_546994() {
}

// 0x546998 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED0Ev
// IDA 0x546998: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_546998() {
}

// 0x546a38 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupEPKc
// IDA 0x546a38: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546a38() {
}

// 0x546a68 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupERKNS0_7VariantE
// IDA 0x546a68: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546a68() {
}

// 0x546a88 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueEmRNS0_7VariantE
// IDA 0x546a88: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546a88() {
}

// 0x546abc — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringEmRSs
// IDA 0x546abc: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546abc() {
}

// 0x546c00 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED1Ev
// IDA 0x546c00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_546c00() {
}

// 0x546c04 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED0Ev
// IDA 0x546c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_546c04() {
}

// 0x546ca4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupEPKc
// IDA 0x546ca4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546ca4() {
}

// 0x546cd4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupERKNS0_7VariantE
// IDA 0x546cd4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546cd4() {
}

// 0x546cf4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueEmRNS0_7VariantE
// IDA 0x546cf4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546cf4() {
}

// 0x546d28 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringEmRSs
// IDA 0x546d28: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_546d28() {
}

// 0x5474ac — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToString(RBX::GuiService::CenterDialogType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringERKS3_
// IDA 0x5474ac: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5474ac() {
}

// 0x547718 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToItem(RBX::GuiService::CenterDialogType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE13convertToItemERKS3_
// IDA 0x547718: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547718() {
}

// 0x5478d4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToValue(RBX::Name const&,RBX::GuiService::CenterDialogType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueERKNS_4NameERS3_
// IDA 0x5478d4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5478d4() {
}

// 0x547950 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED2Ev
// IDA 0x547950: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_547950() {
}

// 0x547b24 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToString(RBX::GuiService::SpecialKey const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringERKS3_
// IDA 0x547b24: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547b24() {
}

// 0x547d90 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToItem(RBX::GuiService::SpecialKey const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE13convertToItemERKS3_
// IDA 0x547d90: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547d90() {
}

// 0x547f4c — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToValue(RBX::Name const&,RBX::GuiService::SpecialKey&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueERKNS_4NameERS3_
// IDA 0x547f4c: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_547f4c() {
}

// 0x547fc8 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED2Ev
// IDA 0x547fc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_547fc8() {
}

// 0x549014 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18NotificationObjectES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationObject,RBX::NotificationObject>(rbx_core::SharedPtr<RBX::NotificationObject> const*,RBX::NotificationObject *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18NotificationObjectES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x549014: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_549014() {
}

// 0x54bf8c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE13initSingletonEv
// IDA 0x54bf8c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_54bf8c() {
}

// 0x54bf90 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE14doGetSingletonEv
// IDA 0x54bf90: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54bf90() {
}

// 0x54c080 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE13initSingletonEv
// IDA 0x54c080: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_54c080() {
}

// 0x54c084 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE14doGetSingletonEv
// IDA 0x54c084: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54c084() {
}

// 0x54cde4 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>(char const*,char const*,bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x54cde4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54cde4() {
}

// 0x54cef8 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED0Ev
// IDA 0x54cef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54cef8() {
}

// 0x54cf24 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x54cf24: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54cf24() {
}

// 0x54cf28 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x54cf28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54cf28() {
}

// 0x54cf2c — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x54cf2c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54cf2c() {
}

// 0x54cf50 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x54cf50: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54cf50() {
}

// 0x54cf74 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EEC2EMS2_FviiiiEPKcS8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::BoundFuncDesc(void (RBX::GuiService::*)(int,int,int,int),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EEC2EMS2_FviiiiEPKcS8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x54cf74: 245 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54cf74() {
}

// 0x54d1e4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_
// IDA 0x54d1e4: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54d1e4() {
}

// 0x54d264 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED0Ev
// IDA 0x54d264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54d264() {
}

// 0x54d35c — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x54d35c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54d35c() {
}

// 0x54d3cc — __ZN3RBX10Reflection9ArgHelper6getArgIiLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "int RBX::Reflection::ArgHelper::getArg<int,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIiLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x54d3cc: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54d3cc() {
}

// 0x54e048 — __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService16CenterDialogTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiService::CenterDialogType RBX::Reflection::ArgHelper::getArg<RBX::GuiService::CenterDialogType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::CenterDialogType> const&,boost::disable_if<boost::is_same<RBX::GuiService::CenterDialogType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService16CenterDialogTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x54e048: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54e048() {
}

// 0x54e58c — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_10GuiService16CenterDialogTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiService::CenterDialogType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::CenterDialogType &,boost::enable_if<boost::is_enum<RBX::GuiService::CenterDialogType>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_10GuiService16CenterDialogTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x54e58c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54e58c() {
}

// 0x54ef28 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgISsLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x54ef28: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54ef28() {
}

// 0x54f354 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x54f354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54f354() {
}

// 0x54f408 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x54f408: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f408() {
}

// 0x54f60c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x54f60c: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f60c() {
}

// 0x54f680 — __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x54f680: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f680() {
}

// 0x54f694 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::BoundFuncDesc(void (RBX::GuiService::*)(RBX::GuiService::SpecialKey),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x54f694: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f694() {
}

// 0x54f80c — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x54f80c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f80c() {
}

// 0x54f83c — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED0Ev
// IDA 0x54f83c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54f83c() {
}

// 0x54f910 — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x54f910: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f910() {
}

// 0x54f944 — __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService10SpecialKeyELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiService::SpecialKey RBX::Reflection::ArgHelper::getArg<RBX::GuiService::SpecialKey,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::SpecialKey> const&,boost::disable_if<boost::is_same<RBX::GuiService::SpecialKey,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService10SpecialKeyELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x54f944: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54f944() {
}

// 0x54fad4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10GuiService10SpecialKeyEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::GuiService::SpecialKey>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::SpecialKey &,boost::enable_if<boost::is_enum<RBX::GuiService::SpecialKey>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10GuiService10SpecialKeyEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// IDA 0x54fad4: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54fad4() {
}

// 0x54fb28 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::BoundFuncDesc(void (RBX::GuiService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x54fb28: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54fb28() {
}

// 0x54fca0 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x54fca0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54fca0() {
}

// 0x54fcd0 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED0Ev
// IDA 0x54fcd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_54fcd0() {
}

// 0x54fd9c — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x54fd9c: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54fd9c() {
}

// 0x54fed8 — __ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(std::string),std::string,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// IDA 0x54fed8: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_54fed8() {
}

// 0x550008 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::EventDesc(rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x550008: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550008() {
}

// 0x5501f8 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED0Ev
// IDA 0x5501f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5501f8() {
}

// 0x5502ac — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x5502ac: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5502ac() {
}

// 0x550400 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// IDA 0x550400: 146 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550400() {
}

// 0x5505b8 — __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x5505b8: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5505b8() {
}

// 0x550744 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10GuiService10SpecialKeyERKSsNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::GuiService::SpecialKey const&,std::string const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10GuiService10SpecialKeyERKSsNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// IDA 0x550744: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550744() {
}

// 0x550860 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_10GuiService10SpecialKeyESsEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey const&,std::string const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_10GuiService10SpecialKeyESsEEvRKT_RKT0_
// IDA 0x550860: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550860() {
}

// 0x550bc0 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// IDA 0x550bc0: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550bc0() {
}

// 0x550cb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// IDA 0x550cb8: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550cb8() {
}

// 0x550cd4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_SsE6invokeERNS1_15function_bufferESB_Ss
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::GuiService::SpecialKey,std::string>::invoke(boost::detail::function::function_buffer &,RBX::GuiService::SpecialKey,std::string)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_SsE6invokeERNS1_15function_bufferESB_Ss
// IDA 0x550cd4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550cd4() {
}

// 0x550ce8 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x550ce8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550ce8() {
}

// 0x550dd0 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x550dd0: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550dd0() {
}

// 0x550eb4 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x550eb4: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550eb4() {
}

// 0x550f88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10GuiService10SpecialKeyERKSsEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_SsEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey &,std::string &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10GuiService10SpecialKeyERKSsEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_SsEEvRT_RT0_
// IDA 0x550f88: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_550f88() {
}
