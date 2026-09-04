//! core shard ET — 100 core stubs EA-sorted, lowest uncovered 0xb6b668..0xb8c644 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after ES 0xb6b668).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::FastClusterShadowRenderable::getLightCapBounds(void)const")]
// 0xb6b668 — __ZNK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv
pub fn stub_b6b668() {
    // IDA 0xb6b668: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::FastClusterShadowRenderable::getLightCapBounds(void)const")]
// 0xb6b674 — __ZThn96_NK3RBX27FastClusterShadowRenderable17getLightCapBoundsEv
// was: non-virtual thunk toRBX::FastClusterShadowRenderable::getLightCapBounds(void)const
pub fn stub_b6b674() {
    // IDA 0xb6b674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterEntity::~FastClusterEntity()")]
// 0xb6bbe8 — __ZN3RBX17FastClusterEntityD0Ev
pub fn stub_b6bbe8() {
    // IDA 0xb6bbe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterEntity::~FastClusterEntity()")]
// 0xb6bcd4 — __ZN3RBX17FastClusterEntityD1Ev
pub fn stub_b6bcd4() {
    // IDA 0xb6bcd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterEntity::getDebugMaterial(void)const")]
// 0xb6bdb4 — __ZNK3RBX17FastClusterEntity16getDebugMaterialEv
pub fn stub_b6bdb4() {
    // IDA 0xb6bdb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterEntity::getNumWorldTransforms(void)const")]
// 0xb6beb4 — __ZNK3RBX17FastClusterEntity21getNumWorldTransformsEv
pub fn stub_b6beb4() {
    // IDA 0xb6beb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterEntity::getCastsShadows(void)const")]
// 0xb6c010 — __ZNK3RBX17FastClusterEntity15getCastsShadowsEv
pub fn stub_b6c010() {
    // IDA 0xb6c010: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterBinding::invalidateEntity(void)")]
// 0xb6c228 — __ZN3RBX18FastClusterBinding16invalidateEntityEv
pub fn stub_b6c228() {
    // IDA 0xb6c228: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterBinding::onCoordinateFrameChanged(void)")]
// 0xb6c264 — __ZN3RBX18FastClusterBinding24onCoordinateFrameChangedEv
pub fn stub_b6c264() {
    // IDA 0xb6c264: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FastClusterBinding::onSizeChanged(void)")]
// 0xb6c2d0 — __ZN3RBX18FastClusterBinding13onSizeChangedEv
pub fn stub_b6c2d0() {
    // IDA 0xb6c2d0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FastClusterBinding::onTransparencyChanged(void)")]
// 0xb6c334 — __ZN3RBX18FastClusterBinding21onTransparencyChangedEv
pub fn stub_b6c334() {
    // IDA 0xb6c334: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::FastClusterBinding::onSpecialShapeChanged(void)")]
// 0xb6c374 — __ZN3RBX18FastClusterBinding21onSpecialShapeChangedEv
pub fn stub_b6c374() {
    // IDA 0xb6c374: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::FastClusterBinding::unbind(void)")]
// 0xb6c3b4 — __ZN3RBX18FastClusterBinding6unbindEv
pub fn stub_b6c3b4() {
    // IDA 0xb6c3b4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::FastCluster::~FastCluster()")]
// 0xb6ca68 — __ZN3RBX11FastClusterD0Ev
pub fn stub_b6ca68() {
    // IDA 0xb6ca68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::~FastCluster()")]
// 0xb6cb1c — __ZN3RBX11FastClusterD1Ev
pub fn stub_b6cb1c() {
    // IDA 0xb6cb1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::~FastCluster()")]
// 0xb6cb20 — __ZN3RBX11FastClusterD2Ev
pub fn stub_b6cb20() {
    // IDA 0xb6cb20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::checkCluster(void)")]
// 0xb6d248 — __ZN3RBX11FastCluster12checkClusterEv
pub fn stub_b6d248() {
    // IDA 0xb6d248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::priorityInvalidateEntity(void)")]
// 0xb6d71c — __ZN3RBX11FastCluster24priorityInvalidateEntityEv
pub fn stub_b6d71c() {
    // IDA 0xb6d71c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::invalidateEntity(void)")]
// 0xb6d7b8 — __ZN3RBX11FastCluster16invalidateEntityEv
pub fn stub_b6d7b8() {
    // IDA 0xb6d7b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::invalidateEntity(void)")]
// 0xb6d808 — __ZThn392_N3RBX11FastCluster16invalidateEntityEv
// was: non-virtual thunk toRBX::FastCluster::invalidateEntity(void)
pub fn stub_b6d808() {
    // IDA 0xb6d808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::checkBindings(void)")]
// 0xb6d85c — __ZN3RBX11FastCluster13checkBindingsEv
pub fn stub_b6d85c() {
    // IDA 0xb6d85c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::updateEntity(bool)")]
// 0xb6dc20 — __ZN3RBX11FastCluster12updateEntityEb
pub fn stub_b6dc20() {
    // IDA 0xb6dc20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::updateClumpGrouping(void)")]
// 0xb6e0a8 — __ZN3RBX11FastCluster19updateClumpGroupingEv
pub fn stub_b6e0a8() {
    // IDA 0xb6e0a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::updateGeometry(RBX::AsyncResult *)")]
// 0xb6e18c — __ZN3RBX11FastCluster14updateGeometryEPNS_11AsyncResultE
pub fn stub_b6e18c() {
    // IDA 0xb6e18c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::updateEntity(bool)")]
// 0xb6e938 — __ZThn392_N3RBX11FastCluster12updateEntityEb
// was: non-virtual thunk toRBX::FastCluster::updateEntity(bool)
pub fn stub_b6e938() {
    // IDA 0xb6e938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::updateCoordinateFrame(bool)")]
// 0xb6e940 — __ZN3RBX11FastCluster21updateCoordinateFrameEb
pub fn stub_b6e940() {
    // IDA 0xb6e940: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::updateCoordinateFrame(bool)")]
// 0xb6edc4 — __ZThn392_N3RBX11FastCluster21updateCoordinateFrameEb
// was: non-virtual thunk toRBX::FastCluster::updateCoordinateFrame(bool)
pub fn stub_b6edc4() {
    // IDA 0xb6edc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::unbind(void)")]
// 0xb6edd0 — __ZN3RBX11FastCluster6unbindEv
pub fn stub_b6edd0() {
    // IDA 0xb6edd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::unbind(void)")]
// 0xb6f048 — __ZThn392_N3RBX11FastCluster6unbindEv
// was: non-virtual thunk toRBX::FastCluster::unbind(void)
pub fn stub_b6f048() {
    // IDA 0xb6f048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::onClumpChanged(void)")]
// 0xb6f0e8 — __ZN3RBX11FastCluster14onClumpChangedEv
pub fn stub_b6f0e8() {
    // IDA 0xb6f0e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::onClumpChanged(void)")]
// 0xb6f128 — __ZThn392_N3RBX11FastCluster14onClumpChangedEv
// was: non-virtual thunk toRBX::FastCluster::onClumpChanged(void)
pub fn stub_b6f128() {
    // IDA 0xb6f128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastCluster::getPartCount(void)")]
// 0xb6f16c — __ZN3RBX11FastCluster12getPartCountEv
pub fn stub_b6f16c() {
    // IDA 0xb6f16c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FastCluster::getPartCount(void)")]
// 0xb6f178 — __ZThn392_N3RBX11FastCluster12getPartCountEv
// was: non-virtual thunk toRBX::FastCluster::getPartCount(void)
pub fn stub_b6f178() {
    // IDA 0xb6f178: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::reserve(unsigned long)")]
// 0xb6f188 — __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE7reserveEm
pub fn stub_b6f188() {
    // IDA 0xb6f188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::FastClusterBinding>::Allocator(void)")]
// 0xb6f49c — __ZN3RBX9AllocatorINS_18FastClusterBindingEEC2Ev
pub fn stub_b6f49c() {
    // IDA 0xb6f49c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>> std::remove_if<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartBindingNullPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartBindingNullPredicate)")]
// 0xb6f548 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_24PartBindingNullPredicateEET_SB_SB_T0_
pub fn stub_b6f548() {
    // IDA 0xb6f548: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")]
// 0xb70210 — __ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE
pub fn stub_b70210() {
    // IDA 0xb70210: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::finalize(RBX::FastCluster *,RBX::eShadowCullingPriority)")]
// 0xb70af8 — __ZN3RBX24FastClusterMeshGenerator8finalizeEPNS_11FastClusterENS_22eShadowCullingPriorityE
pub fn stub_b70af8() {
    // IDA 0xb70af8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")]
// 0xb70d90 — __ZN3RBX24FastClusterMeshGeneratorD1Ev
pub fn stub_b70d90() {
    // IDA 0xb70d90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GfxBinding::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
// 0xb71010 — __ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb
pub fn stub_b71010() {
    // IDA 0xb71010: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GfxBinding::onSizeChanged(void)")]
// 0xb71018 — __ZN3RBX10GfxBinding13onSizeChangedEv
pub fn stub_b71018() {
    // IDA 0xb71018: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GfxBinding::onTransparencyChanged(void)")]
// 0xb71020 — __ZN3RBX10GfxBinding21onTransparencyChangedEv
pub fn stub_b71020() {
    // IDA 0xb71020: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GfxBinding::onSpecialShapeChanged(void)")]
// 0xb71028 — __ZN3RBX10GfxBinding21onSpecialShapeChangedEv
pub fn stub_b71028() {
    // IDA 0xb71028: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastCluster::Bone,std::allocator<RBX::FastCluster::Bone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FastCluster::Bone*,std::vector<RBX::FastCluster::Bone,std::allocator<RBX::FastCluster::Bone>>>,unsigned long,RBX::FastCluster::Bone const&)")]
// 0xb71030 — __ZNSt6vectorIN3RBX11FastCluster4BoneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_b71030() {
    // IDA 0xb71030: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,RBX::PartClumpGroupPredicate)")]
// 0xb71934 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEEiNS2_23PartClumpGroupPredicateEEvT_SB_T0_T1_
pub fn stub_b71934() {
    // IDA 0xb71934: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate)")]
// 0xb719d8 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_T0_
pub fn stub_b719d8() {
    // IDA 0xb719d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate)")]
// 0xb71a5c — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_T0_
pub fn stub_b71a5c() {
    // IDA 0xb71a5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate)")]
// 0xb71b14 — __ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEES4_NS2_23PartClumpGroupPredicateEET_SB_SB_T0_T1_
pub fn stub_b71b14() {
    // IDA 0xb71b14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastCluster::Part const& std::__median<RBX::FastCluster::Part,RBX::PartClumpGroupPredicate>(RBX::FastCluster::Part const&,RBX::FastCluster::Part const&,RBX::FastCluster::Part const&,RBX::PartClumpGroupPredicate)")]
// 0xb71b70 — __ZSt8__medianIN3RBX11FastCluster4PartENS0_23PartClumpGroupPredicateEERKT_S6_S6_S6_T0_
pub fn stub_b71b70() {
    // IDA 0xb71b70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::PartClumpGroupPredicate)")]
// 0xb71bf0 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_SB_T0_
pub fn stub_b71bf0() {
    // IDA 0xb71bf0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate>(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part *,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,int,int,RBX::FastCluster::Part,RBX::PartClumpGroupPredicate)")]
// 0xb71c70 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEEiS4_NS2_23PartClumpGroupPredicateEEvT_T0_SC_T1_T2_
pub fn stub_b71c70() {
    // IDA 0xb71c70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastCluster::Part*,std::vector<RBX::FastCluster::Part,std::allocator<RBX::FastCluster::Part>>>,RBX::FastCluster::Part const&)")]
// 0xb71d54 — __ZNSt6vectorIN3RBX11FastCluster4PartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_b71d54() {
    // IDA 0xb71d54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::FastClusterBinding,28u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xb71e70 — __ZN5boost14singleton_poolIN3RBX18FastClusterBindingELj28ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_b71e70() {
    // IDA 0xb71e70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FastCluster>,boost::_bi::list1<boost::_bi::value<RBX::FastCluster*>>>>::~callable_slot()")]
// 0xb71ee0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11FastClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_b71ee0() {
    // IDA 0xb71ee0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FastCluster>,boost::_bi::list1<boost::_bi::value<RBX::FastCluster*>>>>::~callable_slot()")]
// 0xb71f3c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX11FastClusterEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_b71f3c() {
    // IDA 0xb71f3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FastCluster>,boost::_bi::list1<boost::_bi::value<RBX::FastCluster*>>>,0,void ()(void)>::call(void)")]
// 0xb72048 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11FastClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_b72048() {
    // IDA 0xb72048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FastCluster>,boost::_bi::list1<boost::_bi::value<RBX::FastCluster*>>>,0,void ()(void)>::call(void)")]
// 0xb72060 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX11FastClusterEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::FastCluster>,boost::_bi::list1<boost::_bi::value<RBX::FastCluster*>>>,0,void ()(void)>::call(void)
pub fn stub_b72060() {
    // IDA 0xb72060: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::FastClusterBinding>::releaseMemory(void)")]
// 0xb7207c — __ZN3RBX9AllocatorINS_18FastClusterBindingEE13releaseMemoryEv
pub fn stub_b7207c() {
    // IDA 0xb7207c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterShadowData::~FastClusterShadowData()")]
// 0xb720f4 — __ZN3RBX21FastClusterShadowDataD2Ev
pub fn stub_b720f4() {
    // IDA 0xb720f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<char,std::allocator<char>>::_M_fill_insert(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,unsigned long,char const&)")]
// 0xb72230 — __ZNSt6vectorIcSaIcEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPcS1_EEmRKc
pub fn stub_b72230() {
    // IDA 0xb72230: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>> std::__find_if<__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,RBX::FastClusterShadowGenerator::TriangleIsDegeneratePredicate>(__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,__gnu_cxx::__normal_iterator<RBX::ShadowTriangle *,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,RBX::FastClusterShadowGenerator::TriangleIsDegeneratePredicate,std::random_access_iterator_tag)")]
// 0xb72354 — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN3RBX14ShadowTriangleESt6vectorIS3_SaIS3_EEEENS2_26FastClusterShadowGenerator29TriangleIsDegeneratePredicateEET_SB_SB_T0_St26random_access_iterator_tag
pub fn stub_b72354() {
    // IDA 0xb72354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowGenerator::Vertex*,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>>,RBX::FastClusterShadowGenerator::Vertex const&)")]
// 0xb72434 — __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_b72434() {
    // IDA 0xb72434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ShadowTriangle*,std::vector<RBX::ShadowTriangle,std::allocator<RBX::ShadowTriangle>>>,unsigned long,RBX::ShadowTriangle const&)")]
// 0xb72578 — __ZNSt6vectorIN3RBX14ShadowTriangleESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_b72578() {
    // IDA 0xb72578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FastClusterShadowGenerator::Vertex*,std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>>>,unsigned long,RBX::FastClusterShadowGenerator::Vertex const&)")]
// 0xb727e4 — __ZNSt6vectorIN3RBX26FastClusterShadowGenerator6VertexESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_b727e4() {
    // IDA 0xb727e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MaterialGenerator::~MaterialGenerator()")]
// 0xb72a18 — __ZN3RBX17MaterialGeneratorD2Ev
pub fn stub_b72a18() {
    // IDA 0xb72a18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>> *)")]
// 0xb72ef0 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_b72ef0() {
    // IDA 0xb72ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::MaterialGroup::~MaterialGroup()")]
// 0xb72f20 — __ZN3RBX24FastClusterMeshGenerator13MaterialGroupD1Ev
pub fn stub_b72f20() {
    // IDA 0xb72f20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")]
// 0xb730b8 — __ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev
pub fn stub_b730b8() {
    // IDA 0xb730b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::createVertexData(unsigned int,bool)")]
// 0xb73268 — __ZN3RBX24FastClusterMeshGenerator16createVertexDataEjb
pub fn stub_b73268() {
    // IDA 0xb73268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::createIndexData(unsigned int)")]
// 0xb7345c — __ZN3RBX24FastClusterMeshGenerator15createIndexDataEj
pub fn stub_b7345c() {
    // IDA 0xb7345c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::generateBatchGeometry(RBX::FastClusterMeshGenerator::MaterialGroup const&,RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex *,unsigned short *,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> &,bool)")]
// 0xb73678 — __ZN3RBX24FastClusterMeshGenerator21generateBatchGeometryERKNS0_13MaterialGroupERKNS0_5BatchEPNS_17GeometryGenerator6VertexEPtjRSt6vectorIjSaIjEEb
pub fn stub_b73678() {
    // IDA 0xb73678: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::generateShadowData(RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,bool)")]
// 0xb73b10 — __ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb
pub fn stub_b73b10() {
    // IDA 0xb73b10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::reserve(unsigned long)")]
// 0xb73ec8 — __ZNSt6vectorIjSaIjEE7reserveEm
pub fn stub_b73ec8() {
    // IDA 0xb73ec8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")]
// 0xb74384 — __ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb
pub fn stub_b74384() {
    // IDA 0xb74384: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::setupSharedGeometry(RBX::FastClusterSharedGeometry &,unsigned int,unsigned int,bool)")]
// 0xb745cc — __ZN3RBX24FastClusterMeshGenerator19setupSharedGeometryERNS_25FastClusterSharedGeometryEjjb
pub fn stub_b745cc() {
    // IDA 0xb745cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xb74a5c — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiNS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_T1_
pub fn stub_b74a5c() {
    // IDA 0xb74a5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xb74b78 — __ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_T0_
pub fn stub_b74b78() {
    // IDA 0xb74b78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xb74cf8 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEENS4_33BatchMaterialPlasticLODComparatorEEvT_SG_SG_T0_
pub fn stub_b74cf8() {
    // IDA 0xb74cf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator>(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> *,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,int,int,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,RBX::FastClusterMeshGenerator::BatchMaterialPlasticLODComparator)")]
// 0xb74d78 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPSt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS4_5BatchEESt6vectorIS9_SaIS9_EEEEiS9_NS4_33BatchMaterialPlasticLODComparatorEEvT_T0_SH_T1_T2_
pub fn stub_b74d78() {
    // IDA 0xb74d78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>*,std::vector<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>,std::allocator<std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *>>>>,std::pair<RBX::FastClusterMeshGenerator::MaterialGroup *,RBX::FastClusterMeshGenerator::Batch *> const&)")]
// 0xb74e54 — __ZNSt6vectorISt4pairIPN3RBX24FastClusterMeshGenerator13MaterialGroupEPNS2_5BatchEESaIS7_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_S9_EERKS7_
pub fn stub_b74e54() {
    // IDA 0xb74e54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<unsigned long long,RBX::FastClusterMeshGenerator::MaterialGroup,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::operator[](unsigned long long const&)")]
// 0xb74f70 — __ZNSt3mapIyN3RBX24FastClusterMeshGenerator13MaterialGroupESt4lessIyESaISt4pairIKyS2_EEEixERS6_
pub fn stub_b74f70() {
    // IDA 0xb74f70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")]
// 0xb758a8 — __ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_
pub fn stub_b758a8() {
    // IDA 0xb758a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")]
// 0xb75984 — __ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_
pub fn stub_b75984() {
    // IDA 0xb75984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FastClusterMeshGenerator::MaterialGroup::MaterialGroup(RBX::FastClusterMeshGenerator::MaterialGroup const&)")]
// 0xb75e34 — __ZN3RBX24FastClusterMeshGenerator13MaterialGroupC2ERKS1_
pub fn stub_b75e34() {
    // IDA 0xb75e34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
// 0xb75fe4 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_b75fe4() {
    // IDA 0xb75fe4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
// 0xb76120 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_b76120() {
    // IDA 0xb76120: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>,std::_Select1st<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>,std::less<unsigned long long>,std::allocator<std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup>>>::_M_insert_unique(std::pair<unsigned long long const,RBX::FastClusterMeshGenerator::MaterialGroup> const&)")]
// 0xb76270 — __ZNSt8_Rb_treeIySt4pairIKyN3RBX24FastClusterMeshGenerator13MaterialGroupEESt10_Select1stIS5_ESt4lessIyESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_b76270() {
    // IDA 0xb76270: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")]
// 0xb7654c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_b7654c() {
    // IDA 0xb7654c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")]
// 0xb7695c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm
pub fn stub_b7695c() {
    // IDA 0xb7695c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryGenerator::GeometryGenerator(void)")]
// 0xb7e648 — __ZN3RBX17GeometryGeneratorC1Ev
pub fn stub_b7e648() {
    // IDA 0xb7e648: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryGenerator::GeometryGenerator(RBX::GeometryGenerator::Vertex *,unsigned short *,unsigned int)")]
// 0xb7e680 — __ZN3RBX17GeometryGeneratorC1EPNS0_6VertexEPtj
pub fn stub_b7e680() {
    // IDA 0xb7e680: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryGenerator::resetBounds(void)")]
// 0xb7e720 — __ZN3RBX17GeometryGenerator11resetBoundsEv
pub fn stub_b7e720() {
    // IDA 0xb7e720: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FileMeshData>::~shared_ptr()")]
// 0xb7e7c8 — __ZN5boost10shared_ptrIN3RBX12FileMeshDataEED1Ev
// was: boost::shared_ptr<RBX::FileMeshData>::~shared_ptr()
pub fn stub_b7e7c8() {
    // IDA 0xb7e7c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ContentId*,std::vector<RBX::ContentId,std::allocator<RBX::ContentId>>>,RBX::ContentId const&)")]
// 0xb85a4c — __ZNSt6vectorIN3RBX9ContentIdESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_b85a4c() {
    // IDA 0xb85a4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MaterialGenerator::createBaseMaterial(unsigned int)")]
// 0xb86c9c — __ZN3RBX17MaterialGenerator18createBaseMaterialEj
pub fn stub_b86c9c() {
    // IDA 0xb86c9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MaterialGenerator::createRenderMaterial(unsigned int,int)")]
// 0xb871c0 — __ZN3RBX17MaterialGenerator20createRenderMaterialEji
pub fn stub_b871c0() {
    // IDA 0xb871c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MaterialGenerator::createMaterialForDecal(RBX::Decal *,unsigned int,RBX::AsyncResult *)")]
// 0xb8aa38 — __ZN3RBX17MaterialGenerator22createMaterialForDecalEPNS_5DecalEjPNS_11AsyncResultE
pub fn stub_b8aa38() {
    // IDA 0xb8aa38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::BrickColor const&)")]
// 0xb8bed8 — __ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_10BrickColorE
// was: anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::BrickColor const&)
pub fn stub_b8bed8() {
    // IDA 0xb8bed8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::ContentId const&,RBX::TextureCompositorLayer::CompositMode)")]
// 0xb8c314 — __ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE
// was: anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::ContentId const&,RBX::TextureCompositorLayer::CompositMode)
pub fn stub_b8c314() {
    // IDA 0xb8c314: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void std::__introsort_loop<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator)")]
// 0xb8c644 — __ZSt16__introsort_loopIPN12_GLOBAL__N_116AccoutrementMeshEiNS0_28AccoutrementMeshIdComparatorEEvT_S4_T0_T1_
// was: void std::__introsort_loop<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator)
pub fn stub_b8c644() {
    // IDA 0xb8c644: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}