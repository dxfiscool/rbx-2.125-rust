//! rendering — next 100 Ogre/G3D stubs (EA-sorted filter Ogre|G3D)
//! Filter: Ogre|G3D (13663 total / 13331 strict, 2241 prior stubbed strict, +100 this batch) — 0xcd89b4..0xcdf890 after 0xcd89a8 (remaining 10990 strict, 11307 substring)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcd89b4 — __ZN4Ogre3LogD2Ev
#[doc(alias = "Ogre::Log::~Log()")]
// was: Ogre::Log::~Log()
// IDA 0xcd89b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd89b4() {
}

// 0xcd8c30 — __ZN4Ogre3Log10logMessageERKSsNS_15LogMessageLevelEb
#[doc(alias = "Ogre::Log::logMessage(std::string const&,Ogre::LogMessageLevel,bool)")]
// was: Ogre::Log::logMessage(std::string const&,Ogre::LogMessageLevel,bool)
// IDA 0xcd8c30: 316 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd8c30() {
}

// 0xcd8f74 — __ZN4Ogre3Log11addListenerEPNS_11LogListenerE
#[doc(alias = "Ogre::Log::addListener(Ogre::LogListener *)")]
// was: Ogre::Log::addListener(Ogre::LogListener *)
// IDA 0xcd8f74: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd8f74() {
}

// 0xcd9064 — __ZN4Ogre3Log14removeListenerEPNS_11LogListenerE
#[doc(alias = "Ogre::Log::removeListener(Ogre::LogListener *)")]
// was: Ogre::Log::removeListener(Ogre::LogListener *)
// IDA 0xcd9064: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd9064() {
}

// 0xcd9190 — __ZN4Ogre3Log6streamENS_15LogMessageLevelEb
#[doc(alias = "Ogre::Log::stream(Ogre::LogMessageLevel,bool)")]
// was: Ogre::Log::stream(Ogre::LogMessageLevel,bool)
// IDA 0xcd9190: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd9190() {
}

// 0xcd91b0 — __ZNSt6vectorIPN4Ogre11LogListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LogListener **,std::vector<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LogListener * const&)")]
// was: std::vector<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LogListener **,std::vector<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LogListener * const&)
// IDA 0xcd91b0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_cd91b0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xcd92a8 — __ZNSt12_Vector_baseIPN4Ogre11LogListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcd92a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd92a8() {
}

// 0xcd92ac — __ZNSt12_Vector_baseIPN4Ogre11LogListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::LogListener *,Ogre::STLAllocator<Ogre::LogListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcd92ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd92ac() {
}

// 0xcd92ec — __ZN4Ogre10LogManager15getSingletonPtrEv
#[doc(alias = "Ogre::LogManager::getSingletonPtr(void)")]
// was: Ogre::LogManager::getSingletonPtr(void)
// IDA 0xcd92ec: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd92ec() {
}

// 0xcd92fc — __ZN4Ogre10LogManager12getSingletonEv
#[doc(alias = "Ogre::LogManager::getSingleton(void)")]
// was: Ogre::LogManager::getSingleton(void)
// IDA 0xcd92fc: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd92fc() {
}

// 0xcd930c — __ZN4Ogre10LogManagerC1Ev
#[doc(alias = "Ogre::LogManager::LogManager(void)")]
// was: Ogre::LogManager::LogManager(void)
// IDA 0xcd930c: 19 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd930c() {
}

// 0xcd9344 — __ZN4Ogre10LogManagerD1Ev
#[doc(alias = "Ogre::LogManager::~LogManager()")]
// was: Ogre::LogManager::~LogManager()
// IDA 0xcd9344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd9344() {
}

// 0xcd9350 — __ZN4Ogre10LogManagerD2Ev
#[doc(alias = "Ogre::LogManager::~LogManager()")]
// was: Ogre::LogManager::~LogManager()
// IDA 0xcd9350: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd9350() {
}

// 0xcd94ac — __ZN4Ogre10LogManager14logMessageSafeERKSsNS_15LogMessageLevelEb
#[doc(alias = "Ogre::LogManager::logMessageSafe(std::string const&,Ogre::LogMessageLevel,bool)")]
// was: Ogre::LogManager::logMessageSafe(std::string const&,Ogre::LogMessageLevel,bool)
// IDA 0xcd94ac: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd94ac() {
}

// 0xcd959c — __ZN4Ogre10LogManager9createLogERKSsbbb
#[doc(alias = "Ogre::LogManager::createLog(std::string const&,bool,bool,bool)")]
// was: Ogre::LogManager::createLog(std::string const&,bool,bool,bool)
// IDA 0xcd959c: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd959c() {
}

// 0xcd9768 — __ZN4Ogre10LogManager13getDefaultLogEv
#[doc(alias = "Ogre::LogManager::getDefaultLog(void)")]
// was: Ogre::LogManager::getDefaultLog(void)
// IDA 0xcd9768: 55 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd9768() {
}

// 0xcd980c — __ZN4Ogre10LogManager10logMessageERKSsNS_15LogMessageLevelEb
#[doc(alias = "Ogre::LogManager::logMessage(std::string const&,Ogre::LogMessageLevel,bool)")]
// was: Ogre::LogManager::logMessage(std::string const&,Ogre::LogMessageLevel,bool)
// IDA 0xcd980c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd980c() {
}

// 0xcd98f0 — __ZN4Ogre10LogManager6streamENS_15LogMessageLevelEb
#[doc(alias = "Ogre::LogManager::stream(Ogre::LogMessageLevel,bool)")]
// was: Ogre::LogManager::stream(Ogre::LogMessageLevel,bool)
// IDA 0xcd98f0: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd98f0() {
}

// 0xcd9bac — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Log *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Log *> const&)
// IDA 0xcd9bac: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd9bac() {
}

// 0xcd9c90 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Log *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Log *> const&)
// IDA 0xcd9c90: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd9c90() {
}

// 0xcd9de4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xcd9de4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cd9de4() {
}

// 0xcd9de8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xcd9de8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cd9de8() {
}

// 0xcd9df4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre3LogEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Log *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Log *>,std::_Select1st<std::pair<std::string const,Ogre::Log *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Log *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Log *>> *)
// IDA 0xcd9df4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cd9df4() {
}

// 0xcda010 — __ZN4Ogre12ManualObjectC1ERKSs
#[doc(alias = "Ogre::ManualObject::ManualObject(std::string const&)")]
// was: Ogre::ManualObject::ManualObject(std::string const&)
// IDA 0xcda010: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda010() {
}

// 0xcda130 — __ZN4Ogre12ManualObjectD0Ev
#[doc(alias = "Ogre::ManualObject::~ManualObject()")]
// was: Ogre::ManualObject::~ManualObject()
// IDA 0xcda130: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cda130() {
}

// 0xcda1c0 — __ZN4Ogre12ManualObjectD1Ev
#[doc(alias = "Ogre::ManualObject::~ManualObject()")]
// was: Ogre::ManualObject::~ManualObject()
// IDA 0xcda1c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cda1c0() {
}

// 0xcda1cc — __ZThn4_N4Ogre12ManualObjectD0Ev
#[doc(alias = "non-virtual thunk toOgre::ManualObject::~ManualObject()")]
// was: non-virtual thunk to Ogre::ManualObject::~ManualObject()
// IDA 0xcda1cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cda1cc() {
}

// 0xcda260 — __ZN4Ogre12ManualObjectD2Ev
#[doc(alias = "Ogre::ManualObject::~ManualObject()")]
// was: Ogre::ManualObject::~ManualObject()
// IDA 0xcda260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cda260() {
}

// 0xcda394 — __ZThn4_N4Ogre12ManualObjectD1Ev
#[doc(alias = "non-virtual thunk toOgre::ManualObject::~ManualObject()")]
// was: non-virtual thunk to Ogre::ManualObject::~ManualObject()
// IDA 0xcda394: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cda394() {
}

// 0xcda3a0 — __ZN4Ogre12ManualObject5clearEv
#[doc(alias = "Ogre::ManualObject::clear(void)")]
// was: Ogre::ManualObject::clear(void)
// IDA 0xcda3a0: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda3a0() {
}

// 0xcda4a8 — __ZN4Ogre12ManualObject14resetTempAreasEv
#[doc(alias = "Ogre::ManualObject::resetTempAreas(void)")]
// was: Ogre::ManualObject::resetTempAreas(void)
// IDA 0xcda4a8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda4a8() {
}

// 0xcda4d8 — __ZN4Ogre12ManualObject30resizeTempVertexBufferIfNeededEm
#[doc(alias = "Ogre::ManualObject::resizeTempVertexBufferIfNeeded(unsigned long)")]
// was: Ogre::ManualObject::resizeTempVertexBufferIfNeeded(unsigned long)
// IDA 0xcda4d8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda4d8() {
}

// 0xcda558 — __ZN4Ogre12ManualObject29resizeTempIndexBufferIfNeededEm
#[doc(alias = "Ogre::ManualObject::resizeTempIndexBufferIfNeeded(unsigned long)")]
// was: Ogre::ManualObject::resizeTempIndexBufferIfNeeded(unsigned long)
// IDA 0xcda558: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda558() {
}

// 0xcda5c4 — __ZN4Ogre12ManualObject19estimateVertexCountEm
#[doc(alias = "Ogre::ManualObject::estimateVertexCount(unsigned long)")]
// was: Ogre::ManualObject::estimateVertexCount(unsigned long)
// IDA 0xcda5c4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda5c4() {
}

// 0xcda5dc — __ZN4Ogre12ManualObject18estimateIndexCountEm
#[doc(alias = "Ogre::ManualObject::estimateIndexCount(unsigned long)")]
// was: Ogre::ManualObject::estimateIndexCount(unsigned long)
// IDA 0xcda5dc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda5dc() {
}

// 0xcda5f4 — __ZN4Ogre12ManualObject5beginERKSsNS_15RenderOperation13OperationTypeES2_
#[doc(alias = "Ogre::ManualObject::begin(std::string const&,Ogre::RenderOperation::OperationType,std::string const&)")]
// was: Ogre::ManualObject::begin(std::string const&,Ogre::RenderOperation::OperationType,std::string const&)
// IDA 0xcda5f4: 224 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda5f4() {
}

// 0xcda87c — __ZN4Ogre12ManualObject11beginUpdateEm
#[doc(alias = "Ogre::ManualObject::beginUpdate(unsigned long)")]
// was: Ogre::ManualObject::beginUpdate(unsigned long)
// IDA 0xcda87c: 295 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cda87c() {
}

// 0xcdabe4 — __ZN4Ogre12ManualObject8positionERKNS_7Vector3E
#[doc(alias = "Ogre::ManualObject::position(Ogre::Vector3 const&)")]
// was: Ogre::ManualObject::position(Ogre::Vector3 const&)
// IDA 0xcdabe4: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdabe4() {
}

// 0xcdabf8 — __ZN4Ogre12ManualObject8positionEfff
#[doc(alias = "Ogre::ManualObject::position(float,float,float)")]
// was: Ogre::ManualObject::position(float,float,float)
// IDA 0xcdabf8: 261 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdabf8() {
}

// 0xcdaf24 — __ZN4Ogre12ManualObject6normalERKNS_7Vector3E
#[doc(alias = "Ogre::ManualObject::normal(Ogre::Vector3 const&)")]
// was: Ogre::ManualObject::normal(Ogre::Vector3 const&)
// IDA 0xcdaf24: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdaf24() {
}

// 0xcdaf38 — __ZN4Ogre12ManualObject6normalEfff
#[doc(alias = "Ogre::ManualObject::normal(float,float,float)")]
// was: Ogre::ManualObject::normal(float,float,float)
// IDA 0xcdaf38: 189 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdaf38() {
}

// 0xcdb16c — __ZN4Ogre12ManualObject7tangentERKNS_7Vector3E
#[doc(alias = "Ogre::ManualObject::tangent(Ogre::Vector3 const&)")]
// was: Ogre::ManualObject::tangent(Ogre::Vector3 const&)
// IDA 0xcdb16c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdb16c() {
}

// 0xcdb180 — __ZN4Ogre12ManualObject7tangentEfff
#[doc(alias = "Ogre::ManualObject::tangent(float,float,float)")]
// was: Ogre::ManualObject::tangent(float,float,float)
// IDA 0xcdb180: 189 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdb180() {
}

// 0xcdb3b4 — __ZN4Ogre12ManualObject12textureCoordEf
#[doc(alias = "Ogre::ManualObject::textureCoord(float)")]
// was: Ogre::ManualObject::textureCoord(float)
// IDA 0xcdb3b4: 193 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdb3b4() {
}

// 0xcdb5fc — __ZN4Ogre12ManualObject12textureCoordEff
#[doc(alias = "Ogre::ManualObject::textureCoord(float,float)")]
// was: Ogre::ManualObject::textureCoord(float,float)
// IDA 0xcdb5fc: 197 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdb5fc() {
}

// 0xcdb850 — __ZN4Ogre12ManualObject12textureCoordEfff
#[doc(alias = "Ogre::ManualObject::textureCoord(float,float,float)")]
// was: Ogre::ManualObject::textureCoord(float,float,float)
// IDA 0xcdb850: 200 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdb850() {
}

// 0xcdbab0 — __ZN4Ogre12ManualObject12textureCoordEffff
#[doc(alias = "Ogre::ManualObject::textureCoord(float,float,float,float)")]
// was: Ogre::ManualObject::textureCoord(float,float,float,float)
// IDA 0xcdbab0: 202 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbab0() {
}

// 0xcdbd18 — __ZN4Ogre12ManualObject12textureCoordERKNS_7Vector2E
#[doc(alias = "Ogre::ManualObject::textureCoord(Ogre::Vector2 const&)")]
// was: Ogre::ManualObject::textureCoord(Ogre::Vector2 const&)
// IDA 0xcdbd18: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbd18() {
}

// 0xcdbd30 — __ZN4Ogre12ManualObject12textureCoordERKNS_7Vector3E
#[doc(alias = "Ogre::ManualObject::textureCoord(Ogre::Vector3 const&)")]
// was: Ogre::ManualObject::textureCoord(Ogre::Vector3 const&)
// IDA 0xcdbd30: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbd30() {
}

// 0xcdbd44 — __ZN4Ogre12ManualObject12textureCoordERKNS_7Vector4E
#[doc(alias = "Ogre::ManualObject::textureCoord(Ogre::Vector4 const&)")]
// was: Ogre::ManualObject::textureCoord(Ogre::Vector4 const&)
// IDA 0xcdbd44: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbd44() {
}

// 0xcdbd6c — __ZN4Ogre12ManualObject6colourERKNS_11ColourValueE
#[doc(alias = "Ogre::ManualObject::colour(Ogre::ColourValue const&)")]
// was: Ogre::ManualObject::colour(Ogre::ColourValue const&)
// IDA 0xcdbd6c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbd6c() {
}

// 0xcdbd94 — __ZN4Ogre12ManualObject6colourEffff
#[doc(alias = "Ogre::ManualObject::colour(float,float,float,float)")]
// was: Ogre::ManualObject::colour(float,float,float,float)
// IDA 0xcdbd94: 191 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbd94() {
}

// 0xcdbfd0 — __ZN4Ogre12ManualObject5indexEj
#[doc(alias = "Ogre::ManualObject::index(unsigned int)")]
// was: Ogre::ManualObject::index(unsigned int)
// IDA 0xcdbfd0: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdbfd0() {
}

// 0xcdc23c — __ZN4Ogre12ManualObject8triangleEjjj
#[doc(alias = "Ogre::ManualObject::triangle(unsigned int,unsigned int,unsigned int)")]
// was: Ogre::ManualObject::triangle(unsigned int,unsigned int,unsigned int)
// IDA 0xcdc23c: 291 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdc23c() {
}

// 0xcdc594 — __ZN4Ogre12ManualObject4quadEjjjj
#[doc(alias = "Ogre::ManualObject::quad(unsigned int,unsigned int,unsigned int,unsigned int)")]
// was: Ogre::ManualObject::quad(unsigned int,unsigned int,unsigned int,unsigned int)
// IDA 0xcdc594: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdc594() {
}

// 0xcdc5bc — __ZNK4Ogre12ManualObject21getCurrentVertexCountEv
#[doc(alias = "Ogre::ManualObject::getCurrentVertexCount(void)const")]
// was: Ogre::ManualObject::getCurrentVertexCount(void)const
// IDA 0xcdc5bc: 10 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdc5bc() {
}

// 0xcdc5d4 — __ZNK4Ogre12ManualObject20getCurrentIndexCountEv
#[doc(alias = "Ogre::ManualObject::getCurrentIndexCount(void)const")]
// was: Ogre::ManualObject::getCurrentIndexCount(void)const
// IDA 0xcdc5d4: 11 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdc5d4() {
}

// 0xcdc5ec — __ZN4Ogre12ManualObject22copyTempVertexToBufferEv
#[doc(alias = "Ogre::ManualObject::copyTempVertexToBuffer(void)")]
// was: Ogre::ManualObject::copyTempVertexToBuffer(void)
// IDA 0xcdc5ec: 137 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdc5ec() {
}

// 0xcdc75c — __ZN4Ogre12ManualObject3endEv
#[doc(alias = "Ogre::ManualObject::end(void)")]
// was: Ogre::ManualObject::end(void)
// IDA 0xcdc75c: 701 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdc75c() {
}

// 0xcdce6c — __ZN4Ogre12ManualObject15setMaterialNameEmRKSsS2_
#[doc(alias = "Ogre::ManualObject::setMaterialName(unsigned long,std::string const&,std::string const&)")]
// was: Ogre::ManualObject::setMaterialName(unsigned long,std::string const&,std::string const&)
// IDA 0xcdce6c: 166 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdce6c() {
}

// 0xcdd060 — __ZN4Ogre12ManualObject19ManualObjectSection15setMaterialNameERKSsS3_
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::setMaterialName(std::string const&,std::string const&)")]
// was: Ogre::ManualObject::ManualObjectSection::setMaterialName(std::string const&,std::string const&)
// IDA 0xcdd060: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd060() {
}

// 0xcdd0f4 — __ZN4Ogre12ManualObject13convertToMeshERKSsS2_
#[doc(alias = "Ogre::ManualObject::convertToMesh(std::string const&,std::string const&)")]
// was: Ogre::ManualObject::convertToMesh(std::string const&,std::string const&)
// IDA 0xcdd0f4: 434 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd0f4() {
}

// 0xcdd5bc — __ZNK4Ogre12ManualObject14getNumSectionsEv
#[doc(alias = "Ogre::ManualObject::getNumSections(void)const")]
// was: Ogre::ManualObject::getNumSections(void)const
// IDA 0xcdd5bc: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd5bc() {
}

// 0xcdd5c8 — __ZNK4Ogre12ManualObject14getMovableTypeEv
#[doc(alias = "Ogre::ManualObject::getMovableType(void)const")]
// was: Ogre::ManualObject::getMovableType(void)const
// IDA 0xcdd5c8: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd5c8() {
}

// 0xcdd5d4 — __ZNK4Ogre12ManualObject14getBoundingBoxEv
#[doc(alias = "Ogre::ManualObject::getBoundingBox(void)const")]
// was: Ogre::ManualObject::getBoundingBox(void)const
// IDA 0xcdd5d4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd5d4() {
}

// 0xcdd5dc — __ZNK4Ogre12ManualObject17getBoundingRadiusEv
#[doc(alias = "Ogre::ManualObject::getBoundingRadius(void)const")]
// was: Ogre::ManualObject::getBoundingRadius(void)const
// IDA 0xcdd5dc: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd5dc() {
}

// 0xcdd5e4 — __ZN4Ogre12ManualObject18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::ManualObject::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::ManualObject::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xcdd5e4: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd5e4() {
}

// 0xcdd6b4 — __ZN4Ogre12ManualObject16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::ManualObject::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::ManualObject::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xcdd6b4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd6b4() {
}

// 0xcdd6f8 — __ZN4Ogre12ManualObject11getEdgeListEv
#[doc(alias = "Ogre::ManualObject::getEdgeList(void)")]
// was: Ogre::ManualObject::getEdgeList(void)
// IDA 0xcdd6f8: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd6f8() {
}

// 0xcdd840 — __ZN4Ogre12ManualObject11hasEdgeListEv
#[doc(alias = "Ogre::ManualObject::hasEdgeList(void)")]
// was: Ogre::ManualObject::hasEdgeList(void)
// IDA 0xcdd840: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd840() {
}

// 0xcdd854 — __ZN4Ogre12ManualObject33getShadowVolumeRenderableIteratorENS_15ShadowTechniqueEPKNS_5LightEPNS_28HardwareIndexBufferSharedPtrEbfm
#[doc(alias = "Ogre::ManualObject::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)")]
// was: Ogre::ManualObject::getShadowVolumeRenderableIterator(Ogre::ShadowTechnique,Ogre::Light const*,Ogre::HardwareIndexBufferSharedPtr *,bool,float,unsigned long)
// IDA 0xcdd854: 519 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdd854() {
}

// 0xcddd78 — __ZN4Ogre12ManualObject19ManualObjectSectionC2EPS0_RKSsNS_15RenderOperation13OperationTypeES4_
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::ManualObjectSection(Ogre::ManualObject*,std::string const&,Ogre::RenderOperation::OperationType,std::string const&)")]
// was: Ogre::ManualObject::ManualObjectSection::ManualObjectSection(Ogre::ManualObject*,std::string const&,Ogre::RenderOperation::OperationType,std::string const&)
// IDA 0xcddd78: 309 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cddd78() {
}

// 0xcde09c — __ZN4Ogre12ManualObject19ManualObjectSectionD0Ev
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::~ManualObjectSection()")]
// was: Ogre::ManualObject::ManualObjectSection::~ManualObjectSection()
// IDA 0xcde09c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cde09c() {
}

// 0xcde12c — __ZN4Ogre12ManualObject19ManualObjectSectionD1Ev
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::~ManualObjectSection()")]
// was: Ogre::ManualObject::ManualObjectSection::~ManualObjectSection()
// IDA 0xcde12c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cde12c() {
}

// 0xcde138 — __ZN4Ogre12ManualObject19ManualObjectSectionD2Ev
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::~ManualObjectSection()")]
// was: Ogre::ManualObject::ManualObjectSection::~ManualObjectSection()
// IDA 0xcde138: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cde138() {
}

// 0xcde318 — __ZNK4Ogre12ManualObject19ManualObjectSection11getMaterialEv
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::getMaterial(void)const")]
// was: Ogre::ManualObject::ManualObjectSection::getMaterial(void)const
// IDA 0xcde318: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cde318() {
}

// 0xcde4fc — __ZN4Ogre12ManualObject19ManualObjectSection18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::ManualObject::ManualObjectSection::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xcde4fc: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cde4fc() {
}

// 0xcde518 — __ZNK4Ogre12ManualObject19ManualObjectSection18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::ManualObject::ManualObjectSection::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xcde518: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cde518() {
}

// 0xcde560 — __ZNK4Ogre12ManualObject19ManualObjectSection19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::ManualObject::ManualObjectSection::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xcde560: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cde560() {
}

// 0xcde57c — __ZNK4Ogre12ManualObject19ManualObjectSection9getLightsEv
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::getLights(void)const")]
// was: Ogre::ManualObject::ManualObjectSection::getLights(void)const
// IDA 0xcde57c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cde57c() {
}

// 0xcde58c — __ZN4Ogre12ManualObject35ManualObjectSectionShadowRenderableC2EPS0_PNS_28HardwareIndexBufferSharedPtrEPKNS_10VertexDataEbb
#[doc(alias = "Ogre::ManualObject::ManualObjectSectionShadowRenderable::ManualObjectSectionShadowRenderable(Ogre::ManualObject*,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,bool)")]
// was: Ogre::ManualObject::ManualObjectSectionShadowRenderable::ManualObjectSectionShadowRenderable(Ogre::ManualObject*,Ogre::HardwareIndexBufferSharedPtr *,Ogre::VertexData const*,bool,bool)
// IDA 0xcde58c: 446 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cde58c() {
}

// 0xcde9c8 — __ZN4Ogre12ManualObject35ManualObjectSectionShadowRenderableD0Ev
#[doc(alias = "Ogre::ManualObject::ManualObjectSectionShadowRenderable::~ManualObjectSectionShadowRenderable()")]
// was: Ogre::ManualObject::ManualObjectSectionShadowRenderable::~ManualObjectSectionShadowRenderable()
// IDA 0xcde9c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cde9c8() {
}

// 0xcdea58 — __ZN4Ogre12ManualObject35ManualObjectSectionShadowRenderableD1Ev
#[doc(alias = "Ogre::ManualObject::ManualObjectSectionShadowRenderable::~ManualObjectSectionShadowRenderable()")]
// was: Ogre::ManualObject::ManualObjectSectionShadowRenderable::~ManualObjectSectionShadowRenderable()
// IDA 0xcdea58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdea58() {
}

// 0xcdea64 — __ZN4Ogre12ManualObject35ManualObjectSectionShadowRenderableD2Ev
#[doc(alias = "Ogre::ManualObject::ManualObjectSectionShadowRenderable::~ManualObjectSectionShadowRenderable()")]
// was: Ogre::ManualObject::ManualObjectSectionShadowRenderable::~ManualObjectSectionShadowRenderable()
// IDA 0xcdea64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdea64() {
}

// 0xcdec04 — __ZNK4Ogre12ManualObject35ManualObjectSectionShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::ManualObject::ManualObjectSectionShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::ManualObject::ManualObjectSectionShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xcdec04: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdec04() {
}

// 0xcdec4c — __ZN4Ogre12ManualObject35ManualObjectSectionShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
#[doc(alias = "Ogre::ManualObject::ManualObjectSectionShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
// was: Ogre::ManualObject::ManualObjectSectionShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)
// IDA 0xcdec4c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdec4c() {
}

// 0xcdec6c — __ZNK4Ogre19ManualObjectFactory7getTypeEv
#[doc(alias = "Ogre::ManualObjectFactory::getType(void)const")]
// was: Ogre::ManualObjectFactory::getType(void)const
// IDA 0xcdec6c: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdec6c() {
}

// 0xcdec78 — __ZN4Ogre19ManualObjectFactory18createInstanceImplERKSsPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::ManualObjectFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::ManualObjectFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xcdec78: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdec78() {
}

// 0xcdee70 — __ZN4Ogre19ManualObjectFactory15destroyInstanceEPNS_13MovableObjectE
#[doc(alias = "Ogre::ManualObjectFactory::destroyInstance(Ogre::MovableObject *)")]
// was: Ogre::ManualObjectFactory::destroyInstance(Ogre::MovableObject *)
// IDA 0xcdee70: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdee70() {
}

// 0xcdee84 — __ZN4Ogre12ManualObject10setDynamicEb
#[doc(alias = "Ogre::ManualObject::setDynamic(bool)")]
// was: Ogre::ManualObject::setDynamic(bool)
// IDA 0xcdee84: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdee84() {
}

// 0xcdee8c — __ZNK4Ogre12ManualObject10getDynamicEv
#[doc(alias = "Ogre::ManualObject::getDynamic(void)const")]
// was: Ogre::ManualObject::getDynamic(void)const
// IDA 0xcdee8c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdee8c() {
}

// 0xcdee94 — __ZNSt6vectorIPN4Ogre12ManualObject19ManualObjectSectionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject::ManualObjectSection **,std::vector<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ManualObject::ManualObjectSection * const&)")]
// was: std::vector<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ManualObject::ManualObjectSection **,std::vector<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ManualObject::ManualObjectSection * const&)
// IDA 0xcdee94: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_cdee94() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xcdef8c — __ZNSt12_Vector_baseIPN4Ogre12ManualObject19ManualObjectSectionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcdef8c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cdef8c() {
}

// 0xcdef90 — __ZNSt12_Vector_baseIPN4Ogre12ManualObject19ManualObjectSectionENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ManualObject::ManualObjectSection *,Ogre::STLAllocator<Ogre::ManualObject::ManualObjectSection *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcdef90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdef90() {
}

// 0xcdf008 — __ZN4Ogre8MaterialC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Material::Material(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Material::Material(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xcdf008: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdf008() {
}

// 0xcdf024 — __ZN4Ogre8MaterialC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Material::Material(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Material::Material(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xcdf024: 455 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdf024() {
}

// 0xcdf508 — __ZN4Ogre8Material13applyDefaultsEv
#[doc(alias = "Ogre::Material::applyDefaults(void)")]
// was: Ogre::Material::applyDefaults(void)
// IDA 0xcdf508: 282 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdf508() {
}

// 0xcdf7f4 — __ZN4Ogre8MaterialD0Ev
#[doc(alias = "Ogre::Material::~Material()")]
// was: Ogre::Material::~Material()
// IDA 0xcdf7f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdf7f4() {
}

// 0xcdf884 — __ZN4Ogre8MaterialD1Ev
#[doc(alias = "Ogre::Material::~Material()")]
// was: Ogre::Material::~Material()
// IDA 0xcdf884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdf884() {
}

// 0xcdf890 — __ZN4Ogre8MaterialD2Ev
#[doc(alias = "Ogre::Material::~Material()")]
// was: Ogre::Material::~Material()
// IDA 0xcdf890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdf890() {
}
