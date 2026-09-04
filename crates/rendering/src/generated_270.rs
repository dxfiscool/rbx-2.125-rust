//! rendering shard 270 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29370->29470 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29370 before -> 29470 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x37b65c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x37b65c: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b65c() {
}

// 0x37b89c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x37b89c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b89c() {
}

// 0x37b8b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x37b8b8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b8b8() {
}

// 0x37b8ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x37b8ec: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b8ec() {
}

// 0x37b8f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x37b8f4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b8f4() {
}

// 0x37b940 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x37b940: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b940() {
}

// 0x37b960 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x37b960: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b960() {
}

// 0x37b994 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToIndexES3_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::convertToIndex(RBX::Soundscape::ReverbType)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE14convertToIndexES3_
// IDA 0x37b994: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37b994() {
}

// 0x37ba04 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x37ba04: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ba04() {
}

// 0x37ba44 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE10isReadOnlyEv
// IDA 0x37ba44: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ba44() {
}

// 0x37ba48 — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE11isWriteOnlyEv
// IDA 0x37ba48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ba48() {
}

// 0x37ba4c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x37ba4c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ba4c() {
}

// 0x37ba6c — __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE8setValueEPNS0_13DescribedBaseESA_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::GetSetImpl<RBX::Soundscape::ReverbType (RBX::Soundscape::SoundService::*)(void)const,void (RBX::Soundscape::SoundService::*)(RBX::Soundscape::ReverbType const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::Soundscape::ReverbType const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEE10GetSetImplIMS3_KFS4_vEMS3_FvRKS4_EE8setValueEPNS0_13DescribedBaseESA_
// IDA 0x37ba6c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ba6c() {
}

// 0x37ba90 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundServiceEEEPKcS8_MT_fMS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Soundscape::SoundService>(char const*,char const*,float RBX::Soundscape::SoundService::*,void (RBX::Soundscape::SoundService::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_10Soundscape12SoundServiceEEEPKcS8_MT_fMS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// IDA 0x37ba90: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ba90() {
}

// 0x37bc24 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE10isReadOnlyEv
// IDA 0x37bc24: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bc24() {
}

// 0x37bc28 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE11isWriteOnlyEv
// IDA 0x37bc28: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bc28() {
}

// 0x37bc2c — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x37bc2c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bc2c() {
}

// 0x37bc38 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE8setValueEPNS0_13DescribedBaseERKf
// type: float *__fastcall(int, int, float *)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Soundscape::SoundService>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_10Soundscape12SoundServiceEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x37bc38: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bc38() {
}

// 0x37bc94 — __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x37bc94: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bc94() {
}

// 0x37bcec — __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x37bcec: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bcec() {
}

// 0x37bddc — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
// IDA 0x37bddc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bddc() {
}

// 0x37be48 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc
// type: const std::string *__fastcall(const std::string *result, std::string *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc
// IDA 0x37be48: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37be48() {
}

// 0x37be64 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc
// type: int __fastcall(int)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc
// IDA 0x37be64: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37be64() {
}

// 0x37be68 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_
// IDA 0x37be68: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37be68() {
}

// 0x37bf50 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x37bf50: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37bf50() {
}

// 0x37c034 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// IDA 0x37c034: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c034() {
}

// 0x37c12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED1Ev
// IDA 0x37c12c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_37c12c() {
}

// 0x37c130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED0Ev
// IDA 0x37c130: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37c130() {
}

// 0x37c134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE7disposeEv
// IDA 0x37c134: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c134() {
}

// 0x37c144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE11get_deleterERKSt9type_info
// IDA 0x37c144: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c144() {
}

// 0x37c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE19get_untyped_deleterEv
// IDA 0x37c148: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c148() {
}

// 0x37c14c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// IDA 0x37c14c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c14c() {
}

// 0x37c200 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// IDA 0x37c200: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c200() {
}

// 0x37c24c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// IDA 0x37c24c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c24c() {
}

// 0x37c2b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// IDA 0x37c2b4: 86 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c2b4() {
}

// 0x37c3a4 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev
// IDA 0x37c3a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37c3a4() {
}

// 0x37c440 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x37c440: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c440() {
}

// 0x37c4c8 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(__guard *, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv
// IDA 0x37c4c8: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c4c8() {
}

// 0x37c60c — __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv
// IDA 0x37c60c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37c60c() {
}

// 0x37c610 — __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v
// IDA 0x37c610: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c610() {
}

// 0x37c6f0 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev
// IDA 0x37c6f0: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c6f0() {
}

// 0x37c934 — __ZN3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "RBX::StockSound::~StockSound()")]
// was: __ZN3RBX10StockSoundD1Ev
// IDA 0x37c934: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37c934() {
}

// 0x37c938 — __ZN3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "RBX::StockSound::~StockSound()")]
// was: __ZN3RBX10StockSoundD0Ev
// IDA 0x37c938: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37c938() {
}

// 0x37c9d8 — __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// IDA 0x37c9d8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37c9d8() {
}

// 0x37c9e8 — __ZThn32_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// was: __ZThn32_N3RBX10StockSoundD1Ev
// IDA 0x37c9e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37c9e8() {
}

// 0x37c9f0 — __ZThn32_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// was: __ZThn32_N3RBX10StockSoundD0Ev
// IDA 0x37c9f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37c9f0() {
}

// 0x37ca94 — __ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv
// IDA 0x37ca94: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ca94() {
}

// 0x37caa4 — __ZThn36_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// was: __ZThn36_N3RBX10StockSoundD1Ev
// IDA 0x37caa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37caa4() {
}

// 0x37caac — __ZThn36_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// was: __ZThn36_N3RBX10StockSoundD0Ev
// IDA 0x37caac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37caac() {
}

// 0x37cb50 — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv
// IDA 0x37cb50: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37cb50() {
}

// 0x37cbc4 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37cbc4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37cbc4() {
}

// 0x37cbc8 — __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37cbc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37cbc8() {
}

// 0x37cc68 — __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37cc68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37cc68() {
}

// 0x37cc70 — __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37cc70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37cc70() {
}

// 0x37cd14 — __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37cd14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37cd14() {
}

// 0x37cd1c — __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37cd1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37cd1c() {
}

// 0x37cdc0 — __ZN5boost10shared_ptrIN3RBX10StockSoundEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10StockSoundEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x37cdc0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37cdc0() {
}

// 0x37ce88 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10StockSoundES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StockSound,RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const*,RBX::StockSound *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10StockSoundES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x37ce88: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37ce88() {
}

// 0x37cf74 — __ZN5boost6detail12shared_countC2IPN3RBX10StockSoundENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10StockSoundENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x37cf74: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37cf74() {
}

// 0x37d07c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x37d07c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_37d07c() {
}

// 0x37d080 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x37d080: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37d080() {
}

// 0x37d084 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x37d084: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d084() {
}

// 0x37d0a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x37d0a4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d0a4() {
}

// 0x37d0bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x37d0bc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d0bc() {
}

// 0x37d0c0 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>> *)")]
// was: __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// IDA 0x37d0c0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d0c0() {
}

// 0x37d0f0 — __ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)")]
// was: __ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// IDA 0x37d0f0: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d0f0() {
}

// 0x37d1b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
// was: __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// IDA 0x37d1b4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d1b4() {
}

// 0x37d1dc — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
// was: __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// IDA 0x37d1dc: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d1dc() {
}

// 0x37d1f8 — __ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(RBX::Soundscape::CollisionSoundManager **)
#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// IDA 0x37d1f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37d1f8() {
}

// 0x37d2a0 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37d2a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37d2a0() {
}

// 0x37d2a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37d2a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37d2a4() {
}

// 0x37d344 — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37d344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37d344() {
}

// 0x37d34c — __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37d34c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37d34c() {
}

// 0x37d3f0 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x37d3f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37d3f0() {
}

// 0x37d3f8 — __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x37d3f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37d3f8() {
}

// 0x37d49c — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
// was: __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
// IDA 0x37d49c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d49c() {
}

// 0x37d4d0 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
// was: __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
// IDA 0x37d4d0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_37d4d0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x37d4f8 — __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x37d4f8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d4f8() {
}

// 0x37d550 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x37d550: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d550() {
}

// 0x37d604 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x37d604: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d604() {
}

// 0x37d65c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x37d65c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d65c() {
}

// 0x37d6c4 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
// was: __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x37d6c4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_37d6c4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x37d7a8 — __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
// IDA 0x37d7a8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_37d7a8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x37d7c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
// IDA 0x37d7c0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_37d7c0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x37d7fc — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
// was: __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x37d7fc: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d7fc() {
}

// 0x37d98c — __ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, const RBX::Soundscape::SoundService *)
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x37d98c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37d98c() {
}

// 0x37da40 — __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
// type: RBX::Instance *__fastcall(RBX::Instance *, int, int)
#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
// was: __ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
// IDA 0x37da40: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37da40() {
}

// 0x37dbf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x37dbf4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_37dbf4() {
}

// 0x37dbf8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x37dbf8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37dbf8() {
}

// 0x37dc18 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIiEEPKiEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<int>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<int>,int const*>(int const*)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIiEEPKiEEN5boost10shared_ptrIT_EET0_
// IDA 0x37dc18: 90 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37dc18() {
}

// 0x37dd20 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// type: int __fastcall(__int64, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
// IDA 0x37dd20: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37dd20() {
}

// 0x37dd80 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKiPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEiE6invokeERNS1_15function_bufferE
// IDA 0x37dd80: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37dd80() {
}

// 0x37dd90 — __ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIiEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIiEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x37dd90: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37dd90() {
}

// 0x382e04 — __ZN3RBX4Http9urlEncodeESs
// type: void __fastcall(std::string *, int *)
#[doc(alias = "RBX::Http::urlEncode(std::string)")]
// was: __ZN3RBX4Http9urlEncodeESs
// IDA 0x382e04: 150 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382e04() {
}

// 0x382f9c — __Z28FetchLocalClientSettingsDataPKcP10SimpleJSON
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(const char *, SimpleJSON *)
#[doc(alias = "FetchLocalClientSettingsData(char const*,SimpleJSON *)")]
// was: __Z28FetchLocalClientSettingsDataPKcP10SimpleJSON
// IDA 0x382f9c: 455 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_382f9c() {
}

// 0x3834bc — __Z28LoadClientSettingsFromStringPKcRKSsP10SimpleJSON
// type: const char *__fastcall(const char *, const char **, SimpleJSON *this, const char *)
#[doc(alias = "LoadClientSettingsFromString(char const*,std::string const&,SimpleJSON *)")]
// was: __Z28LoadClientSettingsFromStringPKcRKSsP10SimpleJSON
// IDA 0x3834bc: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3834bc() {
}

// 0x383538 — __Z23FetchClientSettingsDataPKcS0_P10SimpleJSON
// type: void __fastcall(const char *, const char *, SimpleJSON *)
#[doc(alias = "FetchClientSettingsData(char const*,char const*,SimpleJSON *)")]
// was: __Z23FetchClientSettingsDataPKcS0_P10SimpleJSON
// IDA 0x383538: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_383538() {
}

// 0x38367c — __Z23FetchClientSettingsDataPKcS0_PSs
// type: void __fastcall(const char *, const char *, std::string *, const char *)
#[doc(alias = "FetchClientSettingsData(char const*,char const*,std::string *)")]
// was: __Z23FetchClientSettingsDataPKcS0_PSs
// IDA 0x38367c: 511 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38367c() {
}

// 0x383c54 — __Z19ReportStatisticPostRKSsS0_S0_PKcS2_S2_S2_
// type: void __fastcall(const std::string *, const std::string *, const std::string *, const char *, char *, char *, char *)
#[doc(alias = "ReportStatisticPost(std::string const&,std::string const&,std::string const&,char const*,char const*,char const*,char const*)")]
// was: __Z19ReportStatisticPostRKSsS0_S0_PKcS2_S2_S2_
// IDA 0x383c54: 1307 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_383c54() {
}

// 0x384ae0 — __Z15ReportStatisticRKSsS0_S0_S0_S0_S0_
// type: void __fastcall(const std::string *, const std::string *, const char **, char **, char **, char **)
#[doc(alias = "ReportStatistic(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
// was: __Z15ReportStatisticRKSsS0_S0_S0_S0_S0_
// IDA 0x384ae0: 119 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_384ae0() {
}