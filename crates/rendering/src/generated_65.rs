//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xc48904..0xc51c74 (100 stubs, 7668 prior -> +100, 5565 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xc48904 — __ZN3G3D8FilePath19removeTrailingSlashERKSs
#[doc(alias = "G3D::FilePath::removeTrailingSlash(std::string const&)")]
// was: G3D::FilePath::removeTrailingSlash(std::string const&)
// IDA 0xc48904: 204 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c48904() {
}

// 0xc48b50 — __ZN3G3D10FileSystem11getContentsERKSsb
#[doc(alias = "G3D::FileSystem::getContents(std::string const&,bool)")]
// was: G3D::FileSystem::getContents(std::string const&,bool)
// IDA 0xc48b50: 546 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c48b50() {
}

// 0xc49440 — __ZN3G3D8FilePath12canonicalizeESs
#[doc(alias = "G3D::FilePath::canonicalize(std::string)")]
// was: G3D::FilePath::canonicalize(std::string)
// IDA 0xc49440: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c49440() {
}

// 0xc494b4 — __ZN3G3D10FileSystem10_inZipfileERKSsRSs
#[doc(alias = "G3D::FileSystem::_inZipfile(std::string const&,std::string &)")]
// was: G3D::FileSystem::_inZipfile(std::string const&,std::string &)
// IDA 0xc494b4: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c494b4() {
}

// 0xc49698 — __ZN3G3D10FileSystem10_isZipfileERKSs
#[doc(alias = "G3D::FileSystem::_isZipfile(std::string const&)")]
// was: G3D::FileSystem::_isZipfile(std::string const&)
// IDA 0xc49698: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c49698() {
}

// 0xc498bc — __ZN3G3D8FilePath3extERKSs
#[doc(alias = "G3D::FilePath::ext(std::string const&)")]
// was: G3D::FilePath::ext(std::string const&)
// IDA 0xc498bc: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c498bc() {
}

// 0xc49920 — __ZN3G3D10FileSystem6_fopenEPKcS2_
#[doc(alias = "G3D::FileSystem::_fopen(char const*,char const*)")]
// was: G3D::FileSystem::_fopen(char const*,char const*)
// IDA 0xc49920: 174 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c49920() {
}

// 0xc49c20 — __ZN3G3D10FileSystem11_clearCacheERKSs
#[doc(alias = "G3D::FileSystem::_clearCache(std::string const&)")]
// was: G3D::FileSystem::_clearCache(std::string const&)
// IDA 0xc49c20: 459 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c49c20() {
}

// 0xc4a118 — __ZN3G3D8FilePath6parentERKSs
#[doc(alias = "G3D::FilePath::parent(std::string const&)")]
// was: G3D::FilePath::parent(std::string const&)
// IDA 0xc4a118: 103 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4a118() {
}

// 0xc4a28c — __ZN3G3D10FileSystem7_existsERKSsb
#[doc(alias = "G3D::FileSystem::_exists(std::string const&,bool)")]
// was: G3D::FileSystem::_exists(std::string const&,bool)
// IDA 0xc4a28c: 313 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4a28c() {
}

// 0xc4a5fc — __ZN3G3D8FilePath7baseExtERKSs
#[doc(alias = "G3D::FilePath::baseExt(std::string const&)")]
// was: G3D::FilePath::baseExt(std::string const&)
// IDA 0xc4a5fc: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4a5fc() {
}

// 0xc4a66c — __ZN3G3D8FilePath6concatERKSsS2_
#[doc(alias = "G3D::FilePath::concat(std::string const&,std::string const&)")]
// was: G3D::FilePath::concat(std::string const&,std::string const&)
// IDA 0xc4a66c: 192 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4a66c() {
}

// 0xc4a894 — __ZN3G3D10FileSystem5_sizeERKSs
#[doc(alias = "G3D::FileSystem::_size(std::string const&)")]
// was: G3D::FileSystem::_size(std::string const&)
// IDA 0xc4a894: 137 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4a894() {
}

// 0xc4aa80 — __ZN3G3D17FileSystemDeleterD1Ev
#[doc(alias = "G3D::FileSystemDeleter::~FileSystemDeleter()")]
// was: G3D::FileSystemDeleter::~FileSystemDeleter()
// IDA 0xc4aa80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4aa80() {
}

// 0xc4ab60 — __ZN3G3D10FileSystem7resolveERKSsS2_
#[doc(alias = "G3D::FileSystem::resolve(std::string const&,std::string const&)")]
// was: G3D::FileSystem::resolve(std::string const&,std::string const&)
// IDA 0xc4ab60: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4ab60() {
}

// 0xc4ac8c — __ZN3G3D10FileSystem16currentDirectoryEv
#[doc(alias = "G3D::FileSystem::currentDirectory(void)")]
// was: G3D::FileSystem::currentDirectory(void)
// IDA 0xc4ac8c: 87 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4ac8c() {
}

// 0xc4add8 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6removeERKSs
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::remove(std::string const&)")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::remove(std::string const&)
// IDA 0xc4add8: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4add8() {
}

// 0xc4af48 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6removeERKSsRSsRS2_b
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::remove(std::string const&,std::string &,G3D::FileSystem::Dir&,bool)")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::remove(std::string const&,std::string &,G3D::FileSystem::Dir&,bool)
// IDA 0xc4af48: 216 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4af48() {
}

// 0xc4b1a0 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE10freeMemoryEv
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::freeMemory(void)")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::freeMemory(void)
// IDA 0xc4b1a0: 140 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4b1a0() {
}

// 0xc4b318 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE14getCreateEntryERKSsRb
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)
// IDA 0xc4b318: 298 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4b318() {
}

// 0xc4b630 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEE6resizeEm
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)
// IDA 0xc4b630: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4b630() {
}

// 0xc4b6c0 — __ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::resize(int,bool)")]
// was: G3D::Array<G3D::FileSystem::Entry,10,32ul>::resize(int,bool)
// IDA 0xc4b6c0: 116 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4b6c0() {
}

// 0xc4b80c — __ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::realloc(int)")]
// was: G3D::Array<G3D::FileSystem::Entry,10,32ul>::realloc(int)
// IDA 0xc4b80c: 179 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4b80c() {
}

// 0xc4ba54 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEEC2Ev
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)
// IDA 0xc4ba54: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4ba54() {
}

// 0xc4bc1c — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEED1Ev
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()
// IDA 0xc4bc1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4bc1c() {
}

// 0xc4bc28 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEED0Ev
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()
// IDA 0xc4bc28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4bc28() {
}

// 0xc4bcc8 — __ZN3G3D5TableISsNS_10FileSystem3DirE9HashTraitISsE11EqualsTraitISsEED2Ev
#[doc(alias = "G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()")]
// was: G3D::Table<std::string,G3D::FileSystem::Dir,HashTrait<std::string>,EqualsTrait<std::string>>::~Table()
// IDA 0xc4bcc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4bcc8() {
}

// 0xc4bde4 — __ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::~Array()")]
// was: G3D::Array<G3D::FileSystem::Entry,10,32ul>::~Array()
// IDA 0xc4bde4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4bde4() {
}

// 0xc4bf60 — __ZN3G3D5ArrayINS_10FileSystem5EntryELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<G3D::FileSystem::Entry,10,32ul>::Array(void)")]
// was: G3D::Array<G3D::FileSystem::Entry,10,32ul>::Array(void)
// IDA 0xc4bf60: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4bf60() {
}

// 0xc4c168 — __ZN3G3D18LightingParametersC1Ev
#[doc(alias = "G3D::LightingParameters::LightingParameters(void)")]
// was: G3D::LightingParameters::LightingParameters(void)
// IDA 0xc4c168: 79 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4c168() {
}

// 0xc4c270 — __ZN3G3D18LightingParameters7setTimeEd
#[doc(alias = "G3D::LightingParameters::setTime(double)")]
// was: G3D::LightingParameters::setTime(double)
// IDA 0xc4c270: 682 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4c270() {
}

// 0xc4cc64 — __ZN3G3D12linearSplineIdNS_6Color3EEET0_dPKT_PKS2_i
#[doc(alias = "G3D::Color3 G3D::linearSpline<double,G3D::Color3>(double,double const*,G3D::Color3 const*,int)")]
// was: G3D::Color3 G3D::linearSpline<double,G3D::Color3>(double,double const*,G3D::Color3 const*,int)
// IDA 0xc4cc64: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4cc64() {
}

// 0xc4cdf0 — __ZN4Ogre13AnimableValue16resetToBaseValueEv
#[doc(alias = "Ogre::AnimableValue::resetToBaseValue(void)")]
// was: Ogre::AnimableValue::resetToBaseValue(void)
// IDA 0xc4cdf0: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4cdf0() {
}

// 0xc4cedc — __ZN4Ogre13AnimableValue14setAsBaseValueERKNS_3AnyE
#[doc(alias = "Ogre::AnimableValue::setAsBaseValue(Ogre::Any const&)")]
// was: Ogre::AnimableValue::setAsBaseValue(Ogre::Any const&)
// IDA 0xc4cedc: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4cedc() {
}

// 0xc4cf84 — __ZN4Ogre13AnimableValue8setValueERKNS_3AnyE
#[doc(alias = "Ogre::AnimableValue::setValue(Ogre::Any const&)")]
// was: Ogre::AnimableValue::setValue(Ogre::Any const&)
// IDA 0xc4cf84: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4cf84() {
}

// 0xc4d02c — __ZN4Ogre13AnimableValue15applyDeltaValueERKNS_3AnyE
#[doc(alias = "Ogre::AnimableValue::applyDeltaValue(Ogre::Any const&)")]
// was: Ogre::AnimableValue::applyDeltaValue(Ogre::Any const&)
// IDA 0xc4d02c: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4d02c() {
}

// 0xc4d0d4 — __ZNSt3mapISsSt6vectorISsN4Ogre12STLAllocatorISsNS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessISsENS2_ISt4pairIKSsS7_ES5_EEED1Ev
#[doc(alias = "std::map<std::string,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~map()")]
// was: std::map<std::string,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~map()
// IDA 0xc4d0d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4d0d4() {
}

// 0xc4d168 — __ZN4Ogre8any_castIiEET_RKNS_3AnyE
#[doc(alias = "int Ogre::any_cast<int>(Ogre::Any const&)")]
// was: int Ogre::any_cast<int>(Ogre::Any const&)
// IDA 0xc4d168: 329 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4d168() {
}

// 0xc4d524 — __ZN4Ogre8any_castIfEET_RKNS_3AnyE
#[doc(alias = "float Ogre::any_cast<float>(Ogre::Any const&)")]
// was: float Ogre::any_cast<float>(Ogre::Any const&)
// IDA 0xc4d524: 329 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4d524() {
}

// 0xc4d8e0 — __ZN4Ogre8any_castINS_7Vector2EEET_RKNS_3AnyE
#[doc(alias = "Ogre::Vector2 Ogre::any_cast<Ogre::Vector2>(Ogre::Any const&)")]
// was: Ogre::Vector2 Ogre::any_cast<Ogre::Vector2>(Ogre::Any const&)
// IDA 0xc4d8e0: 318 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4d8e0() {
}

// 0xc4dc7c — __ZN4Ogre8any_castINS_7Vector3EEET_RKNS_3AnyE
#[doc(alias = "Ogre::Vector3 Ogre::any_cast<Ogre::Vector3>(Ogre::Any const&)")]
// was: Ogre::Vector3 Ogre::any_cast<Ogre::Vector3>(Ogre::Any const&)
// IDA 0xc4dc7c: 318 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4dc7c() {
}

// 0xc4e01c — __ZN4Ogre8any_castINS_7Vector4EEET_RKNS_3AnyE
#[doc(alias = "Ogre::Vector4 Ogre::any_cast<Ogre::Vector4>(Ogre::Any const&)")]
// was: Ogre::Vector4 Ogre::any_cast<Ogre::Vector4>(Ogre::Any const&)
// IDA 0xc4e01c: 317 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4e01c() {
}

// 0xc4e3bc — __ZN4Ogre8any_castINS_10QuaternionEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Quaternion Ogre::any_cast<Ogre::Quaternion>(Ogre::Any const&)")]
// was: Ogre::Quaternion Ogre::any_cast<Ogre::Quaternion>(Ogre::Any const&)
// IDA 0xc4e3bc: 317 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4e3bc() {
}

// 0xc4e75c — __ZN4Ogre8any_castINS_11ColourValueEEET_RKNS_3AnyE
#[doc(alias = "Ogre::ColourValue Ogre::any_cast<Ogre::ColourValue>(Ogre::Any const&)")]
// was: Ogre::ColourValue Ogre::any_cast<Ogre::ColourValue>(Ogre::Any const&)
// IDA 0xc4e75c: 317 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4e75c() {
}

// 0xc4eafc — __ZN4Ogre8any_castINS_6DegreeEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Degree Ogre::any_cast<Ogre::Degree>(Ogre::Any const&)")]
// was: Ogre::Degree Ogre::any_cast<Ogre::Degree>(Ogre::Any const&)
// IDA 0xc4eafc: 317 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4eafc() {
}

// 0xc4ee9c — __ZN4Ogre8any_castINS_6RadianEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Radian Ogre::any_cast<Ogre::Radian>(Ogre::Any const&)")]
// was: Ogre::Radian Ogre::any_cast<Ogre::Radian>(Ogre::Any const&)
// IDA 0xc4ee9c: 317 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4ee9c() {
}

// 0xc4f23c — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorISsN4Ogre12STLAllocatorISsNS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISA_ESt4lessISsENS4_ISA_S7_EEE13_Rb_tree_implISE_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc4f23c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c4f23c() {
}

// 0xc4f240 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorISsN4Ogre12STLAllocatorISsNS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISA_ESt4lessISsENS4_ISA_S7_EEE13_Rb_tree_implISE_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc4f240: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4f240() {
}

// 0xc4f24c — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorISsN4Ogre12STLAllocatorISsNS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISA_ESt4lessISsENS4_ISA_S7_EEE8_M_eraseEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xc4f24c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4f24c() {
}

// 0xc4f27c — __ZN4Ogre12STLAllocatorISt4pairIKSsSt6vectorISsNS0_ISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEES6_E7destroyEPS9_
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>*)")]
// was: Ogre::STLAllocator<std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>*)
// IDA 0xc4f27c: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4f27c() {
}

// 0xc4f420 — __ZN4Ogre9AnimationC1ERKSsf
#[doc(alias = "Ogre::Animation::Animation(std::string const&,float)")]
// was: Ogre::Animation::Animation(std::string const&,float)
// IDA 0xc4f420: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4f420() {
}

// 0xc4f42c — __ZN4Ogre9AnimationC2ERKSsf
#[doc(alias = "Ogre::Animation::Animation(std::string const&,float)")]
// was: Ogre::Animation::Animation(std::string const&,float)
// IDA 0xc4f42c: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4f42c() {
}

// 0xc4f674 — __ZN4Ogre9AnimationD0Ev
#[doc(alias = "Ogre::Animation::~Animation()")]
// was: Ogre::Animation::~Animation()
// IDA 0xc4f674: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4f674() {
}

// 0xc4f704 — __ZN4Ogre9AnimationD1Ev
#[doc(alias = "Ogre::Animation::~Animation()")]
// was: Ogre::Animation::~Animation()
// IDA 0xc4f704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4f704() {
}

// 0xc4f710 — __ZN4Ogre9AnimationD2Ev
#[doc(alias = "Ogre::Animation::~Animation()")]
// was: Ogre::Animation::~Animation()
// IDA 0xc4f710: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c4f710() {
}

// 0xc4f950 — __ZN4Ogre9Animation16destroyAllTracksEv
#[doc(alias = "Ogre::Animation::destroyAllTracks(void)")]
// was: Ogre::Animation::destroyAllTracks(void)
// IDA 0xc4f950: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4f950() {
}

// 0xc4fa1c — __ZNK4Ogre9Animation9getLengthEv
#[doc(alias = "Ogre::Animation::getLength(void)const")]
// was: Ogre::Animation::getLength(void)const
// IDA 0xc4fa1c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4fa1c() {
}

// 0xc4fa20 — __ZN4Ogre9Animation15createNodeTrackEt
#[doc(alias = "Ogre::Animation::createNodeTrack(unsigned short)")]
// was: Ogre::Animation::createNodeTrack(unsigned short)
// IDA 0xc4fa20: 372 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4fa20() {
}

// 0xc4fe3c — __ZNK4Ogre9Animation12hasNodeTrackEt
#[doc(alias = "Ogre::Animation::hasNodeTrack(unsigned short)const")]
// was: Ogre::Animation::hasNodeTrack(unsigned short)const
// IDA 0xc4fe3c: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4fe3c() {
}

// 0xc4fe7c — __ZN4Ogre9Animation15createNodeTrackEtPNS_4NodeE
#[doc(alias = "Ogre::Animation::createNodeTrack(unsigned short,Ogre::Node *)")]
// was: Ogre::Animation::createNodeTrack(unsigned short,Ogre::Node *)
// IDA 0xc4fe7c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4fe7c() {
}

// 0xc4fe98 — __ZNK4Ogre9Animation12getNodeTrackEt
#[doc(alias = "Ogre::Animation::getNodeTrack(unsigned short)const")]
// was: Ogre::Animation::getNodeTrack(unsigned short)const
// IDA 0xc4fe98: 248 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c4fe98() {
}

// 0xc5016c — __ZN4Ogre9Animation17createVertexTrackEtNS_19VertexAnimationTypeE
#[doc(alias = "Ogre::Animation::createVertexTrack(unsigned short,Ogre::VertexAnimationType)")]
// was: Ogre::Animation::createVertexTrack(unsigned short,Ogre::VertexAnimationType)
// IDA 0xc5016c: 374 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5016c() {
}

// 0xc5058c — __ZN4Ogre9Animation17createVertexTrackEtPNS_10VertexDataENS_19VertexAnimationTypeE
#[doc(alias = "Ogre::Animation::createVertexTrack(unsigned short,Ogre::VertexData *,Ogre::VertexAnimationType)")]
// was: Ogre::Animation::createVertexTrack(unsigned short,Ogre::VertexData *,Ogre::VertexAnimationType)
// IDA 0xc5058c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5058c() {
}

// 0xc5059c — __ZNK4Ogre9Animation14getVertexTrackEt
#[doc(alias = "Ogre::Animation::getVertexTrack(unsigned short)const")]
// was: Ogre::Animation::getVertexTrack(unsigned short)const
// IDA 0xc5059c: 248 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5059c() {
}

// 0xc50870 — __ZNK4Ogre9Animation7getNameEv
#[doc(alias = "Ogre::Animation::getName(void)const")]
// was: Ogre::Animation::getName(void)const
// IDA 0xc50870: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50870() {
}

// 0xc50874 — __ZN4Ogre9Animation5applyEfff
#[doc(alias = "Ogre::Animation::apply(float,float,float)")]
// was: Ogre::Animation::apply(float,float,float)
// IDA 0xc50874: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50874() {
}

// 0xc50954 — __ZN4Ogre9Animation18_applyBaseKeyFrameEv
#[doc(alias = "Ogre::Animation::_applyBaseKeyFrame(void)")]
// was: Ogre::Animation::_applyBaseKeyFrame(void)
// IDA 0xc50954: 240 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50954() {
}

// 0xc50bc8 — __ZNK4Ogre9Animation13_getTimeIndexEf
#[doc(alias = "Ogre::Animation::_getTimeIndex(float)const")]
// was: Ogre::Animation::_getTimeIndex(float)const
// IDA 0xc50bc8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50bc8() {
}

// 0xc50c60 — __ZN4Ogre9Animation5applyEPNS_8SkeletonEfff
#[doc(alias = "Ogre::Animation::apply(Ogre::Skeleton *,float,float,float)")]
// was: Ogre::Animation::apply(Ogre::Skeleton *,float,float,float)
// IDA 0xc50c60: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50c60() {
}

// 0xc50cd8 — __ZN4Ogre9Animation5applyEPNS_8SkeletonEffPKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEf
#[doc(alias = "Ogre::Animation::apply(Ogre::Skeleton *,float,float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,float)")]
// was: Ogre::Animation::apply(Ogre::Skeleton *,float,float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,float)
// IDA 0xc50cd8: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50cd8() {
}

// 0xc50d70 — __ZN4Ogre9Animation5applyEPNS_6EntityEffbb
#[doc(alias = "Ogre::Animation::apply(Ogre::Entity *,float,float,bool,bool)")]
// was: Ogre::Animation::apply(Ogre::Entity *,float,float,bool,bool)
// IDA 0xc50d70: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50d70() {
}

// 0xc50e5c — __ZN4Ogre9Animation20setInterpolationModeENS0_17InterpolationModeE
#[doc(alias = "Ogre::Animation::setInterpolationMode(Ogre::Animation::InterpolationMode)")]
// was: Ogre::Animation::setInterpolationMode(Ogre::Animation::InterpolationMode)
// IDA 0xc50e5c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e5c() {
}

// 0xc50e60 — __ZNK4Ogre9Animation20getInterpolationModeEv
#[doc(alias = "Ogre::Animation::getInterpolationMode(void)const")]
// was: Ogre::Animation::getInterpolationMode(void)const
// IDA 0xc50e60: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e60() {
}

// 0xc50e74 — __ZN4Ogre9Animation28setRotationInterpolationModeENS0_25RotationInterpolationModeE
#[doc(alias = "Ogre::Animation::setRotationInterpolationMode(Ogre::Animation::RotationInterpolationMode)")]
// was: Ogre::Animation::setRotationInterpolationMode(Ogre::Animation::RotationInterpolationMode)
// IDA 0xc50e74: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e74() {
}

// 0xc50e78 — __ZNK4Ogre9Animation28getRotationInterpolationModeEv
#[doc(alias = "Ogre::Animation::getRotationInterpolationMode(void)const")]
// was: Ogre::Animation::getRotationInterpolationMode(void)const
// IDA 0xc50e78: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e78() {
}

// 0xc50e7c — __ZN4Ogre9Animation35setDefaultRotationInterpolationModeENS0_25RotationInterpolationModeE
#[doc(alias = "Ogre::Animation::setDefaultRotationInterpolationMode(Ogre::Animation::RotationInterpolationMode)")]
// was: Ogre::Animation::setDefaultRotationInterpolationMode(Ogre::Animation::RotationInterpolationMode)
// IDA 0xc50e7c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e7c() {
}

// 0xc50e8c — __ZN4Ogre9Animation8optimiseEb
#[doc(alias = "Ogre::Animation::optimise(bool)")]
// was: Ogre::Animation::optimise(bool)
// IDA 0xc50e8c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e8c() {
}

// 0xc50ea0 — __ZN4Ogre9Animation18optimiseNodeTracksEb
#[doc(alias = "Ogre::Animation::optimiseNodeTracks(bool)")]
// was: Ogre::Animation::optimiseNodeTracks(bool)
// IDA 0xc50ea0: 200 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50ea0() {
}

// 0xc51088 — __ZN4Ogre9Animation20optimiseVertexTracksEv
#[doc(alias = "Ogre::Animation::optimiseVertexTracks(void)")]
// was: Ogre::Animation::optimiseVertexTracks(void)
// IDA 0xc51088: 198 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c51088() {
}

// 0xc5126c — __ZNK4Ogre9Animation26_collectIdentityNodeTracksERSt3setItSt4lessItENS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::Animation::_collectIdentityNodeTracks(std::set<unsigned short,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")]
// was: Ogre::Animation::_collectIdentityNodeTracks(std::set<unsigned short,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const
// IDA 0xc5126c: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5126c() {
}

// 0xc512f0 — __ZN4Ogre9Animation18_destroyNodeTracksERKSt3setItSt4lessItENS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::Animation::_destroyNodeTracks(std::set<unsigned short,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::Animation::_destroyNodeTracks(std::set<unsigned short,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xc512f0: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c512f0() {
}

// 0xc51374 — __ZNK4Ogre9Animation21buildKeyFrameTimeListEv
#[doc(alias = "Ogre::Animation::buildKeyFrameTimeList(void)const")]
// was: Ogre::Animation::buildKeyFrameTimeList(void)const
// IDA 0xc51374: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c51374() {
}

// 0xc5144c — __ZN4Ogre9Animation18setUseBaseKeyFrameEbfRKSs
#[doc(alias = "Ogre::Animation::setUseBaseKeyFrame(bool,float,std::string const&)")]
// was: Ogre::Animation::setUseBaseKeyFrame(bool,float,std::string const&)
// IDA 0xc5144c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5144c() {
}

// 0xc514c0 — __ZNK4Ogre9Animation18getUseBaseKeyFrameEv
#[doc(alias = "Ogre::Animation::getUseBaseKeyFrame(void)const")]
// was: Ogre::Animation::getUseBaseKeyFrame(void)const
// IDA 0xc514c0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c514c0() {
}

// 0xc514c8 — __ZNK4Ogre9Animation19getBaseKeyFrameTimeEv
#[doc(alias = "Ogre::Animation::getBaseKeyFrameTime(void)const")]
// was: Ogre::Animation::getBaseKeyFrameTime(void)const
// IDA 0xc514c8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c514c8() {
}

// 0xc514cc — __ZNK4Ogre9Animation28getBaseKeyFrameAnimationNameEv
#[doc(alias = "Ogre::Animation::getBaseKeyFrameAnimationName(void)const")]
// was: Ogre::Animation::getBaseKeyFrameAnimationName(void)const
// IDA 0xc514cc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c514cc() {
}

// 0xc514d0 — __ZN4Ogre9Animation16_notifyContainerEPNS_18AnimationContainerE
#[doc(alias = "Ogre::Animation::_notifyContainer(Ogre::AnimationContainer *)")]
// was: Ogre::Animation::_notifyContainer(Ogre::AnimationContainer *)
// IDA 0xc514d0: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c514d0() {
}

// 0xc514d8 — __ZN4Ogre18VertexPoseKeyFrameD1Ev
#[doc(alias = "Ogre::VertexPoseKeyFrame::~VertexPoseKeyFrame()")]
// was: Ogre::VertexPoseKeyFrame::~VertexPoseKeyFrame()
// IDA 0xc514d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c514d8() {
}

// 0xc5157c — __ZNSt10_List_baseItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xc5157c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c5157c() {
}

// 0xc51580 — __ZNSt10_List_baseItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xc51580: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c51580() {
}

// 0xc5158c — __ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorItESC_
#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<unsigned short>,std::_Rb_tree_iterator<unsigned short>)")]
// was: std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<unsigned short>,std::_Rb_tree_iterator<unsigned short>)
// IDA 0xc5158c: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5158c() {
}

// 0xc515f0 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>> *)
// IDA 0xc515f0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c515f0() {
}

// 0xc51618 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::pair<unsigned short const,Ogre::VertexAnimationTrack *> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::pair<unsigned short const,Ogre::VertexAnimationTrack *> const&)
// IDA 0xc51618: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c51618() {
}

// 0xc51820 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::VertexAnimationTrack *> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::VertexAnimationTrack *> const&)
// IDA 0xc51820: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c51820() {
}

// 0xc5191c — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre21NumericAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NumericAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NumericAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::NumericAnimationTrack *>> *)
// IDA 0xc5191c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5191c() {
}

// 0xc51944 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>> *)
// IDA 0xc51944: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c51944() {
}

// 0xc5196c — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::pair<unsigned short const,Ogre::NodeAnimationTrack *> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::pair<unsigned short const,Ogre::NodeAnimationTrack *> const&)
// IDA 0xc5196c: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5196c() {
}

// 0xc51b74 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre18NodeAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::NodeAnimationTrack *> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::NodeAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::NodeAnimationTrack *> const&)
// IDA 0xc51b74: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c51b74() {
}

// 0xc51c70 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xc51c70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c51c70() {
}

// 0xc51c74 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre20VertexAnimationTrackEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,std::_Select1st<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::VertexAnimationTrack *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xc51c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c51c74() {
}
