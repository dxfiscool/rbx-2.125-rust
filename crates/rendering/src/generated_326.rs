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
pub fn stub_53d4d0() -> ! {
    todo!("0x53d4d0 RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::isReadOnly(void)const")
}

// 0x53d4d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_53d4d4() -> ! {
    todo!("0x53d4d4 RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::isWriteOnly(void)const")
}

// 0x53d4d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_53d4d8() -> ! {
    todo!("0x53d4d8 RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x53d500 — __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::setValue(RBX::Reflection::DescribedBase *,RBX::UDim2 const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9GuiObjectENS_5UDim2EE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_53d500() -> ! {
    todo!("0x53d500 RBX::Reflection::PropDescriptor<RBX::GuiObject,RBX::UDim2>::GetSetImpl<RBX::UDim2 (RBX::GuiObject::*)(void)const,void (RBX::GuiObject::*)(RBX::UDim2)>::setValue(RBX::Reflection::DescribedBase *,RBX::UDim2 const&)const")
}

// 0x53dd64 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::UDim2 RBX::Reflection::ArgHelper::getArg<RBX::UDim2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::UDim2> const&,boost::disable_if<boost::is_same<RBX::UDim2,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53dd64() -> ! {
    todo!("0x53dd64 RBX::UDim2 RBX::Reflection::ArgHelper::getArg<RBX::UDim2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::UDim2> const&,boost::disable_if<boost::is_same<RBX::UDim2,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53df00 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingDirection RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingDirection,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingDirection> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingDirection,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53df00() -> ! {
    todo!("0x53df00 RBX::GuiObject::TweenEasingDirection RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingDirection,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingDirection> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingDirection,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53e094 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingStyle RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingStyle,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingStyle> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingStyle,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53e094() -> ! {
    todo!("0x53e094 RBX::GuiObject::TweenEasingStyle RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingStyle,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingStyle> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingStyle,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53e228 — __ZN3RBX10Reflection9ArgHelper6getArgIfLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53e228() -> ! {
    todo!("0x53e228 float RBX::Reflection::ArgHelper::getArg<float,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53e3cc — __ZN3RBX10Reflection9ArgHelper6getArgIbLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIbLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53e3cc() -> ! {
    todo!("0x53e3cc bool RBX::Reflection::ArgHelper::getArg<bool,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53e74c — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::GuiObject::TweenEasingStyle>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingStyle &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingStyle>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_53e74c() -> ! {
    todo!("0x53e74c bool RBX::Reflection::ArgHelper::try_enum<3,RBX::GuiObject::TweenEasingStyle>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingStyle &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingStyle>,void>::type *)")
}

// 0x53e7a0 — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiObject::TweenEasingDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingDirection &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingDirection>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_53e7a0() -> ! {
    todo!("0x53e7a0 bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiObject::TweenEasingDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingDirection &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingDirection>,void>::type *)")
}

// 0x53f0cc — __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::UDim2 RBX::Reflection::ArgHelper::getArg<RBX::UDim2,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::UDim2> const&,boost::disable_if<boost::is_same<RBX::UDim2,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_5UDim2ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53f0cc() -> ! {
    todo!("0x53f0cc RBX::UDim2 RBX::Reflection::ArgHelper::getArg<RBX::UDim2,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::UDim2> const&,boost::disable_if<boost::is_same<RBX::UDim2,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53f268 — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingDirection RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingDirection,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingDirection> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingDirection,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject20TweenEasingDirectionELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53f268() -> ! {
    todo!("0x53f268 RBX::GuiObject::TweenEasingDirection RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingDirection,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingDirection> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingDirection,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53f3fc — __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiObject::TweenEasingStyle RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingStyle,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingStyle> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingStyle,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9GuiObject16TweenEasingStyleELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53f3fc() -> ! {
    todo!("0x53f3fc RBX::GuiObject::TweenEasingStyle RBX::Reflection::ArgHelper::getArg<RBX::GuiObject::TweenEasingStyle,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiObject::TweenEasingStyle> const&,boost::disable_if<boost::is_same<RBX::GuiObject::TweenEasingStyle,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53f590 — __ZN3RBX10Reflection9ArgHelper6getArgIfLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "float RBX::Reflection::ArgHelper::getArg<float,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIfLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53f590() -> ! {
    todo!("0x53f590 float RBX::Reflection::ArgHelper::getArg<float,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<float> const&,boost::disable_if<boost::is_same<float,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53f734 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIbLi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_53f734() -> ! {
    todo!("0x53f734 bool RBX::Reflection::ArgHelper::getArg<bool,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x53fab4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<4,RBX::GuiObject::TweenEasingStyle>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingStyle &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingStyle>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_9GuiObject16TweenEasingStyleEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_53fab4() -> ! {
    todo!("0x53fab4 bool RBX::Reflection::ArgHelper::try_enum<4,RBX::GuiObject::TweenEasingStyle>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingStyle &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingStyle>,void>::type *)")
}

// 0x53fb08 — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::GuiObject::TweenEasingDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingDirection &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingDirection>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_9GuiObject20TweenEasingDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_53fb08() -> ! {
    todo!("0x53fb08 bool RBX::Reflection::ArgHelper::try_enum<3,RBX::GuiObject::TweenEasingDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiObject::TweenEasingDirection &,boost::enable_if<boost::is_enum<RBX::GuiObject::TweenEasingDirection>,void>::type *)")
}

// 0x543264 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEEC2Ev
pub fn stub_543264() -> ! {
    todo!("0x543264 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::EnumDesc(void)")
}

// 0x543480 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEEC2Ev
pub fn stub_543480() -> ! {
    todo!("0x543480 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::EnumDesc(void)")
}

// 0x544a08 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEdED1Ev
pub fn stub_544a08() -> ! {
    todo!("0x544a08 RBX::Reflection::PropDescriptor<RBX::GuiService,double>::~PropDescriptor()")
}

// 0x544a2c — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED1Ev
pub fn stub_544a2c() -> ! {
    todo!("0x544a2c RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::~PropDescriptor()")
}

// 0x544a5c — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
pub fn stub_544a5c() -> ! {
    todo!("0x544a5c RBX::Reflection::EventDesc<RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::~EventDesc()")
}

// 0x544a80 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED1Ev
pub fn stub_544a80() -> ! {
    todo!("0x544a80 RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::~EventDesc()")
}

// 0x544aa4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED1Ev
pub fn stub_544aa4() -> ! {
    todo!("0x544aa4 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x544ae4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED1Ev
pub fn stub_544ae4() -> ! {
    todo!("0x544ae4 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::~BoundFuncDesc()")
}

// 0x544b24 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
pub fn stub_544b24() -> ! {
    todo!("0x544b24 RBX::Reflection::EventDesc<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::~EventDesc()")
}

// 0x544c44 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED1Ev
pub fn stub_544c44() -> ! {
    todo!("0x544c44 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::~BoundFuncDesc()")
}

// 0x544ca8 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::addPair(RBX::GuiService::SpecialKey,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE7addPairES3_PKc
pub fn stub_544ca8() -> ! {
    todo!("0x544ca8 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::addPair(RBX::GuiService::SpecialKey,char const*)")
}

// 0x545008 — __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService10SpecialKeyEEERT_v
#[doc(alias = "RBX::GuiService::SpecialKey & RBX::Reflection::Variant::genericConvert<RBX::GuiService::SpecialKey>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService10SpecialKeyEEERT_v
pub fn stub_545008() -> ! {
    todo!("0x545008 RBX::GuiService::SpecialKey & RBX::Reflection::Variant::genericConvert<RBX::GuiService::SpecialKey>(void)")
}

// 0x5451f4 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::addPair(RBX::GuiService::CenterDialogType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE7addPairES3_PKc
pub fn stub_5451f4() -> ! {
    todo!("0x5451f4 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::addPair(RBX::GuiService::CenterDialogType,char const*)")
}

// 0x545554 — __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService16CenterDialogTypeEEERT_v
#[doc(alias = "RBX::GuiService::CenterDialogType & RBX::Reflection::Variant::genericConvert<RBX::GuiService::CenterDialogType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10GuiService16CenterDialogTypeEEERT_v
pub fn stub_545554() -> ! {
    todo!("0x545554 RBX::GuiService::CenterDialogType & RBX::Reflection::Variant::genericConvert<RBX::GuiService::CenterDialogType>(void)")
}

// 0x546994 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED1Ev
pub fn stub_546994() -> ! {
    todo!("0x546994 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")
}

// 0x546998 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED0Ev
pub fn stub_546998() -> ! {
    todo!("0x546998 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")
}

// 0x546a38 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupEPKc
pub fn stub_546a38() -> ! {
    todo!("0x546a38 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::lookup(char const*)const")
}

// 0x546a68 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE6lookupERKNS0_7VariantE
pub fn stub_546a68() -> ! {
    todo!("0x546a68 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x546a88 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueEmRNS0_7VariantE
pub fn stub_546a88() -> ! {
    todo!("0x546a88 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x546abc — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringEmRSs
pub fn stub_546abc() -> ! {
    todo!("0x546abc RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToString(unsigned long,std::string &)const")
}

// 0x546c00 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED1Ev
pub fn stub_546c00() -> ! {
    todo!("0x546c00 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")
}

// 0x546c04 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED0Ev
pub fn stub_546c04() -> ! {
    todo!("0x546c04 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")
}

// 0x546ca4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupEPKc
pub fn stub_546ca4() -> ! {
    todo!("0x546ca4 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::lookup(char const*)const")
}

// 0x546cd4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE6lookupERKNS0_7VariantE
pub fn stub_546cd4() -> ! {
    todo!("0x546cd4 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x546cf4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueEmRNS0_7VariantE
pub fn stub_546cf4() -> ! {
    todo!("0x546cf4 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x546d28 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringEmRSs
pub fn stub_546d28() -> ! {
    todo!("0x546d28 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToString(unsigned long,std::string &)const")
}

// 0x5474ac — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToString(RBX::GuiService::CenterDialogType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE15convertToStringERKS3_
pub fn stub_5474ac() -> ! {
    todo!("0x5474ac RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToString(RBX::GuiService::CenterDialogType const&)const")
}

// 0x547718 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToItem(RBX::GuiService::CenterDialogType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE13convertToItemERKS3_
pub fn stub_547718() -> ! {
    todo!("0x547718 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToItem(RBX::GuiService::CenterDialogType const&)const")
}

// 0x5478d4 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToValue(RBX::Name const&,RBX::GuiService::CenterDialogType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEE14convertToValueERKNS_4NameERS3_
pub fn stub_5478d4() -> ! {
    todo!("0x5478d4 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::convertToValue(RBX::Name const&,RBX::GuiService::CenterDialogType&)const")
}

// 0x547950 — __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService16CenterDialogTypeEED2Ev
pub fn stub_547950() -> ! {
    todo!("0x547950 RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType>::~EnumDesc()")
}

// 0x547b24 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToString(RBX::GuiService::SpecialKey const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE15convertToStringERKS3_
pub fn stub_547b24() -> ! {
    todo!("0x547b24 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToString(RBX::GuiService::SpecialKey const&)const")
}

// 0x547d90 — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToItem(RBX::GuiService::SpecialKey const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE13convertToItemERKS3_
pub fn stub_547d90() -> ! {
    todo!("0x547d90 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToItem(RBX::GuiService::SpecialKey const&)const")
}

// 0x547f4c — __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToValue(RBX::Name const&,RBX::GuiService::SpecialKey&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEE14convertToValueERKNS_4NameERS3_
pub fn stub_547f4c() -> ! {
    todo!("0x547f4c RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::convertToValue(RBX::Name const&,RBX::GuiService::SpecialKey&)const")
}

// 0x547fc8 — __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_10GuiService10SpecialKeyEED2Ev
pub fn stub_547fc8() -> ! {
    todo!("0x547fc8 RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey>::~EnumDesc()")
}

// 0x549014 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18NotificationObjectES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationObject,RBX::NotificationObject>(boost::shared_ptr<RBX::NotificationObject> const*,RBX::NotificationObject *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18NotificationObjectES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_549014() -> ! {
    todo!("0x549014 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NotificationObject,RBX::NotificationObject>(boost::shared_ptr<RBX::NotificationObject> const*,RBX::NotificationObject *)const")
}

// 0x54bf8c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE13initSingletonEv
pub fn stub_54bf8c() -> ! {
    todo!("0x54bf8c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType> const>::initSingleton(void)")
}

// 0x54bf90 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService16CenterDialogTypeEEEE14doGetSingletonEv
pub fn stub_54bf90() -> ! {
    todo!("0x54bf90 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::CenterDialogType> const>::doGetSingleton(void)")
}

// 0x54c080 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE13initSingletonEv
pub fn stub_54c080() -> ! {
    todo!("0x54c080 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey> const>::initSingleton(void)")
}

// 0x54c084 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10GuiService10SpecialKeyEEEE14doGetSingletonEv
pub fn stub_54c084() -> ! {
    todo!("0x54c084 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiService::SpecialKey> const>::doGetSingleton(void)")
}

// 0x54cde4 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>(char const*,char const*,bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_54cde4() -> ! {
    todo!("0x54cde4 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::PropDescriptor<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>(char const*,char const*,bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x54cef8 — __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10GuiServiceEbED0Ev
pub fn stub_54cef8() -> ! {
    todo!("0x54cef8 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::~PropDescriptor()")
}

// 0x54cf24 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
pub fn stub_54cf24() -> ! {
    todo!("0x54cf24 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::isReadOnly(void)const")
}

// 0x54cf28 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
pub fn stub_54cf28() -> ! {
    todo!("0x54cf28 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::isWriteOnly(void)const")
}

// 0x54cf2c — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_54cf2c() -> ! {
    todo!("0x54cf2c RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x54cf50 — __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10GuiServiceEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_54cf50() -> ! {
    todo!("0x54cf50 RBX::Reflection::PropDescriptor<RBX::GuiService,bool>::GetSetImpl<bool (RBX::GuiService::*)(void)const,void (RBX::GuiService::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x54cf74 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EEC2EMS2_FviiiiEPKcS8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::BoundFuncDesc(void (RBX::GuiService::*)(int,int,int,int),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EEC2EMS2_FviiiiEPKcS8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_54cf74() -> ! {
    todo!("0x54cf74 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::BoundFuncDesc(void (RBX::GuiService::*)(int,int,int,int),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x54d1e4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_
pub fn stub_54d1e4() -> ! {
    todo!("0x54d1e4 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x54d264 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EED0Ev
pub fn stub_54d264() -> ! {
    todo!("0x54d264 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::~BoundFuncDesc()")
}

// 0x54d35c — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFviiiiELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_54d35c() -> ! {
    todo!("0x54d35c RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(int,int,int,int),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x54d3cc — __ZN3RBX10Reflection9ArgHelper6getArgIiLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "int RBX::Reflection::ArgHelper::getArg<int,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIiLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_54d3cc() -> ! {
    todo!("0x54d3cc int RBX::Reflection::ArgHelper::getArg<int,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x54e048 — __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService16CenterDialogTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiService::CenterDialogType RBX::Reflection::ArgHelper::getArg<RBX::GuiService::CenterDialogType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::CenterDialogType> const&,boost::disable_if<boost::is_same<RBX::GuiService::CenterDialogType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService16CenterDialogTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_54e048() -> ! {
    todo!("0x54e048 RBX::GuiService::CenterDialogType RBX::Reflection::ArgHelper::getArg<RBX::GuiService::CenterDialogType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::CenterDialogType> const&,boost::disable_if<boost::is_same<RBX::GuiService::CenterDialogType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x54e58c — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_10GuiService16CenterDialogTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiService::CenterDialogType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::CenterDialogType &,boost::enable_if<boost::is_enum<RBX::GuiService::CenterDialogType>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_10GuiService16CenterDialogTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_54e58c() -> ! {
    todo!("0x54e58c bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiService::CenterDialogType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::CenterDialogType &,boost::enable_if<boost::is_enum<RBX::GuiService::CenterDialogType>,void>::type *)")
}

// 0x54ef28 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgISsLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_54ef28() -> ! {
    todo!("0x54ef28 std::string RBX::Reflection::ArgHelper::getArg<std::string,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x54f354 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
pub fn stub_54f354() -> ! {
    todo!("0x54f354 RBX::Reflection::EventDesc<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::~EventDesc()")
}

// 0x54f408 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_54f408() -> ! {
    todo!("0x54f408 RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x54f60c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
pub fn stub_54f60c() -> ! {
    todo!("0x54f60c RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x54f680 — __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_54f680() -> ! {
    todo!("0x54f680 RBX::Reflection::EventDescBase<RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x54f694 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::BoundFuncDesc(void (RBX::GuiService::*)(RBX::GuiService::SpecialKey),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_54f694() -> ! {
    todo!("0x54f694 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::BoundFuncDesc(void (RBX::GuiService::*)(RBX::GuiService::SpecialKey),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x54f80c — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_54f80c() -> ! {
    todo!("0x54f80c RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x54f83c — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EED0Ev
pub fn stub_54f83c() -> ! {
    todo!("0x54f83c RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::~BoundFuncDesc()")
}

// 0x54f910 — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvNS2_10SpecialKeyEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_54f910() -> ! {
    todo!("0x54f910 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x54f944 — __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService10SpecialKeyELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::GuiService::SpecialKey RBX::Reflection::ArgHelper::getArg<RBX::GuiService::SpecialKey,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::SpecialKey> const&,boost::disable_if<boost::is_same<RBX::GuiService::SpecialKey,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService10SpecialKeyELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_54f944() -> ! {
    todo!("0x54f944 RBX::GuiService::SpecialKey RBX::Reflection::ArgHelper::getArg<RBX::GuiService::SpecialKey,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::SpecialKey> const&,boost::disable_if<boost::is_same<RBX::GuiService::SpecialKey,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x54fad4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10GuiService10SpecialKeyEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::GuiService::SpecialKey>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::SpecialKey &,boost::enable_if<boost::is_enum<RBX::GuiService::SpecialKey>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10GuiService10SpecialKeyEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_54fad4() -> ! {
    todo!("0x54fad4 bool RBX::Reflection::ArgHelper::try_enum<1,RBX::GuiService::SpecialKey>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::SpecialKey &,boost::enable_if<boost::is_enum<RBX::GuiService::SpecialKey>,void>::type *)")
}

// 0x54fb28 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::BoundFuncDesc(void (RBX::GuiService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_54fb28() -> ! {
    todo!("0x54fb28 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::BoundFuncDesc(void (RBX::GuiService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x54fca0 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_54fca0() -> ! {
    todo!("0x54fca0 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x54fcd0 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EED0Ev
pub fn stub_54fcd0() -> ! {
    todo!("0x54fcd0 RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x54fd9c — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_54fd9c() -> ! {
    todo!("0x54fd9c RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x54fed8 — __ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(std::string),std::string,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
pub fn stub_54fed8() -> ! {
    todo!("0x54fed8 RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(std::string),std::string,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x550008 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::EventDesc(rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_EC2ES8_PKcSB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_550008() -> ! {
    todo!("0x550008 RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::EventDesc(rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5501f8 — __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_ED0Ev
pub fn stub_5501f8() -> ! {
    todo!("0x5501f8 RBX::Reflection::EventDesc<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::~EventDesc()")
}

// 0x5502ac — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
pub fn stub_5502ac() -> ! {
    todo!("0x5502ac RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x550400 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
pub fn stub_550400() -> ! {
    todo!("0x550400 RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x5505b8 — __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E13disconnectAllEPNS0_11EventSourceE
pub fn stub_5505b8() -> ! {
    todo!("0x5505b8 RBX::Reflection::EventDescBase<RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x550744 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10GuiService10SpecialKeyERKSsNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::GuiService::SpecialKey const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10GuiService10SpecialKeyERKSsNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
pub fn stub_550744() -> ! {
    todo!("0x550744 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::GuiService::SpecialKey const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")
}

// 0x550860 — __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_10GuiService10SpecialKeyESsEEvRKT_RKT0_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey const&,std::string const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute2INS_10GuiService10SpecialKeyESsEEvRKT_RKT0_
pub fn stub_550860() -> ! {
    todo!("0x550860 void RBX::Reflection::GenericSlotWrapper::execute2<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey const&,std::string const&)")
}

// 0x550bc0 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
pub fn stub_550bc0() -> ! {
    todo!("0x550bc0 void boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x550cb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
pub fn stub_550cb8() -> ! {
    todo!("0x550cb8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x550cd4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_SsE6invokeERNS1_15function_bufferESB_Ss
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::GuiService::SpecialKey,std::string>::invoke(boost::detail::function::function_buffer &,RBX::GuiService::SpecialKey,std::string)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_SsE6invokeERNS1_15function_bufferESB_Ss
pub fn stub_550cd4() -> ! {
    todo!("0x550cd4 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::GuiService::SpecialKey,std::string>::invoke(boost::detail::function::function_buffer &,RBX::GuiService::SpecialKey,std::string)")
}

// 0x550ce8 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_550ce8() -> ! {
    todo!("0x550ce8 bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x550dd0 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_550dd0() -> ! {
    todo!("0x550dd0 bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x550eb4 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_550eb4() -> ! {
    todo!("0x550eb4 void boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x550f88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10GuiService10SpecialKeyERKSsEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_SsEEvRT_RT0_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey &,std::string &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10GuiService10SpecialKeyERKSsEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_SsEEvRT_RT0_
pub fn stub_550f88() -> ! {
    todo!("0x550f88 void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey &,std::string &)")
}
