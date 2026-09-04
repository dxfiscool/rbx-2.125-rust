//! rendering shard 412 — 100 stubs 0x637898..0x65b260 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 83974->84074 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x637898..0x65b260 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x637898 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED1Ev
// IDA 0x637898: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_637898() {
}

// 0x637d08 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5SmokeES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5SmokeES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Smoke,RBX::Smoke>(rbx_core::SharedPtr<RBX::Smoke> const*,RBX::Smoke *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5SmokeES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x637d08: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637d08() {
}

// 0x6386d0 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::PropDescriptor<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>(char const*,char const*,float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x6386d0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6386d0() {
}

// 0x6387e4 — __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5SmokeEfED0Ev
// IDA 0x6387e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6387e4() {
}

// 0x638810 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// IDA 0x638810: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638810() {
}

// 0x638814 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// IDA 0x638814: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638814() {
}

// 0x638818 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x638818: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638818() {
}

// 0x638838 — __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Smoke,float>::GetSetImpl<float (RBX::Smoke::*)(void)const,void (RBX::Smoke::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5SmokeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x638838: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638838() {
}

// 0x638a08 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_5SmokeEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_5SmokeEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Smoke>(char const*,char const*,bool RBX::Smoke::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_5SmokeEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x638a08: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638a08() {
}

// 0x638b98 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE10isReadOnlyEv
// IDA 0x638b98: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638b98() {
}

// 0x638b9c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE11isWriteOnlyEv
// IDA 0x638b9c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638b9c() {
}

// 0x638ba0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x638ba0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638ba0() {
}

// 0x638bac — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Smoke>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_5SmokeEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x638bac: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_638bac() {
}

// 0x639170 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC1Ev
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC1Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC1Ev
// IDA 0x639170: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_639170() {
}

// 0x639174 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC2Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC2Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEEC2Ev
// IDA 0x639174: 254 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_639174() {
}

// 0x63a240 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED1Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED1Ev
// IDA 0x63a240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63a240() {
}

// 0x63a280 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE7addPairES3_PKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::addPair(RBX::SocialService::StuffType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE7addPairES3_PKc
// IDA 0x63a280: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63a280() {
}

// 0x63b910 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::BoundFuncDesc(void (RBX::SocialService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x63b910: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63b910() {
}

// 0x63ba88 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x63ba88: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63ba88() {
}

// 0x63bab8 — __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EED0Ev
// IDA 0x63bab8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63bab8() {
}

// 0x63bb84 — __ZNK3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SocialService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13SocialServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x63bb84: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63bb84() {
}

// 0x63bcc0 — __ZN3RBX10Reflection11Call1HelperINS_13SocialServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_13SocialServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")]
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::SocialService,void (RBX::SocialService::*)(std::string),std::string,void>::call(RBX::SocialService*,void (RBX::SocialService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_13SocialServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// IDA 0x63bcc0: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63bcc0() {
}

// 0x63bdf0 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE5clearEv
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE5clearEv")]
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::clear(void)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE5clearEv
// IDA 0x63bdf0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63bdf0() {
}

// 0x63cc8c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8SparklesEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8SparklesEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sparkles>(char const*,char const*,bool RBX::Sparkles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8SparklesEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x63cc8c: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63cc8c() {
}

// 0x63ce1c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE10isReadOnlyEv
// IDA 0x63ce1c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63ce1c() {
}

// 0x63ce20 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE11isWriteOnlyEv
// IDA 0x63ce20: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63ce20() {
}

// 0x63ce24 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x63ce24: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63ce24() {
}

// 0x63ce30 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sparkles>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8SparklesEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x63ce30: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63ce30() {
}

// 0x63e2a8 — __ZN3RBX13SpawnLocation31onAllowTeamChangeOnTouchChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::SpawnLocation *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "__ZN3RBX13SpawnLocation31onAllowTeamChangeOnTouchChangedERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::SpawnLocation::onAllowTeamChangeOnTouchChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX13SpawnLocation31onAllowTeamChangeOnTouchChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x63e2a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_63e2a8() {
}

// 0x63e2ac — __ZN3RBX10Reflection14PropDescriptorINS_13SpawnLocationENS_10BrickColorEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_13SpawnLocationENS_10BrickColorEED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SpawnLocation,RBX::BrickColor>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13SpawnLocationENS_10BrickColorEED1Ev
// IDA 0x63e2ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63e2ac() {
}

// 0x63ef38 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SpawnLocationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SpawnLocationES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SpawnLocation,RBX::SpawnLocation>(rbx_core::SharedPtr<RBX::SpawnLocation> const*,RBX::SpawnLocation *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SpawnLocationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x63ef38: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63ef38() {
}

// 0x65740c — __GLOBAL__I_a_263
// type: 
#[doc(alias = "__GLOBAL__I_a_263")]
#[doc(alias = "global constructor keyed to_a_263")]
// was: __GLOBAL__I_a_263
// IDA 0x65740c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_65740c() {
}

// 0x657980 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x657980: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_657980() {
}

// 0x6579ac — __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED1Ev
// IDA 0x6579ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6579ac() {
}

// 0x6579d0 — __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::PropDescriptor<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>(char const*,char const*,bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x6579d0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6579d0() {
}

// 0x657ae4 — __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10StudioToolEbED0Ev
// IDA 0x657ae4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_657ae4() {
}

// 0x657b10 — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x657b10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657b10() {
}

// 0x657b14 — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x657b14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657b14() {
}

// 0x657b18 — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x657b18: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657b18() {
}

// 0x657b3c — __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::StudioTool,bool>::GetSetImpl<bool (RBX::StudioTool::*)(void)const,void (RBX::StudioTool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10StudioToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x657b3c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657b3c() {
}

// 0x657b60 — __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// IDA 0x657b60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_657b60() {
}

// 0x657c14 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x657c14: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657c14() {
}

// 0x657e18 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi0ENS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x657e18: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657e18() {
}

// 0x657e8c — __ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::StudioTool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::StudioTool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_10StudioToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x657e8c: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_657e8c() {
}

// 0x6583a0 — __GLOBAL__I_a_264
// type: 
#[doc(alias = "__GLOBAL__I_a_264")]
#[doc(alias = "global constructor keyed to_a_264")]
// was: __GLOBAL__I_a_264
// IDA 0x6583a0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_6583a0() {
}

// 0x658744 — __GLOBAL__I_a_265
// type: 
#[doc(alias = "__GLOBAL__I_a_265")]
#[doc(alias = "global constructor keyed to_a_265")]
// was: __GLOBAL__I_a_265
// IDA 0x658744: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_658744() {
}

// 0x658a7c — __ZN3RBX10Reflection4Type12getSingletonINS_7SurfaceEEERKS1_v
// type: 
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_7SurfaceEEERKS1_v")]
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Surface>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_7SurfaceEEERKS1_v
// IDA 0x658a7c: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_658a7c() {
}

// 0x658b70 — __ZN3RBX7Surface19isSurfaceDescriptorERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::Surface *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "__ZN3RBX7Surface19isSurfaceDescriptorERKNS_10Reflection18PropertyDescriptorE")]
#[doc(alias = "RBX::Surface::isSurfaceDescriptor(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX7Surface19isSurfaceDescriptorERKNS_10Reflection18PropertyDescriptorE
// IDA 0x658b70: 40 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_658b70() {
}

// 0x658e20 — __ZN3RBX10Reflection5TTypeINS_7SurfaceEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_7SurfaceEED1Ev")]
#[doc(alias = "RBX::Reflection::TType<RBX::Surface>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeINS_7SurfaceEED1Ev
// IDA 0x658e20: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_658e20() {
}

// 0x658e24 — __ZN3RBX10Reflection7Variant14genericConvertINS_11SurfaceTypeEEERT_v
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_11SurfaceTypeEEERT_v")]
#[doc(alias = "RBX::SurfaceType & RBX::Reflection::Variant::genericConvert<RBX::SurfaceType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_11SurfaceTypeEEERT_v
// IDA 0x658e24: 116 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_658e24() {
}

// 0x658f8c — __ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v")]
#[doc(alias = "RBX::LegacyController::InputType & RBX::Reflection::Variant::genericConvert<RBX::LegacyController::InputType>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v
// IDA 0x658f8c: 116 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_658f8c() {
}

// 0x659600 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// IDA 0x659600: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659600() {
}

// 0x659628 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// IDA 0x659628: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659628() {
}

// 0x659650 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// IDA 0x659650: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659650() {
}

// 0x6597a8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// IDA 0x6597a8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6597a8() {
}

// 0x6597d0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x6597d0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6597d0() {
}

// 0x659820 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// IDA 0x659820: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659820() {
}

// 0x659884 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// IDA 0x659884: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659884() {
}

// 0x6598a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6598a4: 213 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6598a4() {
}

// 0x659afc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x659afc: 24 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659afc() {
}

// 0x659b44 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// IDA 0x659b44: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659b44() {
}

// 0x659ba0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x659ba0: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659ba0() {
}

// 0x659ba8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// IDA 0x659ba8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659ba8() {
}

// 0x659c1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// IDA 0x659c1c: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659c1c() {
}

// 0x659c6c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// IDA 0x659c6c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659c6c() {
}

// 0x659cc8 — __ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::LegacyController::InputType>::convertToIndex(RBX::LegacyController::InputType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_
// IDA 0x659cc8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659cc8() {
}

// 0x659e7c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// IDA 0x659e7c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659e7c() {
}

// 0x659ea4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// IDA 0x659ea4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659ea4() {
}

// 0x659ecc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// IDA 0x659ecc: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_659ecc() {
}

// 0x65a024 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// IDA 0x65a024: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a024() {
}

// 0x65a04c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65a04c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a04c() {
}

// 0x65a09c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// IDA 0x65a09c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a09c() {
}

// 0x65a100 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// IDA 0x65a100: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a100() {
}

// 0x65a120 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x65a120: 213 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a120() {
}

// 0x65a378 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65a378: 24 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a378() {
}

// 0x65a3c0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// IDA 0x65a3c0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a3c0() {
}

// 0x65a41c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65a41c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a41c() {
}

// 0x65a424 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// IDA 0x65a424: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a424() {
}

// 0x65a498 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// IDA 0x65a498: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a498() {
}

// 0x65a4e8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::SurfaceType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_11SurfaceTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// IDA 0x65a4e8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a4e8() {
}

// 0x65a544 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToIndex(RBX::SurfaceType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_
// IDA 0x65a544: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a544() {
}

// 0x65a884 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_
// IDA 0x65a884: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a884() {
}

// 0x65a8ac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE
// IDA 0x65a8ac: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a8ac() {
}

// 0x65a8d4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE
// IDA 0x65a8d4: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65a8d4() {
}

// 0x65aa2c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_
// IDA 0x65aa2c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65aa2c() {
}

// 0x65aa54 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65aa54: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65aa54() {
}

// 0x65aaa4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs
// IDA 0x65aaa4: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65aaa4() {
}

// 0x65ab08 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement
// IDA 0x65ab08: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65ab08() {
}

// 0x65ab28 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x65ab28: 213 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65ab28() {
}

// 0x65ad80 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65ad80: 24 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65ad80() {
}

// 0x65adc8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm
// IDA 0x65adc8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65adc8() {
}

// 0x65ae24 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65ae24: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65ae24() {
}

// 0x65ae2c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi
// IDA 0x65ae2c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65ae2c() {
}

// 0x65aea0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE
// IDA 0x65aea0: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65aea0() {
}

// 0x65aef0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE
// IDA 0x65aef0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65aef0() {
}

// 0x65b090 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES7_
// IDA 0x65b090: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65b090() {
}

// 0x65b0b8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS4_7VariantE
// IDA 0x65b0b8: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65b0b8() {
}

// 0x65b0e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS4_7VariantE
// IDA 0x65b0e0: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65b0e0() {
}

// 0x65b238 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS5_
// IDA 0x65b238: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65b238() {
}

// 0x65b260 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::SurfaceType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_11SurfaceTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE
// IDA 0x65b260: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_65b260() {
}
