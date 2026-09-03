//! rendering — generated_watchdog_rend_c — 120 stubs Ogre Compositor/PostProcess (Composition) EA-sorted asc
//! Source: ida/export.json (85545 funcs) filtered Ogre Compositor/PostProcess/Composition (291 candidates) -> 120 lowest EAs (global dedup attempted, 0 remaining so taking lowest; UNIQUE within file)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xb8c314 — __ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE
// type: int __fastcall(std::string *, const std::string *, const std::string *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::ContentId const&,RBX::TextureCompositorLayer::CompositMode)")]
#[doc(alias = "__ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE")]
// IDA 0xb8c314: 287 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8c314() {
}

// 0xb8caa0 — __ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()")]
#[doc(alias = "__ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev")]
// IDA 0xb8caa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b8caa0() {
}

// 0xb8cc38 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_
// type: void __fastcall(int, _DWORD *, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::push_back(RBX::TextureCompositorLayer const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_")]
// IDA 0xb8cc38: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_b8cc38() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xb8cd88 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: void __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0xb8cd88: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_b8cd88() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xb8d3ac — __ZN3RBX22TextureCompositorLayerC2ERKNS_6MeshIdERKN3G3D6Color3E
// type: std::string *__fastcall(std::string *, const std::string *, _DWORD *, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TextureCompositorLayer::TextureCompositorLayer(RBX::MeshId const&,G3D::Color3 const&)")]
#[doc(alias = "__ZN3RBX22TextureCompositorLayerC2ERKNS_6MeshIdERKN3G3D6Color3E")]
// IDA 0xb8d3ac: 117 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8d3ac() {
}

// 0xb8d500 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm
// type: unsigned int __fastcall(int *, unsigned int)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm")]
// IDA 0xb8d500: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8d500() {
}

// 0xb8d608 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_
// type: void *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<RBX::TextureCompositorLayer*>(unsigned long,RBX::TextureCompositorLayer*,RBX::TextureCompositorLayer*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_")]
// IDA 0xb8d608: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_b8d608() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xb8d6f4 — __ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_")]
// IDA 0xb8d6f4: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b8d6f4() {
}

// 0xbd791c — __ZN3RBX20TextureCompositorJobC2EPN4Ogre12VisualEngineERKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS9_EEf
// type: int __fastcall(int, int, int, int, float, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::TextureCompositorJob::TextureCompositorJob(Ogre::VisualEngine *,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&,float)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJobC2EPN4Ogre12VisualEngineERKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS9_EEf")]
// IDA 0xbd791c: 140 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd791c() {
}

// 0xbd7b3c — __ZN3RBX20TextureCompositorJob6updateEv
// type: _DWORD __fastcall(RBX::TextureCompositorJob *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::update(void)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob6updateEv")]
// IDA 0xbd7b3c: 2547 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd7b3c() {
}

// 0xbd94dc — __ZN3RBX20TextureCompositorJob6renderERKN4Ogre10TexturePtrE
// type: _DWORD __fastcall(RBX::TextureCompositorJob *__hidden this, const Ogre::TexturePtr *)
#[doc(alias = "RBX::TextureCompositorJob::render(Ogre::TexturePtr const&)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob6renderERKN4Ogre10TexturePtrE")]
// IDA 0xbd94dc: 387 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd94dc() {
}

// 0xbd98fc — __ZN3RBX17TextureCompositorC1EPN4Ogre12VisualEngineE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX17TextureCompositorC1EPN4Ogre12VisualEngineE")]
// IDA 0xbd98fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bd98fc() {
}

// 0xbd9900 — __ZN3RBX17TextureCompositorC2EPN4Ogre12VisualEngineE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX17TextureCompositorC2EPN4Ogre12VisualEngineE")]
// IDA 0xbd9900: 736 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bd9900() {
}

// 0xbda090 — __ZN3RBX17TextureCompositor21prepareDefaultTextureEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::prepareDefaultTexture(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor21prepareDefaultTextureEv")]
// IDA 0xbda090: 217 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bda090() {
}

// 0xbda2bc — __ZN3RBX17TextureCompositorD0Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD0Ev")]
// IDA 0xbda2bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bda2bc() {
}

// 0xbda35c — __ZN3RBX17TextureCompositorD1Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD1Ev")]
// IDA 0xbda35c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bda35c() {
}

// 0xbda360 — __ZN3RBX17TextureCompositorD2Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD2Ev")]
// IDA 0xbda360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bda360() {
}

// 0xbda788 — __ZN3RBX17TextureCompositor6getJobERKSsRKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS8_EE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::getJob(std::string const&,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor6getJobERKSsRKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS8_EE")]
// IDA 0xbda788: 483 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bda788() {
}

// 0xbdad0c — __ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE
// type: _UNKNOWN **__fastcall(_DWORD *, int, int *)
#[doc(alias = "RBX::TextureCompositor::getTexture(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE")]
// IDA 0xbdad0c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdad0c() {
}

// 0xbdadb8 — __ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::TextureCompositor::getTextureId(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE")]
// IDA 0xbdadb8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdadb8() {
}

// 0xbdae14 — __ZN3RBX17TextureCompositor14attachMaterialERKN5boost10shared_ptrINS0_3JobEEERKN4Ogre11MaterialPtrE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::attachMaterial(boost::shared_ptr<RBX::TextureCompositor::Job> const&,Ogre::MaterialPtr const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor14attachMaterialERKN5boost10shared_ptrINS0_3JobEEERKN4Ogre11MaterialPtrE")]
// IDA 0xbdae14: 144 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdae14() {
}

// 0xbdaf9c — __ZN3RBX17TextureCompositor14attachInstanceERKN5boost10shared_ptrINS0_3JobEEERKNS2_INS_12PartInstanceEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::attachInstance(boost::shared_ptr<RBX::TextureCompositor::Job> const&,boost::shared_ptr<RBX::PartInstance> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor14attachInstanceERKN5boost10shared_ptrINS0_3JobEEERKNS2_INS_12PartInstanceEEE")]
// IDA 0xbdaf9c: 324 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdaf9c() {
}

// 0xbdb320 — __ZNK3RBX17TextureCompositor12isQueueEmptyEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::isQueueEmpty(void)const")]
#[doc(alias = "__ZNK3RBX17TextureCompositor12isQueueEmptyEv")]
// IDA 0xbdb320: 15 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdb320() {
}

// 0xbdb344 — __ZN3RBX17TextureCompositor29updatePrioritiesAndOrphanJobsERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::TextureCompositor::updatePrioritiesAndOrphanJobs(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor29updatePrioritiesAndOrphanJobsERKN3G3D7Vector3E")]
// IDA 0xbdb344: 583 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdb344() {
}

// 0xbdb8d8 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_")]
// IDA 0xbdb8d8: 276 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdb8d8() {
}

// 0xbdbb84 — __ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::garbageCollectOrphanedJobs(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv")]
// IDA 0xbdbb84: 506 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdbb84() {
}

// 0xbdc080 — __ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::findRebakeTargetAndEnqueue(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv")]
// IDA 0xbdc080: 313 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc080() {
}

// 0xbdc398 — __ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_")]
// IDA 0xbdc398: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc398() {
}

// 0xbdc510 — __ZN3RBX17TextureCompositor6updateERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::TextureCompositor::update(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor6updateERKN3G3D7Vector3E")]
// IDA 0xbdc510: 330 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc510() {
}

// 0xbdc888 — __ZN3RBX17TextureCompositor9updateJobERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::updateJob(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor9updateJobERNS0_3JobE")]
// IDA 0xbdc888: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc888() {
}

// 0xbdc9e8 — __ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::renderJobFinalize(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE")]
// IDA 0xbdc9e8: 713 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdc9e8() {
}

// 0xbdd154 — __ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *, unsigned int)
#[doc(alias = "RBX::TextureCompositor::renderJobIfNecessary(RBX::TextureCompositor::Job &,unsigned long)")]
#[doc(alias = "__ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm")]
// IDA 0xbdd154: 816 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdd154() {
}

// 0xbdd9d4 — __ZN3RBX17TextureCompositor15getRenderTargetERKN4Ogre10TexturePtrE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, const Ogre::TexturePtr *)
#[doc(alias = "RBX::TextureCompositor::getRenderTarget(Ogre::TexturePtr const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor15getRenderTargetERKN4Ogre10TexturePtrE")]
// IDA 0xbdd9d4: 876 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdd9d4() {
}

// 0xbde4fc — __ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::orphanTextureFromJob(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE")]
// IDA 0xbde4fc: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bde4fc() {
}

// 0xbde708 — __ZN3RBX17TextureCompositor18getOrCreateTextureEj
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, unsigned int)
#[doc(alias = "RBX::TextureCompositor::getOrCreateTexture(unsigned int)")]
#[doc(alias = "__ZN3RBX17TextureCompositor18getOrCreateTextureEj")]
// IDA 0xbde708: 727 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bde708() {
}

// 0xbdee88 — __ZN3RBX17TextureCompositor13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TextureCompositor::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN3RBX17TextureCompositor13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
// IDA 0xbdee88: 446 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdee88() {
}

// 0xbdf32c — __ZNK3RBX17TextureCompositor13getStatisticsEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::getStatistics(void)const")]
#[doc(alias = "__ZNK3RBX17TextureCompositor13getStatisticsEv")]
// IDA 0xbdf32c: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf32c() {
}

// 0xbdf498 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_")]
// IDA 0xbdf498: 285 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf498() {
}

// 0xbdf798 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_")]
// IDA 0xbdf798: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf798() {
}

// 0xbdf9e4 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_")]
// IDA 0xbdf9e4: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdf9e4() {
}

// 0xbdfae0 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,int,boost::shared_ptr<RBX::TextureCompositor::Job>,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_")]
// IDA 0xbdfae0: 323 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdfae0() {
}

// 0xbdfe68 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_")]
// IDA 0xbdfe68: 321 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bdfe68() {
}

// 0xbe04f0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<std::string,boost::shared_ptr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_")]
// IDA 0xbe04f0: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be04f0() {
}

// 0xbe075c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_
// type: int __fastcall(int)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::operator=(std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_")]
// IDA 0xbe075c: 223 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be075c() {
}

// 0xbe09c8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_
// type: int(void)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::push_back(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_")]
// IDA 0xbe09c8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_be09c8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xbe0b20 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm")]
// IDA 0xbe0b20: 51 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be0b20() {
}

// 0xbe1144 — __ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositor::Job>(RBX::TextureCompositor::Job *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_")]
// IDA 0xbe1144: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1144() {
}

// 0xbe1250 — __ZN3RBX17TextureCompositor3JobD2Ev
// type: void __fastcall(RBX::TextureCompositor::Job *__hidden this)
#[doc(alias = "RBX::TextureCompositor::Job::~Job()")]
#[doc(alias = "__ZN3RBX17TextureCompositor3JobD2Ev")]
// IDA 0xbe1250: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be1250() {
}

// 0xbe1550 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev")]
// IDA 0xbe1550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be1550() {
}

// 0xbe15fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev")]
// IDA 0xbe15fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be15fc() {
}

// 0xbe1600 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev")]
// IDA 0xbe1600: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be1600() {
}

// 0xbe1604 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv")]
// IDA 0xbe1604: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1604() {
}

// 0xbe16a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info")]
// IDA 0xbe16a8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be16a8() {
}

// 0xbe16ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv")]
// IDA 0xbe16ac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be16ac() {
}

// 0xbe16b0 — __ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositorJob>(RBX::TextureCompositorJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_")]
// IDA 0xbe16b0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be16b0() {
}

// 0xbe1818 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev")]
// IDA 0xbe1818: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_be1818() {
}

// 0xbe181c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev")]
// IDA 0xbe181c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_be181c() {
}

// 0xbe1820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv")]
// IDA 0xbe1820: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1820() {
}

// 0xbe192c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info")]
// IDA 0xbe192c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be192c() {
}

// 0xbe1930 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv")]
// IDA 0xbe1930: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1930() {
}

// 0xbe1934 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag
// type: int(void)
#[doc(alias = "void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag")]
// IDA 0xbe1934: 894 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be1934() {
}

// 0xbe21d4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_")]
// IDA 0xbe21d4: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be21d4() {
}

// 0xbe2288 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_")]
// IDA 0xbe2288: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_be2288() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xbe2340 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job>* std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<boost::shared_ptr<RBX::TextureCompositor::Job>*>(unsigned long,boost::shared_ptr<RBX::TextureCompositor::Job>*,boost::shared_ptr<RBX::TextureCompositor::Job>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_")]
// IDA 0xbe2340: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_be2340() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xbe2524 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag
#[doc(alias = "void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag")]
// IDA 0xbe2524: 907 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2524() {
}

// 0xbe2dec — __ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>)")]
#[doc(alias = "__ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_")]
// IDA 0xbe2dec: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2dec() {
}

// 0xbe2e74 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E")]
// IDA 0xbe2e74: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2e74() {
}

// 0xbe2fa8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")]
// IDA 0xbe2fa8: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be2fa8() {
}

// 0xbe305c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_create_node(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")]
// IDA 0xbe305c: 103 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be305c() {
}

// 0xbe3278 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")]
// IDA 0xbe3278: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_be3278() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xbe380c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_
#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_")]
// IDA 0xbe380c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_be380c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xbe38f8 — __ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TextureCompositorLayer * std::__uninitialized_copy_aux<RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *>(RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type")]
// IDA 0xbe38f8: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be38f8() {
}

// 0xbe3b44 — __ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TextureCompositorLayer* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*>(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type")]
// IDA 0xbe3b44: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be3b44() {
}

// 0xbe3d90 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// IDA 0xbe3d90: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be3d90() {
}

// 0xbe40d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// IDA 0xbe40d8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be40d8() {
}

// 0xbe414c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_")]
// IDA 0xbe414c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be414c() {
}

// 0xbe4230 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_
// type: int __fastcall(int, int, int, int, void *, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_")]
// IDA 0xbe4230: 93 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4230() {
}

// 0xbe43b4 — __ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_
#[doc(alias = "std::vector<RBX::TextureCompositorJob::LayerData,std::allocator<RBX::TextureCompositorJob::LayerData>>::vector(unsigned long,RBX::TextureCompositorJob::LayerData const&,std::allocator<RBX::TextureCompositorJob::LayerData> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_")]
// IDA 0xbe43b4: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be43b4() {
}

// 0xbe4530 — __ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(RBX::TextureCompositorJob::LayerData const&)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_")]
// IDA 0xbe4530: 160 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4530() {
}

// 0xbe46e4 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TextureCompositor::Job>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0xbe46e4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be46e4() {
}

// 0xbe4714 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// IDA 0xbe4714: 115 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4714() {
}

// 0xbe4854 — __ZN3RBX20TextureCompositorJob9LayerDataD2Ev
// type: void __fastcall(RBX::TextureCompositorJob::LayerData *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::~LayerData()")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataD2Ev")]
// IDA 0xbe4854: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_be4854() {
}

// 0xbe4b50 — __ZN3RBX20TextureCompositorJob9LayerDataC2Ev
// type: _DWORD __fastcall(RBX::TextureCompositorJob::LayerData *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(void)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataC2Ev")]
// IDA 0xbe4b50: 246 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_be4b50() {
}

// 0xc70228 — __ZN4Ogre15CompositionPassC1EPNS_21CompositionTargetPassE
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, Ogre::CompositionTargetPass *)
#[doc(alias = "Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")]
#[doc(alias = "__ZN4Ogre15CompositionPassC1EPNS_21CompositionTargetPassE")]
// IDA 0xc70228: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70228() {
}

// 0xc70234 — __ZN4Ogre15CompositionPassC2EPNS_21CompositionTargetPassE
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, Ogre::CompositionTargetPass *)
#[doc(alias = "Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")]
#[doc(alias = "__ZN4Ogre15CompositionPassC2EPNS_21CompositionTargetPassE")]
// IDA 0xc70234: 267 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70234() {
}

// 0xc70504 — __ZN4Ogre15CompositionPassD1Ev
// type: void __fastcall(Ogre::CompositionPass *__hidden this)
#[doc(alias = "Ogre::CompositionPass::~CompositionPass()")]
#[doc(alias = "__ZN4Ogre15CompositionPassD1Ev")]
// IDA 0xc70504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70504() {
}

// 0xc70510 — __ZN4Ogre15CompositionPassD2Ev
// type: void __fastcall(Ogre::CompositionPass *__hidden this)
#[doc(alias = "Ogre::CompositionPass::~CompositionPass()")]
#[doc(alias = "__ZN4Ogre15CompositionPassD2Ev")]
// IDA 0xc70510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70510() {
}

// 0xc706dc — __ZN4Ogre15CompositionPass7setTypeENS0_8PassTypeE
#[doc(alias = "Ogre::CompositionPass::setType(Ogre::CompositionPass::PassType)")]
#[doc(alias = "__ZN4Ogre15CompositionPass7setTypeENS0_8PassTypeE")]
// IDA 0xc706dc: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c706dc() {
}

// 0xc706e0 — __ZN4Ogre15CompositionPass13setIdentifierEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setIdentifier(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass13setIdentifierEj")]
// IDA 0xc706e0: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c706e0() {
}

// 0xc706e4 — __ZN4Ogre15CompositionPass15setMaterialNameERKSs
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionPass::setMaterialName(std::string const&)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setMaterialNameERKSs")]
// IDA 0xc706e4: 183 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c706e4() {
}

// 0xc708b8 — __ZN4Ogre15CompositionPass15setClearBuffersEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setClearBuffers(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setClearBuffersEj")]
// IDA 0xc708b8: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c708b8() {
}

// 0xc708bc — __ZN4Ogre15CompositionPass14setClearColourENS_11ColourValueE
#[doc(alias = "Ogre::CompositionPass::setClearColour(Ogre::ColourValue)")]
#[doc(alias = "__ZN4Ogre15CompositionPass14setClearColourENS_11ColourValueE")]
// IDA 0xc708bc: 4 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c708bc() {
}

// 0xc708cc — __ZN4Ogre15CompositionPass8setInputEmRKSsm
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int, const std::string *, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")]
#[doc(alias = "__ZN4Ogre15CompositionPass8setInputEmRKSsm")]
// IDA 0xc708cc: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c708cc() {
}

// 0xc709fc — __ZN4Ogre15CompositionPass19setFirstRenderQueueEh
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::CompositionPass::setFirstRenderQueue(unsigned char)")]
#[doc(alias = "__ZN4Ogre15CompositionPass19setFirstRenderQueueEh")]
// IDA 0xc709fc: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c709fc() {
}

// 0xc70a00 — __ZN4Ogre15CompositionPass18setLastRenderQueueEh
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::CompositionPass::setLastRenderQueue(unsigned char)")]
#[doc(alias = "__ZN4Ogre15CompositionPass18setLastRenderQueueEh")]
// IDA 0xc70a00: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a00() {
}

// 0xc70a04 — __ZN4Ogre15CompositionPass17setMaterialSchemeERKSs
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionPass::setMaterialScheme(std::string const&)")]
#[doc(alias = "__ZN4Ogre15CompositionPass17setMaterialSchemeERKSs")]
// IDA 0xc70a04: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a04() {
}

// 0xc70a10 — __ZN4Ogre15CompositionPass13setClearDepthEf
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, float)
#[doc(alias = "Ogre::CompositionPass::setClearDepth(float)")]
#[doc(alias = "__ZN4Ogre15CompositionPass13setClearDepthEf")]
// IDA 0xc70a10: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a10() {
}

// 0xc70a14 — __ZN4Ogre15CompositionPass15setClearStencilEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setClearStencil(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setClearStencilEj")]
// IDA 0xc70a14: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a14() {
}

// 0xc70a18 — __ZN4Ogre15CompositionPass15setStencilCheckEb
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionPass::setStencilCheck(bool)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setStencilCheckEb")]
// IDA 0xc70a18: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a18() {
}

// 0xc70a20 — __ZN4Ogre15CompositionPass14setStencilFuncENS_15CompareFunctionE
#[doc(alias = "Ogre::CompositionPass::setStencilFunc(Ogre::CompareFunction)")]
#[doc(alias = "__ZN4Ogre15CompositionPass14setStencilFuncENS_15CompareFunctionE")]
// IDA 0xc70a20: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a20() {
}

// 0xc70a28 — __ZN4Ogre15CompositionPass18setStencilRefValueEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setStencilRefValue(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass18setStencilRefValueEj")]
// IDA 0xc70a28: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a28() {
}

// 0xc70a30 — __ZN4Ogre15CompositionPass14setStencilMaskEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setStencilMask(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass14setStencilMaskEj")]
// IDA 0xc70a30: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a30() {
}

// 0xc70a38 — __ZN4Ogre15CompositionPass16setStencilFailOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilFailOp(Ogre::StencilOperation)")]
#[doc(alias = "__ZN4Ogre15CompositionPass16setStencilFailOpENS_16StencilOperationE")]
// IDA 0xc70a38: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a38() {
}

// 0xc70a40 — __ZN4Ogre15CompositionPass21setStencilDepthFailOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilDepthFailOp(Ogre::StencilOperation)")]
#[doc(alias = "__ZN4Ogre15CompositionPass21setStencilDepthFailOpENS_16StencilOperationE")]
// IDA 0xc70a40: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a40() {
}

// 0xc70a48 — __ZN4Ogre15CompositionPass16setStencilPassOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilPassOp(Ogre::StencilOperation)")]
#[doc(alias = "__ZN4Ogre15CompositionPass16setStencilPassOpENS_16StencilOperationE")]
// IDA 0xc70a48: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a48() {
}

// 0xc70a50 — __ZN4Ogre15CompositionPass27setStencilTwoSidedOperationEb
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionPass::setStencilTwoSidedOperation(bool)")]
#[doc(alias = "__ZN4Ogre15CompositionPass27setStencilTwoSidedOperationEb")]
// IDA 0xc70a50: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a50() {
}

// 0xc70a58 — __ZN4Ogre15CompositionPass17setQuadFarCornersEbb
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, bool, bool)
#[doc(alias = "Ogre::CompositionPass::setQuadFarCorners(bool,bool)")]
#[doc(alias = "__ZN4Ogre15CompositionPass17setQuadFarCornersEbb")]
// IDA 0xc70a58: 3 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a58() {
}

// 0xc70a64 — __ZN4Ogre15CompositionPass13setCustomTypeERKSs
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionPass::setCustomType(std::string const&)")]
#[doc(alias = "__ZN4Ogre15CompositionPass13setCustomTypeERKSs")]
// IDA 0xc70a64: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a64() {
}

// 0xc70a70 — __ZN4Ogre15CompositionPass12_isSupportedEv
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this)
#[doc(alias = "Ogre::CompositionPass::_isSupported(void)")]
#[doc(alias = "__ZN4Ogre15CompositionPass12_isSupportedEv")]
// IDA 0xc70a70: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70a70() {
}

// 0xc70ad8 — __ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, Ogre::CompositionTechnique *)
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE")]
// IDA 0xc70ad8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70ad8() {
}

// 0xc70ae4 — __ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, Ogre::CompositionTechnique *)
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE")]
// IDA 0xc70ae4: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70ae4() {
}

// 0xc70cb8 — __ZN4Ogre21CompositionTargetPassD1Ev
// type: void __fastcall(Ogre::CompositionTargetPass *__hidden this)
#[doc(alias = "Ogre::CompositionTargetPass::~CompositionTargetPass()")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassD1Ev")]
// IDA 0xc70cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70cb8() {
}

// 0xc70cc4 — __ZN4Ogre21CompositionTargetPassD2Ev
// type: void __fastcall(Ogre::CompositionTargetPass *__hidden this)
#[doc(alias = "Ogre::CompositionTargetPass::~CompositionTargetPass()")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassD2Ev")]
// IDA 0xc70cc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c70cc4() {
}

// 0xc70e08 — __ZN4Ogre21CompositionTargetPass12setInputModeENS0_9InputModeE
#[doc(alias = "Ogre::CompositionTargetPass::setInputMode(Ogre::CompositionTargetPass::InputMode)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass12setInputModeENS0_9InputModeE")]
// IDA 0xc70e08: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e08() {
}

// 0xc70e0c — __ZN4Ogre21CompositionTargetPass13setOutputNameERKSs
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionTargetPass::setOutputName(std::string const&)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass13setOutputNameERKSs")]
// IDA 0xc70e0c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e0c() {
}

// 0xc70e18 — __ZN4Ogre21CompositionTargetPass14setOnlyInitialEb
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionTargetPass::setOnlyInitial(bool)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass14setOnlyInitialEb")]
// IDA 0xc70e18: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e18() {
}

// 0xc70e1c — __ZN4Ogre21CompositionTargetPass17setVisibilityMaskEj
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionTargetPass::setVisibilityMask(unsigned int)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass17setVisibilityMaskEj")]
// IDA 0xc70e1c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e1c() {
}

// 0xc70e20 — __ZN4Ogre21CompositionTargetPass10setLodBiasEf
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, float)
#[doc(alias = "Ogre::CompositionTargetPass::setLodBias(float)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass10setLodBiasEf")]
// IDA 0xc70e20: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e20() {
}

// 0xc70e24 — __ZN4Ogre21CompositionTargetPass17setMaterialSchemeERKSs
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionTargetPass::setMaterialScheme(std::string const&)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass17setMaterialSchemeERKSs")]
// IDA 0xc70e24: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e24() {
}

// 0xc70e30 — __ZN4Ogre21CompositionTargetPass17setShadowsEnabledEb
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionTargetPass::setShadowsEnabled(bool)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass17setShadowsEnabledEb")]
// IDA 0xc70e30: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c70e30() {
}
