//! core wd watchdog_a — 100 core stubs EA-sorted asc next uncovered distinct not yet in crates.
//! Source: ida/export.json (85545 funcs) filtered demangled/mangled excludes Reflection|Instance|DataModel|Ogre|Gfx|Render|G3D|Sound|Audio|Network|Script, EA-sorted asc, next 100 uncovered distinct (global 63232 before).
//! Range: 0x73a0f0..0x740c44 (100 stubs).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::JointStage::JointStage(RBX::IStage *,RBX::World *)")]
// 0x73a0f0 — __ZN3RBX10JointStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x73a0f0() {
    // IDA 0x73a0f0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::JointStage::JointStage(RBX::IStage *,RBX::World *)")]
// 0x73a0f4 — __ZN3RBX10JointStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x73a0f4() {
    // IDA 0x73a0f4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::JointStage::~JointStage()")]
// 0x73a20c — __ZN3RBX10JointStageD0Ev
// type: void __fastcall(RBX::JointStage *__hidden this)
pub fn stub_0x73a20c() {
    // IDA 0x73a20c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::~JointStage()")]
// 0x73a2ac — __ZN3RBX10JointStageD1Ev
// type: void __fastcall(RBX::JointStage *__hidden this)
pub fn stub_0x73a2ac() {
    // IDA 0x73a2ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::~JointStage()")]
// 0x73a2b0 — __ZN3RBX10JointStageD2Ev
// type: void __fastcall(RBX::JointStage *__hidden this)
pub fn stub_0x73a2b0() {
    // IDA 0x73a2b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::moveEdgeToDownstream(RBX::Edge *)")]
// 0x73a514 — __ZN3RBX10JointStage20moveEdgeToDownstreamEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Edge *)
pub fn stub_0x73a514() {
    // IDA 0x73a514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::edgeHasPrimitivesHere(RBX::Edge *)")]
// 0x73a580 — __ZN3RBX10JointStage21edgeHasPrimitivesHereEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Edge *)
pub fn stub_0x73a580() {
    // IDA 0x73a580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::removeEdgeFromDownstream(RBX::Edge *)")]
// 0x73a5a4 — __ZN3RBX10JointStage24removeEdgeFromDownstreamEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Edge *)
pub fn stub_0x73a5a4() {
    // IDA 0x73a5a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::edgeHasPrimitiveHere(RBX::Edge *,RBX::Primitive *)")]
// 0x73a610 — __ZN3RBX10JointStage20edgeHasPrimitiveHereEPNS_4EdgeEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Edge *, RBX::Primitive *)
pub fn stub_0x73a610() {
    // IDA 0x73a610: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::JointStage::visitAddedPrimitive(RBX::Primitive *,RBX::Joint *,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>> &)")]
// 0x73a6b4 — __ZN3RBX10JointStage19visitAddedPrimitiveEPNS_9PrimitiveEPNS_5JointERSt6vectorIS4_SaIS4_EE
// type: int __fastcall(RBX::JointStage *this, RBX::Primitive *, RBX::Edge *)
pub fn stub_0x73a6b4() {
    // IDA 0x73a6b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::JointStage::onPrimitiveAdded(RBX::Primitive *)")]
// 0x73a744 — __ZN3RBX10JointStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Primitive *)
pub fn stub_0x73a744() {
    // IDA 0x73a744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::JointStage::removeJointFromHere(RBX::Joint *)")]
// 0x73a94c — __ZN3RBX10JointStage19removeJointFromHereEPNS_5JointE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Joint *)
pub fn stub_0x73a94c() {
    // IDA 0x73a94c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::JointStage::onPrimitiveRemoving(RBX::Primitive *)")]
// 0x73a9dc — __ZN3RBX10JointStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Primitive *)
pub fn stub_0x73a9dc() {
    // IDA 0x73a9dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::JointStage::putJointHere(RBX::Joint *)")]
// 0x73acec — __ZN3RBX10JointStage12putJointHereEPNS_5JointE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Joint *)
pub fn stub_0x73acec() {
    // IDA 0x73acec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::JointStage::onEdgeAdded(RBX::Edge *)")]
// 0x73ad78 — __ZN3RBX10JointStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Edge *)
pub fn stub_0x73ad78() {
    // IDA 0x73ad78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::JointStage::onEdgeRemoving(RBX::Edge *)")]
// 0x73ae5c — __ZN3RBX10JointStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::JointStage *__hidden this, RBX::Edge *)
pub fn stub_0x73ae5c() {
    // IDA 0x73ae5c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BiMultiMap<RBX::Primitive *,RBX::Joint *>::removePair(RBX::Primitive * const&,RBX::Joint * const&)")]
// 0x73af78 — __ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE10removePairERKS2_RKS4_
pub fn stub_0x73af78() {
    // IDA 0x73af78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::JointStage::getStageType(void)const")]
// 0x73b0a4 — __ZNK3RBX10JointStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::JointStage *__hidden this)
pub fn stub_0x73b0a4() {
    // IDA 0x73b0a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BiMultiMap<RBX::Primitive *,RBX::Joint *>::pairInMap(RBX::Primitive * const&,RBX::Joint * const&)")]
// 0x73b0a8 — __ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE9pairInMapERKS2_RKS4_
pub fn stub_0x73b0a8() {
    // IDA 0x73b0a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_insert_equal(std::pair<RBX::Primitive * const,RBX::Joint *> const&)")]
// 0x73b118 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_insert_equalERKS7_
pub fn stub_0x73b118() {
    // IDA 0x73b118: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Primitive * const,RBX::Joint *> const&)")]
// 0x73b144 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_0x73b144() {
    // IDA 0x73b144: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::erase(RBX::Primitive * const&)")]
// 0x73b19c — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_0x73b19c() {
    // IDA 0x73b19c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::equal_range(RBX::Primitive * const&)")]
// 0x73b1c4 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
// type: _DWORD *__fastcall(_DWORD *result, int, _DWORD *)
pub fn stub_0x73b1c4() {
    // IDA 0x73b1c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::erase(std::_Rb_tree_iterator<RBX::Primitive *>,std::_Rb_tree_iterator<RBX::Primitive *>)")]
// 0x73b210 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x73b210() {
    // IDA 0x73b210: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Primitive * const,RBX::Joint *>> *)")]
// 0x73b2a8 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_0x73b2a8() {
    // IDA 0x73b2a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KernelJoint::putInKernel(RBX::Kernel *)")]
// 0x73b398 — __ZN3RBX11KernelJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this, RBX::Kernel *)
pub fn stub_0x73b398() {
    // IDA 0x73b398: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KernelJoint::removeFromKernel(void)")]
// 0x73b3b4 — __ZN3RBX11KernelJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
pub fn stub_0x73b3b4() {
    // IDA 0x73b3b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SurfaceType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::operator[](RBX::Name const* const&)")]
// 0x73bb4c — __ZNSt3mapIPKN3RBX4NameENS0_11SurfaceTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_0x73bb4c() {
    // IDA 0x73bb4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
// 0x73bba4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x73bba4() {
    // IDA 0x73bba4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
// 0x73bc58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_0x73bc58() {
    // IDA 0x73bc58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
// 0x73bcb0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_0x73bcb0() {
    // IDA 0x73bcb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::resize(unsigned long,RBX::SurfaceType)")]
// 0x73bd18 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_
pub fn stub_0x73bd18() {
    // IDA 0x73bd18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::push_back(RBX::SurfaceType const&)")]
// 0x73bd4c — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_
pub fn stub_0x73bd4c() {
    // IDA 0x73bd4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,RBX::SurfaceType const&)")]
// 0x73bd74 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x73bd74() {
    // IDA 0x73bd74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_allocate(unsigned long)")]
// 0x73be58 — __ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm
pub fn stub_0x73be58() {
    // IDA 0x73be58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SurfaceType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SurfaceType *,RBX::SurfaceType *>(RBX::SurfaceType *,RBX::SurfaceType *,RBX::SurfaceType *)")]
// 0x73be70 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_
pub fn stub_0x73be70() {
    // IDA 0x73be70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,unsigned long,RBX::SurfaceType const&)")]
// 0x73beac — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x73beac() {
    // IDA 0x73beac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Mechanism::Mechanism(void)")]
// 0x73c104 — __ZN3RBX9MechanismC1Ev
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c104() {
    // IDA 0x73c104: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Mechanism::Mechanism(void)")]
// 0x73c108 — __ZN3RBX9MechanismC2Ev
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c108() {
    // IDA 0x73c108: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Mechanism::~Mechanism()")]
// 0x73c1e8 — __ZN3RBX9MechanismD0Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c1e8() {
    // IDA 0x73c1e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::~Mechanism()")]
// 0x73c288 — __ZN3RBX9MechanismD1Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c288() {
    // IDA 0x73c288: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Mechanism::~Mechanism()")]
// 0x73c28c — __ZThn8_N3RBX9MechanismD0Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c28c() {
    // IDA 0x73c28c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::~Mechanism()")]
// 0x73c294 — __ZN3RBX9MechanismD2Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c294() {
    // IDA 0x73c294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Mechanism::~Mechanism()")]
// 0x73c350 — __ZThn8_N3RBX9MechanismD1Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c350() {
    // IDA 0x73c350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::getConstMechanismPrimitive(void)const")]
// 0x73c358 — __ZNK3RBX9Mechanism26getConstMechanismPrimitiveEv
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c358() {
    // IDA 0x73c358: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::getMechanismPrimitive(void)")]
// 0x73c36c — __ZN3RBX9Mechanism21getMechanismPrimitiveEv
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c36c() {
    // IDA 0x73c36c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::isComplexMovingMechanism(RBX::Assembly const*)")]
// 0x73c380 — __ZN3RBX9Mechanism24isComplexMovingMechanismEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Assembly *)
pub fn stub_0x73c380() {
    // IDA 0x73c380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::isMovingAssemblyRoot(RBX::Assembly const*)")]
// 0x73c3e4 — __ZN3RBX9Mechanism20isMovingAssemblyRootEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Assembly *)
pub fn stub_0x73c3e4() {
    // IDA 0x73c3e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Mechanism::getMovingAssemblyRoot(RBX::Assembly *)")]
// 0x73c40c — __ZN3RBX9Mechanism21getMovingAssemblyRootEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, RBX::Assembly *)
pub fn stub_0x73c40c() {
    // IDA 0x73c40c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::getConstMovingAssemblyRoot(RBX::Assembly const*)")]
// 0x73c434 — __ZN3RBX9Mechanism26getConstMovingAssemblyRootEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Assembly *)
pub fn stub_0x73c434() {
    // IDA 0x73c434: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::getConstRootMovingPrimitive(RBX::Primitive const*)")]
// 0x73c45c — __ZN3RBX9Mechanism27getConstRootMovingPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Primitive *)
pub fn stub_0x73c45c() {
    // IDA 0x73c45c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::getRootMovingPrimitive(RBX::Primitive *)")]
// 0x73c4d0 — __ZN3RBX9Mechanism22getRootMovingPrimitiveEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, RBX::Primitive *)
pub fn stub_0x73c4d0() {
    // IDA 0x73c4d0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::getPrimitiveMechanism(RBX::Primitive *)")]
// 0x73c4d4 — __ZN3RBX9Mechanism21getPrimitiveMechanismEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
pub fn stub_0x73c4d4() {
    // IDA 0x73c4d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::getConstPrimitiveMechanism(RBX::Primitive const*)")]
// 0x73c4fc — __ZN3RBX9Mechanism26getConstPrimitiveMechanismEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
pub fn stub_0x73c4fc() {
    // IDA 0x73c4fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::getRootAssembly(void)")]
// 0x73c524 — __ZN3RBX9Mechanism15getRootAssemblyEv
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
pub fn stub_0x73c524() {
    // IDA 0x73c524: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Mechanism::isMechanismRootPrimitive(RBX::Primitive const*)")]
// 0x73c584 — __ZN3RBX9Mechanism24isMechanismRootPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Primitive *)
pub fn stub_0x73c584() {
    // IDA 0x73c584: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MechToAssemblyStage::MechToAssemblyStage(RBX::IStage *,RBX::World *)")]
// 0x73c6f0 — __ZN3RBX19MechToAssemblyStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x73c6f0() {
    // IDA 0x73c6f0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MechToAssemblyStage::MechToAssemblyStage(RBX::IStage *,RBX::World *)")]
// 0x73c6f4 — __ZN3RBX19MechToAssemblyStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x73c6f4() {
    // IDA 0x73c6f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MechToAssemblyStage::~MechToAssemblyStage()")]
// 0x73c7c8 — __ZN3RBX19MechToAssemblyStageD0Ev
// type: void __fastcall(RBX::MechToAssemblyStage *__hidden this)
pub fn stub_0x73c7c8() {
    // IDA 0x73c7c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MechToAssemblyStage::~MechToAssemblyStage()")]
// 0x73c880 — __ZN3RBX19MechToAssemblyStageD1Ev
// type: void __fastcall(RBX::MechToAssemblyStage *__hidden this)
pub fn stub_0x73c880() {
    // IDA 0x73c880: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MechToAssemblyStage::onSimulateAssemblyRootAdded(RBX::Assembly *)")]
// 0x73c8a4 — __ZN3RBX19MechToAssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
pub fn stub_0x73c8a4() {
    // IDA 0x73c8a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MechToAssemblyStage::onSimulateAssemblyRootRemoving(RBX::Assembly *)")]
// 0x73c958 — __ZN3RBX19MechToAssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
pub fn stub_0x73c958() {
    // IDA 0x73c958: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MechToAssemblyStage::onNoSimulateAssemblyRootAdded(RBX::Assembly *)")]
// 0x73ca18 — __ZN3RBX19MechToAssemblyStage29onNoSimulateAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
pub fn stub_0x73ca18() {
    // IDA 0x73ca18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MechToAssemblyStage::onNoSimulateAssemblyRootRemoving(RBX::Assembly *)")]
// 0x73cacc — __ZN3RBX19MechToAssemblyStage32onNoSimulateAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
pub fn stub_0x73cacc() {
    // IDA 0x73cacc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MechToAssemblyStage::onFixedAssemblyAdded(RBX::Assembly *)")]
// 0x73cb8c — __ZN3RBX19MechToAssemblyStage20onFixedAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
pub fn stub_0x73cb8c() {
    // IDA 0x73cb8c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MechToAssemblyStage::onFixedAssemblyRemoving(RBX::Assembly *)")]
// 0x73cba8 — __ZN3RBX19MechToAssemblyStage23onFixedAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
pub fn stub_0x73cba8() {
    // IDA 0x73cba8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::MechToAssemblyStage::getStageType(void)const")]
// 0x73cbc4 — __ZNK3RBX19MechToAssemblyStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this)
pub fn stub_0x73cbc4() {
    // IDA 0x73cbc4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Mesh::addVertex(float,float,float)")]
// 0x73cea8 — __ZN3RBX4POLY4Mesh9addVertexEfff
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, float, float, float)
pub fn stub_0x73cea8() {
    // IDA 0x73cea8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Mesh::addFace(unsigned long,unsigned long,unsigned long,unsigned long)")]
// 0x73cf7c — __ZN3RBX4POLY4Mesh7addFaceEmmmm
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
pub fn stub_0x73cf7c() {
    // IDA 0x73cf7c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Mesh::addFace(unsigned long,unsigned long,unsigned long)")]
// 0x73d4a0 — __ZN3RBX4POLY4Mesh7addFaceEmmm
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, unsigned int, unsigned int, unsigned int)
pub fn stub_0x73d4a0() {
    // IDA 0x73d4a0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Mesh::addFace(int,int *,bool)")]
// 0x73e120 — __ZN3RBX4POLY4Mesh7addFaceEiPib
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, int, int *, bool)
pub fn stub_0x73e120() {
    // IDA 0x73e120: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Mesh::findOrMakeEdge(unsigned long,unsigned long)")]
// 0x73ebac — __ZN3RBX4POLY4Mesh14findOrMakeEdgeEmm
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, unsigned int, unsigned int)
pub fn stub_0x73ebac() {
    // IDA 0x73ebac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::initPlane(void)")]
// 0x73ed38 — __ZN3RBX4POLY4Face9initPlaneEv
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this)
pub fn stub_0x73ed38() {
    // IDA 0x73ed38: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Vertex::findEdge(RBX::POLY::Vertex const*)")]
// 0x73ed88 — __ZN3RBX4POLY6Vertex8findEdgeEPKS1_
pub fn stub_0x73ed88() {
    // IDA 0x73ed88: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Mesh::addEdge(RBX::POLY::Vertex *,RBX::POLY::Vertex *)")]
// 0x73ee28 — __ZN3RBX4POLY4Mesh7addEdgeEPNS0_6VertexES3_
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, RBX::POLY::Vertex *, RBX::POLY::Vertex *)
pub fn stub_0x73ee28() {
    // IDA 0x73ee28: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Vertex::getFace(unsigned long)const")]
// 0x73f16c — __ZNK3RBX4POLY6Vertex7getFaceEm
// type: _DWORD __fastcall(RBX::POLY::Vertex *__hidden this, unsigned int)
pub fn stub_0x73f16c() {
    // IDA 0x73f16c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::Face(unsigned long,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0x73f17c — __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_
pub fn stub_0x73f17c() {
    // IDA 0x73f17c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::Face(unsigned long,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0x73f288 — __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_S3_
pub fn stub_0x73f288() {
    // IDA 0x73f288: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::POLY::Face::Face(unsigned long,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> &)")]
// 0x73f3a8 — __ZN3RBX4POLY4FaceC2EmRSt6vectorIPNS0_4EdgeESaIS4_EE
pub fn stub_0x73f3a8() {
    // IDA 0x73f3a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Face::getCentroid(void)const")]
// 0x73fa80 — __ZNK3RBX4POLY4Face11getCentroidEv
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this)
pub fn stub_0x73fa80() {
    // IDA 0x73fa80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Vertex::recoverEdge(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
// 0x73ff5c — __ZN3RBX4POLY6Vertex11recoverEdgeEPKS1_S3_
// type: _DWORD __fastcall(RBX::POLY::Vertex *__hidden this, const RBX::POLY::Vertex *, const RBX::POLY::Vertex *)
pub fn stub_0x73ff5c() {
    // IDA 0x73ff5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::reserve(unsigned long)")]
// 0x7400a0 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm
// type: int(void)
pub fn stub_0x7400a0() {
    // IDA 0x7400a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::reserve(unsigned long)")]
// 0x74014c — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm
// type: int(void)
pub fn stub_0x74014c() {
    // IDA 0x74014c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::reserve(unsigned long)")]
// 0x7401f8 — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm
// type: int(void)
pub fn stub_0x7401f8() {
    // IDA 0x7401f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::push_back(RBX::POLY::Vertex const&)")]
// 0x740290 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x740290() {
    // IDA 0x740290: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::push_back(RBX::POLY::Face const&)")]
// 0x7402dc — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x7402dc() {
    // IDA 0x7402dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Edge::addFace(RBX::POLY::Face const*)")]
// 0x740314 — __ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE
pub fn stub_0x740314() {
    // IDA 0x740314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::push_back(RBX::POLY::Edge * const&)")]
// 0x7403c0 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x7403c0() {
    // IDA 0x7403c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::push_back(RBX::POLY::Edge const&)")]
// 0x7403ec — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_
// type: int(void)
pub fn stub_0x7403ec() {
    // IDA 0x7403ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Vertex::addEdge(RBX::POLY::Edge *)")]
// 0x74041c — __ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE
pub fn stub_0x74041c() {
    // IDA 0x74041c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Edge::getVertexFace(RBX::POLY::Vertex const*)const")]
// 0x74049c — __ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE
pub fn stub_0x74049c() {
    // IDA 0x74049c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Edge::otherFace(RBX::POLY::Face const*)const")]
// 0x740508 — __ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE
pub fn stub_0x740508() {
    // IDA 0x740508: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge*,std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>>,RBX::POLY::Edge const&)")]
// 0x7405cc — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x7405cc() {
    // IDA 0x7405cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_allocate(unsigned long)")]
// 0x740718 — __ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x740718() {
    // IDA 0x740718: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Edge * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Edge *,RBX::POLY::Edge *>(RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0x74073c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_0x74073c() {
    // IDA 0x74073c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<RBX::POLY::Face>::construct(RBX::POLY::Face*,RBX::POLY::Face const&)")]
// 0x74079c — __ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_
// type: int(void)
pub fn stub_0x74079c() {
    // IDA 0x74079c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Face*,std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>>,RBX::POLY::Face const&)")]
// 0x7407e4 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x7407e4() {
    // IDA 0x7407e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::Face::operator=(RBX::POLY::Face const&)")]
// 0x740bf8 — __ZN3RBX4POLY4FaceaSERKS1_
pub fn stub_0x740bf8() {
    // IDA 0x740bf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate(unsigned long)")]
// 0x740c20 — __ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x740c20() {
    // IDA 0x740c20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::operator=(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
// 0x740c44 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_
// type: int(void)
pub fn stub_0x740c44() {
    // IDA 0x740c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
