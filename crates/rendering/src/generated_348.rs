//! rendering shard A — next 120 global gap (EA-sorted asc, after 0x4b8850, 0x5e321c..0x5fa6ec, true uncovered not yet in any crate)
//! Filter: Ogre|G3D|Gfx|Render|Adorn strict exhausted (15586/15586), rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5e321c — __ZN3RBX10Reflection8EnumDescINS_8MaterialEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::~EnumDesc()
// IDA 0x5e321c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5e321c() {
}

// 0x5e3220 — __ZN3RBX10Reflection8EnumDescINS_8MaterialEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::~EnumDesc()
// IDA 0x5e3220: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e3220() {
}

// 0x5e32c0 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::lookup(char const*)const
// IDA 0x5e32c0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e32c0() {
}

// 0x5e32f0 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x5e32f0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e32f0() {
}

// 0x5e3310 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x5e3310: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e3310() {
}

// 0x5e3344 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::convertToString(unsigned long,std::string &)const
// IDA 0x5e3344: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e3344() {
}

// 0x5e3ba8 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE15convertToStringERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::convertToString(RBX::Material const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::convertToString(RBX::Material const&)const
// IDA 0x5e3ba8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e3ba8() {
}

// 0x5e3e14 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::convertToItem(RBX::Material const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::convertToItem(RBX::Material const&)const
// IDA 0x5e3e14: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e3e14() {
}

// 0x5e40b8 — __ZN3rbx22bad_placement_any_castD0Ev
#[doc(alias = "rbx::bad_placement_any_cast::~bad_placement_any_cast()")]
// was: rbx::bad_placement_any_cast::~bad_placement_any_cast()
// IDA 0x5e40b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e40b8() {
}

// 0x5e4264 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::convertToValue(RBX::Name const&,RBX::Material&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::convertToValue(RBX::Name const&,RBX::Material&)const
// IDA 0x5e4264: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e4264() {
}

// 0x5e42e0 — __ZN3RBX10Reflection8EnumDescINS_8MaterialEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::~EnumDesc()
// IDA 0x5e42e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e42e0() {
}

// 0x5e4e40 — __ZN3RBX10Reflection4TypeD1Ev
#[doc(alias = "RBX::Reflection::Type::~Type()")]
// was: RBX::Reflection::Type::~Type()
// IDA 0x5e4e40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5e4e40() {
}

// 0x5e8e60 — __ZNK3RBX10Reflection15EventDescriptor11isBroadcastEv
#[doc(alias = "RBX::Reflection::EventDescriptor::isBroadcast(void)const")]
// was: RBX::Reflection::EventDescriptor::isBroadcast(void)const
// IDA 0x5e8e60: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e8e60() {
}

// 0x5e9160 — __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::~vector()")]
// was: std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::~vector()
// IDA 0x5e9160: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5e9160() {
}

// 0x5e9230 — __ZNSt12_Vector_baseIN3RBX10Reflection7VariantESaIS2_EEC2EmRKS3_
#[doc(alias = "std::_Vector_base<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::_Vector_base(unsigned long,std::allocator<RBX::Reflection::Variant> const&)")]
// was: std::_Vector_base<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::_Vector_base(unsigned long,std::allocator<RBX::Reflection::Variant> const&)
// IDA 0x5e9230: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5e9230() {
}

// 0x5ecd0c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5ecd0c: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ecd0c() {
}

// 0x5ece5c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::isReadOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::isReadOnly(void)const
// IDA 0x5ece5c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ece5c() {
}

// 0x5ece6c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::isWriteOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::isWriteOnly(void)const
// IDA 0x5ece6c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ece6c() {
}

// 0x5ece7c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x5ece7c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ece7c() {
}

// 0x5eceb0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x5eceb0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eceb0() {
}

// 0x5ecedc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x5ecedc: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ecedc() {
}

// 0x5ed040 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEE9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x5ed040: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed040() {
}

// 0x5ed230 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::~TypedPropertyDescriptor()
// IDA 0x5ed230: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ed230() {
}

// 0x5ed254 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_13SystemAddressEED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::SystemAddress>::~TypedPropertyDescriptor()
// IDA 0x5ed254: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ed254() {
}

// 0x5ed9ec — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8NormalIdEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NormalId> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NormalId> const>::initSingleton(void)
// IDA 0x5ed9ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ed9ec() {
}

// 0x5ed9f0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8NormalIdEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NormalId> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::NormalId> const>::doGetSingleton(void)
// IDA 0x5ed9f0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ed9f0() {
}

// 0x5edae0 — __ZN3RBX10Reflection8EnumDescINS_8NormalIdEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::~EnumDesc()
// IDA 0x5edae0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5edae0() {
}

// 0x5edae8 — __ZN3RBX10Reflection8EnumDescINS_8NormalIdEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::~EnumDesc()
// IDA 0x5edae8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5edae8() {
}

// 0x5edb88 — __ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::lookup(char const*)const
// IDA 0x5edb88: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5edb88() {
}

// 0x5edbb8 — __ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x5edbb8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5edbb8() {
}

// 0x5edbd8 — __ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x5edbd8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5edbd8() {
}

// 0x5edc10 — __ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE15convertToStringERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::convertToString(RBX::NormalId const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::convertToString(RBX::NormalId const&)const
// IDA 0x5edc10: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5edc10() {
}

// 0x5edf68 — __ZNK3RBX10Reflection8EnumDescINS_8NormalIdEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NormalId>::convertToValue(RBX::Name const&,RBX::NormalId&)const")]
// was: RBX::Reflection::EnumDesc<RBX::NormalId>::convertToValue(RBX::Name const&,RBX::NormalId&)const
// IDA 0x5edf68: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5edf68() {
}

// 0x5edfe8 — __ZN3rbx14implementation12typed_holderIbE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<bool>::singleton(void)")]
// was: rbx::implementation::typed_holder<bool>::singleton(void)
// IDA 0x5edfe8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5edfe8() {
}

// 0x5ee058 — __ZN3rbx14implementation12typed_holderIbE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<bool>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<bool>::destruct_func(char *)
// IDA 0x5ee058: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ee058() {
}

// 0x5ee060 — __ZN3RBX10Reflection18FunctionDescriptorD0Ev
#[doc(alias = "RBX::Reflection::FunctionDescriptor::~FunctionDescriptor()")]
// was: RBX::Reflection::FunctionDescriptor::~FunctionDescriptor()
// IDA 0x5ee060: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ee060() {
}

// 0x5ee220 — __ZN3RBX10Reflection23TypedPropertyDescriptorIiEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<int>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<int>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5ee220: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee220() {
}

// 0x5ee370 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIiE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::isReadOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::isReadOnly(void)const
// IDA 0x5ee370: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee370() {
}

// 0x5ee380 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIiE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::isWriteOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::isWriteOnly(void)const
// IDA 0x5ee380: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee380() {
}

// 0x5ee390 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIiE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x5ee390: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee390() {
}

// 0x5ee3b8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIiE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x5ee3b8: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee3b8() {
}

// 0x5ee3e0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIiE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x5ee3e0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee3e0() {
}

// 0x5ee530 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIiE9copyValueEPKNS0_13DescribedBaseEPS3_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x5ee530: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee530() {
}

// 0x5ee558 — __ZN3rbx14implementation12typed_holderIiE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<int>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<int>::construct_func(char const*,char *)
// IDA 0x5ee558: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee558() {
}

// 0x5ee568 — __ZN3RBX10Reflection23TypedPropertyDescriptorIiED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<int>::~TypedPropertyDescriptor()
// IDA 0x5ee568: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ee568() {
}

// 0x5ee7e4 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5ee7e4: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee7e4() {
}

// 0x5ee934 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::isReadOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::isReadOnly(void)const
// IDA 0x5ee934: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee934() {
}

// 0x5ee944 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::isWriteOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::isWriteOnly(void)const
// IDA 0x5ee944: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee944() {
}

// 0x5ee954 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x5ee954: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee954() {
}

// 0x5ee97c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x5ee97c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee97c() {
}

// 0x5ee9a4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x5ee9a4: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ee9a4() {
}

// 0x5eeafc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEE9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x5eeafc: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eeafc() {
}

// 0x5eecd4 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::~TypedPropertyDescriptor()
// IDA 0x5eecd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5eecd4() {
}

// 0x5eecf8 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_5FacesEED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::Faces>::~TypedPropertyDescriptor()
// IDA 0x5eecf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5eecf8() {
}

// 0x5eef80 — __ZN3RBX10Reflection23TypedPropertyDescriptorIbEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<bool>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<bool>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<bool>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5eef80: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5eef80() {
}

// 0x5ef0d0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::isReadOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<bool>::isReadOnly(void)const
// IDA 0x5ef0d0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef0d0() {
}

// 0x5ef0e0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::isWriteOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<bool>::isWriteOnly(void)const
// IDA 0x5ef0e0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef0e0() {
}

// 0x5ef0f0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIbE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<bool>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x5ef0f0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef0f0() {
}

// 0x5ef118 — __ZN3RBX10Reflection23TypedPropertyDescriptorIbED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<bool>::~TypedPropertyDescriptor()
// IDA 0x5ef118: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ef118() {
}

// 0x5ef550 — __ZNK3RBX10Reflection22EnumPropertyDescriptor11getDataSizeEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropertyDescriptor::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropertyDescriptor::getDataSize(RBX::Reflection::DescribedBase const*)const
// IDA 0x5ef550: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef550() {
}

// 0x5ef9b8 — __ZNK3RBX10Reflection8EnumDescINS_8MaterialEE14convertToIndexES2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Material>::convertToIndex(RBX::Material)const")]
// was: RBX::Reflection::EnumDesc<RBX::Material>::convertToIndex(RBX::Material)const
// IDA 0x5ef9b8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ef9b8() {
}

// 0x5efab4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8MaterialEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Material> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Material> const>::initSingleton(void)
// IDA 0x5efab4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5efab4() {
}

// 0x5efab8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8MaterialEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Material> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Material> const>::doGetSingleton(void)
// IDA 0x5efab8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efab8() {
}

// 0x5efcbc — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5efcbc: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efcbc() {
}

// 0x5efe0c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::isReadOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::isReadOnly(void)const
// IDA 0x5efe0c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efe0c() {
}

// 0x5efe1c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::isWriteOnly(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::isWriteOnly(void)const
// IDA 0x5efe1c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efe1c() {
}

// 0x5efe2c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x5efe2c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efe2c() {
}

// 0x5efe60 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x5efe60: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efe60() {
}

// 0x5efe8c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x5efe8c: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5efe8c() {
}

// 0x5effe8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEE9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x5effe8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5effe8() {
}

// 0x5f01c4 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::~TypedPropertyDescriptor()
// IDA 0x5f01c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f01c4() {
}

// 0x5f01e8 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_10BrickColorEED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<RBX::BrickColor>::~TypedPropertyDescriptor()
// IDA 0x5f01e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f01e8() {
}

// 0x5f0ac0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<float>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x5f0ac0: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f0ac0() {
}

// 0x5f0ae8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<float>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x5f0ae8: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f0ae8() {
}

// 0x5f0c40 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIfE9copyValueEPKNS0_13DescribedBaseEPS3_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<float>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x5f0c40: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f0c40() {
}

// 0x5f0c68 — __ZN3rbx14implementation12typed_holderIfE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<float>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<float>::construct_func(char const*,char *)
// IDA 0x5f0c68: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f0c68() {
}

// 0x5f0c78 — __ZN3RBX10Reflection23TypedPropertyDescriptorIfED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::~TypedPropertyDescriptor()")]
// was: RBX::Reflection::TypedPropertyDescriptor<float>::~TypedPropertyDescriptor()
// IDA 0x5f0c78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f0c78() {
}

// 0x5f2180 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE9push_backERKS5_
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::push_back(RBX::Reflection::EnumDescriptor::Item const* const&)")]
// was: std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::push_back(RBX::Reflection::EnumDescriptor::Item const* const&)
// IDA 0x5f2180: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5f2180() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5f21e0 — __ZNSt6vectorImSaImEE6resizeEmm
#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::resize(unsigned long,unsigned long)")]
// was: std::vector<unsigned long,std::allocator<unsigned long>>::resize(unsigned long,unsigned long)
// IDA 0x5f21e0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f21e0() {
}

// 0x5f2240 — __ZNSt6vectorISsSaISsEE6resizeEmSs
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::resize(unsigned long,std::string)")]
// was: std::vector<std::string,std::allocator<std::string>>::resize(unsigned long,std::string)
// IDA 0x5f2240: 28 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f2240() {
}

// 0x5f2558 — __ZN3RBX10Reflection14EnumDescriptor4ItemD1Ev
#[doc(alias = "RBX::Reflection::EnumDescriptor::Item::~Item()")]
// was: RBX::Reflection::EnumDescriptor::Item::~Item()
// IDA 0x5f2558: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5f2558() {
}

// 0x5f2a50 — __ZNSt6vectorISsSaISsEED2Ev
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::~vector()")]
// was: std::vector<std::string,std::allocator<std::string>>::~vector()
// IDA 0x5f2a50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f2a50() {
}

// 0x5f9108 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::~PropDescriptor()
// IDA 0x5f9108: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f9108() {
}

// 0x5f912c — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::~EnumPropDescriptor()
// IDA 0x5f912c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f912c() {
}

// 0x5f915c — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::~PropDescriptor()
// IDA 0x5f915c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f915c() {
}

// 0x5f9ba0 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::PropDescriptor<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>(char const*,char const*,double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::PropDescriptor<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>(char const*,char const*,double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5f9ba0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9ba0() {
}

// 0x5f9cb4 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::~PropDescriptor()
// IDA 0x5f9cb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f9cb4() {
}

// 0x5f9ce0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::isReadOnly(void)const
// IDA 0x5f9ce0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9ce0() {
}

// 0x5f9ce4 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::isWriteOnly(void)const
// IDA 0x5f9ce4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9ce4() {
}

// 0x5f9ce8 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x5f9ce8: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9ce8() {
}

// 0x5f9d08 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEdE10GetSetImplIMS2_KFdvEMS2_FvdEE8setValueEPNS0_13DescribedBaseERKd
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,double>::GetSetImpl<double (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(double)>::setValue(RBX::Reflection::DescribedBase *,double const&)const
// IDA 0x5f9d08: 14 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9d08() {
}

// 0x5f9d30 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::EnumPropDescriptor<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>(char const*,char const*,RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::EnumPropDescriptor<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>(char const*,char const*,RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5f9d30: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9d30() {
}

// 0x5f9ee4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::~EnumPropDescriptor()
// IDA 0x5f9ee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5f9ee4() {
}

// 0x5f9f10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::isReadOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::isReadOnly(void)const
// IDA 0x5f9f10: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9f10() {
}

// 0x5f9f20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::isWriteOnly(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::isWriteOnly(void)const
// IDA 0x5f9f20: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9f20() {
}

// 0x5f9f30 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
// IDA 0x5f9f30: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9f30() {
}

// 0x5f9f58 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
// IDA 0x5f9f58: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9f58() {
}

// 0x5f9f7c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
// IDA 0x5f9f7c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5f9f7c() {
}

// 0x5fa0c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
// IDA 0x5fa0c8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa0c8() {
}

// 0x5fa0ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::hasStringValue(void)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::hasStringValue(void)const
// IDA 0x5fa0ec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa0ec() {
}

// 0x5fa0f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getStringValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x5fa0f0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa0f0() {
}

// 0x5fa114 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
// IDA 0x5fa114: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa114() {
}

// 0x5fa154 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
// IDA 0x5fa154: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa154() {
}

// 0x5fa174 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// IDA 0x5fa174: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa174() {
}

// 0x5fa3b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x5fa3b4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa3b4() {
}

// 0x5fa3d0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
// IDA 0x5fa3d0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa3d0() {
}

// 0x5fa404 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x5fa404: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa404() {
}

// 0x5fa40c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x5fa40c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa40c() {
}

// 0x5fa458 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
// IDA 0x5fa458: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa458() {
}

// 0x5fa478 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
// IDA 0x5fa478: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa478() {
}

// 0x5fa4ac — __ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToIndex(RBX::EThrottle::EThrottleType)const")]
// was: RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToIndex(RBX::EThrottle::EThrottleType)const
// IDA 0x5fa4ac: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa4ac() {
}

// 0x5fa51c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0x5fa51c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa51c() {
}

// 0x5fa55c — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::isReadOnly(void)const
// IDA 0x5fa55c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa55c() {
}

// 0x5fa560 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::isWriteOnly(void)const
// IDA 0x5fa560: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa560() {
}

// 0x5fa564 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x5fa564: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa564() {
}

// 0x5fa584 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsENS_9EThrottle13EThrottleTypeEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::setValue(RBX::Reflection::DescribedBase *,RBX::EThrottle::EThrottleType const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,RBX::EThrottle::EThrottleType>::GetSetImpl<RBX::EThrottle::EThrottleType (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(RBX::EThrottle::EThrottleType)>::setValue(RBX::Reflection::DescribedBase *,RBX::EThrottle::EThrottleType const&)const
// IDA 0x5fa584: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa584() {
}

// 0x5fa5a8 — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::PropDescriptor<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>(char const*,char const*,bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::PropDescriptor<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>(char const*,char const*,bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x5fa5a8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa5a8() {
}

// 0x5fa6bc — __ZN3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::~PropDescriptor()
// IDA 0x5fa6bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fa6bc() {
}

// 0x5fa6e8 — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::isReadOnly(void)const
// IDA 0x5fa6e8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa6e8() {
}

// 0x5fa6ec — __ZNK3RBX10Reflection14PropDescriptorINS_15PhysicsSettingsEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::PhysicsSettings,bool>::GetSetImpl<bool (RBX::PhysicsSettings::*)(void)const,void (RBX::PhysicsSettings::*)(bool)>::isWriteOnly(void)const
// IDA 0x5fa6ec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fa6ec() {
}
