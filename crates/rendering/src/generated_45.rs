//! rendering generated_45 — Ogre::|G3D:: strict 13333 total (13663 substr Ogre|G3D), 5668 prior, 100 this batch — 0xb9a190..0xbadd64
//! EA-sorted ascending earliest gap after 0xb9a170 — rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.


#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xb9a190 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()
// IDA 0xb9a190: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9a190() {
}

// 0xb9a230 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::GpuSharedParameters>::destroy(void)
// IDA 0xb9a230: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a230() {
}

// 0xb9a268 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::swap(Ogre::SharedPtr<Ogre::GpuSharedParameters>&)")]
// was: Ogre::SharedPtr<Ogre::GpuSharedParameters>::swap(Ogre::SharedPtr<Ogre::GpuSharedParameters>&)
// IDA 0xb9a268: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a268() {
}

// 0xb9a284 — __ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)")]
// was: RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)
// IDA 0xb9a284: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a284() {
}

// 0xb9a914 — __ZN3RBX7MeshGen21pushVerticesTransformERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::MeshGen::pushVerticesTransform(G3D::CoordinateFrame const&)")]
// was: RBX::MeshGen::pushVerticesTransform(G3D::CoordinateFrame const&)
// IDA 0xb9a914: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b9a914() {
}

// 0xb9a920 — __ZN4Ogre9SharedPtrINS_4MeshEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()
// IDA 0xb9a920: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9a920() {
}

// 0xb9a9c0 — __ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::Mesh>::destroy(void)
// IDA 0xb9a9c0: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a9c0() {
}

// 0xb9a9f8 — __ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::swap(Ogre::SharedPtr<Ogre::Mesh>&)")]
// was: Ogre::SharedPtr<Ogre::Mesh>::swap(Ogre::SharedPtr<Ogre::Mesh>&)
// IDA 0xb9a9f8: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a9f8() {
}

// 0xb9aa2c — __ZN4Ogre4Node11setListenerEPNS0_8ListenerE
#[doc(alias = "Ogre::Node::setListener(Ogre::Node::Listener *)")]
// was: Ogre::Node::setListener(Ogre::Node::Listener *)
// IDA 0xb9aa2c: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa2c() {
}

// 0xb9aa34 — __ZNK4Ogre4Node11getListenerEv
#[doc(alias = "Ogre::Node::getListener(void)const")]
// was: Ogre::Node::getListener(void)const
// IDA 0xb9aa34: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa34() {
}

// 0xb9aa3c — __ZN4Ogre4Node10setUserAnyERKNS_3AnyE
#[doc(alias = "Ogre::Node::setUserAny(Ogre::Any const&)")]
// was: Ogre::Node::setUserAny(Ogre::Any const&)
// IDA 0xb9aa3c: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa3c() {
}

// 0xb9aa44 — __ZNK4Ogre4Node10getUserAnyEv
#[doc(alias = "Ogre::Node::getUserAny(void)const")]
// was: Ogre::Node::getUserAny(void)const
// IDA 0xb9aa44: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa44() {
}

// 0xb9aa4c — __ZNK4Ogre9SceneNode14isInSceneGraphEv
#[doc(alias = "Ogre::SceneNode::isInSceneGraph(void)const")]
// was: Ogre::SceneNode::isInSceneGraph(void)const
// IDA 0xb9aa4c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa4c() {
}

// 0xb9aa54 — __ZN4Ogre9SceneNode15_notifyRootNodeEv
#[doc(alias = "Ogre::SceneNode::_notifyRootNode(void)")]
// was: Ogre::SceneNode::_notifyRootNode(void)
// IDA 0xb9aa54: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa54() {
}

// 0xb9aa5c — __ZN4Ogre9SceneNode18getAutoTrackTargetEv
#[doc(alias = "Ogre::SceneNode::getAutoTrackTarget(void)")]
// was: Ogre::SceneNode::getAutoTrackTarget(void)
// IDA 0xb9aa5c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa5c() {
}

// 0xb9aa64 — __ZN4Ogre9SceneNode18getAutoTrackOffsetEv
#[doc(alias = "Ogre::SceneNode::getAutoTrackOffset(void)")]
// was: Ogre::SceneNode::getAutoTrackOffset(void)
// IDA 0xb9aa64: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa64() {
}

// 0xb9aa6c — __ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv
#[doc(alias = "Ogre::SceneNode::getAutoTrackLocalDirection(void)")]
// was: Ogre::SceneNode::getAutoTrackLocalDirection(void)
// IDA 0xb9aa6c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa6c() {
}

// 0xb9b3bc — __ZN4Ogre17istreamDataStreamC1EPSib
#[doc(alias = "Ogre::istreamDataStream::istreamDataStream(std::istream *,bool)")]
// was: Ogre::istreamDataStream::istreamDataStream(std::istream *,bool)
// IDA 0xb9b3bc: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9b3bc() {
}

// 0xb9b52c — __ZN4Ogre17istreamDataStreamD0Ev
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
// was: Ogre::istreamDataStream::~istreamDataStream()
// IDA 0xb9b52c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9b52c() {
}

// 0xb9b5e0 — __ZN4Ogre17istreamDataStreamD1Ev
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
// was: Ogre::istreamDataStream::~istreamDataStream()
// IDA 0xb9b5e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b9b5e0() {
}

// 0xb9b5e4 — __ZN4Ogre17istreamDataStreamD2Ev
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
// was: Ogre::istreamDataStream::~istreamDataStream()
// IDA 0xb9b5e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9b5e4() {
}

// 0xb9b744 — __ZN4Ogre17istreamDataStream4readEPvm
#[doc(alias = "Ogre::istreamDataStream::read(void *,unsigned long)")]
// was: Ogre::istreamDataStream::read(void *,unsigned long)
// IDA 0xb9b744: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9b744() {
}

// 0xb9b758 — __ZN4Ogre17istreamDataStream8readLineEPcmRKSs
#[doc(alias = "Ogre::istreamDataStream::readLine(char *,unsigned long,std::string const&)")]
// was: Ogre::istreamDataStream::readLine(char *,unsigned long,std::string const&)
// IDA 0xb9b758: 393 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9b758() {
}

// 0xb9bbd0 — __ZN4Ogre17istreamDataStream4skipEl
#[doc(alias = "Ogre::istreamDataStream::skip(long)")]
// was: Ogre::istreamDataStream::skip(long)
// IDA 0xb9bbd0: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bbd0() {
}

// 0xb9bbf8 — __ZN4Ogre17istreamDataStream4seekEm
#[doc(alias = "Ogre::istreamDataStream::seek(unsigned long)")]
// was: Ogre::istreamDataStream::seek(unsigned long)
// IDA 0xb9bbf8: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bbf8() {
}

// 0xb9bc20 — __ZNK4Ogre17istreamDataStream4tellEv
#[doc(alias = "Ogre::istreamDataStream::tell(void)const")]
// was: Ogre::istreamDataStream::tell(void)const
// IDA 0xb9bc20: 26 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bc20() {
}

// 0xb9bc64 — __ZNK4Ogre17istreamDataStream3eofEv
#[doc(alias = "Ogre::istreamDataStream::eof(void)const")]
// was: Ogre::istreamDataStream::eof(void)const
// IDA 0xb9bc64: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bc64() {
}

// 0xb9bc78 — __ZN4Ogre17istreamDataStream5closeEv
#[doc(alias = "Ogre::istreamDataStream::close(void)")]
// was: Ogre::istreamDataStream::close(void)
// IDA 0xb9bc78: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bc78() {
}

// 0xb9c320 — __ZN4Ogre7QuadricC1Ev
#[doc(alias = "Ogre::Quadric::Quadric(void)")]
// was: Ogre::Quadric::Quadric(void)
// IDA 0xb9c320: 16 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c320() {
}

// 0xb9c344 — __ZN4Ogre7Quadric9setOriginERKNS_7Vector3E
#[doc(alias = "Ogre::Quadric::setOrigin(Ogre::Vector3 const&)")]
// was: Ogre::Quadric::setOrigin(Ogre::Vector3 const&)
// IDA 0xb9c344: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c344() {
}

// 0xb9c358 — __ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii
#[doc(alias = "Ogre::Quadric::createCylinder(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,float,int,int)")]
// was: Ogre::Quadric::createCylinder(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,float,int,int)
// IDA 0xb9c358: 2824 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c358() {
}

// 0xb9e7e8 — __ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii
#[doc(alias = "Ogre::Quadric::createDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int)")]
// was: Ogre::Quadric::createDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int)
// IDA 0xb9e7e8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9e7e8() {
}

// 0xb9e830 — __ZN4Ogre7Quadric17createPartialDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffiiff
#[doc(alias = "Ogre::Quadric::createPartialDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int,float,float)")]
// was: Ogre::Quadric::createPartialDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int,float,float)
// IDA 0xb9e830: 2903 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9e830() {
}

// 0xba0b70 — __ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii
#[doc(alias = "Ogre::Quadric::createSphere(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,int,int)")]
// was: Ogre::Quadric::createSphere(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,int,int)
// IDA 0xba0b70: 3827 insns (PUSH..TBH.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba0b70() {
}

// 0xba3c7c — __ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)")]
// was: std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)
// IDA 0xba3c7c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ba3c7c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xba4494 — __ZNK4Ogre10RbxArchive15isCaseSensitiveEv
#[doc(alias = "Ogre::RbxArchive::isCaseSensitive(void)const")]
// was: Ogre::RbxArchive::isCaseSensitive(void)const
// IDA 0xba4494: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba4494() {
}

// 0xba4498 — __ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
#[doc(alias = "Ogre::RbxArchive::doStaticFindFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
// was: Ogre::RbxArchive::doStaticFindFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const
// IDA 0xba4498: 499 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba4498() {
}

// 0xba4a18 — __ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
#[doc(alias = "Ogre::RbxArchive::findFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
// was: Ogre::RbxArchive::findFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const
// IDA 0xba4a18: 862 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba4a18() {
}

// 0xba5874 — __ZN4OgreL16concatenate_pathERKSsS1_
#[doc(alias = "Ogre::concatenate_path(std::string const&,std::string const&)")]
// was: Ogre::concatenate_path(std::string const&,std::string const&)
// IDA 0xba5874: 167 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba5874() {
}

// 0xba5a54 — __ZN4Ogre10RbxArchiveD0Ev
#[doc(alias = "Ogre::RbxArchive::~RbxArchive()")]
// was: Ogre::RbxArchive::~RbxArchive()
// IDA 0xba5a54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba5a54() {
}

// 0xba5af0 — __ZN4Ogre10RbxArchiveD1Ev
#[doc(alias = "Ogre::RbxArchive::~RbxArchive()")]
// was: Ogre::RbxArchive::~RbxArchive()
// IDA 0xba5af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba5af0() {
}

// 0xba5b88 — __ZN4Ogre10RbxArchive4loadEv
#[doc(alias = "Ogre::RbxArchive::load(void)")]
// was: Ogre::RbxArchive::load(void)
// IDA 0xba5b88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ba5b88() {
}

// 0xba5b8c — __ZN4Ogre10RbxArchive6unloadEv
#[doc(alias = "Ogre::RbxArchive::unload(void)")]
// was: Ogre::RbxArchive::unload(void)
// IDA 0xba5b8c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ba5b8c() {
}

// 0xba5b90 — __ZNK4Ogre10RbxArchive4openERKSsb
#[doc(alias = "Ogre::RbxArchive::open(std::string const&,bool)const")]
// was: Ogre::RbxArchive::open(std::string const&,bool)const
// IDA 0xba5b90: 619 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba5b90() {
}

// 0xba6244 — __ZN4Ogre10RbxArchive4listEbb
#[doc(alias = "Ogre::RbxArchive::list(bool,bool)")]
// was: Ogre::RbxArchive::list(bool,bool)
// IDA 0xba6244: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6244() {
}

// 0xba6460 — __ZN4Ogre10RbxArchive12listFileInfoEbb
#[doc(alias = "Ogre::RbxArchive::listFileInfo(bool,bool)")]
// was: Ogre::RbxArchive::listFileInfo(bool,bool)
// IDA 0xba6460: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6460() {
}

// 0xba667c — __ZN4Ogre10RbxArchive4findERKSsbb
#[doc(alias = "Ogre::RbxArchive::find(std::string const&,bool,bool)")]
// was: Ogre::RbxArchive::find(std::string const&,bool,bool)
// IDA 0xba667c: 147 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba667c() {
}

// 0xba67f8 — __ZNK4Ogre10RbxArchive12findFileInfoERKSsbb
#[doc(alias = "Ogre::RbxArchive::findFileInfo(std::string const&,bool,bool)const")]
// was: Ogre::RbxArchive::findFileInfo(std::string const&,bool,bool)const
// IDA 0xba67f8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba67f8() {
}

// 0xba6974 — __ZN4Ogre10RbxArchive12makeFullPathERKSs
#[doc(alias = "Ogre::RbxArchive::makeFullPath(std::string const&)")]
// was: Ogre::RbxArchive::makeFullPath(std::string const&)
// IDA 0xba6974: 329 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6974() {
}

// 0xba6f04 — __ZN4Ogre10RbxArchive6existsERKSs
#[doc(alias = "Ogre::RbxArchive::exists(std::string const&)")]
// was: Ogre::RbxArchive::exists(std::string const&)
// IDA 0xba6f04: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6f04() {
}

// 0xba7028 — __ZN4Ogre10RbxArchive15getModifiedTimeERKSs
#[doc(alias = "Ogre::RbxArchive::getModifiedTime(std::string const&)")]
// was: Ogre::RbxArchive::getModifiedTime(std::string const&)
// IDA 0xba7028: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7028() {
}

// 0xba7150 — __ZN4Ogre17RbxArchiveFactoryC2EPN3RBX15ContentProviderE
#[doc(alias = "Ogre::RbxArchiveFactory::RbxArchiveFactory(RBX::ContentProvider *)")]
// was: Ogre::RbxArchiveFactory::RbxArchiveFactory(RBX::ContentProvider *)
// IDA 0xba7150: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7150() {
}

// 0xba72c4 — __ZN4Ogre17RbxArchiveFactory18getArchiveTypeNameEPN3RBX15ContentProviderE
#[doc(alias = "Ogre::RbxArchiveFactory::getArchiveTypeName(RBX::ContentProvider *)")]
// was: Ogre::RbxArchiveFactory::getArchiveTypeName(RBX::ContentProvider *)
// IDA 0xba72c4: 172 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba72c4() {
}

// 0xba74cc — __ZNK4Ogre17RbxArchiveFactory7getTypeEv
#[doc(alias = "Ogre::RbxArchiveFactory::getType(void)const")]
// was: Ogre::RbxArchiveFactory::getType(void)const
// IDA 0xba74cc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba74cc() {
}

// 0xba74d0 — __ZN4Ogre17RbxArchiveFactory9singletonEv
#[doc(alias = "Ogre::RbxArchiveFactory::singleton(void)")]
// was: Ogre::RbxArchiveFactory::singleton(void)
// IDA 0xba74d0: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba74d0() {
}

// 0xba75a8 — __ZN4Ogre17RbxArchiveFactoryD0Ev
#[doc(alias = "Ogre::RbxArchiveFactory::~RbxArchiveFactory()")]
// was: Ogre::RbxArchiveFactory::~RbxArchiveFactory()
// IDA 0xba75a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba75a8() {
}

// 0xba7618 — __ZN4Ogre17RbxArchiveFactoryD1Ev
#[doc(alias = "Ogre::RbxArchiveFactory::~RbxArchiveFactory()")]
// was: Ogre::RbxArchiveFactory::~RbxArchiveFactory()
// IDA 0xba7618: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba7618() {
}

// 0xba7684 — __ZN4Ogre17RbxArchiveFactory10destroyAllEv
#[doc(alias = "Ogre::RbxArchiveFactory::destroyAll(void)")]
// was: Ogre::RbxArchiveFactory::destroyAll(void)
// IDA 0xba7684: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7684() {
}

// 0xba769c — __ZN4Ogre17RbxArchiveFactory14createInstanceERKSs
#[doc(alias = "Ogre::RbxArchiveFactory::createInstance(std::string const&)")]
// was: Ogre::RbxArchiveFactory::createInstance(std::string const&)
// IDA 0xba769c: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba769c() {
}

// 0xba7888 — __ZN4Ogre17RbxArchiveFactory15destroyInstanceEPNS_7ArchiveE
#[doc(alias = "Ogre::RbxArchiveFactory::destroyInstance(Ogre::Archive *)")]
// was: Ogre::RbxArchiveFactory::destroyInstance(Ogre::Archive *)
// IDA 0xba7888: 7 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7888() {
}

// 0xba7f20 — __ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerE
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *)")]
// was: Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *)
// IDA 0xba7f20: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7f20() {
}

// 0xba7f94 — __ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerERKSs
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")]
// was: Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)
// IDA 0xba7f94: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7f94() {
}

// 0xba8008 — __ZN4Ogre20RbxCullableSceneNodeC2EPNS_12SceneManagerERKSs
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")]
// was: Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)
// IDA 0xba8008: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8008() {
}

// 0xba807c — __ZN4Ogre20RbxCullableSceneNodeD0Ev
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
// was: Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()
// IDA 0xba807c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba807c() {
}

// 0xba8130 — __ZN4Ogre20RbxCullableSceneNodeD1Ev
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
// was: Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()
// IDA 0xba8130: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_ba8130() {
}

// 0xba8134 — __ZN4Ogre20RbxCullableSceneNodeD2Ev
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
// was: Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()
// IDA 0xba8134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba8134() {
}

// 0xba827c — __ZN4Ogre20RbxCullableSceneNode27calculateSqDistanceToCameraEPKNS_6CameraE
#[doc(alias = "Ogre::RbxCullableSceneNode::calculateSqDistanceToCamera(Ogre::Camera const*)")]
// was: Ogre::RbxCullableSceneNode::calculateSqDistanceToCamera(Ogre::Camera const*)
// IDA 0xba827c: 115 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba827c() {
}

// 0xba841c — __ZN4Ogre20RbxCullableSceneNode8IsCulledEPKNS_6CameraEb
#[doc(alias = "Ogre::RbxCullableSceneNode::IsCulled(Ogre::Camera const*,bool)")]
// was: Ogre::RbxCullableSceneNode::IsCulled(Ogre::Camera const*,bool)
// IDA 0xba841c: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba841c() {
}

// 0xba854c — __ZN4Ogre20RbxCullableSceneNode17ShouldCastShadowsEPKNS_6CameraE
#[doc(alias = "Ogre::RbxCullableSceneNode::ShouldCastShadows(Ogre::Camera const*)")]
// was: Ogre::RbxCullableSceneNode::ShouldCastShadows(Ogre::Camera const*)
// IDA 0xba854c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba854c() {
}

// 0xba8594 — __ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbN3RBX15IntersectResultE
#[doc(alias = "Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool,RBX::IntersectResult)")]
// was: Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool,RBX::IntersectResult)
// IDA 0xba8594: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8594() {
}

// 0xba85e4 — __ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
#[doc(alias = "Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
// was: Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)
// IDA 0xba85e4: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba85e4() {
}

// 0xba8750 — __ZN4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv
#[doc(alias = "Ogre::RbxCullableSceneNode::getFastFuzzyExtents(void)")]
// was: Ogre::RbxCullableSceneNode::getFastFuzzyExtents(void)
// IDA 0xba8750: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8750() {
}

// 0xba876c — __ZThn392_N4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv
#[doc(alias = "non-virtual thunk toOgre::RbxCullableSceneNode::getFastFuzzyExtents(void)")]
// was: non-virtual thunk to Ogre::RbxCullableSceneNode::getFastFuzzyExtents(void)
// IDA 0xba876c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba876c() {
}

// 0xba8e18 — __ZN4Ogre9RbxEntityC1Ev
#[doc(alias = "Ogre::RbxEntity::RbxEntity(void)")]
// was: Ogre::RbxEntity::RbxEntity(void)
// IDA 0xba8e18: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8e18() {
}

// 0xba8eb4 — __ZN4Ogre9RbxEntity7setMeshENS_7MeshPtrE
#[doc(alias = "Ogre::RbxEntity::setMesh(Ogre::MeshPtr)")]
// was: Ogre::RbxEntity::setMesh(Ogre::MeshPtr)
// IDA 0xba8eb4: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8eb4() {
}

// 0xba8f9c — __ZN4Ogre9RbxEntity16clearSubEntitiesEv
#[doc(alias = "Ogre::RbxEntity::clearSubEntities(void)")]
// was: Ogre::RbxEntity::clearSubEntities(void)
// IDA 0xba8f9c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8f9c() {
}

// 0xba8fa8 — __ZN4Ogre9RbxEntity15appendSubEntityEPNS_12RbxSubEntityE
#[doc(alias = "Ogre::RbxEntity::appendSubEntity(Ogre::RbxSubEntity *)")]
// was: Ogre::RbxEntity::appendSubEntity(Ogre::RbxSubEntity *)
// IDA 0xba8fa8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8fa8() {
}

// 0xba8fdc — __ZN4Ogre9RbxEntity18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::RbxEntity::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::RbxEntity::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xba8fdc: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8fdc() {
}

// 0xba905c — __ZN4Ogre9RbxEntity20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::RbxEntity::_notifyCurrentCamera(Ogre::Camera *)")]
// was: Ogre::RbxEntity::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xba905c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba905c() {
}

// 0xba9080 — __ZN4Ogre9RbxEntity13setVisibleAllEb
#[doc(alias = "Ogre::RbxEntity::setVisibleAll(bool)")]
// was: Ogre::RbxEntity::setVisibleAll(bool)
// IDA 0xba9080: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba9080() {
}

// 0xba90b4 — __ZN4Ogre9RbxEntity4cullEPKNS_6CameraE
#[doc(alias = "Ogre::RbxEntity::cull(Ogre::Camera const*)")]
// was: Ogre::RbxEntity::cull(Ogre::Camera const*)
// IDA 0xba90b4: 181 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba90b4() {
}

// 0xba92b8 — __ZN4Ogre9RbxEntityD0Ev
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
// was: Ogre::RbxEntity::~RbxEntity()
// IDA 0xba92b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba92b8() {
}

// 0xba936c — __ZN4Ogre9RbxEntityD1Ev
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
// was: Ogre::RbxEntity::~RbxEntity()
// IDA 0xba936c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_ba936c() {
}

// 0xba9370 — __ZThn4_N4Ogre9RbxEntityD0Ev
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
// was: non-virtual thunk to Ogre::RbxEntity::~RbxEntity()
// IDA 0xba9370: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba9370() {
}

// 0xba9428 — __ZThn188_N4Ogre9RbxEntityD0Ev
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
// was: non-virtual thunk to Ogre::RbxEntity::~RbxEntity()
// IDA 0xba9428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba9428() {
}

// 0xba94e0 — __ZN4Ogre9RbxEntityD2Ev
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
// was: Ogre::RbxEntity::~RbxEntity()
// IDA 0xba94e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba94e0() {
}

// 0xba9694 — __ZThn4_N4Ogre9RbxEntityD1Ev
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
// was: non-virtual thunk to Ogre::RbxEntity::~RbxEntity()
// IDA 0xba9694: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba9694() {
}

// 0xba969c — __ZThn188_N4Ogre9RbxEntityD1Ev
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
// was: non-virtual thunk to Ogre::RbxEntity::~RbxEntity()
// IDA 0xba969c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba969c() {
}

// 0xba96a4 — __ZNSt6vectorIN4Ogre9BlockSortESaIS1_EE7reserveEm
#[doc(alias = "std::vector<Ogre::BlockSort,std::allocator<Ogre::BlockSort>>::reserve(unsigned long)")]
// was: std::vector<Ogre::BlockSort,std::allocator<Ogre::BlockSort>>::reserve(unsigned long)
// IDA 0xba96a4: 46 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba96a4() {
}

// 0xba9dbc — __ZN4Ogre11RootManager9GetOrInitENS_11GraphicsAPIERKSs
#[doc(alias = "Ogre::RootManager::GetOrInit(Ogre::GraphicsAPI,std::string const&)")]
// was: Ogre::RootManager::GetOrInit(Ogre::GraphicsAPI,std::string const&)
// IDA 0xba9dbc: 774 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba9dbc() {
}

// 0xbaae98 — __ZN4Ogre11RootManagerD2Ev
#[doc(alias = "Ogre::RootManager::~RootManager()")]
// was: Ogre::RootManager::~RootManager()
// IDA 0xbaae98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_baae98() {
}

// 0xbab8f8 — __ZN4Ogre11RootManager16cleanUpResourcesERNS_15ResourceManagerERKSsS4_RKSt6vectorISsSaISsEE
#[doc(alias = "Ogre::RootManager::cleanUpResources(Ogre::ResourceManager &,std::string const&,std::string const&,std::vector<std::string,std::allocator<std::string>> const&)")]
// was: Ogre::RootManager::cleanUpResources(Ogre::ResourceManager &,std::string const&,std::string const&,std::vector<std::string,std::allocator<std::string>> const&)
// IDA 0xbab8f8: 1348 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bab8f8() {
}

// 0xbac7fc — __ZN4Ogre11RootManager14printResourcesERNS_15ResourceManagerERKSs
#[doc(alias = "Ogre::RootManager::printResources(Ogre::ResourceManager &,std::string const&)")]
// was: Ogre::RootManager::printResources(Ogre::ResourceManager &,std::string const&)
// IDA 0xbac7fc: 718 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bac7fc() {
}

// 0xbacfcc — __ZN4Ogre12VisualEngineC1Ev
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
// was: Ogre::VisualEngine::VisualEngine(void)
// IDA 0xbacfcc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bacfcc() {
}

// 0xbacfd0 — __ZN4Ogre12VisualEngineC2Ev
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
// was: Ogre::VisualEngine::VisualEngine(void)
// IDA 0xbacfd0: 294 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bacfd0() {
}

// 0xbad2c8 — __ZN4Ogre12VisualEngineD0Ev
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
// was: Ogre::VisualEngine::~VisualEngine()
// IDA 0xbad2c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bad2c8() {
}

// 0xbad368 — __ZN4Ogre12VisualEngineD1Ev
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
// was: Ogre::VisualEngine::~VisualEngine()
// IDA 0xbad368: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bad368() {
}

// 0xbad36c — __ZN4Ogre12VisualEngineD2Ev
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
// was: Ogre::VisualEngine::~VisualEngine()
// IDA 0xbad36c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bad36c() {
}

// 0xbadb50 — __ZN4Ogre12VisualEngine7setViewEPN3RBX8ViewBaseE
#[doc(alias = "Ogre::VisualEngine::setView(RBX::ViewBase *)")]
// was: Ogre::VisualEngine::setView(RBX::ViewBase *)
// IDA 0xbadb50: 199 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_badb50() {
}

// 0xbadd64 — __ZN4Ogre14SaveRBXDbgInfoEPKNS_24RenderSystemCapabilitiesE
#[doc(alias = "Ogre::SaveRBXDbgInfo(Ogre::RenderSystemCapabilities const*)")]
// was: Ogre::SaveRBXDbgInfo(Ogre::RenderSystemCapabilities const*)
// IDA 0xbadd64: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_badd64() {
}
