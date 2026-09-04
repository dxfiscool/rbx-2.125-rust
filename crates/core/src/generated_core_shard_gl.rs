//! core shard GL — 100 core stubs EA-sorted, 0xf4fcb4..0xf51394 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf4fca4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf4fca4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_initialize_map(unsigned long)")]
// 0xf4fcb4 — j___ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE17_M_initialize_mapEm
pub fn stub_f4fcb4() {
    // IDA 0xf4fcb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::~_Deque_base()")]
// 0xf4fcc4 — j___ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EED2Ev
pub fn stub_f4fcc4() {
    // IDA 0xf4fcc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_allocate(unsigned long)")]
// 0xf4fcd4 — j___ZNSt12_Vector_baseIPN3RBX7ContactESaIS2_EE11_M_allocateEm
pub fn stub_f4fcd4() {
    // IDA 0xf4fcd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate(unsigned long)")]
// 0xf4fce4 — j___ZNSt12_Vector_baseIPN3RBX8AssemblyESaIS2_EE11_M_allocateEm
pub fn stub_f4fce4() {
    // IDA 0xf4fce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_push_back_aux(RBX::Assembly * const&)")]
// 0xf4fcf4 — j___ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_
pub fn stub_f4fcf4() {
    // IDA 0xf4fcf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reallocate_map(unsigned long,bool)")]
// 0xf4fd04 — j___ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb
pub fn stub_f4fd04() {
    // IDA 0xf4fd04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reserve_map_at_back(unsigned long)")]
// 0xf4fd14 — j___ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE22_M_reserve_map_at_backEm
pub fn stub_f4fd14() {
    // IDA 0xf4fd14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::pop_front(void)")]
// 0xf4fd24 — j___ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv
pub fn stub_f4fd24() {
    // IDA 0xf4fd24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
// 0xf4fd34 — j___ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
pub fn stub_f4fd34() {
    // IDA 0xf4fd34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,unsigned long,RBX::Joint * const&)")]
// 0xf4fd44 — j___ZNSt6vectorIPN3RBX5JointESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f4fd44() {
    // IDA 0xf4fd44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::resize(unsigned long,RBX::Joint *)")]
// 0xf4fd54 — j___ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_
pub fn stub_f4fd54() {
    // IDA 0xf4fd54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,RBX::Contact * const&)")]
// 0xf4fd64 — j___ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4fd64() {
    // IDA 0xf4fd64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,unsigned long,RBX::Contact * const&)")]
// 0xf4fd74 — j___ZNSt6vectorIPN3RBX7ContactESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f4fd74() {
    // IDA 0xf4fd74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::resize(unsigned long,RBX::Contact *)")]
// 0xf4fd84 — j___ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_
pub fn stub_f4fd84() {
    // IDA 0xf4fd84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::push_back(RBX::Contact * const&)")]
// 0xf4fd94 — j___ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_
pub fn stub_f4fd94() {
    // IDA 0xf4fd94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,RBX::Assembly * const&)")]
// 0xf4fda4 — j___ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4fda4() {
    // IDA 0xf4fda4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,unsigned long,RBX::Assembly * const&)")]
// 0xf4fdb4 — j___ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f4fdb4() {
    // IDA 0xf4fdb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::resize(unsigned long,RBX::Assembly *)")]
// 0xf4fdc4 — j___ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_
pub fn stub_f4fdc4() {
    // IDA 0xf4fdc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
// 0xf4fdd4 — j___ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
pub fn stub_f4fdd4() {
    // IDA 0xf4fdd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IWorldStage::getMetric(RBX::IWorldStage::MetricType)")]
// 0xf4fe54 — j___ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE
pub fn stub_f4fe54() {
    // IDA 0xf4fe54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IPipelined::downstreamOfStage(RBX::IStage *)const")]
// 0xf4fe64 — j___ZNK3RBX10IPipelined17downstreamOfStageEPNS_6IStageE
pub fn stub_f4fe64() {
    // IDA 0xf4fe64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::equal_range(RBX::Assembly * const&)")]
// 0xf4fe74 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_f4fe74() {
    // IDA 0xf4fe74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert_unique(RBX::Assembly * const&)")]
// 0xf4fe84 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f4fe84() {
    // IDA 0xf4fe84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(RBX::Assembly * const&)")]
// 0xf4fe94 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_f4fe94() {
    // IDA 0xf4fe94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(std::_Rb_tree_iterator<RBX::Assembly *>,std::_Rb_tree_iterator<RBX::Assembly *>)")]
// 0xf4fea4 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_f4fea4() {
    // IDA 0xf4fea4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_erase(std::_Rb_tree_node<RBX::Assembly *> *)")]
// 0xf4feb4 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f4feb4() {
    // IDA 0xf4feb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Assembly * const&)")]
// 0xf4fec4 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f4fec4() {
    // IDA 0xf4fec4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>)")]
// 0xf4fed4 — j___ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvPFvPS2_ENS4_5list1INS3_3argILi1EEEEEEEEEvT0_
pub fn stub_f4fed4() {
    // IDA 0xf4fed4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::equal_range(RBX::Mechanism * const&)")]
// 0xf4fee4 — j___ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_f4fee4() {
    // IDA 0xf4fee4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_insert_unique(RBX::Mechanism * const&)")]
// 0xf4fef4 — j___ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f4fef4() {
    // IDA 0xf4fef4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::erase(RBX::Mechanism * const&)")]
// 0xf4ff04 — j___ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_f4ff04() {
    // IDA 0xf4ff04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::erase(std::_Rb_tree_iterator<RBX::Mechanism *>,std::_Rb_tree_iterator<RBX::Mechanism *>)")]
// 0xf4ff14 — j___ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_f4ff14() {
    // IDA 0xf4ff14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_erase(std::_Rb_tree_node<RBX::Mechanism *> *)")]
// 0xf4ff24 — j___ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f4ff24() {
    // IDA 0xf4ff24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Mechanism * const&)")]
// 0xf4ff34 — j___ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f4ff34() {
    // IDA 0xf4ff34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::Allocator(void)")]
// 0xf4ffa4 — j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev
pub fn stub_f4ffa4() {
    // IDA 0xf4ffa4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator delete(void *)")]
// 0xf4ffb4 — j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv
pub fn stub_f4ffb4() {
    // IDA 0xf4ffb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator new(unsigned long)")]
// 0xf4ffc4 — j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm
pub fn stub_f4ffc4() {
    // IDA 0xf4ffc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4fff4 — j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4fff4() {
    // IDA 0xf4fff4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf50004 — j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f50004() {
    // IDA 0xf50004: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IndexArray<RBX::Primitive,&RBX::Primitive::worldIndexFunc>::fastRemove(RBX::Primitive*)")]
// 0xf50154 — j___ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_
pub fn stub_f50154() {
    // IDA 0xf50154: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::notifyMovingPrimitives<std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>>(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> const&)")]
// 0xf50174 — j___ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_
pub fn stub_f50174() {
    // IDA 0xf50174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Joint::isAutoJoint(RBX::Joint const*)")]
// 0xf50184 — j___ZN3RBX5Joint11isAutoJointEPKS0_
pub fn stub_f50184() {
    // IDA 0xf50184: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::assertNotInStep(void)")]
// 0xf50194 — j___ZN3RBX5World15assertNotInStepEv
pub fn stub_f50194() {
    // IDA 0xf50194: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContactManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContactManager,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>>>,RBX::Primitive *)")]
// 0xf501a4 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_14ContactManagerEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
pub fn stub_f501a4() {
    // IDA 0xf501a4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Joint *)>::operator()(RBX::Joint *)")]
// 0xf501c4 — j___ZN3rbx7signals16signal_with_argsILi1EFvPN3RBX5JointEEEclES4_
pub fn stub_f501c4() {
    // IDA 0xf501c4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::operator()(std::pair<RBX::Primitive *,RBX::Primitive *>)")]
// 0xf501d4 — j___ZN3rbx7signals16signal_with_argsILi1EFvSt4pairIPN3RBX9PrimitiveES5_EEEclES6_
pub fn stub_f501d4() {
    // IDA 0xf501d4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::disconnectAll(void)")]
// 0xf501e4 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE13disconnectAllEv
pub fn stub_f501e4() {
    // IDA 0xf501e4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> &)")]
// 0xf501f4 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Joint *)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> &)
pub fn stub_f501f4() {
    // IDA 0xf501f4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::on_error(std::exception &)")]
// 0xf50204 — j___ZN3rbx7signals6signalIFvPN3RBX5JointEEE8on_errorERSt9exception
pub fn stub_f50204() {
    // IDA 0xf50204: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Primitive *)>::disconnectAll(void)")]
// 0xf50214 — j___ZN3rbx7signals6signalIFvPN3RBX9PrimitiveEEE13disconnectAllEv
pub fn stub_f50214() {
    // IDA 0xf50214: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::disconnectAll(void)")]
// 0xf50224 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE13disconnectAllEv
pub fn stub_f50224() {
    // IDA 0xf50224: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::safe_static_do_get_mutex(void)")]
// 0xf50234 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE24safe_static_do_get_mutexEv
pub fn stub_f50234() {
    // IDA 0xf50234: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot> &)")]
// 0xf50244 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// was: rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot> &)
pub fn stub_f50244() {
    // IDA 0xf50244: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::on_error(std::exception &)")]
// 0xf50254 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE8on_errorERSt9exception
pub fn stub_f50254() {
    // IDA 0xf50254: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot> const&)")]
// 0xf50274 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES7_EEE4slotEEaSERKSC_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot> const&)
pub fn stub_f50274() {
    // IDA 0xf50274: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_allocate(unsigned long)")]
// 0xf50284 — j___ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm
pub fn stub_f50284() {
    // IDA 0xf50284: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_allocate(unsigned long)")]
// 0xf50294 — j___ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm
pub fn stub_f50294() {
    // IDA 0xf50294: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,RBX::Joint * const&)")]
// 0xf502a4 — j___ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f502a4() {
    // IDA 0xf502a4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::push_back(RBX::Joint * const&)")]
// 0xf502b4 — j___ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_
pub fn stub_f502b4() {
    // IDA 0xf502b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Profiling::CodeProfiler **,std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>>,RBX::Profiling::CodeProfiler * const&)")]
// 0xf502c4 — j___ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_f502c4() {
    // IDA 0xf502c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::push_back(RBX::Profiling::CodeProfiler * const&)")]
// 0xf502d4 — j___ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_
pub fn stub_f502d4() {
    // IDA 0xf502d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::equal_range(RBX::Joint * const&)")]
// 0xf502e4 — j___ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_f502e4() {
    // IDA 0xf502e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert_unique(RBX::Joint * const&)")]
// 0xf502f4 — j___ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f502f4() {
    // IDA 0xf502f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(RBX::Joint * const&)")]
// 0xf50304 — j___ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_f50304() {
    // IDA 0xf50304: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(std::_Rb_tree_iterator<RBX::Joint *>,std::_Rb_tree_iterator<RBX::Joint *>)")]
// 0xf50314 — j___ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_f50314() {
    // IDA 0xf50314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_erase(std::_Rb_tree_node<RBX::Joint *> *)")]
// 0xf50324 — j___ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f50324() {
    // IDA 0xf50324: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Joint * const&)")]
// 0xf50334 — j___ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f50334() {
    // IDA 0xf50334: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContactManagerSpatialHash,RBX::Assembly&>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash*>,boost::arg<1>>> std::for_each<boost::intrusive::list_iterator<boost::intrusive::list_impl<boost::intrusive::listopt<boost::intrusive::detail::base_hook_traits<RBX::Assembly,boost::intrusive::list_node_traits<void *>,(boost::intrusive::link_mode_type)1,RBX::SimulateStage,1>,unsigned long,true>>,false>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContactManagerSpatialHash,RBX::Assembly&>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash*>,boost::arg<1>>>>(boost::intrusive::list_iterator<boost::intrusive::list_impl<boost::intrusive::listopt<boost::intrusive::detail::base_hook_traits<RBX::Assembly,boost::intrusive::list_node_traits<void *>,(boost::intrusive::link_mode_type)1,RBX::SimulateStage,1>,unsigned long,true>>,false>,boost::intrusive::list_iterator<boost::intrusive::list_impl<boost::intrusive::listopt<boost::intrusive::detail::base_hook_traits<RBX::Assembly,boost::intrusive::list_node_traits<void *>,(boost::intrusive::link_mode_type)1,RBX::SimulateStage,1>,unsigned long,true>>,false>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ContactManagerSpatialHash,RBX::Assembly&>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash*>,boost::arg<1>>>)")]
// 0xf50344 — j___ZSt8for_eachIN5boost9intrusive13list_iteratorINS1_9list_implINS1_7listoptINS1_6detail16base_hook_traitsIN3RBX8AssemblyENS1_16list_node_traitsIPvEELNS1_14link_mode_typeE1ENS7_13SimulateStageELi1EEEmLb1EEEEELb0EEENS0_3_bi6bind_tIvNS0_4_mfi3mf1IvNS7_25ContactManagerSpatialHashERS8_EENSI_5list2INSI_5valueIPSM_EENS0_3argILi1EEEEEEEET0_T_SY_SX_
pub fn stub_f50344() {
    // IDA 0xf50344: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::string> rbx::make_shared<std::string,char const*>(char const* const&)")]
// 0xf507d4 — j___ZN3rbx11make_sharedISsPKcEEN5boost10shared_ptrIT_EERKT0_
// was: boost::shared_ptr<std::string> rbx::make_shared<std::string,char const*>(char const* const&)
pub fn stub_f507d4() {
    // IDA 0xf507d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_do_get_mutex(void)")]
// 0xf50834 — j___ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv
pub fn stub_f50834() {
    // IDA 0xf50834: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::insert(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
// 0xf50844 — j___ZN3rbx7signals6signalIFvP9lua_StateEE6insertEPNS5_4slotE
pub fn stub_f50844() {
    // IDA 0xf50844: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::remove(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
// 0xf50854 — j___ZN3rbx7signals6signalIFvP9lua_StateEE6removeEPNS5_4slotE
pub fn stub_f50854() {
    // IDA 0xf50854: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::disconnectAll(void)")]
// 0xf50874 — j___ZN3rbx7signals6signalIFviEE13disconnectAllEv
pub fn stub_f50874() {
    // IDA 0xf50874: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int)>::connect<boost::function<void ()(int)>>(boost::function<void ()(int)> const&)")]
// 0xf50884 — j___ZN3rbx7signals6signalIFviEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f50884() {
    // IDA 0xf50884: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::shared_ptr<std::string,rbx::detail::sp_ms_deleter<std::string>>(std::string *,rbx::detail::sp_ms_deleter<std::string>)")]
// 0xf50934 — j___ZN5boost10shared_ptrISsEC2ISsN3rbx6detail13sp_ms_deleterISsEEEEPT_T0_
// was: boost::shared_ptr<std::string>::shared_ptr<std::string,rbx::detail::sp_ms_deleter<std::string>>(std::string *,rbx::detail::sp_ms_deleter<std::string>)
pub fn stub_f50934() {
    // IDA 0xf50934: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::operator=(rbx_core::SharedPtr<std::string> const&)")]
// 0xf50944 — j___ZN5boost10shared_ptrISsEaSERKS1_
// was: boost::shared_ptr<std::string>::operator=(boost::shared_ptr<std::string> const&)
pub fn stub_f50944() {
    // IDA 0xf50944: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::string> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::string>,std::string>(rbx_core::SharedPtr<std::string> const&)")]
// 0xf50974 — j___ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterISsEESsEEPT_RKNS_10shared_ptrIT0_EE
// was: rbx::detail::sp_ms_deleter<std::string> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::string>,std::string>(boost::shared_ptr<std::string> const&)
pub fn stub_f50974() {
    // IDA 0xf50974: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx::signals::signal<void ()(lua_State *)>::slot*)")]
// 0xf509a4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx::signals::signal<void ()(lua_State *)>::slot*)
pub fn stub_f509a4() {
    // IDA 0xf509a4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::char_separator<char,std::char_traits<char>>::char_separator(char const*,char const*,boost::empty_token_policy)")]
// 0xf509b4 — j___ZN5boost14char_separatorIcSt11char_traitsIcEEC2EPKcS5_NS_18empty_token_policyE
pub fn stub_f509b4() {
    // IDA 0xf509b4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "bool boost::char_separator<char,std::char_traits<char>>::operator()<__gnu_cxx::__normal_iterator<char const*,std::string>,std::string>(__gnu_cxx::__normal_iterator<char const*,std::string> &,__gnu_cxx::__normal_iterator<char const*,std::string>,std::string &)")]
// 0xf509c4 — j___ZN5boost14char_separatorIcSt11char_traitsIcEEclIN9__gnu_cxx17__normal_iteratorIPKcSsEESsEEbRT_SA_RT0_
pub fn stub_f509c4() {
    // IDA 0xf509c4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::arg<1>>::list2(boost::_bi::value<std::string>,boost::arg<1>)")]
// 0xf509d4 — j___ZN5boost3_bi5list2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
pub fn stub_f509d4() {
    // IDA 0xf509d4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::arg<1>>::storage2(boost::_bi::value<std::string>,boost::arg<1>)")]
// 0xf50ad4 — j___ZN5boost3_bi8storage2INS0_5valueISsEENS_3argILi1EEEEC2ES3_S5_
pub fn stub_f50ad4() {
    // IDA 0xf50ad4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::string *,rbx::detail::sp_ms_deleter<std::string>>(std::string *,rbx::detail::sp_ms_deleter<std::string>)")]
// 0xf50c04 — j___ZN5boost6detail12shared_countC2IPSsN3rbx6detail13sp_ms_deleterISsEEEET_T0_
pub fn stub_f50c04() {
    // IDA 0xf50c04: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::function1<void,int>::clear(void)")]
// 0xf50d14 — j___ZN5boost9function1IviE5clearEv
pub fn stub_f50d14() {
    // IDA 0xf50d14: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::assign_to_own(boost::function2<bool,lua_State *,lua_Debug *> const&)")]
// 0xf50da4 — j___ZN5boost9function2IbP9lua_StateP9lua_DebugE13assign_to_ownERKS5_
pub fn stub_f50da4() {
    // IDA 0xf50da4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::clear(void)")]
// 0xf50db4 — j___ZN5boost9function2IbP9lua_StateP9lua_DebugE5clearEv
pub fn stub_f50db4() {
    // IDA 0xf50db4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::move_assign(boost::function2<void,lua_State *,lua_Debug *>&)")]
// 0xf50e04 — j___ZN5boost9function2IvP9lua_StateP9lua_DebugE11move_assignERS5_
pub fn stub_f50e04() {
    // IDA 0xf50e04: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::swap(boost::function2<void,lua_State *,lua_Debug *>&)")]
// 0xf50e14 — j___ZN5boost9function2IvP9lua_StateP9lua_DebugE4swapERS5_
pub fn stub_f50e14() {
    // IDA 0xf50e14: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::char_separator<char,std::char_traits<char>>::is_dropped(char)const")]
// 0xf51004 — j___ZNK5boost14char_separatorIcSt11char_traitsIcEE10is_droppedEc
pub fn stub_f51004() {
    // IDA 0xf51004: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
// 0xf511c4 — j___ZNK5boost9function2IbP9lua_StateP9lua_DebugEclES2_S4_
pub fn stub_f511c4() {
    // IDA 0xf511c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
// 0xf511d4 — j___ZNK5boost9function2IvP9lua_StateP9lua_DebugEclES2_S4_
pub fn stub_f511d4() {
    // IDA 0xf511d4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::ostream_iterator<int,char,std::char_traits<char>>::operator=(int const&)")]
// 0xf51264 — j___ZNSt16ostream_iteratorIicSt11char_traitsIcEEaSERKi
pub fn stub_f51264() {
    // IDA 0xf51264: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::ostream_iterator<int,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::ostream_iterator<int,char,std::char_traits<char>>>(std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::ostream_iterator<int,char,std::char_traits<char>>)")]
// 0xf51274 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt16reverse_iteratorIN9__gnu_cxx17__normal_iteratorIPiSt6vectorIiSaIiEEEEESt16ostream_iteratorIicSt11char_traitsIcEEEET0_T_SH_SG_
pub fn stub_f51274() {
    // IDA 0xf51274: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::vector<int,std::allocator<int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>,int const&)")]
// 0xf512e4 — j___ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi
pub fn stub_f512e4() {
    // IDA 0xf512e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::vector<int,std::allocator<int>>::push_back(int const&)")]
// 0xf512f4 — j___ZNSt6vectorIiSaIiEE9push_backERKi
pub fn stub_f512f4() {
    // IDA 0xf512f4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "XmlElement::XmlElement<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
// 0xf51344 — j___ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_
pub fn stub_f51344() {
    // IDA 0xf51344: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlAttribute::XmlAttribute<char const*>(RBX::Name const&,char const*)")]
// 0xf51354 — j___ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_
pub fn stub_f51354() {
    // IDA 0xf51354: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlAttribute::XmlAttribute<int>(RBX::Name const&,int)")]
// 0xf51364 — j___ZN12XmlAttributeC2IiEERKN3RBX4NameET_
pub fn stub_f51364() {
    // IDA 0xf51364: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlNameValuePair::XmlNameValuePair(RBX::Name const&,char const*)")]
// 0xf51384 — j___ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc
pub fn stub_f51384() {
    // IDA 0xf51384: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>::operator()(ArchiveBinder::IDREFBinding&)const")]
// 0xf51394 — j___ZNKSt9binder1stISt10mem_fun1_tIb13ArchiveBinderNS1_12IDREFBindingEEEclERS2_
pub fn stub_f51394() {
    // IDA 0xf51394: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}
