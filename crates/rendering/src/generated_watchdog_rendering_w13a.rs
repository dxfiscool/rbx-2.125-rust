//! rendering shard watchdog_rendering_w13a — 120 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xcdf890..0xd02934, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|G3D), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcdf890 — __ZN4Ogre8MaterialD2Ev
// type: void __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::~Material()")]
// was: __ZN4Ogre8MaterialD2Ev
// IDA 0xcdf890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cdf890() {
}

// 0xcdfae0 — __ZN4Ogre8Material19removeAllTechniquesEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::removeAllTechniques(void)")]
// was: __ZN4Ogre8Material19removeAllTechniquesEv
// IDA 0xcdfae0: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdfae0() {
}

// 0xcdfbac — __ZN4Ogre8MaterialaSERKS0_
#[doc(alias = "Ogre::Material::operator=(Ogre::Material const&)")]
// was: __ZN4Ogre8MaterialaSERKS0_
// IDA 0xcdfbac: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdfbac() {
}

// 0xcdfd44 — __ZN4Ogre8Material15createTechniqueEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::createTechnique(void)")]
// was: __ZN4Ogre8Material15createTechniqueEv
// IDA 0xcdfd44: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdfd44() {
}

// 0xcdfe40 — __ZN4Ogre8Material24insertSupportedTechniqueEPNS_9TechniqueE
// type: _DWORD __fastcall(Ogre::Material *__hidden this, Ogre::Technique *)
#[doc(alias = "Ogre::Material::insertSupportedTechnique(Ogre::Technique *)")]
// was: __ZN4Ogre8Material24insertSupportedTechniqueEPNS_9TechniqueE
// IDA 0xcdfe40: 105 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdfe40() {
}

// 0xcdff50 — __ZN4Ogre8Material11prepareImplEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::prepareImpl(void)")]
// was: __ZN4Ogre8Material11prepareImplEv
// IDA 0xcdff50: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdff50() {
}

// 0xcdff7c — __ZN4Ogre8Material7compileEb
// type: _DWORD __fastcall(Ogre::Material *__hidden this, bool)
#[doc(alias = "Ogre::Material::compile(bool)")]
// was: __ZN4Ogre8Material7compileEb
// IDA 0xcdff7c: 465 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cdff7c() {
}

// 0xce04b4 — __ZN4Ogre8Material13unprepareImplEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::unprepareImpl(void)")]
// was: __ZN4Ogre8Material13unprepareImplEv
// IDA 0xce04b4: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce04b4() {
}

// 0xce04d0 — __ZN4Ogre8Material8loadImplEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::loadImpl(void)")]
// was: __ZN4Ogre8Material8loadImplEv
// IDA 0xce04d0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce04d0() {
}

// 0xce04ec — __ZN4Ogre8Material10unloadImplEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::unloadImpl(void)")]
// was: __ZN4Ogre8Material10unloadImplEv
// IDA 0xce04ec: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce04ec() {
}

// 0xce0508 — __ZNK4Ogre8Material5cloneERKSsbS2_
// type: _DWORD __fastcall(Ogre::Material *__hidden this, const std::string *, bool, const std::string *)
#[doc(alias = "Ogre::Material::clone(std::string const&,bool,std::string const&)const")]
// was: __ZNK4Ogre8Material5cloneERKSsbS2_
// IDA 0xce0508: 450 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0508() {
}

// 0xce0954 — __ZNK4Ogre8Material13copyDetailsToERNS_11MaterialPtrE
// type: _DWORD __fastcall(Ogre::Material *__hidden this, Ogre::MaterialPtr *)
#[doc(alias = "Ogre::Material::copyDetailsTo(Ogre::MaterialPtr &)const")]
// was: __ZNK4Ogre8Material13copyDetailsToERNS_11MaterialPtrE
// IDA 0xce0954: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0954() {
}

// 0xce0b48 — __ZN4Ogre8Material12getTechniqueEt
// type: _DWORD __fastcall(Ogre::Material *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Material::getTechnique(unsigned short)")]
// was: __ZN4Ogre8Material12getTechniqueEt
// IDA 0xce0b48: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0b48() {
}

// 0xce0b50 — __ZN4Ogre8Material12getTechniqueERKSs
#[doc(alias = "Ogre::Material::getTechnique(std::string const&)")]
// was: __ZN4Ogre8Material12getTechniqueERKSs
// IDA 0xce0b50: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0b50() {
}

// 0xce0bb0 — __ZNK4Ogre8Material16getNumTechniquesEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::getNumTechniques(void)const")]
// was: __ZNK4Ogre8Material16getNumTechniquesEv
// IDA 0xce0bb0: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0bb0() {
}

// 0xce0bbc — __ZN4Ogre8Material21getSupportedTechniqueEt
// type: _DWORD __fastcall(Ogre::Material *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::Material::getSupportedTechnique(unsigned short)")]
// was: __ZN4Ogre8Material21getSupportedTechniqueEt
// IDA 0xce0bbc: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0bbc() {
}

// 0xce0bc4 — __ZNK4Ogre8Material25getNumSupportedTechniquesEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::getNumSupportedTechniques(void)const")]
// was: __ZNK4Ogre8Material25getNumSupportedTechniquesEv
// IDA 0xce0bc4: 4 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0bc4() {
}

// 0xce0bd0 — __ZN4Ogre8Material16getBestTechniqueEtPKNS_10RenderableE
// type: _DWORD __fastcall(Ogre::Material *__hidden this, unsigned __int16, const Ogre::Renderable *)
#[doc(alias = "Ogre::Material::getBestTechnique(unsigned short,Ogre::Renderable const*)")]
// was: __ZN4Ogre8Material16getBestTechniqueEtPKNS_10RenderableE
// IDA 0xce0bd0: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0bd0() {
}

// 0xce0cb8 — __ZN4Ogre8Material22clearBestTechniqueListEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::clearBestTechniqueList(void)")]
// was: __ZN4Ogre8Material22clearBestTechniqueListEv
// IDA 0xce0cb8: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0cb8() {
}

// 0xce0da4 — __ZN4Ogre8Material20getTechniqueIteratorEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::getTechniqueIterator(void)")]
// was: __ZN4Ogre8Material20getTechniqueIteratorEv
// IDA 0xce0da4: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0da4() {
}

// 0xce0db0 — __ZN4Ogre8Material29getSupportedTechniqueIteratorEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::getSupportedTechniqueIterator(void)")]
// was: __ZN4Ogre8Material29getSupportedTechniqueIteratorEv
// IDA 0xce0db0: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0db0() {
}

// 0xce0dbc — __ZNK4Ogre8Material13isTransparentEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::isTransparent(void)const")]
// was: __ZNK4Ogre8Material13isTransparentEv
// IDA 0xce0dbc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0dbc() {
}

// 0xce0ddc — __ZN4Ogre8Material20setDepthCheckEnabledEb
// type: _DWORD __fastcall(Ogre::Material *__hidden this, bool)
#[doc(alias = "Ogre::Material::setDepthCheckEnabled(bool)")]
// was: __ZN4Ogre8Material20setDepthCheckEnabledEb
// IDA 0xce0ddc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0ddc() {
}

// 0xce0dfc — __ZN4Ogre8Material20setDepthWriteEnabledEb
// type: _DWORD __fastcall(Ogre::Material *__hidden this, bool)
#[doc(alias = "Ogre::Material::setDepthWriteEnabled(bool)")]
// was: __ZN4Ogre8Material20setDepthWriteEnabledEb
// IDA 0xce0dfc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0dfc() {
}

// 0xce0e1c — __ZN4Ogre8Material18setLightingEnabledEb
// type: _DWORD __fastcall(Ogre::Material *__hidden this, bool)
#[doc(alias = "Ogre::Material::setLightingEnabled(bool)")]
// was: __ZN4Ogre8Material18setLightingEnabledEb
// IDA 0xce0e1c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0e1c() {
}

// 0xce0e3c — __ZN4Ogre8Material6setFogEbNS_7FogModeERKNS_11ColourValueEfff
// type: int __fastcall(int, int, int, int, float, float, float)
#[doc(alias = "Ogre::Material::setFog(bool,Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
// was: __ZN4Ogre8Material6setFogEbNS_7FogModeERKNS_11ColourValueEfff
// IDA 0xce0e3c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0e3c() {
}

// 0xce0ea4 — __ZN4Ogre8Material16setSceneBlendingENS_14SceneBlendTypeE
#[doc(alias = "Ogre::Material::setSceneBlending(Ogre::SceneBlendType)")]
// was: __ZN4Ogre8Material16setSceneBlendingENS_14SceneBlendTypeE
// IDA 0xce0ea4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0ea4() {
}

// 0xce0ec4 — __ZN4Ogre8Material16setSceneBlendingENS_16SceneBlendFactorES1_
#[doc(alias = "Ogre::Material::setSceneBlending(Ogre::SceneBlendFactor,Ogre::SceneBlendFactor)")]
// was: __ZN4Ogre8Material16setSceneBlendingENS_16SceneBlendFactorES1_
// IDA 0xce0ec4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0ec4() {
}

// 0xce0ef0 — __ZN4Ogre8Material21_notifyNeedsRecompileEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::_notifyNeedsRecompile(void)")]
// was: __ZN4Ogre8Material21_notifyNeedsRecompileEv
// IDA 0xce0ef0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0ef0() {
}

// 0xce0f14 — __ZN4Ogre8Material12setLodLevelsERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::Material::setLodLevels(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: __ZN4Ogre8Material12setLodLevelsERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xce0f14: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce0f14() {
}

// 0xce1014 — __ZNK4Ogre8Material11getLodIndexEf
// type: _DWORD __fastcall(Ogre::Material *__hidden this, float)
#[doc(alias = "Ogre::Material::getLodIndex(float)const")]
// was: __ZNK4Ogre8Material11getLodIndexEf
// IDA 0xce1014: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1014() {
}

// 0xce102c — __ZNK4Ogre8Material19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "Ogre::Material::applyTextureAliases(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,bool)const")]
// was: __ZNK4Ogre8Material19applyTextureAliasesERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEb
// IDA 0xce102c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce102c() {
}

// 0xce1060 — __ZNK4Ogre8Material14getLodStrategyEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::getLodStrategy(void)const")]
// was: __ZNK4Ogre8Material14getLodStrategyEv
// IDA 0xce1060: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1060() {
}

// 0xce1068 — __ZN4Ogre8Material14setLodStrategyEPNS_11LodStrategyE
// type: _DWORD __fastcall(Ogre::Material *__hidden this, Ogre::LodStrategy *)
#[doc(alias = "Ogre::Material::setLodStrategy(Ogre::LodStrategy *)")]
// was: __ZN4Ogre8Material14setLodStrategyEPNS_11LodStrategyE
// IDA 0xce1068: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1068() {
}

// 0xce10c0 — __ZNK4Ogre8Material13calculateSizeEv
// type: _DWORD __fastcall(Ogre::Material *__hidden this)
#[doc(alias = "Ogre::Material::calculateSize(void)const")]
// was: __ZNK4Ogre8Material13calculateSizeEv
// IDA 0xce10c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce10c0() {
}

// 0xce10e0 — __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE8_M_eraseEPSt13_Rb_tree_nodeISG_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE8_M_eraseEPSt13_Rb_tree_nodeISG_E
// IDA 0xce10e0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce10e0() {
}

// 0xce1108 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Technique *>,std::_Select1st<std::pair<unsigned short const,Ogre::Technique *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::Technique *> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// IDA 0xce1108: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1108() {
}

// 0xce1204 — __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISG_ERKSG_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISG_ERKSG_
// IDA 0xce1204: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1204() {
}

// 0xce140c — __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueERKSG_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE16_M_insert_uniqueERKSG_
// IDA 0xce140c: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce140c() {
}

// 0xce1508 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Technique *>,std::_Select1st<std::pair<unsigned short const,Ogre::Technique *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// IDA 0xce1508: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ce1508() {
}

// 0xce150c — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Technique *>,std::_Select1st<std::pair<unsigned short const,Ogre::Technique *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// IDA 0xce150c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce150c() {
}

// 0xce1518 — __ZNSt6vectorIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Technique *,Ogre::STLAllocator<Ogre::Technique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique **,std::vector<Ogre::Technique *,Ogre::STLAllocator<Ogre::Technique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique * const&)")]
// was: __ZNSt6vectorIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// IDA 0xce1518: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ce1518() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xce1610 — __ZNSt12_Vector_baseIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique *,Ogre::STLAllocator<Ogre::Technique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xce1610: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ce1610() {
}

// 0xce1614 — __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE13_Rb_tree_implIS7_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE13_Rb_tree_implIS7_Lb0EED1Ev
// IDA 0xce1614: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ce1614() {
}

// 0xce1618 — __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE13_Rb_tree_implIS7_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::map<unsigned short,Ogre::Technique *,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPSt3mapItPN4Ogre9TechniqueESt4lessItENS3_12STLAllocatorIS0_IS1_S5_ENS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISG_ES7_NS8_ISG_SC_EEE13_Rb_tree_implIS7_Lb0EED0Ev
// IDA 0xce1618: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce1618() {
}

// 0xce1624 — __ZNSt12_Vector_baseIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique *,Ogre::STLAllocator<Ogre::Technique *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre9TechniqueENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xce1624: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce1624() {
}

// 0xce1630 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::Technique *>,std::_Select1st<std::pair<unsigned short const,Ogre::Technique *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::Technique *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::Technique *>> *)")]
// was: __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre9TechniqueEESt10_Select1stIS5_ESt4lessItENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0xce1630: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1630() {
}

// 0xce168c — __ZN4Ogre15MaterialManager15getSingletonPtrEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::getSingletonPtr(void)")]
// was: __ZN4Ogre15MaterialManager15getSingletonPtrEv
// IDA 0xce168c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce168c() {
}

// 0xce169c — __ZN4Ogre15MaterialManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::getSingleton(void)")]
// was: __ZN4Ogre15MaterialManager12getSingletonEv
// IDA 0xce169c: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce169c() {
}

// 0xce16ac — __ZN4Ogre15MaterialManagerC1Ev
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::MaterialManager(void)")]
// was: __ZN4Ogre15MaterialManagerC1Ev
// IDA 0xce16ac: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce16ac() {
}

// 0xce16b8 — __ZN4Ogre15MaterialManagerC2Ev
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::MaterialManager(void)")]
// was: __ZN4Ogre15MaterialManagerC2Ev
// IDA 0xce16b8: 269 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce16b8() {
}

// 0xce19b8 — __ZN4Ogre15MaterialManagerD0Ev
// type: void __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::~MaterialManager()")]
// was: __ZN4Ogre15MaterialManagerD0Ev
// IDA 0xce19b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce19b8() {
}

// 0xce1a48 — __ZN4Ogre15MaterialManagerD1Ev
// type: void __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::~MaterialManager()")]
// was: __ZN4Ogre15MaterialManagerD1Ev
// IDA 0xce1a48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce1a48() {
}

// 0xce1a54 — __ZN4Ogre15MaterialManagerD2Ev
// type: void __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::~MaterialManager()")]
// was: __ZN4Ogre15MaterialManagerD2Ev
// IDA 0xce1a54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce1a54() {
}

// 0xce1d64 — __ZN4Ogre15MaterialManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, Ogre::ManualResourceLoader *, int, int, int)
#[doc(alias = "Ogre::MaterialManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: __ZN4Ogre15MaterialManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xce1d64: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1d64() {
}

// 0xce1e38 — __ZN4Ogre15MaterialManager10initialiseEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::initialise(void)")]
// was: __ZN4Ogre15MaterialManager10initialiseEv
// IDA 0xce1e38: 547 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce1e38() {
}

// 0xce23cc — __ZN4Ogre15MaterialManager26setDefaultTextureFilteringENS_20TextureFilterOptionsE
#[doc(alias = "Ogre::MaterialManager::setDefaultTextureFiltering(Ogre::TextureFilterOptions)")]
// was: __ZN4Ogre15MaterialManager26setDefaultTextureFilteringENS_20TextureFilterOptionsE
// IDA 0xce23cc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce23cc() {
}

// 0xce2418 — __ZNK4Ogre15MaterialManager20getDefaultAnisotropyEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::getDefaultAnisotropy(void)const")]
// was: __ZNK4Ogre15MaterialManager20getDefaultAnisotropyEv
// IDA 0xce2418: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2418() {
}

// 0xce2420 — __ZN4Ogre15MaterialManager26setDefaultTextureFilteringENS_10FilterTypeENS_13FilterOptionsE
#[doc(alias = "Ogre::MaterialManager::setDefaultTextureFiltering(Ogre::FilterType,Ogre::FilterOptions)")]
// was: __ZN4Ogre15MaterialManager26setDefaultTextureFilteringENS_10FilterTypeENS_13FilterOptionsE
// IDA 0xce2420: 13 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2420() {
}

// 0xce2440 — __ZN4Ogre15MaterialManager26setDefaultTextureFilteringENS_13FilterOptionsES1_S1_
#[doc(alias = "Ogre::MaterialManager::setDefaultTextureFiltering(Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)")]
// was: __ZN4Ogre15MaterialManager26setDefaultTextureFilteringENS_13FilterOptionsES1_S1_
// IDA 0xce2440: 3 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2440() {
}

// 0xce244c — __ZNK4Ogre15MaterialManager26getDefaultTextureFilteringENS_10FilterTypeE
#[doc(alias = "Ogre::MaterialManager::getDefaultTextureFiltering(Ogre::FilterType)const")]
// was: __ZNK4Ogre15MaterialManager26getDefaultTextureFilteringENS_10FilterTypeE
// IDA 0xce244c: 14 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce244c() {
}

// 0xce2468 — __ZN4Ogre15MaterialManager15_getSchemeIndexERKSs
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::MaterialManager::_getSchemeIndex(std::string const&)")]
// was: __ZN4Ogre15MaterialManager15_getSchemeIndexERKSs
// IDA 0xce2468: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2468() {
}

// 0xce249c — __ZN4Ogre15MaterialManager14_getSchemeNameEt
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::MaterialManager::_getSchemeName(unsigned short)")]
// was: __ZN4Ogre15MaterialManager14_getSchemeNameEt
// IDA 0xce249c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce249c() {
}

// 0xce24dc — __ZNK4Ogre15MaterialManager21_getActiveSchemeIndexEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::_getActiveSchemeIndex(void)const")]
// was: __ZNK4Ogre15MaterialManager21_getActiveSchemeIndexEv
// IDA 0xce24dc: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce24dc() {
}

// 0xce24e4 — __ZNK4Ogre15MaterialManager15getActiveSchemeEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::getActiveScheme(void)const")]
// was: __ZNK4Ogre15MaterialManager15getActiveSchemeEv
// IDA 0xce24e4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce24e4() {
}

// 0xce24e8 — __ZN4Ogre15MaterialManager15setActiveSchemeERKSs
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::MaterialManager::setActiveScheme(std::string const&)")]
// was: __ZN4Ogre15MaterialManager15setActiveSchemeERKSs
// IDA 0xce24e8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce24e8() {
}

// 0xce253c — __ZN4Ogre15MaterialManager11addListenerEPNS0_8ListenerERKSs
#[doc(alias = "Ogre::MaterialManager::addListener(Ogre::MaterialManager::Listener *,std::string const&)")]
// was: __ZN4Ogre15MaterialManager11addListenerEPNS0_8ListenerERKSs
// IDA 0xce253c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce253c() {
}

// 0xce2564 — __ZN4Ogre15MaterialManager14removeListenerEPNS0_8ListenerERKSs
#[doc(alias = "Ogre::MaterialManager::removeListener(Ogre::MaterialManager::Listener *,std::string const&)")]
// was: __ZN4Ogre15MaterialManager14removeListenerEPNS0_8ListenerERKSs
// IDA 0xce2564: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2564() {
}

// 0xce25a0 — __ZN4Ogre15MaterialManager41_arbitrateMissingTechniqueForActiveSchemeEPNS_8MaterialEtPKNS_10RenderableE
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this, Ogre::Material *, unsigned __int16, const Ogre::Renderable *)
#[doc(alias = "Ogre::MaterialManager::_arbitrateMissingTechniqueForActiveScheme(Ogre::Material *,unsigned short,Ogre::Renderable const*)")]
// was: __ZN4Ogre15MaterialManager41_arbitrateMissingTechniqueForActiveSchemeEPNS_8MaterialEtPKNS_10RenderableE
// IDA 0xce25a0: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce25a0() {
}

// 0xce264c — __ZNSt3mapISstSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSstENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_
#[doc(alias = "std::map<std::string,unsigned short,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISstSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSstENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_
// IDA 0xce264c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce264c() {
}

// 0xce2808 — __ZNSt3mapISsSt4listIPN4Ogre15MaterialManager8ListenerENS1_12STLAllocatorIS4_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessISsESaISt4pairIKSsSA_EEEixERSE_
#[doc(alias = "std::map<std::string,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsSt4listIPN4Ogre15MaterialManager8ListenerENS1_12STLAllocatorIS4_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessISsESaISt4pairIKSsSA_EEEixERSE_
// IDA 0xce2808: 247 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2808() {
}

// 0xce2a88 — __ZNK4Ogre15MaterialManager18getDefaultSettingsEv
// type: _DWORD __fastcall(Ogre::MaterialManager *__hidden this)
#[doc(alias = "Ogre::MaterialManager::getDefaultSettings(void)const")]
// was: __ZNK4Ogre15MaterialManager18getDefaultSettingsEv
// IDA 0xce2a88: 22 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2a88() {
}

// 0xce2ac8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE4findERS1_
// IDA 0xce2ac8: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2ac8() {
}

// 0xce2b6c — __ZNSt10_List_baseIPN4Ogre15MaterialManager8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIPN4Ogre15MaterialManager8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
// IDA 0xce2b6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ce2b6c() {
}

// 0xce2b70 — __ZNSt10_List_baseIPN4Ogre15MaterialManager8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIPN4Ogre15MaterialManager8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
// IDA 0xce2b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce2b70() {
}

// 0xce2b7c — __ZNSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEC2ERS0_RKSB_
#[doc(alias = "std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::pair(std::string const&,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: __ZNSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEC2ERS0_RKSB_
// IDA 0xce2b7c: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2b7c() {
}

// 0xce2ce4 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISD_ERKSD_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISD_ERKSD_
// IDA 0xce2ce4: 341 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce2ce4() {
}

// 0xce302c — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE9_M_insertEPSt18_Rb_tree_node_baseSL_RKSD_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, const void **)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE9_M_insertEPSt18_Rb_tree_node_baseSL_RKSD_
// IDA 0xce302c: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce302c() {
}

// 0xce30a0 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueERKSD_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_insert_unique(std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE16_M_insert_uniqueERKSD_
// IDA 0xce30a0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce30a0() {
}

// 0xce3184 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE14_M_create_nodeERKSD_
// type: _DWORD *__fastcall(int, const std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_create_node(std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE14_M_create_nodeERKSD_
// IDA 0xce3184: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce3184() {
}

// 0xce3334 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
// IDA 0xce3334: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce3334() {
}

// 0xce33d8 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,unsigned short>>,std::pair<std::string const,unsigned short> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// IDA 0xce33d8: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce33d8() {
}

// 0xce35b8 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned short> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS2_
// IDA 0xce35b8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce35b8() {
}

// 0xce370c — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,unsigned short> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// IDA 0xce370c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce370c() {
}

// 0xce37f0 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// IDA 0xce37f0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ce37f0() {
}

// 0xce37f4 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// IDA 0xce37f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ce37f4() {
}

// 0xce3800 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned short>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0xce3800: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce3800() {
}

// 0xce3878 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE8_M_eraseEPSt13_Rb_tree_nodeISD_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<Ogre::MaterialManager::Listener *,Ogre::STLAllocator<Ogre::MaterialManager::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIPN4Ogre15MaterialManager8ListenerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessISsESaISD_EE8_M_eraseEPSt13_Rb_tree_nodeISD_E
// IDA 0xce3878: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce3878() {
}

// 0xce5b08 — __ZN4Ogre18convertBlendFactorERKSs
// type: _DWORD __fastcall(Ogre *__hidden this, const std::string *)
#[doc(alias = "Ogre::convertBlendFactor(std::string const&)")]
// was: __ZN4Ogre18convertBlendFactorERKSs
// IDA 0xce5b08: 244 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce5b08() {
}

// 0xce6e70 — __ZN4Ogre22convertCompareFunctionERKSs
// type: _DWORD __fastcall(Ogre *__hidden this, const std::string *)
#[doc(alias = "Ogre::convertCompareFunction(std::string const&)")]
// was: __ZN4Ogre22convertCompareFunctionERKSs
// IDA 0xce6e70: 225 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ce6e70() {
}

// 0xcebdb0 — __ZN4Ogre16convertBlendOpExERKSs
// type: _DWORD __fastcall(Ogre *__hidden this, const std::string *)
#[doc(alias = "Ogre::convertBlendOpEx(std::string const&)")]
// was: __ZN4Ogre16convertBlendOpExERKSs
// IDA 0xcebdb0: 291 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cebdb0() {
}

// 0xcec104 — __ZN4Ogre18convertBlendSourceERKSs
// type: _DWORD __fastcall(Ogre *__hidden this, const std::string *)
#[doc(alias = "Ogre::convertBlendSource(std::string const&)")]
// was: __ZN4Ogre18convertBlendSourceERKSs
// IDA 0xcec104: 196 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cec104() {
}

// 0xcf7e50 — __ZN4Ogre18MaterialSerializerC1Ev
// type: _DWORD __fastcall(Ogre::MaterialSerializer *__hidden this)
#[doc(alias = "Ogre::MaterialSerializer::MaterialSerializer(void)")]
// was: __ZN4Ogre18MaterialSerializerC1Ev
// IDA 0xcf7e50: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cf7e50() {
}

// 0xcf7e5c — __ZN4Ogre18MaterialSerializerC2Ev
// type: _DWORD __fastcall(Ogre::MaterialSerializer *__hidden this)
#[doc(alias = "Ogre::MaterialSerializer::MaterialSerializer(void)")]
// was: __ZN4Ogre18MaterialSerializerC2Ev
// IDA 0xcf7e5c: 5000 insns (PUSH..STREX.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cf7e5c() {
}

// 0xd0140c — __ZN4Ogre18MaterialSerializerD1Ev
// type: void __fastcall(Ogre::MaterialSerializer *__hidden this)
#[doc(alias = "Ogre::MaterialSerializer::~MaterialSerializer()")]
// was: __ZN4Ogre18MaterialSerializerD1Ev
// IDA 0xd0140c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0140c() {
}

// 0xd01418 — __ZN4Ogre18MaterialSerializerD0Ev
// type: void __fastcall(Ogre::MaterialSerializer *__hidden this)
#[doc(alias = "Ogre::MaterialSerializer::~MaterialSerializer()")]
// was: __ZN4Ogre18MaterialSerializerD0Ev
// IDA 0xd01418: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d01418() {
}

// 0xd014a8 — __ZN4Ogre18MaterialSerializerD2Ev
// type: void __fastcall(Ogre::MaterialSerializer *__hidden this)
#[doc(alias = "Ogre::MaterialSerializer::~MaterialSerializer()")]
// was: __ZN4Ogre18MaterialSerializerD2Ev
// IDA 0xd014a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d014a8() {
}

// 0xd018c4 — __ZNSt12_Vector_baseIPN4Ogre18MaterialSerializer8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::MaterialSerializer::Listener *,Ogre::STLAllocator<Ogre::MaterialSerializer::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre18MaterialSerializer8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xd018c4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d018c4() {
}

// 0xd018c8 — __ZNSt12_Vector_baseIPN4Ogre18MaterialSerializer8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::MaterialSerializer::Listener *,Ogre::STLAllocator<Ogre::MaterialSerializer::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre18MaterialSerializer8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xd018c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d018c8() {
}

// 0xd018e4 — __ZNSt12_Vector_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xd018e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d018e4() {
}

// 0xd01c38 — __ZNSt12_Vector_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xd01c38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d01c38() {
}

// 0xd01c78 — __ZN4Ogre4Math4ACosEf
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float)
#[doc(alias = "Ogre::Math::ACos(float)")]
// was: __ZN4Ogre4Math4ACosEf
// IDA 0xd01c78: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01c78() {
}

// 0xd01cd0 — __ZN4Ogre4Math4SignEf
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float)
#[doc(alias = "Ogre::Math::Sign(float)")]
// was: __ZN4Ogre4Math4SignEf
// IDA 0xd01cd0: 13 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01cd0() {
}

// 0xd01d04 — __ZN4Ogre4Math7InvSqrtEf
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float)
#[doc(alias = "Ogre::Math::InvSqrt(float)")]
// was: __ZN4Ogre4Math7InvSqrtEf
// IDA 0xd01d04: 8 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01d04() {
}

// 0xd01d24 — __ZN4Ogre4Math10UnitRandomEv
// type: _DWORD __fastcall(Ogre::Math *__hidden this)
#[doc(alias = "Ogre::Math::UnitRandom(void)")]
// was: __ZN4Ogre4Math10UnitRandomEv
// IDA 0xd01d24: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01d24() {
}

// 0xd01d44 — __ZN4Ogre4Math11RangeRandomEff
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float, float)
#[doc(alias = "Ogre::Math::RangeRandom(float,float)")]
// was: __ZN4Ogre4Math11RangeRandomEff
// IDA 0xd01d44: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01d44() {
}

// 0xd01d80 — __ZN4Ogre4Math15SymmetricRandomEv
// type: _DWORD __fastcall(Ogre::Math *__hidden this)
#[doc(alias = "Ogre::Math::SymmetricRandom(void)")]
// was: __ZN4Ogre4Math15SymmetricRandomEv
// IDA 0xd01d80: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01d80() {
}

// 0xd01dac — __ZN4Ogre4Math19AngleUnitsToRadiansEf
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float)
#[doc(alias = "Ogre::Math::AngleUnitsToRadians(float)")]
// was: __ZN4Ogre4Math19AngleUnitsToRadiansEf
// IDA 0xd01dac: 12 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01dac() {
}

// 0xd01dd8 — __ZN4Ogre4Math19RadiansToAngleUnitsEf
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float)
#[doc(alias = "Ogre::Math::RadiansToAngleUnits(float)")]
// was: __ZN4Ogre4Math19RadiansToAngleUnitsEf
// IDA 0xd01dd8: 12 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01dd8() {
}

// 0xd01e04 — __ZN4Ogre4Math9RealEqualEfff
// type: _DWORD __fastcall(Ogre::Math *__hidden this, float, float, float)
#[doc(alias = "Ogre::Math::RealEqual(float,float,float)")]
// was: __ZN4Ogre4Math9RealEqualEfff
// IDA 0xd01e04: 11 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01e04() {
}

// 0xd01e28 — __ZN4Ogre4Math10intersectsERKNS_3RayERKNS_14AxisAlignedBoxE
#[doc(alias = "Ogre::Math::intersects(Ogre::Ray const&,Ogre::AxisAlignedBox const&)")]
// was: __ZN4Ogre4Math10intersectsERKNS_3RayERKNS_14AxisAlignedBoxE
// IDA 0xd01e28: 329 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d01e28() {
}

// 0xd02350 — __ZN4Ogre4Math35calculateFaceNormalWithoutNormalizeERKNS_7Vector3ES3_S3_
// type: _DWORD __fastcall(Ogre::Math *__hidden this, const Vector3 *, const Vector3 *, const Vector3 *)
#[doc(alias = "Ogre::Math::calculateFaceNormalWithoutNormalize(Ogre::Vector3 const&,Ogre::Vector3 const&,Ogre::Vector3 const&)")]
// was: __ZN4Ogre4Math35calculateFaceNormalWithoutNormalizeERKNS_7Vector3ES3_S3_
// IDA 0xd02350: 36 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d02350() {
}

// 0xd024d4 — __ZN4Ogre4Math22boundingRadiusFromAABBERKNS_14AxisAlignedBoxE
#[doc(alias = "Ogre::Math::boundingRadiusFromAABB(Ogre::AxisAlignedBox const&)")]
// was: __ZN4Ogre4Math22boundingRadiusFromAABBERKNS_14AxisAlignedBoxE
// IDA 0xd024d4: 56 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d024d4() {
}

// 0xd02678 — __ZNK4Ogre7Matrix39GetColumnEm
// type: _DWORD __fastcall(Ogre::Matrix3 *__hidden this, unsigned int)
#[doc(alias = "Ogre::Matrix3::GetColumn(unsigned long)const")]
// was: __ZNK4Ogre7Matrix39GetColumnEm
// IDA 0xd02678: 9 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d02678() {
}

// 0xd0269c — __ZN4Ogre7Matrix38FromAxesERKNS_7Vector3ES3_S3_
// type: _DWORD __fastcall(Ogre::Matrix3 *__hidden this, const Vector3 *, const Vector3 *, const Vector3 *)
#[doc(alias = "Ogre::Matrix3::FromAxes(Ogre::Vector3 const&,Ogre::Vector3 const&,Ogre::Vector3 const&)")]
// was: __ZN4Ogre7Matrix38FromAxesERKNS_7Vector3ES3_S3_
// IDA 0xd0269c: 19 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0269c() {
}

// 0xd026cc — __ZNK4Ogre7Matrix3mlERKS0_
#[doc(alias = "Ogre::Matrix3::operator*(Ogre::Matrix3 const&)const")]
// was: __ZNK4Ogre7Matrix3mlERKS0_
// IDA 0xd026cc: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d026cc() {
}

// 0xd02818 — __ZNK4Ogre7Matrix3mlERKNS_7Vector3E
#[doc(alias = "Ogre::Matrix3::operator*(Ogre::Vector3 const&)const")]
// was: __ZNK4Ogre7Matrix3mlERKNS_7Vector3E
// IDA 0xd02818: 32 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d02818() {
}

// 0xd02898 — __ZNK4Ogre7Matrix3ngEv
#[doc(alias = "Ogre::Matrix3::operator-(void)const")]
// was: __ZNK4Ogre7Matrix3ngEv
// IDA 0xd02898: 29 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d02898() {
}

// 0xd0290c — __ZNK4Ogre7Matrix39TransposeEv
// type: _DWORD __fastcall(Ogre::Matrix3 *__hidden this)
#[doc(alias = "Ogre::Matrix3::Transpose(void)const")]
// was: __ZNK4Ogre7Matrix39TransposeEv
// IDA 0xd0290c: 19 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0290c() {
}

// 0xd02934 — __ZNK4Ogre7Matrix316QDUDecompositionERS0_RNS_7Vector3ES3_
// type: _DWORD __fastcall(Ogre::Matrix3 *__hidden this, Ogre::Matrix3 *, Vector3 *, Vector3 *)
#[doc(alias = "Ogre::Matrix3::QDUDecomposition(Ogre::Matrix3&,Ogre::Vector3 &,Ogre::Vector3 &)const")]
// was: __ZNK4Ogre7Matrix316QDUDecompositionERS0_RNS_7Vector3ES3_
// IDA 0xd02934: 249 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d02934() {
}
