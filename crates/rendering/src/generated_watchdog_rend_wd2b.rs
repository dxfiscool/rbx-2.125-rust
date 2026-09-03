//! Generated watchdog rendering wd2b — 120 stubs Material/Compositor/PostProcess (Ogre) global dedup EA-sorted asc
//! Source: ida/export.json (85545 funcs) filtered Ogre+Material|Compositor|PostProcess (477 total, 477 already stubbed, 0 candidates) fallback to 120 lowest EA unused
//! Range: 0x7c6b08..0x88dd2c (120 stubs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x7c6b08 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev")]
// IDA 0x7c6b08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c6b08() {
}


// 0x7c6c18 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev")]
// IDA 0x7c6c18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c6c18() {
}


// 0x7c6e48 — __ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_")]
// IDA 0x7c6e48: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c6e48() {
}


// 0x7c6e78 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Humanoid::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EEC2EMS2_FSA_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c6e78: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c6e78() {
}


// 0x7c6f7c — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED0Ev")]
// IDA 0x7c6f7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c6f7c() {
}


// 0x7c7030 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7c7030: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7030() {
}


// 0x7c7054 — __ZN3RBX10Reflection11Call0HelperINS_8HumanoidEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Humanoid,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Humanoid::*)(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::call(RBX::Humanoid*,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> (RBX::Humanoid::*)(void),RBX::Reflection::Variant&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_8HumanoidEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_")]
// IDA 0x7c7054: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7054() {
}


// 0x7c713c — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(void),0>::BoundFuncDesc(void (RBX::Humanoid::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c713c: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c713c() {
}


// 0x7c7240 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvvELi0EED0Ev")]
// IDA 0x7c7240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c7240() {
}


// 0x7c72f4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7c72f4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c72f4() {
}


// 0x7c7314 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Humanoid::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c7314: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7314() {
}


// 0x7c7490 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7c7490: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7490() {
}


// 0x7c74c0 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")]
// IDA 0x7c74c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c74c0() {
}


// 0x7c75c8 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7c75c8: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c75c8() {
}


// 0x7c76ac — __ZN3RBX10Reflection11Call1HelperINS_8HumanoidEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Humanoid,void (RBX::Humanoid::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::Humanoid*,void (RBX::Humanoid::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_8HumanoidEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")]
// IDA 0x7c76ac: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c76ac() {
}


// 0x7c7794 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(std::string),1>::BoundFuncDesc(bool (RBX::Humanoid::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EEC2EMS2_FbSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c7794: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7794() {
}


// 0x7c7910 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7c7910: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7910() {
}


// 0x7c7940 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EED0Ev")]
// IDA 0x7c7940: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c7940() {
}


// 0x7c7a48 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7c7a48: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7a48() {
}


// 0x7c7b88 — __ZN3RBX10Reflection11Call1HelperINS_8HumanoidEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Humanoid,bool (RBX::Humanoid::*)(std::string),std::string,bool>::call(RBX::Humanoid*,bool (RBX::Humanoid::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_8HumanoidEMS2_FbSsESsbE4callEPS2_S4_RNS0_7VariantERKSs")]
// IDA 0x7c7b88: 116 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7b88() {
}


// 0x7c7cdc — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EEC2EMS2_FbS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(RBX::Humanoid::Status),1>::BoundFuncDesc(bool (RBX::Humanoid::*)(RBX::Humanoid::Status),char const*,char const*,RBX::Humanoid::Status,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EEC2EMS2_FbS3_EPKcS9_S3_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c7cdc: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7cdc() {
}


// 0x7c7e88 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(RBX::Humanoid::Status),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7c7e88: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7e88() {
}


// 0x7c7eb8 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(RBX::Humanoid::Status),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EED0Ev")]
// IDA 0x7c7eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c7eb8() {
}


// 0x7c7f8c — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,bool ()(RBX::Humanoid::Status),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFbNS2_6StatusEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7c7f8c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7f8c() {
}


// 0x7c7fcc — __ZN3RBX10Reflection11Call1HelperINS_8HumanoidEMS2_FbNS2_6StatusEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Humanoid,bool (RBX::Humanoid::*)(RBX::Humanoid::Status),RBX::Humanoid::Status,bool>::call(RBX::Humanoid*,bool (RBX::Humanoid::*)(RBX::Humanoid::Status),RBX::Reflection::Variant &,RBX::Humanoid::Status const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_8HumanoidEMS2_FbNS2_6StatusEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_")]
// IDA 0x7c7fcc: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c7fcc() {
}


// 0x7c8004 — __ZN3RBX10Reflection9ArgHelper6getArgINS_8Humanoid6StatusELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Humanoid::Status RBX::Reflection::ArgHelper::getArg<RBX::Humanoid::Status,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Humanoid::Status> const&,boost::disable_if<boost::is_same<RBX::Humanoid::Status,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_8Humanoid6StatusELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// IDA 0x7c8004: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8004() {
}


// 0x7c8194 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8Humanoid6StatusEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::Humanoid::Status>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Humanoid::Status &,boost::enable_if<boost::is_enum<RBX::Humanoid::Status>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_8Humanoid6StatusEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
// IDA 0x7c8194: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8194() {
}


// 0x7c81e8 — __ZN3RBX10Reflection9EventDescINS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Humanoid,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Humanoid::*>::EventDesc(rbx::signal<void ()(bool)> RBX::Humanoid::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c81e8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c81e8() {
}


// 0x7c836c — __ZN3RBX10Reflection9EventDescINS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Humanoid,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Humanoid::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
// IDA 0x7c836c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c836c() {
}


// 0x7c8420 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Humanoid,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Humanoid::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x7c8420: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8420() {
}


// 0x7c8574 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Humanoid,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Humanoid::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
// IDA 0x7c8574: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8574() {
}


// 0x7c8600 — __ZNK3RBX10Reflection13EventDescBaseINS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Humanoid,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Humanoid::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8HumanoidEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x7c8600: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8600() {
}


// 0x7c8614 — __ZN3RBX10Reflection9EventDescINS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Humanoid,void ()(float),rbx::signal<void ()(float)>,rbx::signal<void ()(float)> RBX::Humanoid::*>::EventDesc(rbx::signal<void ()(float)> RBX::Humanoid::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7c8614: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8614() {
}


// 0x7c8798 — __ZN3RBX10Reflection9EventDescINS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Humanoid,void ()(float),rbx::signal<void ()(float)>,rbx::signal<void ()(float)> RBX::Humanoid::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
// IDA 0x7c8798: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c8798() {
}


// 0x7c884c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Humanoid,void ()(float),rbx::signal<void ()(float)>,rbx::signal<void ()(float)> RBX::Humanoid::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x7c884c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c884c() {
}


// 0x7c89a0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Humanoid,void ()(float),rbx::signal<void ()(float)>,rbx::signal<void ()(float)> RBX::Humanoid::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
// IDA 0x7c89a0: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c89a0() {
}


// 0x7c8a2c — __ZNK3RBX10Reflection13EventDescBaseINS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Humanoid,void ()(float),rbx::signal<void ()(float)>,rbx::signal<void ()(float)> RBX::Humanoid::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8HumanoidEFvfEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x7c8a2c: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8a2c() {
}


// 0x7c8a40 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKfNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_")]
// IDA 0x7c8a40: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8a40() {
}


// 0x7c8b5c — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IfEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<float>(float const&)")]
#[doc(alias = "__ZN3RBX10Reflection18GenericSlotWrapper8execute1IfEEvRKT_")]
// IDA 0x7c8b5c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8b5c() {
}


// 0x7c8ca0 — __ZN5boost9function1IvfE5clearEv
#[doc(alias = "boost::function1<void,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvfE5clearEv")]
// IDA 0x7c8ca0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8ca0() {
}


// 0x7c8ccc — __ZN5boost8functionIFvfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvfEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// IDA 0x7c8ccc: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8ccc() {
}


// 0x7c8db0 — __ZN5boost9function1IvfEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvfEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvfEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// IDA 0x7c8db0: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8db0() {
}


// 0x7c8e98 — __ZN5boost9function1IvfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_")]
// IDA 0x7c8e98: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8e98() {
}


// 0x7c8f90 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
// IDA 0x7c8f90: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8f90() {
}


// 0x7c8fac — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvfE6invokeERNS1_15function_bufferEf
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,float>::invoke(boost::detail::function::function_buffer &,float)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvfE6invokeERNS1_15function_bufferEf")]
// IDA 0x7c8fac: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8fac() {
}


// 0x7c8fc0 — __ZNK5boost6detail8function13basic_vtable1IvfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// IDA 0x7c8fc0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c8fc0() {
}


// 0x7c90a8 — __ZNK5boost6detail8function13basic_vtable1IvfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvfE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// IDA 0x7c90a8: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c90a8() {
}


// 0x7c918c — __ZNK5boost6detail8function13basic_vtable1IvfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvfE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// IDA 0x7c918c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c918c() {
}


// 0x7c9260 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIfEEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<float>(float &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIfEEvRT_")]
// IDA 0x7c9260: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9260() {
}


// 0x7c9278 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,float const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// IDA 0x7c9278: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9278() {
}


// 0x7c93d0 — __ZN3rbx7signals6signalIFvfEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::function<void ()(float)>>(boost::function<void ()(float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
// IDA 0x7c93d0: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c93d0() {
}


// 0x7c94c4 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::callable<rbx::signals::signal<void ()(float)>*>(boost::function<void ()(float)> const&,rbx::signals::signal<void ()(float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")]
// IDA 0x7c94c4: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c94c4() {
}


// 0x7c95c0 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::function<void ()(float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
// IDA 0x7c95c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c95c0() {
}


// 0x7c96d0 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::function<void ()(float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
// IDA 0x7c96d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c96d0() {
}


// 0x7c9800 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::call(float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf")]
// IDA 0x7c9800: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9800() {
}


// 0x7c9808 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::call(float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_E4callEf")]
// IDA 0x7c9808: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9808() {
}


// 0x7c9810 — __ZNK5boost9function1IvfEclEf
#[doc(alias = "boost::function1<void,float>::operator()(float)const")]
#[doc(alias = "__ZNK5boost9function1IvfEclEf")]
// IDA 0x7c9810: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9810() {
}


// 0x7c98d8 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev")]
// IDA 0x7c98d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c98d8() {
}


// 0x7c99e8 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev")]
// IDA 0x7c99e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c99e8() {
}


// 0x7c9b18 — __ZN5boost9function1IvfE13assign_to_ownERKS1_
#[doc(alias = "boost::function1<void,float>::assign_to_own(boost::function1<void,float> const&)")]
#[doc(alias = "__ZN5boost9function1IvfE13assign_to_ownERKS1_")]
// IDA 0x7c9b18: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9b18() {
}


// 0x7c9b48 — __ZN3RBX10Reflection9EventDescINS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Humanoid,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Humanoid::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
// IDA 0x7c9b48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7c9b48() {
}


// 0x7c9bfc — __ZNK3RBX10Reflection13EventDescImplILi0ENS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Humanoid,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Humanoid::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x7c9bfc: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9bfc() {
}


// 0x7c9e00 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Humanoid,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Humanoid::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
// IDA 0x7c9e00: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9e00() {
}


// 0x7c9e74 — __ZNK3RBX10Reflection13EventDescBaseINS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Humanoid,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Humanoid::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_8HumanoidEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x7c9e74: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7c9e74() {
}


// 0x7ca3b8 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(bool),1>::BoundFuncDesc(void (RBX::Humanoid::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7ca3b8: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca3b8() {
}


// 0x7ca530 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7ca530: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca530() {
}


// 0x7ca560 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EED0Ev")]
// IDA 0x7ca560: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7ca560() {
}


// 0x7ca634 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7ca634: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca634() {
}


// 0x7ca668 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(float),1>::BoundFuncDesc(void (RBX::Humanoid::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EEC2EMS2_FvfEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x7ca668: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca668() {
}


// 0x7ca7e0 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x7ca7e0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca7e0() {
}


// 0x7ca810 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(float),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EED0Ev")]
// IDA 0x7ca810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7ca810() {
}


// 0x7ca8e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x7ca8e4: 21 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca8e4() {
}


// 0x7ca920 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEfEC2IMS2_KFfvEMS2_FvRKfEEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::PropDescriptor<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float const&)>(char const*,char const*,float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEfEC2IMS2_KFfvEMS2_FvRKfEEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x7ca920: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7ca920() {
}


// 0x7caa34 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEfED0Ev")]
// IDA 0x7caa34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7caa34() {
}


// 0x7caa60 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE10isReadOnlyEv")]
// IDA 0x7caa60: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7caa60() {
}


// 0x7caa64 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE11isWriteOnlyEv")]
// IDA 0x7caa64: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7caa64() {
}


// 0x7caa68 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x7caa68: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7caa68() {
}


// 0x7caa88 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float const&)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvRKfEE8setValueEPNS0_13DescribedBaseES8_")]
// IDA 0x7caa88: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7caa88() {
}


// 0x7cac40 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,bool>::PropDescriptor<bool (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(bool)>(char const*,char const*,bool (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x7cac40: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cac40() {
}


// 0x7cad54 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEbED0Ev")]
// IDA 0x7cad54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7cad54() {
}


// 0x7cad80 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,bool>::GetSetImpl<bool (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
// IDA 0x7cad80: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cad80() {
}


// 0x7cad84 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,bool>::GetSetImpl<bool (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
// IDA 0x7cad84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cad84() {
}


// 0x7cad88 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,bool>::GetSetImpl<bool (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x7cad88: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cad88() {
}


// 0x7cadac — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,bool>::GetSetImpl<bool (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
// IDA 0x7cadac: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cadac() {
}


// 0x7caf44 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::PropDescriptor<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float)>(char const*,char const*,float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x7caf44: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7caf44() {
}


// 0x7cb058 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
// IDA 0x7cb058: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb058() {
}


// 0x7cb05c — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
// IDA 0x7cb05c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb05c() {
}


// 0x7cb060 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x7cb060: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb060() {
}


// 0x7cb080 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,float>::GetSetImpl<float (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
// IDA 0x7cb080: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb080() {
}


// 0x7cb0a4 — __ZN3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x7cb0a4: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb0a4() {
}


// 0x7cb148 — __ZN3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEED0Ev")]
// IDA 0x7cb148: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x7cb148() {
}


// 0x7cb178 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10isReadOnlyEv")]
// IDA 0x7cb178: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb178() {
}


// 0x7cb188 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11isWriteOnlyEv")]
// IDA 0x7cb188: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb188() {
}


// 0x7cb198 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_")]
// IDA 0x7cb198: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb198() {
}


// 0x7cb1c0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// IDA 0x7cb1c0: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb1c0() {
}


// 0x7cb2d8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// IDA 0x7cb2d8: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb2d8() {
}


// 0x7cb3a0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
// IDA 0x7cb3a0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb3a0() {
}


// 0x7cb3c4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0x7cb3c4: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb3c4() {
}


// 0x7cb498 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0x7cb498: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb498() {
}


// 0x7cb4bc — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE")]
// IDA 0x7cb4bc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb4bc() {
}


// 0x7cb4d0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_")]
// IDA 0x7cb4d0: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb4d0() {
}


// 0x7cb54c — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")]
// IDA 0x7cb54c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb54c() {
}


// 0x7cb56c — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// IDA 0x7cb56c: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb56c() {
}


// 0x7cb64c — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Humanoid,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
// IDA 0x7cb64c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb64c() {
}


// 0x7cb654 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
// IDA 0x7cb654: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb654() {
}


// 0x7cb658 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
// IDA 0x7cb658: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb658() {
}


// 0x7cb65c — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x7cb65c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb65c() {
}


// 0x7cb67c — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// IDA 0x7cb67c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb67c() {
}


// 0x7cb7ac — __ZN3RBX8Instance25HumanoidChangedSignalDataD1Ev
// type: void __fastcall(RBX::Instance::HumanoidChangedSignalData *__hidden this)
#[doc(alias = "RBX::Instance::HumanoidChangedSignalData::~HumanoidChangedSignalData()")]
#[doc(alias = "__ZN3RBX8Instance25HumanoidChangedSignalDataD1Ev")]
// IDA 0x7cb7ac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x7cb7ac() {
}


// 0x7cb7b0 — __ZN3RBX8Instance25HumanoidChangedSignalDataD0Ev
// type: void __fastcall(RBX::Instance::HumanoidChangedSignalData *__hidden this)
#[doc(alias = "RBX::Instance::HumanoidChangedSignalData::~HumanoidChangedSignalData()")]
#[doc(alias = "__ZN3RBX8Instance25HumanoidChangedSignalDataD0Ev")]
// IDA 0x7cb7b0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x7cb7b0() {
}


// 0x7cb7b4 — ___cxx_global_array_dtor
#[doc(alias = "___cxx_global_array_dtor")]
#[doc(alias = "___cxx_global_array_dtor")]
// IDA 0x7cb7b4: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x7cb7b4() {
}


// 0x7cb890 — __GLOBAL__I_a_376
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "global constructor keyed to_a_376")]
#[doc(alias = "__GLOBAL__I_a_376")]
// IDA 0x7cb890: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_0x7cb890() {
}


// 0x88da34 — __ZN5boost10shared_ptrIN3RBX13PluginManagerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::PluginManager>::shared_ptr<RBX::PluginManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13PluginManagerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x88da34: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88da34() {
}


// 0x88dafc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13PluginManagerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PluginManager,RBX::PluginManager>(boost::shared_ptr<RBX::PluginManager> const*,RBX::PluginManager *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13PluginManagerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x88dafc: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88dafc() {
}


// 0x88dbe4 — __ZN5boost6detail12shared_countC2IPN3RBX13PluginManagerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX13PluginManagerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x88dbe4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88dbe4() {
}


// 0x88dcec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x88dcec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x88dcec() {
}


// 0x88dcf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x88dcf0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x88dcf0() {
}


// 0x88dcf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x88dcf4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88dcf4() {
}


// 0x88dd14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x88dd14: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88dd14() {
}


// 0x88dd2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PluginManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13PluginManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x88dd2c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88dd2c() {
}
