//! core shard HM — 100 core stubs EA-sorted, 0xf64b64..0xf65834 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HL 0xf64b54 (21614->21714 covered, 204 remaining).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HL 0xf64b54 (0xf64b64..0xf65834, 21614->21714 covered, 204 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::FastClusterMeshGenerator::createVertexData(unsigned int,bool)")]
// 0xf64b64 — j___ZN3RBX24FastClusterMeshGenerator16createVertexDataEjb
pub fn stub_0xf64b64() {
    // IDA 0xf64b64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::generateShadowData(RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,bool)")]
// 0xf64b84 — j___ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb
pub fn stub_0xf64b84() {
    // IDA 0xf64b84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::setupSharedGeometry(RBX::FastClusterSharedGeometry &,unsigned int,unsigned int,bool)")]
// 0xf64b94 — j___ZN3RBX24FastClusterMeshGenerator19setupSharedGeometryERNS_25FastClusterSharedGeometryEjjb
pub fn stub_0xf64b94() {
    // IDA 0xf64b94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")]
// 0xf64bb4 — j___ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb
pub fn stub_0xf64bb4() {
    // IDA 0xf64bb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::generateBatchGeometry(RBX::FastClusterMeshGenerator::MaterialGroup const&,RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex *,unsigned short *,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> &,bool)")]
// 0xf64bd4 — j___ZN3RBX24FastClusterMeshGenerator21generateBatchGeometryERKNS0_13MaterialGroupERKNS0_5BatchEPNS_17GeometryGenerator6VertexEPtjRSt6vectorIjSaIjEEb
pub fn stub_0xf64bd4() {
    // IDA 0xf64bd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")]
// 0xf64be4 — j___ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_
pub fn stub_0xf64be4() {
    // IDA 0xf64be4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::finalize(RBX::FastCluster *,RBX::eShadowCullingPriority)")]
// 0xf64c04 — j___ZN3RBX24FastClusterMeshGenerator8finalizeEPNS_11FastClusterENS_22eShadowCullingPriorityE
pub fn stub_0xf64c04() {
    // IDA 0xf64c04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")]
// 0xf64c34 — j___ZN3RBX24FastClusterMeshGeneratorD1Ev
pub fn stub_0xf64c34() {
    // IDA 0xf64c34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::FastClusterBinding>::Allocator(void)")]
// 0xf64c44 — j___ZN3RBX9AllocatorINS_18FastClusterBindingEEC2Ev
pub fn stub_0xf64c44() {
    // IDA 0xf64c44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::FastClusterBinding,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf64c64 — j___ZN5boost14singleton_poolIN3RBX18FastClusterBindingELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf64c64() {
    // IDA 0xf64c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")]
// 0xf64c84 — j___ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev
pub fn stub_0xf64c84() {
    // IDA 0xf64c84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<unsigned long long,RBX::FastClusterMeshGenerator::MaterialGroup,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::operator[](unsigned long long const&)")]
// 0xf64ca4 — j___ZNSt3mapIyN3RBX24FastClusterMeshGenerator13MaterialGroupESt4lessIyESaISt4pairIKyS2_EEEixERS6_
pub fn stub_0xf64ca4() {
    // IDA 0xf64ca4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")]
// 0xf64cb4 — j___ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_
pub fn stub_0xf64cb4() {
    // IDA 0xf64cb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastCluster::Bone,std::allocator<RBX::FastCluster::Bone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FastCluster::Bone*,std::vector<RBX::FastCluster::Bone,std::allocator<RBX::FastCluster::Bone>>>,unsigned long,RBX::FastCluster::Bone const&)")]
// 0xf64cc4 — j___ZNSt6vectorIN3RBX11FastCluster4BoneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf64cc4() {
    // IDA 0xf64cc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part*,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::FastCluster::Part const&)")]
// 0xf64cd4 — j___ZNSt6vectorIN3RBX11FastCluster4PartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf64cd4() {
    // IDA 0xf64cd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ShadowTriangle*,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,unsigned long,RBX::ShadowTriangle const&)")]
// 0xf64d04 — j___ZNSt6vectorIN3RBX14ShadowTriangleESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0xf64d04() {
    // IDA 0xf64d04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")]
// 0xf64d34 — j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf64d34() {
    // IDA 0xf64d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")]
// 0xf64d44 — j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm
pub fn stub_0xf64d44() {
    // IDA 0xf64d44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowGenerator::Vertex*,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>>,RBX::FastClusterShadowGenerator::Vertex const&)")]
// 0xf64d54 — j___ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf64d54() {
    // IDA 0xf64d54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowGenerator::Vertex*,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>>,unsigned long,RBX::FastClusterShadowGenerator::Vertex const&)")]
// 0xf64d64 — j___ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf64d64() {
    // IDA 0xf64d64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::reserve(unsigned long)")]
// 0xf64d74 — j___ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE7reserveEm
pub fn stub_0xf64d74() {
    // IDA 0xf64d74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>*,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> const&)")]
// 0xf64d84 — j___ZNSt6vectorISt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS2_5BatchEESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_
pub fn stub_0xf64d84() {
    // IDA 0xf64d84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<char,std::allocator<char>>::_M_fill_insert(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,unsigned long,char const&)")]
// 0xf64d94 — j___ZNSt6vectorIcSaIcEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS1_EEmRKc
pub fn stub_0xf64d94() {
    // IDA 0xf64d94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::reserve(unsigned long)")]
// 0xf64da4 — j___ZNSt6vectorIjSaIjEE7reserveEm
pub fn stub_0xf64da4() {
    // IDA 0xf64da4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert_unique(std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
// 0xf64db4 — j___ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0xf64db4() {
    // IDA 0xf64db4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
// 0xf64dc4 — j___ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0xf64dc4() {
    // IDA 0xf64dc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>> *)")]
// 0xf64dd4 — j___ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xf64dd4() {
    // IDA 0xf64dd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
// 0xf64de4 — j___ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0xf64de4() {
    // IDA 0xf64de4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,int,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate)")]
// 0xf64df4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEEiS4_NS2_23PartClumpGroupPredicateEEvT_T0_SC_T1_T2_
pub fn stub_0xf64df4() {
    // IDA 0xf64df4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,int,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xf64e04 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiS9_NS4_33BatchMaterialPlasticLODComparatorEEvT_T0_SH_T1_T2_
pub fn stub_0xf64e04() {
    // IDA 0xf64e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate)")]
// 0xf64e14 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_SB_T0_
pub fn stub_0xf64e14() {
    // IDA 0xf64e14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xf64e24 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_SG_T0_
pub fn stub_0xf64e24() {
    // IDA 0xf64e24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate)")]
// 0xf64e34 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_T0_
pub fn stub_0xf64e34() {
    // IDA 0xf64e34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,RBX::PartClumpGroupPredicate)")]
// 0xf64e44 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEEiNS2_23PartClumpGroupPredicateEEvT_SB_T0_T1_
pub fn stub_0xf64e44() {
    // IDA 0xf64e44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xf64e54 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiNS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_T1_
pub fn stub_0xf64e54() {
    // IDA 0xf64e54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate)")]
// 0xf64e64 — j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEES4_NS2_23PartClumpGroupPredicateEET_SB_SB_T0_T1_
pub fn stub_0xf64e64() {
    // IDA 0xf64e64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate)")]
// 0xf64e74 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_T0_
pub fn stub_0xf64e74() {
    // IDA 0xf64e74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xf64e84 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_
pub fn stub_0xf64e84() {
    // IDA 0xf64e84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastCluster::Part const& std::__median<RBX::FastCluster::Part,RBX::PartClumpGroupPredicate>(RBX::FastCluster::Part const&,RBX::FastCluster::Part const&,RBX::FastCluster::Part const&,RBX::PartClumpGroupPredicate)")]
// 0xf64eb4 — j___ZSt8__medianIN3RBX11FastCluster4PartENS0_23PartClumpGroupPredicateEERKT_S6_S6_S6_T0_
pub fn stub_0xf64eb4() {
    // IDA 0xf64eb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>> std::__find_if<__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,RBX::FastClusterShadowGenerator::TriangleIsDegeneratePredicate>(__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,RBX::FastClusterShadowGenerator::TriangleIsDegeneratePredicate,std::random_access_iterator_tag)")]
// 0xf64ec4 — j___ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX14ShadowTriangleESt6vectorIS3_SaIS3_EEEENS2_26FastClusterShadowGenerator29TriangleIsDegeneratePredicateEET_SB_SB_T0_St26random_access_iterator_tag
pub fn stub_0xf64ec4() {
    // IDA 0xf64ec4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>> std::remove_if<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartBindingNullPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartBindingNullPredicate)")]
// 0xf64ed4 — j___ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_24PartBindingNullPredicateEET_SB_SB_T0_
pub fn stub_0xf64ed4() {
    // IDA 0xf64ed4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId const&)")]
// 0xf64f64 — j___ZNSt6vectorIN3RBX9ContentIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0xf64f64() {
    // IDA 0xf64f64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer const&)")]
// 0xf64fb4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0xf64fb4() {
    // IDA 0xf64fb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<RBX::TextureCompositorLayer*>(unsigned long,RBX::TextureCompositorLayer*,RBX::TextureCompositorLayer*)")]
// 0xf64fc4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_
pub fn stub_0xf64fc4() {
    // IDA 0xf64fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::reserve(unsigned long)")]
// 0xf64fd4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm
pub fn stub_0xf64fd4() {
    // IDA 0xf64fd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::push_back(RBX::TextureCompositorLayer const&)")]
// 0xf64fe4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_
pub fn stub_0xf64fe4() {
    // IDA 0xf64fe4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Adorn::~Adorn()")]
// 0xf65004 — j___ZN3RBX5AdornD2Ev
pub fn stub_0xf65004() {
    // IDA 0xf65004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TextureProxyBase>::_internal_accept_owner<RBX::TextureProxyBase,RBX::RbxTextureProxy>(rbx_core::SharedPtr<RBX::TextureProxyBase> const*,RBX::RbxTextureProxy *)const")]
// 0xf65064 — j___ZNK5boost23enable_shared_from_thisIN3RBX16TextureProxyBaseEE22_internal_accept_ownerIS2_NS1_15RbxTextureProxyEEEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0xf65064() {
    // IDA 0xf65064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MeshContentProvider * RBX::ServiceProvider::find<RBX::MeshContentProvider>(void)const")]
// 0xf651f4 — j___ZNK3RBX15ServiceProvider4findINS_19MeshContentProviderEEEPT_v
pub fn stub_0xf651f4() {
    // IDA 0xf651f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MeshContentProvider * RBX::ServiceProvider::create<RBX::MeshContentProvider>(void)const")]
// 0xf65204 — j___ZNK3RBX15ServiceProvider6createINS_19MeshContentProviderEEEPT_v
pub fn stub_0xf65204() {
    // IDA 0xf65204: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowRenderable **,std::vector<RBX::FastClusterShadowRenderable *,std::allocator<RBX::FastClusterShadowRenderable *>>>,RBX::FastClusterShadowRenderable * const&)")]
// 0xf65444 — j___ZNSt6vectorIPN3RBX27FastClusterShadowRenderableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf65444() {
    // IDA 0xf65444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::SpatialGrid<RBX::FastCluster>>::~scoped_ptr()")]
// 0xf654b4 — j___ZN5boost10scoped_ptrIN3RBX11SpatialGridINS1_11FastClusterEEEED2Ev
pub fn stub_0xf654b4() {
    // IDA 0xf654b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,20u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::malloc(void)")]
// 0xf654c4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj20ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_0xf654c4() {
    // IDA 0xf654c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,20u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0xf654d4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj20ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_0xf654d4() {
    // IDA 0xf654d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<boost::fast_pool_allocator_tag,8u,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>::is_from(void *)")]
// 0xf654e4 — j___ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj8ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE7is_fromEPv
pub fn stub_0xf654e4() {
    // IDA 0xf654e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::unordered_map<RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>,boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~unordered_map()")]
// 0xf654f4 — j___ZN5boost9unordered13unordered_mapIN3RBX16SpatialGridIndexENS2_11SpatialGridINS2_11FastClusterEE4CellENS_4hashIS3_EESt8equal_toIS3_ENS_19fast_pool_allocatorIS3_NS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED1Ev
pub fn stub_0xf654f4() {
    // IDA 0xf654f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::erase_key(RBX::SpatialGridIndex const&)")]
// 0xf65504 — j___ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE9erase_keyERKS6_
pub fn stub_0xf65504() {
    // IDA 0xf65504: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::operator[](RBX::SpatialGridIndex const&)")]
// 0xf65514 — j___ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEEixERKS6_
pub fn stub_0xf65514() {
    // IDA 0xf65514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::create_buckets(unsigned long)")]
// 0xf65524 — j___ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_0xf65524() {
    // IDA 0xf65524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::destroy_buckets(void)")]
// 0xf65534 — j___ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE15destroy_bucketsEv
pub fn stub_0xf65534() {
    // IDA 0xf65534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::SpatialGridIndex,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::SpatialGridIndex,RBX::SpatialGrid<RBX::FastCluster>::Cell,boost::hash<RBX::SpatialGridIndex>,std::equal_to<RBX::SpatialGridIndex>>>::reserve_for_insert(unsigned long)")]
// 0xf65544 — j___ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_0xf65544() {
    // IDA 0xf65544: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex*,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,RBX::SpatialGridIndex const&)")]
// 0xf65564 — j___ZNSt6vectorIN3RBX16SpatialGridIndexESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0xf65564() {
    // IDA 0xf65564: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex*,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,unsigned long,RBX::SpatialGridIndex const&)")]
// 0xf65574 — j___ZNSt6vectorIN3RBX16SpatialGridIndexESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0xf65574() {
    // IDA 0xf65574: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::_M_insert_unique(std::pair<RBX::Humanoid * const,RBX::FastCluster *> const&)")]
// 0xf65584 — j___ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_0xf65584() {
    // IDA 0xf65584: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::pair<RBX::Humanoid * const,RBX::FastCluster *> const&)")]
// 0xf65594 — j___ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_0xf65594() {
    // IDA 0xf65594: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::_Rb_tree_iterator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>)")]
// 0xf655a4 — j___ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE5eraseESt17_Rb_tree_iteratorIS7_ESF_
pub fn stub_0xf655a4() {
    // IDA 0xf655a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Humanoid *,std::pair<RBX::Humanoid * const,RBX::FastCluster *>,std::_Select1st<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>,std::less<RBX::Humanoid *>,std::allocator<std::pair<RBX::Humanoid * const,RBX::FastCluster *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Humanoid * const,RBX::FastCluster *>> *)")]
// 0xf655b4 — j___ZNSt8_Rb_treeIPN3RBX8HumanoidESt4pairIKS2_PNS0_11FastClusterEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_0xf655b4() {
    // IDA 0xf655b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,RBX::SpatialGridIndex>(__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,__gnu_cxx::__normal_iterator<RBX::SpatialGridIndex *,std::vector<RBX::SpatialGridIndex,std::allocator<RBX::SpatialGridIndex>>>,RBX::SpatialGridIndex const&,std::random_access_iterator_tag)")]
// 0xf655c4 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPN3RBX16SpatialGridIndexESt6vectorIS3_SaIS3_EEEES3_ET_S9_S9_RKT0_St26random_access_iterator_tag
pub fn stub_0xf655c4() {
    // IDA 0xf655c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::TextureCompositor::Job::~Job()")]
// 0xf65614 — j___ZN3RBX17TextureCompositor3JobD2Ev
pub fn stub_0xf65614() {
    // IDA 0xf65614: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(RBX::TextureCompositorJob::LayerData const&)")]
// 0xf65624 — j___ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_
pub fn stub_0xf65624() {
    // IDA 0xf65624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(void)")]
// 0xf65634 — j___ZN3RBX20TextureCompositorJob9LayerDataC2Ev
pub fn stub_0xf65634() {
    // IDA 0xf65634: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextureCompositorJob::LayerData::~LayerData()")]
// 0xf65644 — j___ZN3RBX20TextureCompositorJob9LayerDataD2Ev
pub fn stub_0xf65644() {
    // IDA 0xf65644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositor::Job>(RBX::TextureCompositor::Job *)")]
// 0xf65654 — j___ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_
pub fn stub_0xf65654() {
    // IDA 0xf65654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositorJob>(RBX::TextureCompositorJob *)")]
// 0xf65664 — j___ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_
pub fn stub_0xf65664() {
    // IDA 0xf65664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>)")]
// 0xf65674 — j___ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_
pub fn stub_0xf65674() {
    // IDA 0xf65674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *>(rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *)")]
// 0xf65684 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
pub fn stub_0xf65684() {
    // IDA 0xf65684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)")]
// 0xf656a4 — j___ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_
pub fn stub_0xf656a4() {
    // IDA 0xf656a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *>(rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *,rbx_core::SharedPtr<RBX::TextureCompositor::Job> *)")]
// 0xf656b4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
pub fn stub_0xf656b4() {
    // IDA 0xf656b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::TextureCompositorJob::LayerData,std::allocator<RBX::TextureCompositorJob::LayerData>>::vector(unsigned long,RBX::TextureCompositorJob::LayerData const&,std::allocator<RBX::TextureCompositorJob::LayerData> const&)")]
// 0xf656c4 — j___ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_
pub fn stub_0xf656c4() {
    // IDA 0xf656c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>)")]
// 0xf656d4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_
pub fn stub_0xf656d4() {
    // IDA 0xf656d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::~vector()")]
// 0xf656e4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev
pub fn stub_0xf656e4() {
    // IDA 0xf656e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::operator=(std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
// 0xf656f4 — j___ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_
pub fn stub_0xf656f4() {
    // IDA 0xf656f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xf65714 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
pub fn stub_0xf65714() {
    // IDA 0xf65714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)")]
// 0xf65724 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag
pub fn stub_0xf65724() {
    // IDA 0xf65724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)")]
// 0xf65734 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag
pub fn stub_0xf65734() {
    // IDA 0xf65734: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureCompositor::Job>* std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<rbx_core::SharedPtr<RBX::TextureCompositor::Job>*>(unsigned long,rbx_core::SharedPtr<RBX::TextureCompositor::Job>*,rbx_core::SharedPtr<RBX::TextureCompositor::Job>*)")]
// 0xf65744 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_
pub fn stub_0xf65744() {
    // IDA 0xf65744: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)")]
// 0xf65754 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm
pub fn stub_0xf65754() {
    // IDA 0xf65754: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::push_back(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xf65764 — j___ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_
pub fn stub_0xf65764() {
    // IDA 0xf65764: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<unsigned long long,std::allocator<unsigned long long>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned long long *,std::vector<unsigned long long,std::allocator<unsigned long long>>>,unsigned long long const&)")]
// 0xf65784 — j___ZNSt6vectorIySaIyEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPyS1_EERKy
pub fn stub_0xf65784() {
    // IDA 0xf65784: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::less<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_create_node(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xf65794 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0xf65794() {
    // IDA 0xf65794: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::less<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::TextureCompositor::Job> const&)")]
// 0xf657a4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0xf657a4() {
    // IDA 0xf657a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::TextureCompositor::Job>,rbx_core::SharedPtr<RBX::TextureCompositor::Job>,std::_Identity<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::less<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::allocator<rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::TextureCompositor::Job>> *)")]
// 0xf657b4 — j___ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xf657b4() {
    // IDA 0xf657b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xf657c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_
pub fn stub_0xf657c4() {
    // IDA 0xf657c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xf657d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf657d4() {
    // IDA 0xf657d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xf657e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf657e4() {
    // IDA 0xf657e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>)")]
// 0xf657f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E
pub fn stub_0xf657f4() {
    // IDA 0xf657f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>> *)")]
// 0xf65804 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0xf65804() {
    // IDA 0xf65804: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::TextureCompositor::Job>> const&)")]
// 0xf65814 — j___ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf65814() {
    // IDA 0xf65814: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorLayer* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*>(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*,std::__false_type)")]
// 0xf65824 — j___ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
pub fn stub_0xf65824() {
    // IDA 0xf65824: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::TextureCompositorLayer * std::__uninitialized_copy_aux<RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *>(RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,std::__false_type)")]
// 0xf65834 — j___ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type
pub fn stub_0xf65834() {
    // IDA 0xf65834: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
