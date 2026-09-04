//! rendering — next 120 Ogre/G3D stubs (EA-sorted filter Ogre|G3D)
//! Filter: Ogre|G3D (13663 total, 2036 prior stubbed, +120 this batch) — 0xcd0afc..0xcd89a8 after 0xcd0a4c (remaining 7980 after batch)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcd0afc — __ZN4Ogre9SharedPtrINS_13AnimableValueEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()
// IDA 0xcd0afc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd0afc() {
}

// 0xcd0bf0 — __ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)
// IDA 0xcd0bf0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd0bf0() {
}

// 0xcd0c28 — __ZN4Ogre9SharedPtrINS_13AnimableValueEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::swap(Ogre::SharedPtr<Ogre::AnimableValue>&)")]
// was: Ogre::SharedPtr<Ogre::AnimableValue>::swap(Ogre::SharedPtr<Ogre::AnimableValue>&)
// IDA 0xcd0c28: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd0c28() {
}

// 0xcd0c44 — __ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)")]
// was: std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)
// IDA 0xcd0c44: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_cd0c44() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xcd0f04 — __ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias = "Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xcd0f04: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd0f04() {
}

// 0xcd103c — __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xcd103c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd103c() {
}

// 0xcd1048 — __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcd1048: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd1048() {
}

// 0xcd104c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xcd104c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd104c() {
}

// 0xcd1050 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xcd1050: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd1050() {
}

// 0xcd105c — __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcd105c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd105c() {
}

// 0xcd1068 — __ZN4Ogre13AnimableValue14setAsBaseValueEi
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(int)")]
// was: Ogre::AnimableValue::setAsBaseValue(int)
// IDA 0xcd1068: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1068() {
}

// 0xcd106c — __ZN4Ogre13AnimableValue14setAsBaseValueEf
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(float)")]
// was: Ogre::AnimableValue::setAsBaseValue(float)
// IDA 0xcd106c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd106c() {
}

// 0xcd1070 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector2E
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector2 const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Vector2 const&)
// IDA 0xcd1070: 3 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1070() {
}

// 0xcd107c — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector3E
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector3 const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Vector3 const&)
// IDA 0xcd107c: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd107c() {
}

// 0xcd108c — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector4 const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Vector4 const&)
// IDA 0xcd108c: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd108c() {
}

// 0xcd1098 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_10QuaternionE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Quaternion const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Quaternion const&)
// IDA 0xcd1098: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1098() {
}

// 0xcd10a4 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_11ColourValueE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::ColourValue const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::ColourValue const&)
// IDA 0xcd10a4: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd10a4() {
}

// 0xcd10b8 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Radian const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Radian const&)
// IDA 0xcd10b8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd10b8() {
}

// 0xcd10c0 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Degree const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Degree const&)
// IDA 0xcd10c0: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd10c0() {
}

// 0xcd10e4 — __ZN4Ogre26LightSpotlightFalloffValueD1Ev
#[doc(alias = "Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")]
// was: Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()
// IDA 0xcd10e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd10e4() {
}

// 0xcd10e8 — __ZN4Ogre26LightSpotlightFalloffValueD0Ev
#[doc(alias = "Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")]
// was: Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()
// IDA 0xcd10e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd10e8() {
}

// 0xcd1174 — __ZN4Ogre26LightSpotlightFalloffValue26setCurrentStateAsBaseValueEv
#[doc(alias = "Ogre::LightSpotlightFalloffValue::setCurrentStateAsBaseValue(void)")]
// was: Ogre::LightSpotlightFalloffValue::setCurrentStateAsBaseValue(void)
// IDA 0xcd1174: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1174() {
}

// 0xcd1188 — __ZN4Ogre13AnimableValue8setValueEi
#[doc(alias = "Ogre::AnimableValue::setValue(int)")]
// was: Ogre::AnimableValue::setValue(int)
// IDA 0xcd1188: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1188() {
}

// 0xcd1338 — __ZN4Ogre26LightSpotlightFalloffValue8setValueEf
#[doc(alias = "Ogre::LightSpotlightFalloffValue::setValue(float)")]
// was: Ogre::LightSpotlightFalloffValue::setValue(float)
// IDA 0xcd1338: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1338() {
}

// 0xcd1340 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector2E
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector2 const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Vector2 const&)
// IDA 0xcd1340: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1340() {
}

// 0xcd14f0 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector3E
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector3 const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Vector3 const&)
// IDA 0xcd14f0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd14f0() {
}

// 0xcd16a0 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector4 const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Vector4 const&)
// IDA 0xcd16a0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd16a0() {
}

// 0xcd1850 — __ZN4Ogre13AnimableValue8setValueERKNS_10QuaternionE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Quaternion const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Quaternion const&)
// IDA 0xcd1850: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1850() {
}

// 0xcd1a00 — __ZN4Ogre13AnimableValue8setValueERKNS_11ColourValueE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::ColourValue const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::ColourValue const&)
// IDA 0xcd1a00: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1a00() {
}

// 0xcd1bb0 — __ZN4Ogre13AnimableValue8setValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Radian const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Radian const&)
// IDA 0xcd1bb0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1bb0() {
}

// 0xcd1d60 — __ZN4Ogre13AnimableValue8setValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Degree const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Degree const&)
// IDA 0xcd1d60: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1d60() {
}

// 0xcd1f10 — __ZN4Ogre13AnimableValue15applyDeltaValueEi
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(int)")]
// was: Ogre::AnimableValue::applyDeltaValue(int)
// IDA 0xcd1f10: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1f10() {
}

// 0xcd20c0 — __ZN4Ogre26LightSpotlightFalloffValue15applyDeltaValueEf
#[doc(alias = "Ogre::LightSpotlightFalloffValue::applyDeltaValue(float)")]
// was: Ogre::LightSpotlightFalloffValue::applyDeltaValue(float)
// IDA 0xcd20c0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd20c0() {
}

// 0xcd20e0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector2E
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector2 const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Vector2 const&)
// IDA 0xcd20e0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd20e0() {
}

// 0xcd2290 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector3E
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector3 const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Vector3 const&)
// IDA 0xcd2290: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2290() {
}

// 0xcd2440 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector4 const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Vector4 const&)
// IDA 0xcd2440: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2440() {
}

// 0xcd25f0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_10QuaternionE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Quaternion const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Quaternion const&)
// IDA 0xcd25f0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd25f0() {
}

// 0xcd27a0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_11ColourValueE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::ColourValue const&)
// IDA 0xcd27a0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd27a0() {
}

// 0xcd2950 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Degree const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Degree const&)
// IDA 0xcd2950: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2950() {
}

// 0xcd2b00 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Radian const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Radian const&)
// IDA 0xcd2b00: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2b00() {
}

// 0xcd2cb0 — __ZN4Ogre13AnimableValue8setValueEf
#[doc(alias = "Ogre::AnimableValue::setValue(float)")]
// was: Ogre::AnimableValue::setValue(float)
// IDA 0xcd2cb0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2cb0() {
}

// 0xcd2e60 — __ZN4Ogre13AnimableValue15applyDeltaValueEf
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(float)")]
// was: Ogre::AnimableValue::applyDeltaValue(float)
// IDA 0xcd2e60: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2e60() {
}

// 0xcd3010 — __ZN4Ogre24LightSpotlightOuterValueD1Ev
#[doc(alias = "Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")]
// was: Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()
// IDA 0xcd3010: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd3010() {
}

// 0xcd3014 — __ZN4Ogre24LightSpotlightOuterValueD0Ev
#[doc(alias = "Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")]
// was: Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()
// IDA 0xcd3014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3014() {
}

// 0xcd30a0 — __ZN4Ogre24LightSpotlightOuterValue26setCurrentStateAsBaseValueEv
#[doc(alias = "Ogre::LightSpotlightOuterValue::setCurrentStateAsBaseValue(void)")]
// was: Ogre::LightSpotlightOuterValue::setCurrentStateAsBaseValue(void)
// IDA 0xcd30a0: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd30a0() {
}

// 0xcd30b4 — __ZN4Ogre24LightSpotlightOuterValue8setValueEf
#[doc(alias = "Ogre::LightSpotlightOuterValue::setValue(float)")]
// was: Ogre::LightSpotlightOuterValue::setValue(float)
// IDA 0xcd30b4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd30b4() {
}

// 0xcd30bc — __ZN4Ogre24LightSpotlightOuterValue15applyDeltaValueEf
#[doc(alias = "Ogre::LightSpotlightOuterValue::applyDeltaValue(float)")]
// was: Ogre::LightSpotlightOuterValue::applyDeltaValue(float)
// IDA 0xcd30bc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd30bc() {
}

// 0xcd30dc — __ZN4Ogre24LightSpotlightInnerValueD1Ev
#[doc(alias = "Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")]
// was: Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()
// IDA 0xcd30dc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd30dc() {
}

// 0xcd30e0 — __ZN4Ogre24LightSpotlightInnerValueD0Ev
#[doc(alias = "Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")]
// was: Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()
// IDA 0xcd30e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd30e0() {
}

// 0xcd316c — __ZN4Ogre24LightSpotlightInnerValue26setCurrentStateAsBaseValueEv
#[doc(alias = "Ogre::LightSpotlightInnerValue::setCurrentStateAsBaseValue(void)")]
// was: Ogre::LightSpotlightInnerValue::setCurrentStateAsBaseValue(void)
// IDA 0xcd316c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd316c() {
}

// 0xcd3180 — __ZN4Ogre24LightSpotlightInnerValue8setValueEf
#[doc(alias = "Ogre::LightSpotlightInnerValue::setValue(float)")]
// was: Ogre::LightSpotlightInnerValue::setValue(float)
// IDA 0xcd3180: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3180() {
}

// 0xcd3188 — __ZN4Ogre24LightSpotlightInnerValue15applyDeltaValueEf
#[doc(alias = "Ogre::LightSpotlightInnerValue::applyDeltaValue(float)")]
// was: Ogre::LightSpotlightInnerValue::applyDeltaValue(float)
// IDA 0xcd3188: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3188() {
}

// 0xcd31a8 — __ZN4Ogre21LightAttenuationValueD1Ev
#[doc(alias = "Ogre::LightAttenuationValue::~LightAttenuationValue()")]
// was: Ogre::LightAttenuationValue::~LightAttenuationValue()
// IDA 0xcd31a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd31a8() {
}

// 0xcd31ac — __ZN4Ogre21LightAttenuationValueD0Ev
#[doc(alias = "Ogre::LightAttenuationValue::~LightAttenuationValue()")]
// was: Ogre::LightAttenuationValue::~LightAttenuationValue()
// IDA 0xcd31ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd31ac() {
}

// 0xcd3238 — __ZN4Ogre21LightAttenuationValue26setCurrentStateAsBaseValueEv
#[doc(alias = "Ogre::LightAttenuationValue::setCurrentStateAsBaseValue(void)")]
// was: Ogre::LightAttenuationValue::setCurrentStateAsBaseValue(void)
// IDA 0xcd3238: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3238() {
}

// 0xcd32b0 — __ZN4Ogre21LightAttenuationValue8setValueERKNS_7Vector4E
#[doc(alias = "Ogre::LightAttenuationValue::setValue(Ogre::Vector4 const&)")]
// was: Ogre::LightAttenuationValue::setValue(Ogre::Vector4 const&)
// IDA 0xcd32b0: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd32b0() {
}

// 0xcd32c0 — __ZN4Ogre21LightAttenuationValue15applyDeltaValueERKNS_7Vector4E
#[doc(alias = "Ogre::LightAttenuationValue::applyDeltaValue(Ogre::Vector4 const&)")]
// was: Ogre::LightAttenuationValue::applyDeltaValue(Ogre::Vector4 const&)
// IDA 0xcd32c0: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd32c0() {
}

// 0xcd335c — __ZN4Ogre24LightSpecularColourValueD1Ev
#[doc(alias = "Ogre::LightSpecularColourValue::~LightSpecularColourValue()")]
// was: Ogre::LightSpecularColourValue::~LightSpecularColourValue()
// IDA 0xcd335c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd335c() {
}

// 0xcd3360 — __ZN4Ogre24LightSpecularColourValueD0Ev
#[doc(alias = "Ogre::LightSpecularColourValue::~LightSpecularColourValue()")]
// was: Ogre::LightSpecularColourValue::~LightSpecularColourValue()
// IDA 0xcd3360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3360() {
}

// 0xcd33ec — __ZN4Ogre24LightSpecularColourValue26setCurrentStateAsBaseValueEv
#[doc(alias = "Ogre::LightSpecularColourValue::setCurrentStateAsBaseValue(void)")]
// was: Ogre::LightSpecularColourValue::setCurrentStateAsBaseValue(void)
// IDA 0xcd33ec: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd33ec() {
}

// 0xcd33fc — __ZN4Ogre24LightSpecularColourValue8setValueERKNS_11ColourValueE
#[doc(alias = "Ogre::LightSpecularColourValue::setValue(Ogre::ColourValue const&)")]
// was: Ogre::LightSpecularColourValue::setValue(Ogre::ColourValue const&)
// IDA 0xcd33fc: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd33fc() {
}

// 0xcd340c — __ZN4Ogre24LightSpecularColourValue15applyDeltaValueERKNS_11ColourValueE
#[doc(alias = "Ogre::LightSpecularColourValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: Ogre::LightSpecularColourValue::applyDeltaValue(Ogre::ColourValue const&)
// IDA 0xcd340c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd340c() {
}

// 0xcd3464 — __ZN4Ogre23LightDiffuseColourValueD1Ev
#[doc(alias = "Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")]
// was: Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()
// IDA 0xcd3464: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd3464() {
}

// 0xcd3468 — __ZN4Ogre23LightDiffuseColourValueD0Ev
#[doc(alias = "Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")]
// was: Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()
// IDA 0xcd3468: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3468() {
}

// 0xcd34f4 — __ZN4Ogre23LightDiffuseColourValue26setCurrentStateAsBaseValueEv
#[doc(alias = "Ogre::LightDiffuseColourValue::setCurrentStateAsBaseValue(void)")]
// was: Ogre::LightDiffuseColourValue::setCurrentStateAsBaseValue(void)
// IDA 0xcd34f4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd34f4() {
}

// 0xcd3504 — __ZN4Ogre23LightDiffuseColourValue8setValueERKNS_11ColourValueE
#[doc(alias = "Ogre::LightDiffuseColourValue::setValue(Ogre::ColourValue const&)")]
// was: Ogre::LightDiffuseColourValue::setValue(Ogre::ColourValue const&)
// IDA 0xcd3504: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3504() {
}

// 0xcd3514 — __ZN4Ogre23LightDiffuseColourValue15applyDeltaValueERKNS_11ColourValueE
#[doc(alias = "Ogre::LightDiffuseColourValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: Ogre::LightDiffuseColourValue::applyDeltaValue(Ogre::ColourValue const&)
// IDA 0xcd3514: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3514() {
}

// 0xcd356c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)
// IDA 0xcd356c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd356c() {
}

// 0xcd3600 — __ZN4Ogre11LodStrategyC2ERKSs
#[doc(alias = "Ogre::LodStrategy::LodStrategy(std::string const&)")]
// was: Ogre::LodStrategy::LodStrategy(std::string const&)
// IDA 0xcd3600: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3600() {
}

// 0xcd3624 — __ZN4Ogre11LodStrategyD0Ev
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: Ogre::LodStrategy::~LodStrategy()
// IDA 0xcd3624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3624() {
}

// 0xcd36fc — __ZN4Ogre11LodStrategyD1Ev
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: Ogre::LodStrategy::~LodStrategy()
// IDA 0xcd36fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd36fc() {
}

// 0xcd3758 — __ZN4Ogre11LodStrategyD2Ev
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: Ogre::LodStrategy::~LodStrategy()
// IDA 0xcd3758: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3758() {
}

// 0xcd37b4 — __ZNK4Ogre11LodStrategy18transformUserValueEf
#[doc(alias = "Ogre::LodStrategy::transformUserValue(float)const")]
// was: Ogre::LodStrategy::transformUserValue(float)const
// IDA 0xcd37b4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd37b4() {
}

// 0xcd37b8 — __ZNK4Ogre11LodStrategy8getValueEPKNS_13MovableObjectEPKNS_6CameraE
#[doc(alias = "Ogre::LodStrategy::getValue(Ogre::MovableObject const*,Ogre::Camera const*)const")]
// was: Ogre::LodStrategy::getValue(Ogre::MovableObject const*,Ogre::Camera const*)const
// IDA 0xcd37b8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd37b8() {
}

// 0xcd37d8 — __ZN4Ogre11LodStrategy17isSortedAscendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::isSortedAscending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::LodStrategy::isSortedAscending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcd37d8: 16 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd37d8() {
}

// 0xcd3804 — __ZN4Ogre11LodStrategy18isSortedDescendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::isSortedDescending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::LodStrategy::isSortedDescending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcd3804: 16 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3804() {
}

// 0xcd3830 — __ZN4Ogre11LodStrategy13sortAscendingERSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::sortAscending(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// was: Ogre::LodStrategy::sortAscending(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)
// IDA 0xcd3830: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3830() {
}

// 0xcd3874 — __ZN4Ogre11LodStrategy14sortDescendingERSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::sortDescending(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// was: Ogre::LodStrategy::sortDescending(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)
// IDA 0xcd3874: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3874() {
}

// 0xcd38b8 — __ZN4Ogre11LodStrategy17getIndexAscendingEfRKSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::getIndexAscending(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::LodStrategy::getIndexAscending(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcd38b8: 32 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd38b8() {
}

// 0xcd3910 — __ZN4Ogre11LodStrategy18getIndexDescendingEfRKSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::getIndexDescending(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::LodStrategy::getIndexDescending(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcd3910: 32 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3910() {
}

// 0xcd3968 — __ZN4Ogre11LodStrategy17getIndexAscendingEfRKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::getIndexAscending(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::LodStrategy::getIndexAscending(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcd3968: 30 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3968() {
}

// 0xcd39b8 — __ZN4Ogre11LodStrategy18getIndexDescendingEfRKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::getIndexDescending(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::LodStrategy::getIndexDescending(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xcd39b8: 30 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd39b8() {
}

// 0xcd3a08 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_19LodUsageSortGreaterEEvT_SE_T0_T1_
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater)")]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortGreater)
// IDA 0xcd3a08: 386 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3a08() {
}

// 0xcd3e24 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xcd3e24: 309 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3e24() {
}

// 0xcd4160 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xcd4160: 590 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd4160() {
}

// 0xcd4798 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xcd4798: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd4798() {
}

// 0xcd482c — __ZNSt11__iter_swapILb1EE9iter_swapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS5_NS4_12STLAllocatorIS5_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEEEESE_EEvT_T0_
#[doc(alias = "void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)")]
// was: void std::__iter_swap<true>::iter_swap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>)
// IDA 0xcd482c: 330 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd482c() {
}

// 0xcd4ba8 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_SE_T0_
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xcd4ba8: 304 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd4ba8() {
}

// 0xcd4ee0 — __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
// was: void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xcd4ee0: 285 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd4ee0() {
}

// 0xcd51f0 — __ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_19LodUsageSortGreaterEEvT_SE_SE_T0_T1_
#[doc(alias = "void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
// was: void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xcd51f0: 298 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd51f0() {
}

// 0xcd552c — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xcd552c: 367 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd552c() {
}

// 0xcd5938 — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_19LodUsageSortGreaterEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)")]
// was: void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortGreater)
// IDA 0xcd5938: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd5938() {
}

// 0xcd59f8 — __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_19LodUsageSortGreaterEEvT_SE_T0_
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)")]
// was: void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortGreater)
// IDA 0xcd59f8: 312 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd59f8() {
}

// 0xcd5d38 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_16LodUsageSortLessEEvT_SE_T0_T1_
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess)")]
// was: void std::__introsort_loop<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::LodUsageSortLess)
// IDA 0xcd5d38: 386 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd5d38() {
}

// 0xcd6154 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
// was: void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xcd6154: 309 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd6154() {
}

// 0xcd6490 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xcd6490: 590 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd6490() {
}

// 0xcd6ac8 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
// was: void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xcd6ac8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd6ac8() {
}

// 0xcd6b5c — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_SE_T0_
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
// was: void std::__heap_select<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xcd6b5c: 304 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd6b5c() {
}

// 0xcd6e94 — __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
// was: void std::pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xcd6e94: 285 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd6e94() {
}

// 0xcd71a4 — __ZSt10__pop_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_16LodUsageSortLessEEvT_SE_SE_T0_T1_
#[doc(alias = "void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
// was: void std::__pop_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xcd71a4: 298 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd71a4() {
}

// 0xcd74e0 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
// was: void std::__adjust_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xcd74e0: 367 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd74e0() {
}

// 0xcd78ec — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS3_NS2_16LodUsageSortLessEEvT_T0_SF_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)")]
// was: void std::__push_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::MeshLodUsage,Ogre::LodUsageSortLess)
// IDA 0xcd78ec: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd78ec() {
}

// 0xcd79ac — __ZSt9make_heapIN9__gnu_cxx17__normal_iteratorIPN4Ogre12MeshLodUsageESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_16LodUsageSortLessEEvT_SE_T0_
#[doc(alias = "void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)")]
// was: void std::make_heap<__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess>(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage *,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LodUsageSortLess)
// IDA 0xcd79ac: 312 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd79ac() {
}

// 0xcd7d20 — __ZN4Ogre18LodStrategyManager12getSingletonEv
#[doc(alias = "Ogre::LodStrategyManager::getSingleton(void)")]
// was: Ogre::LodStrategyManager::getSingleton(void)
// IDA 0xcd7d20: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd7d20() {
}

// 0xcd7d30 — __ZN4Ogre18LodStrategyManagerC1Ev
#[doc(alias = "Ogre::LodStrategyManager::LodStrategyManager(void)")]
// was: Ogre::LodStrategyManager::LodStrategyManager(void)
// IDA 0xcd7d30: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd7d30() {
}

// 0xcd7d3c — __ZN4Ogre18LodStrategyManagerC2Ev
#[doc(alias = "Ogre::LodStrategyManager::LodStrategyManager(void)")]
// was: Ogre::LodStrategyManager::LodStrategyManager(void)
// IDA 0xcd7d3c: 138 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd7d3c() {
}

// 0xcd7eb0 — __ZN4Ogre18LodStrategyManager11addStrategyEPNS_11LodStrategyE
#[doc(alias = "Ogre::LodStrategyManager::addStrategy(Ogre::LodStrategy *)")]
// was: Ogre::LodStrategyManager::addStrategy(Ogre::LodStrategy *)
// IDA 0xcd7eb0: 315 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd7eb0() {
}

// 0xcd8250 — __ZN4Ogre18LodStrategyManagerD1Ev
#[doc(alias = "Ogre::LodStrategyManager::~LodStrategyManager()")]
// was: Ogre::LodStrategyManager::~LodStrategyManager()
// IDA 0xcd8250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd8250() {
}

// 0xcd825c — __ZN4Ogre18LodStrategyManagerD2Ev
#[doc(alias = "Ogre::LodStrategyManager::~LodStrategyManager()")]
// was: Ogre::LodStrategyManager::~LodStrategyManager()
// IDA 0xcd825c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd825c() {
}

// 0xcd838c — __ZN4Ogre18LodStrategyManager11getStrategyERKSs
#[doc(alias = "Ogre::LodStrategyManager::getStrategy(std::string const&)")]
// was: Ogre::LodStrategyManager::getStrategy(std::string const&)
// IDA 0xcd838c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd838c() {
}

// 0xcd83c4 — __ZN4Ogre18LodStrategyManager18getDefaultStrategyEv
#[doc(alias = "Ogre::LodStrategyManager::getDefaultStrategy(void)")]
// was: Ogre::LodStrategyManager::getDefaultStrategy(void)
// IDA 0xcd83c4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd83c4() {
}

// 0xcd83c8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::LodStrategy *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::LodStrategy *>> *)
// IDA 0xcd83c8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd83c8() {
}

// 0xcd8440 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xcd8440: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd8440() {
}

// 0xcd84e4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::LodStrategy *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::LodStrategy *> const&)
// IDA 0xcd84e4: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd84e4() {
}

// 0xcd85c8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::LodStrategy *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::LodStrategy *> const&)
// IDA 0xcd85c8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd85c8() {
}

// 0xcd871c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xcd871c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd871c() {
}

// 0xcd8720 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre11LodStrategyEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::LodStrategy *>,std::_Select1st<std::pair<std::string const,Ogre::LodStrategy *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::LodStrategy *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xcd8720: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd8720() {
}

// 0xcd8760 — __ZN4Ogre3LogC1ERKSsbb
#[doc(alias = "Ogre::Log::Log(std::string const&,bool,bool)")]
// was: Ogre::Log::Log(std::string const&,bool,bool)
// IDA 0xcd8760: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd8760() {
}

// 0xcd876c — __ZN4Ogre3LogC2ERKSsbb
#[doc(alias = "Ogre::Log::Log(std::string const&,bool,bool)")]
// was: Ogre::Log::Log(std::string const&,bool,bool)
// IDA 0xcd876c: 202 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd876c() {
}

// 0xcd89a8 — __ZN4Ogre3LogD1Ev
#[doc(alias = "Ogre::Log::~Log()")]
// was: Ogre::Log::~Log()
// IDA 0xcd89a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd89a8() {
}
