//! core bg11 — 100 core stubs EA-sorted asc distinct not yet in rbx_core nor global set.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in crates/rbx_core/src nor /tmp/global_eas.txt — next 100 uncovered after 0xb623b0 (prior max 0xb623b0) -> 0xb6256c..0xbc1d7c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Rb_tree<float,std::pair<float const,RBX::StreamRegion::Id>,std::_Select1st<std::pair<float const,RBX::StreamRegion::Id>>,std::less<float>,std::allocator<std::pair<float const,RBX::StreamRegion::Id>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,RBX::StreamRegion::Id>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfN3RBX12StreamRegion2IdEESt10_Select1stIS5_ESt4lessIfESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xb6256c — __ZNSt8_Rb_treeIfSt4pairIKfN3RBX12StreamRegion2IdEESt10_Select1stIS5_ESt4lessIfESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xb6256c() -> ! {
    todo!("0xb6256c __ZNSt8_Rb_treeIfSt4pairIKfN3RBX12StreamRegion2IdEESt10_Select1stIS5_ESt4lessIfESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamDataItem::~StreamDataItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD1Ev")]
// 0xb62598 — __ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob::StreamDataItem *__hidden this)
pub fn stub_0xb62598() -> ! {
    todo!("0xb62598 __ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamDataItem::~StreamDataItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD0Ev")]
// 0xb626d4 — __ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob::StreamDataItem *__hidden this)
pub fn stub_0xb626d4() -> ! {
    todo!("0xb626d4 __ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::JoinDataItem::~JoinDataItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator12JoinDataItemD0Ev")]
// 0xb628e0 — __ZN3RBX7Network10Replicator12JoinDataItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::JoinDataItem *__hidden this)
pub fn stub_0xb628e0() -> ! {
    todo!("0xb628e0 __ZN3RBX7Network10Replicator12JoinDataItemD0Ev")
}

#[doc(alias = "std::vector<RBX::Guid::Data,std::allocator<RBX::Guid::Data>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Guid::Data*,std::vector<RBX::Guid::Data,std::allocator<RBX::Guid::Data>>>,RBX::Guid::Data const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4Guid4DataESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xb6848c — __ZNSt6vectorIN3RBX4Guid4DataESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _QWORD *)
pub fn stub_0xb6848c() -> ! {
    todo!("0xb6848c __ZNSt6vectorIN3RBX4Guid4DataESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::VertexDeclarationManager::VertexDeclarationManager(void)")]
#[doc(alias = "__ZN3RBX24VertexDeclarationManagerC1Ev")]
// 0xb68e2c — __ZN3RBX24VertexDeclarationManagerC1Ev
// type: int __fastcall(int this)
pub fn stub_0xb68e2c() -> ! {
    todo!("0xb68e2c __ZN3RBX24VertexDeclarationManagerC1Ev")
}

#[doc(alias = "RBX::VertexDeclarationManager::~VertexDeclarationManager()")]
#[doc(alias = "__ZN3RBX24VertexDeclarationManagerD1Ev")]
// 0xb68e40 — __ZN3RBX24VertexDeclarationManagerD1Ev
// type: void __fastcall(RBX::VertexDeclarationManager *__hidden this)
pub fn stub_0xb68e40() -> ! {
    todo!("0xb68e40 __ZN3RBX24VertexDeclarationManagerD1Ev")
}

#[doc(alias = "RBX::VertexDeclarationManager::get(std::string const&)")]
#[doc(alias = "__ZN3RBX24VertexDeclarationManager3getERKSs")]
// 0xb68e50 — __ZN3RBX24VertexDeclarationManager3getERKSs
// type: int __fastcall(RBX::VertexDeclarationManager *this, const std::string *, int)
pub fn stub_0xb68e50() -> ! {
    todo!("0xb68e50 __ZN3RBX24VertexDeclarationManager3getERKSs")
}

#[doc(alias = "RBX::FastClusterShadowGenerator::extractIndexData(std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>> &,unsigned short const*,unsigned int)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator16extractIndexDataERSt6vectorINS_14ShadowTriangleESaIS2_EEPKtj")]
// 0xb69c8c — __ZN3RBX26FastClusterShadowGenerator16extractIndexDataERSt6vectorINS_14ShadowTriangleESaIS2_EEPKtj
// type: int __fastcall(int *, int, unsigned int)
pub fn stub_0xb69c8c() -> ! {
    todo!("0xb69c8c __ZN3RBX26FastClusterShadowGenerator16extractIndexDataERSt6vectorINS_14ShadowTriangleESaIS2_EEPKtj")
}

#[doc(alias = "RBX::FastClusterShadowGenerator::weldVertices(std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> &,std::vector&<RBX::ShadowTriangle,std::allocator<std::vector&>>)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator12weldVerticesERSt6vectorINS0_6VertexESaIS2_EERS1_INS_14ShadowTriangleESaIS6_EE")]
// 0xb69d78 — __ZN3RBX26FastClusterShadowGenerator12weldVerticesERSt6vectorINS0_6VertexESaIS2_EERS1_INS_14ShadowTriangleESaIS6_EE
// type: void __fastcall(_DWORD *, __int64 *)
pub fn stub_0xb69d78() -> ! {
    todo!("0xb69d78 __ZN3RBX26FastClusterShadowGenerator12weldVerticesERSt6vectorINS0_6VertexESaIS2_EERS1_INS_14ShadowTriangleESaIS6_EE")
}

#[doc(alias = "RBX::FastClusterShadowGenerator::fillAdjacencyTable(std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>> &,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>> const&,unsigned int)")]
#[doc(alias = "__ZN3RBX26FastClusterShadowGenerator18fillAdjacencyTableERSt6vectorINS_14ShadowTriangleESaIS2_EERKS4_j")]
// 0xb6a0c4 — __ZN3RBX26FastClusterShadowGenerator18fillAdjacencyTableERSt6vectorINS_14ShadowTriangleESaIS2_EERKS4_j
// type: size_t __fastcall(int *, __int64 *, unsigned int)
pub fn stub_0xb6a0c4() -> ! {
    todo!("0xb6a0c4 __ZN3RBX26FastClusterShadowGenerator18fillAdjacencyTableERSt6vectorINS_14ShadowTriangleESaIS2_EERKS4_j")
}

#[doc(alias = "RBX::FastClusterEntity::~FastClusterEntity()")]
#[doc(alias = "__ZN3RBX17FastClusterEntityD0Ev")]
// 0xb6bbe8 — __ZN3RBX17FastClusterEntityD0Ev
// type: void __fastcall(RBX::FastClusterEntity *__hidden this)
pub fn stub_0xb6bbe8() -> ! {
    todo!("0xb6bbe8 __ZN3RBX17FastClusterEntityD0Ev")
}

#[doc(alias = "RBX::FastClusterEntity::~FastClusterEntity()")]
#[doc(alias = "__ZN3RBX17FastClusterEntityD1Ev")]
// 0xb6bcd4 — __ZN3RBX17FastClusterEntityD1Ev
// type: void __fastcall(RBX::FastClusterEntity *__hidden this)
pub fn stub_0xb6bcd4() -> ! {
    todo!("0xb6bcd4 __ZN3RBX17FastClusterEntityD1Ev")
}

#[doc(alias = "RBX::FastClusterEntity::getDebugMaterial(void)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity16getDebugMaterialEv")]
// 0xb6bdb4 — __ZNK3RBX17FastClusterEntity16getDebugMaterialEv
// type: int __fastcall(RBX::FastClusterEntity *this)
pub fn stub_0xb6bdb4() -> ! {
    todo!("0xb6bdb4 __ZNK3RBX17FastClusterEntity16getDebugMaterialEv")
}

#[doc(alias = "RBX::FastClusterEntity::getNumWorldTransforms(void)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity21getNumWorldTransformsEv")]
// 0xb6beb4 — __ZNK3RBX17FastClusterEntity21getNumWorldTransformsEv
// type: int __fastcall(RBX::FastClusterEntity *this)
pub fn stub_0xb6beb4() -> ! {
    todo!("0xb6beb4 __ZNK3RBX17FastClusterEntity21getNumWorldTransformsEv")
}

#[doc(alias = "RBX::FastClusterEntity::getCastsShadows(void)const")]
#[doc(alias = "__ZNK3RBX17FastClusterEntity15getCastsShadowsEv")]
// 0xb6c010 — __ZNK3RBX17FastClusterEntity15getCastsShadowsEv
// type: bool __fastcall(RBX::FastClusterEntity *this)
pub fn stub_0xb6c010() -> ! {
    todo!("0xb6c010 __ZNK3RBX17FastClusterEntity15getCastsShadowsEv")
}

#[doc(alias = "RBX::FastClusterBinding::invalidateEntity(void)")]
#[doc(alias = "__ZN3RBX18FastClusterBinding16invalidateEntityEv")]
// 0xb6c228 — __ZN3RBX18FastClusterBinding16invalidateEntityEv
// type: int __fastcall(RBX::FastClusterBinding *this)
pub fn stub_0xb6c228() -> ! {
    todo!("0xb6c228 __ZN3RBX18FastClusterBinding16invalidateEntityEv")
}

#[doc(alias = "RBX::FastClusterBinding::onCoordinateFrameChanged(void)")]
#[doc(alias = "__ZN3RBX18FastClusterBinding24onCoordinateFrameChangedEv")]
// 0xb6c264 — __ZN3RBX18FastClusterBinding24onCoordinateFrameChangedEv
// type: int __fastcall(RBX::FastClusterBinding *this)
pub fn stub_0xb6c264() -> ! {
    todo!("0xb6c264 __ZN3RBX18FastClusterBinding24onCoordinateFrameChangedEv")
}

#[doc(alias = "RBX::FastClusterBinding::onSizeChanged(void)")]
#[doc(alias = "__ZN3RBX18FastClusterBinding13onSizeChangedEv")]
// 0xb6c2d0 — __ZN3RBX18FastClusterBinding13onSizeChangedEv
// type: int __fastcall(RBX::FastClusterBinding *this)
pub fn stub_0xb6c2d0() -> ! {
    todo!("0xb6c2d0 __ZN3RBX18FastClusterBinding13onSizeChangedEv")
}

#[doc(alias = "RBX::FastClusterBinding::onTransparencyChanged(void)")]
#[doc(alias = "__ZN3RBX18FastClusterBinding21onTransparencyChangedEv")]
// 0xb6c334 — __ZN3RBX18FastClusterBinding21onTransparencyChangedEv
// type: int __fastcall(RBX::FastClusterBinding *this)
pub fn stub_0xb6c334() -> ! {
    todo!("0xb6c334 __ZN3RBX18FastClusterBinding21onTransparencyChangedEv")
}

#[doc(alias = "RBX::FastClusterBinding::onSpecialShapeChanged(void)")]
#[doc(alias = "__ZN3RBX18FastClusterBinding21onSpecialShapeChangedEv")]
// 0xb6c374 — __ZN3RBX18FastClusterBinding21onSpecialShapeChangedEv
// type: int __fastcall(RBX::FastClusterBinding *this)
pub fn stub_0xb6c374() -> ! {
    todo!("0xb6c374 __ZN3RBX18FastClusterBinding21onSpecialShapeChangedEv")
}

#[doc(alias = "RBX::FastClusterBinding::unbind(void)")]
#[doc(alias = "__ZN3RBX18FastClusterBinding6unbindEv")]
// 0xb6c3b4 — __ZN3RBX18FastClusterBinding6unbindEv
// type: int __fastcall(RBX::FastClusterBinding *this)
pub fn stub_0xb6c3b4() -> ! {
    todo!("0xb6c3b4 __ZN3RBX18FastClusterBinding6unbindEv")
}

#[doc(alias = "RBX::FastCluster::~FastCluster()")]
#[doc(alias = "__ZN3RBX11FastClusterD0Ev")]
// 0xb6ca68 — __ZN3RBX11FastClusterD0Ev
// type: void __fastcall(RBX::FastCluster *__hidden this)
pub fn stub_0xb6ca68() -> ! {
    todo!("0xb6ca68 __ZN3RBX11FastClusterD0Ev")
}

#[doc(alias = "RBX::FastCluster::~FastCluster()")]
#[doc(alias = "__ZN3RBX11FastClusterD1Ev")]
// 0xb6cb1c — __ZN3RBX11FastClusterD1Ev
// type: void __fastcall(RBX::FastCluster *__hidden this)
pub fn stub_0xb6cb1c() -> ! {
    todo!("0xb6cb1c __ZN3RBX11FastClusterD1Ev")
}

#[doc(alias = "RBX::FastCluster::~FastCluster()")]
#[doc(alias = "__ZN3RBX11FastClusterD2Ev")]
// 0xb6cb20 — __ZN3RBX11FastClusterD2Ev
// type: void __fastcall(RBX::FastCluster *this, int, int)
pub fn stub_0xb6cb20() -> ! {
    todo!("0xb6cb20 __ZN3RBX11FastClusterD2Ev")
}

#[doc(alias = "RBX::FastCluster::checkCluster(void)")]
#[doc(alias = "__ZN3RBX11FastCluster12checkClusterEv")]
// 0xb6d248 — __ZN3RBX11FastCluster12checkClusterEv
// type: void __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6d248() -> ! {
    todo!("0xb6d248 __ZN3RBX11FastCluster12checkClusterEv")
}

#[doc(alias = "RBX::FastCluster::priorityInvalidateEntity(void)")]
#[doc(alias = "__ZN3RBX11FastCluster24priorityInvalidateEntityEv")]
// 0xb6d71c — __ZN3RBX11FastCluster24priorityInvalidateEntityEv
// type: int __fastcall(RBX::FastCluster *this, int, int, const void *)
pub fn stub_0xb6d71c() -> ! {
    todo!("0xb6d71c __ZN3RBX11FastCluster24priorityInvalidateEntityEv")
}

#[doc(alias = "RBX::FastCluster::invalidateEntity(void)")]
#[doc(alias = "__ZN3RBX11FastCluster16invalidateEntityEv")]
// 0xb6d7b8 — __ZN3RBX11FastCluster16invalidateEntityEv
// type: int __fastcall(RBX::FastCluster *this, int, int, const void *)
pub fn stub_0xb6d7b8() -> ! {
    todo!("0xb6d7b8 __ZN3RBX11FastCluster16invalidateEntityEv")
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::invalidateEntity(void)")]
#[doc(alias = "__ZThn392_N3RBX11FastCluster16invalidateEntityEv")]
// 0xb6d808 — __ZThn392_N3RBX11FastCluster16invalidateEntityEv
// type: int __fastcall(RBX::FastCluster *this, int, int, const void *)
pub fn stub_0xb6d808() -> ! {
    todo!("0xb6d808 __ZThn392_N3RBX11FastCluster16invalidateEntityEv")
}

#[doc(alias = "RBX::FastCluster::checkBindings(void)")]
#[doc(alias = "__ZN3RBX11FastCluster13checkBindingsEv")]
// 0xb6d85c — __ZN3RBX11FastCluster13checkBindingsEv
// type: void __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6d85c() -> ! {
    todo!("0xb6d85c __ZN3RBX11FastCluster13checkBindingsEv")
}

#[doc(alias = "RBX::FastCluster::updateEntity(bool)")]
#[doc(alias = "__ZN3RBX11FastCluster12updateEntityEb")]
// 0xb6dc20 — __ZN3RBX11FastCluster12updateEntityEb
// type: void __fastcall(RBX::FastCluster *this, int)
pub fn stub_0xb6dc20() -> ! {
    todo!("0xb6dc20 __ZN3RBX11FastCluster12updateEntityEb")
}

#[doc(alias = "RBX::FastCluster::updateClumpGrouping(void)")]
#[doc(alias = "__ZN3RBX11FastCluster19updateClumpGroupingEv")]
// 0xb6e0a8 — __ZN3RBX11FastCluster19updateClumpGroupingEv
// type: int __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6e0a8() -> ! {
    todo!("0xb6e0a8 __ZN3RBX11FastCluster19updateClumpGroupingEv")
}

#[doc(alias = "RBX::FastCluster::updateGeometry(RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX11FastCluster14updateGeometryEPNS_11AsyncResultE")]
// 0xb6e18c — __ZN3RBX11FastCluster14updateGeometryEPNS_11AsyncResultE
// type: RBX::PartInstance *__fastcall(int, int)
pub fn stub_0xb6e18c() -> ! {
    todo!("0xb6e18c __ZN3RBX11FastCluster14updateGeometryEPNS_11AsyncResultE")
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::updateEntity(bool)")]
#[doc(alias = "__ZThn392_N3RBX11FastCluster12updateEntityEb")]
// 0xb6e938 — __ZThn392_N3RBX11FastCluster12updateEntityEb
// type: void __fastcall(RBX::FastCluster *this, bool)
pub fn stub_0xb6e938() -> ! {
    todo!("0xb6e938 __ZThn392_N3RBX11FastCluster12updateEntityEb")
}

#[doc(alias = "RBX::FastCluster::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZN3RBX11FastCluster21updateCoordinateFrameEb")]
// 0xb6e940 — __ZN3RBX11FastCluster21updateCoordinateFrameEb
// type: void __fastcall(RBX::FastCluster *this, bool)
pub fn stub_0xb6e940() -> ! {
    todo!("0xb6e940 __ZN3RBX11FastCluster21updateCoordinateFrameEb")
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::updateCoordinateFrame(bool)")]
#[doc(alias = "__ZThn392_N3RBX11FastCluster21updateCoordinateFrameEb")]
// 0xb6edc4 — __ZThn392_N3RBX11FastCluster21updateCoordinateFrameEb
// type: void __fastcall(RBX::FastCluster *this, bool)
pub fn stub_0xb6edc4() -> ! {
    todo!("0xb6edc4 __ZThn392_N3RBX11FastCluster21updateCoordinateFrameEb")
}

#[doc(alias = "RBX::FastCluster::unbind(void)")]
#[doc(alias = "__ZN3RBX11FastCluster6unbindEv")]
// 0xb6edd0 — __ZN3RBX11FastCluster6unbindEv
// type: void __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6edd0() -> ! {
    todo!("0xb6edd0 __ZN3RBX11FastCluster6unbindEv")
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::unbind(void)")]
#[doc(alias = "__ZThn392_N3RBX11FastCluster6unbindEv")]
// 0xb6f048 — __ZThn392_N3RBX11FastCluster6unbindEv
// type: void __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6f048() -> ! {
    todo!("0xb6f048 __ZThn392_N3RBX11FastCluster6unbindEv")
}

#[doc(alias = "RBX::FastCluster::onClumpChanged(void)")]
#[doc(alias = "__ZN3RBX11FastCluster14onClumpChangedEv")]
// 0xb6f0e8 — __ZN3RBX11FastCluster14onClumpChangedEv
// type: int __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6f0e8() -> ! {
    todo!("0xb6f0e8 __ZN3RBX11FastCluster14onClumpChangedEv")
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::onClumpChanged(void)")]
#[doc(alias = "__ZThn392_N3RBX11FastCluster14onClumpChangedEv")]
// 0xb6f128 — __ZThn392_N3RBX11FastCluster14onClumpChangedEv
// type: int __fastcall(RBX::FastCluster *this)
pub fn stub_0xb6f128() -> ! {
    todo!("0xb6f128 __ZThn392_N3RBX11FastCluster14onClumpChangedEv")
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE7reserveEm")]
// 0xb6f188 — __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE7reserveEm
// type: unsigned int __fastcall(void **, unsigned int)
pub fn stub_0xb6f188() -> ! {
    todo!("0xb6f188 __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE7reserveEm")
}

#[doc(alias = "RBX::Allocator<RBX::FastClusterBinding>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18FastClusterBindingEEC2Ev")]
// 0xb6f49c — __ZN3RBX9AllocatorINS_18FastClusterBindingEEC2Ev
// type: int __fastcall(int)
pub fn stub_0xb6f49c() -> ! {
    todo!("0xb6f49c __ZN3RBX9AllocatorINS_18FastClusterBindingEEC2Ev")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE")]
// 0xb70210 — __ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE
// type: int __fastcall(int, bool *, int, _DWORD *)
pub fn stub_0xb70210() -> ! {
    todo!("0xb70210 __ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::finalize(RBX::FastCluster *,RBX::eShadowCullingPriority)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator8finalizeEPNS_11FastClusterENS_22eShadowCullingPriorityE")]
// 0xb70af8 — __ZN3RBX24FastClusterMeshGenerator8finalizeEPNS_11FastClusterENS_22eShadowCullingPriorityE
// type: int __fastcall(int, bool *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, void *, int, int, void *, int, int, int, int)
pub fn stub_0xb70af8() -> ! {
    todo!("0xb70af8 __ZN3RBX24FastClusterMeshGenerator8finalizeEPNS_11FastClusterENS_22eShadowCullingPriorityE")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGeneratorD1Ev")]
// 0xb70d90 — __ZN3RBX24FastClusterMeshGeneratorD1Ev
// type: void __fastcall(RBX::FastClusterMeshGenerator *__hidden this)
pub fn stub_0xb70d90() -> ! {
    todo!("0xb70d90 __ZN3RBX24FastClusterMeshGeneratorD1Ev")
}

#[doc(alias = "RBX::GfxBinding::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
#[doc(alias = "__ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb")]
// 0xb71010 — __ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb
// type: void()
pub fn stub_0xb71010() -> ! {
    todo!("0xb71010 __ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb")
}

#[doc(alias = "RBX::GfxBinding::onSizeChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding13onSizeChangedEv")]
// 0xb71018 — __ZN3RBX10GfxBinding13onSizeChangedEv
// type: int __fastcall(RBX::GfxBinding *this)
pub fn stub_0xb71018() -> ! {
    todo!("0xb71018 __ZN3RBX10GfxBinding13onSizeChangedEv")
}

#[doc(alias = "RBX::GfxBinding::onTransparencyChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding21onTransparencyChangedEv")]
// 0xb71020 — __ZN3RBX10GfxBinding21onTransparencyChangedEv
// type: int __fastcall(RBX::GfxBinding *this)
pub fn stub_0xb71020() -> ! {
    todo!("0xb71020 __ZN3RBX10GfxBinding21onTransparencyChangedEv")
}

#[doc(alias = "RBX::GfxBinding::onSpecialShapeChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding21onSpecialShapeChangedEv")]
// 0xb71028 — __ZN3RBX10GfxBinding21onSpecialShapeChangedEv
// type: int __fastcall(RBX::GfxBinding *this)
pub fn stub_0xb71028() -> ! {
    todo!("0xb71028 __ZN3RBX10GfxBinding21onSpecialShapeChangedEv")
}

#[doc(alias = "std::vector<RBX::FastCluster::Bone,std::allocator<RBX::FastCluster::Bone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FastCluster::Bone*,std::vector<RBX::FastCluster::Bone,std::allocator<RBX::FastCluster::Bone>>>,unsigned long,RBX::FastCluster::Bone const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11FastCluster4BoneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xb71030 — __ZNSt6vectorIN3RBX11FastCluster4BoneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(unsigned int *, char *, unsigned int, int *)
pub fn stub_0xb71030() -> ! {
    todo!("0xb71030 __ZNSt6vectorIN3RBX11FastCluster4BoneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Allocator<RBX::FastClusterBinding>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18FastClusterBindingEE13releaseMemoryEv")]
// 0xb7207c — __ZN3RBX9AllocatorINS_18FastClusterBindingEE13releaseMemoryEv
pub fn stub_0xb7207c() -> ! {
    todo!("0xb7207c __ZN3RBX9AllocatorINS_18FastClusterBindingEE13releaseMemoryEv")
}

#[doc(alias = "RBX::FastClusterShadowData::~FastClusterShadowData()")]
#[doc(alias = "__ZN3RBX21FastClusterShadowDataD2Ev")]
// 0xb720f4 — __ZN3RBX21FastClusterShadowDataD2Ev
// type: void __fastcall(RBX::FastClusterShadowData *__hidden this)
pub fn stub_0xb720f4() -> ! {
    todo!("0xb720f4 __ZN3RBX21FastClusterShadowDataD2Ev")
}

#[doc(alias = "std::vector<char,std::allocator<char>>::_M_fill_insert(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,unsigned long,char const&)")]
#[doc(alias = "__ZNSt6vectorIcSaIcEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS1_EEmRKc")]
// 0xb72230 — __ZNSt6vectorIcSaIcEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS1_EEmRKc
// type: void **__fastcall(void **result, char *, size_t __len, unsigned __int8 *)
pub fn stub_0xb72230() -> ! {
    todo!("0xb72230 __ZNSt6vectorIcSaIcEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS1_EEmRKc")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>> std::__find_if<__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,RBX::FastClusterShadowGenerator::TriangleIsDegeneratePredicate>(__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,RBX::FastClusterShadowGenerator::TriangleIsDegeneratePredicate,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX14ShadowTriangleESt6vectorIS3_SaIS3_EEEENS2_26FastClusterShadowGenerator29TriangleIsDegeneratePredicateEET_SB_SB_T0_St26random_access_iterator_tag")]
// 0xb72354 — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX14ShadowTriangleESt6vectorIS3_SaIS3_EEEENS2_26FastClusterShadowGenerator29TriangleIsDegeneratePredicateEET_SB_SB_T0_St26random_access_iterator_tag
// type: unsigned __int16 *__fastcall(unsigned __int16 *result, int)
pub fn stub_0xb72354() -> ! {
    todo!("0xb72354 __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX14ShadowTriangleESt6vectorIS3_SaIS3_EEEENS2_26FastClusterShadowGenerator29TriangleIsDegeneratePredicateEET_SB_SB_T0_St26random_access_iterator_tag")
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowGenerator::Vertex*,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>>,RBX::FastClusterShadowGenerator::Vertex const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xb72434 — __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: _QWORD *__fastcall(int, __int64 *, int *)
pub fn stub_0xb72434() -> ! {
    todo!("0xb72434 __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ShadowTriangle*,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,unsigned long,RBX::ShadowTriangle const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14ShadowTriangleESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// 0xb72578 — __ZNSt6vectorIN3RBX14ShadowTriangleESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(__int64, unsigned int, int *)
pub fn stub_0xb72578() -> ! {
    todo!("0xb72578 __ZNSt6vectorIN3RBX14ShadowTriangleESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowGenerator::Vertex*,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>>,unsigned long,RBX::FastClusterShadowGenerator::Vertex const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xb727e4 — __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: unsigned int __fastcall(unsigned int result, char *, unsigned int, _DWORD *)
pub fn stub_0xb727e4() -> ! {
    todo!("0xb727e4 __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::MaterialGenerator::~MaterialGenerator()")]
#[doc(alias = "__ZN3RBX17MaterialGeneratorD2Ev")]
// 0xb72a18 — __ZN3RBX17MaterialGeneratorD2Ev
// type: void __fastcall(RBX::MaterialGenerator *__hidden this)
pub fn stub_0xb72a18() -> ! {
    todo!("0xb72a18 __ZN3RBX17MaterialGeneratorD2Ev")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xb72ef0 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xb72ef0() -> ! {
    todo!("0xb72ef0 __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::MaterialGroup::~MaterialGroup()")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator13MaterialGroupD1Ev")]
// 0xb72f20 — __ZN3RBX24FastClusterMeshGenerator13MaterialGroupD1Ev
// type: void __fastcall(RBX::FastClusterMeshGenerator::MaterialGroup *__hidden this)
pub fn stub_0xb72f20() -> ! {
    todo!("0xb72f20 __ZN3RBX24FastClusterMeshGenerator13MaterialGroupD1Ev")
}

#[doc(alias = "std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")]
#[doc(alias = "__ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev")]
// 0xb730b8 — __ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev
// type: _DWORD **__fastcall(_DWORD **)
pub fn stub_0xb730b8() -> ! {
    todo!("0xb730b8 __ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::createVertexData(unsigned int,bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator16createVertexDataEjb")]
// 0xb73268 — __ZN3RBX24FastClusterMeshGenerator16createVertexDataEjb
// type: Ogre::VertexData *__fastcall(RBX::FastClusterMeshGenerator *this, unsigned int, bool)
pub fn stub_0xb73268() -> ! {
    todo!("0xb73268 __ZN3RBX24FastClusterMeshGenerator16createVertexDataEjb")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::createIndexData(unsigned int)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator15createIndexDataEj")]
// 0xb7345c — __ZN3RBX24FastClusterMeshGenerator15createIndexDataEj
// type: Ogre::IndexData *__fastcall(RBX::FastClusterMeshGenerator *this, unsigned int)
pub fn stub_0xb7345c() -> ! {
    todo!("0xb7345c __ZN3RBX24FastClusterMeshGenerator15createIndexDataEj")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::generateBatchGeometry(RBX::FastClusterMeshGenerator::MaterialGroup const&,RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex *,unsigned short *,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> &,bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator21generateBatchGeometryERKNS0_13MaterialGroupERKNS0_5BatchEPNS_17GeometryGenerator6VertexEPtjRSt6vectorIjSaIjEEb")]
// 0xb73678 — __ZN3RBX24FastClusterMeshGenerator21generateBatchGeometryERKNS0_13MaterialGroupERKNS0_5BatchEPNS_17GeometryGenerator6VertexEPtjRSt6vectorIjSaIjEEb
// type: int __fastcall(RBX::FastClusterMeshGenerator *, _DWORD *, int, _DWORD *, int, int, int, int)
pub fn stub_0xb73678() -> ! {
    todo!("0xb73678 __ZN3RBX24FastClusterMeshGenerator21generateBatchGeometryERKNS0_13MaterialGroupERKNS0_5BatchEPNS_17GeometryGenerator6VertexEPtjRSt6vectorIjSaIjEEb")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::generateShadowData(RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb")]
// 0xb73b10 — __ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb
// type: int __fastcall(Ogre::VisualEngine **, int, int, unsigned int, int, unsigned int, _DWORD *, struct _Unwind_Exception *, int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
pub fn stub_0xb73b10() -> ! {
    todo!("0xb73b10 __ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIjSaIjEE7reserveEm")]
// 0xb73ec8 — __ZNSt6vectorIjSaIjEE7reserveEm
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_0xb73ec8() -> ! {
    todo!("0xb73ec8 __ZNSt6vectorIjSaIjEE7reserveEm")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb")]
// 0xb74384 — __ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, int)
pub fn stub_0xb74384() -> ! {
    todo!("0xb74384 __ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::setupSharedGeometry(RBX::FastClusterSharedGeometry &,unsigned int,unsigned int,bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator19setupSharedGeometryERNS_25FastClusterSharedGeometryEjjb")]
// 0xb745cc — __ZN3RBX24FastClusterMeshGenerator19setupSharedGeometryERNS_25FastClusterSharedGeometryEjjb
// type: void __fastcall(Ogre::HardwareBufferManager *, int *, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, char, int, int, int, int, int, int, int)
pub fn stub_0xb745cc() -> ! {
    todo!("0xb745cc __ZN3RBX24FastClusterMeshGenerator19setupSharedGeometryERNS_25FastClusterSharedGeometryEjjb")
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
#[doc(alias = "__ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiNS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_T1_")]
// 0xb74a5c — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiNS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_T1_
// type: int __fastcall(char *, _QWORD *, int)
pub fn stub_0xb74a5c() -> ! {
    todo!("0xb74a5c __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiNS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_T1_")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
#[doc(alias = "__ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_")]
// 0xb74b78 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_
// type: char *__fastcall(char *result, char *)
pub fn stub_0xb74b78() -> ! {
    todo!("0xb74b78 __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
#[doc(alias = "__ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_SG_T0_")]
// 0xb74cf8 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_SG_T0_
// type: signed int __fastcall(char *, _DWORD *, unsigned int)
pub fn stub_0xb74cf8() -> ! {
    todo!("0xb74cf8 __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_SG_T0_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,int,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
#[doc(alias = "__ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiS9_NS4_33BatchMaterialPlasticLODComparatorEEvT_T0_SH_T1_T2_")]
// 0xb74d78 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiS9_NS4_33BatchMaterialPlasticLODComparatorEEvT_T0_SH_T1_T2_
// type: int __fastcall(int result, int, int, int, int)
pub fn stub_0xb74d78() -> ! {
    todo!("0xb74d78 __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiS9_NS4_33BatchMaterialPlasticLODComparatorEEvT_T0_SH_T1_T2_")
}

#[doc(alias = "std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>*,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS2_5BatchEESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_")]
// 0xb74e54 — __ZNSt6vectorISt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS2_5BatchEESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_
// type: int __fastcall(int, char *, _QWORD *)
pub fn stub_0xb74e54() -> ! {
    todo!("0xb74e54 __ZNSt6vectorISt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS2_5BatchEESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_")
}

#[doc(alias = "std::map<unsigned long long,RBX::FastClusterMeshGenerator::MaterialGroup,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::operator[](unsigned long long const&)")]
#[doc(alias = "__ZNSt3mapIyN3RBX24FastClusterMeshGenerator13MaterialGroupESt4lessIyESaISt4pairIKyS2_EEEixERS6_")]
// 0xb74f70 — __ZNSt3mapIyN3RBX24FastClusterMeshGenerator13MaterialGroupESt4lessIyESaISt4pairIKyS2_EEEixERS6_
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, RBX::FastClusterMeshGenerator::MaterialGroup *, int, int, int, int)
pub fn stub_0xb74f70() -> ! {
    todo!("0xb74f70 __ZNSt3mapIyN3RBX24FastClusterMeshGenerator13MaterialGroupESt4lessIyESaISt4pairIKyS2_EEEixERS6_")
}

#[doc(alias = "std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "__ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_")]
// 0xb758a8 — __ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_
// type: void __fastcall(int, std::_List_node_base *, int, int, void *, int)
pub fn stub_0xb758a8() -> ! {
    todo!("0xb758a8 __ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_")]
// 0xb75984 — __ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_
// type: int __fastcall(int, __int64 *)
pub fn stub_0xb75984() -> ! {
    todo!("0xb75984 __ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_")
}

#[doc(alias = "RBX::FastClusterMeshGenerator::MaterialGroup::MaterialGroup(RBX::FastClusterMeshGenerator::MaterialGroup const&)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator13MaterialGroupC2ERKS1_")]
// 0xb75e34 — __ZN3RBX24FastClusterMeshGenerator13MaterialGroupC2ERKS1_
// type: RBX::FastClusterMeshGenerator::MaterialGroup *__fastcall(RBX::FastClusterMeshGenerator::MaterialGroup *this, const RBX::FastClusterMeshGenerator::MaterialGroup *, int, int)
pub fn stub_0xb75e34() -> ! {
    todo!("0xb75e34 __ZN3RBX24FastClusterMeshGenerator13MaterialGroupC2ERKS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xb75fe4 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: struct _Unwind_Exception *__fastcall(_DWORD *, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0xb75fe4() -> ! {
    todo!("0xb75fe4 __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0xb76120 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: _Rb_tree_node_base *__fastcall(int, int, int, int *, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
pub fn stub_0xb76120() -> ! {
    todo!("0xb76120 __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert_unique(std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xb76270 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueERKS5_
// type: _Rb_tree_node_base *__fastcall(int, int, __int64 *)
pub fn stub_0xb76270() -> ! {
    todo!("0xb76270 __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xb7654c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, __int64 *, char **)
pub fn stub_0xb7654c() -> ! {
    todo!("0xb7654c __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm")]
// 0xb7695c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm
// type: unsigned int __fastcall(void **, unsigned int)
pub fn stub_0xb7695c() -> ! {
    todo!("0xb7695c __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm")
}

#[doc(alias = "RBX::MeshGen::addRefVertex(int)")]
#[doc(alias = "__ZN3RBX7MeshGen12addRefVertexEi")]
// 0xb9a3e4 — __ZN3RBX7MeshGen12addRefVertexEi
// type: int __fastcall(RBX::MeshGen *this, int)
pub fn stub_0xb9a3e4() -> ! {
    todo!("0xb9a3e4 __ZN3RBX7MeshGen12addRefVertexEi")
}

#[doc(alias = "RBX::MeshGen::releaseVertex(int)")]
#[doc(alias = "__ZN3RBX7MeshGen13releaseVertexEi")]
// 0xb9a3e8 — __ZN3RBX7MeshGen13releaseVertexEi
// type: void __fastcall(RBX::MeshGen *this, int)
pub fn stub_0xb9a3e8() -> ! {
    todo!("0xb9a3e8 __ZN3RBX7MeshGen13releaseVertexEi")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getVertex(int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi")]
// 0xb9a3ec — __ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this, int)
pub fn stub_0xb9a3ec() -> ! {
    todo!("0xb9a3ec __ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm")]
// 0xb9a50c — __ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, unsigned int)
pub fn stub_0xb9a50c() -> ! {
    todo!("0xb9a50c __ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE")]
// 0xb9a524 — __ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE
// type: int __fastcall(int, _DWORD *)
pub fn stub_0xb9a524() -> ! {
    todo!("0xb9a524 __ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm")]
// 0xb9a558 — __ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, unsigned int)
pub fn stub_0xb9a558() -> ! {
    todo!("0xb9a558 __ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii")]
// 0xb9a568 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, int, int, int)
pub fn stub_0xb9a568() -> ! {
    todo!("0xb9a568 __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii")]
// 0xb9a580 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, int, int, int, int)
pub fn stub_0xb9a580() -> ! {
    todo!("0xb9a580 __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii")]
// 0xb9a5b0 — __ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this, int, int)
pub fn stub_0xb9a5b0() -> ! {
    todo!("0xb9a5b0 __ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getIndexCount(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv")]
// 0xb9a6d0 — __ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this)
pub fn stub_0xb9a6d0() -> ! {
    todo!("0xb9a6d0 __ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv")]
// 0xb9a6d4 — __ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this)
pub fn stub_0xb9a6d4() -> ! {
    todo!("0xb9a6d4 __ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv")
}

#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv")]
// 0xb9a7f4 — __ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this)
pub fn stub_0xb9a7f4() -> ! {
    todo!("0xb9a7f4 __ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv")
}

#[doc(alias = "RBX::MeshGen::popVerticesTransform(void)")]
#[doc(alias = "__ZN3RBX7MeshGen20popVerticesTransformEv")]
// 0xb9a918 — __ZN3RBX7MeshGen20popVerticesTransformEv
// type: void __fastcall(RBX::MeshGen *this)
pub fn stub_0xb9a918() -> ! {
    todo!("0xb9a918 __ZN3RBX7MeshGen20popVerticesTransformEv")
}

#[doc(alias = "RBX::Adorn::~Adorn()")]
#[doc(alias = "__ZN3RBX5AdornD1Ev")]
// 0xb9aa18 — __ZN3RBX5AdornD1Ev
// type: void __fastcall(RBX::Adorn *__hidden this)
pub fn stub_0xb9aa18() -> ! {
    todo!("0xb9aa18 __ZN3RBX5AdornD1Ev")
}

#[doc(alias = "RBX::Adorn::finishRenderPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn16finishRenderPassEv")]
// 0xb9aa20 — __ZN3RBX5Adorn16finishRenderPassEv
// type: void __fastcall(RBX::Adorn *this)
pub fn stub_0xb9aa20() -> ! {
    todo!("0xb9aa20 __ZN3RBX5Adorn16finishRenderPassEv")
}

#[doc(alias = "RBX::Adorn::postSubmitPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn14postSubmitPassEv")]
// 0xb9aa28 — __ZN3RBX5Adorn14postSubmitPassEv
// type: void __fastcall(RBX::Adorn *this)
pub fn stub_0xb9aa28() -> ! {
    todo!("0xb9aa28 __ZN3RBX5Adorn14postSubmitPassEv")
}

#[doc(alias = "RBX::RbxParticleFactory::getParticleSystemsBegin(void)")]
#[doc(alias = "__ZN3RBX18RbxParticleFactory23getParticleSystemsBeginEv")]
// 0xbc1d78 — __ZN3RBX18RbxParticleFactory23getParticleSystemsBeginEv
// type: _DWORD __fastcall(RBX::RbxParticleFactory *__hidden this)
pub fn stub_0xbc1d78() -> ! {
    todo!("0xbc1d78 __ZN3RBX18RbxParticleFactory23getParticleSystemsBeginEv")
}

#[doc(alias = "RBX::RbxParticleFactory::getParticleSystemsEnd(void)")]
#[doc(alias = "__ZN3RBX18RbxParticleFactory21getParticleSystemsEndEv")]
// 0xbc1d7c — __ZN3RBX18RbxParticleFactory21getParticleSystemsEndEv
// type: _DWORD __fastcall(RBX::RbxParticleFactory *__hidden this)
pub fn stub_0xbc1d7c() -> ! {
    todo!("0xbc1d7c __ZN3RBX18RbxParticleFactory21getParticleSystemsEndEv")
}
