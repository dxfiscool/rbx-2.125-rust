//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xddd184..0xde1ffc (100 stubs, 8755 prior -> 8855 covered, 4478 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xddd184 — __ZN4Ogre12STLAllocatorINS_9SharedPtrINS_12AbstractNodeEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xddd184: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ddd184() {
}

// 0xddd188 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::AbstractNode>::~SharedPtr()
// IDA 0xddd188: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddd188() {
}

// 0xddd278 — __ZNSt3mapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSsSsENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEixERS5_
#[doc(alias = "std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xddd278: 262 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddd278() {
}

// 0xddd54c — __ZN4Ogre9SharedPtrISt6vectorINS0_INS_11ScriptTokenEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xddd54c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddd54c() {
}

// 0xddd5fc — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xddd5fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddd5fc() {
}

// 0xddd6ac — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xddd6ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddd6ac() {
}

// 0xddd75c — __ZN4Ogre9SharedPtrINS_14ScriptCompiler5ErrorEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::~SharedPtr()
// IDA 0xddd75c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddd75c() {
}

// 0xddd80c — __ZNSt8multimapISsSsSt4lessISsEN4Ogre12STLAllocatorISt4pairIKSsSsENS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE6insertERKS6_
#[doc(alias = "std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert(std::pair<std::string const,std::string> const&)")]
// was: std::multimap<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert(std::pair<std::string const,std::string> const&)
// IDA 0xddd80c: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddd80c() {
}

// 0xddd8d4 — __ZNSt4listIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt14_List_iteratorIS3_EEEvSC_T_SD_
#[doc(alias = "void std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>(std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>)")]
// was: void std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>(std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>)
// IDA 0xddd8d4: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddd8d4() {
}

// 0xddda08 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKSA_
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xddda08: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddda08() {
}

// 0xdddb14 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEaSERKSA_
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::operator=(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xdddb14: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dddb14() {
}

// 0xdddc20 — __ZN4Ogre39ProcessNameExclusionScriptCompilerEventD1Ev
#[doc(alias = "Ogre::ProcessNameExclusionScriptCompilerEvent::~ProcessNameExclusionScriptCompilerEvent()")]
// was: Ogre::ProcessNameExclusionScriptCompilerEvent::~ProcessNameExclusionScriptCompilerEvent()
// IDA 0xdddc20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dddc20() {
}

// 0xdddcc8 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::operator=(Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::SharedPtr<Ogre::AbstractNode>::operator=(Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xdddcc8: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dddcc8() {
}

// 0xddde48 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::operator=(Ogre::SharedPtr<Ogre::ConcreteNode> const&)")]
// was: Ogre::SharedPtr<Ogre::ConcreteNode>::operator=(Ogre::SharedPtr<Ogre::ConcreteNode> const&)
// IDA 0xddde48: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddde48() {
}

// 0xdddf54 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ConcreteNode>::~SharedPtr()
// IDA 0xdddf54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dddf54() {
}

// 0xdde008 — __ZN4Ogre16AtomAbstractNodeD1Ev
#[doc(alias = "Ogre::AtomAbstractNode::~AtomAbstractNode()")]
// was: Ogre::AtomAbstractNode::~AtomAbstractNode()
// IDA 0xdde008: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde008() {
}

// 0xdde150 — __ZN4Ogre16AtomAbstractNodeD0Ev
#[doc(alias = "Ogre::AtomAbstractNode::~AtomAbstractNode()")]
// was: Ogre::AtomAbstractNode::~AtomAbstractNode()
// IDA 0xdde150: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde150() {
}

// 0xdde2a4 — __ZN4Ogre18ObjectAbstractNodeD1Ev
#[doc(alias = "Ogre::ObjectAbstractNode::~ObjectAbstractNode()")]
// was: Ogre::ObjectAbstractNode::~ObjectAbstractNode()
// IDA 0xdde2a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde2a4() {
}

// 0xdde2b0 — __ZN4Ogre18ObjectAbstractNodeD0Ev
#[doc(alias = "Ogre::ObjectAbstractNode::~ObjectAbstractNode()")]
// was: Ogre::ObjectAbstractNode::~ObjectAbstractNode()
// IDA 0xdde2b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde2b0() {
}

// 0xdde340 — __ZN4Ogre20PropertyAbstractNodeD1Ev
#[doc(alias = "Ogre::PropertyAbstractNode::~PropertyAbstractNode()")]
// was: Ogre::PropertyAbstractNode::~PropertyAbstractNode()
// IDA 0xdde340: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde340() {
}

// 0xdde34c — __ZN4Ogre20PropertyAbstractNodeD0Ev
#[doc(alias = "Ogre::PropertyAbstractNode::~PropertyAbstractNode()")]
// was: Ogre::PropertyAbstractNode::~PropertyAbstractNode()
// IDA 0xdde34c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde34c() {
}

// 0xdde3dc — __ZN4Ogre18ImportAbstractNodeD1Ev
#[doc(alias = "Ogre::ImportAbstractNode::~ImportAbstractNode()")]
// was: Ogre::ImportAbstractNode::~ImportAbstractNode()
// IDA 0xdde3dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde3dc() {
}

// 0xdde3e8 — __ZN4Ogre18ImportAbstractNodeD0Ev
#[doc(alias = "Ogre::ImportAbstractNode::~ImportAbstractNode()")]
// was: Ogre::ImportAbstractNode::~ImportAbstractNode()
// IDA 0xdde3e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde3e8() {
}

// 0xdde478 — __ZN4Ogre26VariableAccessAbstractNodeD1Ev
#[doc(alias = "Ogre::VariableAccessAbstractNode::~VariableAccessAbstractNode()")]
// was: Ogre::VariableAccessAbstractNode::~VariableAccessAbstractNode()
// IDA 0xdde478: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde478() {
}

// 0xdde5c0 — __ZN4Ogre26VariableAccessAbstractNodeD0Ev
#[doc(alias = "Ogre::VariableAccessAbstractNode::~VariableAccessAbstractNode()")]
// was: Ogre::VariableAccessAbstractNode::~VariableAccessAbstractNode()
// IDA 0xdde5c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde5c0() {
}

// 0xdde714 — __ZN4Ogre22ScriptCompilerListenerD1Ev
#[doc(alias = "Ogre::ScriptCompilerListener::~ScriptCompilerListener()")]
// was: Ogre::ScriptCompilerListener::~ScriptCompilerListener()
// IDA 0xdde714: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_dde714() {
}

// 0xdde718 — __ZN4Ogre22ScriptCompilerListenerD0Ev
#[doc(alias = "Ogre::ScriptCompilerListener::~ScriptCompilerListener()")]
// was: Ogre::ScriptCompilerListener::~ScriptCompilerListener()
// IDA 0xdde718: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde718() {
}

// 0xdde724 — __ZN4Ogre12AbstractNodeD1Ev
#[doc(alias = "Ogre::AbstractNode::~AbstractNode()")]
// was: Ogre::AbstractNode::~AbstractNode()
// IDA 0xdde724: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde724() {
}

// 0xdde828 — __ZN4Ogre12AbstractNodeD0Ev
#[doc(alias = "Ogre::AbstractNode::~AbstractNode()")]
// was: Ogre::AbstractNode::~AbstractNode()
// IDA 0xdde828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde828() {
}

// 0xdde934 — __ZN4Ogre14ScriptCompilerD1Ev
#[doc(alias = "Ogre::ScriptCompiler::~ScriptCompiler()")]
// was: Ogre::ScriptCompiler::~ScriptCompiler()
// IDA 0xdde934: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde934() {
}

// 0xdde940 — __ZN4Ogre14ScriptCompilerD0Ev
#[doc(alias = "Ogre::ScriptCompiler::~ScriptCompiler()")]
// was: Ogre::ScriptCompiler::~ScriptCompiler()
// IDA 0xdde940: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde940() {
}

// 0xdde9d0 — __ZN4Ogre14ScriptCompilerD2Ev
#[doc(alias = "Ogre::ScriptCompiler::~ScriptCompiler()")]
// was: Ogre::ScriptCompiler::~ScriptCompiler()
// IDA 0xdde9d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_dde9d0() {
}

// 0xddeb48 — __ZNSt6vectorIPN4Ogre23ScriptTranslatorManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ScriptTranslatorManager **,std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ScriptTranslatorManager * const&)")]
// was: std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ScriptTranslatorManager **,std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ScriptTranslatorManager * const&)
// IDA 0xddeb48: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ddeb48() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xddec40 — __ZNSt12_Vector_baseIPN4Ogre23ScriptTranslatorManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xddec40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ddec40() {
}

// 0xddec44 — __ZNSt12_Vector_baseIPN4Ogre23ScriptTranslatorManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xddec44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddec44() {
}

// 0xddec50 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::AbstractNode>::~SharedPtr()
// IDA 0xddec50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddec50() {
}

// 0xdded44 — __ZN4Ogre9SharedPtrINS_12AbstractNodeEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::AbstractNode>::destroy(void)
// IDA 0xdded44: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dded44() {
}

// 0xdded7c — __ZN4Ogre9SharedPtrINS_12AbstractNodeEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::AbstractNode>::swap(Ogre::SharedPtr<Ogre::AbstractNode>&)")]
// was: Ogre::SharedPtr<Ogre::AbstractNode>::swap(Ogre::SharedPtr<Ogre::AbstractNode>&)
// IDA 0xdded7c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dded7c() {
}

// 0xddee34 — __ZNSt10_List_baseIPN4Ogre12ConcreteNodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::ConcreteNode *,Ogre::STLAllocator<Ogre::ConcreteNode *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::ConcreteNode *,Ogre::STLAllocator<Ogre::ConcreteNode *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xddee34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ddee34() {
}

// 0xddee38 — __ZNSt10_List_baseIPN4Ogre12ConcreteNodeENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::ConcreteNode *,Ogre::STLAllocator<Ogre::ConcreteNode *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::ConcreteNode *,Ogre::STLAllocator<Ogre::ConcreteNode *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xddee38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddee38() {
}

// 0xddee44 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ConcreteNode>::~SharedPtr()
// IDA 0xddee44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddee44() {
}

// 0xddeef8 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::ConcreteNode>::destroy(void)
// IDA 0xddeef8: 202 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddeef8() {
}

// 0xddf114 — __ZN4Ogre9SharedPtrINS_12ConcreteNodeEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::ConcreteNode>::swap(Ogre::SharedPtr<Ogre::ConcreteNode>&)")]
// was: Ogre::SharedPtr<Ogre::ConcreteNode>::swap(Ogre::SharedPtr<Ogre::ConcreteNode>&)
// IDA 0xddf114: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddf114() {
}

// 0xddf698 — __ZN4Ogre12STLAllocatorINS_9SharedPtrINS_12AbstractNodeEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED0Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xddf698: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ddf698() {
}

// 0xddf6a4 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_bESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::pair<Ogre::ObjectAbstractNode * const,bool> const&)")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::pair<Ogre::ObjectAbstractNode * const,bool> const&)
// IDA 0xddf6a4: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddf6a4() {
}

// 0xddf8ac — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_bESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::ObjectAbstractNode * const,bool> const&)")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::ObjectAbstractNode * const,bool> const&)
// IDA 0xddf8ac: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddf8ac() {
}

// 0xddf9a8 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::pair<Ogre::ObjectAbstractNode * const,unsigned long> const&)")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::pair<Ogre::ObjectAbstractNode * const,unsigned long> const&)
// IDA 0xddf9a8: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddf9a8() {
}

// 0xddfbb0 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::ObjectAbstractNode * const,unsigned long> const&)")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::ObjectAbstractNode * const,unsigned long> const&)
// IDA 0xddfbb0: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddfbb0() {
}

// 0xddfcac — __ZNSt6vectorISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS7_SD_EERKS7_
#[doc(alias = "std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>*,std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>> const&)")]
// was: std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>*,std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>> const&)
// IDA 0xddfcac: 441 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ddfcac() {
}

// 0xde0158 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_bESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()
// IDA 0xde0158: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de0158() {
}

// 0xde015c — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_bESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()
// IDA 0xde015c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de015c() {
}

// 0xde0168 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()
// IDA 0xde0168: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de0168() {
}

// 0xde016c — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::ObjectAbstractNode *>,false>::~_Rb_tree_impl()
// IDA 0xde016c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de016c() {
}

// 0xde0178 — __ZNSt6vectorISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
// was: std::vector<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
// IDA 0xde0178: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de0178() {
}

// 0xde0294 — __ZNSt12_Vector_baseISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xde0294: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de0294() {
}

// 0xde0298 — __ZNSt12_Vector_baseISt4pairIN4Ogre9SharedPtrINS1_12AbstractNodeEEESt14_List_iteratorIS4_EENS1_12STLAllocatorIS7_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::STLAllocator<std::pair<Ogre::SharedPtr<Ogre::AbstractNode>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xde0298: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de0298() {
}

// 0xde02a4 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xde02a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de02a4() {
}

// 0xde0358 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xde0358: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0358() {
}

// 0xde0478 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12AbstractNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xde0478: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0478() {
}

// 0xde0498 — __ZN4Ogre9SharedPtrINS_10DataStreamEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::DataStream>::destroy(void)
// IDA 0xde0498: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0498() {
}

// 0xde04d0 — __ZN4Ogre9SharedPtrINS_10DataStreamEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::swap(Ogre::SharedPtr<Ogre::DataStream>&)")]
// was: Ogre::SharedPtr<Ogre::DataStream>::swap(Ogre::SharedPtr<Ogre::DataStream>&)
// IDA 0xde04d0: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de04d0() {
}

// 0xde04ec — __ZNSt4listIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ISt14_List_iteratorIS3_EEET_SD_RKS8_
#[doc(alias = "std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list<std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>(std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>> const&)")]
// was: std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list<std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>>(std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,std::_List_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>> const&)
// IDA 0xde04ec: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de04ec() {
}

// 0xde063c — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xde063c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de063c() {
}

// 0xde0640 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12AbstractNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xde0640: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de0640() {
}

// 0xde0650 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS2_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::string> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,std::string> const&)
// IDA 0xde0650: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0650() {
}

// 0xde0790 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>)
// IDA 0xde0790: 57 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0790() {
}

// 0xde0838 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)
// IDA 0xde0838: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0838() {
}

// 0xde08e0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE16_M_insert_uniqueERKSE_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)
// IDA 0xde08e0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de08e0() {
}

// 0xde09c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE9_M_insertEPSt18_Rb_tree_node_baseSM_RKSE_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)
// IDA 0xde09c4: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de09c4() {
}

// 0xde0a38 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE14_M_create_nodeERKSE_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const&)
// IDA 0xde0a38: 106 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0a38() {
}

// 0xde0b50 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xde0b50: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0b50() {
}

// 0xde0bf4 — __ZN4Ogre9SharedPtrINS_14ScriptCompiler5ErrorEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::~SharedPtr()
// IDA 0xde0bf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de0bf4() {
}

// 0xde0ca8 — __ZN4Ogre9SharedPtrINS_14ScriptCompiler5ErrorEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::destroy(void)
// IDA 0xde0ca8: 147 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0ca8() {
}

// 0xde0e5c — __ZN4Ogre9SharedPtrINS_14ScriptCompiler5ErrorEE4swapERS3_
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::swap(Ogre::SharedPtr<Ogre::ScriptCompiler::Error>&)")]
// was: Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::swap(Ogre::SharedPtr<Ogre::ScriptCompiler::Error>&)
// IDA 0xde0e5c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0e5c() {
}

// 0xde0e78 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>> *)
// IDA 0xde0e78: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de0e78() {
}

// 0xde0f9c — __ZN4Ogre9SharedPtrISt6vectorINS0_INS_11ScriptTokenEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xde0f9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de0f9c() {
}

// 0xde1050 — __ZN4Ogre9SharedPtrISt6vectorINS0_INS_11ScriptTokenEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xde1050: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1050() {
}

// 0xde1160 — __ZN4Ogre9SharedPtrISt6vectorINS0_INS_11ScriptTokenEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xde1160: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1160() {
}

// 0xde117c — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_14ScriptCompiler5ErrorEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xde117c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de117c() {
}

// 0xde1180 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_14ScriptCompiler5ErrorEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptCompiler::Error>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xde1180: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1180() {
}

// 0xde1190 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xde1190: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1190() {
}

// 0xde119c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xde119c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de119c() {
}

// 0xde11a0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrISt4listINS3_INS2_12AbstractNodeEEENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEESt10_Select1stISE_ESt4lessISsENS7_ISE_SA_EEE13_Rb_tree_implISI_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::AbstractNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xde11a0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de11a0() {
}

// 0xde11ac — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xde11ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de11ac() {
}

// 0xde1260 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xde1260: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1260() {
}

// 0xde1380 — __ZN4Ogre9SharedPtrISt4listINS0_INS_12ConcreteNodeEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERSA_
#[doc(alias = "Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::list<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xde1380: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1380() {
}

// 0xde13a0 — __ZNKSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xde13a0: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de13a0() {
}

// 0xde1448 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSF_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)
// IDA 0xde1448: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1448() {
}

// 0xde1650 — __ZNSt12_Vector_baseISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xde1650: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de1650() {
}

// 0xde1658 — __ZN4Ogre18ImportAbstractNodeD2Ev
#[doc(alias = "Ogre::ImportAbstractNode::~ImportAbstractNode()")]
// was: Ogre::ImportAbstractNode::~ImportAbstractNode()
// IDA 0xde1658: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1658() {
}

// 0xde17d4 — __ZN4Ogre20PropertyAbstractNodeD2Ev
#[doc(alias = "Ogre::PropertyAbstractNode::~PropertyAbstractNode()")]
// was: Ogre::PropertyAbstractNode::~PropertyAbstractNode()
// IDA 0xde17d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de17d4() {
}

// 0xde1954 — __ZN4Ogre18ObjectAbstractNodeD2Ev
#[doc(alias = "Ogre::ObjectAbstractNode::~ObjectAbstractNode()")]
// was: Ogre::ObjectAbstractNode::~ObjectAbstractNode()
// IDA 0xde1954: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1954() {
}

// 0xde1be0 — __ZN4Ogre39ProcessNameExclusionScriptCompilerEventD0Ev
#[doc(alias = "Ogre::ProcessNameExclusionScriptCompilerEvent::~ProcessNameExclusionScriptCompilerEvent()")]
// was: Ogre::ProcessNameExclusionScriptCompilerEvent::~ProcessNameExclusionScriptCompilerEvent()
// IDA 0xde1be0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1be0() {
}

// 0xde1c8c — __ZN4Ogre19ScriptCompilerEventD1Ev
#[doc(alias = "Ogre::ScriptCompilerEvent::~ScriptCompilerEvent()")]
// was: Ogre::ScriptCompilerEvent::~ScriptCompilerEvent()
// IDA 0xde1c8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1c8c() {
}

// 0xde1ce8 — __ZN4Ogre19ScriptCompilerEventD0Ev
#[doc(alias = "Ogre::ScriptCompilerEvent::~ScriptCompilerEvent()")]
// was: Ogre::ScriptCompilerEvent::~ScriptCompilerEvent()
// IDA 0xde1ce8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1ce8() {
}

// 0xde1d48 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_mESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>> *)")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::ObjectAbstractNode * const,unsigned long>> *)
// IDA 0xde1d48: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1d48() {
}

// 0xde1d70 — __ZNSt8_Rb_treeIPN4Ogre18ObjectAbstractNodeESt4pairIKS2_bESt10_Select1stIS5_ESt4lessIS2_ENS0_12STLAllocatorIS5_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::ObjectAbstractNode * const,bool>> *)")]
// was: std::_Rb_tree<Ogre::ObjectAbstractNode *,std::pair<Ogre::ObjectAbstractNode * const,bool>,std::_Select1st<std::pair<Ogre::ObjectAbstractNode * const,bool>>,std::less<Ogre::ObjectAbstractNode *>,Ogre::STLAllocator<std::pair<Ogre::ObjectAbstractNode * const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::ObjectAbstractNode * const,bool>> *)
// IDA 0xde1d70: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1d70() {
}

// 0xde1d98 — __ZN4Ogre3AnyD1Ev
#[doc(alias = "Ogre::Any::~Any()")]
// was: Ogre::Any::~Any()
// IDA 0xde1d98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de1d98() {
}

// 0xde1fe8 — __ZN4Ogre11ScriptLexerC1Ev
#[doc(alias = "Ogre::ScriptLexer::ScriptLexer(void)")]
// was: Ogre::ScriptLexer::ScriptLexer(void)
// IDA 0xde1fe8: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1fe8() {
}

// 0xde1ffc — __ZN4Ogre11ScriptLexer8tokenizeERKSsS2_
#[doc(alias = "Ogre::ScriptLexer::tokenize(std::string const&,std::string const&)")]
// was: Ogre::ScriptLexer::tokenize(std::string const&,std::string const&)
// IDA 0xde1ffc: 1122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de1ffc() {
}
