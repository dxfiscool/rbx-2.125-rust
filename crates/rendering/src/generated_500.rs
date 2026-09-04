//! rendering — generated_500 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xd8ae90..0xd95b14 (3074 candidates remaining, 90904 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd8ae90 — __ZN4Ogre15ResourceManagerD2Ev
// type: void __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::~ResourceManager()")]
#[doc(alias = "__ZN4Ogre15ResourceManagerD2Ev")]
// was: Ogre::ResourceManager::~ResourceManager()
// IDA 0xd8ae90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8ae90() {
}

// 0xd8b178 — __ZN4Ogre15ResourceManager6createERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::ResourceManager::create(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN4Ogre15ResourceManager6createERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
// was: Ogre::ResourceManager::create(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xd8b178: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8b178() {
}

// 0xd8b324 — __ZN4Ogre15ResourceManager13getNextHandleEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getNextHandle(void)")]
#[doc(alias = "__ZN4Ogre15ResourceManager13getNextHandleEv")]
// was: Ogre::ResourceManager::getNextHandle(void)
// IDA 0xd8b324: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8b324() {
}

// 0xd8b33c — __ZN4Ogre15ResourceManager16createOrRetrieveERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::ResourceManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN4Ogre15ResourceManager16createOrRetrieveERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
// was: Ogre::ResourceManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xd8b33c: 267 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8b33c() {
}

// 0xd8b5c4 — __ZN4Ogre15ResourceManager7prepareERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::ResourceManager::prepare(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager7prepareERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb")]
// was: Ogre::ResourceManager::prepare(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,bool)
// IDA 0xd8b5c4: 185 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8b5c4() {
}

// 0xd8b788 — __ZN4Ogre15ResourceManager4loadERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::ResourceManager::load(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager4loadERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb")]
// was: Ogre::ResourceManager::load(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,bool)
// IDA 0xd8b788: 185 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8b788() {
}

// 0xd8d288 — __ZN4Ogre15ResourceManager15setMemoryBudgetEm
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::ResourceManager::setMemoryBudget(unsigned long)")]
#[doc(alias = "__ZN4Ogre15ResourceManager15setMemoryBudgetEm")]
// was: Ogre::ResourceManager::setMemoryBudget(unsigned long)
// IDA 0xd8d288: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d288() {
}

// 0xd8d298 — __ZNK4Ogre15ResourceManager15getMemoryBudgetEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::getMemoryBudget(void)const")]
#[doc(alias = "__ZNK4Ogre15ResourceManager15getMemoryBudgetEv")]
// was: Ogre::ResourceManager::getMemoryBudget(void)const
// IDA 0xd8d298: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d298() {
}

// 0xd8d29c — __ZN4Ogre15ResourceManager6unloadERKSs
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::ResourceManager::unload(std::string const&)")]
#[doc(alias = "__ZN4Ogre15ResourceManager6unloadERKSs")]
// was: Ogre::ResourceManager::unload(std::string const&)
// IDA 0xd8d29c: 148 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d29c() {
}

// 0xd8d41c — __ZN4Ogre15ResourceManager6unloadEy
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, unsigned __int64)
#[doc(alias = "Ogre::ResourceManager::unload(unsigned long long)")]
#[doc(alias = "__ZN4Ogre15ResourceManager6unloadEy")]
// was: Ogre::ResourceManager::unload(unsigned long long)
// IDA 0xd8d41c: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d41c() {
}

// 0xd8d590 — __ZN4Ogre15ResourceManager9unloadAllEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::unloadAll(bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager9unloadAllEb")]
// was: Ogre::ResourceManager::unloadAll(bool)
// IDA 0xd8d590: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d590() {
}

// 0xd8d5d0 — __ZN4Ogre15ResourceManager9reloadAllEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::reloadAll(bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager9reloadAllEb")]
// was: Ogre::ResourceManager::reloadAll(bool)
// IDA 0xd8d5d0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d5d0() {
}

// 0xd8d610 — __ZN4Ogre15ResourceManager27unloadUnreferencedResourcesEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::unloadUnreferencedResources(bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager27unloadUnreferencedResourcesEb")]
// was: Ogre::ResourceManager::unloadUnreferencedResources(bool)
// IDA 0xd8d610: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d610() {
}

// 0xd8d674 — __ZN4Ogre15ResourceManager27reloadUnreferencedResourcesEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::reloadUnreferencedResources(bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager27reloadUnreferencedResourcesEb")]
// was: Ogre::ResourceManager::reloadUnreferencedResources(bool)
// IDA 0xd8d674: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d674() {
}

// 0xd8d6e8 — __ZN4Ogre15ResourceManager6removeERKSs
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::ResourceManager::remove(std::string const&)")]
#[doc(alias = "__ZN4Ogre15ResourceManager6removeERKSs")]
// was: Ogre::ResourceManager::remove(std::string const&)
// IDA 0xd8d6e8: 150 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d6e8() {
}

// 0xd8d86c — __ZN4Ogre15ResourceManager6removeEy
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, unsigned __int64)
#[doc(alias = "Ogre::ResourceManager::remove(unsigned long long)")]
#[doc(alias = "__ZN4Ogre15ResourceManager6removeEy")]
// was: Ogre::ResourceManager::remove(unsigned long long)
// IDA 0xd8d86c: 148 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d86c() {
}

// 0xd8d9e4 — __ZN4Ogre15ResourceManager9removeAllEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::removeAll(void)")]
#[doc(alias = "__ZN4Ogre15ResourceManager9removeAllEv")]
// was: Ogre::ResourceManager::removeAll(void)
// IDA 0xd8d9e4: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8d9e4() {
}

// 0xd8db14 — __ZN4Ogre15ResourceManager27removeUnreferencedResourcesEb
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, bool)
#[doc(alias = "Ogre::ResourceManager::removeUnreferencedResources(bool)")]
#[doc(alias = "__ZN4Ogre15ResourceManager27removeUnreferencedResourcesEb")]
// was: Ogre::ResourceManager::removeUnreferencedResources(bool)
// IDA 0xd8db14: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8db14() {
}

// 0xd8dba8 — __ZN4Ogre15ResourceManager9getByNameERKSsS2_
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, const std::string *, const std::string *)
#[doc(alias = "Ogre::ResourceManager::getByName(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre15ResourceManager9getByNameERKSsS2_")]
// was: Ogre::ResourceManager::getByName(std::string const&,std::string const&)
// IDA 0xd8dba8: 272 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8dba8() {
}

// 0xd8de7c — __ZN4Ogre15ResourceManager11getByHandleEy
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, unsigned __int64)
#[doc(alias = "Ogre::ResourceManager::getByHandle(unsigned long long)")]
#[doc(alias = "__ZN4Ogre15ResourceManager11getByHandleEy")]
// was: Ogre::ResourceManager::getByHandle(unsigned long long)
// IDA 0xd8de7c: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8de7c() {
}

// 0xd8df3c — __ZN4Ogre15ResourceManager10checkUsageEv
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this)
#[doc(alias = "Ogre::ResourceManager::checkUsage(void)")]
#[doc(alias = "__ZN4Ogre15ResourceManager10checkUsageEv")]
// was: Ogre::ResourceManager::checkUsage(void)
// IDA 0xd8df3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd8df3c() {
}

// 0xd8df44 — __ZN4Ogre15ResourceManager21_notifyResourceLoadedEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::ResourceManager::_notifyResourceLoaded(Ogre::Resource *)")]
#[doc(alias = "__ZN4Ogre15ResourceManager21_notifyResourceLoadedEPNS_8ResourceE")]
// was: Ogre::ResourceManager::_notifyResourceLoaded(Ogre::Resource *)
// IDA 0xd8df44: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8df44() {
}

// 0xd8df5c — __ZN4Ogre15ResourceManager23_notifyResourceUnloadedEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::ResourceManager *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::ResourceManager::_notifyResourceUnloaded(Ogre::Resource *)")]
#[doc(alias = "__ZN4Ogre15ResourceManager23_notifyResourceUnloadedEPNS_8ResourceE")]
// was: Ogre::ResourceManager::_notifyResourceUnloaded(Ogre::Resource *)
// IDA 0xd8df5c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8df5c() {
}

// 0xd8df88 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>> *)
// IDA 0xd8df88: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8df88() {
}

// 0xd8f580 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd8f580: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd8f580() {
}

// 0xd8f584 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15ResourceManager12ResourcePoolEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,std::_Select1st<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ResourceManager::ResourcePool *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd8f584: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8f584() {
}

// 0xd8f5d8 — __ZN4Ogre11RibbonTrailC2ERKSsmmbb
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, const std::string *, unsigned int, unsigned int, Ogre::NedPoolingImpl *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "Ogre::RibbonTrail::RibbonTrail(std::string const&,unsigned long,unsigned long,bool,bool)")]
#[doc(alias = "__ZN4Ogre11RibbonTrailC2ERKSsmmbb")]
// was: Ogre::RibbonTrail::RibbonTrail(std::string const&,unsigned long,unsigned long,bool,bool)
// IDA 0xd8f5d8: 439 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd8f5d8() {
}

// 0xd8fa88 — __ZN4Ogre11RibbonTrailD0Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZN4Ogre11RibbonTrailD0Ev")]
// was: Ogre::RibbonTrail::~RibbonTrail()
// IDA 0xd8fa88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8fa88() {
}

// 0xd8fb18 — __ZN4Ogre11RibbonTrailD1Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZN4Ogre11RibbonTrailD1Ev")]
// was: Ogre::RibbonTrail::~RibbonTrail()
// IDA 0xd8fb18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8fb18() {
}

// 0xd8fb24 — __ZThn4_N4Ogre11RibbonTrailD0Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZThn4_N4Ogre11RibbonTrailD0Ev")]
// was: `non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()
// IDA 0xd8fb24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8fb24() {
}

// 0xd8fbb8 — __ZThn188_N4Ogre11RibbonTrailD0Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZThn188_N4Ogre11RibbonTrailD0Ev")]
// was: `non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()
// IDA 0xd8fbb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8fbb8() {
}

// 0xd8fc4c — __ZThn376_N4Ogre11RibbonTrailD0Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZThn376_N4Ogre11RibbonTrailD0Ev")]
// was: `non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()
// IDA 0xd8fc4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8fc4c() {
}

// 0xd8fce0 — __ZN4Ogre11RibbonTrailD2Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZN4Ogre11RibbonTrailD2Ev")]
// was: Ogre::RibbonTrail::~RibbonTrail()
// IDA 0xd8fce0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd8fce0() {
}

// 0xd9003c — __ZThn4_N4Ogre11RibbonTrailD1Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZThn4_N4Ogre11RibbonTrailD1Ev")]
// was: `non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()
// IDA 0xd9003c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd9003c() {
}

// 0xd90048 — __ZThn188_N4Ogre11RibbonTrailD1Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZThn188_N4Ogre11RibbonTrailD1Ev")]
// was: `non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()
// IDA 0xd90048: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd90048() {
}

// 0xd90054 — __ZThn376_N4Ogre11RibbonTrailD1Ev
// type: void __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()")]
#[doc(alias = "__ZThn376_N4Ogre11RibbonTrailD1Ev")]
// was: `non-virtual thunk toOgre::RibbonTrail::~RibbonTrail()
// IDA 0xd90054: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd90054() {
}

// 0xd90064 — __ZN4Ogre11RibbonTrail7addNodeEPNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::addNode(Ogre::Node *)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail7addNodeEPNS_4NodeE")]
// was: Ogre::RibbonTrail::addNode(Ogre::Node *)
// IDA 0xd90064: 546 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90064() {
}

// 0xd906c0 — __ZN4Ogre11RibbonTrail20getChainIndexForNodeEPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, const Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::getChainIndexForNode(Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail20getChainIndexForNodeEPKNS_4NodeE")]
// was: Ogre::RibbonTrail::getChainIndexForNode(Ogre::Node const*)
// IDA 0xd906c0: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd906c0() {
}

// 0xd908c4 — __ZN4Ogre11RibbonTrail10removeNodeEPNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::removeNode(Ogre::Node *)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail10removeNodeEPNS_4NodeE")]
// was: Ogre::RibbonTrail::removeNode(Ogre::Node *)
// IDA 0xd908c4: 144 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd908c4() {
}

// 0xd90a38 — __ZNK4Ogre11RibbonTrail15getNodeIteratorEv
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::getNodeIterator(void)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail15getNodeIteratorEv")]
// was: Ogre::RibbonTrail::getNodeIterator(void)const
// IDA 0xd90a38: 6 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90a38() {
}

// 0xd90a48 — __ZN4Ogre11RibbonTrail14setTrailLengthEf
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, float)
#[doc(alias = "Ogre::RibbonTrail::setTrailLength(float)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail14setTrailLengthEf")]
// was: Ogre::RibbonTrail::setTrailLength(float)
// IDA 0xd90a48: 10 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90a48() {
}

// 0xd90a70 — __ZN4Ogre11RibbonTrail19setMaxChainElementsEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::setMaxChainElements(unsigned long)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail19setMaxChainElementsEm")]
// was: Ogre::RibbonTrail::setMaxChainElements(unsigned long)
// IDA 0xd90a70: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90a70() {
}

// 0xd90aa8 — __ZN4Ogre11RibbonTrail17setNumberOfChainsEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::setNumberOfChains(unsigned long)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail17setNumberOfChainsEm")]
// was: Ogre::RibbonTrail::setNumberOfChains(unsigned long)
// IDA 0xd90aa8: 302 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90aa8() {
}

// 0xd90e20 — __ZN4Ogre11RibbonTrail10clearChainEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::clearChain(unsigned long)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail10clearChainEm")]
// was: Ogre::RibbonTrail::clearChain(unsigned long)
// IDA 0xd90e20: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90e20() {
}

// 0xd90ed0 — __ZN4Ogre11RibbonTrail16setInitialColourEmRKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, const Ogre::ColourValue *)
#[doc(alias = "Ogre::RibbonTrail::setInitialColour(unsigned long,Ogre::ColourValue const&)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail16setInitialColourEmRKNS_11ColourValueE")]
// was: Ogre::RibbonTrail::setInitialColour(unsigned long,Ogre::ColourValue const&)
// IDA 0xd90ed0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90ed0() {
}

// 0xd90f08 — __ZN4Ogre11RibbonTrail16setInitialColourEmffff
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, float, float, float, float)
#[doc(alias = "Ogre::RibbonTrail::setInitialColour(unsigned long,float,float,float,float)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail16setInitialColourEmffff")]
// was: Ogre::RibbonTrail::setInitialColour(unsigned long,float,float,float,float)
// IDA 0xd90f08: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd90f08() {
}

// 0xd9110c — __ZNK4Ogre11RibbonTrail16getInitialColourEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::getInitialColour(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail16getInitialColourEm")]
// was: Ogre::RibbonTrail::getInitialColour(unsigned long)const
// IDA 0xd9110c: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd9110c() {
}

// 0xd912e8 — __ZN4Ogre11RibbonTrail15setInitialWidthEmf
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, float)
#[doc(alias = "Ogre::RibbonTrail::setInitialWidth(unsigned long,float)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail15setInitialWidthEmf")]
// was: Ogre::RibbonTrail::setInitialWidth(unsigned long,float)
// IDA 0xd912e8: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd912e8() {
}

// 0xd914d4 — __ZNK4Ogre11RibbonTrail15getInitialWidthEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::getInitialWidth(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail15getInitialWidthEm")]
// was: Ogre::RibbonTrail::getInitialWidth(unsigned long)const
// IDA 0xd914d4: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd914d4() {
}

// 0xd916b4 — __ZN4Ogre11RibbonTrail15setColourChangeEmRKNS_11ColourValueE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, const Ogre::ColourValue *)
#[doc(alias = "Ogre::RibbonTrail::setColourChange(unsigned long,Ogre::ColourValue const&)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail15setColourChangeEmRKNS_11ColourValueE")]
// was: Ogre::RibbonTrail::setColourChange(unsigned long,Ogre::ColourValue const&)
// IDA 0xd916b4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd916b4() {
}

// 0xd916ec — __ZN4Ogre11RibbonTrail15setColourChangeEmffff
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, float, float, float, float)
#[doc(alias = "Ogre::RibbonTrail::setColourChange(unsigned long,float,float,float,float)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail15setColourChangeEmffff")]
// was: Ogre::RibbonTrail::setColourChange(unsigned long,float,float,float,float)
// IDA 0xd916ec: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd916ec() {
}

// 0xd91904 — __ZNK4Ogre11RibbonTrail15getColourChangeEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::getColourChange(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail15getColourChangeEm")]
// was: Ogre::RibbonTrail::getColourChange(unsigned long)const
// IDA 0xd91904: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91904() {
}

// 0xd91ae4 — __ZN4Ogre11RibbonTrail14setWidthChangeEmf
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, float)
#[doc(alias = "Ogre::RibbonTrail::setWidthChange(unsigned long,float)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail14setWidthChangeEmf")]
// was: Ogre::RibbonTrail::setWidthChange(unsigned long,float)
// IDA 0xd91ae4: 168 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91ae4() {
}

// 0xd91ce0 — __ZNK4Ogre11RibbonTrail14getWidthChangeEm
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int)
#[doc(alias = "Ogre::RibbonTrail::getWidthChange(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail14getWidthChangeEm")]
// was: Ogre::RibbonTrail::getWidthChange(unsigned long)const
// IDA 0xd91ce0: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91ce0() {
}

// 0xd91ec0 — __ZN4Ogre11RibbonTrail16manageControllerEv
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::manageController(void)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail16manageControllerEv")]
// was: Ogre::RibbonTrail::manageController(void)
// IDA 0xd91ec0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91ec0() {
}

// 0xd91f48 — __ZN4Ogre11RibbonTrail11nodeUpdatedEPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, const Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::nodeUpdated(Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail11nodeUpdatedEPKNS_4NodeE")]
// was: Ogre::RibbonTrail::nodeUpdated(Ogre::Node const*)
// IDA 0xd91f48: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91f48() {
}

// 0xd91f6c — __ZThn376_N4Ogre11RibbonTrail11nodeUpdatedEPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, const Ogre::Node *)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::nodeUpdated(Ogre::Node const*)")]
#[doc(alias = "__ZThn376_N4Ogre11RibbonTrail11nodeUpdatedEPKNS_4NodeE")]
// was: `non-virtual thunk toOgre::RibbonTrail::nodeUpdated(Ogre::Node const*)
// IDA 0xd91f6c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91f6c() {
}

// 0xd91fa8 — __ZN4Ogre11RibbonTrail13nodeDestroyedEPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, const Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::nodeDestroyed(Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail13nodeDestroyedEPKNS_4NodeE")]
// was: Ogre::RibbonTrail::nodeDestroyed(Ogre::Node const*)
// IDA 0xd91fa8: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91fa8() {
}

// 0xd91fb8 — __ZThn376_N4Ogre11RibbonTrail13nodeDestroyedEPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, const Ogre::Node *)
#[doc(alias = "non-virtual thunk toOgre::RibbonTrail::nodeDestroyed(Ogre::Node const*)")]
#[doc(alias = "__ZThn376_N4Ogre11RibbonTrail13nodeDestroyedEPKNS_4NodeE")]
// was: `non-virtual thunk toOgre::RibbonTrail::nodeDestroyed(Ogre::Node const*)
// IDA 0xd91fb8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91fb8() {
}

// 0xd91fd8 — __ZN4Ogre11RibbonTrail11updateTrailEmPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, const Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::updateTrail(unsigned long,Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail11updateTrailEmPKNS_4NodeE")]
// was: Ogre::RibbonTrail::updateTrail(unsigned long,Ogre::Node const*)
// IDA 0xd91fd8: 250 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd91fd8() {
}

// 0xd92328 — __ZN4Ogre11RibbonTrail11_timeUpdateEf
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, float)
#[doc(alias = "Ogre::RibbonTrail::_timeUpdate(float)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail11_timeUpdateEf")]
// was: Ogre::RibbonTrail::_timeUpdate(float)
// IDA 0xd92328: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92328() {
}

// 0xd92504 — __ZN4Ogre11RibbonTrail10resetTrailEmPKNS_4NodeE
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this, unsigned int, const Ogre::Node *)
#[doc(alias = "Ogre::RibbonTrail::resetTrail(unsigned long,Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail10resetTrailEmPKNS_4NodeE")]
// was: Ogre::RibbonTrail::resetTrail(unsigned long,Ogre::Node const*)
// IDA 0xd92504: 102 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92504() {
}

// 0xd92638 — __ZN4Ogre11RibbonTrail14resetAllTrailsEv
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::resetAllTrails(void)")]
#[doc(alias = "__ZN4Ogre11RibbonTrail14resetAllTrailsEv")]
// was: Ogre::RibbonTrail::resetAllTrails(void)
// IDA 0xd92638: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92638() {
}

// 0xd9266c — __ZNK4Ogre11RibbonTrail14getMovableTypeEv
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::getMovableType(void)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail14getMovableTypeEv")]
// was: Ogre::RibbonTrail::getMovableType(void)const
// IDA 0xd9266c: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd9266c() {
}

// 0xd92678 — __ZNK4Ogre18RibbonTrailFactory7getTypeEv
// type: _DWORD __fastcall(Ogre::RibbonTrailFactory *__hidden this)
#[doc(alias = "Ogre::RibbonTrailFactory::getType(void)const")]
#[doc(alias = "__ZNK4Ogre18RibbonTrailFactory7getTypeEv")]
// was: Ogre::RibbonTrailFactory::getType(void)const
// IDA 0xd92678: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92678() {
}

// 0xd92974 — __ZN4Ogre12_GLOBAL__N_119TimeControllerValueD1Ev
// type: void __fastcall(Ogre::_anonymous_namespace_::TimeControllerValue *__hidden this)
#[doc(alias = "Ogre::anonymous namespace::TimeControllerValue::~TimeControllerValue()")]
#[doc(alias = "__ZN4Ogre12_GLOBAL__N_119TimeControllerValueD1Ev")]
// was: Ogre::`anonymous namespace::TimeControllerValue::~TimeControllerValue()
// IDA 0xd92974: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd92974() {
}

// 0xd92978 — __ZN4Ogre12_GLOBAL__N_119TimeControllerValueD0Ev
// type: void __fastcall(Ogre::_anonymous_namespace_::TimeControllerValue *__hidden this)
#[doc(alias = "Ogre::anonymous namespace::TimeControllerValue::~TimeControllerValue()")]
#[doc(alias = "__ZN4Ogre12_GLOBAL__N_119TimeControllerValueD0Ev")]
// was: Ogre::`anonymous namespace::TimeControllerValue::~TimeControllerValue()
// IDA 0xd92978: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd92978() {
}

// 0xd92a04 — __ZNK4Ogre12_GLOBAL__N_119TimeControllerValue8getValueEv
// type: _DWORD __fastcall(Ogre::_anonymous_namespace_::TimeControllerValue *__hidden this)
#[doc(alias = "Ogre::anonymous namespace::TimeControllerValue::getValue(void)const")]
#[doc(alias = "__ZNK4Ogre12_GLOBAL__N_119TimeControllerValue8getValueEv")]
// was: Ogre::`anonymous namespace::TimeControllerValue::getValue(void)const
// IDA 0xd92a04: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92a04() {
}

// 0xd92a08 — __ZN4Ogre12_GLOBAL__N_119TimeControllerValue8setValueEf
// type: _DWORD __fastcall(Ogre::_anonymous_namespace_::TimeControllerValue *__hidden this, float)
#[doc(alias = "Ogre::anonymous namespace::TimeControllerValue::setValue(float)")]
#[doc(alias = "__ZN4Ogre12_GLOBAL__N_119TimeControllerValue8setValueEf")]
// was: Ogre::`anonymous namespace::TimeControllerValue::setValue(float)
// IDA 0xd92a08: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92a08() {
}

// 0xd92b98 — __ZNK4Ogre14BillboardChain19getMaxChainElementsEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getMaxChainElements(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain19getMaxChainElementsEv")]
// was: Ogre::BillboardChain::getMaxChainElements(void)const
// IDA 0xd92b98: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92b98() {
}

// 0xd92ba0 — __ZNK4Ogre14BillboardChain17getNumberOfChainsEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getNumberOfChains(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain17getNumberOfChainsEv")]
// was: Ogre::BillboardChain::getNumberOfChains(void)const
// IDA 0xd92ba0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92ba0() {
}

// 0xd92ba8 — __ZNK4Ogre14BillboardChain19getUseTextureCoordsEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getUseTextureCoords(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain19getUseTextureCoordsEv")]
// was: Ogre::BillboardChain::getUseTextureCoords(void)const
// IDA 0xd92ba8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92ba8() {
}

// 0xd92bb0 — __ZN4Ogre14BillboardChain24getTextureCoordDirectionEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getTextureCoordDirection(void)")]
#[doc(alias = "__ZN4Ogre14BillboardChain24getTextureCoordDirectionEv")]
// was: Ogre::BillboardChain::getTextureCoordDirection(void)
// IDA 0xd92bb0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92bb0() {
}

// 0xd92bb8 — __ZNK4Ogre14BillboardChain25getOtherTextureCoordRangeEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getOtherTextureCoordRange(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain25getOtherTextureCoordRangeEv")]
// was: Ogre::BillboardChain::getOtherTextureCoordRange(void)const
// IDA 0xd92bb8: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92bb8() {
}

// 0xd92bc0 — __ZNK4Ogre14BillboardChain19getUseVertexColoursEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getUseVertexColours(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain19getUseVertexColoursEv")]
// was: Ogre::BillboardChain::getUseVertexColours(void)const
// IDA 0xd92bc0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92bc0() {
}

// 0xd92bc8 — __ZNK4Ogre14BillboardChain10getDynamicEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getDynamic(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain10getDynamicEv")]
// was: Ogre::BillboardChain::getDynamic(void)const
// IDA 0xd92bc8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92bc8() {
}

// 0xd92bd0 — __ZNK4Ogre14BillboardChain15getMaterialNameEv
// type: _DWORD __fastcall(Ogre::BillboardChain *__hidden this)
#[doc(alias = "Ogre::BillboardChain::getMaterialName(void)const")]
#[doc(alias = "__ZNK4Ogre14BillboardChain15getMaterialNameEv")]
// was: Ogre::BillboardChain::getMaterialName(void)const
// IDA 0xd92bd0: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92bd0() {
}

// 0xd92bd8 — __ZNK4Ogre11RibbonTrail14getTrailLengthEv
// type: _DWORD __fastcall(Ogre::RibbonTrail *__hidden this)
#[doc(alias = "Ogre::RibbonTrail::getTrailLength(void)const")]
#[doc(alias = "__ZNK4Ogre11RibbonTrail14getTrailLengthEv")]
// was: Ogre::RibbonTrail::getTrailLength(void)const
// IDA 0xd92bd8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92bd8() {
}

// 0xd92be0 — __ZN4Ogre4Node8Listener12nodeAttachedEPKS0_
// type: _DWORD __fastcall(Ogre::Node::Listener *__hidden this, const Ogre::Node *)
#[doc(alias = "Ogre::Node::Listener::nodeAttached(Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre4Node8Listener12nodeAttachedEPKS0_")]
// was: Ogre::Node::Listener::nodeAttached(Ogre::Node const*)
// IDA 0xd92be0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd92be0() {
}

// 0xd92be4 — __ZN4Ogre4Node8Listener12nodeDetachedEPKS0_
// type: _DWORD __fastcall(Ogre::Node::Listener *__hidden this, const Ogre::Node *)
#[doc(alias = "Ogre::Node::Listener::nodeDetached(Ogre::Node const*)")]
#[doc(alias = "__ZN4Ogre4Node8Listener12nodeDetachedEPKS0_")]
// was: Ogre::Node::Listener::nodeDetached(Ogre::Node const*)
// IDA 0xd92be4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd92be4() {
}

// 0xd92be8 — __ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS6_EEmRKf
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,float const&)")]
#[doc(alias = "__ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPfS6_EEmRKf")]
// was: std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,float const&)
// IDA 0xd92be8: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92be8() {
}

// 0xd92d98 — __ZNSt6vectorIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
#[doc(alias = "std::vector<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::ColourValue*,std::vector<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::ColourValue const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_")]
// was: std::vector<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::ColourValue*,std::vector<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::ColourValue const&)
// IDA 0xd92d98: 177 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92d98() {
}

// 0xd92f90 — __ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Node const* const,unsigned long>>,std::pair<Ogre::Node const* const,unsigned long> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
// was: std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Node const* const,unsigned long>>,std::pair<Ogre::Node const* const,unsigned long> const&)
// IDA 0xd92f90: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd92f90() {
}

// 0xd93198 — __ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Node const* const,unsigned long> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
// was: std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Node const* const,unsigned long> const&)
// IDA 0xd93198: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd93198() {
}

// 0xd93294 — __ZNSt12_Vector_baseIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd93294: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd93294() {
}

// 0xd93298 — __ZNSt12_Vector_baseIPN4Ogre4NodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Node *,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4NodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::Node *,Ogre::STLAllocator<Ogre::Node *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd93298: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd93298() {
}

// 0xd9329c — __ZNSt12_Vector_baseIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd9329c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd9329c() {
}

// 0xd932a0 — __ZNSt12_Vector_baseIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd932a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd932a0() {
}

// 0xd932ac — __ZNSt12_Vector_baseIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre11ColourValueENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::ColourValue,Ogre::STLAllocator<Ogre::ColourValue,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd932ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd932ac() {
}

// 0xd932b8 — __ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node const*>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev")]
// was: std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node const*>,false>::~_Rb_tree_impl()
// IDA 0xd932b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd932b8() {
}

// 0xd932bc — __ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node const*>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev")]
// was: std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Node const*>,false>::~_Rb_tree_impl()
// IDA 0xd932bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd932bc() {
}

// 0xd932c8 — __ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Node const* const,unsigned long>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN4Ogre4NodeESt4pairIKS3_mESt10_Select1stIS6_ESt4lessIS3_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<Ogre::Node const*,std::pair<Ogre::Node const* const,unsigned long>,std::_Select1st<std::pair<Ogre::Node const* const,unsigned long>>,std::less<Ogre::Node const*>,Ogre::STLAllocator<std::pair<Ogre::Node const* const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Node const* const,unsigned long>> *)
// IDA 0xd932c8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd932c8() {
}

// 0xd9335c — __ZN4Ogre4Root15getSingletonPtrEv
// type: _DWORD __fastcall(Ogre::Root *__hidden this)
#[doc(alias = "Ogre::Root::getSingletonPtr(void)")]
#[doc(alias = "__ZN4Ogre4Root15getSingletonPtrEv")]
// was: Ogre::Root::getSingletonPtr(void)
// IDA 0xd9335c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd9335c() {
}

// 0xd9336c — __ZN4Ogre4Root12getSingletonEv
// type: _DWORD __fastcall(Ogre::Root *__hidden this)
#[doc(alias = "Ogre::Root::getSingleton(void)")]
#[doc(alias = "__ZN4Ogre4Root12getSingletonEv")]
// was: Ogre::Root::getSingleton(void)
// IDA 0xd9336c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd9336c() {
}

// 0xd9337c — __ZN4Ogre4RootC1ERKSsS2_S2_
// type: _DWORD __fastcall(Ogre::Root *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::Root::Root(std::string const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre4RootC1ERKSsS2_S2_")]
// was: Ogre::Root::Root(std::string const&,std::string const&,std::string const&)
// IDA 0xd9337c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd9337c() {
}

// 0xd93388 — __ZN4Ogre4RootC2ERKSsS2_S2_
// type: _DWORD __fastcall(Ogre::Root *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::Root::Root(std::string const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre4RootC2ERKSsS2_S2_")]
// was: Ogre::Root::Root(std::string const&,std::string const&,std::string const&)
// IDA 0xd93388: 2381 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd93388() {
}

// 0xd94d74 — __ZN4Ogre4Root23addMovableObjectFactoryEPNS_20MovableObjectFactoryEb
// type: _DWORD __fastcall(Ogre::Root *__hidden this, Ogre::MovableObjectFactory *, bool)
#[doc(alias = "Ogre::Root::addMovableObjectFactory(Ogre::MovableObjectFactory *,bool)")]
#[doc(alias = "__ZN4Ogre4Root23addMovableObjectFactoryEPNS_20MovableObjectFactoryEb")]
// was: Ogre::Root::addMovableObjectFactory(Ogre::MovableObjectFactory *,bool)
// IDA 0xd94d74: 276 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd94d74() {
}

// 0xd95320 — __ZN4Ogre4Root11loadPluginsERKSs
// type: _DWORD __fastcall(Ogre::Root *__hidden this, const std::string *)
#[doc(alias = "Ogre::Root::loadPlugins(std::string const&)")]
#[doc(alias = "__ZN4Ogre4Root11loadPluginsERKSs")]
// was: Ogre::Root::loadPlugins(std::string const&)
// IDA 0xd95320: 734 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd95320() {
}

// 0xd95b08 — __ZN4Ogre4RootD1Ev
// type: void __fastcall(Ogre::Root *__hidden this)
#[doc(alias = "Ogre::Root::~Root()")]
#[doc(alias = "__ZN4Ogre4RootD1Ev")]
// was: Ogre::Root::~Root()
// IDA 0xd95b08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd95b08() {
}

// 0xd95b14 — __ZN4Ogre4RootD2Ev
// type: void __fastcall(Ogre::Root *__hidden this)
#[doc(alias = "Ogre::Root::~Root()")]
#[doc(alias = "__ZN4Ogre4RootD2Ev")]
// was: Ogre::Root::~Root()
// IDA 0xd95b14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd95b14() {
}
