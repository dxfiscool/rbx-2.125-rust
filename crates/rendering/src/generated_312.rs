//! rendering shard 312 — 100 stubs 0x461efc..0x467b00 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 33681->33781 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33681 before -> 33781 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x461efc (lowest remaining 0x461efc..0x467b00, next lowest 0x467b4c)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x461efc — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel5GenreEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::Genre>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::Genre &,boost::enable_if<boost::is_enum<RBX::DataModel::Genre>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel5GenreEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_461efc() -> ! {
    todo!("0x461efc bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::Genre>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::Genre &,boost::enable_if<boost::is_enum<RBX::DataModel::Genre>,void>::type *)")
}

// 0x461f50 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EEC2EMS2_FviS3_EPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,RBX::DataModel::CreatorType),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EEC2EMS2_FviS3_EPKcS9_S9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_461f50() -> ! {
    todo!("0x461f50 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,RBX::DataModel::CreatorType),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x462118 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE16declareSignatureEPKcNS0_7VariantES7_S8_
pub fn stub_462118() -> ! {
    todo!("0x462118 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x462164 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED0Ev
pub fn stub_462164() -> ! {
    todo!("0x462164 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")
}

// 0x462244 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_462244() -> ! {
    todo!("0x462244 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x46229c — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel11CreatorTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "RBX::DataModel::CreatorType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::CreatorType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::CreatorType> const&,boost::disable_if<boost::is_same<RBX::DataModel::CreatorType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel11CreatorTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_46229c() -> ! {
    todo!("0x46229c RBX::DataModel::CreatorType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::CreatorType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::CreatorType> const&,boost::disable_if<boost::is_same<RBX::DataModel::CreatorType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x462430 — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9DataModel11CreatorTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::DataModel::CreatorType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::CreatorType &,boost::enable_if<boost::is_enum<RBX::DataModel::CreatorType>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_9DataModel11CreatorTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_462430() -> ! {
    todo!("0x462430 bool RBX::Reflection::ArgHelper::try_enum<2,RBX::DataModel::CreatorType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::CreatorType &,boost::enable_if<boost::is_enum<RBX::DataModel::CreatorType>,void>::type *)")
}

// 0x462484 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EEC2EMS2_FvibEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EEC2EMS2_FvibEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_462484() -> ! {
    todo!("0x462484 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::BoundFuncDesc(void (RBX::DataModel::*)(int,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x462680 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_462680() -> ! {
    todo!("0x462680 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x4626cc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED0Ev
pub fn stub_4626cc() -> ! {
    todo!("0x4626cc RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")
}

// 0x4627ac — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_4627ac() -> ! {
    todo!("0x4627ac RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x462800 — __ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_462800() -> ! {
    todo!("0x462800 bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x4629a8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::BoundFuncDesc(void (RBX::DataModel::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_4629a8() -> ! {
    todo!("0x4629a8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::BoundFuncDesc(void (RBX::DataModel::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x462b20 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_462b20() -> ! {
    todo!("0x462b20 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x462b50 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED0Ev
pub fn stub_462b50() -> ! {
    todo!("0x462b50 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")
}

// 0x462c24 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_462c24() -> ! {
    todo!("0x462c24 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x462c64 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EEC2EMS2_FdSsdEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::BoundFuncDesc(double (RBX::DataModel::*)(std::string,double),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EEC2EMS2_FdSsdEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_462c64() -> ! {
    todo!("0x462c64 RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::BoundFuncDesc(double (RBX::DataModel::*)(std::string,double),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x462e2c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_462e2c() -> ! {
    todo!("0x462e2c RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x462e78 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED0Ev
pub fn stub_462e78() -> ! {
    todo!("0x462e78 RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")
}

// 0x462f54 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_462f54() -> ! {
    todo!("0x462f54 RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x4630b8 — __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FdSsdESsddE4callEPS2_S4_RNS0_7VariantERKSsRKd
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,double (RBX::DataModel::*)(std::string,double),std::string,double,double>::call(RBX::DataModel*,double (RBX::DataModel::*)(std::string,double),RBX::Reflection::Variant &,std::string const&,double const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FdSsdESsddE4callEPS2_S4_RNS0_7VariantERKSsRKd
pub fn stub_4630b8() -> ! {
    todo!("0x4630b8 RBX::Reflection::Call2Helper<RBX::DataModel,double (RBX::DataModel::*)(std::string,double),std::string,double,double>::call(RBX::DataModel*,double (RBX::DataModel::*)(std::string,double),RBX::Reflection::Variant &,std::string const&,double const&)")
}

// 0x463220 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEEC2INS_9DataModelEEEPKcMT_N5boost8functionIS2_EENS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::BoundCallbackDesc<RBX::DataModel>(char const*,boost::function<bool ()(void)> RBX::DataModel::*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection17BoundCallbackDescIFbvEEC2INS_9DataModelEEEPKcMT_N5boost8functionIS2_EENS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_463220() -> ! {
    todo!("0x463220 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::BoundCallbackDesc<RBX::DataModel>(char const*,boost::function<bool ()(void)> RBX::DataModel::*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x4633a4 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EEC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EEC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE
pub fn stub_4633a4() -> ! {
    todo!("0x4633a4 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")
}

// 0x4634e4 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEED0Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")]
// was: __ZN3RBX10Reflection17BoundCallbackDescIFbvEED0Ev
pub fn stub_4634e4() -> ! {
    todo!("0x4634e4 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")
}

// 0x4635f0 — __ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")]
// was: __ZNK3RBX10Reflection16CallbackDescImplIFbvELi0EE18setGenericCallbackEPNS0_13DescribedBaseEN5boost10shared_ptrINS6_8functionIFNS7_INS0_5TupleEEENS7_IKS9_EEEEEEE
pub fn stub_4635f0() -> ! {
    todo!("0x4635f0 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::setGenericCallback(RBX::Reflection::DescribedBase *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)const")
}

// 0x463730 — __ZNK3RBX10Reflection12CallbackDescIFbvEE13clearCallbackEPNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection12CallbackDescIFbvEE13clearCallbackEPNS0_13DescribedBaseE
pub fn stub_463730() -> ! {
    todo!("0x463730 RBX::Reflection::CallbackDesc<bool ()(void)>::clearCallback(RBX::Reflection::DescribedBase *)const")
}

// 0x4637f0 — __ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// was: __ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_
pub fn stub_4637f0() -> ! {
    todo!("0x4637f0 boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::type> boost::bind<bool,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")
}

// 0x463908 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// was: __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EE11callGenericEN5boost10shared_ptrINS4_8functionIFNS5_INS0_5TupleEEENS5_IKS7_EEEEEEE
pub fn stub_463908() -> ! {
    todo!("0x463908 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::callGeneric(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")
}

// 0x463a5c — __ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
// was: __ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_
pub fn stub_463a5c() -> ! {
    todo!("0x463a5c boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>)")
}

// 0x463b98 — __ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE
// type: int(void)
#[doc(alias = "boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")]
// was: __ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE
pub fn stub_463b98() -> ! {
    todo!("0x463b98 boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(rbx_core::SharedPtr<RBX::Reflection::Tuple>)")
}

// 0x463ce8 — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
// was: __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_
pub fn stub_463ce8() -> ! {
    todo!("0x463ce8 rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

// 0x463dc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEED1Ev
pub fn stub_463dc0() -> ! {
    todo!("0x463dc0 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")
}

// 0x463dc8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE7disposeEv
pub fn stub_463dc8() -> ! {
    todo!("0x463dc8 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::dispose(void)")
}

// 0x463e70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Reflection5TupleEE19get_untyped_deleterEv
pub fn stub_463e70() -> ! {
    todo!("0x463e70 boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_untyped_deleter(void)")
}

// 0x463e74 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_
pub fn stub_463e74() -> ! {
    todo!("0x463e74 boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>)")
}

// 0x463f54 — __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_463f54() -> ! {
    todo!("0x463f54 __ZN5boost8functionIFbvEEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS0_IFNS6_IN3RBX10Reflection5TupleEEENS6_IKS9_EEEEEEEENS4_5list1INS4_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x464030 — __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_464030() -> ! {
    todo!("0x464030 __ZN5boost9function0IbEC2INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x464110 — __ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>)")]
// was: __ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_
pub fn stub_464110() -> ! {
    todo!("0x464110 void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>)")
}

// 0x464200 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_464200() -> ! {
    todo!("0x464200 boost::detail::function::functor_manager<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46421c — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEbE6invokeERNS1_15function_bufferE
pub fn stub_46421c() -> ! {
    todo!("0x46421c boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,bool>::invoke(boost::detail::function::function_buffer &)")
}

// 0x464230 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_464230() -> ! {
    todo!("0x464230 bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")
}

// 0x464310 — __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_464310() -> ! {
    todo!("0x464310 bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x464408 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l
#[doc(alias = "bool boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l
pub fn stub_464408() -> ! {
    todo!("0x464408 bool boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list0>(boost::_bi::type<bool>,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>) &,boost::_bi::list0 &,long)")
}

// 0x4644d8 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// type: int(void)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_4644d8() -> ! {
    todo!("0x4644d8 boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x46455c — __ZN3RBX10Reflection12CallbackDescIFbvEED1Ev
#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
// was: __ZN3RBX10Reflection12CallbackDescIFbvEED1Ev
pub fn stub_46455c() -> ! {
    todo!("0x46455c RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")
}

// 0x464654 — __ZN3RBX10Reflection12CallbackDescIFbvEED0Ev
#[doc(alias = "RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")]
// was: __ZN3RBX10Reflection12CallbackDescIFbvEED0Ev
pub fn stub_464654() -> ! {
    todo!("0x464654 RBX::Reflection::CallbackDesc<bool ()(void)>::~CallbackDesc()")
}

// 0x464760 — __ZN3RBX10Reflection18CallbackDescriptorD1Ev
// type: void __fastcall(RBX::Reflection::CallbackDescriptor *__hidden this)
#[doc(alias = "RBX::Reflection::CallbackDescriptor::~CallbackDescriptor()")]
// was: __ZN3RBX10Reflection18CallbackDescriptorD1Ev
pub fn stub_464760() -> ! {
    todo!("0x464760 RBX::Reflection::CallbackDescriptor::~CallbackDescriptor()")
}

// 0x464788 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEED1Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")]
// was: __ZN3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEED1Ev
pub fn stub_464788() -> ! {
    todo!("0x464788 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")
}

// 0x46478c — __ZN3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEED0Ev
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")]
// was: __ZN3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEED0Ev
pub fn stub_46478c() -> ! {
    todo!("0x46478c RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::~Setter()")
}

// 0x464790 — __ZNK3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEE11setCallbackEPNS0_13DescribedBaseERKN5boost8functionIS2_EE
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::setCallback(RBX::Reflection::DescribedBase *,boost::function<bool ()(void)> const&)const")]
// was: __ZNK3RBX10Reflection17BoundCallbackDescIFbvEE6SetterINS_9DataModelEE11setCallbackEPNS0_13DescribedBaseERKN5boost8functionIS2_EE
pub fn stub_464790() -> ! {
    todo!("0x464790 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::Setter<RBX::DataModel>::setCallback(RBX::Reflection::DescribedBase *,boost::function<bool ()(void)> const&)const")
}

// 0x4647cc — __ZN5boost8functionIFbvEEaSERKS2_
#[doc(alias = "boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)")]
// was: __ZN5boost8functionIFbvEEaSERKS2_
pub fn stub_4647cc() -> ! {
    todo!("0x4647cc boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)")
}

// 0x464890 — __ZN5boost9function0IbE4swapERS1_
#[doc(alias = "boost::function0<bool>::swap(boost::function0<bool>&)")]
// was: __ZN5boost9function0IbE4swapERS1_
pub fn stub_464890() -> ! {
    todo!("0x464890 boost::function0<bool>::swap(boost::function0<bool>&)")
}

// 0x46496c — __ZN5boost9function0IbE11move_assignERS1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function0<bool>::move_assign(boost::function0<bool>&)")]
// was: __ZN5boost9function0IbE11move_assignERS1_
pub fn stub_46496c() -> ! {
    todo!("0x46496c boost::function0<bool>::move_assign(boost::function0<bool>&)")
}

// 0x464a70 — __ZN5boost9function0IbE5clearEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::function0<bool>::clear(void)")]
// was: __ZN5boost9function0IbE5clearEv
pub fn stub_464a70() -> ! {
    todo!("0x464a70 boost::function0<bool>::clear(void)")
}

// 0x464a9c — __ZN5boost9function0IbE13assign_to_ownERKS1_
// type: int(void)
#[doc(alias = "boost::function0<bool>::assign_to_own(boost::function0<bool> const&)")]
// was: __ZN5boost9function0IbE13assign_to_ownERKS1_
pub fn stub_464a9c() -> ! {
    todo!("0x464a9c boost::function0<bool>::assign_to_own(boost::function0<bool> const&)")
}

// 0x464acc — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EED1Ev
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")]
// was: __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EED1Ev
pub fn stub_464acc() -> ! {
    todo!("0x464acc RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")
}

// 0x464bc4 — __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EED0Ev
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")]
// was: __ZN3RBX10Reflection16CallbackDescImplIFbvELi0EED0Ev
pub fn stub_464bc4() -> ! {
    todo!("0x464bc4 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::~CallbackDescImpl()")
}

// 0x464cd0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EEC2EMS2_FvN5boost8functionIFvbEEENS6_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::DataModel::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EEC2EMS2_FvN5boost8functionIFvbEEENS6_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_464cd0() -> ! {
    todo!("0x464cd0 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::DataModel::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x464dd4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED0Ev
pub fn stub_464dd4() -> ! {
    todo!("0x464dd4 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")
}

// 0x464e88 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
pub fn stub_464e88() -> ! {
    todo!("0x464e88 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x465010 — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEEbS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// was: __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEEbS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
pub fn stub_465010() -> ! {
    todo!("0x465010 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")
}

// 0x465110 — __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_465110() -> ! {
    todo!("0x465110 __ZN5boost9function1IvbEC2INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

// 0x4651e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_4651e8() -> ! {
    todo!("0x4651e8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x465208 — __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_465208() -> ! {
    todo!("0x465208 bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x4652e0 — __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_4652e0() -> ! {
    todo!("0x4652e0 bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x4653b0 — __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_4653b0() -> ! {
    todo!("0x4653b0 void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x465474 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,int>(char const*,char const*,bool (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_465474() -> ! {
    todo!("0x465474 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,int>(char const*,char const*,bool (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x465584 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbED0Ev
pub fn stub_465584() -> ! {
    todo!("0x465584 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")
}

// 0x4655b4 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE10isReadOnlyEv
pub fn stub_4655b4() -> ! {
    todo!("0x4655b4 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isReadOnly(void)const")
}

// 0x4655b8 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE11isWriteOnlyEv
pub fn stub_4655b8() -> ! {
    todo!("0x4655b8 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")
}

// 0x4655bc — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_4655bc() -> ! {
    todo!("0x4655bc RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x4655e0 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE7GetImplIMS2_KFbvEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_4655e0() -> ! {
    todo!("0x4655e0 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetImpl<bool (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x465700 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>(char const*,char const*,bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_465700() -> ! {
    todo!("0x465700 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::PropDescriptor<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>(char const*,char const*,bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x465814 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
pub fn stub_465814() -> ! {
    todo!("0x465814 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isReadOnly(void)const")
}

// 0x465818 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
pub fn stub_465818() -> ! {
    todo!("0x465818 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::isWriteOnly(void)const")
}

// 0x46581c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_46581c() -> ! {
    todo!("0x46581c RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x465840 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
pub fn stub_465840() -> ! {
    todo!("0x465840 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::GetSetImpl<bool (RBX::DataModel::*)(void)const,void (RBX::DataModel::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x465864 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EEC2EMS2_FvSsSsSsSsSsEPKcS8_S8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EEC2EMS2_FvSsSsSsSsSsEPKcS8_S8_S8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_465864() -> ! {
    todo!("0x465864 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x465b24 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_S6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_S6_S7_S6_S7_
pub fn stub_465b24() -> ! {
    todo!("0x465b24 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x465bc4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED0Ev
pub fn stub_465bc4() -> ! {
    todo!("0x465bc4 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")
}

// 0x465cb0 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_465cb0() -> ! {
    todo!("0x465cb0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x46602c — __ZN3RBX10Reflection11Call5HelperINS_9DataModelEMS2_FvSsSsSsSsSsESsSsSsSsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_SA_SA_SA_
// type: int __fastcall(int, int, int, int, std::string *, int, std::string *, std::string *, std::string *)
#[doc(alias = "RBX::Reflection::Call5Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),std::string,std::string,std::string,std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
// was: __ZN3RBX10Reflection11Call5HelperINS_9DataModelEMS2_FvSsSsSsSsSsESsSsSsSsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_SA_SA_SA_
pub fn stub_46602c() -> ! {
    todo!("0x46602c RBX::Reflection::Call5Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),std::string,std::string,std::string,std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string,std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")
}

// 0x466388 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgISsLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_466388() -> ! {
    todo!("0x466388 std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x4665dc — __ZN3RBX10Reflection9ArgHelper6getArgISsLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgISsLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
pub fn stub_4665dc() -> ! {
    todo!("0x4665dc std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0x466830 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_466830() -> ! {
    todo!("0x466830 RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x466934 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
pub fn stub_466934() -> ! {
    todo!("0x466934 RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x4669e8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_4669e8() -> ! {
    todo!("0x4669e8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x466a0c — __ZN3RBX10Reflection11Call0HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::DataModel*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),RBX::Reflection::Variant&)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_9DataModelEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
pub fn stub_466a0c() -> ! {
    todo!("0x466a0c RBX::Reflection::Call0Helper<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::DataModel*,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::DataModel::*)(void),RBX::Reflection::Variant&)")
}

// 0x466af8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EEC2EMS2_FSsSsSsbEPKcS8_S8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::BoundFuncDesc(std::string (RBX::DataModel::*)(std::string,std::string,bool),char const*,char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EEC2EMS2_FSsSsSsbEPKcS8_S8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_466af8() -> ! {
    todo!("0x466af8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::BoundFuncDesc(std::string (RBX::DataModel::*)(std::string,std::string,bool),char const*,char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x466d48 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_
pub fn stub_466d48() -> ! {
    todo!("0x466d48 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x466db0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED0Ev
pub fn stub_466db0() -> ! {
    todo!("0x466db0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")
}

// 0x466e94 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_466e94() -> ! {
    todo!("0x466e94 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x467084 — __ZN3RBX10Reflection11Call3HelperINS_9DataModelEMS2_FSsSsSsbESsSsbSsE4callEPS2_S4_RNS0_7VariantERKSsSA_RKb
// type: int __fastcall(int, int, int, int, std::string *, int, int)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::DataModel,std::string (RBX::DataModel::*)(std::string,std::string,bool),std::string,std::string,bool,std::string>::call(RBX::DataModel*,std::string (RBX::DataModel::*)(std::string,std::string,bool),RBX::Reflection::Variant &,std::string const&,std::string const&,bool const&)")]
// was: __ZN3RBX10Reflection11Call3HelperINS_9DataModelEMS2_FSsSsSsbESsSsbSsE4callEPS2_S4_RNS0_7VariantERKSsSA_RKb
pub fn stub_467084() -> ! {
    todo!("0x467084 RBX::Reflection::Call3Helper<RBX::DataModel,std::string (RBX::DataModel::*)(std::string,std::string,bool),std::string,std::string,bool,std::string>::call(RBX::DataModel*,std::string (RBX::DataModel::*)(std::string,std::string,bool),RBX::Reflection::Variant &,std::string const&,std::string const&,bool const&)")
}

// 0x4672e4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EEC2EMS2_FSsSsbEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::BoundFuncDesc(std::string (RBX::DataModel::*)(std::string,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EEC2EMS2_FSsSsbEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_4672e4() -> ! {
    todo!("0x4672e4 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::BoundFuncDesc(std::string (RBX::DataModel::*)(std::string,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x4674e0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_4674e0() -> ! {
    todo!("0x4674e0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x46752c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED0Ev
pub fn stub_46752c() -> ! {
    todo!("0x46752c RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")
}

// 0x467608 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_467608() -> ! {
    todo!("0x467608 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x467768 — __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FSsSsbESsbSsE4callEPS2_S4_RNS0_7VariantERKSsRKb
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,std::string (RBX::DataModel::*)(std::string,bool),std::string,bool,std::string>::call(RBX::DataModel*,std::string (RBX::DataModel::*)(std::string,bool),RBX::Reflection::Variant &,std::string const&,bool const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FSsSsbESsbSsE4callEPS2_S4_RNS0_7VariantERKSsRKb
pub fn stub_467768() -> ! {
    todo!("0x467768 RBX::Reflection::Call2Helper<RBX::DataModel,std::string (RBX::DataModel::*)(std::string,bool),std::string,bool,std::string>::call(RBX::DataModel*,std::string (RBX::DataModel::*)(std::string,bool),RBX::Reflection::Variant &,std::string const&,bool const&)")
}

// 0x467938 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EEC2EMS2_FvSsSsN5boost8functionIFvSsEEES8_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EEC2EMS2_FvSsSsN5boost8functionIFvSsEEES8_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_467938() -> ! {
    todo!("0x467938 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x467b00 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_467b00() -> ! {
    todo!("0x467b00 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}
