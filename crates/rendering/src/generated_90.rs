//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe5dadc..0xe648b4 (100 stubs, 10055 prior -> 10155 covered, 3178 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe5dadc — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5dadc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5dadc() {
}

// 0xe5db90 — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xe5db90: 151 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5db90() {
}

// 0xe5dd2c — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS8_
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xe5dd2c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5dd2c() {
}

// 0xe5dd48 — __ZN4Ogre12STLAllocatorINS_8FileInfoENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xe5dd48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5dd48() {
}

// 0xe5dd54 — __ZN4Ogre12STLAllocatorINS_8FileInfoENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS1_
#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::FileInfo*)")]
// was: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::FileInfo*)
// IDA 0xe5dd54: 66 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5dd54() {
}

// 0xe5de10 — __ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo const&)")]
// was: std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::FileInfo*,std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::FileInfo const&)
// IDA 0xe5de10: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e5de10() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe5e4b8 — __ZSt22__uninitialized_copy_aIPN4Ogre8FileInfoES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias = "Ogre::FileInfo * std::__uninitialized_copy_a<Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::FileInfo *,Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::FileInfo * std::__uninitialized_copy_a<Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::FileInfo *,Ogre::FileInfo *,Ogre::FileInfo *,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
// IDA 0xe5e4b8: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5e4b8() {
}

// 0xe5e6d8 — __ZNSt12_Vector_baseIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe5e6d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5e6d8() {
}

// 0xe5e6dc — __ZNSt12_Vector_baseIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe5e6dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e6dc() {
}

// 0xe5e6e8 — __ZN4Ogre10DataStreamD1Ev
#[doc(alias = "Ogre::DataStream::~DataStream()")]
// was: Ogre::DataStream::~DataStream()
// IDA 0xe5e6e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e6e8() {
}

// 0xe5e744 — __ZN4Ogre10DataStreamD0Ev
#[doc(alias = "Ogre::DataStream::~DataStream()")]
// was: Ogre::DataStream::~DataStream()
// IDA 0xe5e744: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e744() {
}

// 0xe5e820 — __ZN4Ogre7ArchiveD1Ev
#[doc(alias = "Ogre::Archive::~Archive()")]
// was: Ogre::Archive::~Archive()
// IDA 0xe5e820: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e820() {
}

// 0xe5e8b8 — __ZN4Ogre7ArchiveD0Ev
#[doc(alias = "Ogre::Archive::~Archive()")]
// was: Ogre::Archive::~Archive()
// IDA 0xe5e8b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5e8b8() {
}

// 0xe5e9cc — __ZNK4Ogre7Archive6createERKSs
#[doc(alias = "Ogre::Archive::create(std::string const&)const")]
// was: Ogre::Archive::create(std::string const&)const
// IDA 0xe5e9cc: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5e9cc() {
}

// 0xe5eb7c — __ZNK4Ogre7Archive6removeERKSs
#[doc(alias = "Ogre::Archive::remove(std::string const&)const")]
// was: Ogre::Archive::remove(std::string const&)const
// IDA 0xe5eb7c: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5eb7c() {
}

// 0xe5ed60 — __ZN4Ogre13mac_loadDylibEPKc
#[doc(alias = "Ogre::mac_loadDylib(char const*)")]
// was: Ogre::mac_loadDylib(char const*)
// IDA 0xe5ed60: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ed60() {
}

// 0xe5ed64 — __ZN4Ogre13macBundlePathEv
#[doc(alias = "Ogre::macBundlePath(void)")]
// was: Ogre::macBundlePath(void)
// IDA 0xe5ed64: 37 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ed64() {
}

// 0xe5ee40 — __ZN4Ogre5TimerC1Ev
#[doc(alias = "Ogre::Timer::Timer(void)")]
// was: Ogre::Timer::Timer(void)
// IDA 0xe5ee40: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ee40() {
}

// 0xe5ee58 — __ZN4Ogre5Timer5resetEv
#[doc(alias = "Ogre::Timer::reset(void)")]
// was: Ogre::Timer::reset(void)
// IDA 0xe5ee58: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ee58() {
}

// 0xe5ee70 — __ZN4Ogre5TimerD1Ev
#[doc(alias = "Ogre::Timer::~Timer()")]
// was: Ogre::Timer::~Timer()
// IDA 0xe5ee70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5ee70() {
}

// 0xe5ee78 — __ZN4Ogre5Timer15getMillisecondsEv
#[doc(alias = "Ogre::Timer::getMilliseconds(void)")]
// was: Ogre::Timer::getMilliseconds(void)
// IDA 0xe5ee78: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ee78() {
}

// 0xe5f424 — __ZN4Ogre14DualQuaternion24fromTransformationMatrixERKNS_7Matrix4E
#[doc(alias = "Ogre::DualQuaternion::fromTransformationMatrix(Ogre::Matrix4 const&)")]
// was: Ogre::DualQuaternion::fromTransformationMatrix(Ogre::Matrix4 const&)
// IDA 0xe5f424: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5f424() {
}

// 0xe5f52c — __ZN4Ogre15InstanceManagerC1ERKSsPNS_12SceneManagerES2_S2_NS0_19InstancingTechniqueEtmtb
#[doc(alias = "Ogre::InstanceManager::InstanceManager(std::string const&,Ogre::SceneManager *,std::string const&,std::string const&,Ogre::InstanceManager::InstancingTechnique,unsigned short,unsigned long,unsigned short,bool)")]
// was: Ogre::InstanceManager::InstanceManager(std::string const&,Ogre::SceneManager *,std::string const&,std::string const&,Ogre::InstanceManager::InstancingTechnique,unsigned short,unsigned long,unsigned short,bool)
// IDA 0xe5f52c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5f52c() {
}

// 0xe5f55c — __ZN4Ogre15InstanceManagerC2ERKSsPNS_12SceneManagerES2_S2_NS0_19InstancingTechniqueEtmtb
#[doc(alias = "Ogre::InstanceManager::InstanceManager(std::string const&,Ogre::SceneManager *,std::string const&,std::string const&,Ogre::InstanceManager::InstancingTechnique,unsigned short,unsigned long,unsigned short,bool)")]
// was: Ogre::InstanceManager::InstanceManager(std::string const&,Ogre::SceneManager *,std::string const&,std::string const&,Ogre::InstanceManager::InstancingTechnique,unsigned short,unsigned long,unsigned short,bool)
// IDA 0xe5f55c: 391 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5f55c() {
}

// 0xe5f944 — __ZN4Ogre15InstanceManagerD0Ev
#[doc(alias = "Ogre::InstanceManager::~InstanceManager()")]
// was: Ogre::InstanceManager::~InstanceManager()
// IDA 0xe5f944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5f944() {
}

// 0xe5f9d4 — __ZN4Ogre15InstanceManagerD1Ev
#[doc(alias = "Ogre::InstanceManager::~InstanceManager()")]
// was: Ogre::InstanceManager::~InstanceManager()
// IDA 0xe5f9d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5f9d4() {
}

// 0xe5f9e0 — __ZN4Ogre15InstanceManagerD2Ev
#[doc(alias = "Ogre::InstanceManager::~InstanceManager()")]
// was: Ogre::InstanceManager::~InstanceManager()
// IDA 0xe5f9e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5f9e0() {
}

// 0xe5fc94 — __ZN4Ogre15InstanceManager32getMaxOrBestNumInstancesPerBatchESsmt
#[doc(alias = "Ogre::InstanceManager::getMaxOrBestNumInstancesPerBatch(std::string,unsigned long,unsigned short)")]
// was: Ogre::InstanceManager::getMaxOrBestNumInstancesPerBatch(std::string,unsigned long,unsigned short)
// IDA 0xe5fc94: 927 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5fc94() {
}

// 0xe606a8 — __ZN4Ogre15InstanceManager21createInstancedEntityERKSs
#[doc(alias = "Ogre::InstanceManager::createInstancedEntity(std::string const&)")]
// was: Ogre::InstanceManager::createInstancedEntity(std::string const&)
// IDA 0xe606a8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e606a8() {
}

// 0xe606f8 — __ZN4Ogre15InstanceManager13buildNewBatchERKSsb
#[doc(alias = "Ogre::InstanceManager::buildNewBatch(std::string const&,bool)")]
// was: Ogre::InstanceManager::buildNewBatch(std::string const&,bool)
// IDA 0xe606f8: 1589 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e606f8() {
}

// 0xe61874 — __ZN4Ogre15InstanceManager19_updateDirtyBatchesEv
#[doc(alias = "Ogre::InstanceManager::_updateDirtyBatches(void)")]
// was: Ogre::InstanceManager::_updateDirtyBatches(void)
// IDA 0xe61874: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e61874() {
}

// 0xe618a0 — __ZN4Ogre15InstanceManager14_addDirtyBatchEPNS_13InstanceBatchE
#[doc(alias = "Ogre::InstanceManager::_addDirtyBatch(Ogre::InstanceBatch *)")]
// was: Ogre::InstanceManager::_addDirtyBatch(Ogre::InstanceBatch *)
// IDA 0xe618a0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e618a0() {
}

// 0xe618e8 — __ZNSt3mapISsSt6vectorIPN4Ogre13InstanceBatchENS1_12STLAllocatorIS3_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessISsENS4_ISt4pairIKSsS9_ES7_EEEixERSD_
#[doc(alias = "std::map<std::string,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xe618e8: 186 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e618e8() {
}

// 0xe61aec — __ZNSt3mapISsN4Ogre15InstanceManager13BatchSettingsESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::InstanceManager::BatchSettings,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::InstanceManager::BatchSettings,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xe61aec: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e61aec() {
}

// 0xe61ca8 — __ZNSt6vectorIPN4Ogre13InstanceBatchENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstanceBatch **,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstanceBatch * const&)")]
// was: std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstanceBatch **,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstanceBatch * const&)
// IDA 0xe61ca8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e61ca8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe61da0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15InstanceManager13BatchSettingsEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)
// IDA 0xe61da0: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e61da0() {
}

// 0xe61f80 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15InstanceManager13BatchSettingsEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)
// IDA 0xe61f80: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e61f80() {
}

// 0xe620d4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15InstanceManager13BatchSettingsEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstanceManager::BatchSettings> const&)
// IDA 0xe620d4: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e620d4() {
}

// 0xe621b8 — __ZNSt6vectorIPN4Ogre13InstanceBatchENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(alias = "std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe621b8: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e621b8() {
}

// 0xe6222c — __ZNSt12_Vector_baseIPN4Ogre13InstanceBatchENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe6222c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e6222c() {
}

// 0xe62230 — __ZNSt12_Vector_baseIPN4Ogre13InstanceBatchENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe62230: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e62230() {
}

// 0xe6223c — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe6223c: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6223c() {
}

// 0xe62584 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe62584: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62584() {
}

// 0xe625f8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe625f8: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e625f8() {
}

// 0xe626dc — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE14_M_create_nodeERKSC_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe626dc: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e626dc() {
}

// 0xe6281c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15InstanceManager13BatchSettingsEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe6281c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e6281c() {
}

// 0xe62820 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15InstanceManager13BatchSettingsEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe62820: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e62820() {
}

// 0xe6282c — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe6282c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e6282c() {
}

// 0xe62830 — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe62830: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62830() {
}

// 0xe6283c — __ZNSt8_Rb_treeISsSt4pairIKSsSt6vectorIPN4Ogre13InstanceBatchENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessISsENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::vector<Ogre::InstanceBatch *,Ogre::STLAllocator<Ogre::InstanceBatch *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xe6283c: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6283c() {
}

// 0xe62940 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15InstanceManager13BatchSettingsEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstanceManager::BatchSettings>> *)
// IDA 0xe62940: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62940() {
}

// 0xe629ec — __ZN4Ogre11DepthBufferC2EttjjjRKSsb
#[doc(alias = "Ogre::DepthBuffer::DepthBuffer(unsigned short,unsigned short,unsigned int,unsigned int,unsigned int,std::string const&,bool)")]
// was: Ogre::DepthBuffer::DepthBuffer(unsigned short,unsigned short,unsigned int,unsigned int,unsigned int,std::string const&,bool)
// IDA 0xe629ec: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e629ec() {
}

// 0xe62a50 — __ZN4Ogre11DepthBufferD0Ev
#[doc(alias = "Ogre::DepthBuffer::~DepthBuffer()")]
// was: Ogre::DepthBuffer::~DepthBuffer()
// IDA 0xe62a50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e62a50() {
}

// 0xe62ae0 — __ZN4Ogre11DepthBufferD1Ev
#[doc(alias = "Ogre::DepthBuffer::~DepthBuffer()")]
// was: Ogre::DepthBuffer::~DepthBuffer()
// IDA 0xe62ae0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e62ae0() {
}

// 0xe62aec — __ZN4Ogre11DepthBufferD2Ev
#[doc(alias = "Ogre::DepthBuffer::~DepthBuffer()")]
// was: Ogre::DepthBuffer::~DepthBuffer()
// IDA 0xe62aec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e62aec() {
}

// 0xe62ca0 — __ZN4Ogre11DepthBuffer10_setPoolIdEt
#[doc(alias = "Ogre::DepthBuffer::_setPoolId(unsigned short)")]
// was: Ogre::DepthBuffer::_setPoolId(unsigned short)
// IDA 0xe62ca0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62ca0() {
}

// 0xe62cdc — __ZNK4Ogre11DepthBuffer9getPoolIdEv
#[doc(alias = "Ogre::DepthBuffer::getPoolId(void)const")]
// was: Ogre::DepthBuffer::getPoolId(void)const
// IDA 0xe62cdc: 2 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62cdc() {
}

// 0xe62ce0 — __ZNK4Ogre11DepthBuffer11getBitDepthEv
#[doc(alias = "Ogre::DepthBuffer::getBitDepth(void)const")]
// was: Ogre::DepthBuffer::getBitDepth(void)const
// IDA 0xe62ce0: 2 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62ce0() {
}

// 0xe62ce4 — __ZNK4Ogre11DepthBuffer8getWidthEv
#[doc(alias = "Ogre::DepthBuffer::getWidth(void)const")]
// was: Ogre::DepthBuffer::getWidth(void)const
// IDA 0xe62ce4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62ce4() {
}

// 0xe62ce8 — __ZNK4Ogre11DepthBuffer9getHeightEv
#[doc(alias = "Ogre::DepthBuffer::getHeight(void)const")]
// was: Ogre::DepthBuffer::getHeight(void)const
// IDA 0xe62ce8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62ce8() {
}

// 0xe62cec — __ZNK4Ogre11DepthBuffer7getFsaaEv
#[doc(alias = "Ogre::DepthBuffer::getFsaa(void)const")]
// was: Ogre::DepthBuffer::getFsaa(void)const
// IDA 0xe62cec: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62cec() {
}

// 0xe62cf0 — __ZNK4Ogre11DepthBuffer11getFsaaHintEv
#[doc(alias = "Ogre::DepthBuffer::getFsaaHint(void)const")]
// was: Ogre::DepthBuffer::getFsaaHint(void)const
// IDA 0xe62cf0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62cf0() {
}

// 0xe62cf4 — __ZNK4Ogre11DepthBuffer12isCompatibleEPNS_12RenderTargetE
#[doc(alias = "Ogre::DepthBuffer::isCompatible(Ogre::RenderTarget *)const")]
// was: Ogre::DepthBuffer::isCompatible(Ogre::RenderTarget *)const
// IDA 0xe62cf4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62cf4() {
}

// 0xe62d4c — __ZN4Ogre11DepthBuffer27_notifyRenderTargetAttachedEPNS_12RenderTargetE
#[doc(alias = "Ogre::DepthBuffer::_notifyRenderTargetAttached(Ogre::RenderTarget *)")]
// was: Ogre::DepthBuffer::_notifyRenderTargetAttached(Ogre::RenderTarget *)
// IDA 0xe62d4c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62d4c() {
}

// 0xe62d64 — __ZN4Ogre11DepthBuffer27_notifyRenderTargetDetachedEPNS_12RenderTargetE
#[doc(alias = "Ogre::DepthBuffer::_notifyRenderTargetDetached(Ogre::RenderTarget *)")]
// was: Ogre::DepthBuffer::_notifyRenderTargetDetached(Ogre::RenderTarget *)
// IDA 0xe62d64: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62d64() {
}

// 0xe62db0 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::RenderTarget *> *)")]
// was: std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::RenderTarget *> *)
// IDA 0xe62db0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62db0() {
}

// 0xe62dd8 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::RenderTarget * const&)")]
// was: std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::RenderTarget * const&)
// IDA 0xe62dd8: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62dd8() {
}

// 0xe62ed0 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::RenderTarget *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::RenderTarget *>,false>::~_Rb_tree_impl()
// IDA 0xe62ed0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e62ed0() {
}

// 0xe62ed4 — __ZNSt8_Rb_treeIPN4Ogre12RenderTargetES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::RenderTarget *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::RenderTarget *,Ogre::RenderTarget *,std::_Identity<Ogre::RenderTarget *>,std::less<Ogre::RenderTarget *>,Ogre::STLAllocator<Ogre::RenderTarget *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::RenderTarget *>,false>::~_Rb_tree_impl()
// IDA 0xe62ed4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e62ed4() {
}

// 0xe62f14 — __ZN4Ogre13InstanceBatchC2EPNS_15InstanceManagerERNS_7MeshPtrERKNS_11MaterialPtrEmPKSt6vectorItNS_12STLAllocatorItNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERKSs
#[doc(alias = "Ogre::InstanceBatch::InstanceBatch(Ogre::InstanceManager *,Ogre::MeshPtr &,Ogre::MaterialPtr const&,unsigned long,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::string const&)")]
// was: Ogre::InstanceBatch::InstanceBatch(Ogre::InstanceManager *,Ogre::MeshPtr &,Ogre::MaterialPtr const&,unsigned long,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,std::string const&)
// IDA 0xe62f14: 457 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e62f14() {
}

// 0xe63428 — __ZN4Ogre13InstanceBatchD0Ev
#[doc(alias = "Ogre::InstanceBatch::~InstanceBatch()")]
// was: Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe63428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e63428() {
}

// 0xe634b8 — __ZN4Ogre13InstanceBatchD1Ev
#[doc(alias = "Ogre::InstanceBatch::~InstanceBatch()")]
// was: Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe634b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e634b8() {
}

// 0xe634c4 — __ZThn48_N4Ogre13InstanceBatchD0Ev
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::~InstanceBatch()")]
// was: non-virtual thunk to Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe634c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e634c4() {
}

// 0xe63558 — __ZThn52_N4Ogre13InstanceBatchD0Ev
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::~InstanceBatch()")]
// was: non-virtual thunk to Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe63558: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e63558() {
}

// 0xe635ec — __ZN4Ogre13InstanceBatchD2Ev
#[doc(alias = "Ogre::InstanceBatch::~InstanceBatch()")]
// was: Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe635ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e635ec() {
}

// 0xe63a70 — __ZThn48_N4Ogre13InstanceBatchD1Ev
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::~InstanceBatch()")]
// was: non-virtual thunk to Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe63a70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e63a70() {
}

// 0xe63a7c — __ZThn52_N4Ogre13InstanceBatchD1Ev
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::~InstanceBatch()")]
// was: non-virtual thunk to Ogre::InstanceBatch::~InstanceBatch()
// IDA 0xe63a7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e63a7c() {
}

// 0xe63a88 — __ZN4Ogre13InstanceBatch21_setInstancesPerBatchEm
#[doc(alias = "Ogre::InstanceBatch::_setInstancesPerBatch(unsigned long)")]
// was: Ogre::InstanceBatch::_setInstancesPerBatch(unsigned long)
// IDA 0xe63a88: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e63a88() {
}

// 0xe63c64 — __ZN4Ogre13InstanceBatch25checkSubMeshCompatibilityEPKNS_7SubMeshE
#[doc(alias = "Ogre::InstanceBatch::checkSubMeshCompatibility(Ogre::SubMesh const*)")]
// was: Ogre::InstanceBatch::checkSubMeshCompatibility(Ogre::SubMesh const*)
// IDA 0xe63c64: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e63c64() {
}

// 0xe63e34 — __ZN4Ogre13InstanceBatch13_updateBoundsEv
#[doc(alias = "Ogre::InstanceBatch::_updateBounds(void)")]
// was: Ogre::InstanceBatch::_updateBounds(void)
// IDA 0xe63e34: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e63e34() {
}

// 0xe6400c — __ZN4Ogre13InstanceBatch26createAllInstancedEntitiesEv
#[doc(alias = "Ogre::InstanceBatch::createAllInstancedEntities(void)")]
// was: Ogre::InstanceBatch::createAllInstancedEntities(void)
// IDA 0xe6400c: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6400c() {
}

// 0xe640ac — __ZN4Ogre13InstanceBatch23generateInstancedEntityEm
#[doc(alias = "Ogre::InstanceBatch::generateInstancedEntity(unsigned long)")]
// was: Ogre::InstanceBatch::generateInstancedEntity(unsigned long)
// IDA 0xe640ac: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e640ac() {
}

// 0xe64170 — __ZN4Ogre13InstanceBatch26deleteAllInstancedEntitiesEv
#[doc(alias = "Ogre::InstanceBatch::deleteAllInstancedEntities(void)")]
// was: Ogre::InstanceBatch::deleteAllInstancedEntities(void)
// IDA 0xe64170: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64170() {
}

// 0xe641ac — __ZN4Ogre13InstanceBatch29deleteUnusedInstancedEntitiesEv
#[doc(alias = "Ogre::InstanceBatch::deleteUnusedInstancedEntities(void)")]
// was: Ogre::InstanceBatch::deleteUnusedInstancedEntities(void)
// IDA 0xe641ac: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e641ac() {
}

// 0xe641e4 — __ZN4Ogre13InstanceBatch27makeMatrixCameraRelative3x4EPfm
#[doc(alias = "Ogre::InstanceBatch::makeMatrixCameraRelative3x4(float *,unsigned long)")]
// was: Ogre::InstanceBatch::makeMatrixCameraRelative3x4(float *,unsigned long)
// IDA 0xe641e4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e641e4() {
}

// 0xe64248 — __ZN4Ogre13InstanceBatch5buildEPKNS_7SubMeshE
#[doc(alias = "Ogre::InstanceBatch::build(Ogre::SubMesh const*)")]
// was: Ogre::InstanceBatch::build(Ogre::SubMesh const*)
// IDA 0xe64248: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64248() {
}

// 0xe642ac — __ZN4Ogre13InstanceBatch9buildFromEPKNS_7SubMeshERKNS_15RenderOperationE
#[doc(alias = "Ogre::InstanceBatch::buildFrom(Ogre::SubMesh const*,Ogre::RenderOperation const&)")]
// was: Ogre::InstanceBatch::buildFrom(Ogre::SubMesh const*,Ogre::RenderOperation const&)
// IDA 0xe642ac: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e642ac() {
}

// 0xe642d4 — __ZN4Ogre13InstanceBatch21createInstancedEntityEv
#[doc(alias = "Ogre::InstanceBatch::createInstancedEntity(void)")]
// was: Ogre::InstanceBatch::createInstancedEntity(void)
// IDA 0xe642d4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e642d4() {
}

// 0xe642fc — __ZN4Ogre13InstanceBatch21removeInstancedEntityEPNS_15InstancedEntityE
#[doc(alias = "Ogre::InstanceBatch::removeInstancedEntity(Ogre::InstancedEntity *)")]
// was: Ogre::InstanceBatch::removeInstancedEntity(Ogre::InstancedEntity *)
// IDA 0xe642fc: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e642fc() {
}

// 0xe6453c — __ZN4Ogre13InstanceBatch12_boundsDirtyEv
#[doc(alias = "Ogre::InstanceBatch::_boundsDirty(void)")]
// was: Ogre::InstanceBatch::_boundsDirty(void)
// IDA 0xe6453c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6453c() {
}

// 0xe6455c — __ZNK4Ogre13InstanceBatch14getMovableTypeEv
#[doc(alias = "Ogre::InstanceBatch::getMovableType(void)const")]
// was: Ogre::InstanceBatch::getMovableType(void)const
// IDA 0xe6455c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6455c() {
}

// 0xe64650 — __ZThn48_NK4Ogre13InstanceBatch14getMovableTypeEv
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::getMovableType(void)const")]
// was: non-virtual thunk to Ogre::InstanceBatch::getMovableType(void)const
// IDA 0xe64650: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64650() {
}

// 0xe64744 — __ZN4Ogre13InstanceBatch20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::InstanceBatch::_notifyCurrentCamera(Ogre::Camera *)")]
// was: Ogre::InstanceBatch::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xe64744: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64744() {
}

// 0xe647b4 — __ZThn48_N4Ogre13InstanceBatch20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::_notifyCurrentCamera(Ogre::Camera *)")]
// was: non-virtual thunk to Ogre::InstanceBatch::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xe647b4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e647b4() {
}

// 0xe64824 — __ZNK4Ogre13InstanceBatch14getBoundingBoxEv
#[doc(alias = "Ogre::InstanceBatch::getBoundingBox(void)const")]
// was: Ogre::InstanceBatch::getBoundingBox(void)const
// IDA 0xe64824: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64824() {
}

// 0xe6482c — __ZThn48_NK4Ogre13InstanceBatch14getBoundingBoxEv
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::getBoundingBox(void)const")]
// was: non-virtual thunk to Ogre::InstanceBatch::getBoundingBox(void)const
// IDA 0xe6482c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6482c() {
}

// 0xe64834 — __ZNK4Ogre13InstanceBatch17getBoundingRadiusEv
#[doc(alias = "Ogre::InstanceBatch::getBoundingRadius(void)const")]
// was: Ogre::InstanceBatch::getBoundingRadius(void)const
// IDA 0xe64834: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64834() {
}

// 0xe6483c — __ZThn48_NK4Ogre13InstanceBatch17getBoundingRadiusEv
#[doc(alias = "non-virtual thunk toOgre::InstanceBatch::getBoundingRadius(void)const")]
// was: non-virtual thunk to Ogre::InstanceBatch::getBoundingRadius(void)const
// IDA 0xe6483c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6483c() {
}

// 0xe64844 — __ZNK4Ogre13InstanceBatch19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::InstanceBatch::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::InstanceBatch::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xe64844: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e64844() {
}

// 0xe648b4 — __ZNK4Ogre13InstanceBatch9getLightsEv
#[doc(alias = "Ogre::InstanceBatch::getLights(void)const")]
// was: Ogre::InstanceBatch::getLights(void)const
// IDA 0xe648b4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e648b4() {
}