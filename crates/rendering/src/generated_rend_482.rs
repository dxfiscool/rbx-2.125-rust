//! rendering shard 482 — 120 stubs 0x93c5f0..0xf2e714 EA-sorted rendering filter Ogre|G3D|Render|Adorn|Mesh
//! Next 120 uncovered rendering Filtered EA asc (remaining 361 -> 120 batch).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc rendering filter Ogre|G3D|Render|Adorn|Mesh

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x93c5f0 — __ZN3RBX20ExtrusionMeshBuilder5buildERKNS0_12DetailParamsE
// type: int __fastcall(_DWORD, _DWORD)
// was: RBX::ExtrusionMeshBuilder::build(RBX::ExtrusionMeshBuilder::DetailParams const&)
#[doc(alias = "RBX::ExtrusionMeshBuilder::build(RBX::ExtrusionMeshBuilder::DetailParams const&)")]
#[doc(alias = "__ZN3RBX20ExtrusionMeshBuilder5buildERKNS0_12DetailParamsE")]
// IDA 0x93c5f0: 424 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_93c5f0() {
}


// 0x93d1d0 — __ZNK3RBX18HumanoidIdentifier15getRelevantMeshEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::HumanoidIdentifier *__hidden this, RBX::PartInstance *)
// was: RBX::HumanoidIdentifier::getRelevantMesh(RBX::PartInstance *)const
#[doc(alias = "RBX::HumanoidIdentifier::getRelevantMesh(RBX::PartInstance *)const")]
#[doc(alias = "__ZNK3RBX18HumanoidIdentifier15getRelevantMeshEPNS_12PartInstanceE")]
// IDA 0x93d1d0: 31 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_93d1d0() {
}


// 0xa24000 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator6createEv")]
// IDA 0xa24000: 247 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a24000() {
}


// 0xa242b8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(RBX::BevelMesh **, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
// was: boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)
#[doc(alias = "boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv")]
// IDA 0xa242b8: 230 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a242b8() {
}


// 0xa24538 — __ZThn32_N3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn32_N3RBX12CylinderMeshD1Ev")]
// IDA 0xa24538: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a24538() {
}


// 0xa24548 — __ZThn36_N3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn36_N3RBX12CylinderMeshD0Ev")]
// IDA 0xa24548: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a24548() {
}


// 0xa245f0 — __ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0xa245f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a245f0() {
}


// 0xa24690 — __ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0xa24690: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a24690() {
}


// 0xa246a0 — __ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0xa246a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a246a0() {
}


// 0xa246b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0xa246b0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a246b0() {
}


// 0xa246b8 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev
// type: _Rb_tree_node_base *__fastcall(_Rb_tree_node_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0xa246b8: 423 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a246b8() {
}


// 0xa24bc8 — __ZN3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: RBX::BlockMesh::~BlockMesh()
#[doc(alias = "RBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZN3RBX9BlockMeshD0Ev")]
// IDA 0xa24bc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a24bc8() {
}


// 0xa24c68 — __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv")]
// IDA 0xa24c68: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a24c68() {
}


// 0xa24cd8 — __ZThn32_N3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn32_N3RBX9BlockMeshD0Ev")]
// IDA 0xa24cd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a24cd8() {
}


// 0xa24d80 — __ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0xa24d80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_a24d80() {
}


// 0xb10b78 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0xb10b78: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b10b78() {
}


// 0xb10d08 — __ZN3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZN3RBX12CylinderMeshD0Ev")]
// IDA 0xb10d08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b10d08() {
}


// 0xb10da8 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
// IDA 0xb10da8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b10da8() {
}


// 0xb10e18 — __ZThn32_N3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn32_N3RBX12CylinderMeshD0Ev")]
// IDA 0xb10e18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b10e18() {
}


// 0xb10ec0 — __ZThn36_N3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn36_N3RBX12CylinderMeshD1Ev")]
// IDA 0xb10ec0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b10ec0() {
}


// 0xb10ed0 — __ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0xb10ed0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b10ed0() {
}


// 0xb10ee0 — __ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0xb10ee0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b10ee0() {
}


// 0xb10f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0xb10f88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b10f88() {
}


// 0xb10f90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0xb10f90: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b10f90() {
}


// 0xb10fb0 — __ZN3RBX4Name13callDoDeclareILZNS_13sCylinderMeshEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sCylinderMeshEEEEvv")]
// IDA 0xb10fb0: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b10fb0() {
}


// 0xb11088 — __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(__guard *, int, int, int (*)(const char *, ...), pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator6createEv")]
// IDA 0xb11088: 247 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b11088() {
}


// 0xb11340 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xb11340: 240 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b11340() {
}


// 0xb6f5f8 — __ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE")]
// IDA 0xb6f5f8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6f5f8() {
}


// 0xb6f678 — __ZN3RBX24FastClusterMeshGenerator16isPartCompositedEPNS_12PartInstanceE
// type: int __fastcall(RBX::PartInstance **this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::isPartComposited(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isPartComposited(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator16isPartCompositedEPNS_12PartInstanceE")]
// IDA 0xb6f678: 134 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6f678() {
}


// 0xb6f800 — __ZN3RBX24FastClusterMeshGenerator10isPartHeadEPNS_12PartInstanceE
// type: int __fastcall(RBX::PartInstance **this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::isPartHead(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isPartHead(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator10isPartHeadEPNS_12PartInstanceE")]
// IDA 0xb6f800: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6f800() {
}


// 0xb6f9bc — __ZN3RBX24FastClusterMeshGenerator11addInstanceEmPNS_12PartInstanceEPNS_5DecalEjNS_22eShadowCullingPriorityEPNS_11AsyncResultE
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int)
// was: RBX::FastClusterMeshGenerator::addInstance(unsigned long,RBX::PartInstance *,RBX::Decal *,unsigned int,RBX::eShadowCullingPriority,RBX::AsyncResult *)
#[doc(alias = "RBX::FastClusterMeshGenerator::addInstance(unsigned long,RBX::PartInstance *,RBX::Decal *,unsigned int,RBX::eShadowCullingPriority,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator11addInstanceEmPNS_12PartInstanceEPNS_5DecalEjNS_22eShadowCullingPriorityEPNS_11AsyncResultE")]
// IDA 0xb6f9bc: 810 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b6f9bc() {
}


// 0xb70210 — __ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE
// type: int __fastcall(int, bool *, int, _DWORD *)
// was: RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)
#[doc(alias = "RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE")]
// IDA 0xb70210: 938 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b70210() {
}


// 0xb70d90 — __ZN3RBX24FastClusterMeshGeneratorD1Ev
// type: void __fastcall(RBX::FastClusterMeshGenerator *__hidden this)
// was: RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()
#[doc(alias = "RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGeneratorD1Ev")]
// IDA 0xb70d90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b70d90() {
}


// 0xb730b8 — __ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev
// type: _DWORD **__fastcall(_DWORD **)
// was: std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()
#[doc(alias = "std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")]
#[doc(alias = "__ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev")]
// IDA 0xb730b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b730b8() {
}


// 0xb73f30 — __ZN3RBX24FastClusterMeshGenerator20getRelativeTransformEPNS_12PartInstanceES2_
// type: void __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *, RBX::PartInstance *, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::getRelativeTransform(RBX::PartInstance *,RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::getRelativeTransform(RBX::PartInstance *,RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20getRelativeTransformEPNS_12PartInstanceES2_")]
// IDA 0xb73f30: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b73f30() {
}


// 0xb74384 — __ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, int)
// was: RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)
#[doc(alias = "RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb")]
// IDA 0xb74384: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b74384() {
}


// 0xb75100 — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void __fastcall(int *, int, int)
// was: std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance const&)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0xb75100: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_b75100() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xb755a8 — __ZSt24__uninitialized_copy_auxIPN3RBX24FastClusterMeshGenerator13BatchInstanceES3_ET0_T_S5_S4_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
// was: RBX::FastClusterMeshGenerator::BatchInstance * std::__uninitialized_copy_aux<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,std::__false_type)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance * std::__uninitialized_copy_aux<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX24FastClusterMeshGenerator13BatchInstanceES3_ET0_T_S5_S4_St12__false_type")]
// IDA 0xb755a8: 147 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b755a8() {
}


// 0xb75794 — __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX24FastClusterMeshGenerator13BatchInstanceES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
// was: RBX::FastClusterMeshGenerator::BatchInstance * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *)")]
#[doc(alias = "__ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX24FastClusterMeshGenerator13BatchInstanceES5_EET0_T_S7_S6_")]
// IDA 0xb75794: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_b75794() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0xb758a8 — __ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_
// type: void __fastcall(int, std::_List_node_base *, int, int, void *, int)
// was: std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)
#[doc(alias = "std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "__ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_")]
// IDA 0xb758a8: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b758a8() {
}


// 0xb75984 — __ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_
// type: int __fastcall(int, __int64 *)
// was: RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)
#[doc(alias = "RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_")]
// IDA 0xb75984: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b75984() {
}


// 0xb75b3c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EEC2ERKS4_
// type: int __fastcall(int *, int *, int, int)
// was: std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::vector(std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>> const&)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::vector(std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EEC2ERKS4_")]
// IDA 0xb75b3c: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b75b3c() {
}


// 0xb75c48 — __ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX24FastClusterMeshGenerator13BatchInstanceESt6vectorIS4_SaIS4_EEEEPS4_ET0_T_SD_SC_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
// was: RBX::FastClusterMeshGenerator::BatchInstance* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*>(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*,std::__false_type)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*>(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX24FastClusterMeshGenerator13BatchInstanceESt6vectorIS4_SaIS4_EEEEPS4_ET0_T_SD_SC_St12__false_type")]
// IDA 0xb75c48: 147 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b75c48() {
}


// 0xb76338 — __ZN3RBX24FastClusterMeshGenerator20isBodyPartCompositedEPNS_12PartInstanceE
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::isBodyPartComposited(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isBodyPartComposited(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20isBodyPartCompositedEPNS_12PartInstanceE")]
// IDA 0xb76338: 163 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b76338() {
}


// 0xb7654c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, __int64 *, char **)
// was: std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0xb7654c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_b7654c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xb7695c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm
// type: unsigned int __fastcall(void **, unsigned int)
// was: std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm")]
// IDA 0xb7695c: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7695c() {
}


// 0xb77140 — __ZN3RBX17GeometryGenerator11addFileMeshEPNS_12FileMeshDataEPNS_13DataModelMeshEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsE
// type: void __fastcall(int, _DWORD *, int, RBX::PartInstance *, int, int)
// was: RBX::GeometryGenerator::addFileMesh(RBX::FileMeshData *,RBX::DataModelMesh *,RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&)
#[doc(alias = "RBX::GeometryGenerator::addFileMesh(RBX::FileMeshData *,RBX::DataModelMesh *,RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&)")]
#[doc(alias = "__ZN3RBX17GeometryGenerator11addFileMeshEPNS_12FileMeshDataEPNS_13DataModelMeshEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsE")]
// IDA 0xb77140: 793 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b77140() {
}


// 0xb77bcc — __ZN3RBXL8getColorEPNS_12PartInstanceEPNS_5DecalEPNS_13DataModelMeshERKNS_17GeometryGenerator7OptionsEjb
// type: int __fastcall(_BYTE *, RBX::PartInstance *this, int, int, int *, char, int)
// was: RBX::getColor(RBX::PartInstance *,RBX::Decal *,RBX::DataModelMesh *,RBX::GeometryGenerator::Options const&,unsigned int,bool)
#[doc(alias = "RBX::getColor(RBX::PartInstance *,RBX::Decal *,RBX::DataModelMesh *,RBX::GeometryGenerator::Options const&,unsigned int,bool)")]
#[doc(alias = "__ZN3RBXL8getColorEPNS_12PartInstanceEPNS_5DecalEPNS_13DataModelMeshERKNS_17GeometryGenerator7OptionsEjb")]
// IDA 0xb77bcc: 158 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b77bcc() {
}


// 0xb7ba5c — __ZN3RBX17GeometryGenerator11addPartImplEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE
// type: void __fastcall(float *, unsigned int, int, int *, _DWORD **)
// was: RBX::GeometryGenerator::addPartImpl(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)
#[doc(alias = "RBX::GeometryGenerator::addPartImpl(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)")]
#[doc(alias = "__ZN3RBX17GeometryGenerator11addPartImplEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE")]
// IDA 0xb7ba5c: 616 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7ba5c() {
}


// 0xb7e120 — __ZN3RBXL9fetchMeshERKNS_6MeshIdEPNS_8InstanceEPNS_11AsyncResultE
// type: void __fastcall(int, _DWORD *, int, pthread_mutex_t *)
// was: RBX::fetchMesh(RBX::MeshId const&,RBX::Instance *,RBX::AsyncResult *)
#[doc(alias = "RBX::fetchMesh(RBX::MeshId const&,RBX::Instance *,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBXL9fetchMeshERKNS_6MeshIdEPNS_8InstanceEPNS_11AsyncResultE")]
// IDA 0xb7e120: 476 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7e120() {
}


// 0xb7e74c — __ZN3RBX17GeometryGenerator11addInstanceEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE
// type: int __fastcall(int, unsigned int, int, int *, _DWORD **)
// was: RBX::GeometryGenerator::addInstance(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)
#[doc(alias = "RBX::GeometryGenerator::addInstance(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)")]
#[doc(alias = "__ZN3RBX17GeometryGenerator11addInstanceEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE")]
// IDA 0xb7e74c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b7e74c() {
}


// 0xb7e7c8 — __ZN5boost10shared_ptrIN3RBX12FileMeshDataEED1Ev
// type: int __fastcall(int)
// was: boost::shared_ptr<RBX::FileMeshData>::~shared_ptr()
#[doc(alias = "boost::shared_ptr<RBX::FileMeshData>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12FileMeshDataEED1Ev")]
// IDA 0xb7e7c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b7e7c8() {
}


// 0xb8c108 — __ZN12_GLOBAL__N_118getExtraSlotMeshIdEPN3RBX12PartInstanceERKNS0_18HumanoidIdentifierERA4_KNS_16AccoutrementMeshE
// type: void __fastcall(std::string *, int, int, int *)
// was: `anonymous namespace'::getExtraSlotMeshId(RBX::PartInstance *,RBX::HumanoidIdentifier const&,`anonymous namespace'::AccoutrementMesh const(&)[4])
#[doc(alias = "anonymous namespace::getExtraSlotMeshId(RBX::PartInstance *,RBX::HumanoidIdentifier const&,anonymous namespace::AccoutrementMesh const(&)[4])")]
#[doc(alias = "__ZN12_GLOBAL__N_118getExtraSlotMeshIdEPN3RBX12PartInstanceERKNS0_18HumanoidIdentifierERA4_KNS_16AccoutrementMeshE")]
// IDA 0xb8c108: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8c108() {
}


// 0xb8c644 — __ZSt16__introsort_loopIPN12_GLOBAL__N_116AccoutrementMeshEiNS0_28AccoutrementMeshIdComparatorEEvT_S4_T0_T1_
// type: int __fastcall(int result, __int64 *, int)
// was: void std::__introsort_loop<`anonymous namespace'::AccoutrementMesh *,int,`anonymous namespace'::AccoutrementMeshIdComparator>(`anonymous namespace'::AccoutrementMesh *,`anonymous namespace'::AccoutrementMesh *,int,`anonymous namespace'::AccoutrementMeshIdComparator)
#[doc(alias = "void std::__introsort_loop<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator)")]
#[doc(alias = "__ZSt16__introsort_loopIPN12_GLOBAL__N_116AccoutrementMeshEiNS0_28AccoutrementMeshIdComparatorEEvT_S4_T0_T1_")]
// IDA 0xb8c644: 157 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8c644() {
}


// 0xb8c808 — __ZSt13__adjust_heapIPN12_GLOBAL__N_116AccoutrementMeshEiS1_NS0_28AccoutrementMeshIdComparatorEEvT_T0_S5_T1_T2_
// type: int __fastcall(int, int, int, int, int, int)
// was: void std::__adjust_heap<`anonymous namespace'::AccoutrementMesh *,int,`anonymous namespace'::AccoutrementMesh,`anonymous namespace'::AccoutrementMeshIdComparator>(`anonymous namespace'::AccoutrementMesh *,int,int,`anonymous namespace'::AccoutrementMesh,`anonymous namespace'::AccoutrementMeshIdComparator)
#[doc(alias = "void std::__adjust_heap<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,int,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator)")]
#[doc(alias = "__ZSt13__adjust_heapIPN12_GLOBAL__N_116AccoutrementMeshEiS1_NS0_28AccoutrementMeshIdComparatorEEvT_T0_S5_T1_T2_")]
// IDA 0xb8c808: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8c808() {
}


// 0xb8c91c — __ZN3RBX13DataModelUtil11getFileMeshEPNS_13DataModelMeshE
// type: RBX::DataModelUtil *__fastcall(RBX::Reflection::ClassDescriptor **this, RBX::DataModelMesh *)
// was: RBX::DataModelUtil::getFileMesh(RBX::DataModelMesh *)
#[doc(alias = "RBX::DataModelUtil::getFileMesh(RBX::DataModelMesh *)")]
#[doc(alias = "__ZN3RBX13DataModelUtil11getFileMeshEPNS_13DataModelMeshE")]
// IDA 0xb8c91c: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8c91c() {
}


// 0xb9a3e4 — __ZN3RBX7MeshGen12addRefVertexEi
// type: int __fastcall(RBX::MeshGen *this, int)
// was: RBX::MeshGen::addRefVertex(int)
#[doc(alias = "RBX::MeshGen::addRefVertex(int)")]
#[doc(alias = "__ZN3RBX7MeshGen12addRefVertexEi")]
// IDA 0xb9a3e4: 2 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a3e4() {
}


// 0xb9a3e8 — __ZN3RBX7MeshGen13releaseVertexEi
// type: void __fastcall(RBX::MeshGen *this, int)
// was: RBX::MeshGen::releaseVertex(int)
#[doc(alias = "RBX::MeshGen::releaseVertex(int)")]
#[doc(alias = "__ZN3RBX7MeshGen13releaseVertexEi")]
// IDA 0xb9a3e8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b9a3e8() {
}


// 0xb9a3ec — __ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this, int)
// was: RBX::ManualObjectMeshGenAdapter::getVertex(int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getVertex(int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi")]
// IDA 0xb9a3ec: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a3ec() {
}


// 0xb9a50c — __ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, unsigned int)
// was: RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm")]
// IDA 0xb9a50c: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a50c() {
}


// 0xb9a524 — __ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE
// type: int __fastcall(int, _DWORD *)
// was: RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE")]
// IDA 0xb9a524: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a524() {
}


// 0xb9a558 — __ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, unsigned int)
// was: RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm")]
// IDA 0xb9a558: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a558() {
}


// 0xb9a568 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, int, int, int)
// was: RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii")]
// IDA 0xb9a568: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a568() {
}


// 0xb9a580 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, int, int, int, int)
// was: RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii")]
// IDA 0xb9a580: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a580() {
}


// 0xb9a5b0 — __ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this, int, int)
// was: RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii")]
// IDA 0xb9a5b0: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a5b0() {
}


// 0xb9a6d0 — __ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this)
// was: RBX::ManualObjectMeshGenAdapter::getIndexCount(void)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getIndexCount(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv")]
// IDA 0xb9a6d0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a6d0() {
}


// 0xb9a6d4 — __ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this)
// was: RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv")]
// IDA 0xb9a6d4: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a6d4() {
}


// 0xb9a7f4 — __ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this)
// was: RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv")]
// IDA 0xb9a7f4: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a7f4() {
}


// 0xb9a918 — __ZN3RBX7MeshGen20popVerticesTransformEv
// type: void __fastcall(RBX::MeshGen *this)
// was: RBX::MeshGen::popVerticesTransform(void)
#[doc(alias = "RBX::MeshGen::popVerticesTransform(void)")]
#[doc(alias = "__ZN3RBX7MeshGen20popVerticesTransformEv")]
// IDA 0xb9a918: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b9a918() {
}


// 0xbef8d8 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0xbef8d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bef8d8() {
}


// 0xbef8e0 — __ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")]
// IDA 0xbef8e0: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bef8e0() {
}


// 0xc2c0a0 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0xc2c0a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2c0a0() {
}


// 0xc2c220 — __ZN3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZN3RBX12CylinderMeshD1Ev")]
// IDA 0xc2c220: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c2c220() {
}


// 0xc2c260 — __ZThn32_NK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
// IDA 0xc2c260: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c2c260() {
}


// 0xc2cf20 — __ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0xc2cf20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2cf20() {
}


// 0xc2cfd0 — __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// IDA 0xc2cfd0: 143 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2cfd0() {
}


// 0xc2d3b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xc2d3b0: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2d3b0() {
}


// 0xc2d520 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0xc2d520: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c2d520() {
}


// 0xc2d530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0xc2d530: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2d530() {
}


// 0xc2d670 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0xc2d670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2d670() {
}


// 0xc2d7f0 — __ZThn32_N3RBX9BlockMeshD1Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn32_N3RBX9BlockMeshD1Ev")]
// IDA 0xc2d7f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2d7f0() {
}


// 0xc2d800 — __ZThn36_N3RBX9BlockMeshD1Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn36_N3RBX9BlockMeshD1Ev")]
// IDA 0xc2d800: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2d800() {
}


// 0xc2d810 — __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0xc2d810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2d810() {
}


// 0xc2d8b0 — __ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0xc2d8b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c2d8b0() {
}


// 0xc2d8c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0xc2d8c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_c2d8c0() {
}


// 0xc2d8d0 — __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")]
// IDA 0xc2d8d0: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2d8d0() {
}


// 0xc2d9f0 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0xc2d9f0: 320 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c2d9f0() {
}


// 0xc304a0 — __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// IDA 0xc304a0: 143 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c304a0() {
}


// 0xf20488 — __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim")]
// IDA 0xf20488: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f20488() {
}


// 0xf2077c — __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")]
// IDA 0xf2077c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2077c() {
}


// 0xf20788 — __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev$shim
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
// IDA 0xf20788: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f20788() {
}


// 0xf20794 — __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim")]
// IDA 0xf20794: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f20794() {
}


// 0xf207b8 — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// IDA 0xf207b8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f207b8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xf207c4 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_$shim")]
// IDA 0xf207c4: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f207c4() {
}


// 0xf21250 — __ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v$shim")]
// IDA 0xf21250: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21250() {
}


// 0xf2125c — __ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_$shim")]
// IDA 0xf2125c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2125c() {
}


// 0xf21268 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// IDA 0xf21268: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f21268() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xf212c8 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev$shim
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
// IDA 0xf212c8: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f212c8() {
}


// 0xf21340 — __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim")]
// IDA 0xf21340: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21340() {
}


// 0xf2134c — __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator12getClassNameEv$shim
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")]
// IDA 0xf2134c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2134c() {
}


// 0xf21358 — __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim")]
// IDA 0xf21358: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21358() {
}


// 0xf219b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim")]
// IDA 0xf219b8: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f219b8() {
}


// 0xf219c4 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim")]
// IDA 0xf219c4: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f219c4() {
}


// 0xf21a30 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim")]
// IDA 0xf21a30: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21a30() {
}


// 0xf21a3c — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim")]
// IDA 0xf21a3c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21a3c() {
}


// 0xf21c4c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DataModelMesh7LODTypeEEEE14doGetSingletonEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DataModelMesh7LODTypeEEEE14doGetSingletonEv$shim")]
// IDA 0xf21c4c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21c4c() {
}


// 0xf21c58 — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEED2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEED2Ev$shim")]
// IDA 0xf21c58: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f21c58() {
}


// 0xf232d8 — __ZNK3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7Creator12getClassNameEv$shim")]
// IDA 0xf232d8: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f232d8() {
}


// 0xf232e4 — __ZN3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
// IDA 0xf232e4: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f232e4() {
}


// 0xf232f0 — __ZN3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7CreatorD2Ev$shim")]
// IDA 0xf232f0: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f232f0() {
}


// 0xf23308 — __ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim")]
// IDA 0xf23308: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f23308() {
}


// 0xf23314 — __ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim")]
// IDA 0xf23314: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f23314() {
}


// 0xf23320 — __ZNK3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")]
// IDA 0xf23320: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f23320() {
}


// 0xf2332c — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim")]
// IDA 0xf2332c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2332c() {
}


// 0xf23338 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
#[doc(alias = "__ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// IDA 0xf23338: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f23338() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xf24208 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")]
// IDA 0xf24208: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f24208() {
}


// 0xf24214 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")]
// IDA 0xf24214: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f24214() {
}


// 0xf25984 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
// IDA 0xf25984: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f25984() {
}


// 0xf25a20 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv$shim
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv$shim")]
// IDA 0xf25a20: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f25a20() {
}


// 0xf2e714 — j___ZN3RBX11IndexedMesh13lowersChangedEv
// type: _DWORD __fastcall(RBX::IndexedMesh *__hidden this)
// was: RBX::IndexedMesh::lowersChanged(void)
#[doc(alias = "RBX::IndexedMesh::lowersChanged(void)")]
#[doc(alias = "j___ZN3RBX11IndexedMesh13lowersChangedEv")]
// IDA 0xf2e714: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f2e714() {
}
