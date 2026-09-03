//! rendering — generated_watchdog_rend_c — 120 stubs Ogre Compositor/PostProcess (Composition) EA-sorted asc
//! Source: ida/export.json (85545 funcs) filtered Ogre Compositor/PostProcess/Composition (291 candidates) -> 120 lowest EAs (global dedup attempted, 0 remaining so taking lowest; UNIQUE within file)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xb8c314 — __ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE
// type: int __fastcall(std::string *, const std::string *, const std::string *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "`anonymous namespace::TextureCompositingDescription::add(RBX::MeshId const&,RBX::ContentId const&,RBX::TextureCompositorLayer::CompositMode)")]
#[doc(alias = "__ZN12_GLOBAL__N_129TextureCompositingDescription3addERKN3RBX6MeshIdERKNS1_9ContentIdENS1_22TextureCompositorLayer12CompositModeE")]
pub fn stub_b8c314() -> ! {
    todo!("0xb8c314 `anonymous namespace'::TextureCompositingDescription::add(RBX::MeshId const&,RBX::ContentId const&,RBX::TextureCompositorLayer::CompositMode)")
}

// 0xb8caa0 — __ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()")]
#[doc(alias = "__ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev")]
pub fn stub_b8caa0() -> ! {
    todo!("0xb8caa0 std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()")
}

// 0xb8cc38 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_
// type: void __fastcall(int, _DWORD *, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::push_back(RBX::TextureCompositorLayer const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE9push_backERKS1_")]
pub fn stub_b8cc38() -> ! {
    todo!("0xb8cc38 std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::push_back(RBX::TextureCompositorLayer const&)")
}

// 0xb8cd88 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: void __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_b8cd88() -> ! {
    todo!("0xb8cd88 std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer const&)")
}

// 0xb8d3ac — __ZN3RBX22TextureCompositorLayerC2ERKNS_6MeshIdERKN3G3D6Color3E
// type: std::string *__fastcall(std::string *, const std::string *, _DWORD *, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TextureCompositorLayer::TextureCompositorLayer(RBX::MeshId const&,G3D::Color3 const&)")]
#[doc(alias = "__ZN3RBX22TextureCompositorLayerC2ERKNS_6MeshIdERKN3G3D6Color3E")]
pub fn stub_b8d3ac() -> ! {
    todo!("0xb8d3ac RBX::TextureCompositorLayer::TextureCompositorLayer(RBX::MeshId const&,G3D::Color3 const&)")
}

// 0xb8d500 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm
// type: unsigned int __fastcall(int *, unsigned int)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE7reserveEm")]
pub fn stub_b8d500() -> ! {
    todo!("0xb8d500 std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::reserve(unsigned long)")
}

// 0xb8d608 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_
// type: void *__fastcall(int, unsigned int, int, int)
#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<RBX::TextureCompositorLayer*>(unsigned long,RBX::TextureCompositorLayer*,RBX::TextureCompositorLayer*)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIPS1_EES5_mT_S6_")]
pub fn stub_b8d608() -> ! {
    todo!("0xb8d608 RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<RBX::TextureCompositorLayer*>(unsigned long,RBX::TextureCompositorLayer*,RBX::TextureCompositorLayer*)")
}

// 0xb8d6f4 — __ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_")]
pub fn stub_b8d6f4() -> ! {
    todo!("0xb8d6f4 std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbd791c — __ZN3RBX20TextureCompositorJobC2EPN4Ogre12VisualEngineERKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS9_EEf
// type: int __fastcall(int, int, int, int, float, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::TextureCompositorJob::TextureCompositorJob(Ogre::VisualEngine *,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&,float)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJobC2EPN4Ogre12VisualEngineERKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS9_EEf")]
pub fn stub_bd791c() -> ! {
    todo!("0xbd791c RBX::TextureCompositorJob::TextureCompositorJob(Ogre::VisualEngine *,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&,float)")
}

// 0xbd7b3c — __ZN3RBX20TextureCompositorJob6updateEv
// type: _DWORD __fastcall(RBX::TextureCompositorJob *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::update(void)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob6updateEv")]
pub fn stub_bd7b3c() -> ! {
    todo!("0xbd7b3c RBX::TextureCompositorJob::update(void)")
}

// 0xbd94dc — __ZN3RBX20TextureCompositorJob6renderERKN4Ogre10TexturePtrE
// type: _DWORD __fastcall(RBX::TextureCompositorJob *__hidden this, const Ogre::TexturePtr *)
#[doc(alias = "RBX::TextureCompositorJob::render(Ogre::TexturePtr const&)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob6renderERKN4Ogre10TexturePtrE")]
pub fn stub_bd94dc() -> ! {
    todo!("0xbd94dc RBX::TextureCompositorJob::render(Ogre::TexturePtr const&)")
}

// 0xbd98fc — __ZN3RBX17TextureCompositorC1EPN4Ogre12VisualEngineE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX17TextureCompositorC1EPN4Ogre12VisualEngineE")]
pub fn stub_bd98fc() -> ! {
    todo!("0xbd98fc RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")
}

// 0xbd9900 — __ZN3RBX17TextureCompositorC2EPN4Ogre12VisualEngineE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, Ogre::VisualEngine *)
#[doc(alias = "RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")]
#[doc(alias = "__ZN3RBX17TextureCompositorC2EPN4Ogre12VisualEngineE")]
pub fn stub_bd9900() -> ! {
    todo!("0xbd9900 RBX::TextureCompositor::TextureCompositor(Ogre::VisualEngine *)")
}

// 0xbda090 — __ZN3RBX17TextureCompositor21prepareDefaultTextureEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::prepareDefaultTexture(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor21prepareDefaultTextureEv")]
pub fn stub_bda090() -> ! {
    todo!("0xbda090 RBX::TextureCompositor::prepareDefaultTexture(void)")
}

// 0xbda2bc — __ZN3RBX17TextureCompositorD0Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD0Ev")]
pub fn stub_bda2bc() -> ! {
    todo!("0xbda2bc RBX::TextureCompositor::~TextureCompositor()")
}

// 0xbda35c — __ZN3RBX17TextureCompositorD1Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD1Ev")]
pub fn stub_bda35c() -> ! {
    todo!("0xbda35c RBX::TextureCompositor::~TextureCompositor()")
}

// 0xbda360 — __ZN3RBX17TextureCompositorD2Ev
// type: void __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::~TextureCompositor()")]
#[doc(alias = "__ZN3RBX17TextureCompositorD2Ev")]
pub fn stub_bda360() -> ! {
    todo!("0xbda360 RBX::TextureCompositor::~TextureCompositor()")
}

// 0xbda788 — __ZN3RBX17TextureCompositor6getJobERKSsRKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS8_EE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::getJob(std::string const&,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor6getJobERKSsRKN3G3D7Vector2ERKSt6vectorINS_22TextureCompositorLayerESaIS8_EE")]
pub fn stub_bda788() -> ! {
    todo!("0xbda788 RBX::TextureCompositor::getJob(std::string const&,G3D::Vector2 const&,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")
}

// 0xbdad0c — __ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE
// type: _UNKNOWN **__fastcall(_DWORD *, int, int *)
#[doc(alias = "RBX::TextureCompositor::getTexture(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor10getTextureERKN5boost10shared_ptrINS0_3JobEEE")]
pub fn stub_bdad0c() -> ! {
    todo!("0xbdad0c RBX::TextureCompositor::getTexture(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbdadb8 — __ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::TextureCompositor::getTextureId(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor12getTextureIdERKN5boost10shared_ptrINS0_3JobEEE")]
pub fn stub_bdadb8() -> ! {
    todo!("0xbdadb8 RBX::TextureCompositor::getTextureId(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbdae14 — __ZN3RBX17TextureCompositor14attachMaterialERKN5boost10shared_ptrINS0_3JobEEERKN4Ogre11MaterialPtrE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::attachMaterial(boost::shared_ptr<RBX::TextureCompositor::Job> const&,Ogre::MaterialPtr const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor14attachMaterialERKN5boost10shared_ptrINS0_3JobEEERKN4Ogre11MaterialPtrE")]
pub fn stub_bdae14() -> ! {
    todo!("0xbdae14 RBX::TextureCompositor::attachMaterial(boost::shared_ptr<RBX::TextureCompositor::Job> const&,Ogre::MaterialPtr const&)")
}

// 0xbdaf9c — __ZN3RBX17TextureCompositor14attachInstanceERKN5boost10shared_ptrINS0_3JobEEERKNS2_INS_12PartInstanceEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::TextureCompositor::attachInstance(boost::shared_ptr<RBX::TextureCompositor::Job> const&,boost::shared_ptr<RBX::PartInstance> const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor14attachInstanceERKN5boost10shared_ptrINS0_3JobEEERKNS2_INS_12PartInstanceEEE")]
pub fn stub_bdaf9c() -> ! {
    todo!("0xbdaf9c RBX::TextureCompositor::attachInstance(boost::shared_ptr<RBX::TextureCompositor::Job> const&,boost::shared_ptr<RBX::PartInstance> const&)")
}

// 0xbdb320 — __ZNK3RBX17TextureCompositor12isQueueEmptyEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::isQueueEmpty(void)const")]
#[doc(alias = "__ZNK3RBX17TextureCompositor12isQueueEmptyEv")]
pub fn stub_bdb320() -> ! {
    todo!("0xbdb320 RBX::TextureCompositor::isQueueEmpty(void)const")
}

// 0xbdb344 — __ZN3RBX17TextureCompositor29updatePrioritiesAndOrphanJobsERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::TextureCompositor::updatePrioritiesAndOrphanJobs(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor29updatePrioritiesAndOrphanJobsERKN3G3D7Vector3E")]
pub fn stub_bdb344() -> ! {
    todo!("0xbdb344 RBX::TextureCompositor::updatePrioritiesAndOrphanJobs(G3D::Vector3 const&)")
}

// 0xbdb8d8 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_120ExistsInSetPredicateIS7_EEET_SG_SG_T0_")]
pub fn stub_bdb8d8() -> ! {
    todo!("0xbdb8d8 __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::ExistsInSetPredicate<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbdbb84 — __ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::garbageCollectOrphanedJobs(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor26garbageCollectOrphanedJobsEv")]
pub fn stub_bdbb84() -> ! {
    todo!("0xbdbb84 RBX::TextureCompositor::garbageCollectOrphanedJobs(void)")
}

// 0xbdc080 — __ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::findRebakeTargetAndEnqueue(void)")]
#[doc(alias = "__ZN3RBX17TextureCompositor26findRebakeTargetAndEnqueueEv")]
pub fn stub_bdc080() -> ! {
    todo!("0xbdc080 RBX::TextureCompositor::findRebakeTargetAndEnqueue(void)")
}

// 0xbdc398 — __ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt4sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_")]
pub fn stub_bdc398() -> ! {
    todo!("0xbdc398 void std::sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbdc510 — __ZN3RBX17TextureCompositor6updateERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::TextureCompositor::update(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor6updateERKN3G3D7Vector3E")]
pub fn stub_bdc510() -> ! {
    todo!("0xbdc510 RBX::TextureCompositor::update(G3D::Vector3 const&)")
}

// 0xbdc888 — __ZN3RBX17TextureCompositor9updateJobERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::updateJob(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor9updateJobERNS0_3JobE")]
pub fn stub_bdc888() -> ! {
    todo!("0xbdc888 RBX::TextureCompositor::updateJob(RBX::TextureCompositor::Job &)")
}

// 0xbdc9e8 — __ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::renderJobFinalize(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor17renderJobFinalizeERNS0_3JobE")]
pub fn stub_bdc9e8() -> ! {
    todo!("0xbdc9e8 RBX::TextureCompositor::renderJobFinalize(RBX::TextureCompositor::Job &)")
}

// 0xbdd154 — __ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *, unsigned int)
#[doc(alias = "RBX::TextureCompositor::renderJobIfNecessary(RBX::TextureCompositor::Job &,unsigned long)")]
#[doc(alias = "__ZN3RBX17TextureCompositor20renderJobIfNecessaryERNS0_3JobEm")]
pub fn stub_bdd154() -> ! {
    todo!("0xbdd154 RBX::TextureCompositor::renderJobIfNecessary(RBX::TextureCompositor::Job &,unsigned long)")
}

// 0xbdd9d4 — __ZN3RBX17TextureCompositor15getRenderTargetERKN4Ogre10TexturePtrE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, const Ogre::TexturePtr *)
#[doc(alias = "RBX::TextureCompositor::getRenderTarget(Ogre::TexturePtr const&)")]
#[doc(alias = "__ZN3RBX17TextureCompositor15getRenderTargetERKN4Ogre10TexturePtrE")]
pub fn stub_bdd9d4() -> ! {
    todo!("0xbdd9d4 RBX::TextureCompositor::getRenderTarget(Ogre::TexturePtr const&)")
}

// 0xbde4fc — __ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, RBX::TextureCompositor::Job *)
#[doc(alias = "RBX::TextureCompositor::orphanTextureFromJob(RBX::TextureCompositor::Job &)")]
#[doc(alias = "__ZN3RBX17TextureCompositor20orphanTextureFromJobERNS0_3JobE")]
pub fn stub_bde4fc() -> ! {
    todo!("0xbde4fc RBX::TextureCompositor::orphanTextureFromJob(RBX::TextureCompositor::Job &)")
}

// 0xbde708 — __ZN3RBX17TextureCompositor18getOrCreateTextureEj
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this, unsigned int)
#[doc(alias = "RBX::TextureCompositor::getOrCreateTexture(unsigned int)")]
#[doc(alias = "__ZN3RBX17TextureCompositor18getOrCreateTextureEj")]
pub fn stub_bde708() -> ! {
    todo!("0xbde708 RBX::TextureCompositor::getOrCreateTexture(unsigned int)")
}

// 0xbdee88 — __ZN3RBX17TextureCompositor13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, std::string *this, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TextureCompositor::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN3RBX17TextureCompositor13eventOccurredERKSsPKSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIS1_SsENS6_22CategorisedAllocPolicyILNS6_14MemoryCategoryE0EEEEEE")]
pub fn stub_bdee88() -> ! {
    todo!("0xbdee88 RBX::TextureCompositor::eventOccurred(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xbdf32c — __ZNK3RBX17TextureCompositor13getStatisticsEv
// type: _DWORD __fastcall(RBX::TextureCompositor *__hidden this)
#[doc(alias = "RBX::TextureCompositor::getStatistics(void)const")]
#[doc(alias = "__ZNK3RBX17TextureCompositor13getStatisticsEv")]
pub fn stub_bdf32c() -> ! {
    todo!("0xbdf32c RBX::TextureCompositor::getStatistics(void)const")
}

// 0xbdf498 — __ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_SG_T0_")]
pub fn stub_bdf498() -> ! {
    todo!("0xbdf498 void std::__heap_select<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbdf798 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_")]
pub fn stub_bdf798() -> ! {
    todo!("0xbdf798 void std::__insertion_sort<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbdf9e4 — __ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_
#[doc(alias = "void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt25__unguarded_linear_insertIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEES7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_T1_")]
pub fn stub_bdf9e4() -> ! {
    todo!("0xbdf9e4 void std::__unguarded_linear_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbdfae0 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,int,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiS7_N12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_T0_SH_T1_T2_")]
pub fn stub_bdfae0() -> ! {
    todo!("0xbdfae0 void std::__adjust_heap<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,int,boost::shared_ptr<RBX::TextureCompositor::Job>,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbdfe68 — __ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,`anonymous namespace::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")]
#[doc(alias = "__ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt6vectorIS7_SaIS7_EEEEiN12_GLOBAL__N_118PriorityComparatorIS7_EEEvT_SG_T0_T1_")]
pub fn stub_bdfe68() -> ! {
    todo!("0xbdfe68 void std::__introsort_loop<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job> *,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,int,`anonymous namespace'::PriorityComparator<boost::shared_ptr<RBX::TextureCompositor::Job>>)")
}

// 0xbe04f0 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<std::string,boost::shared_ptr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESt4lessISsESaISt4pairIKSsS5_EEEixERS9_")]
pub fn stub_be04f0() -> ! {
    todo!("0xbe04f0 std::map<std::string,boost::shared_ptr<RBX::TextureCompositor::Job>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::operator[](std::string const&)")
}

// 0xbe075c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_
// type: int __fastcall(int)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::operator=(std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EEaSERKS3_")]
pub fn stub_be075c() -> ! {
    todo!("0xbe075c std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::operator=(std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>> const&)")
}

// 0xbe09c8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_
// type: int(void)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::push_back(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE9push_backERKS5_")]
pub fn stub_be09c8() -> ! {
    todo!("0xbe09c8 std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::push_back(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbe0b20 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE7reserveEm")]
pub fn stub_be0b20() -> ! {
    todo!("0xbe0b20 std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::reserve(unsigned long)")
}

// 0xbe1144 — __ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositor::Job>(RBX::TextureCompositor::Job *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX17TextureCompositor3JobEEEPT_")]
pub fn stub_be1144() -> ! {
    todo!("0xbe1144 boost::detail::shared_count::shared_count<RBX::TextureCompositor::Job>(RBX::TextureCompositor::Job *)")
}

// 0xbe1250 — __ZN3RBX17TextureCompositor3JobD2Ev
// type: void __fastcall(RBX::TextureCompositor::Job *__hidden this)
#[doc(alias = "RBX::TextureCompositor::Job::~Job()")]
#[doc(alias = "__ZN3RBX17TextureCompositor3JobD2Ev")]
pub fn stub_be1250() -> ! {
    todo!("0xbe1250 RBX::TextureCompositor::Job::~Job()")
}

// 0xbe1550 — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EED2Ev")]
pub fn stub_be1550() -> ! {
    todo!("0xbe1550 std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::~vector()")
}

// 0xbe15fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED1Ev")]
pub fn stub_be15fc() -> ! {
    todo!("0xbe15fc boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")
}

// 0xbe1600 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEED0Ev")]
pub fn stub_be1600() -> ! {
    todo!("0xbe1600 boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::~sp_counted_impl_p()")
}

// 0xbe1604 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE7disposeEv")]
pub fn stub_be1604() -> ! {
    todo!("0xbe1604 boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::dispose(void)")
}

// 0xbe16a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE11get_deleterERKSt9type_info")]
pub fn stub_be16a8() -> ! {
    todo!("0xbe16a8 boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_deleter(std::type_info const&)")
}

// 0xbe16ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX17TextureCompositor3JobEE19get_untyped_deleterEv")]
pub fn stub_be16ac() -> ! {
    todo!("0xbe16ac boost::detail::sp_counted_impl_p<RBX::TextureCompositor::Job>::get_untyped_deleter(void)")
}

// 0xbe16b0 — __ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextureCompositorJob>(RBX::TextureCompositorJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX20TextureCompositorJobEEEPT_")]
pub fn stub_be16b0() -> ! {
    todo!("0xbe16b0 boost::detail::shared_count::shared_count<RBX::TextureCompositorJob>(RBX::TextureCompositorJob *)")
}

// 0xbe1818 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED1Ev")]
pub fn stub_be1818() -> ! {
    todo!("0xbe1818 boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")
}

// 0xbe181c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEED0Ev")]
pub fn stub_be181c() -> ! {
    todo!("0xbe181c boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::~sp_counted_impl_p()")
}

// 0xbe1820 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE7disposeEv")]
pub fn stub_be1820() -> ! {
    todo!("0xbe1820 boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::dispose(void)")
}

// 0xbe192c — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE11get_deleterERKSt9type_info")]
pub fn stub_be192c() -> ! {
    todo!("0xbe192c boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_deleter(std::type_info const&)")
}

// 0xbe1930 — __ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX20TextureCompositorJobEE19get_untyped_deleterEv")]
pub fn stub_be1930() -> ! {
    todo!("0xbe1930 boost::detail::sp_counted_impl_p<RBX::TextureCompositorJob>::get_untyped_deleter(void)")
}

// 0xbe1934 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag
// type: int(void)
#[doc(alias = "void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS5_S7_EEEEvSC_T_SD_St20forward_iterator_tag")]
pub fn stub_be1934() -> ! {
    todo!("0xbe1934 void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::forward_iterator_tag)")
}

// 0xbe21d4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_")]
pub fn stub_be21d4() -> ! {
    todo!("0xbe21d4 boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")
}

// 0xbe2288 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES9_EET0_T_SB_SA_")]
pub fn stub_be2288() -> ! {
    todo!("0xbe2288 boost::shared_ptr<RBX::TextureCompositor::Job> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *>(boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *,boost::shared_ptr<RBX::TextureCompositor::Job> *)")
}

// 0xbe2340 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_
#[doc(alias = "boost::shared_ptr<RBX::TextureCompositor::Job>* std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<boost::shared_ptr<RBX::TextureCompositor::Job>*>(unsigned long,boost::shared_ptr<RBX::TextureCompositor::Job>*,boost::shared_ptr<RBX::TextureCompositor::Job>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE20_M_allocate_and_copyIPS5_EES9_mT_SA_")]
pub fn stub_be2340() -> ! {
    todo!("0xbe2340 boost::shared_ptr<RBX::TextureCompositor::Job>* std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_allocate_and_copy<boost::shared_ptr<RBX::TextureCompositor::Job>*>(unsigned long,boost::shared_ptr<RBX::TextureCompositor::Job>*,boost::shared_ptr<RBX::TextureCompositor::Job>*)")
}

// 0xbe2524 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag
#[doc(alias = "void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE15_M_range_insertISt23_Rb_tree_const_iteratorIS5_EEEvN9__gnu_cxx17__normal_iteratorIPS5_S7_EET_SF_St20forward_iterator_tag")]
pub fn stub_be2524() -> ! {
    todo!("0xbe2524 void std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_range_insert<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::forward_iterator_tag)")
}

// 0xbe2dec — __ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_
#[doc(alias = "__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>)")]
#[doc(alias = "__ZNSt13__copy_normalILb0ELb1EE8__copy_nISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEN9__gnu_cxx17__normal_iteratorIPS8_St6vectorIS8_SaIS8_EEEEEET0_T_SI_SH_")]
pub fn stub_be2dec() -> ! {
    todo!("0xbe2dec __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>> std::__copy_normal<false,true>::__copy_n<std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>>(std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Rb_tree_const_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>)")
}

// 0xbe2e74 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE5eraseESt17_Rb_tree_iteratorIS8_E")]
pub fn stub_be2e74() -> ! {
    todo!("0xbe2e74 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>)")
}

// 0xbe2fa8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_be2fa8() -> ! {
    todo!("0xbe2fa8 std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_unique(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbe305c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_create_node(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE14_M_create_nodeERKS5_")]
pub fn stub_be305c() -> ! {
    todo!("0xbe305c std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_create_node(boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbe3278 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
#[doc(alias = "std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")]
pub fn stub_be3278() -> ! {
    todo!("0xbe3278 std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TextureCompositor::Job>*,std::vector<boost::shared_ptr<RBX::TextureCompositor::Job>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>>,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xbe380c — __ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_
#[doc(alias = "RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>)")]
#[doc(alias = "__ZNSt6vectorIN3RBX22TextureCompositorLayerESaIS1_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS1_S3_EEEEPS1_mT_SB_")]
pub fn stub_be380c() -> ! {
    todo!("0xbe380c RBX::TextureCompositorLayer* std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>)")
}

// 0xbe38f8 — __ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TextureCompositorLayer * std::__uninitialized_copy_aux<RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *>(RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX22TextureCompositorLayerES2_ET0_T_S4_S3_St12__false_type")]
pub fn stub_be38f8() -> ! {
    todo!("0xbe38f8 RBX::TextureCompositorLayer * std::__uninitialized_copy_aux<RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *>(RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,RBX::TextureCompositorLayer *,std::__false_type)")
}

// 0xbe3b44 — __ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TextureCompositorLayer* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*>(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX22TextureCompositorLayerESt6vectorIS3_SaIS3_EEEEPS3_ET0_T_SC_SB_St12__false_type")]
pub fn stub_be3b44() -> ! {
    todo!("0xbe3b44 RBX::TextureCompositorLayer* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*>(__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,__gnu_cxx::__normal_iterator<RBX::TextureCompositorLayer const*,std::vector<RBX::TextureCompositorLayer,std::allocator<RBX::TextureCompositorLayer>>>,RBX::TextureCompositorLayer*,std::__false_type)")
}

// 0xbe3d90 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, int, void *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_be3d90() -> ! {
    todo!("0xbe3d90 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")
}

// 0xbe40d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_be40d8() -> ! {
    todo!("0xbe40d8 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")
}

// 0xbe414c — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_be414c() -> ! {
    todo!("0xbe414c std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_insert_unique(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")
}

// 0xbe4230 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_
// type: int __fastcall(int, int, int, int, void *, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE14_M_create_nodeERKS8_")]
pub fn stub_be4230() -> ! {
    todo!("0xbe4230 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_create_node(std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>> const&)")
}

// 0xbe43b4 — __ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_
#[doc(alias = "std::vector<RBX::TextureCompositorJob::LayerData,std::allocator<RBX::TextureCompositorJob::LayerData>>::vector(unsigned long,RBX::TextureCompositorJob::LayerData const&,std::allocator<RBX::TextureCompositorJob::LayerData> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX20TextureCompositorJob9LayerDataESaIS2_EEC2EmRKS2_RKS3_")]
pub fn stub_be43b4() -> ! {
    todo!("0xbe43b4 std::vector<RBX::TextureCompositorJob::LayerData,std::allocator<RBX::TextureCompositorJob::LayerData>>::vector(unsigned long,RBX::TextureCompositorJob::LayerData const&,std::allocator<RBX::TextureCompositorJob::LayerData> const&)")
}

// 0xbe4530 — __ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(RBX::TextureCompositorJob::LayerData const&)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataC2ERKS1_")]
pub fn stub_be4530() -> ! {
    todo!("0xbe4530 RBX::TextureCompositorJob::LayerData::LayerData(RBX::TextureCompositorJob::LayerData const&)")
}

// 0xbe46e4 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TextureCompositor::Job>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEES5_St9_IdentityIS5_ESt4lessIS5_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_be46e4() -> ! {
    todo!("0xbe46e4 std::_Rb_tree<boost::shared_ptr<RBX::TextureCompositor::Job>,boost::shared_ptr<RBX::TextureCompositor::Job>,std::_Identity<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::less<boost::shared_ptr<RBX::TextureCompositor::Job>>,std::allocator<boost::shared_ptr<RBX::TextureCompositor::Job>>>::_M_erase(std::_Rb_tree_node<boost::shared_ptr<RBX::TextureCompositor::Job>> *)")
}

// 0xbe4714 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEESt10_Select1stIS8_ESt4lessISsESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_be4714() -> ! {
    todo!("0xbe4714 std::_Rb_tree<std::string,std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>,std::_Select1st<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>,std::less<std::string>,std::allocator<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,boost::shared_ptr<RBX::TextureCompositor::Job>>> *)")
}

// 0xbe4854 — __ZN3RBX20TextureCompositorJob9LayerDataD2Ev
// type: void __fastcall(RBX::TextureCompositorJob::LayerData *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::~LayerData()")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataD2Ev")]
pub fn stub_be4854() -> ! {
    todo!("0xbe4854 RBX::TextureCompositorJob::LayerData::~LayerData()")
}

// 0xbe4b50 — __ZN3RBX20TextureCompositorJob9LayerDataC2Ev
// type: _DWORD __fastcall(RBX::TextureCompositorJob::LayerData *__hidden this)
#[doc(alias = "RBX::TextureCompositorJob::LayerData::LayerData(void)")]
#[doc(alias = "__ZN3RBX20TextureCompositorJob9LayerDataC2Ev")]
pub fn stub_be4b50() -> ! {
    todo!("0xbe4b50 RBX::TextureCompositorJob::LayerData::LayerData(void)")
}

// 0xc70228 — __ZN4Ogre15CompositionPassC1EPNS_21CompositionTargetPassE
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, Ogre::CompositionTargetPass *)
#[doc(alias = "Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")]
#[doc(alias = "__ZN4Ogre15CompositionPassC1EPNS_21CompositionTargetPassE")]
pub fn stub_c70228() -> ! {
    todo!("0xc70228 Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")
}

// 0xc70234 — __ZN4Ogre15CompositionPassC2EPNS_21CompositionTargetPassE
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, Ogre::CompositionTargetPass *)
#[doc(alias = "Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")]
#[doc(alias = "__ZN4Ogre15CompositionPassC2EPNS_21CompositionTargetPassE")]
pub fn stub_c70234() -> ! {
    todo!("0xc70234 Ogre::CompositionPass::CompositionPass(Ogre::CompositionTargetPass *)")
}

// 0xc70504 — __ZN4Ogre15CompositionPassD1Ev
// type: void __fastcall(Ogre::CompositionPass *__hidden this)
#[doc(alias = "Ogre::CompositionPass::~CompositionPass()")]
#[doc(alias = "__ZN4Ogre15CompositionPassD1Ev")]
pub fn stub_c70504() -> ! {
    todo!("0xc70504 Ogre::CompositionPass::~CompositionPass()")
}

// 0xc70510 — __ZN4Ogre15CompositionPassD2Ev
// type: void __fastcall(Ogre::CompositionPass *__hidden this)
#[doc(alias = "Ogre::CompositionPass::~CompositionPass()")]
#[doc(alias = "__ZN4Ogre15CompositionPassD2Ev")]
pub fn stub_c70510() -> ! {
    todo!("0xc70510 Ogre::CompositionPass::~CompositionPass()")
}

// 0xc706dc — __ZN4Ogre15CompositionPass7setTypeENS0_8PassTypeE
#[doc(alias = "Ogre::CompositionPass::setType(Ogre::CompositionPass::PassType)")]
#[doc(alias = "__ZN4Ogre15CompositionPass7setTypeENS0_8PassTypeE")]
pub fn stub_c706dc() -> ! {
    todo!("0xc706dc Ogre::CompositionPass::setType(Ogre::CompositionPass::PassType)")
}

// 0xc706e0 — __ZN4Ogre15CompositionPass13setIdentifierEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setIdentifier(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass13setIdentifierEj")]
pub fn stub_c706e0() -> ! {
    todo!("0xc706e0 Ogre::CompositionPass::setIdentifier(unsigned int)")
}

// 0xc706e4 — __ZN4Ogre15CompositionPass15setMaterialNameERKSs
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionPass::setMaterialName(std::string const&)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setMaterialNameERKSs")]
pub fn stub_c706e4() -> ! {
    todo!("0xc706e4 Ogre::CompositionPass::setMaterialName(std::string const&)")
}

// 0xc708b8 — __ZN4Ogre15CompositionPass15setClearBuffersEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setClearBuffers(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setClearBuffersEj")]
pub fn stub_c708b8() -> ! {
    todo!("0xc708b8 Ogre::CompositionPass::setClearBuffers(unsigned int)")
}

// 0xc708bc — __ZN4Ogre15CompositionPass14setClearColourENS_11ColourValueE
#[doc(alias = "Ogre::CompositionPass::setClearColour(Ogre::ColourValue)")]
#[doc(alias = "__ZN4Ogre15CompositionPass14setClearColourENS_11ColourValueE")]
pub fn stub_c708bc() -> ! {
    todo!("0xc708bc Ogre::CompositionPass::setClearColour(Ogre::ColourValue)")
}

// 0xc708cc — __ZN4Ogre15CompositionPass8setInputEmRKSsm
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int, const std::string *, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")]
#[doc(alias = "__ZN4Ogre15CompositionPass8setInputEmRKSsm")]
pub fn stub_c708cc() -> ! {
    todo!("0xc708cc Ogre::CompositionPass::setInput(unsigned long,std::string const&,unsigned long)")
}

// 0xc709fc — __ZN4Ogre15CompositionPass19setFirstRenderQueueEh
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::CompositionPass::setFirstRenderQueue(unsigned char)")]
#[doc(alias = "__ZN4Ogre15CompositionPass19setFirstRenderQueueEh")]
pub fn stub_c709fc() -> ! {
    todo!("0xc709fc Ogre::CompositionPass::setFirstRenderQueue(unsigned char)")
}

// 0xc70a00 — __ZN4Ogre15CompositionPass18setLastRenderQueueEh
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::CompositionPass::setLastRenderQueue(unsigned char)")]
#[doc(alias = "__ZN4Ogre15CompositionPass18setLastRenderQueueEh")]
pub fn stub_c70a00() -> ! {
    todo!("0xc70a00 Ogre::CompositionPass::setLastRenderQueue(unsigned char)")
}

// 0xc70a04 — __ZN4Ogre15CompositionPass17setMaterialSchemeERKSs
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionPass::setMaterialScheme(std::string const&)")]
#[doc(alias = "__ZN4Ogre15CompositionPass17setMaterialSchemeERKSs")]
pub fn stub_c70a04() -> ! {
    todo!("0xc70a04 Ogre::CompositionPass::setMaterialScheme(std::string const&)")
}

// 0xc70a10 — __ZN4Ogre15CompositionPass13setClearDepthEf
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, float)
#[doc(alias = "Ogre::CompositionPass::setClearDepth(float)")]
#[doc(alias = "__ZN4Ogre15CompositionPass13setClearDepthEf")]
pub fn stub_c70a10() -> ! {
    todo!("0xc70a10 Ogre::CompositionPass::setClearDepth(float)")
}

// 0xc70a14 — __ZN4Ogre15CompositionPass15setClearStencilEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setClearStencil(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setClearStencilEj")]
pub fn stub_c70a14() -> ! {
    todo!("0xc70a14 Ogre::CompositionPass::setClearStencil(unsigned int)")
}

// 0xc70a18 — __ZN4Ogre15CompositionPass15setStencilCheckEb
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionPass::setStencilCheck(bool)")]
#[doc(alias = "__ZN4Ogre15CompositionPass15setStencilCheckEb")]
pub fn stub_c70a18() -> ! {
    todo!("0xc70a18 Ogre::CompositionPass::setStencilCheck(bool)")
}

// 0xc70a20 — __ZN4Ogre15CompositionPass14setStencilFuncENS_15CompareFunctionE
#[doc(alias = "Ogre::CompositionPass::setStencilFunc(Ogre::CompareFunction)")]
#[doc(alias = "__ZN4Ogre15CompositionPass14setStencilFuncENS_15CompareFunctionE")]
pub fn stub_c70a20() -> ! {
    todo!("0xc70a20 Ogre::CompositionPass::setStencilFunc(Ogre::CompareFunction)")
}

// 0xc70a28 — __ZN4Ogre15CompositionPass18setStencilRefValueEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setStencilRefValue(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass18setStencilRefValueEj")]
pub fn stub_c70a28() -> ! {
    todo!("0xc70a28 Ogre::CompositionPass::setStencilRefValue(unsigned int)")
}

// 0xc70a30 — __ZN4Ogre15CompositionPass14setStencilMaskEj
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionPass::setStencilMask(unsigned int)")]
#[doc(alias = "__ZN4Ogre15CompositionPass14setStencilMaskEj")]
pub fn stub_c70a30() -> ! {
    todo!("0xc70a30 Ogre::CompositionPass::setStencilMask(unsigned int)")
}

// 0xc70a38 — __ZN4Ogre15CompositionPass16setStencilFailOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilFailOp(Ogre::StencilOperation)")]
#[doc(alias = "__ZN4Ogre15CompositionPass16setStencilFailOpENS_16StencilOperationE")]
pub fn stub_c70a38() -> ! {
    todo!("0xc70a38 Ogre::CompositionPass::setStencilFailOp(Ogre::StencilOperation)")
}

// 0xc70a40 — __ZN4Ogre15CompositionPass21setStencilDepthFailOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilDepthFailOp(Ogre::StencilOperation)")]
#[doc(alias = "__ZN4Ogre15CompositionPass21setStencilDepthFailOpENS_16StencilOperationE")]
pub fn stub_c70a40() -> ! {
    todo!("0xc70a40 Ogre::CompositionPass::setStencilDepthFailOp(Ogre::StencilOperation)")
}

// 0xc70a48 — __ZN4Ogre15CompositionPass16setStencilPassOpENS_16StencilOperationE
#[doc(alias = "Ogre::CompositionPass::setStencilPassOp(Ogre::StencilOperation)")]
#[doc(alias = "__ZN4Ogre15CompositionPass16setStencilPassOpENS_16StencilOperationE")]
pub fn stub_c70a48() -> ! {
    todo!("0xc70a48 Ogre::CompositionPass::setStencilPassOp(Ogre::StencilOperation)")
}

// 0xc70a50 — __ZN4Ogre15CompositionPass27setStencilTwoSidedOperationEb
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionPass::setStencilTwoSidedOperation(bool)")]
#[doc(alias = "__ZN4Ogre15CompositionPass27setStencilTwoSidedOperationEb")]
pub fn stub_c70a50() -> ! {
    todo!("0xc70a50 Ogre::CompositionPass::setStencilTwoSidedOperation(bool)")
}

// 0xc70a58 — __ZN4Ogre15CompositionPass17setQuadFarCornersEbb
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, bool, bool)
#[doc(alias = "Ogre::CompositionPass::setQuadFarCorners(bool,bool)")]
#[doc(alias = "__ZN4Ogre15CompositionPass17setQuadFarCornersEbb")]
pub fn stub_c70a58() -> ! {
    todo!("0xc70a58 Ogre::CompositionPass::setQuadFarCorners(bool,bool)")
}

// 0xc70a64 — __ZN4Ogre15CompositionPass13setCustomTypeERKSs
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionPass::setCustomType(std::string const&)")]
#[doc(alias = "__ZN4Ogre15CompositionPass13setCustomTypeERKSs")]
pub fn stub_c70a64() -> ! {
    todo!("0xc70a64 Ogre::CompositionPass::setCustomType(std::string const&)")
}

// 0xc70a70 — __ZN4Ogre15CompositionPass12_isSupportedEv
// type: _DWORD __fastcall(Ogre::CompositionPass *__hidden this)
#[doc(alias = "Ogre::CompositionPass::_isSupported(void)")]
#[doc(alias = "__ZN4Ogre15CompositionPass12_isSupportedEv")]
pub fn stub_c70a70() -> ! {
    todo!("0xc70a70 Ogre::CompositionPass::_isSupported(void)")
}

// 0xc70ad8 — __ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, Ogre::CompositionTechnique *)
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassC1EPNS_20CompositionTechniqueE")]
pub fn stub_c70ad8() -> ! {
    todo!("0xc70ad8 Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")
}

// 0xc70ae4 — __ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, Ogre::CompositionTechnique *)
#[doc(alias = "Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassC2EPNS_20CompositionTechniqueE")]
pub fn stub_c70ae4() -> ! {
    todo!("0xc70ae4 Ogre::CompositionTargetPass::CompositionTargetPass(Ogre::CompositionTechnique *)")
}

// 0xc70cb8 — __ZN4Ogre21CompositionTargetPassD1Ev
// type: void __fastcall(Ogre::CompositionTargetPass *__hidden this)
#[doc(alias = "Ogre::CompositionTargetPass::~CompositionTargetPass()")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassD1Ev")]
pub fn stub_c70cb8() -> ! {
    todo!("0xc70cb8 Ogre::CompositionTargetPass::~CompositionTargetPass()")
}

// 0xc70cc4 — __ZN4Ogre21CompositionTargetPassD2Ev
// type: void __fastcall(Ogre::CompositionTargetPass *__hidden this)
#[doc(alias = "Ogre::CompositionTargetPass::~CompositionTargetPass()")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPassD2Ev")]
pub fn stub_c70cc4() -> ! {
    todo!("0xc70cc4 Ogre::CompositionTargetPass::~CompositionTargetPass()")
}

// 0xc70e08 — __ZN4Ogre21CompositionTargetPass12setInputModeENS0_9InputModeE
#[doc(alias = "Ogre::CompositionTargetPass::setInputMode(Ogre::CompositionTargetPass::InputMode)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass12setInputModeENS0_9InputModeE")]
pub fn stub_c70e08() -> ! {
    todo!("0xc70e08 Ogre::CompositionTargetPass::setInputMode(Ogre::CompositionTargetPass::InputMode)")
}

// 0xc70e0c — __ZN4Ogre21CompositionTargetPass13setOutputNameERKSs
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionTargetPass::setOutputName(std::string const&)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass13setOutputNameERKSs")]
pub fn stub_c70e0c() -> ! {
    todo!("0xc70e0c Ogre::CompositionTargetPass::setOutputName(std::string const&)")
}

// 0xc70e18 — __ZN4Ogre21CompositionTargetPass14setOnlyInitialEb
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionTargetPass::setOnlyInitial(bool)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass14setOnlyInitialEb")]
pub fn stub_c70e18() -> ! {
    todo!("0xc70e18 Ogre::CompositionTargetPass::setOnlyInitial(bool)")
}

// 0xc70e1c — __ZN4Ogre21CompositionTargetPass17setVisibilityMaskEj
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, unsigned int)
#[doc(alias = "Ogre::CompositionTargetPass::setVisibilityMask(unsigned int)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass17setVisibilityMaskEj")]
pub fn stub_c70e1c() -> ! {
    todo!("0xc70e1c Ogre::CompositionTargetPass::setVisibilityMask(unsigned int)")
}

// 0xc70e20 — __ZN4Ogre21CompositionTargetPass10setLodBiasEf
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, float)
#[doc(alias = "Ogre::CompositionTargetPass::setLodBias(float)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass10setLodBiasEf")]
pub fn stub_c70e20() -> ! {
    todo!("0xc70e20 Ogre::CompositionTargetPass::setLodBias(float)")
}

// 0xc70e24 — __ZN4Ogre21CompositionTargetPass17setMaterialSchemeERKSs
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, const std::string *)
#[doc(alias = "Ogre::CompositionTargetPass::setMaterialScheme(std::string const&)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass17setMaterialSchemeERKSs")]
pub fn stub_c70e24() -> ! {
    todo!("0xc70e24 Ogre::CompositionTargetPass::setMaterialScheme(std::string const&)")
}

// 0xc70e30 — __ZN4Ogre21CompositionTargetPass17setShadowsEnabledEb
// type: _DWORD __fastcall(Ogre::CompositionTargetPass *__hidden this, bool)
#[doc(alias = "Ogre::CompositionTargetPass::setShadowsEnabled(bool)")]
#[doc(alias = "__ZN4Ogre21CompositionTargetPass17setShadowsEnabledEb")]
pub fn stub_c70e30() -> ! {
    todo!("0xc70e30 Ogre::CompositionTargetPass::setShadowsEnabled(bool)")
}
