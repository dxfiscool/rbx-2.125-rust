//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xf69e14..0xf6a444 (100 stubs, EA-sorted asc, 13109->13209 covered, 124 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xf69e14 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>> *)
pub fn stub_f69e14() -> ! {
    todo!("0xf69e14 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>> *)")
}

// 0xf69e24 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE9_M_insertEPSt18_Rb_tree_node_baseSM_RKSE_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)
pub fn stub_f69e24() -> ! {
    todo!("0xf69e24 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")
}

// 0xf69e34 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS2_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::string> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::string> const&)
pub fn stub_f69e34() -> ! {
    todo!("0xf69e34 std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::string> const&)")
}

// 0xf69e44 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>)
pub fn stub_f69e44() -> ! {
    todo!("0xf69e44 std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>)")
}

// 0xf69e54 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSF_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)
pub fn stub_f69e54() -> ! {
    todo!("0xf69e54 std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)")
}

// 0xf69e64 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)
pub fn stub_f69e64() -> ! {
    todo!("0xf69e64 std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)")
}

// 0xf69e74 — j___ZNSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSsSsENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE6insertERKS6_
#[doc(alias = "std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert(std::pair<std::string const,std::string> const&)")]
// was: std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert(std::pair<std::string const,std::string> const&)
pub fn stub_f69e74() -> ! {
    todo!("0xf69e74 std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert(std::pair<std::string const,std::string> const&)")
}

// 0xf69e84 — j___ZN4Ogre9SharedPtrINS_11ScriptTokenEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)
pub fn stub_f69e84() -> ! {
    todo!("0xf69e84 Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)")
}

// 0xf69e94 — j___ZN4Ogre9SharedPtrINS_11ScriptTokenEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)
pub fn stub_f69e94() -> ! {
    todo!("0xf69e94 Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)")
}

// 0xf69ea4 — j___ZNSt6vectorIN4Ogre9SharedPtrINS0_11ScriptTokenEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)")]
// was: std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)
pub fn stub_f69ea4() -> ! {
    todo!("0xf69ea4 std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)")
}

// 0xf69eb4 — j___ZN4Ogre33CreateMaterialScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
pub fn stub_f69eb4() -> ! {
    todo!("0xf69eb4 Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69ec4 — j___ZN4Ogre33CreateMaterialScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")]
// was: Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()
pub fn stub_f69ec4() -> ! {
    todo!("0xf69ec4 Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")
}

// 0xf69ed4 — j___ZN4Ogre35CreateCompositorScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
pub fn stub_f69ed4() -> ! {
    todo!("0xf69ed4 Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69ee4 — j___ZN4Ogre35CreateCompositorScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")]
// was: Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()
pub fn stub_f69ee4() -> ! {
    todo!("0xf69ee4 Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")
}

// 0xf69ef4 — j___ZN4Ogre35CreateGpuProgramScriptCompilerEventC2ERKSsS2_S2_S2_S2_NS_14GpuProgramTypeE
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)")]
// was: Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)
pub fn stub_f69ef4() -> ! {
    todo!("0xf69ef4 Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)")
}

// 0xf69f04 — j___ZN4Ogre35CreateGpuProgramScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()
pub fn stub_f69f04() -> ! {
    todo!("0xf69f04 Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")
}

// 0xf69f14 — j___ZN4Ogre39CreateParticleSystemScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
pub fn stub_f69f14() -> ! {
    todo!("0xf69f14 Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69f24 — j___ZN4Ogre39CreateParticleSystemScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")]
// was: Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()
pub fn stub_f69f24() -> ! {
    todo!("0xf69f24 Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")
}

// 0xf69f34 — j___ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventC2ERKSsS2_S2_
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
// was: Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)
pub fn stub_f69f34() -> ! {
    todo!("0xf69f34 Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69f44 — j___ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")]
// was: Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()
pub fn stub_f69f44() -> ! {
    todo!("0xf69f44 Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")
}

// 0xf69f54 — j___ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventC2ERKSsS2_S2_S2_S2_NS_14GpuProgramTypeEPKSt4listISt4pairISsSsENS_12STLAllocatorIS6_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
pub fn stub_f69f54() -> ! {
    todo!("0xf69f54 Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xf69f64 — j___ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventD2Ev
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()
pub fn stub_f69f64() -> ! {
    todo!("0xf69f64 Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")
}

// 0xf69f74 — j___ZN4Ogre8any_castIPNS_10CompositorEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)")]
// was: Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)
pub fn stub_f69f74() -> ! {
    todo!("0xf69f74 Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)")
}

// 0xf69f84 — j___ZN4Ogre8any_castIPNS_14ParticleSystemEEET_RKNS_3AnyE
#[doc(alias = "Ogre::ParticleSystem * Ogre::any_cast<Ogre::ParticleSystem *>(Ogre::Any const&)")]
// was: Ogre::ParticleSystem * Ogre::any_cast<Ogre::ParticleSystem *>(Ogre::Any const&)
pub fn stub_f69f84() -> ! {
    todo!("0xf69f84 Ogre::ParticleSystem * Ogre::any_cast<Ogre::ParticleSystem *>(Ogre::Any const&)")
}

// 0xf69f94 — j___ZN4Ogre8any_castIPNS_16TextureUnitStateEEET_RKNS_3AnyE
#[doc(alias = "Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)")]
// was: Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)
pub fn stub_f69f94() -> ! {
    todo!("0xf69f94 Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)")
}

// 0xf69fa4 — j___ZN4Ogre8any_castIPNS_20CompositionTechniqueEEET_RKNS_3AnyE
#[doc(alias = "Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)")]
// was: Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)
pub fn stub_f69fa4() -> ! {
    todo!("0xf69fa4 Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)")
}

// 0xf69fb4 — j___ZN4Ogre8any_castIPNS_21CompositionTargetPassEEET_RKNS_3AnyE
#[doc(alias = "Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)")]
// was: Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)
pub fn stub_f69fb4() -> ! {
    todo!("0xf69fb4 Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)")
}

// 0xf69fc4 — j___ZN4Ogre8any_castIPNS_4PassEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)")]
// was: Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)
pub fn stub_f69fc4() -> ! {
    todo!("0xf69fc4 Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)")
}

// 0xf69fd4 — j___ZN4Ogre8any_castIPNS_8MaterialEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")]
// was: Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)
pub fn stub_f69fd4() -> ! {
    todo!("0xf69fd4 Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")
}

// 0xf69fe4 — j___ZN4Ogre8any_castIPNS_9TechniqueEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)")]
// was: Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)
pub fn stub_f69fe4() -> ! {
    todo!("0xf69fe4 Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)")
}

// 0xf69ff4 — j___ZNSt10_List_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")]
// was: std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()
pub fn stub_f69ff4() -> ! {
    todo!("0xf69ff4 std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")
}

// 0xf6a004 — j___ZNSt4listISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS1_
#[doc(alias = "std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)")]
// was: std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)
pub fn stub_f6a004() -> ! {
    todo!("0xf6a004 std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)")
}

// 0xf6a014 — j___ZNSt6vectorIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)")]
// was: std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)
pub fn stub_f6a014() -> ! {
    todo!("0xf6a014 std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)")
}

// 0xf6a024 — j___ZNSt6vectorIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_f6a024() -> ! {
    todo!("0xf6a024 std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a034 — j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS6_EERKi
#[doc(alias = "std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)")]
// was: std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)
pub fn stub_f6a034() -> ! {
    todo!("0xf6a034 std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)")
}

// 0xf6a044 — j___ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")]
// was: std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)
pub fn stub_f6a044() -> ! {
    todo!("0xf6a044 std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")
}

// 0xf6a054 — j___ZN4Ogre9SharedPtrINS_10GpuProgramEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)
pub fn stub_f6a054() -> ! {
    todo!("0xf6a054 Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")
}

// 0xf6a064 — j___ZNSt6vectorIN4Ogre7Vector3ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
#[doc(alias = "std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)")]
// was: std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)
pub fn stub_f6a064() -> ! {
    todo!("0xf6a064 std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)")
}

// 0xf6a074 — j___ZN4Ogre12STLAllocatorINS_29LinkedSkeletonAnimationSourceENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS1_
#[doc(alias = "Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::LinkedSkeletonAnimationSource*)")]
// was: Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::LinkedSkeletonAnimationSource*)
pub fn stub_f6a074() -> ! {
    todo!("0xf6a074 Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::LinkedSkeletonAnimationSource*)")
}

// 0xf6a084 — j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
pub fn stub_f6a084() -> ! {
    todo!("0xf6a084 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")
}

// 0xf6a094 — j___ZNSt3mapISsPN4Ogre4BoneESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::Bone *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::Bone *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
pub fn stub_f6a094() -> ! {
    todo!("0xf6a094 std::map<std::string,Ogre::Bone *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xf6a0a4 — j___ZNSt6vectorIN4Ogre29LinkedSkeletonAnimationSourceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LinkedSkeletonAnimationSource*,std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LinkedSkeletonAnimationSource const&)")]
// was: std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LinkedSkeletonAnimationSource*,std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LinkedSkeletonAnimationSource const&)
pub fn stub_f6a0a4() -> ! {
    todo!("0xf6a0a4 std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LinkedSkeletonAnimationSource*,std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LinkedSkeletonAnimationSource const&)")
}

// 0xf6a0b4 — j___ZNSt6vectorIN4Ogre29LinkedSkeletonAnimationSourceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias = "std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::LinkedSkeletonAnimationSource const&)")]
// was: std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::LinkedSkeletonAnimationSource const&)
pub fn stub_f6a0b4() -> ! {
    todo!("0xf6a0b4 std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::LinkedSkeletonAnimationSource const&)")
}

// 0xf6a0c4 — j___ZNSt6vectorIPN4Ogre4BoneENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Bone * const&)")]
// was: std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Bone * const&)
pub fn stub_f6a0c4() -> ! {
    todo!("0xf6a0c4 std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Bone * const&)")
}

// 0xf6a0d4 — j___ZNSt8_Rb_treeIPN4Ogre4BoneES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Bone * const&)")]
// was: std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Bone * const&)
pub fn stub_f6a0d4() -> ! {
    todo!("0xf6a0d4 std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Bone * const&)")
}

// 0xf6a0e4 — j___ZNSt8_Rb_treeIPN4Ogre4BoneES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Bone *>,std::_Rb_tree_iterator<Ogre::Bone *>)")]
// was: std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Bone *>,std::_Rb_tree_iterator<Ogre::Bone *>)
pub fn stub_f6a0e4() -> ! {
    todo!("0xf6a0e4 std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Bone *>,std::_Rb_tree_iterator<Ogre::Bone *>)")
}

// 0xf6a0f4 — j___ZNSt8_Rb_treeIPN4Ogre4BoneES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Bone *> *)")]
// was: std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Bone *> *)
pub fn stub_f6a0f4() -> ! {
    todo!("0xf6a0f4 std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Bone *> *)")
}

// 0xf6a104 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Bone *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Bone *> const&)
pub fn stub_f6a104() -> ! {
    todo!("0xf6a104 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Bone *> const&)")
}

// 0xf6a114 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Bone *>>,std::pair<std::string const,Ogre::Bone *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Bone *>>,std::pair<std::string const,Ogre::Bone *> const&)
pub fn stub_f6a114() -> ! {
    todo!("0xf6a114 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Bone *>>,std::pair<std::string const,Ogre::Bone *> const&)")
}

// 0xf6a124 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
pub fn stub_f6a124() -> ! {
    todo!("0xf6a124 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xf6a134 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Bone *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Bone *>> *)
pub fn stub_f6a134() -> ! {
    todo!("0xf6a134 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Bone *>> *)")
}

// 0xf6a144 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Bone *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Bone *> const&)
pub fn stub_f6a144() -> ! {
    todo!("0xf6a144 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Bone *> const&)")
}

// 0xf6a154 — j___ZSt22__uninitialized_copy_aIPN4Ogre29LinkedSkeletonAnimationSourceES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
#[doc(alias = "Ogre::LinkedSkeletonAnimationSource * std::__uninitialized_copy_a<Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::LinkedSkeletonAnimationSource * std::__uninitialized_copy_a<Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
pub fn stub_f6a154() -> ! {
    todo!("0xf6a154 Ogre::LinkedSkeletonAnimationSource * std::__uninitialized_copy_a<Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}

// 0xf6a164 — j___ZNSt6vectorIPN4Ogre4BoneENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Bone * const&)")]
// was: std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Bone * const&)
pub fn stub_f6a164() -> ! {
    todo!("0xf6a164 std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Bone * const&)")
}

// 0xf6a174 — j___ZN4OgrelsERSoNS_14AxisAlignedBoxE
#[doc(alias = "Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")]
// was: Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)
pub fn stub_f6a174() -> ! {
    todo!("0xf6a174 Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")
}

// 0xf6a184 — j___ZNK4Ogre14AxisAlignedBox12intersectionERKS0_
#[doc(alias = "Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")]
// was: Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const
pub fn stub_f6a184() -> ! {
    todo!("0xf6a184 Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")
}

// 0xf6a194 — j___ZNSt3mapISsPN4Ogre14StaticGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
pub fn stub_f6a194() -> ! {
    todo!("0xf6a194 std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xf6a1a4 — j___ZNSt3mapISsPN4Ogre14StaticGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
pub fn stub_f6a1a4() -> ! {
    todo!("0xf6a1a4 std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xf6a1b4 — j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt20_List_const_iteratorIS1_EEEvSt14_List_iteratorIS1_ET_SD_
#[doc(alias = "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::VertexElement>>(std::_List_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>)")]
// was: void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::VertexElement>>(std::_List_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>)
pub fn stub_f6a1b4() -> ! {
    todo!("0xf6a1b4 void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::VertexElement>>(std::_List_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>)")
}

// 0xf6a1c4 — j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
#[doc(alias = "std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_f6a1c4() -> ! {
    todo!("0xf6a1c4 std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a1d4 — j___ZNSt6vectorIN4Ogre14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)")]
// was: std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)
pub fn stub_f6a1d4() -> ! {
    todo!("0xf6a1d4 std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)")
}

// 0xf6a1e4 — j___ZNSt6vectorIPN4Ogre14StaticGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)")]
// was: std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)
pub fn stub_f6a1e4() -> ! {
    todo!("0xf6a1e4 std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)")
}

// 0xf6a1f4 — j___ZNSt6vectorIPN4Ogre14StaticGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::GeometryBucket **,std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::GeometryBucket * const&)")]
// was: std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::GeometryBucket **,std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::GeometryBucket * const&)
pub fn stub_f6a1f4() -> ! {
    todo!("0xf6a1f4 std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::GeometryBucket **,std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::GeometryBucket * const&)")
}

// 0xf6a204 — j___ZNSt6vectorIPN4Ogre14StaticGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)")]
// was: std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)
pub fn stub_f6a204() -> ! {
    todo!("0xf6a204 std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)")
}

// 0xf6a214 — j___ZNSt6vectorIPN4Ogre14StaticGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)")]
// was: std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)
pub fn stub_f6a214() -> ! {
    todo!("0xf6a214 std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)")
}

// 0xf6a224 — j___ZNSt6vectorIPhN4Ogre12STLAllocatorIS0_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S7_EERKS0_
#[doc(alias = "std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned char **,std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned char * const&)")]
// was: std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned char **,std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned char * const&)
pub fn stub_f6a224() -> ! {
    todo!("0xf6a224 std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned char **,std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned char * const&)")
}

// 0xf6a234 — j___ZNSt6vectorISt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEENS3_IS8_S6_EEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS8_SA_EERKS8_
#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>*,std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>*,std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_f6a234() -> ! {
    todo!("0xf6a234 std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>*,std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a244 — j___ZNSt6vectorISt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEENS3_IS8_S6_EEE9push_backERKS8_
#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_f6a244() -> ! {
    todo!("0xf6a244 std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a254 — j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS6_EERKf
#[doc(alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)")]
// was: std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)
pub fn stub_f6a254() -> ! {
    todo!("0xf6a254 std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)")
}

// 0xf6a264 — j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueERKSF_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
pub fn stub_f6a264() -> ! {
    todo!("0xf6a264 std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")
}

// 0xf6a274 — j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISF_ERKSF_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
pub fn stub_f6a274() -> ! {
    todo!("0xf6a274 std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")
}

// 0xf6a284 — j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISF_E
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)
pub fn stub_f6a284() -> ! {
    todo!("0xf6a284 std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")
}

// 0xf6a294 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)
pub fn stub_f6a294() -> ! {
    todo!("0xf6a294 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")
}

// 0xf6a2a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)
pub fn stub_f6a2a4() -> ! {
    todo!("0xf6a2a4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")
}

// 0xf6a2b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
pub fn stub_f6a2b4() -> ! {
    todo!("0xf6a2b4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xf6a2c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>> *)
pub fn stub_f6a2c4() -> ! {
    todo!("0xf6a2c4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>> *)")
}

// 0xf6a2d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)
pub fn stub_f6a2d4() -> ! {
    todo!("0xf6a2d4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *> const&)")
}

// 0xf6a2e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)
pub fn stub_f6a2e4() -> ! {
    todo!("0xf6a2e4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")
}

// 0xf6a2f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)
pub fn stub_f6a2f4() -> ! {
    todo!("0xf6a2f4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")
}

// 0xf6a304 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
pub fn stub_f6a304() -> ! {
    todo!("0xf6a304 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xf6a314 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>> *)
pub fn stub_f6a314() -> ! {
    todo!("0xf6a314 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>> *)")
}

// 0xf6a324 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)
pub fn stub_f6a324() -> ! {
    todo!("0xf6a324 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")
}

// 0xf6a334 — j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)
pub fn stub_f6a334() -> ! {
    todo!("0xf6a334 std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")
}

// 0xf6a344 — j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)
pub fn stub_f6a344() -> ! {
    todo!("0xf6a344 std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")
}

// 0xf6a354 — j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>> *)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>> *)
pub fn stub_f6a354() -> ! {
    todo!("0xf6a354 std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>> *)")
}

// 0xf6a364 — j___ZNSt8_Rb_treeImSt4pairIKmmESt10_Select1stIS2_ESt4lessImEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,unsigned long> const&)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,unsigned long> const&)
pub fn stub_f6a364() -> ! {
    todo!("0xf6a364 std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,unsigned long> const&)")
}

// 0xf6a374 — j___ZNSt8_Rb_treeImSt4pairIKmmESt10_Select1stIS2_ESt4lessImEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,unsigned long>> *)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,unsigned long>> *)
pub fn stub_f6a374() -> ! {
    todo!("0xf6a374 std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,unsigned long>> *)")
}

// 0xf6a384 — j___ZSt22__uninitialized_copy_aIPSt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEES9_NS3_IS8_S6_EEET0_T_SC_SB_T1_
#[doc(alias = "std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> * std::__uninitialized_copy_a<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> * std::__uninitialized_copy_a<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
pub fn stub_f6a384() -> ! {
    todo!("0xf6a384 std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> * std::__uninitialized_copy_a<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}

// 0xf6a394 — j___ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// was: std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)
pub fn stub_f6a394() -> ! {
    todo!("0xf6a394 std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")
}

// 0xf6a3a4 — j___ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
// was: std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
pub fn stub_f6a3a4() -> ! {
    todo!("0xf6a3a4 std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")
}

// 0xf6a3b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamDictionary>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamDictionary>> *)
pub fn stub_f6a3b4() -> ! {
    todo!("0xf6a3b4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamDictionary>> *)")
}

// 0xf6a3c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
pub fn stub_f6a3c4() -> ! {
    todo!("0xf6a3c4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xf6a3d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> *)
pub fn stub_f6a3d4() -> ! {
    todo!("0xf6a3d4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> *)")
}

// 0xf6a3e4 — j___ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)")]
// was: std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)
pub fn stub_f6a3e4() -> ! {
    todo!("0xf6a3e4 std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUVendorRule*,std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUVendorRule const&)")
}

// 0xf6a3f4 — j___ZNSt6vectorIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias = "std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_f6a3f4() -> ! {
    todo!("0xf6a3f4 std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a404 — j___ZNSt6vectorIN4Ogre9Technique17GPUDeviceNameRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule const&)")]
// was: std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule const&)
pub fn stub_f6a404() -> ! {
    todo!("0xf6a404 std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule const&)")
}

// 0xf6a414 — j___ZNSt6vectorIN4Ogre9Technique17GPUDeviceNameRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS8_
#[doc(alias = "std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_f6a414() -> ! {
    todo!("0xf6a414 std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a424 — j___ZNSt6vectorIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)")]
// was: std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)
pub fn stub_f6a424() -> ! {
    todo!("0xf6a424 std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::IlluminationPass **,std::vector<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::IlluminationPass * const&)")
}

// 0xf6a434 — j___ZNSt6vectorIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)")]
// was: std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)
pub fn stub_f6a434() -> ! {
    todo!("0xf6a434 std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pass **,std::vector<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pass * const&)")
}

// 0xf6a444 — j___ZSt22__uninitialized_copy_aIN9__gnu_cxx17__normal_iteratorIPKN4Ogre9Technique17GPUDeviceNameRuleESt6vectorIS4_NS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEPS4_SC_ET0_T_SH_SG_T1_
#[doc(alias = "Ogre::Technique::GPUDeviceNameRule* std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
// was: Ogre::Technique::GPUDeviceNameRule* std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)
pub fn stub_f6a444() -> ! {
    todo!("0xf6a444 Ogre::Technique::GPUDeviceNameRule* std::__uninitialized_copy_a<__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::Technique::GPUDeviceNameRule const*,std::vector<Ogre::Technique::GPUDeviceNameRule,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Technique::GPUDeviceNameRule*,Ogre::STLAllocator<Ogre::Technique::GPUDeviceNameRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}
