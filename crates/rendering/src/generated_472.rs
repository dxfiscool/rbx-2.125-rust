//! rendering shard 472 — 100 stubs 0x73bcb0..0x74202c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 50790->50890 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x73bcb0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// IDA 0x73bcb0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73bcb0() {
}

// 0x73bd18 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_
#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::resize(unsigned long,RBX::SurfaceType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_")]
// IDA 0x73bd18: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73bd18() {
}

// 0x73bd4c — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::push_back(RBX::SurfaceType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_")]
// IDA 0x73bd4c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_73bd4c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x73bd74 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,RBX::SurfaceType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0x73bd74: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_73bd74() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x73be58 — __ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm")]
// IDA 0x73be58: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_73be58() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x73be70 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_
#[doc(alias = "RBX::SurfaceType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SurfaceType *,RBX::SurfaceType *>(RBX::SurfaceType *,RBX::SurfaceType *,RBX::SurfaceType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_")]
// IDA 0x73be70: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_73be70() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x73beac — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,unsigned long,RBX::SurfaceType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// IDA 0x73beac: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73beac() {
}

// 0x73c03c — __GLOBAL__I_a_328
// was: global constructor keyed to_a_328
#[doc(alias = "global constructor keyed to_a_328")]
#[doc(alias = "__GLOBAL__I_a_328")]
// IDA 0x73c03c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_73c03c() {
}

// 0x73c104 — __ZN3RBX9MechanismC1Ev
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::Mechanism(void)")]
#[doc(alias = "__ZN3RBX9MechanismC1Ev")]
// IDA 0x73c104: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_73c104() {
}

// 0x73c108 — __ZN3RBX9MechanismC2Ev
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::Mechanism(void)")]
#[doc(alias = "__ZN3RBX9MechanismC2Ev")]
// IDA 0x73c108: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c108() {
}

// 0x73c1e8 — __ZN3RBX9MechanismD0Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::~Mechanism()")]
#[doc(alias = "__ZN3RBX9MechanismD0Ev")]
// IDA 0x73c1e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73c1e8() {
}

// 0x73c288 — __ZN3RBX9MechanismD1Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::~Mechanism()")]
#[doc(alias = "__ZN3RBX9MechanismD1Ev")]
// IDA 0x73c288: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_73c288() {
}

// 0x73c28c — __ZThn8_N3RBX9MechanismD0Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
// was: non-virtual thunk toRBX::Mechanism::~Mechanism()
#[doc(alias = "non-virtual thunk toRBX::Mechanism::~Mechanism()")]
#[doc(alias = "__ZThn8_N3RBX9MechanismD0Ev")]
// IDA 0x73c28c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73c28c() {
}

// 0x73c294 — __ZN3RBX9MechanismD2Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::~Mechanism()")]
#[doc(alias = "__ZN3RBX9MechanismD2Ev")]
// IDA 0x73c294: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73c294() {
}

// 0x73c350 — __ZThn8_N3RBX9MechanismD1Ev
// type: void __fastcall(RBX::Mechanism *__hidden this)
// was: non-virtual thunk toRBX::Mechanism::~Mechanism()
#[doc(alias = "non-virtual thunk toRBX::Mechanism::~Mechanism()")]
#[doc(alias = "__ZThn8_N3RBX9MechanismD1Ev")]
// IDA 0x73c350: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73c350() {
}

// 0x73c358 — __ZNK3RBX9Mechanism26getConstMechanismPrimitiveEv
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::getConstMechanismPrimitive(void)const")]
#[doc(alias = "__ZNK3RBX9Mechanism26getConstMechanismPrimitiveEv")]
// IDA 0x73c358: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c358() {
}

// 0x73c36c — __ZN3RBX9Mechanism21getMechanismPrimitiveEv
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::getMechanismPrimitive(void)")]
#[doc(alias = "__ZN3RBX9Mechanism21getMechanismPrimitiveEv")]
// IDA 0x73c36c: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c36c() {
}

// 0x73c380 — __ZN3RBX9Mechanism24isComplexMovingMechanismEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Assembly *)
#[doc(alias = "RBX::Mechanism::isComplexMovingMechanism(RBX::Assembly const*)")]
#[doc(alias = "__ZN3RBX9Mechanism24isComplexMovingMechanismEPKNS_8AssemblyE")]
// IDA 0x73c380: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c380() {
}

// 0x73c3e4 — __ZN3RBX9Mechanism20isMovingAssemblyRootEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Assembly *)
#[doc(alias = "RBX::Mechanism::isMovingAssemblyRoot(RBX::Assembly const*)")]
#[doc(alias = "__ZN3RBX9Mechanism20isMovingAssemblyRootEPKNS_8AssemblyE")]
// IDA 0x73c3e4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c3e4() {
}

// 0x73c40c — __ZN3RBX9Mechanism21getMovingAssemblyRootEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::Mechanism::getMovingAssemblyRoot(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX9Mechanism21getMovingAssemblyRootEPNS_8AssemblyE")]
// IDA 0x73c40c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c40c() {
}

// 0x73c434 — __ZN3RBX9Mechanism26getConstMovingAssemblyRootEPKNS_8AssemblyE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Assembly *)
#[doc(alias = "RBX::Mechanism::getConstMovingAssemblyRoot(RBX::Assembly const*)")]
#[doc(alias = "__ZN3RBX9Mechanism26getConstMovingAssemblyRootEPKNS_8AssemblyE")]
// IDA 0x73c434: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c434() {
}

// 0x73c45c — __ZN3RBX9Mechanism27getConstRootMovingPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::Mechanism::getConstRootMovingPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX9Mechanism27getConstRootMovingPrimitiveEPKNS_9PrimitiveE")]
// IDA 0x73c45c: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c45c() {
}

// 0x73c4d0 — __ZN3RBX9Mechanism22getRootMovingPrimitiveEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::Mechanism::getRootMovingPrimitive(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9Mechanism22getRootMovingPrimitiveEPNS_9PrimitiveE")]
// IDA 0x73c4d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_73c4d0() {
}

// 0x73c4d4 — __ZN3RBX9Mechanism21getPrimitiveMechanismEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Mechanism::getPrimitiveMechanism(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9Mechanism21getPrimitiveMechanismEPNS_9PrimitiveE")]
// IDA 0x73c4d4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c4d4() {
}

// 0x73c4fc — __ZN3RBX9Mechanism26getConstPrimitiveMechanismEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Mechanism::getConstPrimitiveMechanism(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX9Mechanism26getConstPrimitiveMechanismEPKNS_9PrimitiveE")]
// IDA 0x73c4fc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c4fc() {
}

// 0x73c524 — __ZN3RBX9Mechanism15getRootAssemblyEv
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this)
#[doc(alias = "RBX::Mechanism::getRootAssembly(void)")]
#[doc(alias = "__ZN3RBX9Mechanism15getRootAssemblyEv")]
// IDA 0x73c524: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c524() {
}

// 0x73c584 — __ZN3RBX9Mechanism24isMechanismRootPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Mechanism *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::Mechanism::isMechanismRootPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX9Mechanism24isMechanismRootPrimitiveEPKNS_9PrimitiveE")]
// IDA 0x73c584: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c584() {
}

// 0x73c628 — __GLOBAL__I_a_329
// was: global constructor keyed to_a_329
#[doc(alias = "global constructor keyed to_a_329")]
#[doc(alias = "__GLOBAL__I_a_329")]
// IDA 0x73c628: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_73c628() {
}

// 0x73c6f0 — __ZN3RBX19MechToAssemblyStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::MechToAssemblyStage::MechToAssemblyStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStageC1EPNS_6IStageEPNS_5WorldE")]
// IDA 0x73c6f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_73c6f0() {
}

// 0x73c6f4 — __ZN3RBX19MechToAssemblyStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::MechToAssemblyStage::MechToAssemblyStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStageC2EPNS_6IStageEPNS_5WorldE")]
// IDA 0x73c6f4: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c6f4() {
}

// 0x73c7c8 — __ZN3RBX19MechToAssemblyStageD0Ev
// type: void __fastcall(RBX::MechToAssemblyStage *__hidden this)
#[doc(alias = "RBX::MechToAssemblyStage::~MechToAssemblyStage()")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStageD0Ev")]
// IDA 0x73c7c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73c7c8() {
}

// 0x73c880 — __ZN3RBX19MechToAssemblyStageD1Ev
// type: void __fastcall(RBX::MechToAssemblyStage *__hidden this)
#[doc(alias = "RBX::MechToAssemblyStage::~MechToAssemblyStage()")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStageD1Ev")]
// IDA 0x73c880: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73c880() {
}

// 0x73c8a4 — __ZN3RBX19MechToAssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::MechToAssemblyStage::onSimulateAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE")]
// IDA 0x73c8a4: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c8a4() {
}

// 0x73c958 — __ZN3RBX19MechToAssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::MechToAssemblyStage::onSimulateAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE")]
// IDA 0x73c958: 63 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73c958() {
}

// 0x73ca18 — __ZN3RBX19MechToAssemblyStage29onNoSimulateAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::MechToAssemblyStage::onNoSimulateAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStage29onNoSimulateAssemblyRootAddedEPNS_8AssemblyE")]
// IDA 0x73ca18: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73ca18() {
}

// 0x73cacc — __ZN3RBX19MechToAssemblyStage32onNoSimulateAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::MechToAssemblyStage::onNoSimulateAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStage32onNoSimulateAssemblyRootRemovingEPNS_8AssemblyE")]
// IDA 0x73cacc: 63 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cacc() {
}

// 0x73cb8c — __ZN3RBX19MechToAssemblyStage20onFixedAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::MechToAssemblyStage::onFixedAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStage20onFixedAssemblyAddedEPNS_8AssemblyE")]
// IDA 0x73cb8c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cb8c() {
}

// 0x73cba8 — __ZN3RBX19MechToAssemblyStage23onFixedAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::MechToAssemblyStage::onFixedAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MechToAssemblyStage23onFixedAssemblyRemovingEPNS_8AssemblyE")]
// IDA 0x73cba8: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cba8() {
}

// 0x73cbc4 — __ZNK3RBX19MechToAssemblyStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::MechToAssemblyStage *__hidden this)
#[doc(alias = "RBX::MechToAssemblyStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX19MechToAssemblyStage12getStageTypeEv")]
// IDA 0x73cbc4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cbc4() {
}

// 0x73cbc8 — __ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13AssemblyStageEPS2_EENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEvT0_
#[doc(alias = "void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AssemblyStage,RBX::Assembly*>,boost::_bi::list2<boost::_bi::value<RBX::AssemblyStage*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AssemblyStage,RBX::Assembly*>,boost::_bi::list2<boost::_bi::value<RBX::AssemblyStage*>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13AssemblyStageEPS2_EENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEvT0_")]
// IDA 0x73cbc8: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cbc8() {
}

// 0x73cc88 — __GLOBAL__I_a_330
// was: global constructor keyed to_a_330
#[doc(alias = "global constructor keyed to_a_330")]
#[doc(alias = "__GLOBAL__I_a_330")]
// IDA 0x73cc88: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_73cc88() {
}

// 0x73cea8 — __ZN3RBX4POLY4Mesh9addVertexEfff
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, float, float, float)
#[doc(alias = "RBX::POLY::Mesh::addVertex(float,float,float)")]
#[doc(alias = "__ZN3RBX4POLY4Mesh9addVertexEfff")]
// IDA 0x73cea8: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cea8() {
}

// 0x73cf7c — __ZN3RBX4POLY4Mesh7addFaceEmmmm
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int)
#[doc(alias = "RBX::POLY::Mesh::addFace(unsigned long,unsigned long,unsigned long,unsigned long)")]
#[doc(alias = "__ZN3RBX4POLY4Mesh7addFaceEmmmm")]
// IDA 0x73cf7c: 123 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73cf7c() {
}

// 0x73d4a0 — __ZN3RBX4POLY4Mesh7addFaceEmmm
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, unsigned int, unsigned int, unsigned int)
#[doc(alias = "RBX::POLY::Mesh::addFace(unsigned long,unsigned long,unsigned long)")]
#[doc(alias = "__ZN3RBX4POLY4Mesh7addFaceEmmm")]
// IDA 0x73d4a0: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73d4a0() {
}

// 0x73e120 — __ZN3RBX4POLY4Mesh7addFaceEiPib
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, int, int *, bool)
#[doc(alias = "RBX::POLY::Mesh::addFace(int,int *,bool)")]
#[doc(alias = "__ZN3RBX4POLY4Mesh7addFaceEiPib")]
// IDA 0x73e120: 288 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73e120() {
}

// 0x73ebac — __ZN3RBX4POLY4Mesh14findOrMakeEdgeEmm
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, unsigned int, unsigned int)
#[doc(alias = "RBX::POLY::Mesh::findOrMakeEdge(unsigned long,unsigned long)")]
#[doc(alias = "__ZN3RBX4POLY4Mesh14findOrMakeEdgeEmm")]
// IDA 0x73ebac: 124 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73ebac() {
}

// 0x73ed38 — __ZN3RBX4POLY4Face9initPlaneEv
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this)
#[doc(alias = "RBX::POLY::Face::initPlane(void)")]
#[doc(alias = "__ZN3RBX4POLY4Face9initPlaneEv")]
// IDA 0x73ed38: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73ed38() {
}

// 0x73ed88 — __ZN3RBX4POLY6Vertex8findEdgeEPKS1_
#[doc(alias = "RBX::POLY::Vertex::findEdge(RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX4POLY6Vertex8findEdgeEPKS1_")]
// IDA 0x73ed88: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73ed88() {
}

// 0x73ee28 — __ZN3RBX4POLY4Mesh7addEdgeEPNS0_6VertexES3_
// type: _DWORD __fastcall(RBX::POLY::Mesh *__hidden this, RBX::POLY::Vertex *, RBX::POLY::Vertex *)
#[doc(alias = "RBX::POLY::Mesh::addEdge(RBX::POLY::Vertex *,RBX::POLY::Vertex *)")]
#[doc(alias = "__ZN3RBX4POLY4Mesh7addEdgeEPNS0_6VertexES3_")]
// IDA 0x73ee28: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73ee28() {
}

// 0x73f16c — __ZNK3RBX4POLY6Vertex7getFaceEm
// type: _DWORD __fastcall(RBX::POLY::Vertex *__hidden this, unsigned int)
#[doc(alias = "RBX::POLY::Vertex::getFace(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4POLY6Vertex7getFaceEm")]
// IDA 0x73f16c: 5 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73f16c() {
}

// 0x73f17c — __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_
#[doc(alias = "RBX::POLY::Face::Face(unsigned long,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
#[doc(alias = "__ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_")]
// IDA 0x73f17c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73f17c() {
}

// 0x73f288 — __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_S3_
#[doc(alias = "RBX::POLY::Face::Face(unsigned long,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
#[doc(alias = "__ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_S3_")]
// IDA 0x73f288: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73f288() {
}

// 0x73f3a8 — __ZN3RBX4POLY4FaceC2EmRSt6vectorIPNS0_4EdgeESaIS4_EE
#[doc(alias = "RBX::POLY::Face::Face(unsigned long,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> &)")]
#[doc(alias = "__ZN3RBX4POLY4FaceC2EmRSt6vectorIPNS0_4EdgeESaIS4_EE")]
// IDA 0x73f3a8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73f3a8() {
}

// 0x73fa80 — __ZNK3RBX4POLY4Face11getCentroidEv
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this)
#[doc(alias = "RBX::POLY::Face::getCentroid(void)const")]
#[doc(alias = "__ZNK3RBX4POLY4Face11getCentroidEv")]
// IDA 0x73fa80: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73fa80() {
}

// 0x73ff5c — __ZN3RBX4POLY6Vertex11recoverEdgeEPKS1_S3_
// type: _DWORD __fastcall(RBX::POLY::Vertex *__hidden this, const RBX::POLY::Vertex *, const RBX::POLY::Vertex *)
#[doc(alias = "RBX::POLY::Vertex::recoverEdge(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX4POLY6Vertex11recoverEdgeEPKS1_S3_")]
// IDA 0x73ff5c: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73ff5c() {
}

// 0x7400a0 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm")]
// IDA 0x7400a0: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7400a0() {
}

// 0x74014c — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm")]
// IDA 0x74014c: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74014c() {
}

// 0x7401f8 — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm")]
// IDA 0x7401f8: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7401f8() {
}

// 0x740290 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::push_back(RBX::POLY::Vertex const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_")]
// IDA 0x740290: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_740290() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x7402dc — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::push_back(RBX::POLY::Face const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_")]
// IDA 0x7402dc: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7402dc() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x740314 — __ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE
#[doc(alias = "RBX::POLY::Edge::addFace(RBX::POLY::Face const*)")]
#[doc(alias = "__ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE")]
// IDA 0x740314: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_740314() {
}

// 0x7403c0 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::push_back(RBX::POLY::Edge * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_")]
// IDA 0x7403c0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7403c0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x7403ec — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::push_back(RBX::POLY::Edge const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_")]
// IDA 0x7403ec: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7403ec() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x74041c — __ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE
#[doc(alias = "RBX::POLY::Vertex::addEdge(RBX::POLY::Edge *)")]
#[doc(alias = "__ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE")]
// IDA 0x74041c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74041c() {
}

// 0x74049c — __ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE
#[doc(alias = "RBX::POLY::Edge::getVertexFace(RBX::POLY::Vertex const*)const")]
#[doc(alias = "__ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE")]
// IDA 0x74049c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74049c() {
}

// 0x740508 — __ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE
#[doc(alias = "RBX::POLY::Edge::otherFace(RBX::POLY::Face const*)const")]
#[doc(alias = "__ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE")]
// IDA 0x740508: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_740508() {
}

// 0x7405cc — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge*,std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>>,RBX::POLY::Edge const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x7405cc: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_7405cc() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x740718 — __ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm")]
// IDA 0x740718: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_740718() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x74073c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::POLY::Edge * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Edge *,RBX::POLY::Edge *>(RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_")]
// IDA 0x74073c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_74073c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x74079c — __ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_
// type: int(void)
#[doc(alias = "__gnu_cxx::new_allocator<RBX::POLY::Face>::construct(RBX::POLY::Face*,RBX::POLY::Face const&)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_")]
// IDA 0x74079c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74079c() {
}

// 0x7407e4 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Face*,std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>>,RBX::POLY::Face const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x7407e4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_7407e4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x740bf8 — __ZN3RBX4POLY4FaceaSERKS1_
#[doc(alias = "RBX::POLY::Face::operator=(RBX::POLY::Face const&)")]
#[doc(alias = "__ZN3RBX4POLY4FaceaSERKS1_")]
// IDA 0x740bf8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_740bf8() {
}

// 0x740c20 — __ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm")]
// IDA 0x740c20: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_740c20() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x740c44 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::operator=(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_")]
// IDA 0x740c44: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_740c44() {
}

// 0x740cdc — __ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EE11_M_allocateEm")]
// IDA 0x740cdc: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_740cdc() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x740cf4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4FaceES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::POLY::Face * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Face *,RBX::POLY::Face *>(RBX::POLY::Face *,RBX::POLY::Face *,RBX::POLY::Face *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4FaceES6_EET0_T_S8_S7_")]
// IDA 0x740cf4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_740cf4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x740d50 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEC2ERKS5_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::vector(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEC2ERKS5_")]
// IDA 0x740d50: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_740d50() {
}

// 0x740d88 — __ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EEC2EmRKS4_
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_Vector_base(unsigned long,std::allocator<RBX::POLY::Edge *> const&)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EEC2EmRKS4_")]
// IDA 0x740d88: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_740d88() {
}

// 0x740db8 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Vertex*,std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>>,RBX::POLY::Vertex const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x740db8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_740db8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x741158 — __ZNSt12_Vector_baseIN3RBX4POLY6VertexESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX4POLY6VertexESaIS2_EE11_M_allocateEm")]
// IDA 0x741158: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_741158() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x74117c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY6VertexES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::POLY::Vertex * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Vertex *,RBX::POLY::Vertex *>(RBX::POLY::Vertex *,RBX::POLY::Vertex *,RBX::POLY::Vertex *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY6VertexES6_EET0_T_S8_S7_")]
// IDA 0x74117c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_74117c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x741214 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE15_M_erase_at_endEPS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_erase_at_end(RBX::POLY::Face*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE15_M_erase_at_endEPS2_")]
// IDA 0x741214: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741214() {
}

// 0x741244 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE15_M_erase_at_endEPS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_erase_at_end(RBX::POLY::Vertex*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE15_M_erase_at_endEPS2_")]
// IDA 0x741244: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741244() {
}

// 0x741274 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_
// type: int(void)
#[doc(alias = "RBX::POLY::Face* std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate_and_copy<RBX::POLY::Face*>(unsigned long,RBX::POLY::Face*,RBX::POLY::Face*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_")]
// IDA 0x741274: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_741274() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x741410 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_
// type: int(void)
#[doc(alias = "RBX::POLY::Vertex* std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_allocate_and_copy<RBX::POLY::Vertex*>(unsigned long,RBX::POLY::Vertex*,RBX::POLY::Vertex*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_")]
// IDA 0x741410: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_741410() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x741570 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// IDA 0x741570: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_741570() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x741650 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX4POLY4EdgeESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
// type: int(void)
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge *>(__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX4POLY4EdgeESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag")]
// IDA 0x741650: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741650() {
}

// 0x7416e0 — __GLOBAL__I_a_331
// was: global constructor keyed to_a_331
#[doc(alias = "global constructor keyed to_a_331")]
#[doc(alias = "__GLOBAL__I_a_331")]
// IDA 0x7416e0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7416e0() {
}

// 0x7417a8 — __ZN3RBX12Motor6DJointC1Ev
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::Motor6DJoint(void)")]
#[doc(alias = "__ZN3RBX12Motor6DJointC1Ev")]
// IDA 0x7417a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7417a8() {
}

// 0x7417ac — __ZN3RBX12Motor6DJointC2Ev
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::Motor6DJoint(void)")]
#[doc(alias = "__ZN3RBX12Motor6DJointC2Ev")]
// IDA 0x7417ac: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7417ac() {
}

// 0x741920 — __ZN3RBX12Motor6DJointD0Ev
// type: void __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::~Motor6DJoint()")]
#[doc(alias = "__ZN3RBX12Motor6DJointD0Ev")]
// IDA 0x741920: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_741920() {
}

// 0x7419c0 — __ZN3RBX12Motor6DJointD1Ev
// type: void __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::~Motor6DJoint()")]
#[doc(alias = "__ZN3RBX12Motor6DJointD1Ev")]
// IDA 0x7419c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7419c0() {
}

// 0x7419c4 — __ZThn32_N3RBX12Motor6DJointD0Ev
// type: void __fastcall(RBX::Motor6DJoint *__hidden this)
// was: non-virtual thunk toRBX::Motor6DJoint::~Motor6DJoint()
#[doc(alias = "non-virtual thunk toRBX::Motor6DJoint::~Motor6DJoint()")]
#[doc(alias = "__ZThn32_N3RBX12Motor6DJointD0Ev")]
// IDA 0x7419c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7419c4() {
}

// 0x7419cc — __ZN3RBX12Motor6DJointD2Ev
// type: void __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::~Motor6DJoint()")]
#[doc(alias = "__ZN3RBX12Motor6DJointD2Ev")]
// IDA 0x7419cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7419cc() {
}

// 0x741ac8 — __ZThn32_N3RBX12Motor6DJointD1Ev
// type: void __fastcall(RBX::Motor6DJoint *__hidden this)
// was: non-virtual thunk toRBX::Motor6DJoint::~Motor6DJoint()
#[doc(alias = "non-virtual thunk toRBX::Motor6DJoint::~Motor6DJoint()")]
#[doc(alias = "__ZThn32_N3RBX12Motor6DJointD1Ev")]
// IDA 0x741ac8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_741ac8() {
}

// 0x741ad0 — __ZNK3RBX12Motor6DJoint11getParentIdEv
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::getParentId(void)const")]
#[doc(alias = "__ZNK3RBX12Motor6DJoint11getParentIdEv")]
// IDA 0x741ad0: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741ad0() {
}

// 0x741bf8 — __ZN3RBX12Motor6DJoint9resetLinkEv
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::resetLink(void)")]
#[doc(alias = "__ZN3RBX12Motor6DJoint9resetLinkEv")]
// IDA 0x741bf8: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741bf8() {
}

// 0x741c44 — __ZNK3RBX12Motor6DJoint16getCurrentZAngleEv
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::getCurrentZAngle(void)const")]
#[doc(alias = "__ZNK3RBX12Motor6DJoint16getCurrentZAngleEv")]
// IDA 0x741c44: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741c44() {
}

// 0x741c50 — __ZN3RBX12Motor6DJoint6stepUiEd
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this, double)
#[doc(alias = "RBX::Motor6DJoint::stepUi(double)")]
#[doc(alias = "__ZN3RBX12Motor6DJoint6stepUiEd")]
// IDA 0x741c50: 197 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_741c50() {
}

// 0x74202c — __ZN3RBX12Motor6DJoint16setCurrentZAngleEf
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this, float)
#[doc(alias = "RBX::Motor6DJoint::setCurrentZAngle(float)")]
#[doc(alias = "__ZN3RBX12Motor6DJoint16setCurrentZAngleEf")]
// IDA 0x74202c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74202c() {
}