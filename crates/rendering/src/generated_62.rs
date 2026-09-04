//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xc231d8..0xc3ae54 (100 stubs, 7370 prior -> +100, 5863 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xc231d8 — __ZN3RBX9LightGrid13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
#[doc(alias = "RBX::LightGrid::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: RBX::LightGrid::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xc231d8: 36 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c231d8() {
}

// 0xc23458 — __ZN4Ogre10TexturePtrD1Ev
#[doc(alias = "Ogre::TexturePtr::~TexturePtr()")]
// was: Ogre::TexturePtr::~TexturePtr()
// IDA 0xc23458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c23458() {
}

// 0xc24a3c — __ZN3RBX9LightGrid15occupancyFillDFINS_22SphereDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::SphereDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::SphereDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::SphereDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::SphereDistanceFunction &)
// IDA 0xc24a3c: 372 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c24a3c() {
}

// 0xc24f80 — __ZN3RBX9LightGrid15occupancyFillDFINS_25EllipsoidDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::EllipsoidDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::EllipsoidDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::EllipsoidDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::EllipsoidDistanceFunction &)
// IDA 0xc24f80: 391 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c24f80() {
}

// 0xc25514 — __ZN3RBX9LightGrid15occupancyFillDFINS_25CylinderXDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::CylinderXDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderXDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::CylinderXDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderXDistanceFunction &)
// IDA 0xc25514: 375 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c25514() {
}

// 0xc25a64 — __ZN3RBX9LightGrid15occupancyFillDFINS_25CylinderYDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::CylinderYDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderYDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::CylinderYDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CylinderYDistanceFunction &)
// IDA 0xc25a64: 375 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c25a64() {
}

// 0xc25fb4 — __ZN3RBX9LightGrid15occupancyFillDFINS_21WedgeDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::WedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::WedgeDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::WedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::WedgeDistanceFunction &)
// IDA 0xc25fb4: 412 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c25fb4() {
}

// 0xc26594 — __ZN3RBX9LightGrid15occupancyFillDFINS_27CornerWedgeDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::CornerWedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CornerWedgeDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::CornerWedgeDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::CornerWedgeDistanceFunction &)
// IDA 0xc26594: 436 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c26594() {
}

// 0xc26bb0 — __ZN3RBX9LightGrid15occupancyFillDFINS_21TorsoDistanceFunctionEEEvRNS_14LightGridChunkERKNS_7ExtentsERKN3G3D7Vector3ERKNS8_15CoordinateFrameEfRT_
#[doc(alias = "void RBX::LightGrid::occupancyFillDF<RBX::TorsoDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::TorsoDistanceFunction &)")]
// was: void RBX::LightGrid::occupancyFillDF<RBX::TorsoDistanceFunction>(RBX::LightGridChunk &,RBX::Extents const&,G3D::Vector3 const&,G3D::CoordinateFrame const&,float,RBX::TorsoDistanceFunction &)
// IDA 0xc26bb0: 386 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c26bb0() {
}

// 0xc296f0 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE24getPrimitivesOverlappingINS_12DenseHashSetIPS2_N5boost4hashIS8_EESaIS8_EEEEEvRKNS_7ExtentsERT_
#[doc(alias = "void RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::getPrimitivesOverlapping<RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>>>(RBX::Extents const&,RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>> &)")]
// was: void RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::getPrimitivesOverlapping<RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>>>(RBX::Extents const&,RBX::DenseHashSet<Ogre::RbxCullableSceneNode*,boost::hash<Ogre::RbxCullableSceneNode*>,std::allocator<Ogre::RbxCullableSceneNode*>> &)
// IDA 0xc296f0: 253 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c296f0() {
}

// 0xc29a00 — __ZN3RBX9LightGrid35lightingUpdatePointLightScratchSIMDILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratchSIMD<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdatePointLightScratchSIMD<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc29a00: 211 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c29a00() {
}

// 0xc29d28 — __ZN3RBX9LightGrid31lightingUpdatePointLightScratchILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratch<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdatePointLightScratch<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc29d28: 180 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c29d28() {
}

// 0xc29f70 — __ZN3RBX9LightGrid35lightingUpdatePointLightScratchSIMDILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratchSIMD<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdatePointLightScratchSIMD<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc29f70: 185 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c29f70() {
}

// 0xc2a218 — __ZN3RBX9LightGrid31lightingUpdatePointLightScratchILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratch<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdatePointLightScratch<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc2a218: 165 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2a218() {
}

// 0xc2a430 — __ZN3RBX9LightGrid34lightingUpdateSpotLightScratchSIMDILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdateSpotLightScratchSIMD<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdateSpotLightScratchSIMD<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc2a430: 307 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2a430() {
}

// 0xc2a8c8 — __ZN3RBX9LightGrid30lightingUpdateSpotLightScratchILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdateSpotLightScratch<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdateSpotLightScratch<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc2a8c8: 238 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2a8c8() {
}

// 0xc2ac10 — __ZN3RBX9LightGrid34lightingUpdateSpotLightScratchSIMDILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdateSpotLightScratchSIMD<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdateSpotLightScratchSIMD<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc2ac10: 278 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2ac10() {
}

// 0xc2b030 — __ZN3RBX9LightGrid30lightingUpdateSpotLightScratchILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef
#[doc(alias = "void RBX::LightGrid::lightingUpdateSpotLightScratch<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
// was: void RBX::LightGrid::lightingUpdateSpotLightScratch<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)
// IDA 0xc2b030: 220 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2b030() {
}

// 0xc2bea0 — __ZN3RBX9LightGrid27lightingComputeShadowMaskYZILb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskYZ<true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskYZ<true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2bea0: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2bea0() {
}

// 0xc2bf38 — __ZN3RBX9LightGrid27lightingComputeShadowMaskYZILb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskYZ<false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskYZ<false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2bf38: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2bf38() {
}

// 0xc2e150 — __ZN3RBX9LightGrid26lightingComputeShadowMaskZILb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2e150: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2e150() {
}

// 0xc2e1d8 — __ZN3RBX9LightGrid26lightingComputeShadowMaskZILb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2e1d8: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2e1d8() {
}

// 0xc2e260 — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2e260: 248 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2e260() {
}

// 0xc2e508 — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2e508: 256 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2e508() {
}

// 0xc2e7c0 — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2e7c0: 236 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2e7c0() {
}

// 0xc2ea4c — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2ea4c: 248 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2ea4c() {
}

// 0xc2ecec — __ZN3RBX9LightGrid26lightingComputeShadowMaskZILb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2ecec: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2ecec() {
}

// 0xc2ed74 — __ZN3RBX9LightGrid26lightingComputeShadowMaskZILb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2ed74: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2ed74() {
}

// 0xc2edfc — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2edfc: 260 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2edfc() {
}

// 0xc2f0b8 — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2f0b8: 267 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2f0b8() {
}

// 0xc2f394 — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2f394: 241 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2f394() {
}

// 0xc2f630 — __ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xc2f630: 239 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2f630() {
}

// 0xc2f9bc — __ZN3RBX12DenseHashSetIPN4Ogre20RbxCullableSceneNodeEN5boost4hashIS3_EESaIS3_EE6insertERKS3_
#[doc(alias = "RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::insert(Ogre::RbxCullableSceneNode * const&)")]
// was: RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::insert(Ogre::RbxCullableSceneNode * const&)
// IDA 0xc2f9bc: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2f9bc() {
}

// 0xc2facc — __ZN3RBX12DenseHashSetIPN4Ogre20RbxCullableSceneNodeEN5boost4hashIS3_EESaIS3_EE6rehashEv
#[doc(alias = "RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::rehash(void)")]
// was: RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::rehash(void)
// IDA 0xc2facc: 141 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2facc() {
}

// 0xc2fc44 — __ZN3RBX12DenseHashSetIPN4Ogre20RbxCullableSceneNodeEN5boost4hashIS3_EESaIS3_EEC2ERKS3_mRKS6_
#[doc(alias = "RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::DenseHashSet(Ogre::RbxCullableSceneNode * const&,unsigned long,boost::hash<Ogre::RbxCullableSceneNode *> const&)")]
// was: RBX::DenseHashSet<Ogre::RbxCullableSceneNode *,boost::hash<Ogre::RbxCullableSceneNode *>,std::allocator<Ogre::RbxCullableSceneNode *>>::DenseHashSet(Ogre::RbxCullableSceneNode * const&,unsigned long,boost::hash<Ogre::RbxCullableSceneNode *> const&)
// IDA 0xc2fc44: 117 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2fc44() {
}

// 0xc2fdb0 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::realloc(int)")]
// was: G3D::Array<G3D::Plane,10,32ul>::realloc(int)
// IDA 0xc2fdb0: 165 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2fdb0() {
}

// 0xc2ffe8 — __ZN3G3D5PlaneD0Ev
#[doc(alias = "G3D::Plane::~Plane()")]
// was: G3D::Plane::~Plane()
// IDA 0xc2ffe8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c2ffe8() {
}

// 0xc2ffec — __ZNK3RBX27CornerWedgeDistanceFunctionclERKN3G3D7Vector3E
#[doc(alias = "RBX::CornerWedgeDistanceFunction::operator()(G3D::Vector3 const&)const")]
// was: RBX::CornerWedgeDistanceFunction::operator()(G3D::Vector3 const&)const
// IDA 0xc2ffec: 77 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2ffec() {
}

// 0xc30bf0 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::~Array()")]
// was: G3D::Array<G3D::Plane,10,32ul>::~Array()
// IDA 0xc30bf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c30bf0() {
}

// 0xc30d54 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE5_copyERKS2_
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::_copy(G3D::Array<G3D::Plane,10,32ul> const&)")]
// was: G3D::Array<G3D::Plane,10,32ul>::_copy(G3D::Array<G3D::Plane,10,32ul> const&)
// IDA 0xc30d54: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c30d54() {
}

// 0xc30f00 — __ZN4Ogre9SharedPtrINS_7TextureEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Texture>::~SharedPtr()
// IDA 0xc30f00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c30f00() {
}

// 0xc30f58 — __ZN4Ogre10TexturePtrD0Ev
#[doc(alias = "Ogre::TexturePtr::~TexturePtr()")]
// was: Ogre::TexturePtr::~TexturePtr()
// IDA 0xc30f58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c30f58() {
}

// 0xc31068 — __ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::~SharedPtr()
// IDA 0xc31068: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c31068() {
}

// 0xc31984 — __ZN3RBX11LightObjectC1EPN4Ogre12SceneManagerE
#[doc(alias = "RBX::LightObject::LightObject(Ogre::SceneManager *)")]
// was: RBX::LightObject::LightObject(Ogre::SceneManager *)
// IDA 0xc31984: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c31984() {
}

// 0xc31988 — __ZN3RBX11LightObjectC2EPN4Ogre12SceneManagerE
#[doc(alias = "RBX::LightObject::LightObject(Ogre::SceneManager *)")]
// was: RBX::LightObject::LightObject(Ogre::SceneManager *)
// IDA 0xc31988: 165 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c31988() {
}

// 0xc33b64 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS2_PKNS_12ExtentsInt32ES9_S9_b
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::changeMinMax(Ogre::RbxCullableSceneNode*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::changeMinMax(Ogre::RbxCullableSceneNode*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)
// IDA 0xc33b64: 158 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c33b64() {
}

// 0xc33cf8 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS2_RKNS_12Vector3int32E
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findNode(Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findNode(Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&)
// IDA 0xc33cf8: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c33cf8() {
}

// 0xc33e08 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS5_11SpatialNodeE
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)
// IDA 0xc33e08: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c33e08() {
}

// 0xc33edc — __ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE7destroyEPS8_
#[doc(alias = "boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::destroy(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode*)")]
// was: boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::destroy(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode*)
// IDA 0xc33edc: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c33edc() {
}

// 0xc34074 — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)
// IDA 0xc34074: 248 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c34074() {
}

// 0xc3432c — __ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS5_11SpatialNodeEPS2_RKNS_12Vector3int32Ei
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&,int)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&,int)
// IDA 0xc3432c: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3432c() {
}

// 0xc343f0 — __ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE9constructEv
#[doc(alias = "boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::construct(void)")]
// was: boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::construct(void)
// IDA 0xc343f0: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c343f0() {
}

// 0xc3457c — __ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
// was: RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)
// IDA 0xc3457c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3457c() {
}

// 0xc34624 — __ZN5boost14singleton_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// was: boost::singleton_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
// IDA 0xc34624: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c34624() {
}

// 0xc34694 — __ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEE9constructIiiNS1_12Vector3int32EEEPS8_RT_RT0_RKT1_
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode* boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::construct<int,int,RBX::Vector3int32>(int &,int &,RBX::Vector3int32 const&)")]
// was: RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode* boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::construct<int,int,RBX::Vector3int32>(int &,int &,RBX::Vector3int32 const&)
// IDA 0xc34694: 88 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c34694() {
}

// 0xc34808 — __ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
// was: RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)
// IDA 0xc34808: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c34808() {
}

// 0xc348b0 — __ZN5boost14singleton_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// was: boost::singleton_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
// IDA 0xc348b0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c348b0() {
}

// 0xc350ac — __ZN3RBX12RenderEntityC1EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh
#[doc(alias = "RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")]
// was: RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)
// IDA 0xc350ac: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c350ac() {
}

// 0xc350c8 — __ZN3RBX12RenderEntityC2EPNS_10RenderNodeEPN4Ogre10VertexDataEPNS3_9IndexDataERKNS3_11MaterialPtrEhh
#[doc(alias = "RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)")]
// was: RBX::RenderEntity::RenderEntity(RBX::RenderNode *,Ogre::VertexData *,Ogre::IndexData *,Ogre::MaterialPtr const&,unsigned char,unsigned char)
// IDA 0xc350c8: 328 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c350c8() {
}

// 0xc357cc — __ZN3RBX12RenderEntity17updateRenderQueueEPN4Ogre11RenderQueueEj
#[doc(alias = "RBX::RenderEntity::updateRenderQueue(Ogre::RenderQueue *,unsigned int)")]
// was: RBX::RenderEntity::updateRenderQueue(Ogre::RenderQueue *,unsigned int)
// IDA 0xc357cc: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c357cc() {
}

// 0xc3583c — __ZN3RBX12RenderEntity17setActualMaterialERKN4Ogre11MaterialPtrE
#[doc(alias = "RBX::RenderEntity::setActualMaterial(Ogre::MaterialPtr const&)")]
// was: RBX::RenderEntity::setActualMaterial(Ogre::MaterialPtr const&)
// IDA 0xc3583c: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3583c() {
}

// 0xc359f0 — __ZN3RBX12RenderEntity18getRenderOperationERN4Ogre15RenderOperationE
#[doc(alias = "RBX::RenderEntity::getRenderOperation(Ogre::RenderOperation &)")]
// was: RBX::RenderEntity::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xc359f0: 12 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c359f0() {
}

// 0xc35a08 — __ZNK3RBX12RenderEntity18getWorldTransformsEPN4Ogre7Matrix4E
#[doc(alias = "RBX::RenderEntity::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: RBX::RenderEntity::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xc35a08: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35a08() {
}

// 0xc35a50 — __ZNK3RBX12RenderEntity19getSquaredViewDepthEPKN4Ogre6CameraE
#[doc(alias = "RBX::RenderEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: RBX::RenderEntity::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xc35a50: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35a50() {
}

// 0xc35ab8 — __ZN3RBX10RenderNodeC1EPN4Ogre12SceneManagerERKSs
#[doc(alias = "RBX::RenderNode::RenderNode(Ogre::SceneManager *,std::string const&)")]
// was: RBX::RenderNode::RenderNode(Ogre::SceneManager *,std::string const&)
// IDA 0xc35ab8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35ab8() {
}

// 0xc35aec — __ZN3RBX10RenderNodeC2EPN4Ogre12SceneManagerERKSs
#[doc(alias = "RBX::RenderNode::RenderNode(Ogre::SceneManager *,std::string const&)")]
// was: RBX::RenderNode::RenderNode(Ogre::SceneManager *,std::string const&)
// IDA 0xc35aec: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35aec() {
}

// 0xc35f1c — __ZN3RBX10RenderNode9setBoundsERKN4Ogre14AxisAlignedBoxE
#[doc(alias = "RBX::RenderNode::setBounds(Ogre::AxisAlignedBox const&)")]
// was: RBX::RenderNode::setBounds(Ogre::AxisAlignedBox const&)
// IDA 0xc35f1c: 26 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35f1c() {
}

// 0xc35f84 — __ZN3RBX10RenderNode19_findVisibleObjectsEPN4Ogre6CameraEPNS1_11RenderQueueEPNS1_24VisibleObjectsBoundsInfoEbbb
#[doc(alias = "RBX::RenderNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
// was: RBX::RenderNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)
// IDA 0xc35f84: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c35f84() {
}

// 0xc367a8 — __ZN3RBX5Water6createEPN4Ogre12VisualEngineE
#[doc(alias = "RBX::Water::create(Ogre::VisualEngine *)")]
// was: RBX::Water::create(Ogre::VisualEngine *)
// IDA 0xc367a8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c367a8() {
}

// 0xc36858 — __ZN3RBX9WaterImplC2EPN4Ogre12VisualEngineE
#[doc(alias = "RBX::WaterImpl::WaterImpl(Ogre::VisualEngine *)")]
// was: RBX::WaterImpl::WaterImpl(Ogre::VisualEngine *)
// IDA 0xc36858: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c36858() {
}

// 0xc37440 — __ZN4Ogre20GpuProgramParametersD2Ev
#[doc(alias = "Ogre::GpuProgramParameters::~GpuProgramParameters()")]
// was: Ogre::GpuProgramParameters::~GpuProgramParameters()
// IDA 0xc37440: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c37440() {
}

// 0xc376b8 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::~SharedPtr()
// IDA 0xc376b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c376b8() {
}

// 0xc376e8 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>> *)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>> *)
// IDA 0xc376e8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c376e8() {
}

// 0xc37710 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuNamedConstants>::~SharedPtr()
// IDA 0xc37710: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c37710() {
}

// 0xc377d0 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::GpuNamedConstants>::destroy(void)
// IDA 0xc377d0: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c377d0() {
}

// 0xc378d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuConstantDefinition>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuConstantDefinition>> *)
// IDA 0xc378d8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c378d8() {
}

// 0xc37950 — __ZNSt6vectorIN4Ogre24GpuSharedParametersUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
// was: std::vector<Ogre::GpuSharedParametersUsage,Ogre::STLAllocator<Ogre::GpuSharedParametersUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
// IDA 0xc37950: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c37950() {
}

// 0xc37aa8 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()
// IDA 0xc37aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c37aa8() {
}

// 0xc37b00 — __ZN4Ogre3AnyD0Ev
#[doc(alias = "Ogre::Any::~Any()")]
// was: Ogre::Any::~Any()
// IDA 0xc37b00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c37b00() {
}

// 0xc37bc8 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)
// IDA 0xc37bc8: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c37bc8() {
}

// 0xc38330 — __ZN4Ogre9SharedPtrINS_7TextureEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::Texture>::destroy(void)
// IDA 0xc38330: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c38330() {
}

// 0xc38368 — __ZN4Ogre9SharedPtrINS_7TextureEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::swap(Ogre::SharedPtr<Ogre::Texture>&)")]
// was: Ogre::SharedPtr<Ogre::Texture>::swap(Ogre::SharedPtr<Ogre::Texture>&)
// IDA 0xc38368: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c38368() {
}

// 0xc38f24 — __ZN3G3D11BinaryInput14loadIntoMemoryExx
#[doc(alias = "G3D::BinaryInput::loadIntoMemory(long long,long long)")]
// was: G3D::BinaryInput::loadIntoMemory(long long,long long)
// IDA 0xc38f24: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c38f24() {
}

// 0xc3902c — __ZN3G3D11BinaryInputC1ERKSsNS_9G3DEndianEb
#[doc(alias = "G3D::BinaryInput::BinaryInput(std::string const&,G3D::G3DEndian,bool)")]
// was: G3D::BinaryInput::BinaryInput(std::string const&,G3D::G3DEndian,bool)
// IDA 0xc3902c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3902c() {
}

// 0xc39038 — __ZN3G3D11BinaryInputC2ERKSsNS_9G3DEndianEb
#[doc(alias = "G3D::BinaryInput::BinaryInput(std::string const&,G3D::G3DEndian,bool)")]
// was: G3D::BinaryInput::BinaryInput(std::string const&,G3D::G3DEndian,bool)
// IDA 0xc39038: 467 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c39038() {
}

// 0xc3955c — __ZN3G3D11BinaryInput10decompressEv
#[doc(alias = "G3D::BinaryInput::decompress(void)")]
// was: G3D::BinaryInput::decompress(void)
// IDA 0xc3955c: 164 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3955c() {
}

// 0xc3978c — __ZN3G3D11BinaryInputD0Ev
#[doc(alias = "G3D::BinaryInput::~BinaryInput()")]
// was: G3D::BinaryInput::~BinaryInput()
// IDA 0xc3978c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c3978c() {
}

// 0xc398cc — __ZN3G3D11BinaryInputD1Ev
#[doc(alias = "G3D::BinaryInput::~BinaryInput()")]
// was: G3D::BinaryInput::~BinaryInput()
// IDA 0xc398cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c398cc() {
}

// 0xc39a04 — __ZN3G3D11BinaryInput10readStringEx
#[doc(alias = "G3D::BinaryInput::readString(long long)")]
// was: G3D::BinaryInput::readString(long long)
// IDA 0xc39a04: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c39a04() {
}

// 0xc39b84 — __ZN3G3D5ArrayIbLi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<bool,10,32ul>::resize(int,bool)")]
// was: G3D::Array<bool,10,32ul>::resize(int,bool)
// IDA 0xc39b84: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c39b84() {
}

// 0xc39c28 — __ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE14getCreateEntryERKSsRb
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)")]
// was: G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::getCreateEntry(std::string const&,bool &)
// IDA 0xc39c28: 166 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c39c28() {
}

// 0xc39de8 — __ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEE6resizeEm
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)")]
// was: G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::resize(unsigned long)
// IDA 0xc39de8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c39de8() {
}

// 0xc3a3e0 — __ZN3G3D5ArrayIbLi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<bool,10,32ul>::realloc(int)")]
// was: G3D::Array<bool,10,32ul>::realloc(int)
// IDA 0xc3a3e0: 140 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3a3e0() {
}

// 0xc3a5e4 — __ZN3G3D5ArrayISsLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<std::string,10,32ul>::~Array()")]
// was: G3D::Array<std::string,10,32ul>::~Array()
// IDA 0xc3a5e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c3a5e4() {
}

// 0xc3a760 — __ZN3G3D5ArrayISsLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<std::string,10,32ul>::Array(void)")]
// was: G3D::Array<std::string,10,32ul>::Array(void)
// IDA 0xc3a760: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3a760() {
}

// 0xc3a94c — __ZN3G3D3BoxC1ERKNS_5AABoxE
#[doc(alias = "G3D::Box::Box(G3D::AABox const&)")]
// was: G3D::Box::Box(G3D::AABox const&)
// IDA 0xc3a94c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3a94c() {
}

// 0xc3a96c — __ZN3G3D3Box4initERKNS_7Vector3ES3_
#[doc(alias = "G3D::Box::init(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: G3D::Box::init(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0xc3a96c: 166 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3a96c() {
}

// 0xc3abec — __ZN3G3D7CapsuleC1ERKNS_7Vector3ES3_f
#[doc(alias = "G3D::Capsule::Capsule(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// was: G3D::Capsule::Capsule(G3D::Vector3 const&,G3D::Vector3 const&,float)
// IDA 0xc3abec: 10 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3abec() {
}

// 0xc3ac40 — __ZN3G3D18CollisionDetection38collisionTimeForMovingPointFixedSphereERKNS_7Vector3ES3_RKNS_6SphereERS1_S7_b
#[doc(alias = "G3D::CollisionDetection::collisionTimeForMovingPointFixedSphere(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Sphere const&,G3D::Vector3&,G3D::Vector3&,bool)")]
// was: G3D::CollisionDetection::collisionTimeForMovingPointFixedSphere(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Sphere const&,G3D::Vector3&,G3D::Vector3&,bool)
// IDA 0xc3ac40: 149 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3ac40() {
}

// 0xc3ae54 — __ZN3G3D18CollisionDetection41collisionLocationForMovingPointFixedAABoxERKNS_7Vector3ES3_RKNS_5AABoxERS1_RbS7_
#[doc(alias = "G3D::CollisionDetection::collisionLocationForMovingPointFixedAABox(G3D::Vector3 const&,G3D::Vector3 const&,G3D::AABox const&,G3D::Vector3&,bool &,G3D::Vector3&)")]
// was: G3D::CollisionDetection::collisionLocationForMovingPointFixedAABox(G3D::Vector3 const&,G3D::Vector3 const&,G3D::AABox const&,G3D::Vector3&,bool &,G3D::Vector3&)
// IDA 0xc3ae54: 194 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c3ae54() {
}
