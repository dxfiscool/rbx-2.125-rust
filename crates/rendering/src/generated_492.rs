//! rendering shard 492 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xccea78..0xcd3804, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|Gfx|Render|G3D), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xccea78 — __ZNK4Ogre5Light22_getFrustumClipVolumesEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::Light::_getFrustumClipVolumes(Ogre::Camera const*)const")]
// was: __ZNK4Ogre5Light22_getFrustumClipVolumesEPKNS_6CameraE
// IDA 0xccea78: 532 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccea78() {
}


// 0xccf0d0 — __ZNK4Ogre5Light12getTypeFlagsEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getTypeFlags(void)const")]
// was: __ZNK4Ogre5Light12getTypeFlagsEv
// IDA 0xccf0d0: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf0d0() {
}


// 0xccf0e0 — __ZN4Ogre5Light19_calcTempSquareDistERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::Light::_calcTempSquareDist(Ogre::Vector3 const&)")]
// was: __ZN4Ogre5Light19_calcTempSquareDistERKNS_7Vector3E
// IDA 0xccf0e0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf0e0() {
}


// 0xccf140 — __ZNK4Ogre5Light25getAnimableDictionaryNameEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getAnimableDictionaryName(void)const")]
// was: __ZNK4Ogre5Light25getAnimableDictionaryNameEv
// IDA 0xccf140: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf140() {
}


// 0xccf14c — __ZThn4_NK4Ogre5Light25getAnimableDictionaryNameEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Light::getAnimableDictionaryName(void)const")]
// was: __ZThn4_NK4Ogre5Light25getAnimableDictionaryNameEv
// IDA 0xccf14c: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf14c() {
}


// 0xccf158 — __ZNK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::Light::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")]
// was: __ZNK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xccf158: 479 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf158() {
}


// 0xccf684 — __ZThn4_NK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "non-virtual thunk toOgre::Light::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")]
// was: __ZThn4_NK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xccf684: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf684() {
}


// 0xccf690 — __ZN4Ogre5Light19createAnimableValueERKSs
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const std::string *)
#[doc(alias = "Ogre::Light::createAnimableValue(std::string const&)")]
// was: __ZN4Ogre5Light19createAnimableValueERKSs
// IDA 0xccf690: 153 insns (PUSH..NOP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf690() {
}


// 0xccf824 — __ZThn4_N4Ogre5Light19createAnimableValueERKSs
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toOgre::Light::createAnimableValue(std::string const&)")]
// was: __ZThn4_N4Ogre5Light19createAnimableValueERKSs
// IDA 0xccf824: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf824() {
}


// 0xccf834 — __ZNK4Ogre5Light26getCustomShadowCameraSetupEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getCustomShadowCameraSetup(void)const")]
// was: __ZNK4Ogre5Light26getCustomShadowCameraSetupEv
// IDA 0xccf834: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf834() {
}


// 0xccf840 — __ZNK4Ogre5Light20getShadowFarDistanceEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getShadowFarDistance(void)const")]
// was: __ZNK4Ogre5Light20getShadowFarDistanceEv
// IDA 0xccf840: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf840() {
}


// 0xccf86c — __ZNK4Ogre5Light27getShadowFarDistanceSquaredEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getShadowFarDistanceSquared(void)const")]
// was: __ZNK4Ogre5Light27getShadowFarDistanceSquaredEv
// IDA 0xccf86c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf86c() {
}


// 0xccf898 — __ZN4Ogre5Light18_setCameraRelativeEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::Light::_setCameraRelative(Ogre::Camera *)")]
// was: __ZN4Ogre5Light18_setCameraRelativeEPNS_6CameraE
// IDA 0xccf898: 4 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf898() {
}


// 0xccf8a4 — __ZNK4Ogre5Light29_deriveShadowNearClipDistanceEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::Light::_deriveShadowNearClipDistance(Ogre::Camera const*)const")]
// was: __ZNK4Ogre5Light29_deriveShadowNearClipDistanceEPKNS_6CameraE
// IDA 0xccf8a4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf8a4() {
}


// 0xccf8cc — __ZNK4Ogre5Light28_deriveShadowFarClipDistanceEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::Light::_deriveShadowFarClipDistance(Ogre::Camera const*)const")]
// was: __ZNK4Ogre5Light28_deriveShadowFarClipDistanceEPKNS_6CameraE
// IDA 0xccf8cc: 11 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf8cc() {
}


// 0xccf8f0 — __ZN4Ogre5Light18setCustomParameterEtRKNS_7Vector4E
#[doc(alias = "Ogre::Light::setCustomParameter(unsigned short,Ogre::Vector4 const&)")]
// was: __ZN4Ogre5Light18setCustomParameterEtRKNS_7Vector4E
// IDA 0xccf8f0: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf8f0() {
}


// 0xccf95c — __ZNK4Ogre5Light25_updateCustomGpuParameterEtRKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "Ogre::Light::_updateCustomGpuParameter(unsigned short,Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const")]
// was: __ZNK4Ogre5Light25_updateCustomGpuParameterEtRKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
// IDA 0xccf95c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf95c() {
}


// 0xccf9b4 — __ZNK4Ogre12LightFactory7getTypeEv
// type: _DWORD __fastcall(Ogre::LightFactory *__hidden this)
#[doc(alias = "Ogre::LightFactory::getType(void)const")]
// was: __ZNK4Ogre12LightFactory7getTypeEv
// IDA 0xccf9b4: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccf9b4() {
}


// 0xcd0648 — __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
// IDA 0xcd0648: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd0648() {
}


// 0xcd064c — __ZN4Ogre14AxisAlignedBoxD1Ev
// type: void __fastcall(Ogre::AxisAlignedBox *__hidden this)
#[doc(alias = "Ogre::AxisAlignedBox::~AxisAlignedBox()")]
// was: __ZN4Ogre14AxisAlignedBoxD1Ev
// IDA 0xcd064c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd064c() {
}


// 0xcd06e0 — __ZNK4Ogre5Light17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getBoundingRadius(void)const")]
// was: __ZNK4Ogre5Light17getBoundingRadiusEv
// IDA 0xcd06e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd06e0() {
}


// 0xcd06e4 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// IDA 0xcd06e4: 233 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd06e4() {
}


// 0xcd093c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// IDA 0xcd093c: 105 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd093c() {
}


// 0xcd0a4c — __ZN4Ogre9SharedPtrINS_13AnimableValueEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEED1Ev
// IDA 0xcd0a4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd0a4c() {
}


// 0xcd0afc — __ZN4Ogre9SharedPtrINS_13AnimableValueEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEED0Ev
// IDA 0xcd0afc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd0afc() {
}


// 0xcd0bf0 — __ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
// IDA 0xcd0bf0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd0bf0() {
}


// 0xcd0c28 — __ZN4Ogre9SharedPtrINS_13AnimableValueEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::swap(Ogre::SharedPtr<Ogre::AnimableValue>&)")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEE4swapERS2_
// IDA 0xcd0c28: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd0c28() {
}


// 0xcd0c44 — __ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)")]
// was: __ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
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
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: __ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// IDA 0xcd0f04: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd0f04() {
}


// 0xcd103c — __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
// IDA 0xcd103c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd103c() {
}


// 0xcd1048 — __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xcd1048: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd1048() {
}


// 0xcd104c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
// IDA 0xcd104c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd104c() {
}


// 0xcd1050 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
// IDA 0xcd1050: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd1050() {
}


// 0xcd105c — __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xcd105c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd105c() {
}


// 0xcd1068 — __ZN4Ogre13AnimableValue14setAsBaseValueEi
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, int)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(int)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueEi
// IDA 0xcd1068: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1068() {
}


// 0xcd106c — __ZN4Ogre13AnimableValue14setAsBaseValueEf
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, float)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(float)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueEf
// IDA 0xcd106c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd106c() {
}


// 0xcd1070 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector2 const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector2E
// IDA 0xcd1070: 3 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1070() {
}


// 0xcd107c — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector3 const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector3E
// IDA 0xcd107c: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd107c() {
}


// 0xcd108c — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector4E
// IDA 0xcd108c: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd108c() {
}


// 0xcd1098 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_10QuaternionE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Quaternion const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_10QuaternionE
// IDA 0xcd1098: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1098() {
}


// 0xcd10a4 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_11ColourValueE
// IDA 0xcd10a4: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd10a4() {
}


// 0xcd10b8 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Radian const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6RadianE
// IDA 0xcd10b8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd10b8() {
}


// 0xcd10c0 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Degree const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6DegreeE
// IDA 0xcd10c0: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd10c0() {
}


// 0xcd10e4 — __ZN4Ogre26LightSpotlightFalloffValueD1Ev
// type: void __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")]
// was: __ZN4Ogre26LightSpotlightFalloffValueD1Ev
// IDA 0xcd10e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd10e4() {
}


// 0xcd10e8 — __ZN4Ogre26LightSpotlightFalloffValueD0Ev
// type: void __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")]
// was: __ZN4Ogre26LightSpotlightFalloffValueD0Ev
// IDA 0xcd10e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd10e8() {
}


// 0xcd1174 — __ZN4Ogre26LightSpotlightFalloffValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre26LightSpotlightFalloffValue26setCurrentStateAsBaseValueEv
// IDA 0xcd1174: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1174() {
}


// 0xcd1188 — __ZN4Ogre13AnimableValue8setValueEi
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, int)
#[doc(alias = "Ogre::AnimableValue::setValue(int)")]
// was: __ZN4Ogre13AnimableValue8setValueEi
// IDA 0xcd1188: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1188() {
}


// 0xcd1338 — __ZN4Ogre26LightSpotlightFalloffValue8setValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::setValue(float)")]
// was: __ZN4Ogre26LightSpotlightFalloffValue8setValueEf
// IDA 0xcd1338: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1338() {
}


// 0xcd1340 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector2 const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_7Vector2E
// IDA 0xcd1340: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1340() {
}


// 0xcd14f0 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Vector3 *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector3 const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_7Vector3E
// IDA 0xcd14f0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd14f0() {
}


// 0xcd16a0 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_7Vector4E
// IDA 0xcd16a0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd16a0() {
}


// 0xcd1850 — __ZN4Ogre13AnimableValue8setValueERKNS_10QuaternionE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Quaternion *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Quaternion const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_10QuaternionE
// IDA 0xcd1850: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1850() {
}


// 0xcd1a00 — __ZN4Ogre13AnimableValue8setValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_11ColourValueE
// IDA 0xcd1a00: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1a00() {
}


// 0xcd1bb0 — __ZN4Ogre13AnimableValue8setValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Radian const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_6RadianE
// IDA 0xcd1bb0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1bb0() {
}


// 0xcd1d60 — __ZN4Ogre13AnimableValue8setValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Degree const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_6DegreeE
// IDA 0xcd1d60: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1d60() {
}


// 0xcd1f10 — __ZN4Ogre13AnimableValue15applyDeltaValueEi
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, int)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(int)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueEi
// IDA 0xcd1f10: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd1f10() {
}


// 0xcd20c0 — __ZN4Ogre26LightSpotlightFalloffValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::applyDeltaValue(float)")]
// was: __ZN4Ogre26LightSpotlightFalloffValue15applyDeltaValueEf
// IDA 0xcd20c0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd20c0() {
}


// 0xcd20e0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector2 const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector2E
// IDA 0xcd20e0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd20e0() {
}


// 0xcd2290 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Vector3 *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector3 const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector3E
// IDA 0xcd2290: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2290() {
}


// 0xcd2440 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector4E
// IDA 0xcd2440: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2440() {
}


// 0xcd25f0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_10QuaternionE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Quaternion *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Quaternion const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_10QuaternionE
// IDA 0xcd25f0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd25f0() {
}


// 0xcd27a0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_11ColourValueE
// IDA 0xcd27a0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd27a0() {
}


// 0xcd2950 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Degree const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6DegreeE
// IDA 0xcd2950: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2950() {
}


// 0xcd2b00 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Radian const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6RadianE
// IDA 0xcd2b00: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2b00() {
}


// 0xcd2cb0 — __ZN4Ogre13AnimableValue8setValueEf
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, float)
#[doc(alias = "Ogre::AnimableValue::setValue(float)")]
// was: __ZN4Ogre13AnimableValue8setValueEf
// IDA 0xcd2cb0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2cb0() {
}


// 0xcd2e60 — __ZN4Ogre13AnimableValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, float)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(float)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueEf
// IDA 0xcd2e60: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd2e60() {
}


// 0xcd3010 — __ZN4Ogre24LightSpotlightOuterValueD1Ev
// type: void __fastcall(Ogre::LightSpotlightOuterValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")]
// was: __ZN4Ogre24LightSpotlightOuterValueD1Ev
// IDA 0xcd3010: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd3010() {
}


// 0xcd3014 — __ZN4Ogre24LightSpotlightOuterValueD0Ev
// type: void __fastcall(Ogre::LightSpotlightOuterValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")]
// was: __ZN4Ogre24LightSpotlightOuterValueD0Ev
// IDA 0xcd3014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3014() {
}


// 0xcd30a0 — __ZN4Ogre24LightSpotlightOuterValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpotlightOuterValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightOuterValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre24LightSpotlightOuterValue26setCurrentStateAsBaseValueEv
// IDA 0xcd30a0: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd30a0() {
}


// 0xcd30b4 — __ZN4Ogre24LightSpotlightOuterValue8setValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightOuterValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightOuterValue::setValue(float)")]
// was: __ZN4Ogre24LightSpotlightOuterValue8setValueEf
// IDA 0xcd30b4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd30b4() {
}


// 0xcd30bc — __ZN4Ogre24LightSpotlightOuterValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightOuterValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightOuterValue::applyDeltaValue(float)")]
// was: __ZN4Ogre24LightSpotlightOuterValue15applyDeltaValueEf
// IDA 0xcd30bc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd30bc() {
}


// 0xcd30dc — __ZN4Ogre24LightSpotlightInnerValueD1Ev
// type: void __fastcall(Ogre::LightSpotlightInnerValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")]
// was: __ZN4Ogre24LightSpotlightInnerValueD1Ev
// IDA 0xcd30dc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd30dc() {
}


// 0xcd30e0 — __ZN4Ogre24LightSpotlightInnerValueD0Ev
// type: void __fastcall(Ogre::LightSpotlightInnerValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")]
// was: __ZN4Ogre24LightSpotlightInnerValueD0Ev
// IDA 0xcd30e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd30e0() {
}


// 0xcd316c — __ZN4Ogre24LightSpotlightInnerValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpotlightInnerValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightInnerValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre24LightSpotlightInnerValue26setCurrentStateAsBaseValueEv
// IDA 0xcd316c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd316c() {
}


// 0xcd3180 — __ZN4Ogre24LightSpotlightInnerValue8setValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightInnerValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightInnerValue::setValue(float)")]
// was: __ZN4Ogre24LightSpotlightInnerValue8setValueEf
// IDA 0xcd3180: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3180() {
}


// 0xcd3188 — __ZN4Ogre24LightSpotlightInnerValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightInnerValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightInnerValue::applyDeltaValue(float)")]
// was: __ZN4Ogre24LightSpotlightInnerValue15applyDeltaValueEf
// IDA 0xcd3188: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3188() {
}


// 0xcd31a8 — __ZN4Ogre21LightAttenuationValueD1Ev
// type: void __fastcall(Ogre::LightAttenuationValue *__hidden this)
#[doc(alias = "Ogre::LightAttenuationValue::~LightAttenuationValue()")]
// was: __ZN4Ogre21LightAttenuationValueD1Ev
// IDA 0xcd31a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd31a8() {
}


// 0xcd31ac — __ZN4Ogre21LightAttenuationValueD0Ev
// type: void __fastcall(Ogre::LightAttenuationValue *__hidden this)
#[doc(alias = "Ogre::LightAttenuationValue::~LightAttenuationValue()")]
// was: __ZN4Ogre21LightAttenuationValueD0Ev
// IDA 0xcd31ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd31ac() {
}


// 0xcd3238 — __ZN4Ogre21LightAttenuationValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightAttenuationValue *__hidden this)
#[doc(alias = "Ogre::LightAttenuationValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre21LightAttenuationValue26setCurrentStateAsBaseValueEv
// IDA 0xcd3238: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3238() {
}


// 0xcd32b0 — __ZN4Ogre21LightAttenuationValue8setValueERKNS_7Vector4E
#[doc(alias = "Ogre::LightAttenuationValue::setValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre21LightAttenuationValue8setValueERKNS_7Vector4E
// IDA 0xcd32b0: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd32b0() {
}


// 0xcd32c0 — __ZN4Ogre21LightAttenuationValue15applyDeltaValueERKNS_7Vector4E
#[doc(alias = "Ogre::LightAttenuationValue::applyDeltaValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre21LightAttenuationValue15applyDeltaValueERKNS_7Vector4E
// IDA 0xcd32c0: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd32c0() {
}


// 0xcd335c — __ZN4Ogre24LightSpecularColourValueD1Ev
// type: void __fastcall(Ogre::LightSpecularColourValue *__hidden this)
#[doc(alias = "Ogre::LightSpecularColourValue::~LightSpecularColourValue()")]
// was: __ZN4Ogre24LightSpecularColourValueD1Ev
// IDA 0xcd335c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd335c() {
}


// 0xcd3360 — __ZN4Ogre24LightSpecularColourValueD0Ev
// type: void __fastcall(Ogre::LightSpecularColourValue *__hidden this)
#[doc(alias = "Ogre::LightSpecularColourValue::~LightSpecularColourValue()")]
// was: __ZN4Ogre24LightSpecularColourValueD0Ev
// IDA 0xcd3360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3360() {
}


// 0xcd33ec — __ZN4Ogre24LightSpecularColourValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpecularColourValue *__hidden this)
#[doc(alias = "Ogre::LightSpecularColourValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre24LightSpecularColourValue26setCurrentStateAsBaseValueEv
// IDA 0xcd33ec: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd33ec() {
}


// 0xcd33fc — __ZN4Ogre24LightSpecularColourValue8setValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightSpecularColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightSpecularColourValue::setValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre24LightSpecularColourValue8setValueERKNS_11ColourValueE
// IDA 0xcd33fc: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd33fc() {
}


// 0xcd340c — __ZN4Ogre24LightSpecularColourValue15applyDeltaValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightSpecularColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightSpecularColourValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre24LightSpecularColourValue15applyDeltaValueERKNS_11ColourValueE
// IDA 0xcd340c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd340c() {
}


// 0xcd3464 — __ZN4Ogre23LightDiffuseColourValueD1Ev
// type: void __fastcall(Ogre::LightDiffuseColourValue *__hidden this)
#[doc(alias = "Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")]
// was: __ZN4Ogre23LightDiffuseColourValueD1Ev
// IDA 0xcd3464: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd3464() {
}


// 0xcd3468 — __ZN4Ogre23LightDiffuseColourValueD0Ev
// type: void __fastcall(Ogre::LightDiffuseColourValue *__hidden this)
#[doc(alias = "Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")]
// was: __ZN4Ogre23LightDiffuseColourValueD0Ev
// IDA 0xcd3468: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3468() {
}


// 0xcd34f4 — __ZN4Ogre23LightDiffuseColourValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightDiffuseColourValue *__hidden this)
#[doc(alias = "Ogre::LightDiffuseColourValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre23LightDiffuseColourValue26setCurrentStateAsBaseValueEv
// IDA 0xcd34f4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd34f4() {
}


// 0xcd3504 — __ZN4Ogre23LightDiffuseColourValue8setValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightDiffuseColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightDiffuseColourValue::setValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre23LightDiffuseColourValue8setValueERKNS_11ColourValueE
// IDA 0xcd3504: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3504() {
}


// 0xcd3514 — __ZN4Ogre23LightDiffuseColourValue15applyDeltaValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightDiffuseColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightDiffuseColourValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre23LightDiffuseColourValue15applyDeltaValueERKNS_11ColourValueE
// IDA 0xcd3514: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3514() {
}


// 0xcd356c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// IDA 0xcd356c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd356c() {
}


// 0xcd3600 — __ZN4Ogre11LodStrategyC2ERKSs
// type: _DWORD __fastcall(Ogre::LodStrategy *__hidden this, const std::string *)
#[doc(alias = "Ogre::LodStrategy::LodStrategy(std::string const&)")]
// was: __ZN4Ogre11LodStrategyC2ERKSs
// IDA 0xcd3600: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3600() {
}


// 0xcd3624 — __ZN4Ogre11LodStrategyD0Ev
// type: void __fastcall(Ogre::LodStrategy *__hidden this)
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: __ZN4Ogre11LodStrategyD0Ev
// IDA 0xcd3624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3624() {
}


// 0xcd36fc — __ZN4Ogre11LodStrategyD1Ev
// type: void __fastcall(Ogre::LodStrategy *__hidden this)
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: __ZN4Ogre11LodStrategyD1Ev
// IDA 0xcd36fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd36fc() {
}


// 0xcd3758 — __ZN4Ogre11LodStrategyD2Ev
// type: void __fastcall(Ogre::LodStrategy *__hidden this)
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: __ZN4Ogre11LodStrategyD2Ev
// IDA 0xcd3758: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd3758() {
}


// 0xcd37b4 — __ZNK4Ogre11LodStrategy18transformUserValueEf
// type: _DWORD __fastcall(Ogre::LodStrategy *__hidden this, float)
#[doc(alias = "Ogre::LodStrategy::transformUserValue(float)const")]
// was: __ZNK4Ogre11LodStrategy18transformUserValueEf
// IDA 0xcd37b4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd37b4() {
}


// 0xcd37b8 — __ZNK4Ogre11LodStrategy8getValueEPKNS_13MovableObjectEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::LodStrategy *__hidden this, const Ogre::MovableObject *, const Ogre::Camera *)
#[doc(alias = "Ogre::LodStrategy::getValue(Ogre::MovableObject const*,Ogre::Camera const*)const")]
// was: __ZNK4Ogre11LodStrategy8getValueEPKNS_13MovableObjectEPKNS_6CameraE
// IDA 0xcd37b8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd37b8() {
}


// 0xcd37d8 — __ZN4Ogre11LodStrategy17isSortedAscendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::isSortedAscending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: __ZN4Ogre11LodStrategy17isSortedAscendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xcd37d8: 16 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd37d8() {
}


// 0xcd3804 — __ZN4Ogre11LodStrategy18isSortedDescendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::isSortedDescending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: __ZN4Ogre11LodStrategy18isSortedDescendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xcd3804: 16 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd3804() {
}
