//! rendering shard 250 — 100 stubs EA-sorted asc global gap filler after 0xee22f0 not yet in rendering (Ogre|G3D 13663/13663 complete, 27020->27120 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2bb9d0 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bb9d0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bb9d0() {
}

// 0x2bba10 — __ZN3RBX10Reflection11Call1HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEbEbS7_E4callEPS2_S9_RNS0_7VariantERKb
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),bool,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ScriptContext::*)(bool),RBX::Reflection::Variant &,bool const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_13ScriptContextEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEbEbS7_E4callEPS2_S9_RNS0_7VariantERKb
// IDA 0x2bba10: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bba10() {
}

// 0x2bbafc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(double),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EEC2EMS2_FvdEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bbafc: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bbafc() {
}

// 0x2bbc74 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x2bbc74: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bbc74() {
}

// 0x2bbca4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EED0Ev
// IDA 0x2bbca4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bbca4() {
}

// 0x2bbd78 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvdELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bbd78: 21 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bbd78() {
}

// 0x2bbdb8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13ScriptContextEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ScriptContext>(char const*,char const*,bool RBX::ScriptContext::*,void (RBX::ScriptContext::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13ScriptContextEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
// IDA 0x2bbdb8: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bbdb8() {
}

// 0x2bbf50 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EED0Ev
// IDA 0x2bbf50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bbf50() {
}

// 0x2bbf80 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x2bbf80: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bbf80() {
}

// 0x2bbfa8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x2bbfa8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bbfa8() {
}

// 0x2bc0f8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE9copyValueEPKNS0_13DescribedBaseEPS3_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE9copyValueEPKNS0_13DescribedBaseEPS3_
// IDA 0x2bc0f8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc0f8() {
}

// 0x2bc120 — __ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "bool const& rbx::any_cast<bool const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x2bc120: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc120() {
}

// 0x2bc208 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<bool>(bool const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_
// IDA 0x2bc208: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc208() {
}

// 0x2bc244 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE10isReadOnlyEv
// IDA 0x2bc244: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc244() {
}

// 0x2bc248 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE11isWriteOnlyEv
// IDA 0x2bc248: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc248() {
}

// 0x2bc24c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x2bc24c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc24c() {
}

// 0x2bc258 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ScriptContext>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_13ScriptContextEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x2bc258: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc258() {
}

// 0x2bc2a8 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::BoundFuncDesc(void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EEC2EMS2_FviS6_SsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bc2a8: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc2a8() {
}

// 0x2bc4c4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
// IDA 0x2bc4c4: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc4c4() {
}

// 0x2bc530 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED0Ev
// IDA 0x2bc530: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bc530() {
}

// 0x2bc658 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bc658: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc658() {
}

// 0x2bc7f0 — __ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),int,rbx_core::SharedPtr<RBX::Instance>,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(int,rbx_core::SharedPtr<RBX::Instance>,std::string),RBX::Reflection::Variant &,int const&,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&)")]
// was: __ZN3RBX10Reflection11Call3HelperINS_13ScriptContextEMS2_FviN5boost10shared_ptrINS_8InstanceEEESsEiS6_SsvE4callEPS2_S8_RNS0_7VariantERKiRKS6_RKSs
// IDA 0x2bc7f0: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc7f0() {
}

// 0x2bc988 — __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<RBX::Instance>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgIN5boost10shared_ptrINS_8InstanceEEELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKNS3_10scoped_ptrIS7_EEPNS3_10disable_ifINS3_7is_sameIS7_NS4_IKNS0_5TupleEEEEEvE4typeE
// IDA 0x2bc988: 206 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bc988() {
}

// 0x2bcba0 — __ZN3RBX10Reflection9ArgHelper10try_objectILi2ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_object<2,RBX::Instance>(RBX::Reflection::FunctionDescriptor::Arguments &,rbx_core::SharedPtr<RBX::Instance> &,boost::enable_if<boost::is_base_of<RBX::Reflection::DescribedBase,RBX::Instance>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper10try_objectILi2ENS_8InstanceEEEbRNS0_18FunctionDescriptor9ArgumentsERN5boost10shared_ptrIT0_EEPNS7_9enable_ifINS7_10is_base_ofINS0_13DescribedBaseES9_EEvE4typeE
// IDA 0x2bcba0: 107 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bcba0() {
}

// 0x2bccdc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::BoundFuncDesc(void (RBX::ScriptContext::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bccdc: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bccdc() {
}

// 0x2bcde0 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvvELi0EED0Ev
// IDA 0x2bcde0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bcde0() {
}

// 0x2bce94 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bce94: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bce94() {
}

// 0x2bceb4 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvSsS6_EPKcSC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bceb4: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bceb4() {
}

// 0x2bd080 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESA_SB_
// IDA 0x2bd080: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bd080() {
}

// 0x2bd0cc — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
// IDA 0x2bd0cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bd0cc() {
}

// 0x2bd1e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bd1e4: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bd1e4() {
}

// 0x2bd368 — __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),std::string,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsN5boost10shared_ptrINS_8InstanceEEEESsS6_vE4callEPS2_S8_RNS0_7VariantERKSsRKS6_
// IDA 0x2bd368: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bd368() {
}

// 0x2bd4f0 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: __ZN3RBX10Reflection9ArgHelper6getArgISsLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// IDA 0x2bd4f0: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bd4f0() {
}

// 0x2bd744 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EEC2EMS2_FvSsSsEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::ScriptContext::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EEC2EMS2_FvSsSsEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bd744: 177 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bd744() {
}

// 0x2bd90c — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// IDA 0x2bd90c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bd90c() {
}

// 0x2bd958 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EED0Ev
// IDA 0x2bd958: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bd958() {
}

// 0x2bda2c — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2bda2c: 160 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bda2c() {
}

// 0x2bdbf8 — __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsSsESsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::ScriptContext,void (RBX::ScriptContext::*)(std::string,std::string),std::string,std::string,void>::call(RBX::ScriptContext*,void (RBX::ScriptContext::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&)")]
// was: __ZN3RBX10Reflection11Call2HelperINS_13ScriptContextEMS2_FvSsSsESsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_
// IDA 0x2bdbf8: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bdbf8() {
}

// 0x2bddc0 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::BoundFuncDesc(void (RBX::ScriptContext::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2bddc0: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bddc0() {
}

// 0x2bdf38 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x2bdf38: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bdf38() {
}

// 0x2bdf68 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EED0Ev
// IDA 0x2bdf68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bdf68() {
}

// 0x2be03c — __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x2be03c: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be03c() {
}

// 0x2be070 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_SE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x2be070: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be070() {
}

// 0x2be2cc — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED0Ev
// IDA 0x2be2cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2be2cc() {
}

// 0x2be380 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x2be380: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be380() {
}

// 0x2be4d4 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// IDA 0x2be4d4: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be4d4() {
}

// 0x2be728 — __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x2be728: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be728() {
}

// 0x2be73c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13disconnectAllEv
// IDA 0x2be73c: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be73c() {
}

// 0x2be8b4 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EEclES6_SsS6_
// type: void __fastcall(_DWORD *, shared_count *, std::string *, shared_count *, int, int, boost::detail::sp_counted_base *, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EEclES6_SsS6_
// IDA 0x2be8b4: 150 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2be8b4() {
}

// 0x2beb34 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// IDA 0x2beb34: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2beb34() {
}

// 0x2bec94 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8fireItemEPNS0_6signalIS7_E4slotES6_SsS6_
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8fireItemEPNS0_6signalIS7_E4slotES6_SsS6_
// IDA 0x2bec94: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bec94() {
}

// 0x2bee54 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE8on_errorERSt9exception
// IDA 0x2bee54: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bee54() {
}

// 0x2bee7c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsS8_NS4_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsS8_NS4_IS3_EENS_3argILi1EEENSC_ILi2EEENSC_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISI_T0_T1_T2_T3_EENSG_9list_av_4IT4_T5_T6_T7_E4typeEEEMSL_FSI_SM_SN_SO_ESR_SS_ST_SU_
// IDA 0x2bee7c: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bee7c() {
}

// 0x2bef98 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsS6_EEvRKT_RKT0_RKT1_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsS6_EEvRKT_RKT0_RKT1_
// IDA 0x2bef98: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bef98() {
}

// 0x2bf124 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E5clearEv
#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E5clearEv
// IDA 0x2bf124: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf124() {
}

// 0x2bf150 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// IDA 0x2bf150: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf150() {
}

// 0x2bf234 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// IDA 0x2bf234: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf234() {
}

// 0x2bf31c — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEEvT_
// IDA 0x2bf31c: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf31c() {
}

// 0x2bf414 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
// IDA 0x2bf414: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf414() {
}

// 0x2bf430 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSC_SsSC_E6invokeERNS1_15function_bufferESC_SsSC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEvSC_SsSC_E6invokeERNS1_15function_bufferESC_SsSC_
// IDA 0x2bf430: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf430() {
}

// 0x2bf450 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2bf450: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf450() {
}

// 0x2bf538 — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2bf538: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf538() {
}

// 0x2bf61c — __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvNS_10shared_ptrIN3RBX8InstanceEEESsS6_E14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvNS4_10Reflection18GenericSlotWrapperERKS6_RKSsSG_EENS9_5list4INS9_5valueINS3_ISE_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2bf61c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf61c() {
}

// 0x2bf6f0 — __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsSK_EENS0_5list3IRSI_RSsSP_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueINS_10shared_ptrIN3RBX10Reflection18GenericSlotWrapperEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf3IvS6_RKNS3_INS4_8InstanceEEERKSsSK_EENS0_5list3IRSI_RSsSP_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2bf6f0: 14 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf6f0() {
}

// 0x2bf718 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKSsSE_EENS3_5list4INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2bf718: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf718() {
}

// 0x2bf870 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
// IDA 0x2bf870: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf870() {
}

// 0x2bf964 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_EC2IPS9_EERKSC_T_
// IDA 0x2bf964: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bf964() {
}

// 0x2bfa60 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED1Ev
// IDA 0x2bfa60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bfa60() {
}

// 0x2bfb70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS6_EE13callable_slotINS2_8functionIS7_EEED0Ev
// IDA 0x2bfb70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bfb70() {
}

// 0x2bfca0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_
// IDA 0x2bfca0: 153 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bfca0() {
}

// 0x2bfe48 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_E4callES7_SsS7_
// IDA 0x2bfe48: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bfe48() {
}

// 0x2bfe50 — __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EclES4_SsS4_
#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EclES4_SsS4_
// IDA 0x2bfe50: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2bfe50() {
}

// 0x2c0038 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED1Ev
// IDA 0x2c0038: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2c0038() {
}

// 0x2c0148 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsS7_EE4slotENS3_8functionIS8_EELi3ES8_ED0Ev
// IDA 0x2c0148: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2c0148() {
}

// 0x2c0278 — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E13assign_to_ownERKS5_
#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_E13assign_to_ownERKS5_
// IDA 0x2c0278: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0278() {
}

// 0x2c02a8 — __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// was: __ZN5boost14singleton_poolIN3RBX12PartInstance20OnDemandPartInstanceELj200ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// IDA 0x2c02a8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c02a8() {
}

// 0x2c04b0 — __ZN3RBX20RuntimeScriptServiceD2Ev
// type: void __fastcall(RBX::RuntimeScriptService *__hidden this)
#[doc(alias = "RBX::RuntimeScriptService::~RuntimeScriptService()")]
// was: __ZN3RBX20RuntimeScriptServiceD2Ev
// IDA 0x2c04b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2c04b0() {
}

// 0x2c090c — __ZN11LuaProfilerD2Ev
// type: void __fastcall(LuaProfiler *__hidden this)
#[doc(alias = "LuaProfiler::~LuaProfiler()")]
// was: __ZN11LuaProfilerD2Ev
// IDA 0x2c090c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2c090c() {
}

// 0x2c09a4 — __ZN11LuaProfilerC2EP9lua_Statei
#[doc(alias = "LuaProfiler::LuaProfiler(lua_State *,int)")]
// was: __ZN11LuaProfilerC2EP9lua_Statei
// IDA 0x2c09a4: 211 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c09a4() {
}

// 0x2c0c60 — __ZN11LuaProfiler17getResumePositionEP9lua_Statei
#[doc(alias = "LuaProfiler::getResumePosition(lua_State *,int)")]
// was: __ZN11LuaProfiler17getResumePositionEP9lua_Statei
// IDA 0x2c0c60: 102 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0c60() {
}

// 0x2c0d9c — __ZN11LuaProfiler11StringCache8getBeginERKSs
// type: _DWORD __fastcall(LuaProfiler::StringCache *__hidden this, const std::string *)
#[doc(alias = "LuaProfiler::StringCache::getBegin(std::string const&)")]
// was: __ZN11LuaProfiler11StringCache8getBeginERKSs
// IDA 0x2c0d9c: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0d9c() {
}

// 0x2c0edc — __ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::push_back(char const* const&)")]
// was: __ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_
// IDA 0x2c0edc: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_2c0edc() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x2c0f08 — __ZN11LuaProfiler11StringCache7getCallEPKcS2_S2_i
// type: _DWORD __fastcall(LuaProfiler::StringCache *__hidden this, const char *, const char *, const char *, int)
#[doc(alias = "LuaProfiler::StringCache::getCall(char const*,char const*,char const*,int)")]
// was: __ZN11LuaProfiler11StringCache7getCallEPKcS2_S2_i
// IDA 0x2c0f08: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0f08() {
}

// 0x2c10cc — __ZNSt3mapIN11LuaProfiler11StringCache8FunctionESsSt4lessIS2_ESaISt4pairIKS2_SsEEEixERS6_
#[doc(alias = "std::map<LuaProfiler::StringCache::Function,std::string,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::operator[](LuaProfiler::StringCache::Function const&)")]
// was: __ZNSt3mapIN11LuaProfiler11StringCache8FunctionESsSt4lessIS2_ESaISt4pairIKS2_SsEEEixERS6_
// IDA 0x2c10cc: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c10cc() {
}

// 0x2c12ac — __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
// was: __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// IDA 0x2c12ac: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c12ac() {
}

// 0x2c138c — __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
// was: __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// IDA 0x2c138c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c138c() {
}

// 0x2c13dc — __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert_unique(std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
// was: __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// IDA 0x2c13dc: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c13dc() {
}

// 0x2c145c — __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_create_node(std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
// was: __ZNSt8_Rb_treeIN11LuaProfiler11StringCache8FunctionESt4pairIKS2_SsESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE14_M_create_nodeERKS5_
// IDA 0x2c145c: 82 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c145c() {
}

// 0x2c154c — __ZNK11LuaProfiler11StringCache8FunctionltERKS1_
#[doc(alias = "LuaProfiler::StringCache::Function::operator<(LuaProfiler::StringCache::Function const&)const")]
// was: __ZNK11LuaProfiler11StringCache8FunctionltERKS1_
// IDA 0x2c154c: 23 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c154c() {
}

// 0x2c157c — __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<char const**,std::vector<char const*,std::allocator<char const*>>>,char const* const&)")]
// was: __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// IDA 0x2c157c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_2c157c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x2c165c — __ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<char const*,std::allocator<char const*>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm
// IDA 0x2c165c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_2c165c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x2c1674 — __ZNSt4pairIKSsSsEC2ERS0_S2_
#[doc(alias = "std::pair<std::string const,std::string>::pair(std::string const&,std::string const&)")]
// was: __ZNSt4pairIKSsSsEC2ERS0_S2_
// IDA 0x2c1674: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c1674() {
}

// 0x2c171c — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>,std::pair<std::string const,std::string> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// IDA 0x2c171c: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c171c() {
}

// 0x2c1808 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::string> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// IDA 0x2c1808: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c1808() {
}

// 0x2c1858 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
// IDA 0x2c1858: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c1858() {
}

// 0x2c18dc — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
// type: _DWORD *__fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
// IDA 0x2c18dc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c18dc() {
}

// 0x2c190c — __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef>::operator=(rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef> const&)")]
// was: __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSERKS5_
// IDA 0x2c190c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c190c() {
}

// 0x2c1930 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::detail::LiveThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::detail::LiveThreadRef,int,0> const*)")]
// was: __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua6detail13LiveThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
// IDA 0x2c1930: 93 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c1930() {
}

// 0x2c1a48 — __ZN3RBX9ContentIdC2ERKSs
// type: _DWORD __fastcall(RBX::ContentId *__hidden this, const std::string *)
#[doc(alias = "RBX::ContentId::ContentId(std::string const&)")]
// was: __ZN3RBX9ContentIdC2ERKSs
// IDA 0x2c1a48: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c1a48() {
}

// 0x2c1af8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::LuaStatsItem,RBX::ScriptContext *>(RBX::ScriptContext *)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LuaStatsItemEPNS_13ScriptContextEEEN5boost10shared_ptrIT_EET0_
// IDA 0x2c1af8: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c1af8() {
}
