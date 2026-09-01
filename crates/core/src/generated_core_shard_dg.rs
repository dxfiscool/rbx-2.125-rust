//! core shard DG — 100 core stubs EA-sorted, next uncovered after DF 0x763610 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::World::getUiStepId(void)")]
// 0x763a84 — __ZN3RBX5World11getUiStepIdEv
pub fn stub_763a84() -> ! {
    todo!("0x763a84 __ZN3RBX5World11getUiStepIdEv")
}

#[doc(alias = "RBX::World::step(bool,double,float,int)")]
// 0x763aa0 — __ZN3RBX5World4stepEbdfi
pub fn stub_763aa0() -> ! {
    todo!("0x763aa0 __ZN3RBX5World4stepEbdfi")
}

#[doc(alias = "RBX::World::reportTouchInfo(RBX::World::TouchInfo const&)")]
// 0x764044 — __ZN3RBX5World15reportTouchInfoERKNS0_9TouchInfoE
pub fn stub_764044() -> ! {
    todo!("0x764044 __ZN3RBX5World15reportTouchInfoERKNS0_9TouchInfoE")
}

#[doc(alias = "RBX::World::onPrimitiveCollided(RBX::Primitive *,RBX::Primitive *)")]
// 0x76404c — __ZN3RBX5World19onPrimitiveCollidedEPNS_9PrimitiveES2_
pub fn stub_76404c() -> ! {
    todo!("0x76404c __ZN3RBX5World19onPrimitiveCollidedEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::World::insertJoint(RBX::Joint *)")]
// 0x764230 — __ZN3RBX5World11insertJointEPNS_5JointE
pub fn stub_764230() -> ! {
    todo!("0x764230 __ZN3RBX5World11insertJointEPNS_5JointE")
}

#[doc(alias = "RBX::World::destroyJoint(RBX::Joint *)")]
// 0x7643d8 — __ZN3RBX5World12destroyJointEPNS_5JointE
pub fn stub_7643d8() -> ! {
    todo!("0x7643d8 __ZN3RBX5World12destroyJointEPNS_5JointE")
}

#[doc(alias = "RBX::World::removeFromBreakable(RBX::Joint *)")]
// 0x764440 — __ZN3RBX5World19removeFromBreakableEPNS_5JointE
pub fn stub_764440() -> ! {
    todo!("0x764440 __ZN3RBX5World19removeFromBreakableEPNS_5JointE")
}

#[doc(alias = "RBX::World::removeJoint(RBX::Joint *)")]
// 0x7644b8 — __ZN3RBX5World11removeJointEPNS_5JointE
pub fn stub_7644b8() -> ! {
    todo!("0x7644b8 __ZN3RBX5World11removeJointEPNS_5JointE")
}

#[doc(alias = "RBX::World::notifyMoved(RBX::Primitive *)")]
// 0x7644e0 — __ZN3RBX5World11notifyMovedEPNS_9PrimitiveE
pub fn stub_7644e0() -> ! {
    todo!("0x7644e0 __ZN3RBX5World11notifyMovedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::World::jointCoordsChanged(RBX::Joint *)")]
// 0x764528 — __ZN3RBX5World18jointCoordsChangedEPNS_5JointE
pub fn stub_764528() -> ! {
    todo!("0x764528 __ZN3RBX5World18jointCoordsChangedEPNS_5JointE")
}

#[doc(alias = "RBX::World::insertContact(RBX::Contact *)")]
// 0x7646b4 — __ZN3RBX5World13insertContactEPNS_7ContactE
pub fn stub_7646b4() -> ! {
    todo!("0x7646b4 __ZN3RBX5World13insertContactEPNS_7ContactE")
}

#[doc(alias = "RBX::World::destroyContact(RBX::Contact *)")]
// 0x7646cc — __ZN3RBX5World14destroyContactEPNS_7ContactE
pub fn stub_7646cc() -> ! {
    todo!("0x7646cc __ZN3RBX5World14destroyContactEPNS_7ContactE")
}

#[doc(alias = "RBX::World::joinAll(void)")]
// 0x764748 — __ZN3RBX5World7joinAllEv
pub fn stub_764748() -> ! {
    todo!("0x764748 __ZN3RBX5World7joinAllEv")
}

#[doc(alias = "RBX::World::createAutoJoints(RBX::Primitive *)")]
// 0x764854 — __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveE
pub fn stub_764854() -> ! {
    todo!("0x764854 __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::World::insertPrimitive(RBX::Primitive *)")]
// 0x76485c — __ZN3RBX5World15insertPrimitiveEPNS_9PrimitiveE
pub fn stub_76485c() -> ! {
    todo!("0x76485c __ZN3RBX5World15insertPrimitiveEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::World::removePrimitive(RBX::Primitive *,bool)")]
// 0x764b38 — __ZN3RBX5World15removePrimitiveEPNS_9PrimitiveEb
pub fn stub_764b38() -> ! {
    todo!("0x764b38 __ZN3RBX5World15removePrimitiveEPNS_9PrimitiveEb")
}

#[doc(alias = "RBX::World::destroyAutoJoints(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,bool,bool)")]
// 0x764e34 — __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EEbb
pub fn stub_764e34() -> ! {
    todo!("0x764e34 __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EEbb")
}

#[doc(alias = "RBX::doNotIgnore(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *)")]
// 0x7651e8 — __ZN3RBX11doNotIgnoreEPNS_9PrimitiveEPSt3setIS1_St4lessIS1_ESaIS1_EES7_
pub fn stub_7651e8() -> ! {
    todo!("0x7651e8 __ZN3RBX11doNotIgnoreEPNS_9PrimitiveEPSt3setIS1_St4lessIS1_ESaIS1_EES7_")
}

#[doc(alias = "RBX::World::destroyAutoJoints(RBX::Primitive *,bool)")]
// 0x765414 — __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEb
pub fn stub_765414() -> ! {
    todo!("0x765414 __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEb")
}

#[doc(alias = "RBX::World::createAutoJoints(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *)")]
// 0x7655a0 — __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EES8_
pub fn stub_7655a0() -> ! {
    todo!("0x7655a0 __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EES8_")
}

#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::push_back(RBX::Profiling::CodeProfiler * const&)")]
// 0x765980 — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_
pub fn stub_765980() -> ! {
    todo!("0x765980 __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "void RBX::notifyMovingPrimitives<std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>>(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> const&)")]
// 0x7659ac — __ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_
pub fn stub_7659ac() -> ! {
    todo!("0x7659ac __ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_")
}

#[doc(alias = "RBX::World::assertNotInStep(void)")]
// 0x765c1c — __ZN3RBX5World15assertNotInStepEv
pub fn stub_765c1c() -> ! {
    todo!("0x765c1c __ZN3RBX5World15assertNotInStepEv")
}

#[doc(alias = "RBX::IndexArray<RBX::Primitive,&RBX::Primitive::worldIndexFunc>::fastRemove(RBX::Primitive*)")]
// 0x765dc4 — __ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_
pub fn stub_765dc4() -> ! {
    todo!("0x765dc4 __ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::Joint::isAutoJoint(RBX::Joint const*)")]
// 0x765e9c — __ZN3RBX5Joint11isAutoJointEPKS0_
pub fn stub_765e9c() -> ! {
    todo!("0x765e9c __ZN3RBX5Joint11isAutoJointEPKS0_")
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::push_back(RBX::Joint * const&)")]
// 0x766074 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_
pub fn stub_766074() -> ! {
    todo!("0x766074 __ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,RBX::Joint * const&)")]
// 0x7660a0 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7660a0() -> ! {
    todo!("0x7660a0 __ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_allocate(unsigned long)")]
// 0x766180 — __ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm
pub fn stub_766180() -> ! {
    todo!("0x766180 __ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(RBX::Joint * const&)")]
// 0x7667fc — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_7667fc() -> ! {
    todo!("0x7667fc __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::equal_range(RBX::Joint * const&)")]
// 0x766824 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_766824() -> ! {
    todo!("0x766824 __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(std::_Rb_tree_iterator<RBX::Joint *>,std::_Rb_tree_iterator<RBX::Joint *>)")]
// 0x766870 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_766870() -> ! {
    todo!("0x766870 __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_erase(std::_Rb_tree_node<RBX::Joint *> *)")]
// 0x7668d0 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_7668d0() -> ! {
    todo!("0x7668d0 __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert_unique(RBX::Joint * const&)")]
// 0x7668f8 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_7668f8() -> ! {
    todo!("0x7668f8 __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Joint * const&)")]
// 0x766960 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_766960() -> ! {
    todo!("0x766960 __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Profiling::CodeProfiler **,std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>>,RBX::Profiling::CodeProfiler * const&)")]
// 0x766d1c — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_766d1c() -> ! {
    todo!("0x766d1c __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::_Vector_base<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_allocate(unsigned long)")]
// 0x766dfc — __ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm
pub fn stub_766dfc() -> ! {
    todo!("0x766dfc __ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "readWatchValue(std::string,lua_State *)")]
// 0x76cd58 — __ZL14readWatchValueSsP9lua_State
pub fn stub_76cd58() -> ! {
    todo!("0x76cd58 __ZL14readWatchValueSsP9lua_State")
}

#[doc(alias = "std::vector<int,std::allocator<int>>::push_back(int const&)")]
// 0x774260 — __ZNSt6vectorIiSaIiEE9push_backERKi
pub fn stub_774260() -> ! {
    todo!("0x774260 __ZNSt6vectorIiSaIiEE9push_backERKi")
}

#[doc(alias = "std::ostream_iterator<int,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::ostream_iterator<int,char,std::char_traits<char>>>(std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::ostream_iterator<int,char,std::char_traits<char>>)")]
// 0x775d3c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt16reverse_iteratorIN9__gnu_cxx17__normal_iteratorIPiSt6vectorIiSaIiEEEEESt16ostream_iteratorIicSt11char_traitsIcEEEET0_T_SH_SG_
pub fn stub_775d3c() -> ! {
    todo!("0x775d3c __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt16reverse_iteratorIN9__gnu_cxx17__normal_iteratorIPiSt6vectorIiSaIiEEEEESt16ostream_iteratorIicSt11char_traitsIcEEEET0_T_SH_SG_")
}

#[doc(alias = "std::ostream_iterator<int,char,std::char_traits<char>>::operator=(int const&)")]
// 0x775d80 — __ZNSt16ostream_iteratorIicSt11char_traitsIcEEaSERKi
pub fn stub_775d80() -> ! {
    todo!("0x775d80 __ZNSt16ostream_iteratorIicSt11char_traitsIcEEaSERKi")
}

#[doc(alias = "std::vector<int,std::allocator<int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>,int const&)")]
// 0x775da8 — __ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi
pub fn stub_775da8() -> ! {
    todo!("0x775da8 __ZNSt6vectorIiSaIiEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS1_EERKi")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::insert(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
// 0x77a7dc — __ZN3rbx7signals6signalIFvP9lua_StateEE6insertEPNS5_4slotE
pub fn stub_77a7dc() -> ! {
    todo!("0x77a7dc __ZN3rbx7signals6signalIFvP9lua_StateEE6insertEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_init_mutex(void)")]
// 0x77aa10 — __ZN3rbx7signals6signalIFvP9lua_StateEE22safe_static_init_mutexEv
pub fn stub_77aa10() -> ! {
    todo!("0x77aa10 __ZN3rbx7signals6signalIFvP9lua_StateEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::disconnect(void)")]
// 0x77ab14 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot10disconnectEv
pub fn stub_77ab14() -> ! {
    todo!("0x77ab14 __ZN3rbx7signals6signalIFvP9lua_StateEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::connected(void)const")]
// 0x77ac24 — __ZNK3rbx7signals6signalIFvP9lua_StateEE4slot9connectedEv
pub fn stub_77ac24() -> ! {
    todo!("0x77ac24 __ZNK3rbx7signals6signalIFvP9lua_StateEE4slot9connectedEv")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::remove(rbx::signals::signal<void ()(lua_State *)>::slot *)")]
// 0x77ac70 — __ZN3rbx7signals6signalIFvP9lua_StateEE6removeEPNS5_4slotE
pub fn stub_77ac70() -> ! {
    todo!("0x77ac70 __ZN3rbx7signals6signalIFvP9lua_StateEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_init_mutex(void)")]
// 0x77ad60 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot22safe_static_init_mutexEv
pub fn stub_77ad60() -> ! {
    todo!("0x77ad60 __ZN3rbx7signals6signalIFvP9lua_StateEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::safe_static_do_get_mutex(void)")]
// 0x77ad64 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv
pub fn stub_77ad64() -> ! {
    todo!("0x77ad64 __ZN3rbx7signals6signalIFvP9lua_StateEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::~slot()")]
// 0x77ae54 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slotD1Ev
pub fn stub_77ae54() -> ! {
    todo!("0x77ae54 __ZN3rbx7signals6signalIFvP9lua_StateEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::~slot()")]
// 0x77ae80 — __ZN3rbx7signals6signalIFvP9lua_StateEE4slotD0Ev
pub fn stub_77ae80() -> ! {
    todo!("0x77ae80 __ZN3rbx7signals6signalIFvP9lua_StateEE4slotD0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::disconnectAll(void)")]
// 0x780eb0 — __ZN3rbx7signals6signalIFviEE13disconnectAllEv
pub fn stub_780eb0() -> ! {
    todo!("0x780eb0 __ZN3rbx7signals6signalIFviEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(int)>::slot::~slot()")]
// 0x7819f0 — __ZN3rbx7signals6signalIFviEE4slotD0Ev
pub fn stub_7819f0() -> ! {
    todo!("0x7819f0 __ZN3rbx7signals6signalIFviEE4slotD0Ev")
}

#[doc(alias = "SerializerV2::newRootElement(std::string const&)")]
// 0x789a38 — __ZN12SerializerV214newRootElementERKSs
pub fn stub_789a38() -> ! {
    todo!("0x789a38 __ZN12SerializerV214newRootElementERKSs")
}

#[doc(alias = "XmlElement::XmlElement<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
// 0x78a098 — __ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_
pub fn stub_78a098() -> ! {
    todo!("0x78a098 __ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_")
}

#[doc(alias = "XmlAttribute::XmlAttribute<int>(RBX::Name const&,int)")]
// 0x78a16c — __ZN12XmlAttributeC2IiEERKN3RBX4NameET_
pub fn stub_78a16c() -> ! {
    todo!("0x78a16c __ZN12XmlAttributeC2IiEERKN3RBX4NameET_")
}

#[doc(alias = "XmlAttribute::XmlAttribute<char const*>(RBX::Name const&,char const*)")]
// 0x78a230 — __ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_
pub fn stub_78a230() -> ! {
    todo!("0x78a230 __ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_")
}

#[doc(alias = "XmlNameValuePair::XmlNameValuePair(RBX::Name const&,char const*)")]
// 0x78a2ec — __ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc
pub fn stub_78a2ec() -> ! {
    todo!("0x78a2ec __ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc")
}

#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::push_back(MemoryBinder::IDREFItem const&)")]
// 0x78a824 — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE9push_backERKS1_
pub fn stub_78a824() -> ! {
    todo!("0x78a824 __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<MemoryBinder::IDREFItem*,std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>>,MemoryBinder::IDREFItem const&)")]
// 0x78a880 — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_78a880() -> ! {
    todo!("0x78a880 __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::_M_allocate(unsigned long)")]
// 0x78ac98 — __ZNSt12_Vector_baseIN12MemoryBinder9IDREFItemESaIS1_EE11_M_allocateEm
pub fn stub_78ac98() -> ! {
    todo!("0x78ac98 __ZNSt12_Vector_baseIN12MemoryBinder9IDREFItemESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "MemoryBinder::IDREFItem * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<MemoryBinder::IDREFItem *,MemoryBinder::IDREFItem *>(MemoryBinder::IDREFItem *,MemoryBinder::IDREFItem *,MemoryBinder::IDREFItem *)")]
// 0x78acb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN12MemoryBinder9IDREFItemES5_EET0_T_S7_S6_
pub fn stub_78acb0() -> ! {
    todo!("0x78acb0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN12MemoryBinder9IDREFItemES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::_M_erase_at_end(MemoryBinder::IDREFItem*)")]
// 0x78b274 — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE15_M_erase_at_endEPS1_
pub fn stub_78b274() -> ! {
    todo!("0x78b274 __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE15_M_erase_at_endEPS1_")
}

#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::~vector()")]
// 0x78b3ec — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EED2Ev
pub fn stub_78b3ec() -> ! {
    todo!("0x78b3ec __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EED2Ev")
}

#[doc(alias = "std::iterator_traits<std::_List_iterator<ArchiveBinder::IDREFBinding>>::difference_type std::count_if<std::_List_iterator<ArchiveBinder::IDREFBinding>,std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>>(std::_List_iterator<ArchiveBinder::IDREFBinding>,std::_List_iterator<ArchiveBinder::IDREFBinding>,std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>)")]
// 0x78bec4 — __ZSt8count_ifISt14_List_iteratorIN13ArchiveBinder12IDREFBindingEESt9binder1stISt10mem_fun1_tIbS1_S2_EEENSt15iterator_traitsIT_E15difference_typeES9_S9_T0_
pub fn stub_78bec4() -> ! {
    todo!("0x78bec4 __ZSt8count_ifISt14_List_iteratorIN13ArchiveBinder12IDREFBindingEESt9binder1stISt10mem_fun1_tIbS1_S2_EEENSt15iterator_traitsIT_E15difference_typeES9_S9_T0_")
}

#[doc(alias = "std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>::operator()(ArchiveBinder::IDREFBinding&)const")]
// 0x78c288 — __ZNKSt9binder1stISt10mem_fun1_tIb13ArchiveBinderNS1_12IDREFBindingEEEclERS2_
pub fn stub_78c288() -> ! {
    todo!("0x78c288 __ZNKSt9binder1stISt10mem_fun1_tIb13ArchiveBinderNS1_12IDREFBindingEEEclERS2_")
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::push_back(unsigned long const&)")]
// 0x78ec1c — __ZNSt6vectorImSaImEE9push_backERKm
pub fn stub_78ec1c() -> ! {
    todo!("0x78ec1c __ZNSt6vectorImSaImEE9push_backERKm")
}

#[doc(alias = "std::bitset<256ul>::test(unsigned long)const")]
// 0x792e78 — __ZNKSt6bitsetILm256EE4testEm
pub fn stub_792e78() -> ! {
    todo!("0x792e78 __ZNKSt6bitsetILm256EE4testEm")
}

#[doc(alias = "std::bitset<256ul>::set(unsigned long,bool)")]
// 0x7936f4 — __ZNSt6bitsetILm256EE3setEmb
pub fn stub_7936f4() -> ! {
    todo!("0x7936f4 __ZNSt6bitsetILm256EE3setEmb")
}

#[doc(alias = "char * std::string::_S_construct<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,std::allocator<char> const&,std::forward_iterator_tag)")]
// 0x794084 — __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEES2_T_S7_RKS4_St20forward_iterator_tag
pub fn stub_794084() -> ! {
    todo!("0x794084 __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEES2_T_S7_RKS4_St20forward_iterator_tag")
}

#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::reserve(unsigned long)")]
// 0x79630c — __ZNSt6vectorImSaImEE7reserveEm
pub fn stub_79630c() -> ! {
    todo!("0x79630c __ZNSt6vectorImSaImEE7reserveEm")
}

#[doc(alias = "void std::vector<char,std::allocator<char>>::_M_range_initialize<std::istreambuf_iterator<char,std::char_traits<char>>>(std::istreambuf_iterator<char,std::char_traits<char>>,std::istreambuf_iterator<char,std::char_traits<char>>,std::input_iterator_tag)")]
// 0x796d60 — __ZNSt6vectorIcSaIcEE19_M_range_initializeISt19istreambuf_iteratorIcSt11char_traitsIcEEEEvT_S7_St18input_iterator_tag
pub fn stub_796d60() -> ! {
    todo!("0x796d60 __ZNSt6vectorIcSaIcEE19_M_range_initializeISt19istreambuf_iteratorIcSt11char_traitsIcEEEEvT_S7_St18input_iterator_tag")
}

#[doc(alias = "std::vector<char,std::allocator<char>>::push_back(char const&)")]
// 0x796dd0 — __ZNSt6vectorIcSaIcEE9push_backERKc
pub fn stub_796dd0() -> ! {
    todo!("0x796dd0 __ZNSt6vectorIcSaIcEE9push_backERKc")
}

#[doc(alias = "std::istreambuf_iterator<char,std::char_traits<char>>::_M_get(void)const")]
// 0x796dfc — __ZNKSt19istreambuf_iteratorIcSt11char_traitsIcEE6_M_getEv
pub fn stub_796dfc() -> ! {
    todo!("0x796dfc __ZNKSt19istreambuf_iteratorIcSt11char_traitsIcEE6_M_getEv")
}

#[doc(alias = "std::istreambuf_iterator<char,std::char_traits<char>>::equal(std::istreambuf_iterator<char,std::char_traits<char>> const&)const")]
// 0x796e3c — __ZNKSt19istreambuf_iteratorIcSt11char_traitsIcEE5equalERKS2_
pub fn stub_796e3c() -> ! {
    todo!("0x796e3c __ZNKSt19istreambuf_iteratorIcSt11char_traitsIcEE5equalERKS2_")
}

#[doc(alias = "RBX::Allocator<XmlElement>::operator delete(void *)")]
// 0x7986c0 — __ZN3RBX9AllocatorI10XmlElementEdlEPv
pub fn stub_7986c0() -> ! {
    todo!("0x7986c0 __ZN3RBX9AllocatorI10XmlElementEdlEPv")
}

#[doc(alias = "XmlElement::findAttribute(RBX::Name const&)const")]
// 0x79894c — __ZNK10XmlElement13findAttributeERKN3RBX4NameE
pub fn stub_79894c() -> ! {
    todo!("0x79894c __ZNK10XmlElement13findAttributeERKN3RBX4NameE")
}

#[doc(alias = "XmlElement::findFirstChildByTag(RBX::Name const&)const")]
// 0x7989a4 — __ZNK10XmlElement19findFirstChildByTagERKN3RBX4NameE
pub fn stub_7989a4() -> ! {
    todo!("0x7989a4 __ZNK10XmlElement19findFirstChildByTagERKN3RBX4NameE")
}

#[doc(alias = "XmlElement::findAttribute(RBX::Name const&)")]
// 0x7989d4 — __ZN10XmlElement13findAttributeERKN3RBX4NameE
pub fn stub_7989d4() -> ! {
    todo!("0x7989d4 __ZN10XmlElement13findAttributeERKN3RBX4NameE")
}

#[doc(alias = "XmlNameValuePair::isValueEqual(RBX::Name const*)const")]
// 0x798af0 — __ZNK16XmlNameValuePair12isValueEqualEPKN3RBX4NameE
pub fn stub_798af0() -> ! {
    todo!("0x798af0 __ZNK16XmlNameValuePair12isValueEqualEPKN3RBX4NameE")
}

#[doc(alias = "XmlNameValuePair::getValue(RBX::Name const*&)const")]
// 0x798b24 — __ZNK16XmlNameValuePair8getValueERPKN3RBX4NameE
pub fn stub_798b24() -> ! {
    todo!("0x798b24 __ZNK16XmlNameValuePair8getValueERPKN3RBX4NameE")
}

#[doc(alias = "bool XmlNameValuePair::isValueType<RBX::ContentId>(void)const")]
// 0x798b64 — __ZNK16XmlNameValuePair11isValueTypeIN3RBX9ContentIdEEEbv
pub fn stub_798b64() -> ! {
    todo!("0x798b64 __ZNK16XmlNameValuePair11isValueTypeIN3RBX9ContentIdEEEbv")
}

#[doc(alias = "bool XmlNameValuePair::isValueType<std::string>(void)const")]
// 0x798b70 — __ZNK16XmlNameValuePair11isValueTypeISsEEbv
pub fn stub_798b70() -> ! {
    todo!("0x798b70 __ZNK16XmlNameValuePair11isValueTypeISsEEbv")
}

#[doc(alias = "XmlNameValuePair::getValue(RBX::ContentId &)const")]
// 0x798b7c — __ZNK16XmlNameValuePair8getValueERN3RBX9ContentIdE
pub fn stub_798b7c() -> ! {
    todo!("0x798b7c __ZNK16XmlNameValuePair8getValueERN3RBX9ContentIdE")
}

#[doc(alias = "XmlNameValuePair::getValue(std::string &)const")]
// 0x798d20 — __ZNK16XmlNameValuePair8getValueERSs
pub fn stub_798d20() -> ! {
    todo!("0x798d20 __ZNK16XmlNameValuePair8getValueERSs")
}

#[doc(alias = "decodeString(std::string const&)")]
// 0x79a624 — __Z12decodeStringRKSs
pub fn stub_79a624() -> ! {
    todo!("0x79a624 __Z12decodeStringRKSs")
}

#[doc(alias = "TextXmlWriter::encodedWrite(std::ostream &,char const*,unsigned long)")]
// 0x79ae3c — __ZN13TextXmlWriter12encodedWriteERSoPKcm
pub fn stub_79ae3c() -> ! {
    todo!("0x79ae3c __ZN13TextXmlWriter12encodedWriteERSoPKcm")
}

#[doc(alias = "TextXmlParser::removeTag(std::string const&,int &)")]
// 0x79b2b8 — __ZN13TextXmlParser9removeTagERKSsRi
pub fn stub_79b2b8() -> ! {
    todo!("0x79b2b8 __ZN13TextXmlParser9removeTagERKSsRi")
}

#[doc(alias = "TextXmlParser::parseAttributes(std::string const&)")]
// 0x79b3c4 — __ZN13TextXmlParser15parseAttributesERKSs
pub fn stub_79b3c4() -> ! {
    todo!("0x79b3c4 __ZN13TextXmlParser15parseAttributesERKSs")
}

#[doc(alias = "XmlParser::XmlParser(std::basic_streambuf<char,std::char_traits<char>> *)")]
// 0x79b924 — __ZN9XmlParserC2EPSt15basic_streambufIcSt11char_traitsIcEE
pub fn stub_79b924() -> ! {
    todo!("0x79b924 __ZN9XmlParserC2EPSt15basic_streambufIcSt11char_traitsIcEE")
}

#[doc(alias = "XmlWriter::XmlWriter(std::ostream &)")]
// 0x79c9c4 — __ZN9XmlWriterC2ERSo
pub fn stub_79c9c4() -> ! {
    todo!("0x79c9c4 __ZN9XmlWriterC2ERSo")
}

#[doc(alias = "std::deque<XmlElement *,std::allocator<XmlElement *>>::push_back(XmlElement * const&)")]
// 0x79ce54 — __ZNSt5dequeIP10XmlElementSaIS1_EE9push_backERKS1_
pub fn stub_79ce54() -> ! {
    todo!("0x79ce54 __ZNSt5dequeIP10XmlElementSaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::deque<XmlElement *,std::allocator<XmlElement *>>::_M_push_back_aux(XmlElement * const&)")]
// 0x79ce74 — __ZNSt5dequeIP10XmlElementSaIS1_EE16_M_push_back_auxERKS1_
pub fn stub_79ce74() -> ! {
    todo!("0x79ce74 __ZNSt5dequeIP10XmlElementSaIS1_EE16_M_push_back_auxERKS1_")
}

#[doc(alias = "std::deque<XmlElement *,std::allocator<XmlElement *>>::_M_reserve_map_at_back(unsigned long)")]
// 0x79ceac — __ZNSt5dequeIP10XmlElementSaIS1_EE22_M_reserve_map_at_backEm
pub fn stub_79ceac() -> ! {
    todo!("0x79ceac __ZNSt5dequeIP10XmlElementSaIS1_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<XmlElement *,std::allocator<XmlElement *>>::_M_reallocate_map(unsigned long,bool)")]
// 0x79cec8 — __ZNSt5dequeIP10XmlElementSaIS1_EE17_M_reallocate_mapEmb
pub fn stub_79cec8() -> ! {
    todo!("0x79cec8 __ZNSt5dequeIP10XmlElementSaIS1_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::_M_allocate_map(unsigned long)")]
// 0x79cfa0 — __ZNSt11_Deque_baseIP10XmlElementSaIS1_EE15_M_allocate_mapEm
pub fn stub_79cfa0() -> ! {
    todo!("0x79cfa0 __ZNSt11_Deque_baseIP10XmlElementSaIS1_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::deque<XmlElement *,std::allocator<XmlElement *>>::pop_back(void)")]
// 0x79cfb8 — __ZNSt5dequeIP10XmlElementSaIS1_EE8pop_backEv
pub fn stub_79cfb8() -> ! {
    todo!("0x79cfb8 __ZNSt5dequeIP10XmlElementSaIS1_EE8pop_backEv")
}

#[doc(alias = "std::deque<XmlElement *,std::allocator<XmlElement *>>::deque(std::deque<XmlElement *,std::allocator<XmlElement *>> const&)")]
// 0x79cfe8 — __ZNSt5dequeIP10XmlElementSaIS1_EEC2ERKS3_
pub fn stub_79cfe8() -> ! {
    todo!("0x79cfe8 __ZNSt5dequeIP10XmlElementSaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::_Deque_iterator<XmlElement *,XmlElement *&,XmlElement **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<XmlElement *,XmlElement * const&,XmlElement * const*>,std::_Deque_iterator<XmlElement *,XmlElement *&,XmlElement **>>(std::_Deque_iterator<XmlElement *,XmlElement * const&,XmlElement * const*>,std::_Deque_iterator<XmlElement *,XmlElement * const&,XmlElement * const*>,std::_Deque_iterator<XmlElement *,XmlElement *&,XmlElement **>)")]
// 0x79d07c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIP10XmlElementRKS5_PS6_ES3_IS5_RS5_PS5_EEET0_T_SE_SD_
pub fn stub_79d07c() -> ! {
    todo!("0x79d07c __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIP10XmlElementRKS5_PS6_ES3_IS5_RS5_PS5_EEET0_T_SE_SD_")
}

#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::_M_initialize_map(unsigned long)")]
// 0x79d118 — __ZNSt11_Deque_baseIP10XmlElementSaIS1_EE17_M_initialize_mapEm
pub fn stub_79d118() -> ! {
    todo!("0x79d118 __ZNSt11_Deque_baseIP10XmlElementSaIS1_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::_M_create_nodes(XmlElement ***,XmlElement ***)")]
// 0x79d270 — __ZNSt11_Deque_baseIP10XmlElementSaIS1_EE15_M_create_nodesEPPS1_S5_
pub fn stub_79d270() -> ! {
    todo!("0x79d270 __ZNSt11_Deque_baseIP10XmlElementSaIS1_EE15_M_create_nodesEPPS1_S5_")
}

