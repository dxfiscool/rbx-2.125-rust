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
pub fn stub_ccea78() -> ! {
    todo!("0xccea78 Ogre::Light::_getFrustumClipVolumes(Ogre::Camera const*)const")
}


// 0xccf0d0 — __ZNK4Ogre5Light12getTypeFlagsEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getTypeFlags(void)const")]
// was: __ZNK4Ogre5Light12getTypeFlagsEv
pub fn stub_ccf0d0() -> ! {
    todo!("0xccf0d0 Ogre::Light::getTypeFlags(void)const")
}


// 0xccf0e0 — __ZN4Ogre5Light19_calcTempSquareDistERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::Light::_calcTempSquareDist(Ogre::Vector3 const&)")]
// was: __ZN4Ogre5Light19_calcTempSquareDistERKNS_7Vector3E
pub fn stub_ccf0e0() -> ! {
    todo!("0xccf0e0 Ogre::Light::_calcTempSquareDist(Ogre::Vector3 const&)")
}


// 0xccf140 — __ZNK4Ogre5Light25getAnimableDictionaryNameEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getAnimableDictionaryName(void)const")]
// was: __ZNK4Ogre5Light25getAnimableDictionaryNameEv
pub fn stub_ccf140() -> ! {
    todo!("0xccf140 Ogre::Light::getAnimableDictionaryName(void)const")
}


// 0xccf14c — __ZThn4_NK4Ogre5Light25getAnimableDictionaryNameEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::Light::getAnimableDictionaryName(void)const")]
// was: __ZThn4_NK4Ogre5Light25getAnimableDictionaryNameEv
pub fn stub_ccf14c() -> ! {
    todo!("0xccf14c `non-virtual thunk to'Ogre::Light::getAnimableDictionaryName(void)const")
}


// 0xccf158 — __ZNK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::Light::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")]
// was: __ZNK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
pub fn stub_ccf158() -> ! {
    todo!("0xccf158 Ogre::Light::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")
}


// 0xccf684 — __ZThn4_NK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "non-virtual thunk toOgre::Light::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")]
// was: __ZThn4_NK4Ogre5Light28initialiseAnimableDictionaryERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
pub fn stub_ccf684() -> ! {
    todo!("0xccf684 `non-virtual thunk to'Ogre::Light::initialiseAnimableDictionary(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")
}


// 0xccf690 — __ZN4Ogre5Light19createAnimableValueERKSs
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const std::string *)
#[doc(alias = "Ogre::Light::createAnimableValue(std::string const&)")]
// was: __ZN4Ogre5Light19createAnimableValueERKSs
pub fn stub_ccf690() -> ! {
    todo!("0xccf690 Ogre::Light::createAnimableValue(std::string const&)")
}


// 0xccf824 — __ZThn4_N4Ogre5Light19createAnimableValueERKSs
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const std::string *)
#[doc(alias = "non-virtual thunk toOgre::Light::createAnimableValue(std::string const&)")]
// was: __ZThn4_N4Ogre5Light19createAnimableValueERKSs
pub fn stub_ccf824() -> ! {
    todo!("0xccf824 `non-virtual thunk to'Ogre::Light::createAnimableValue(std::string const&)")
}


// 0xccf834 — __ZNK4Ogre5Light26getCustomShadowCameraSetupEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getCustomShadowCameraSetup(void)const")]
// was: __ZNK4Ogre5Light26getCustomShadowCameraSetupEv
pub fn stub_ccf834() -> ! {
    todo!("0xccf834 Ogre::Light::getCustomShadowCameraSetup(void)const")
}


// 0xccf840 — __ZNK4Ogre5Light20getShadowFarDistanceEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getShadowFarDistance(void)const")]
// was: __ZNK4Ogre5Light20getShadowFarDistanceEv
pub fn stub_ccf840() -> ! {
    todo!("0xccf840 Ogre::Light::getShadowFarDistance(void)const")
}


// 0xccf86c — __ZNK4Ogre5Light27getShadowFarDistanceSquaredEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getShadowFarDistanceSquared(void)const")]
// was: __ZNK4Ogre5Light27getShadowFarDistanceSquaredEv
pub fn stub_ccf86c() -> ! {
    todo!("0xccf86c Ogre::Light::getShadowFarDistanceSquared(void)const")
}


// 0xccf898 — __ZN4Ogre5Light18_setCameraRelativeEPNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, Ogre::Camera *)
#[doc(alias = "Ogre::Light::_setCameraRelative(Ogre::Camera *)")]
// was: __ZN4Ogre5Light18_setCameraRelativeEPNS_6CameraE
pub fn stub_ccf898() -> ! {
    todo!("0xccf898 Ogre::Light::_setCameraRelative(Ogre::Camera *)")
}


// 0xccf8a4 — __ZNK4Ogre5Light29_deriveShadowNearClipDistanceEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::Light::_deriveShadowNearClipDistance(Ogre::Camera const*)const")]
// was: __ZNK4Ogre5Light29_deriveShadowNearClipDistanceEPKNS_6CameraE
pub fn stub_ccf8a4() -> ! {
    todo!("0xccf8a4 Ogre::Light::_deriveShadowNearClipDistance(Ogre::Camera const*)const")
}


// 0xccf8cc — __ZNK4Ogre5Light28_deriveShadowFarClipDistanceEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::Light *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::Light::_deriveShadowFarClipDistance(Ogre::Camera const*)const")]
// was: __ZNK4Ogre5Light28_deriveShadowFarClipDistanceEPKNS_6CameraE
pub fn stub_ccf8cc() -> ! {
    todo!("0xccf8cc Ogre::Light::_deriveShadowFarClipDistance(Ogre::Camera const*)const")
}


// 0xccf8f0 — __ZN4Ogre5Light18setCustomParameterEtRKNS_7Vector4E
#[doc(alias = "Ogre::Light::setCustomParameter(unsigned short,Ogre::Vector4 const&)")]
// was: __ZN4Ogre5Light18setCustomParameterEtRKNS_7Vector4E
pub fn stub_ccf8f0() -> ! {
    todo!("0xccf8f0 Ogre::Light::setCustomParameter(unsigned short,Ogre::Vector4 const&)")
}


// 0xccf95c — __ZNK4Ogre5Light25_updateCustomGpuParameterEtRKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "Ogre::Light::_updateCustomGpuParameter(unsigned short,Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const")]
// was: __ZNK4Ogre5Light25_updateCustomGpuParameterEtRKNS_20GpuProgramParameters17AutoConstantEntryEPS1_
pub fn stub_ccf95c() -> ! {
    todo!("0xccf95c Ogre::Light::_updateCustomGpuParameter(unsigned short,Ogre::GpuProgramParameters::AutoConstantEntry const&,Ogre::GpuProgramParameters*)const")
}


// 0xccf9b4 — __ZNK4Ogre12LightFactory7getTypeEv
// type: _DWORD __fastcall(Ogre::LightFactory *__hidden this)
#[doc(alias = "Ogre::LightFactory::getType(void)const")]
// was: __ZNK4Ogre12LightFactory7getTypeEv
pub fn stub_ccf9b4() -> ! {
    todo!("0xccf9b4 Ogre::LightFactory::getType(void)const")
}


// 0xcd0648 — __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
pub fn stub_cd0648() -> ! {
    todo!("0xcd0648 Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")
}


// 0xcd064c — __ZN4Ogre14AxisAlignedBoxD1Ev
// type: void __fastcall(Ogre::AxisAlignedBox *__hidden this)
#[doc(alias = "Ogre::AxisAlignedBox::~AxisAlignedBox()")]
// was: __ZN4Ogre14AxisAlignedBoxD1Ev
pub fn stub_cd064c() -> ! {
    todo!("0xcd064c Ogre::AxisAlignedBox::~AxisAlignedBox()")
}


// 0xcd06e0 — __ZNK4Ogre5Light17getBoundingRadiusEv
// type: _DWORD __fastcall(Ogre::Light *__hidden this)
#[doc(alias = "Ogre::Light::getBoundingRadius(void)const")]
// was: __ZNK4Ogre5Light17getBoundingRadiusEv
pub fn stub_cd06e0() -> ! {
    todo!("0xcd06e0 Ogre::Light::getBoundingRadius(void)const")
}


// 0xcd06e4 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
pub fn stub_cd06e4() -> ! {
    todo!("0xcd06e4 std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::Vector4>>,std::pair<unsigned short const,Ogre::Vector4> const&)")
}


// 0xcd093c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
pub fn stub_cd093c() -> ! {
    todo!("0xcd093c std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Vector4> const&)")
}


// 0xcd0a4c — __ZN4Ogre9SharedPtrINS_13AnimableValueEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEED1Ev
pub fn stub_cd0a4c() -> ! {
    todo!("0xcd0a4c Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")
}


// 0xcd0afc — __ZN4Ogre9SharedPtrINS_13AnimableValueEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEED0Ev
pub fn stub_cd0afc() -> ! {
    todo!("0xcd0afc Ogre::SharedPtr<Ogre::AnimableValue>::~SharedPtr()")
}


// 0xcd0bf0 — __ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEE7destroyEv
pub fn stub_cd0bf0() -> ! {
    todo!("0xcd0bf0 Ogre::SharedPtr<Ogre::AnimableValue>::destroy(void)")
}


// 0xcd0c28 — __ZN4Ogre9SharedPtrINS_13AnimableValueEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::AnimableValue>::swap(Ogre::SharedPtr<Ogre::AnimableValue>&)")]
// was: __ZN4Ogre9SharedPtrINS_13AnimableValueEE4swapERS2_
pub fn stub_cd0c28() -> ! {
    todo!("0xcd0c28 Ogre::SharedPtr<Ogre::AnimableValue>::swap(Ogre::SharedPtr<Ogre::AnimableValue>&)")
}


// 0xcd0c44 — __ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)")]
// was: __ZNSt6vectorIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
pub fn stub_cd0c44() -> ! {
    todo!("0xcd0c44 std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PlaneBoundedVolume*,std::vector<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PlaneBoundedVolume const&)")
}


// 0xcd0f04 — __ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: __ZSt22__uninitialized_copy_aIPN4Ogre18PlaneBoundedVolumeES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
pub fn stub_cd0f04() -> ! {
    todo!("0xcd0f04 Ogre::PlaneBoundedVolume * std::__uninitialized_copy_a<Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::PlaneBoundedVolume *,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}


// 0xcd103c — __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: __ZN4Ogre12STLAllocatorINS_18PlaneBoundedVolumeENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
pub fn stub_cd103c() -> ! {
    todo!("0xcd103c Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")
}


// 0xcd1048 — __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
pub fn stub_cd1048() -> ! {
    todo!("0xcd1048 std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xcd104c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
pub fn stub_cd104c() -> ! {
    todo!("0xcd104c std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")
}


// 0xcd1050 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
pub fn stub_cd1050() -> ! {
    todo!("0xcd1050 std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")
}


// 0xcd105c — __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIN4Ogre18PlaneBoundedVolumeENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
pub fn stub_cd105c() -> ! {
    todo!("0xcd105c std::_Vector_base<Ogre::PlaneBoundedVolume,Ogre::STLAllocator<Ogre::PlaneBoundedVolume,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xcd1068 — __ZN4Ogre13AnimableValue14setAsBaseValueEi
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, int)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(int)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueEi
pub fn stub_cd1068() -> ! {
    todo!("0xcd1068 Ogre::AnimableValue::setAsBaseValue(int)")
}


// 0xcd106c — __ZN4Ogre13AnimableValue14setAsBaseValueEf
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, float)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(float)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueEf
pub fn stub_cd106c() -> ! {
    todo!("0xcd106c Ogre::AnimableValue::setAsBaseValue(float)")
}


// 0xcd1070 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector2 const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector2E
pub fn stub_cd1070() -> ! {
    todo!("0xcd1070 Ogre::AnimableValue::setAsBaseValue(Ogre::Vector2 const&)")
}


// 0xcd107c — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector3 *)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector3 const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector3E
pub fn stub_cd107c() -> ! {
    todo!("0xcd107c Ogre::AnimableValue::setAsBaseValue(Ogre::Vector3 const&)")
}


// 0xcd108c — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_7Vector4E
pub fn stub_cd108c() -> ! {
    todo!("0xcd108c Ogre::AnimableValue::setAsBaseValue(Ogre::Vector4 const&)")
}


// 0xcd1098 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_10QuaternionE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Quaternion const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_10QuaternionE
pub fn stub_cd1098() -> ! {
    todo!("0xcd1098 Ogre::AnimableValue::setAsBaseValue(Ogre::Quaternion const&)")
}


// 0xcd10a4 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_11ColourValueE
pub fn stub_cd10a4() -> ! {
    todo!("0xcd10a4 Ogre::AnimableValue::setAsBaseValue(Ogre::ColourValue const&)")
}


// 0xcd10b8 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Radian const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6RadianE
pub fn stub_cd10b8() -> ! {
    todo!("0xcd10b8 Ogre::AnimableValue::setAsBaseValue(Ogre::Radian const&)")
}


// 0xcd10c0 — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Degree const&)")]
// was: __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_6DegreeE
pub fn stub_cd10c0() -> ! {
    todo!("0xcd10c0 Ogre::AnimableValue::setAsBaseValue(Ogre::Degree const&)")
}


// 0xcd10e4 — __ZN4Ogre26LightSpotlightFalloffValueD1Ev
// type: void __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")]
// was: __ZN4Ogre26LightSpotlightFalloffValueD1Ev
pub fn stub_cd10e4() -> ! {
    todo!("0xcd10e4 Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")
}


// 0xcd10e8 — __ZN4Ogre26LightSpotlightFalloffValueD0Ev
// type: void __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")]
// was: __ZN4Ogre26LightSpotlightFalloffValueD0Ev
pub fn stub_cd10e8() -> ! {
    todo!("0xcd10e8 Ogre::LightSpotlightFalloffValue::~LightSpotlightFalloffValue()")
}


// 0xcd1174 — __ZN4Ogre26LightSpotlightFalloffValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre26LightSpotlightFalloffValue26setCurrentStateAsBaseValueEv
pub fn stub_cd1174() -> ! {
    todo!("0xcd1174 Ogre::LightSpotlightFalloffValue::setCurrentStateAsBaseValue(void)")
}


// 0xcd1188 — __ZN4Ogre13AnimableValue8setValueEi
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, int)
#[doc(alias = "Ogre::AnimableValue::setValue(int)")]
// was: __ZN4Ogre13AnimableValue8setValueEi
pub fn stub_cd1188() -> ! {
    todo!("0xcd1188 Ogre::AnimableValue::setValue(int)")
}


// 0xcd1338 — __ZN4Ogre26LightSpotlightFalloffValue8setValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::setValue(float)")]
// was: __ZN4Ogre26LightSpotlightFalloffValue8setValueEf
pub fn stub_cd1338() -> ! {
    todo!("0xcd1338 Ogre::LightSpotlightFalloffValue::setValue(float)")
}


// 0xcd1340 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector2 const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_7Vector2E
pub fn stub_cd1340() -> ! {
    todo!("0xcd1340 Ogre::AnimableValue::setValue(Ogre::Vector2 const&)")
}


// 0xcd14f0 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Vector3 *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector3 const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_7Vector3E
pub fn stub_cd14f0() -> ! {
    todo!("0xcd14f0 Ogre::AnimableValue::setValue(Ogre::Vector3 const&)")
}


// 0xcd16a0 — __ZN4Ogre13AnimableValue8setValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_7Vector4E
pub fn stub_cd16a0() -> ! {
    todo!("0xcd16a0 Ogre::AnimableValue::setValue(Ogre::Vector4 const&)")
}


// 0xcd1850 — __ZN4Ogre13AnimableValue8setValueERKNS_10QuaternionE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Quaternion *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Quaternion const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_10QuaternionE
pub fn stub_cd1850() -> ! {
    todo!("0xcd1850 Ogre::AnimableValue::setValue(Ogre::Quaternion const&)")
}


// 0xcd1a00 — __ZN4Ogre13AnimableValue8setValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_11ColourValueE
pub fn stub_cd1a00() -> ! {
    todo!("0xcd1a00 Ogre::AnimableValue::setValue(Ogre::ColourValue const&)")
}


// 0xcd1bb0 — __ZN4Ogre13AnimableValue8setValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Radian const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_6RadianE
pub fn stub_cd1bb0() -> ! {
    todo!("0xcd1bb0 Ogre::AnimableValue::setValue(Ogre::Radian const&)")
}


// 0xcd1d60 — __ZN4Ogre13AnimableValue8setValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Degree const&)")]
// was: __ZN4Ogre13AnimableValue8setValueERKNS_6DegreeE
pub fn stub_cd1d60() -> ! {
    todo!("0xcd1d60 Ogre::AnimableValue::setValue(Ogre::Degree const&)")
}


// 0xcd1f10 — __ZN4Ogre13AnimableValue15applyDeltaValueEi
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, int)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(int)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueEi
pub fn stub_cd1f10() -> ! {
    todo!("0xcd1f10 Ogre::AnimableValue::applyDeltaValue(int)")
}


// 0xcd20c0 — __ZN4Ogre26LightSpotlightFalloffValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightFalloffValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightFalloffValue::applyDeltaValue(float)")]
// was: __ZN4Ogre26LightSpotlightFalloffValue15applyDeltaValueEf
pub fn stub_cd20c0() -> ! {
    todo!("0xcd20c0 Ogre::LightSpotlightFalloffValue::applyDeltaValue(float)")
}


// 0xcd20e0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector2E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Vector2 *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector2 const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector2E
pub fn stub_cd20e0() -> ! {
    todo!("0xcd20e0 Ogre::AnimableValue::applyDeltaValue(Ogre::Vector2 const&)")
}


// 0xcd2290 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Vector3 *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector3 const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector3E
pub fn stub_cd2290() -> ! {
    todo!("0xcd2290 Ogre::AnimableValue::applyDeltaValue(Ogre::Vector3 const&)")
}


// 0xcd2440 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector4E
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_7Vector4E
pub fn stub_cd2440() -> ! {
    todo!("0xcd2440 Ogre::AnimableValue::applyDeltaValue(Ogre::Vector4 const&)")
}


// 0xcd25f0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_10QuaternionE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::Quaternion *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Quaternion const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_10QuaternionE
pub fn stub_cd25f0() -> ! {
    todo!("0xcd25f0 Ogre::AnimableValue::applyDeltaValue(Ogre::Quaternion const&)")
}


// 0xcd27a0 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_11ColourValueE
pub fn stub_cd27a0() -> ! {
    todo!("0xcd27a0 Ogre::AnimableValue::applyDeltaValue(Ogre::ColourValue const&)")
}


// 0xcd2950 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6DegreeE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Degree const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6DegreeE
pub fn stub_cd2950() -> ! {
    todo!("0xcd2950 Ogre::AnimableValue::applyDeltaValue(Ogre::Degree const&)")
}


// 0xcd2b00 — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6RadianE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Radian const&)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_6RadianE
pub fn stub_cd2b00() -> ! {
    todo!("0xcd2b00 Ogre::AnimableValue::applyDeltaValue(Ogre::Radian const&)")
}


// 0xcd2cb0 — __ZN4Ogre13AnimableValue8setValueEf
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, float)
#[doc(alias = "Ogre::AnimableValue::setValue(float)")]
// was: __ZN4Ogre13AnimableValue8setValueEf
pub fn stub_cd2cb0() -> ! {
    todo!("0xcd2cb0 Ogre::AnimableValue::setValue(float)")
}


// 0xcd2e60 — __ZN4Ogre13AnimableValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::AnimableValue *__hidden this, float)
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(float)")]
// was: __ZN4Ogre13AnimableValue15applyDeltaValueEf
pub fn stub_cd2e60() -> ! {
    todo!("0xcd2e60 Ogre::AnimableValue::applyDeltaValue(float)")
}


// 0xcd3010 — __ZN4Ogre24LightSpotlightOuterValueD1Ev
// type: void __fastcall(Ogre::LightSpotlightOuterValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")]
// was: __ZN4Ogre24LightSpotlightOuterValueD1Ev
pub fn stub_cd3010() -> ! {
    todo!("0xcd3010 Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")
}


// 0xcd3014 — __ZN4Ogre24LightSpotlightOuterValueD0Ev
// type: void __fastcall(Ogre::LightSpotlightOuterValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")]
// was: __ZN4Ogre24LightSpotlightOuterValueD0Ev
pub fn stub_cd3014() -> ! {
    todo!("0xcd3014 Ogre::LightSpotlightOuterValue::~LightSpotlightOuterValue()")
}


// 0xcd30a0 — __ZN4Ogre24LightSpotlightOuterValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpotlightOuterValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightOuterValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre24LightSpotlightOuterValue26setCurrentStateAsBaseValueEv
pub fn stub_cd30a0() -> ! {
    todo!("0xcd30a0 Ogre::LightSpotlightOuterValue::setCurrentStateAsBaseValue(void)")
}


// 0xcd30b4 — __ZN4Ogre24LightSpotlightOuterValue8setValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightOuterValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightOuterValue::setValue(float)")]
// was: __ZN4Ogre24LightSpotlightOuterValue8setValueEf
pub fn stub_cd30b4() -> ! {
    todo!("0xcd30b4 Ogre::LightSpotlightOuterValue::setValue(float)")
}


// 0xcd30bc — __ZN4Ogre24LightSpotlightOuterValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightOuterValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightOuterValue::applyDeltaValue(float)")]
// was: __ZN4Ogre24LightSpotlightOuterValue15applyDeltaValueEf
pub fn stub_cd30bc() -> ! {
    todo!("0xcd30bc Ogre::LightSpotlightOuterValue::applyDeltaValue(float)")
}


// 0xcd30dc — __ZN4Ogre24LightSpotlightInnerValueD1Ev
// type: void __fastcall(Ogre::LightSpotlightInnerValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")]
// was: __ZN4Ogre24LightSpotlightInnerValueD1Ev
pub fn stub_cd30dc() -> ! {
    todo!("0xcd30dc Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")
}


// 0xcd30e0 — __ZN4Ogre24LightSpotlightInnerValueD0Ev
// type: void __fastcall(Ogre::LightSpotlightInnerValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")]
// was: __ZN4Ogre24LightSpotlightInnerValueD0Ev
pub fn stub_cd30e0() -> ! {
    todo!("0xcd30e0 Ogre::LightSpotlightInnerValue::~LightSpotlightInnerValue()")
}


// 0xcd316c — __ZN4Ogre24LightSpotlightInnerValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpotlightInnerValue *__hidden this)
#[doc(alias = "Ogre::LightSpotlightInnerValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre24LightSpotlightInnerValue26setCurrentStateAsBaseValueEv
pub fn stub_cd316c() -> ! {
    todo!("0xcd316c Ogre::LightSpotlightInnerValue::setCurrentStateAsBaseValue(void)")
}


// 0xcd3180 — __ZN4Ogre24LightSpotlightInnerValue8setValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightInnerValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightInnerValue::setValue(float)")]
// was: __ZN4Ogre24LightSpotlightInnerValue8setValueEf
pub fn stub_cd3180() -> ! {
    todo!("0xcd3180 Ogre::LightSpotlightInnerValue::setValue(float)")
}


// 0xcd3188 — __ZN4Ogre24LightSpotlightInnerValue15applyDeltaValueEf
// type: _DWORD __fastcall(Ogre::LightSpotlightInnerValue *__hidden this, float)
#[doc(alias = "Ogre::LightSpotlightInnerValue::applyDeltaValue(float)")]
// was: __ZN4Ogre24LightSpotlightInnerValue15applyDeltaValueEf
pub fn stub_cd3188() -> ! {
    todo!("0xcd3188 Ogre::LightSpotlightInnerValue::applyDeltaValue(float)")
}


// 0xcd31a8 — __ZN4Ogre21LightAttenuationValueD1Ev
// type: void __fastcall(Ogre::LightAttenuationValue *__hidden this)
#[doc(alias = "Ogre::LightAttenuationValue::~LightAttenuationValue()")]
// was: __ZN4Ogre21LightAttenuationValueD1Ev
pub fn stub_cd31a8() -> ! {
    todo!("0xcd31a8 Ogre::LightAttenuationValue::~LightAttenuationValue()")
}


// 0xcd31ac — __ZN4Ogre21LightAttenuationValueD0Ev
// type: void __fastcall(Ogre::LightAttenuationValue *__hidden this)
#[doc(alias = "Ogre::LightAttenuationValue::~LightAttenuationValue()")]
// was: __ZN4Ogre21LightAttenuationValueD0Ev
pub fn stub_cd31ac() -> ! {
    todo!("0xcd31ac Ogre::LightAttenuationValue::~LightAttenuationValue()")
}


// 0xcd3238 — __ZN4Ogre21LightAttenuationValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightAttenuationValue *__hidden this)
#[doc(alias = "Ogre::LightAttenuationValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre21LightAttenuationValue26setCurrentStateAsBaseValueEv
pub fn stub_cd3238() -> ! {
    todo!("0xcd3238 Ogre::LightAttenuationValue::setCurrentStateAsBaseValue(void)")
}


// 0xcd32b0 — __ZN4Ogre21LightAttenuationValue8setValueERKNS_7Vector4E
#[doc(alias = "Ogre::LightAttenuationValue::setValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre21LightAttenuationValue8setValueERKNS_7Vector4E
pub fn stub_cd32b0() -> ! {
    todo!("0xcd32b0 Ogre::LightAttenuationValue::setValue(Ogre::Vector4 const&)")
}


// 0xcd32c0 — __ZN4Ogre21LightAttenuationValue15applyDeltaValueERKNS_7Vector4E
#[doc(alias = "Ogre::LightAttenuationValue::applyDeltaValue(Ogre::Vector4 const&)")]
// was: __ZN4Ogre21LightAttenuationValue15applyDeltaValueERKNS_7Vector4E
pub fn stub_cd32c0() -> ! {
    todo!("0xcd32c0 Ogre::LightAttenuationValue::applyDeltaValue(Ogre::Vector4 const&)")
}


// 0xcd335c — __ZN4Ogre24LightSpecularColourValueD1Ev
// type: void __fastcall(Ogre::LightSpecularColourValue *__hidden this)
#[doc(alias = "Ogre::LightSpecularColourValue::~LightSpecularColourValue()")]
// was: __ZN4Ogre24LightSpecularColourValueD1Ev
pub fn stub_cd335c() -> ! {
    todo!("0xcd335c Ogre::LightSpecularColourValue::~LightSpecularColourValue()")
}


// 0xcd3360 — __ZN4Ogre24LightSpecularColourValueD0Ev
// type: void __fastcall(Ogre::LightSpecularColourValue *__hidden this)
#[doc(alias = "Ogre::LightSpecularColourValue::~LightSpecularColourValue()")]
// was: __ZN4Ogre24LightSpecularColourValueD0Ev
pub fn stub_cd3360() -> ! {
    todo!("0xcd3360 Ogre::LightSpecularColourValue::~LightSpecularColourValue()")
}


// 0xcd33ec — __ZN4Ogre24LightSpecularColourValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightSpecularColourValue *__hidden this)
#[doc(alias = "Ogre::LightSpecularColourValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre24LightSpecularColourValue26setCurrentStateAsBaseValueEv
pub fn stub_cd33ec() -> ! {
    todo!("0xcd33ec Ogre::LightSpecularColourValue::setCurrentStateAsBaseValue(void)")
}


// 0xcd33fc — __ZN4Ogre24LightSpecularColourValue8setValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightSpecularColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightSpecularColourValue::setValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre24LightSpecularColourValue8setValueERKNS_11ColourValueE
pub fn stub_cd33fc() -> ! {
    todo!("0xcd33fc Ogre::LightSpecularColourValue::setValue(Ogre::ColourValue const&)")
}


// 0xcd340c — __ZN4Ogre24LightSpecularColourValue15applyDeltaValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightSpecularColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightSpecularColourValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre24LightSpecularColourValue15applyDeltaValueERKNS_11ColourValueE
pub fn stub_cd340c() -> ! {
    todo!("0xcd340c Ogre::LightSpecularColourValue::applyDeltaValue(Ogre::ColourValue const&)")
}


// 0xcd3464 — __ZN4Ogre23LightDiffuseColourValueD1Ev
// type: void __fastcall(Ogre::LightDiffuseColourValue *__hidden this)
#[doc(alias = "Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")]
// was: __ZN4Ogre23LightDiffuseColourValueD1Ev
pub fn stub_cd3464() -> ! {
    todo!("0xcd3464 Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")
}


// 0xcd3468 — __ZN4Ogre23LightDiffuseColourValueD0Ev
// type: void __fastcall(Ogre::LightDiffuseColourValue *__hidden this)
#[doc(alias = "Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")]
// was: __ZN4Ogre23LightDiffuseColourValueD0Ev
pub fn stub_cd3468() -> ! {
    todo!("0xcd3468 Ogre::LightDiffuseColourValue::~LightDiffuseColourValue()")
}


// 0xcd34f4 — __ZN4Ogre23LightDiffuseColourValue26setCurrentStateAsBaseValueEv
// type: _DWORD __fastcall(Ogre::LightDiffuseColourValue *__hidden this)
#[doc(alias = "Ogre::LightDiffuseColourValue::setCurrentStateAsBaseValue(void)")]
// was: __ZN4Ogre23LightDiffuseColourValue26setCurrentStateAsBaseValueEv
pub fn stub_cd34f4() -> ! {
    todo!("0xcd34f4 Ogre::LightDiffuseColourValue::setCurrentStateAsBaseValue(void)")
}


// 0xcd3504 — __ZN4Ogre23LightDiffuseColourValue8setValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightDiffuseColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightDiffuseColourValue::setValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre23LightDiffuseColourValue8setValueERKNS_11ColourValueE
pub fn stub_cd3504() -> ! {
    todo!("0xcd3504 Ogre::LightDiffuseColourValue::setValue(Ogre::ColourValue const&)")
}


// 0xcd3514 — __ZN4Ogre23LightDiffuseColourValue15applyDeltaValueERKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::LightDiffuseColourValue *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "Ogre::LightDiffuseColourValue::applyDeltaValue(Ogre::ColourValue const&)")]
// was: __ZN4Ogre23LightDiffuseColourValue15applyDeltaValueERKNS_11ColourValueE
pub fn stub_cd3514() -> ! {
    todo!("0xcd3514 Ogre::LightDiffuseColourValue::applyDeltaValue(Ogre::ColourValue const&)")
}


// 0xcd356c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtN4Ogre7Vector4EESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
pub fn stub_cd356c() -> ! {
    todo!("0xcd356c std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Vector4>,std::_Select1st<std::pair<unsigned short const,Ogre::Vector4>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Vector4>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Vector4>> *)")
}


// 0xcd3600 — __ZN4Ogre11LodStrategyC2ERKSs
// type: _DWORD __fastcall(Ogre::LodStrategy *__hidden this, const std::string *)
#[doc(alias = "Ogre::LodStrategy::LodStrategy(std::string const&)")]
// was: __ZN4Ogre11LodStrategyC2ERKSs
pub fn stub_cd3600() -> ! {
    todo!("0xcd3600 Ogre::LodStrategy::LodStrategy(std::string const&)")
}


// 0xcd3624 — __ZN4Ogre11LodStrategyD0Ev
// type: void __fastcall(Ogre::LodStrategy *__hidden this)
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: __ZN4Ogre11LodStrategyD0Ev
pub fn stub_cd3624() -> ! {
    todo!("0xcd3624 Ogre::LodStrategy::~LodStrategy()")
}


// 0xcd36fc — __ZN4Ogre11LodStrategyD1Ev
// type: void __fastcall(Ogre::LodStrategy *__hidden this)
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: __ZN4Ogre11LodStrategyD1Ev
pub fn stub_cd36fc() -> ! {
    todo!("0xcd36fc Ogre::LodStrategy::~LodStrategy()")
}


// 0xcd3758 — __ZN4Ogre11LodStrategyD2Ev
// type: void __fastcall(Ogre::LodStrategy *__hidden this)
#[doc(alias = "Ogre::LodStrategy::~LodStrategy()")]
// was: __ZN4Ogre11LodStrategyD2Ev
pub fn stub_cd3758() -> ! {
    todo!("0xcd3758 Ogre::LodStrategy::~LodStrategy()")
}


// 0xcd37b4 — __ZNK4Ogre11LodStrategy18transformUserValueEf
// type: _DWORD __fastcall(Ogre::LodStrategy *__hidden this, float)
#[doc(alias = "Ogre::LodStrategy::transformUserValue(float)const")]
// was: __ZNK4Ogre11LodStrategy18transformUserValueEf
pub fn stub_cd37b4() -> ! {
    todo!("0xcd37b4 Ogre::LodStrategy::transformUserValue(float)const")
}


// 0xcd37b8 — __ZNK4Ogre11LodStrategy8getValueEPKNS_13MovableObjectEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::LodStrategy *__hidden this, const Ogre::MovableObject *, const Ogre::Camera *)
#[doc(alias = "Ogre::LodStrategy::getValue(Ogre::MovableObject const*,Ogre::Camera const*)const")]
// was: __ZNK4Ogre11LodStrategy8getValueEPKNS_13MovableObjectEPKNS_6CameraE
pub fn stub_cd37b8() -> ! {
    todo!("0xcd37b8 Ogre::LodStrategy::getValue(Ogre::MovableObject const*,Ogre::Camera const*)const")
}


// 0xcd37d8 — __ZN4Ogre11LodStrategy17isSortedAscendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::isSortedAscending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: __ZN4Ogre11LodStrategy17isSortedAscendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
pub fn stub_cd37d8() -> ! {
    todo!("0xcd37d8 Ogre::LodStrategy::isSortedAscending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}


// 0xcd3804 — __ZN4Ogre11LodStrategy18isSortedDescendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::LodStrategy::isSortedDescending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: __ZN4Ogre11LodStrategy18isSortedDescendingERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
pub fn stub_cd3804() -> ! {
    todo!("0xcd3804 Ogre::LodStrategy::isSortedDescending(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

