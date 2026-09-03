//! rendering shard 479 — 120 stubs 0x764b38..0x771218 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 51684->51804 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc after shard 478 (0x764b38..0x771218)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x764b38 — __ZN3RBX5World15removePrimitiveEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, bool)
#[doc(alias = "RBX::World::removePrimitive(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX5World15removePrimitiveEPNS_9PrimitiveEb")]
// IDA 0x764b38: 236 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764b38() {
}

// 0x764e34 — __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EEbb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::World::destroyAutoJoints(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,bool,bool)")]
#[doc(alias = "__ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EEbb")]
// IDA 0x764e34: 326 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764e34() {
}

// 0x7651e8 — __ZN3RBX11doNotIgnoreEPNS_9PrimitiveEPSt3setIS1_St4lessIS1_ESaIS1_EES7_
#[doc(alias = "RBX::doNotIgnore(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *)")]
#[doc(alias = "__ZN3RBX11doNotIgnoreEPNS_9PrimitiveEPSt3setIS1_St4lessIS1_ESaIS1_EES7_")]
// IDA 0x7651e8: 56 insns (CBZ..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7651e8() {
}

// 0x765414 — __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, bool)
#[doc(alias = "RBX::World::destroyAutoJoints(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEb")]
// IDA 0x765414: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765414() {
}

// 0x7655a0 — __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EES8_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::World::createAutoJoints(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *)")]
#[doc(alias = "__ZN3RBX5World16createAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EES8_")]
// IDA 0x7655a0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7655a0() {
}

// 0x765980 — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_
#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::push_back(RBX::Profiling::CodeProfiler * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_")]
// IDA 0x765980: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_765980() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x7659ac — __ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_
#[doc(alias = "void RBX::notifyMovingPrimitives<std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>>(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> const&)")]
#[doc(alias = "__ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_")]
// IDA 0x7659ac: 14 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7659ac() {
}

// 0x765c1c — __ZN3RBX5World15assertNotInStepEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::assertNotInStep(void)")]
#[doc(alias = "__ZN3RBX5World15assertNotInStepEv")]
// IDA 0x765c1c: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765c1c() {
}

// 0x765dc4 — __ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_
#[doc(alias = "RBX::IndexArray<RBX::Primitive,&RBX::Primitive::worldIndexFunc>::fastRemove(RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_")]
// IDA 0x765dc4: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765dc4() {
}

// 0x765e9c — __ZN3RBX5Joint11isAutoJointEPKS0_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Joint *)
#[doc(alias = "RBX::Joint::isAutoJoint(RBX::Joint const*)")]
#[doc(alias = "__ZN3RBX5Joint11isAutoJointEPKS0_")]
// IDA 0x765e9c: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765e9c() {
}

// 0x765f04 — __ZN3RBX11shared_fromINS_13JointInstanceEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance> RBX::shared_from<RBX::JointInstance>(RBX::JointInstance*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_13JointInstanceEEEN5boost10shared_ptrIT_EEPS4_")]
// was: boost::shared_ptr<RBX::JointInstance> RBX::shared_from<RBX::JointInstance>(RBX::JointInstance*)
// IDA 0x765f04: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765f04() {
}

// 0x766074 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::push_back(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_")]
// IDA 0x766074: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_766074() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x7660a0 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,RBX::Joint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x7660a0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_7660a0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x766180 — __ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm")]
// IDA 0x766180: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_766180() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x7661e4 — __ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::JointInstance>::operator=(rbx_core::SharedPtr<RBX::JointInstance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13JointInstanceEEaSERKS3_")]
// was: boost::shared_ptr<RBX::JointInstance>::operator=(boost::shared_ptr<RBX::JointInstance> const&)
// IDA 0x7661e4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7661e4() {
}

// 0x7667fc — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
// IDA 0x7667fc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7667fc() {
}

// 0x766824 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::equal_range(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
// IDA 0x766824: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_766824() {
}

// 0x766870 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(std::_Rb_tree_iterator<RBX::Joint *>,std::_Rb_tree_iterator<RBX::Joint *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
// IDA 0x766870: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_766870() {
}

// 0x7668d0 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_erase(std::_Rb_tree_node<RBX::Joint *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// IDA 0x7668d0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7668d0() {
}

// 0x7668f8 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert_unique(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// IDA 0x7668f8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7668f8() {
}

// 0x766960 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// IDA 0x766960: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_766960() {
}

// 0x766d1c — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Profiling::CodeProfiler **,std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>>,RBX::Profiling::CodeProfiler * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// IDA 0x766d1c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_766d1c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x766dfc — __ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm")]
// IDA 0x766dfc: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_766dfc() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x767640 — __GLOBAL__I_a_358
#[doc(alias = "__GLOBAL__I_a_358")]
// IDA 0x767640: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_767640() {
}

// 0x767950 — __ZN3RBX9Scripting15DebuggerManager22addDebugger_ReflectionEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Scripting::DebuggerManager::addDebugger_Reflection(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager22addDebugger_ReflectionEN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::Scripting::DebuggerManager::addDebugger_Reflection(boost::shared_ptr<RBX::Instance>)
// IDA 0x767950: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767950() {
}

// 0x767b10 — __ZN3RBX9Scripting15DebuggerManager15enableDebuggingEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::enableDebugging(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager15enableDebuggingEv")]
// IDA 0x767b10: 7 insns (MOVW..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767b10() {
}

// 0x767b28 — __ZN3RBX9Scripting15DebuggerManager23getDebuggers_ReflectionEv
// type: void __fastcall(RBX::Scripting::DebuggerManager *this, _DWORD *)
#[doc(alias = "RBX::Scripting::DebuggerManager::getDebuggers_Reflection(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager23getDebuggers_ReflectionEv")]
// IDA 0x767b28: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767b28() {
}

// 0x767c70 — __ZN3RBX9Scripting14ScriptDebugger24setBreakpoint_ReflectionEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::setBreakpoint_Reflection(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger24setBreakpoint_ReflectionEi")]
// IDA 0x767c70: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767c70() {
}

// 0x767d38 — __ZN3RBX9Scripting14ScriptDebugger25getBreakpoints_ReflectionEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getBreakpoints_Reflection(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger25getBreakpoints_ReflectionEv")]
// IDA 0x767d38: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767d38() {
}

// 0x767e80 — __ZN3RBX9Scripting14ScriptDebugger19addWatch_ReflectionESs
#[doc(alias = "RBX::Scripting::ScriptDebugger::addWatch_Reflection(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger19addWatch_ReflectionESs")]
// IDA 0x767e80: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767e80() {
}

// 0x767fe0 — __ZN3RBX9Scripting14ScriptDebugger21getWatches_ReflectionEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatches_Reflection(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger21getWatches_ReflectionEv")]
// IDA 0x767fe0: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_767fe0() {
}

// 0x768120 — __ZN3RBX9Scripting14ScriptDebugger24getWatchValue_ReflectionEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatchValue_Reflection(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger24getWatchValue_ReflectionEN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::Scripting::ScriptDebugger::getWatchValue_Reflection(boost::shared_ptr<RBX::Instance>)
// IDA 0x768120: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_768120() {
}

// 0x76829c — __ZN3RBX9Scripting14ScriptDebugger6resumeEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::resume(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger6resumeEv")]
// IDA 0x76829c: 273 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76829c() {
}

// 0x7685c4 — __ZN3RBX9Scripting14ScriptDebugger8stepOverEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOver(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8stepOverEv")]
// IDA 0x7685c4: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7685c4() {
}

// 0x768750 — __ZN3RBX9Scripting14ScriptDebugger8stepIntoEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::stepInto(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8stepIntoEv")]
// IDA 0x768750: 134 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_768750() {
}

// 0x7688d8 — __ZN3RBX9Scripting14ScriptDebugger7stepOutEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOut(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger7stepOutEv")]
// IDA 0x7688d8: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7688d8() {
}

// 0x768a64 — __ZN3RBX9Scripting14ScriptDebugger19getStack_ReflectionEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getStack_Reflection(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger19getStack_ReflectionEv")]
// IDA 0x768a64: 781 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_768a64() {
}

// 0x769338 — __ZN3RBX9Scripting14ScriptDebugger9getLocalsEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getLocals(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9getLocalsEi")]
// IDA 0x769338: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_769338() {
}

// 0x769414 — __ZN3RBX9Scripting14ScriptDebugger11getUpvaluesEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getUpvalues(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger11getUpvaluesEi")]
// IDA 0x769414: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_769414() {
}

// 0x7694f0 — __ZN3RBX9Scripting14ScriptDebugger10getGlobalsEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getGlobals(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10getGlobalsEv")]
// IDA 0x7694f0: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7694f0() {
}

// 0x7695c8 — __ZN3RBX9Scripting14ScriptDebugger8setLocalESsNS_10Reflection7VariantEi
#[doc(alias = "RBX::Scripting::ScriptDebugger::setLocal(std::string,RBX::Reflection::Variant,int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8setLocalESsNS_10Reflection7VariantEi")]
// IDA 0x7695c8: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7695c8() {
}

// 0x76986c — __ZN3RBX9Scripting14ScriptDebugger10setUpvalueESsNS_10Reflection7VariantEi
#[doc(alias = "RBX::Scripting::ScriptDebugger::setUpvalue(std::string,RBX::Reflection::Variant,int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10setUpvalueESsNS_10Reflection7VariantEi")]
// IDA 0x76986c: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76986c() {
}

// 0x769b10 — __ZN3RBX9Scripting14ScriptDebugger9setGlobalESsNS_10Reflection7VariantE
#[doc(alias = "RBX::Scripting::ScriptDebugger::setGlobal(std::string,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9setGlobalESsNS_10Reflection7VariantE")]
// IDA 0x769b10: 234 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_769b10() {
}

// 0x769db0 — __ZNK3RBX9Scripting14ScriptDebugger13getScriptPathEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptPath(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger13getScriptPathEv")]
// IDA 0x769db0: 181 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_769db0() {
}

// 0x769f7c — __ZN3RBX9Scripting14ScriptDebugger13setScriptPathESs
#[doc(alias = "RBX::Scripting::ScriptDebugger::setScriptPath(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13setScriptPathESs")]
// IDA 0x769f7c: 562 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_769f7c() {
}

// 0x76a5c0 — __ZN3RBX9Scripting13DebuggerWatch21checkExpressionSyntaxEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerWatch::checkExpressionSyntax(void)")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatch21checkExpressionSyntaxEv")]
// IDA 0x76a5c0: 306 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76a5c0() {
}

// 0x76a92c — __ZN3RBX9Scripting15DebuggerManager9singletonEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::singleton(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager9singletonEv")]
// IDA 0x76a92c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76a92c() {
}

// 0x76a954 — __ZL28initDebuggerManagerSingletonv
// type: _DWORD __fastcall()
#[doc(alias = "initDebuggerManagerSingleton(void)")]
#[doc(alias = "__ZL28initDebuggerManagerSingletonv")]
// IDA 0x76a954: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76a954() {
}

// 0x76aa84 — __ZL16doBasicSingletonv_0
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZL16doBasicSingletonv_0")]
#[doc(alias = "__ZL16doBasicSingletonv_0")]
// IDA 0x76aa84: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76aa84() {
}

// 0x76ab8c — __ZN3RBX9Scripting15DebuggerManagerC2Ev
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::DebuggerManager(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerC2Ev")]
// IDA 0x76ab8c: 303 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76ab8c() {
}

// 0x76aec4 — __ZN3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerD0Ev")]
// IDA 0x76aec4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76aec4() {
}

// 0x76af64 — __ZN3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerD1Ev")]
// IDA 0x76af64: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_76af64() {
}

// 0x76af68 — __ZThn32_N3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9Scripting15DebuggerManagerD0Ev")]
// IDA 0x76af68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76af68() {
}

// 0x76af70 — __ZThn36_N3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9Scripting15DebuggerManagerD0Ev")]
// IDA 0x76af70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76af70() {
}

// 0x76af78 — __ZN3RBX9Scripting15DebuggerManagerD2Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerD2Ev")]
// IDA 0x76af78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76af78() {
}

// 0x76b128 — __ZThn32_N3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9Scripting15DebuggerManagerD1Ev")]
// IDA 0x76b128: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76b128() {
}

// 0x76b130 — __ZThn36_N3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9Scripting15DebuggerManagerD1Ev")]
// IDA 0x76b130: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76b130() {
}

// 0x76b13c — __ZN3RBX9Scripting15DebuggerManager12findDebuggerEP9lua_State
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager12findDebuggerEP9lua_State")]
// IDA 0x76b13c: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b13c() {
}

// 0x76b2b0 — __ZN3RBX9Scripting15DebuggerManager12findDebuggerEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(RBX::Script *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager12findDebuggerEPNS_6ScriptE")]
// IDA 0x76b2b0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b2b0() {
}

// 0x76b2ec — __ZNK3RBX9Scripting15DebuggerManager14askForbidChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerManager::askForbidChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting15DebuggerManager14askForbidChildEPKNS_8InstanceE")]
// IDA 0x76b2ec: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b2ec() {
}

// 0x76b32c — __ZNK3RBX9Scripting15DebuggerManager14verifyAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerManager::verifyAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting15DebuggerManager14verifyAddChildEPKNS_8InstanceE")]
// IDA 0x76b32c: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b32c() {
}

// 0x76b470 — __ZN3RBX9Scripting15DebuggerManager11addDebuggerEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::DebuggerManager::addDebugger(RBX::Script *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager11addDebuggerEPNS_6ScriptE")]
// IDA 0x76b470: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b470() {
}

// 0x76b64c — __ZN3RBX9Scripting15DebuggerManager14onChildRemovedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerManager::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager14onChildRemovedEPNS_8InstanceE")]
// IDA 0x76b64c: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b64c() {
}

// 0x76b798 — __ZN3RBX9Scripting15DebuggerManager17addScriptDebuggerEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerManager::addScriptDebugger(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager17addScriptDebuggerEPNS_8InstanceE")]
// IDA 0x76b798: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b798() {
}

// 0x76b964 — __ZN3RBX9Scripting15DebuggerManager12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerManager::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager12onChildAddedEPNS_8InstanceE")]
// IDA 0x76b964: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_76b964() {
}

// 0x76b968 — __ZN3RBX9Scripting15DebuggerManager14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
#[doc(alias = "RBX::Scripting::DebuggerManager::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE")]
// IDA 0x76b968: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b968() {
}

// 0x76b99c — __ZN3RBX9Scripting14ScriptDebuggerC2ERNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::ScriptDebugger(RBX::Script &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerC2ERNS_6ScriptE")]
// IDA 0x76b99c: 641 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76b99c() {
}

// 0x76c054 — __ZN3RBX9Scripting14ScriptDebugger9setScriptEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::setScript(RBX::Script *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9setScriptEPNS_6ScriptE")]
// IDA 0x76c054: 296 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76c054() {
}

// 0x76c3a4 — __ZN3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerD0Ev")]
// IDA 0x76c3a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76c3a4() {
}

// 0x76c444 — __ZN3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerD1Ev")]
// IDA 0x76c444: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_76c444() {
}

// 0x76c448 — __ZThn32_N3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9Scripting14ScriptDebuggerD0Ev")]
// IDA 0x76c448: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76c448() {
}

// 0x76c450 — __ZThn36_N3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9Scripting14ScriptDebuggerD0Ev")]
// IDA 0x76c450: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76c450() {
}

// 0x76c458 — __ZN3RBX9Scripting14ScriptDebuggerD2Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerD2Ev")]
// IDA 0x76c458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76c458() {
}

// 0x76ca0c — __ZThn32_N3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9Scripting14ScriptDebuggerD1Ev")]
// IDA 0x76ca0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76ca0c() {
}

// 0x76ca14 — __ZThn36_N3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9Scripting14ScriptDebuggerD1Ev")]
// IDA 0x76ca14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76ca14() {
}

// 0x76ca1c — __ZN3RBX9Scripting14ScriptDebugger8addWatchESs
#[doc(alias = "RBX::Scripting::ScriptDebugger::addWatch(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8addWatchESs")]
// IDA 0x76ca1c: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76ca1c() {
}

// 0x76cb6c — __ZN3RBX9Scripting14ScriptDebugger13getWatchValueEPNS0_13DebuggerWatchE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Scripting::DebuggerWatch *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatchValue(RBX::Scripting::DebuggerWatch *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13getWatchValueEPNS0_13DebuggerWatchE")]
// IDA 0x76cb6c: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76cb6c() {
}

// 0x76cd58 — __ZL14readWatchValueSsP9lua_State
#[doc(alias = "readWatchValue(std::string,lua_State *)")]
#[doc(alias = "__ZL14readWatchValueSsP9lua_State")]
// IDA 0x76cd58: 481 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76cd58() {
}

// 0x76d500 — __ZN3RBX9Scripting14ScriptDebugger4hookEP9lua_StateP9lua_Debug
#[doc(alias = "RBX::Scripting::ScriptDebugger::hook(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger4hookEP9lua_StateP9lua_Debug")]
// IDA 0x76d500: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76d500() {
}

// 0x76d5e0 — __ZN3RBX9Scripting14ScriptDebugger13debuggerBreakEP9lua_StateP9lua_Debug
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::debuggerBreak(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13debuggerBreakEP9lua_StateP9lua_Debug")]
// IDA 0x76d5e0: 312 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76d5e0() {
}

// 0x76d95c — __ZN3RBX9Scripting14ScriptDebugger10readLocalsEiP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readLocals(int,lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10readLocalsEiP9lua_State")]
// IDA 0x76d95c: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76d95c() {
}

// 0x76dc5c — __ZN3RBX9Scripting14ScriptDebugger11readGlobalsEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readGlobals(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger11readGlobalsEP9lua_State")]
// IDA 0x76dc5c: 311 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76dc5c() {
}

// 0x76dfcc — __ZN3RBX9Scripting14ScriptDebugger12readUpvaluesEiP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readUpvalues(int,lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger12readUpvaluesEiP9lua_State")]
// IDA 0x76dfcc: 291 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76dfcc() {
}

// 0x76e434 — __ZN3RBX9Scripting14ScriptDebugger9readStackEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readStack(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9readStackEP9lua_State")]
// IDA 0x76e434: 383 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76e434() {
}

// 0x76e860 — __ZN3RBX9Scripting14ScriptDebugger20getScriptForLuaStateEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptForLuaState(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger20getScriptForLuaStateEP9lua_State")]
// IDA 0x76e860: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76e860() {
}

// 0x76ea28 — __ZN3RBX9Scripting14ScriptDebugger10onLineHookEP9lua_StateP9lua_Debug
// type: int __fastcall(char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onLineHook(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10onLineHookEP9lua_StateP9lua_Debug")]
// IDA 0x76ea28: 246 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76ea28() {
}

// 0x76ecb0 — __ZN3RBX9Scripting14ScriptDebugger14findBreakpointEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::findBreakpoint(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger14findBreakpointEi")]
// IDA 0x76ecb0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76ecb0() {
}

// 0x76ece8 — __ZN3RBX9Scripting14ScriptDebugger11shouldBreakEPNS0_18DebuggerBreakpointEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::shouldBreak(RBX::Scripting::DebuggerBreakpoint *,lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger11shouldBreakEPNS0_18DebuggerBreakpointEP9lua_State")]
// IDA 0x76ece8: 415 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76ece8() {
}

// 0x76f178 — __ZNK3RBX9Scripting14ScriptDebugger14askForbidChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::askForbidChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger14askForbidChildEPKNS_8InstanceE")]
// IDA 0x76f178: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76f178() {
}

// 0x76f1e0 — __ZNK3RBX9Scripting14ScriptDebugger14verifyAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::verifyAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger14verifyAddChildEPKNS_8InstanceE")]
// IDA 0x76f1e0: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76f1e0() {
}

// 0x76f324 — __ZNK3RBX9Scripting14ScriptDebugger15verifySetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::verifySetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger15verifySetParentEPKNS_8InstanceE")]
// IDA 0x76f324: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76f324() {
}

// 0x76f488 — __ZN3RBX9Scripting14ScriptDebugger13setBreakpointEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::setBreakpoint(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13setBreakpointEi")]
// IDA 0x76f488: 107 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76f488() {
}

// 0x76f5ac — __ZN3RBX9Scripting14ScriptDebugger14onChildRemovedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger14onChildRemovedEPNS_8InstanceE")]
// IDA 0x76f5ac: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76f5ac() {
}

// 0x76f7ac — __ZN3RBX9Scripting14ScriptDebugger12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger12onChildAddedEPNS_8InstanceE")]
// IDA 0x76f7ac: 224 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76f7ac() {
}

// 0x76fa0c — __ZN3RBX9Scripting14ScriptDebugger16onScriptStartingEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStarting(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16onScriptStartingEP9lua_State")]
// IDA 0x76fa0c: 154 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76fa0c() {
}

// 0x76fbc8 — __ZN3RBX9Scripting14ScriptDebugger15onScriptStoppedEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStopped(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger15onScriptStoppedEv")]
// IDA 0x76fbc8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76fbc8() {
}

// 0x76fbfc — __ZL10doSetLocalSsRKN3RBX10Reflection7VariantEiP9lua_State
#[doc(alias = "doSetLocal(std::string,RBX::Reflection::Variant const&,int,lua_State *)")]
#[doc(alias = "__ZL10doSetLocalSsRKN3RBX10Reflection7VariantEiP9lua_State")]
// IDA 0x76fbfc: 168 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76fbfc() {
}

// 0x76fe68 — __ZL12doSetUpvalueSsRKN3RBX10Reflection7VariantEiP9lua_State
#[doc(alias = "doSetUpvalue(std::string,RBX::Reflection::Variant const&,int,lua_State *)")]
#[doc(alias = "__ZL12doSetUpvalueSsRKN3RBX10Reflection7VariantEiP9lua_State")]
// IDA 0x76fe68: 216 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76fe68() {
}

// 0x77014c — __ZL11doSetGlobalSsRKN3RBX10Reflection7VariantEP9lua_State
#[doc(alias = "doSetGlobal(std::string,RBX::Reflection::Variant const&,lua_State *)")]
#[doc(alias = "__ZL11doSetGlobalSsRKN3RBX10Reflection7VariantEP9lua_State")]
// IDA 0x77014c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77014c() {
}

// 0x770184 — __ZN3RBX9Scripting14ScriptDebugger8getStackEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getStack(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8getStackEv")]
// IDA 0x770184: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770184() {
}

// 0x770384 — __ZN3RBX9Scripting18DebuggerBreakpointC2Ev
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::DebuggerBreakpoint(void)")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointC2Ev")]
// IDA 0x770384: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770384() {
}

// 0x7704dc — __ZN3RBX9Scripting18DebuggerBreakpointC2Ei
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this, int)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::DebuggerBreakpoint(int)")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointC2Ei")]
// IDA 0x7704dc: 225 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7704dc() {
}

// 0x770764 — __ZN3RBX9Scripting18DebuggerBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointD0Ev")]
// IDA 0x770764: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_770764() {
}

// 0x770804 — __ZN3RBX9Scripting18DebuggerBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointD1Ev")]
// IDA 0x770804: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_770804() {
}

// 0x770808 — __ZThn32_N3RBX9Scripting18DebuggerBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9Scripting18DebuggerBreakpointD0Ev")]
// IDA 0x770808: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_770808() {
}

// 0x770810 — __ZThn36_N3RBX9Scripting18DebuggerBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9Scripting18DebuggerBreakpointD0Ev")]
// IDA 0x770810: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_770810() {
}

// 0x770818 — __ZN3RBX9Scripting18DebuggerBreakpointD2Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointD2Ev")]
// IDA 0x770818: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_770818() {
}

// 0x7708fc — __ZThn32_N3RBX9Scripting18DebuggerBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9Scripting18DebuggerBreakpointD1Ev")]
// IDA 0x7708fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7708fc() {
}

// 0x770904 — __ZThn36_N3RBX9Scripting18DebuggerBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9Scripting18DebuggerBreakpointD1Ev")]
// IDA 0x770904: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_770904() {
}

// 0x770910 — __ZN3RBX9Scripting13DebuggerWatchC2ESs
#[doc(alias = "RBX::Scripting::DebuggerWatch::DebuggerWatch(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatchC2ESs")]
// IDA 0x770910: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770910() {
}

// 0x770b34 — __ZNK3RBX9Scripting18DebuggerBreakpoint15verifySetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::verifySetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting18DebuggerBreakpoint15verifySetParentEPKNS_8InstanceE")]
// IDA 0x770b34: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770b34() {
}

// 0x770c98 — __ZNK3RBX9Scripting13DebuggerWatch15verifySetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerWatch *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerWatch::verifySetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting13DebuggerWatch15verifySetParentEPKNS_8InstanceE")]
// IDA 0x770c98: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770c98() {
}

// 0x770dfc — __ZL12getIndexInfoP9lua_State
#[doc(alias = "getIndexInfo(lua_State *)")]
#[doc(alias = "__ZL12getIndexInfoP9lua_State")]
// IDA 0x770dfc: 172 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770dfc() {
}

// 0x770fe0 — __ZL12setIndexInfoP9lua_State
#[doc(alias = "setIndexInfo(lua_State *)")]
#[doc(alias = "__ZL12setIndexInfoP9lua_State")]
// IDA 0x770fe0: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_770fe0() {
}

// 0x7710b0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
// IDA 0x7710b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7710b0() {
}

// 0x7711a4 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFvvELi0EED1Ev")]
// IDA 0x7711a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7711a4() {
}

// 0x7711c8 — __ZNK3RBX9Scripting15DebuggerManager10getEnabledEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::getEnabled(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting15DebuggerManager10getEnabledEv")]
// IDA 0x7711c8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7711c8() {
}

// 0x7711d0 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerManager,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting15DebuggerManagerEbED1Ev")]
// IDA 0x7711d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7711d0() {
}

// 0x7711f4 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting15DebuggerManagerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerManager,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
// IDA 0x7711f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7711f4() {
}

// 0x771218 — __ZN3RBX10Reflection9EventDescINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting15DebuggerManagerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev")]
// was: RBX::Reflection::EventDesc<RBX::Scripting::DebuggerManager,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::DebuggerManager::*>::~EventDesc()
// IDA 0x771218: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771218() {
}
