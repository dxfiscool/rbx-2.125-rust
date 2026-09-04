//! core shard HC — 100 core stubs EA-sorted, 0xf57c34..0xf588d4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HB 0xf57c24).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HB 0xf57c24 (0xf57c34..0xf588d4, 20614->20714 covered, 1204 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
// 0xf57c34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf57c34() {
    // IDA 0xf57c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
// 0xf57c44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf57c44() {
    // IDA 0xf57c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
// 0xf57c54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf57c54() {
    // IDA 0xf57c54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
// 0xf57c64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf57c64() {
    // IDA 0xf57c64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
// 0xf57c74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf57c74() {
    // IDA 0xf57c74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
// 0xf57c84 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf57c84() {
    // IDA 0xf57c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
// 0xf57c94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf57c94() {
    // IDA 0xf57c94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
// 0xf57ca4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf57ca4() {
    // IDA 0xf57ca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
// 0xf57cb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf57cb4() {
    // IDA 0xf57cb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
// 0xf57cc4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf57cc4() {
    // IDA 0xf57cc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
// 0xf57cd4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf57cd4() {
    // IDA 0xf57cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
// 0xf57ce4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf57ce4() {
    // IDA 0xf57ce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
// 0xf57cf4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf57cf4() {
    // IDA 0xf57cf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
// 0xf57d04 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf57d04() {
    // IDA 0xf57d04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener *>(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&,std::random_access_iterator_tag)")]
// 0xf57d14 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX5Voxel18CellChangeListenerESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
pub fn stub_0xf57d14() {
    // IDA 0xf57d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::fastRemove(unsigned long)")]
// 0xf57d24 — j___ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE10fastRemoveEm
pub fn stub_0xf57d24() {
    // IDA 0xf57d24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::replace(unsigned long,RBX::PolyConnector * const&)")]
// 0xf57d34 — j___ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE7replaceEmRKS2_
pub fn stub_0xf57d34() {
    // IDA 0xf57d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::operator[](unsigned long)")]
// 0xf57d44 — j___ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm
pub fn stub_0xf57d44() {
    // IDA 0xf57d44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PolyConnector::match(RBX::PolyConnector*,RBX::PolyConnector*)")]
// 0xf57d54 — j___ZN3RBX13PolyConnector5matchEPS0_S1_
pub fn stub_0xf57d54() {
    // IDA 0xf57d54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::Allocator(void)")]
// 0xf57df4 — j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEC2Ev
pub fn stub_0xf57df4() {
    // IDA 0xf57df4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void *)")]
// 0xf57e04 — j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv
pub fn stub_0xf57e04() {
    // IDA 0xf57e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator new(unsigned long)")]
// 0xf57e14 — j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEnwEm
pub fn stub_0xf57e14() {
    // IDA 0xf57e14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf57e44 — j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf57e44() {
    // IDA 0xf57e44: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf57e54 — j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf57e54() {
    // IDA 0xf57e54: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul>::push_back(RBX::CellFaceFacePair::VertexStatus const&)")]
// 0xf57f84 — j___ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EE9push_backERKS2_
pub fn stub_0xf57f84() {
    // IDA 0xf57f84: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul>::operator[](unsigned long)")]
// 0xf57f94 — j___ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EEixEm
pub fn stub_0xf57f94() {
    // IDA 0xf57f94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::Allocator(void)")]
// 0xf57fc4 — j___ZN3RBX9AllocatorINS_15PolyCellContactEEC2Ev
pub fn stub_0xf57fc4() {
    // IDA 0xf57fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::Allocator(void)")]
// 0xf57fd4 — j___ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEC2Ev
pub fn stub_0xf57fd4() {
    // IDA 0xf57fd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::operator new(unsigned long)")]
// 0xf57fe4 — j___ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEnwEm
pub fn stub_0xf57fe4() {
    // IDA 0xf57fe4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::Allocator(void)")]
// 0xf57ff4 — j___ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEC2Ev
pub fn stub_0xf57ff4() {
    // IDA 0xf57ff4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::operator new(unsigned long)")]
// 0xf58004 — j___ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEnwEm
pub fn stub_0xf58004() {
    // IDA 0xf58004: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::Allocator(void)")]
// 0xf58014 — j___ZN3RBX9AllocatorINS_19FaceVertexConnectorEEC2Ev
pub fn stub_0xf58014() {
    // IDA 0xf58014: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::operator new(unsigned long)")]
// 0xf58024 — j___ZN3RBX9AllocatorINS_19FaceVertexConnectorEEnwEm
pub fn stub_0xf58024() {
    // IDA 0xf58024: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf58034 — j___ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf58034() {
    // IDA 0xf58034: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf58044 — j___ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf58044() {
    // IDA 0xf58044: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf58054 — j___ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf58054() {
    // IDA 0xf58054: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf58064 — j___ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf58064() {
    // IDA 0xf58064: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf58074 — j___ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf58074() {
    // IDA 0xf58074: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf58084 — j___ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf58084() {
    // IDA 0xf58084: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf58094 — j___ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf58094() {
    // IDA 0xf58094: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::getSidePlane(unsigned long)const")]
// 0xf580c4 — j___ZNK3RBX4POLY4Face12getSidePlaneEm
pub fn stub_0xf580c4() {
    // IDA 0xf580c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::PluginManager::~PluginManager()")]
// 0xf581b4 — j___ZN3RBX13PluginManagerD2Ev
pub fn stub_0xf581b4() {
    // IDA 0xf581b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Button>::operator=(rbx_core::SharedPtr<RBX::Button> const&)")]
// 0xf58314 — j___ZN5boost10shared_ptrIN3RBX6ButtonEEaSERKS3_
// was: boost::shared_ptr<RBX::Button>::operator=(boost::shared_ptr<RBX::Button> const&)
pub fn stub_0xf58314() {
    // IDA 0xf58314: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Toolbar>::operator=(rbx_core::SharedPtr<RBX::Toolbar> const&)")]
// 0xf58344 — j___ZN5boost10shared_ptrIN3RBX7ToolbarEEaSERKS3_
// was: boost::shared_ptr<RBX::Toolbar>::operator=(boost::shared_ptr<RBX::Toolbar> const&)
pub fn stub_0xf58344() {
    // IDA 0xf58344: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>*)")]
// 0xf583b4 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEE7destroyEPS8_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>::destroy(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>*)
pub fn stub_0xf583b4() {
    // IDA 0xf583b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_clear(void)")]
// 0xf58444 — j___ZNSt10_List_baseIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_clear(void)
pub fn stub_0xf58444() {
    // IDA 0xf58444: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<void *,rbx_core::SharedPtr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::operator[](void * const&)")]
// 0xf58454 — j___ZNSt3mapIPvN5boost10shared_ptrIN3RBX6ButtonEEESt4lessIS0_ESaISt4pairIKS0_S5_EEEixERS9_
// was: std::map<void *,boost::shared_ptr<RBX::Button>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::operator[](void * const&)
pub fn stub_0xf58454() {
    // IDA 0xf58454: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::operator[](std::string const&)")]
// 0xf58464 — j___ZNSt3mapISsN5boost10shared_ptrIN3RBX7ToolbarEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
// was: std::map<std::string,boost::shared_ptr<RBX::Toolbar>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::operator[](std::string const&)
pub fn stub_0xf58464() {
    // IDA 0xf58464: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_create_node(rbx_core::SharedPtr<RBX::Plugin> const&)")]
// 0xf58474 — j___ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_create_node(boost::shared_ptr<RBX::Plugin> const&)
pub fn stub_0xf58474() {
    // IDA 0xf58474: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>>(std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::_List_const_iterator<rbx_core::SharedPtr<RBX::Plugin>>,std::__false_type)")]
// 0xf58484 — j___ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EE22_M_initialize_dispatchISt20_List_const_iteratorIS4_EEEvT_SA_St12__false_type
// was: void std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>>(std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>,std::_List_const_iterator<boost::shared_ptr<RBX::Plugin>>,std::__false_type)
pub fn stub_0xf58484() {
    // IDA 0xf58484: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>>::list(std::list<rbx_core::SharedPtr<RBX::Plugin>,std::allocator<rbx_core::SharedPtr<RBX::Plugin>>> const&)")]
// 0xf58494 — j___ZNSt4listIN5boost10shared_ptrIN3RBX6PluginEEESaIS4_EEC2ERKS6_
// was: std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>>::list(std::list<boost::shared_ptr<RBX::Plugin>,std::allocator<boost::shared_ptr<RBX::Plugin>>> const&)
pub fn stub_0xf58494() {
    // IDA 0xf58494: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Toolbar> const&)")]
// 0xf584a4 — j___ZNSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEEC2ERS0_RKS5_
// was: std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>::pair(std::string const&,boost::shared_ptr<RBX::Toolbar> const&)
pub fn stub_0xf584a4() {
    // IDA 0xf584a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_create_node(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0xf58504 — j___ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE14_M_create_nodeERKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_create_node(std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_0xf58504() {
    // IDA 0xf58504: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)")]
// 0xf58514 — j___ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)
pub fn stub_0xf58514() {
    // IDA 0xf58514: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0xf58524 — j___ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert_unique(std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_0xf58524() {
    // IDA 0xf58524: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0xf58534 — j___ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_0xf58534() {
    // IDA 0xf58534: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>> *)")]
// 0xf58544 — j___ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_erase(std::_Rb_tree_node<std::pair<void * const,boost::shared_ptr<RBX::Button>>> *)
pub fn stub_0xf58544() {
    // IDA 0xf58544: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<void *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>,std::_Select1st<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,rbx_core::SharedPtr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,rbx_core::SharedPtr<RBX::Button>> const&)")]
// 0xf58554 — j___ZNSt8_Rb_treeIPvSt4pairIKS0_N5boost10shared_ptrIN3RBX6ButtonEEEESt10_Select1stIS8_ESt4lessIS0_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: std::_Rb_tree<void *,std::pair<void * const,boost::shared_ptr<RBX::Button>>,std::_Select1st<std::pair<void * const,boost::shared_ptr<RBX::Button>>>,std::less<void *>,std::allocator<std::pair<void * const,boost::shared_ptr<RBX::Button>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<void * const,boost::shared_ptr<RBX::Button>> const&)
pub fn stub_0xf58554() {
    // IDA 0xf58554: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::lower_bound(std::string const&)")]
// 0xf58564 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::lower_bound(std::string const&)
pub fn stub_0xf58564() {
    // IDA 0xf58564: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0xf58574 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE14_M_create_nodeERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_0xf58574() {
    // IDA 0xf58574: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0xf58584 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_0xf58584() {
    // IDA 0xf58584: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0xf58594 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_0xf58594() {
    // IDA 0xf58594: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::find(std::string const&)")]
// 0xf585a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE4findERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::find(std::string const&)
pub fn stub_0xf585a4() {
    // IDA 0xf585a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>*)")]
// 0xf585b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE7_M_copyEPKSt13_Rb_tree_nodeIS7_EPSF_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>> const*,std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>*)
pub fn stub_0xf585b4() {
    // IDA 0xf585b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>> *)")]
// 0xf585c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>> *)
pub fn stub_0xf585c4() {
    // IDA 0xf585c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>> const&)")]
// 0xf585d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>> const&)
pub fn stub_0xf585d4() {
    // IDA 0xf585d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Toolbar>>>> const&)")]
// 0xf585e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX7ToolbarEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EEC2ERKSD_
// was: std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>>::_Rb_tree(std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::Toolbar>>>> const&)
pub fn stub_0xf585e4() {
    // IDA 0xf585e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::push_back(RBX::PolyConnector * const&)")]
// 0xf58614 — j___ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE9push_backERKS2_
pub fn stub_0xf58614() {
    // IDA 0xf58614: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::Allocator(void)")]
// 0xf58624 — j___ZN3RBX9AllocatorINS_15BallCellContactEEC2Ev
pub fn stub_0xf58624() {
    // IDA 0xf58624: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::Allocator(void)")]
// 0xf58634 — j___ZN3RBX9AllocatorINS_17BallEdgeConnectorEEC2Ev
pub fn stub_0xf58634() {
    // IDA 0xf58634: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::operator new(unsigned long)")]
// 0xf58644 — j___ZN3RBX9AllocatorINS_17BallEdgeConnectorEEnwEm
pub fn stub_0xf58644() {
    // IDA 0xf58644: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::Allocator(void)")]
// 0xf58654 — j___ZN3RBX9AllocatorINS_18BallPlaneConnectorEEC2Ev
pub fn stub_0xf58654() {
    // IDA 0xf58654: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::operator new(unsigned long)")]
// 0xf58664 — j___ZN3RBX9AllocatorINS_18BallPlaneConnectorEEnwEm
pub fn stub_0xf58664() {
    // IDA 0xf58664: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::Allocator(void)")]
// 0xf58674 — j___ZN3RBX9AllocatorINS_19BallVertexConnectorEEC2Ev
pub fn stub_0xf58674() {
    // IDA 0xf58674: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::operator new(unsigned long)")]
// 0xf58684 — j___ZN3RBX9AllocatorINS_19BallVertexConnectorEEnwEm
pub fn stub_0xf58684() {
    // IDA 0xf58684: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf58694 — j___ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf58694() {
    // IDA 0xf58694: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf586a4 — j___ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf586a4() {
    // IDA 0xf586a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf586b4 — j___ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf586b4() {
    // IDA 0xf586b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallEdgeConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf586c4 — j___ZN5boost14singleton_poolIN3RBX17BallEdgeConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0xf586c4() {
    // IDA 0xf586c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf586d4 — j___ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0xf586d4() {
    // IDA 0xf586d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf586e4 — j___ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0xf586e4() {
    // IDA 0xf586e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf586f4 — j___ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf586f4() {
    // IDA 0xf586f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf58704 — j___ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf58704() {
    // IDA 0xf58704: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPlaneConnector,300u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf58714 — j___ZN5boost14singleton_poolIN3RBX18BallPlaneConnectorELj300ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0xf58714() {
    // IDA 0xf58714: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf58724 — j___ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0xf58724() {
    // IDA 0xf58724: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf58734 — j___ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf58734() {
    // IDA 0xf58734: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallVertexConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf58744 — j___ZN5boost14singleton_poolIN3RBX19BallVertexConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0xf58744() {
    // IDA 0xf58744: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf58754 — j___ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0xf58754() {
    // IDA 0xf58754: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Edge::computeNormal(RBX::POLY::Face const*)const")]
// 0xf58774 — j___ZNK3RBX4POLY4Edge13computeNormalEPKNS0_4FaceE
pub fn stub_0xf58774() {
    // IDA 0xf58774: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::plane(void)const")]
// 0xf58794 — j___ZNK3RBX4POLY4Face5planeEv
pub fn stub_0xf58794() {
    // IDA 0xf58794: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::PersonalServerService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xf58844 — j___ZN3RBX21PersonalServerService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_0xf58844() {
    // IDA 0xf58844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::PersonalServerService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xf58854 — j___ZN3RBX21PersonalServerService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_0xf58854() {
    // IDA 0xf58854: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_allocate(unsigned long)")]
// 0xf58864 — j___ZNSt12_Vector_baseIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE11_M_allocateEm
pub fn stub_0xf58864() {
    // IDA 0xf58864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::PrivilegeType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *>(RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *,RBX::PersonalServerService::PrivilegeType *)")]
// 0xf58874 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX21PersonalServerService13PrivilegeTypeES6_EET0_T_S8_S7_
pub fn stub_0xf58874() {
    // IDA 0xf58874: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::PersonalServerService::PrivilegeType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::operator[](RBX::Name const* const&)")]
// 0xf58884 — j___ZNSt3mapIPKN3RBX4NameENS0_21PersonalServerService13PrivilegeTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf58884() {
    // IDA 0xf58884: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PersonalServerService::PrivilegeType*,std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>>,RBX::PersonalServerService::PrivilegeType const&)")]
// 0xf58894 — j___ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf58894() {
    // IDA 0xf58894: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PersonalServerService::PrivilegeType*,std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>>,unsigned long,RBX::PersonalServerService::PrivilegeType const&)")]
// 0xf588a4 — j___ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf588a4() {
    // IDA 0xf588a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::resize(unsigned long,RBX::PersonalServerService::PrivilegeType)")]
// 0xf588b4 — j___ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE6resizeEmS2_
pub fn stub_0xf588b4() {
    // IDA 0xf588b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::PersonalServerService::PrivilegeType,std::allocator<RBX::PersonalServerService::PrivilegeType>>::push_back(RBX::PersonalServerService::PrivilegeType const&)")]
// 0xf588c4 — j___ZNSt6vectorIN3RBX21PersonalServerService13PrivilegeTypeESaIS2_EE9push_backERKS2_
pub fn stub_0xf588c4() {
    // IDA 0xf588c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType> const&)")]
// 0xf588d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf588d4() {
    // IDA 0xf588d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
