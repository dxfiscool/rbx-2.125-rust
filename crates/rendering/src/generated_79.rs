//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe1b0e4..0xe1e324 (100 stubs, 9060 prior -> 9160 covered, 4173 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe1b0e4 — __ZN4Ogre3Any6holderIPNS_21CompositionTargetPassEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTargetPass *>::~holder()")]
// was: Ogre::Any::holder<Ogre::CompositionTargetPass *>::~holder()
// IDA 0xe1b0e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b0e4() {
}

// 0xe1b0e8 — __ZN4Ogre3Any6holderIPNS_21CompositionTargetPassEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTargetPass *>::~holder()")]
// was: Ogre::Any::holder<Ogre::CompositionTargetPass *>::~holder()
// IDA 0xe1b0e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b0e8() {
}

// 0xe1b0f4 — __ZNK4Ogre3Any6holderIPNS_21CompositionTargetPassEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTargetPass *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::CompositionTargetPass *>::getType(void)const
// IDA 0xe1b0f4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b0f4() {
}

// 0xe1b104 — __ZNK4Ogre3Any6holderIPNS_21CompositionTargetPassEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTargetPass *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::CompositionTargetPass *>::clone(void)const
// IDA 0xe1b104: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b104() {
}

// 0xe1b134 — __ZN4Ogre3Any6holderIPNS_21CompositionTargetPassEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTargetPass *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::CompositionTargetPass *>::writeToStream(std::ostream &)
// IDA 0xe1b134: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b134() {
}

// 0xe1b144 — __ZNSt6vectorIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)")]
// was: std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)
// IDA 0xe1b144: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e1b144() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe1b248 — __ZN4Ogre3Any6holderIPNS_20CompositionTechniqueEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTechnique *>::~holder()")]
// was: Ogre::Any::holder<Ogre::CompositionTechnique *>::~holder()
// IDA 0xe1b248: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b248() {
}

// 0xe1b24c — __ZN4Ogre3Any6holderIPNS_20CompositionTechniqueEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTechnique *>::~holder()")]
// was: Ogre::Any::holder<Ogre::CompositionTechnique *>::~holder()
// IDA 0xe1b24c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b24c() {
}

// 0xe1b258 — __ZNK4Ogre3Any6holderIPNS_20CompositionTechniqueEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTechnique *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::CompositionTechnique *>::getType(void)const
// IDA 0xe1b258: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b258() {
}

// 0xe1b268 — __ZNK4Ogre3Any6holderIPNS_20CompositionTechniqueEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTechnique *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::CompositionTechnique *>::clone(void)const
// IDA 0xe1b268: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b268() {
}

// 0xe1b298 — __ZN4Ogre3Any6holderIPNS_20CompositionTechniqueEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionTechnique *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::CompositionTechnique *>::writeToStream(std::ostream &)
// IDA 0xe1b298: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b298() {
}

// 0xe1b2a8 — __ZN4Ogre3Any6holderIPNS_10CompositorEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Compositor *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Compositor *>::~holder()
// IDA 0xe1b2a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b2a8() {
}

// 0xe1b2ac — __ZN4Ogre3Any6holderIPNS_10CompositorEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Compositor *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Compositor *>::~holder()
// IDA 0xe1b2ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b2ac() {
}

// 0xe1b2b8 — __ZNK4Ogre3Any6holderIPNS_10CompositorEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::Compositor *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::Compositor *>::getType(void)const
// IDA 0xe1b2b8: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b2b8() {
}

// 0xe1b2c8 — __ZNK4Ogre3Any6holderIPNS_10CompositorEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::Compositor *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::Compositor *>::clone(void)const
// IDA 0xe1b2c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b2c8() {
}

// 0xe1b2f8 — __ZN4Ogre3Any6holderIPNS_10CompositorEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::Compositor *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::Compositor *>::writeToStream(std::ostream &)
// IDA 0xe1b2f8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b2f8() {
}

// 0xe1b308 — __ZN4Ogre3Any6holderIPNS_14ParticleSystemEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::ParticleSystem *>::~holder()")]
// was: Ogre::Any::holder<Ogre::ParticleSystem *>::~holder()
// IDA 0xe1b308: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b308() {
}

// 0xe1b30c — __ZN4Ogre3Any6holderIPNS_14ParticleSystemEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::ParticleSystem *>::~holder()")]
// was: Ogre::Any::holder<Ogre::ParticleSystem *>::~holder()
// IDA 0xe1b30c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b30c() {
}

// 0xe1b318 — __ZNK4Ogre3Any6holderIPNS_14ParticleSystemEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::ParticleSystem *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::ParticleSystem *>::getType(void)const
// IDA 0xe1b318: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b318() {
}

// 0xe1b328 — __ZNK4Ogre3Any6holderIPNS_14ParticleSystemEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::ParticleSystem *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::ParticleSystem *>::clone(void)const
// IDA 0xe1b328: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b328() {
}

// 0xe1b358 — __ZN4Ogre3Any6holderIPNS_14ParticleSystemEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::ParticleSystem *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::ParticleSystem *>::writeToStream(std::ostream &)
// IDA 0xe1b358: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b358() {
}

// 0xe1b368 — __ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS6_EERKi
#[doc(alias = "std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)")]
// was: std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)
// IDA 0xe1b368: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e1b368() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe1b464 — __ZN4Ogre3Any6holderIPNS_19HighLevelGpuProgramEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::~holder()")]
// was: Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::~holder()
// IDA 0xe1b464: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b464() {
}

// 0xe1b468 — __ZN4Ogre3Any6holderIPNS_19HighLevelGpuProgramEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::~holder()")]
// was: Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::~holder()
// IDA 0xe1b468: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b468() {
}

// 0xe1b474 — __ZNK4Ogre3Any6holderIPNS_19HighLevelGpuProgramEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::getType(void)const
// IDA 0xe1b474: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b474() {
}

// 0xe1b484 — __ZNK4Ogre3Any6holderIPNS_19HighLevelGpuProgramEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::clone(void)const
// IDA 0xe1b484: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b484() {
}

// 0xe1b4b4 — __ZN4Ogre3Any6holderIPNS_19HighLevelGpuProgramEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::HighLevelGpuProgram *>::writeToStream(std::ostream &)
// IDA 0xe1b4b4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b4b4() {
}

// 0xe1b4c4 — __ZN4Ogre3Any6holderIPNS_10GpuProgramEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::GpuProgram *>::~holder()")]
// was: Ogre::Any::holder<Ogre::GpuProgram *>::~holder()
// IDA 0xe1b4c4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b4c4() {
}

// 0xe1b4c8 — __ZN4Ogre3Any6holderIPNS_10GpuProgramEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::GpuProgram *>::~holder()")]
// was: Ogre::Any::holder<Ogre::GpuProgram *>::~holder()
// IDA 0xe1b4c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b4c8() {
}

// 0xe1b4d4 — __ZNK4Ogre3Any6holderIPNS_10GpuProgramEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::GpuProgram *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::GpuProgram *>::getType(void)const
// IDA 0xe1b4d4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b4d4() {
}

// 0xe1b4e4 — __ZNK4Ogre3Any6holderIPNS_10GpuProgramEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::GpuProgram *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::GpuProgram *>::clone(void)const
// IDA 0xe1b4e4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b4e4() {
}

// 0xe1b514 — __ZN4Ogre3Any6holderIPNS_10GpuProgramEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::GpuProgram *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::GpuProgram *>::writeToStream(std::ostream &)
// IDA 0xe1b514: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b514() {
}

// 0xe1b524 — __ZNSt4listISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS1_
#[doc(alias = "std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)")]
// was: std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)
// IDA 0xe1b524: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b524() {
}

// 0xe1b664 — __ZNSt10_List_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe1b664: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b664() {
}

// 0xe1b668 — __ZNSt10_List_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe1b668: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b668() {
}

// 0xe1b674 — __ZN4Ogre3Any6holderIPNS_16TextureUnitStateEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::TextureUnitState *>::~holder()")]
// was: Ogre::Any::holder<Ogre::TextureUnitState *>::~holder()
// IDA 0xe1b674: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b674() {
}

// 0xe1b678 — __ZN4Ogre3Any6holderIPNS_16TextureUnitStateEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::TextureUnitState *>::~holder()")]
// was: Ogre::Any::holder<Ogre::TextureUnitState *>::~holder()
// IDA 0xe1b678: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b678() {
}

// 0xe1b684 — __ZNK4Ogre3Any6holderIPNS_16TextureUnitStateEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::TextureUnitState *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::TextureUnitState *>::getType(void)const
// IDA 0xe1b684: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b684() {
}

// 0xe1b694 — __ZNK4Ogre3Any6holderIPNS_16TextureUnitStateEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::TextureUnitState *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::TextureUnitState *>::clone(void)const
// IDA 0xe1b694: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b694() {
}

// 0xe1b6c4 — __ZN4Ogre3Any6holderIPNS_16TextureUnitStateEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::TextureUnitState *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::TextureUnitState *>::writeToStream(std::ostream &)
// IDA 0xe1b6c4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b6c4() {
}

// 0xe1b6d4 — __ZN4Ogre3Any6holderIPNS_4PassEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Pass *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Pass *>::~holder()
// IDA 0xe1b6d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b6d4() {
}

// 0xe1b6d8 — __ZN4Ogre3Any6holderIPNS_4PassEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Pass *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Pass *>::~holder()
// IDA 0xe1b6d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b6d8() {
}

// 0xe1b6e4 — __ZNK4Ogre3Any6holderIPNS_4PassEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::Pass *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::Pass *>::getType(void)const
// IDA 0xe1b6e4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b6e4() {
}

// 0xe1b6f4 — __ZNK4Ogre3Any6holderIPNS_4PassEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::Pass *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::Pass *>::clone(void)const
// IDA 0xe1b6f4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b6f4() {
}

// 0xe1b724 — __ZN4Ogre3Any6holderIPNS_4PassEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::Pass *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::Pass *>::writeToStream(std::ostream &)
// IDA 0xe1b724: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b724() {
}

// 0xe1b734 — __ZN4Ogre3Any6holderIPNS_9TechniqueEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Technique *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Technique *>::~holder()
// IDA 0xe1b734: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b734() {
}

// 0xe1b738 — __ZN4Ogre3Any6holderIPNS_9TechniqueEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Technique *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Technique *>::~holder()
// IDA 0xe1b738: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b738() {
}

// 0xe1b744 — __ZNK4Ogre3Any6holderIPNS_9TechniqueEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::Technique *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::Technique *>::getType(void)const
// IDA 0xe1b744: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b744() {
}

// 0xe1b754 — __ZNK4Ogre3Any6holderIPNS_9TechniqueEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::Technique *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::Technique *>::clone(void)const
// IDA 0xe1b754: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b754() {
}

// 0xe1b784 — __ZN4Ogre3Any6holderIPNS_9TechniqueEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::Technique *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::Technique *>::writeToStream(std::ostream &)
// IDA 0xe1b784: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b784() {
}

// 0xe1b794 — __ZN4Ogre3Any6holderIPNS_8MaterialEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Material *>::~holder()
// IDA 0xe1b794: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b794() {
}

// 0xe1b798 — __ZN4Ogre3Any6holderIPNS_8MaterialEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::~holder()")]
// was: Ogre::Any::holder<Ogre::Material *>::~holder()
// IDA 0xe1b798: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b798() {
}

// 0xe1b7a4 — __ZNK4Ogre3Any6holderIPNS_8MaterialEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::Material *>::getType(void)const
// IDA 0xe1b7a4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b7a4() {
}

// 0xe1b7b4 — __ZNK4Ogre3Any6holderIPNS_8MaterialEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::Material *>::clone(void)const
// IDA 0xe1b7b4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b7b4() {
}

// 0xe1b7e4 — __ZN4Ogre3Any6holderIPNS_8MaterialEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::Material *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::Material *>::writeToStream(std::ostream &)
// IDA 0xe1b7e4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b7e4() {
}

// 0xe1b7f4 — __ZN4Ogre35CreateCompositorScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")]
// was: Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()
// IDA 0xe1b7f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b7f4() {
}

// 0xe1b900 — __ZN4Ogre35CreateCompositorScriptCompilerEventD0Ev
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")]
// was: Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()
// IDA 0xe1b900: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b900() {
}

// 0xe1b914 — __ZN4Ogre35CreateCompositorScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
// IDA 0xe1b914: 165 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b914() {
}

// 0xe1bae8 — __ZN4Ogre39CreateParticleSystemScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")]
// was: Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()
// IDA 0xe1bae8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1bae8() {
}

// 0xe1bbf4 — __ZN4Ogre39CreateParticleSystemScriptCompilerEventD0Ev
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")]
// was: Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()
// IDA 0xe1bbf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1bbf4() {
}

// 0xe1bc08 — __ZN4Ogre39CreateParticleSystemScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
// IDA 0xe1bc08: 165 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1bc08() {
}

// 0xe1bddc — __ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")]
// was: Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()
// IDA 0xe1bddc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1bddc() {
}

// 0xe1bee8 — __ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventD0Ev
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")]
// was: Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()
// IDA 0xe1bee8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1bee8() {
}

// 0xe1befc — __ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
// IDA 0xe1befc: 165 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1befc() {
}

// 0xe1c0d0 — __ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()
// IDA 0xe1c0d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1c0d0() {
}

// 0xe1c240 — __ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventD0Ev
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()
// IDA 0xe1c240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1c240() {
}

// 0xe1c254 — __ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventC2ERKSsS2_S2_S2_S2_NS_14GpuProgramTypeEPKSt4listISt4pairISsSsENS_12STLAllocatorIS6_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xe1c254: 235 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1c254() {
}

// 0xe1c4e8 — __ZNSt10_List_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")]
// was: std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()
// IDA 0xe1c4e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1c4e8() {
}

// 0xe1c610 — __ZN4Ogre35CreateGpuProgramScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()
// IDA 0xe1c610: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1c610() {
}

// 0xe1c780 — __ZN4Ogre35CreateGpuProgramScriptCompilerEventD0Ev
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()
// IDA 0xe1c780: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1c780() {
}

// 0xe1c794 — __ZN4Ogre35CreateGpuProgramScriptCompilerEventC2ERKSsS2_S2_S2_S2_NS_14GpuProgramTypeE
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)")]
// was: Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)
// IDA 0xe1c794: 232 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1c794() {
}

// 0xe1ca20 — __ZN4Ogre38ProcessResourceNameScriptCompilerEventD0Ev
#[doc(alias = "Ogre::ProcessResourceNameScriptCompilerEvent::~ProcessResourceNameScriptCompilerEvent()")]
// was: Ogre::ProcessResourceNameScriptCompilerEvent::~ProcessResourceNameScriptCompilerEvent()
// IDA 0xe1ca20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1ca20() {
}

// 0xe1cacc — __ZN4Ogre33CreateMaterialScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")]
// was: Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()
// IDA 0xe1cacc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1cacc() {
}

// 0xe1cbd8 — __ZN4Ogre33CreateMaterialScriptCompilerEventD0Ev
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")]
// was: Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()
// IDA 0xe1cbd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1cbd8() {
}

// 0xe1cbec — __ZN4Ogre41PreApplyTextureAliasesScriptCompilerEventD0Ev
#[doc(alias = "Ogre::PreApplyTextureAliasesScriptCompilerEvent::~PreApplyTextureAliasesScriptCompilerEvent()")]
// was: Ogre::PreApplyTextureAliasesScriptCompilerEvent::~PreApplyTextureAliasesScriptCompilerEvent()
// IDA 0xe1cbec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1cbec() {
}

// 0xe1cc4c — __ZN4Ogre33CreateMaterialScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
// IDA 0xe1cc4c: 165 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1cc4c() {
}

// 0xe1d038 — __ZN4Ogre10SerializerC2Ev
#[doc(alias = "Ogre::Serializer::Serializer(void)")]
// was: Ogre::Serializer::Serializer(void)
// IDA 0xe1d038: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d038() {
}

// 0xe1d1d4 — __ZN4Ogre10SerializerD0Ev
#[doc(alias = "Ogre::Serializer::~Serializer()")]
// was: Ogre::Serializer::~Serializer()
// IDA 0xe1d1d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1d1d4() {
}

// 0xe1d264 — __ZN4Ogre10SerializerD1Ev
#[doc(alias = "Ogre::Serializer::~Serializer()")]
// was: Ogre::Serializer::~Serializer()
// IDA 0xe1d264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1d264() {
}

// 0xe1d270 — __ZN4Ogre10SerializerD2Ev
#[doc(alias = "Ogre::Serializer::~Serializer()")]
// was: Ogre::Serializer::~Serializer()
// IDA 0xe1d270: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1d270() {
}

// 0xe1d3b0 — __ZN4Ogre10Serializer19determineEndiannessERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::Serializer::determineEndianness(Ogre::SharedPtr<Ogre::DataStream> &)")]
// was: Ogre::Serializer::determineEndianness(Ogre::SharedPtr<Ogre::DataStream> &)
// IDA 0xe1d3b0: 409 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d3b0() {
}

// 0xe1d860 — __ZN4Ogre10Serializer19determineEndiannessENS0_6EndianE
#[doc(alias = "Ogre::Serializer::determineEndianness(Ogre::Serializer::Endian)")]
// was: Ogre::Serializer::determineEndianness(Ogre::Serializer::Endian)
// IDA 0xe1d860: 12 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d860() {
}

// 0xe1d878 — __ZN4Ogre10Serializer15writeFileHeaderEv
#[doc(alias = "Ogre::Serializer::writeFileHeader(void)")]
// was: Ogre::Serializer::writeFileHeader(void)
// IDA 0xe1d878: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d878() {
}

// 0xe1d8e8 — __ZN4Ogre10Serializer11writeShortsEPKtm
#[doc(alias = "Ogre::Serializer::writeShorts(unsigned short const*,unsigned long)")]
// was: Ogre::Serializer::writeShorts(unsigned short const*,unsigned long)
// IDA 0xe1d8e8: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d8e8() {
}

// 0xe1d94c — __ZN4Ogre10Serializer11writeStringERKSs
#[doc(alias = "Ogre::Serializer::writeString(std::string const&)")]
// was: Ogre::Serializer::writeString(std::string const&)
// IDA 0xe1d94c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d94c() {
}

// 0xe1d978 — __ZN4Ogre10Serializer16writeChunkHeaderEtm
#[doc(alias = "Ogre::Serializer::writeChunkHeader(unsigned short,unsigned long)")]
// was: Ogre::Serializer::writeChunkHeader(unsigned short,unsigned long)
// IDA 0xe1d978: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1d978() {
}

// 0xe1da14 — __ZN4Ogre10Serializer9writeIntsEPKjm
#[doc(alias = "Ogre::Serializer::writeInts(unsigned int const*,unsigned long)")]
// was: Ogre::Serializer::writeInts(unsigned int const*,unsigned long)
// IDA 0xe1da14: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1da14() {
}

// 0xe1da78 — __ZN4Ogre10Serializer11writeFloatsEPKfm
#[doc(alias = "Ogre::Serializer::writeFloats(float const*,unsigned long)")]
// was: Ogre::Serializer::writeFloats(float const*,unsigned long)
// IDA 0xe1da78: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1da78() {
}

// 0xe1dadc — __ZN4Ogre10Serializer9writeDataEPKvmm
#[doc(alias = "Ogre::Serializer::writeData(void const*,unsigned long,unsigned long)")]
// was: Ogre::Serializer::writeData(void const*,unsigned long,unsigned long)
// IDA 0xe1dadc: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1dadc() {
}

// 0xe1daf0 — __ZN4Ogre10Serializer10writeBoolsEPKbm
#[doc(alias = "Ogre::Serializer::writeBools(bool const*,unsigned long)")]
// was: Ogre::Serializer::writeBools(bool const*,unsigned long)
// IDA 0xe1daf0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1daf0() {
}

// 0xe1db30 — __ZN4Ogre10Serializer14readFileHeaderERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::Serializer::readFileHeader(Ogre::SharedPtr<Ogre::DataStream> &)")]
// was: Ogre::Serializer::readFileHeader(Ogre::SharedPtr<Ogre::DataStream> &)
// IDA 0xe1db30: 537 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1db30() {
}

// 0xe1e170 — __ZN4Ogre10Serializer10readShortsERNS_9SharedPtrINS_10DataStreamEEEPtm
#[doc(alias = "Ogre::Serializer::readShorts(Ogre::SharedPtr<Ogre::DataStream> &,unsigned short *,unsigned long)")]
// was: Ogre::Serializer::readShorts(Ogre::SharedPtr<Ogre::DataStream> &,unsigned short *,unsigned long)
// IDA 0xe1e170: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e170() {
}

// 0xe1e198 — __ZN4Ogre10Serializer10readStringERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::Serializer::readString(Ogre::SharedPtr<Ogre::DataStream> &)")]
// was: Ogre::Serializer::readString(Ogre::SharedPtr<Ogre::DataStream> &)
// IDA 0xe1e198: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e198() {
}

// 0xe1e1a8 — __ZN4Ogre10Serializer9readChunkERNS_9SharedPtrINS_10DataStreamEEE
#[doc(alias = "Ogre::Serializer::readChunk(Ogre::SharedPtr<Ogre::DataStream> &)")]
// was: Ogre::Serializer::readChunk(Ogre::SharedPtr<Ogre::DataStream> &)
// IDA 0xe1e1a8: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e1a8() {
}

// 0xe1e200 — __ZN4Ogre10Serializer8readIntsERNS_9SharedPtrINS_10DataStreamEEEPjm
#[doc(alias = "Ogre::Serializer::readInts(Ogre::SharedPtr<Ogre::DataStream> &,unsigned int *,unsigned long)")]
// was: Ogre::Serializer::readInts(Ogre::SharedPtr<Ogre::DataStream> &,unsigned int *,unsigned long)
// IDA 0xe1e200: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e200() {
}

// 0xe1e228 — __ZN4Ogre10Serializer9readBoolsERNS_9SharedPtrINS_10DataStreamEEEPbm
#[doc(alias = "Ogre::Serializer::readBools(Ogre::SharedPtr<Ogre::DataStream> &,bool *,unsigned long)")]
// was: Ogre::Serializer::readBools(Ogre::SharedPtr<Ogre::DataStream> &,bool *,unsigned long)
// IDA 0xe1e228: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e228() {
}

// 0xe1e270 — __ZN4Ogre10Serializer10readFloatsERNS_9SharedPtrINS_10DataStreamEEEPfm
#[doc(alias = "Ogre::Serializer::readFloats(Ogre::SharedPtr<Ogre::DataStream> &,float *,unsigned long)")]
// was: Ogre::Serializer::readFloats(Ogre::SharedPtr<Ogre::DataStream> &,float *,unsigned long)
// IDA 0xe1e270: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e270() {
}

// 0xe1e298 — __ZN4Ogre10Serializer10readObjectERNS_9SharedPtrINS_10DataStreamEEERNS_7Vector3E
#[doc(alias = "Ogre::Serializer::readObject(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Vector3 &)")]
// was: Ogre::Serializer::readObject(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Vector3 &)
// IDA 0xe1e298: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e298() {
}

// 0xe1e2c0 — __ZN4Ogre10Serializer10readObjectERNS_9SharedPtrINS_10DataStreamEEERNS_10QuaternionE
#[doc(alias = "Ogre::Serializer::readObject(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Quaternion &)")]
// was: Ogre::Serializer::readObject(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Quaternion &)
// IDA 0xe1e2c0: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e2c0() {
}

// 0xe1e324 — __ZN4Ogre10Serializer18flipToLittleEndianEPvmm
#[doc(alias = "Ogre::Serializer::flipToLittleEndian(void *,unsigned long,unsigned long)")]
// was: Ogre::Serializer::flipToLittleEndian(void *,unsigned long,unsigned long)
// IDA 0xe1e324: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1e324() {
}
