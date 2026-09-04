//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xde2b7c..0xe1b0d4 (100 stubs, 8960 prior -> 9060 covered, 4273 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xde2b7c — __ZN4Ogre11ScriptLexer8setTokenERKSsjS2_PSt6vectorINS_9SharedPtrINS_11ScriptTokenEEENS_12STLAllocatorIS6_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::ScriptLexer::setToken(std::string const&,unsigned int,std::string const&,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *)")]
// was: Ogre::ScriptLexer::setToken(std::string const&,unsigned int,std::string const&,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *)
// IDA 0xde2b7c: 251 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de2b7c() {
}

// 0xde2dec — __ZN4Ogre9SharedPtrINS_11ScriptTokenEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::~SharedPtr()
// IDA 0xde2dec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de2dec() {
}

// 0xde2e9c — __ZN4Ogre11ScriptLexerD1Ev
#[doc(alias = "Ogre::ScriptLexer::~ScriptLexer()")]
// was: Ogre::ScriptLexer::~ScriptLexer()
// IDA 0xde2e9c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de2e9c() {
}

// 0xde2ea0 — __ZN4Ogre11ScriptLexerD0Ev
#[doc(alias = "Ogre::ScriptLexer::~ScriptLexer()")]
// was: Ogre::ScriptLexer::~ScriptLexer()
// IDA 0xde2ea0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de2ea0() {
}

// 0xde2f2c — __ZNSt6vectorIN4Ogre9SharedPtrINS0_11ScriptTokenEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)")]
// was: std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)
// IDA 0xde2f2c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_de2f2c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xde3254 — __ZN4Ogre9SharedPtrINS_11ScriptTokenEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)
// IDA 0xde3254: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de3254() {
}

// 0xde3360 — __ZN4Ogre9SharedPtrINS_11ScriptTokenEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::~SharedPtr()
// IDA 0xde3360: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de3360() {
}

// 0xde3414 — __ZN4Ogre9SharedPtrINS_11ScriptTokenEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)
// IDA 0xde3414: 103 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de3414() {
}

// 0xde3540 — __ZN4Ogre9SharedPtrINS_11ScriptTokenEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::swap(Ogre::SharedPtr<Ogre::ScriptToken>&)")]
// was: Ogre::SharedPtr<Ogre::ScriptToken>::swap(Ogre::SharedPtr<Ogre::ScriptToken>&)
// IDA 0xde3540: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de3540() {
}

// 0xde355c — __ZNSt12_Vector_baseIN4Ogre9SharedPtrINS0_11ScriptTokenEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xde355c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de355c() {
}

// 0xde3560 — __ZNSt12_Vector_baseIN4Ogre9SharedPtrINS0_11ScriptTokenEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xde3560: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de3560() {
}

// 0xde35a0 — __ZN4Ogre12ScriptParserC1Ev
#[doc(alias = "Ogre::ScriptParser::ScriptParser(void)")]
// was: Ogre::ScriptParser::ScriptParser(void)
// IDA 0xde35a0: 6 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de35a0() {
}

// 0xde35b4 — __ZN4Ogre12ScriptParser5parseERKNS_9SharedPtrISt6vectorINS1_INS_11ScriptTokenEEENS_12STLAllocatorIS4_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEE
#[doc(alias = "Ogre::ScriptParser::parse(Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: Ogre::ScriptParser::parse(Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xde35b4: 4684 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de35b4() {
}

// 0xde6628 — __ZN4Ogre12ScriptParser10parseChunkERKNS_9SharedPtrISt6vectorINS1_INS_11ScriptTokenEEENS_12STLAllocatorIS4_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEEE
#[doc(alias = "Ogre::ScriptParser::parseChunk(Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: Ogre::ScriptParser::parseChunk(Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xde6628: 948 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de6628() {
}

// 0xde7010 — __ZN4Ogre12ScriptParserD1Ev
#[doc(alias = "Ogre::ScriptParser::~ScriptParser()")]
// was: Ogre::ScriptParser::~ScriptParser()
// IDA 0xde7010: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de7010() {
}

// 0xde7014 — __ZN4Ogre12ScriptParserD0Ev
#[doc(alias = "Ogre::ScriptParser::~ScriptParser()")]
// was: Ogre::ScriptParser::~ScriptParser()
// IDA 0xde7014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de7014() {
}

// 0xde70a0 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12ConcreteNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xde70a0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_de70a0() {
}

// 0xde70a4 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_12ConcreteNodeEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ConcreteNode>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xde70a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_de70a4() {
}

// 0xde70e4 — __ZN4Ogre16ScriptTranslator11processNodeEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::ScriptTranslator::processNode(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::ScriptTranslator::processNode(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xde70e4: 234 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de70e4() {
}

// 0xde7364 — __ZN4Ogre16ScriptTranslator15getConstantTypeESt20_List_const_iteratorINS_9SharedPtrINS_12AbstractNodeEEEEPNS_15GpuConstantTypeE
#[doc(alias = "Ogre::ScriptTranslator::getConstantType(std::_List_const_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,Ogre::GpuConstantType *)")]
// was: Ogre::ScriptTranslator::getConstantType(std::_List_const_iterator<Ogre::SharedPtr<Ogre::AbstractNode>>,Ogre::GpuConstantType *)
// IDA 0xde7364: 299 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de7364() {
}

// 0xde77d0 — __ZN4Ogre18MaterialTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::MaterialTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::MaterialTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xde77d0: 2697 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de77d0() {
}

// 0xde94f8 — __ZN4Ogre19TechniqueTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::TechniqueTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::TechniqueTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xde94f8: 4087 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_de94f8() {
}

// 0xdec274 — __ZN4Ogre14PassTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::PassTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::PassTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xdec274: 5000 insns (PUSH..ADD). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dec274() {
}

// 0xdf9fe8 — __ZN4Ogre14PassTranslator27translateFragmentProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateFragmentProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateFragmentProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdf9fe8: 521 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_df9fe8() {
}

// 0xdfa56c — __ZN4Ogre14PassTranslator25translateVertexProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateVertexProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateVertexProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdfa56c: 521 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfa56c() {
}

// 0xdfaaf0 — __ZN4Ogre14PassTranslator27translateGeometryProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateGeometryProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateGeometryProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdfaaf0: 521 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfaaf0() {
}

// 0xdfb074 — __ZN4Ogre14PassTranslator37translateShadowCasterVertexProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateShadowCasterVertexProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateShadowCasterVertexProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdfb074: 520 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfb074() {
}

// 0xdfb5f4 — __ZN4Ogre14PassTranslator39translateShadowCasterFragmentProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateShadowCasterFragmentProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateShadowCasterFragmentProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdfb5f4: 520 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfb5f4() {
}

// 0xdfbb74 — __ZN4Ogre14PassTranslator39translateShadowReceiverVertexProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateShadowReceiverVertexProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateShadowReceiverVertexProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdfbb74: 520 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfbb74() {
}

// 0xdfc0f4 — __ZN4Ogre14PassTranslator41translateShadowReceiverFragmentProgramRefEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::PassTranslator::translateShadowReceiverFragmentProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::PassTranslator::translateShadowReceiverFragmentProgramRef(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xdfc0f4: 520 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfc0f4() {
}

// 0xdfc674 — __ZN4Ogre20GpuProgramTranslator26translateProgramParametersEPNS_14ScriptCompilerENS_9SharedPtrINS_20GpuProgramParametersEEEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::GpuProgramTranslator::translateProgramParameters(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::GpuProgramParameters>,Ogre::ObjectAbstractNode *)")]
// was: Ogre::GpuProgramTranslator::translateProgramParameters(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::GpuProgramParameters>,Ogre::ObjectAbstractNode *)
// IDA 0xdfc674: 3143 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfc674() {
}

// 0xdfe7a0 — __ZN4Ogre21TextureUnitTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::TextureUnitTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::TextureUnitTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xdfe7a0: 5000 insns (PUSH..BEQ). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dfe7a0() {
}

// 0xe0a450 — __ZN4Ogre23TextureSourceTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::TextureSourceTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::TextureSourceTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe0a450: 1162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0a450() {
}

// 0xe0b134 — __ZN4Ogre20GpuProgramTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::GpuProgramTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::GpuProgramTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe0b134: 339 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0b134() {
}

// 0xe0b4ec — __ZN4Ogre20GpuProgramTranslator19translateGpuProgramEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::GpuProgramTranslator::translateGpuProgram(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::GpuProgramTranslator::translateGpuProgram(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xe0b4ec: 1620 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0b4ec() {
}

// 0xe0c5c4 — __ZN4Ogre20GpuProgramTranslator26translateUnifiedGpuProgramEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::GpuProgramTranslator::translateUnifiedGpuProgram(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::GpuProgramTranslator::translateUnifiedGpuProgram(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xe0c5c4: 1693 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0c5c4() {
}

// 0xe0d7dc — __ZN4Ogre20GpuProgramTranslator28translateHighLevelGpuProgramEPNS_14ScriptCompilerEPNS_18ObjectAbstractNodeE
#[doc(alias = "Ogre::GpuProgramTranslator::translateHighLevelGpuProgram(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)")]
// was: Ogre::GpuProgramTranslator::translateHighLevelGpuProgram(Ogre::ScriptCompiler *,Ogre::ObjectAbstractNode *)
// IDA 0xe0d7dc: 1643 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0d7dc() {
}

// 0xe0e954 — __ZN4Ogre22SharedParamsTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::SharedParamsTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::SharedParamsTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe0e954: 1333 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0e954() {
}

// 0xe0f7b4 — __ZN4Ogre24ParticleSystemTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::ParticleSystemTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::ParticleSystemTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe0f7b4: 1342 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e0f7b4() {
}

// 0xe10a40 — __ZN4Ogre25ParticleEmitterTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::ParticleEmitterTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::ParticleEmitterTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe10a40: 636 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e10a40() {
}

// 0xe11134 — __ZN4Ogre26ParticleAffectorTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::ParticleAffectorTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::ParticleAffectorTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe11134: 636 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e11134() {
}

// 0xe11828 — __ZN4Ogre20CompositorTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::CompositorTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::CompositorTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe11828: 644 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e11828() {
}

// 0xe11ef0 — __ZN4Ogre30CompositionTechniqueTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::CompositionTechniqueTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::CompositionTechniqueTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe11ef0: 2423 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e11ef0() {
}

// 0xe138bc — __ZN4Ogre31CompositionTargetPassTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::CompositionTargetPassTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::CompositionTargetPassTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe138bc: 1877 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e138bc() {
}

// 0xe14d44 — __ZN4Ogre25CompositionPassTranslator9translateEPNS_14ScriptCompilerERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::CompositionPassTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::CompositionPassTranslator::translate(Ogre::ScriptCompiler *,Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe14d44: 4772 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e14d44() {
}

// 0xe18500 — __ZN4Ogre30BuiltinScriptTranslatorManagerC1Ev
#[doc(alias = "Ogre::BuiltinScriptTranslatorManager::BuiltinScriptTranslatorManager(void)")]
// was: Ogre::BuiltinScriptTranslatorManager::BuiltinScriptTranslatorManager(void)
// IDA 0xe18500: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e18500() {
}

// 0xe1862c — __ZNK4Ogre30BuiltinScriptTranslatorManager17getNumTranslatorsEv
#[doc(alias = "Ogre::BuiltinScriptTranslatorManager::getNumTranslators(void)const")]
// was: Ogre::BuiltinScriptTranslatorManager::getNumTranslators(void)const
// IDA 0xe1862c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1862c() {
}

// 0xe18630 — __ZN4Ogre30BuiltinScriptTranslatorManager13getTranslatorERKNS_9SharedPtrINS_12AbstractNodeEEE
#[doc(alias = "Ogre::BuiltinScriptTranslatorManager::getTranslator(Ogre::SharedPtr<Ogre::AbstractNode> const&)")]
// was: Ogre::BuiltinScriptTranslatorManager::getTranslator(Ogre::SharedPtr<Ogre::AbstractNode> const&)
// IDA 0xe18630: 132 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e18630() {
}

// 0xe1878c — __ZN4Ogre41PreApplyTextureAliasesScriptCompilerEventD1Ev
#[doc(alias = "Ogre::PreApplyTextureAliasesScriptCompilerEvent::~PreApplyTextureAliasesScriptCompilerEvent()")]
// was: Ogre::PreApplyTextureAliasesScriptCompilerEvent::~PreApplyTextureAliasesScriptCompilerEvent()
// IDA 0xe1878c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1878c() {
}

// 0xe187e8 — __ZN4Ogre33CreateMaterialScriptCompilerEventD1Ev
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")]
// was: Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()
// IDA 0xe187e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e187e8() {
}

// 0xe187f4 — __ZN4Ogre8any_castIPNS_8MaterialEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")]
// was: Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)
// IDA 0xe187f4: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e187f4() {
}

// 0xe18b8c — __ZN4Ogre38ProcessResourceNameScriptCompilerEventD1Ev
#[doc(alias = "Ogre::ProcessResourceNameScriptCompilerEvent::~ProcessResourceNameScriptCompilerEvent()")]
// was: Ogre::ProcessResourceNameScriptCompilerEvent::~ProcessResourceNameScriptCompilerEvent()
// IDA 0xe18b8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e18b8c() {
}

// 0xe18c34 — __ZN4Ogre8any_castIPNS_9TechniqueEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)")]
// was: Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)
// IDA 0xe18c34: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e18c34() {
}

// 0xe18fcc — __ZN4Ogre8any_castIPNS_4PassEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)")]
// was: Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)
// IDA 0xe18fcc: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e18fcc() {
}

// 0xe19364 — __ZN4Ogre8any_castIPNS_16TextureUnitStateEEET_RKNS_3AnyE
#[doc(alias = "Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)")]
// was: Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)
// IDA 0xe19364: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e19364() {
}

// 0xe196fc — __ZN4Ogre35CreateGpuProgramScriptCompilerEventD1Ev
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()
// IDA 0xe196fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e196fc() {
}

// 0xe19708 — __ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventD1Ev
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")]
// was: Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()
// IDA 0xe19708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e19708() {
}

// 0xe19714 — __ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventD1Ev
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")]
// was: Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()
// IDA 0xe19714: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e19714() {
}

// 0xe19720 — __ZN4Ogre39CreateParticleSystemScriptCompilerEventD1Ev
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")]
// was: Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()
// IDA 0xe19720: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e19720() {
}

// 0xe1972c — __ZN4Ogre8any_castIPNS_14ParticleSystemEEET_RKNS_3AnyE
#[doc(alias = "Ogre::ParticleSystem * Ogre::any_cast<Ogre::ParticleSystem *>(Ogre::Any const&)")]
// was: Ogre::ParticleSystem * Ogre::any_cast<Ogre::ParticleSystem *>(Ogre::Any const&)
// IDA 0xe1972c: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1972c() {
}

// 0xe19ac4 — __ZN4Ogre35CreateCompositorScriptCompilerEventD1Ev
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")]
// was: Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()
// IDA 0xe19ac4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e19ac4() {
}

// 0xe19ad0 — __ZN4Ogre8any_castIPNS_10CompositorEEET_RKNS_3AnyE
#[doc(alias = "Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)")]
// was: Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)
// IDA 0xe19ad0: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e19ad0() {
}

// 0xe19e68 — __ZNSt6vectorIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe19e68: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e19e68() {
}

// 0xe19f40 — __ZN4Ogre8any_castIPNS_20CompositionTechniqueEEET_RKNS_3AnyE
#[doc(alias = "Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)")]
// was: Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)
// IDA 0xe19f40: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e19f40() {
}

// 0xe1a2d8 — __ZN4Ogre8any_castIPNS_21CompositionTargetPassEEET_RKNS_3AnyE
#[doc(alias = "Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)")]
// was: Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)
// IDA 0xe1a2d8: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1a2d8() {
}

// 0xe1a670 — __ZN4Ogre31CompositionTargetPassTranslatorD1Ev
#[doc(alias = "Ogre::CompositionTargetPassTranslator::~CompositionTargetPassTranslator()")]
// was: Ogre::CompositionTargetPassTranslator::~CompositionTargetPassTranslator()
// IDA 0xe1a670: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a670() {
}

// 0xe1a674 — __ZN4Ogre30CompositionTechniqueTranslatorD1Ev
#[doc(alias = "Ogre::CompositionTechniqueTranslator::~CompositionTechniqueTranslator()")]
// was: Ogre::CompositionTechniqueTranslator::~CompositionTechniqueTranslator()
// IDA 0xe1a674: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a674() {
}

// 0xe1a678 — __ZN4Ogre20CompositorTranslatorD1Ev
#[doc(alias = "Ogre::CompositorTranslator::~CompositorTranslator()")]
// was: Ogre::CompositorTranslator::~CompositorTranslator()
// IDA 0xe1a678: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a678() {
}

// 0xe1a67c — __ZN4Ogre26ParticleAffectorTranslatorD1Ev
#[doc(alias = "Ogre::ParticleAffectorTranslator::~ParticleAffectorTranslator()")]
// was: Ogre::ParticleAffectorTranslator::~ParticleAffectorTranslator()
// IDA 0xe1a67c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a67c() {
}

// 0xe1a680 — __ZN4Ogre25ParticleEmitterTranslatorD1Ev
#[doc(alias = "Ogre::ParticleEmitterTranslator::~ParticleEmitterTranslator()")]
// was: Ogre::ParticleEmitterTranslator::~ParticleEmitterTranslator()
// IDA 0xe1a680: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a680() {
}

// 0xe1a684 — __ZN4Ogre24ParticleSystemTranslatorD1Ev
#[doc(alias = "Ogre::ParticleSystemTranslator::~ParticleSystemTranslator()")]
// was: Ogre::ParticleSystemTranslator::~ParticleSystemTranslator()
// IDA 0xe1a684: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a684() {
}

// 0xe1a688 — __ZN4Ogre22SharedParamsTranslatorD1Ev
#[doc(alias = "Ogre::SharedParamsTranslator::~SharedParamsTranslator()")]
// was: Ogre::SharedParamsTranslator::~SharedParamsTranslator()
// IDA 0xe1a688: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a688() {
}

// 0xe1a68c — __ZN4Ogre20GpuProgramTranslatorD1Ev
#[doc(alias = "Ogre::GpuProgramTranslator::~GpuProgramTranslator()")]
// was: Ogre::GpuProgramTranslator::~GpuProgramTranslator()
// IDA 0xe1a68c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a68c() {
}

// 0xe1a690 — __ZN4Ogre23TextureSourceTranslatorD1Ev
#[doc(alias = "Ogre::TextureSourceTranslator::~TextureSourceTranslator()")]
// was: Ogre::TextureSourceTranslator::~TextureSourceTranslator()
// IDA 0xe1a690: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a690() {
}

// 0xe1a694 — __ZN4Ogre21TextureUnitTranslatorD1Ev
#[doc(alias = "Ogre::TextureUnitTranslator::~TextureUnitTranslator()")]
// was: Ogre::TextureUnitTranslator::~TextureUnitTranslator()
// IDA 0xe1a694: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a694() {
}

// 0xe1a698 — __ZN4Ogre14PassTranslatorD1Ev
#[doc(alias = "Ogre::PassTranslator::~PassTranslator()")]
// was: Ogre::PassTranslator::~PassTranslator()
// IDA 0xe1a698: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a698() {
}

// 0xe1a69c — __ZN4Ogre19TechniqueTranslatorD1Ev
#[doc(alias = "Ogre::TechniqueTranslator::~TechniqueTranslator()")]
// was: Ogre::TechniqueTranslator::~TechniqueTranslator()
// IDA 0xe1a69c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1a69c() {
}

// 0xe1a6a0 — __ZN4Ogre18MaterialTranslatorD1Ev
#[doc(alias = "Ogre::MaterialTranslator::~MaterialTranslator()")]
// was: Ogre::MaterialTranslator::~MaterialTranslator()
// IDA 0xe1a6a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a6a0() {
}

// 0xe1a744 — __ZN4Ogre18MaterialTranslatorD0Ev
#[doc(alias = "Ogre::MaterialTranslator::~MaterialTranslator()")]
// was: Ogre::MaterialTranslator::~MaterialTranslator()
// IDA 0xe1a744: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a744() {
}

// 0xe1a7f4 — __ZN4Ogre19TechniqueTranslatorD0Ev
#[doc(alias = "Ogre::TechniqueTranslator::~TechniqueTranslator()")]
// was: Ogre::TechniqueTranslator::~TechniqueTranslator()
// IDA 0xe1a7f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a7f4() {
}

// 0xe1a880 — __ZN4Ogre14PassTranslatorD0Ev
#[doc(alias = "Ogre::PassTranslator::~PassTranslator()")]
// was: Ogre::PassTranslator::~PassTranslator()
// IDA 0xe1a880: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a880() {
}

// 0xe1a90c — __ZN4Ogre21TextureUnitTranslatorD0Ev
#[doc(alias = "Ogre::TextureUnitTranslator::~TextureUnitTranslator()")]
// was: Ogre::TextureUnitTranslator::~TextureUnitTranslator()
// IDA 0xe1a90c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a90c() {
}

// 0xe1a998 — __ZN4Ogre23TextureSourceTranslatorD0Ev
#[doc(alias = "Ogre::TextureSourceTranslator::~TextureSourceTranslator()")]
// was: Ogre::TextureSourceTranslator::~TextureSourceTranslator()
// IDA 0xe1a998: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1a998() {
}

// 0xe1aa24 — __ZN4Ogre20GpuProgramTranslatorD0Ev
#[doc(alias = "Ogre::GpuProgramTranslator::~GpuProgramTranslator()")]
// was: Ogre::GpuProgramTranslator::~GpuProgramTranslator()
// IDA 0xe1aa24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1aa24() {
}

// 0xe1aab0 — __ZN4Ogre22SharedParamsTranslatorD0Ev
#[doc(alias = "Ogre::SharedParamsTranslator::~SharedParamsTranslator()")]
// was: Ogre::SharedParamsTranslator::~SharedParamsTranslator()
// IDA 0xe1aab0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1aab0() {
}

// 0xe1ab3c — __ZN4Ogre24ParticleSystemTranslatorD0Ev
#[doc(alias = "Ogre::ParticleSystemTranslator::~ParticleSystemTranslator()")]
// was: Ogre::ParticleSystemTranslator::~ParticleSystemTranslator()
// IDA 0xe1ab3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1ab3c() {
}

// 0xe1abc8 — __ZN4Ogre25ParticleEmitterTranslatorD0Ev
#[doc(alias = "Ogre::ParticleEmitterTranslator::~ParticleEmitterTranslator()")]
// was: Ogre::ParticleEmitterTranslator::~ParticleEmitterTranslator()
// IDA 0xe1abc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1abc8() {
}

// 0xe1ac54 — __ZN4Ogre26ParticleAffectorTranslatorD0Ev
#[doc(alias = "Ogre::ParticleAffectorTranslator::~ParticleAffectorTranslator()")]
// was: Ogre::ParticleAffectorTranslator::~ParticleAffectorTranslator()
// IDA 0xe1ac54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1ac54() {
}

// 0xe1ace0 — __ZN4Ogre20CompositorTranslatorD0Ev
#[doc(alias = "Ogre::CompositorTranslator::~CompositorTranslator()")]
// was: Ogre::CompositorTranslator::~CompositorTranslator()
// IDA 0xe1ace0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1ace0() {
}

// 0xe1ad6c — __ZN4Ogre30CompositionTechniqueTranslatorD0Ev
#[doc(alias = "Ogre::CompositionTechniqueTranslator::~CompositionTechniqueTranslator()")]
// was: Ogre::CompositionTechniqueTranslator::~CompositionTechniqueTranslator()
// IDA 0xe1ad6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1ad6c() {
}

// 0xe1adf8 — __ZN4Ogre31CompositionTargetPassTranslatorD0Ev
#[doc(alias = "Ogre::CompositionTargetPassTranslator::~CompositionTargetPassTranslator()")]
// was: Ogre::CompositionTargetPassTranslator::~CompositionTargetPassTranslator()
// IDA 0xe1adf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1adf8() {
}

// 0xe1ae84 — __ZN4Ogre25CompositionPassTranslatorD1Ev
#[doc(alias = "Ogre::CompositionPassTranslator::~CompositionPassTranslator()")]
// was: Ogre::CompositionPassTranslator::~CompositionPassTranslator()
// IDA 0xe1ae84: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1ae84() {
}

// 0xe1ae88 — __ZN4Ogre25CompositionPassTranslatorD0Ev
#[doc(alias = "Ogre::CompositionPassTranslator::~CompositionPassTranslator()")]
// was: Ogre::CompositionPassTranslator::~CompositionPassTranslator()
// IDA 0xe1ae88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1ae88() {
}

// 0xe1af14 — __ZN4Ogre30BuiltinScriptTranslatorManagerD1Ev
#[doc(alias = "Ogre::BuiltinScriptTranslatorManager::~BuiltinScriptTranslatorManager()")]
// was: Ogre::BuiltinScriptTranslatorManager::~BuiltinScriptTranslatorManager()
// IDA 0xe1af14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1af14() {
}

// 0xe1afc8 — __ZN4Ogre30BuiltinScriptTranslatorManagerD0Ev
#[doc(alias = "Ogre::BuiltinScriptTranslatorManager::~BuiltinScriptTranslatorManager()")]
// was: Ogre::BuiltinScriptTranslatorManager::~BuiltinScriptTranslatorManager()
// IDA 0xe1afc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1afc8() {
}

// 0xe1b084 — __ZN4Ogre3Any6holderIPNS_15CompositionPassEED1Ev
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionPass *>::~holder()")]
// was: Ogre::Any::holder<Ogre::CompositionPass *>::~holder()
// IDA 0xe1b084: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e1b084() {
}

// 0xe1b088 — __ZN4Ogre3Any6holderIPNS_15CompositionPassEED0Ev
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionPass *>::~holder()")]
// was: Ogre::Any::holder<Ogre::CompositionPass *>::~holder()
// IDA 0xe1b088: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e1b088() {
}

// 0xe1b094 — __ZNK4Ogre3Any6holderIPNS_15CompositionPassEE7getTypeEv
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionPass *>::getType(void)const")]
// was: Ogre::Any::holder<Ogre::CompositionPass *>::getType(void)const
// IDA 0xe1b094: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b094() {
}

// 0xe1b0a4 — __ZNK4Ogre3Any6holderIPNS_15CompositionPassEE5cloneEv
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionPass *>::clone(void)const")]
// was: Ogre::Any::holder<Ogre::CompositionPass *>::clone(void)const
// IDA 0xe1b0a4: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b0a4() {
}

// 0xe1b0d4 — __ZN4Ogre3Any6holderIPNS_15CompositionPassEE13writeToStreamERSo
#[doc(alias = "Ogre::Any::holder<Ogre::CompositionPass *>::writeToStream(std::ostream &)")]
// was: Ogre::Any::holder<Ogre::CompositionPass *>::writeToStream(std::ostream &)
// IDA 0xe1b0d4: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e1b0d4() {
}
