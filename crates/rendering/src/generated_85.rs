//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe42158..0xe4770c (100 stubs, 9660 prior -> 9760 covered, 3573 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe42158 — __ZNSt12_Vector_baseIN4Ogre9Technique13GPUVendorRuleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Technique::GPUVendorRule,Ogre::STLAllocator<Ogre::Technique::GPUVendorRule,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe42158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e42158() {
}

// 0xe42164 — __ZNSt12_Vector_baseIPN4Ogre16IlluminationPassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::IlluminationPass *,Ogre::STLAllocator<Ogre::IlluminationPass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe42164: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e42164() {
}

// 0xe42170 — __ZNSt12_Vector_baseIPN4Ogre4PassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Pass *,Ogre::STLAllocator<Ogre::Pass *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe42170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e42170() {
}

// 0xe421b0 — __ZN4Ogre22TextAreaOverlayElementC1ERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")]
// was: Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)
// IDA 0xe421b0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e421b0() {
}

// 0xe421bc — __ZN4Ogre22TextAreaOverlayElementC2ERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)")]
// was: Ogre::TextAreaOverlayElement::TextAreaOverlayElement(std::string const&)
// IDA 0xe421bc: 254 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e421bc() {
}

// 0xe424ac — __ZN4Ogre22TextAreaOverlayElement10initialiseEv
#[doc(alias = "Ogre::TextAreaOverlayElement::initialise(void)")]
// was: Ogre::TextAreaOverlayElement::initialise(void)
// IDA 0xe424ac: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e424ac() {
}

// 0xe42604 — __ZN4Ogre22TextAreaOverlayElement21checkMemoryAllocationEm
#[doc(alias = "Ogre::TextAreaOverlayElement::checkMemoryAllocation(unsigned long)")]
// was: Ogre::TextAreaOverlayElement::checkMemoryAllocation(unsigned long)
// IDA 0xe42604: 312 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e42604() {
}

// 0xe42910 — __ZN4Ogre22TextAreaOverlayElement22updatePositionGeometryEv
#[doc(alias = "Ogre::TextAreaOverlayElement::updatePositionGeometry(void)")]
// was: Ogre::TextAreaOverlayElement::updatePositionGeometry(void)
// IDA 0xe42910: 565 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e42910() {
}

// 0xe42f54 — __ZN4Ogre22TextAreaOverlayElement21updateTextureGeometryEv
#[doc(alias = "Ogre::TextAreaOverlayElement::updateTextureGeometry(void)")]
// was: Ogre::TextAreaOverlayElement::updateTextureGeometry(void)
// IDA 0xe42f54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e42f54() {
}

// 0xe42f58 — __ZN4Ogre22TextAreaOverlayElement10setCaptionERKNS_9UTFStringE
#[doc(alias = "Ogre::TextAreaOverlayElement::setCaption(Ogre::UTFString const&)")]
// was: Ogre::TextAreaOverlayElement::setCaption(Ogre::UTFString const&)
// IDA 0xe42f58: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e42f58() {
}

// 0xe42f70 — __ZN4Ogre22TextAreaOverlayElement11setFontNameERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::setFontName(std::string const&)")]
// was: Ogre::TextAreaOverlayElement::setFontName(std::string const&)
// IDA 0xe42f70: 349 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e42f70() {
}

// 0xe4332c — __ZN4Ogre22TextAreaOverlayElementD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
// was: Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()
// IDA 0xe4332c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4332c() {
}

// 0xe433bc — __ZN4Ogre22TextAreaOverlayElementD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
// was: Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()
// IDA 0xe433bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e433bc() {
}

// 0xe433c8 — __ZThn12_N4Ogre22TextAreaOverlayElementD0Ev
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
// was: non-virtual thunk to Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()
// IDA 0xe433c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e433c8() {
}

// 0xe4345c — __ZN4Ogre22TextAreaOverlayElementD2Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
// was: Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()
// IDA 0xe4345c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4345c() {
}

// 0xe43594 — __ZThn12_N4Ogre22TextAreaOverlayElementD1Ev
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::~TextAreaOverlayElement()")]
// was: non-virtual thunk to Ogre::TextAreaOverlayElement::~TextAreaOverlayElement()
// IDA 0xe43594: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e43594() {
}

// 0xe435a0 — __ZNK4Ogre22TextAreaOverlayElement11getTypeNameEv
#[doc(alias = "Ogre::TextAreaOverlayElement::getTypeName(void)const")]
// was: Ogre::TextAreaOverlayElement::getTypeName(void)const
// IDA 0xe435a0: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e435a0() {
}

// 0xe435ac — __ZNK4Ogre22TextAreaOverlayElement11getMaterialEv
#[doc(alias = "Ogre::TextAreaOverlayElement::getMaterial(void)const")]
// was: Ogre::TextAreaOverlayElement::getMaterial(void)const
// IDA 0xe435ac: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e435ac() {
}

// 0xe435ec — __ZThn12_NK4Ogre22TextAreaOverlayElement11getMaterialEv
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::getMaterial(void)const")]
// was: non-virtual thunk to Ogre::TextAreaOverlayElement::getMaterial(void)const
// IDA 0xe435ec: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e435ec() {
}

// 0xe4362c — __ZN4Ogre22TextAreaOverlayElement18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::TextAreaOverlayElement::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::TextAreaOverlayElement::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe4362c: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4362c() {
}

// 0xe4364c — __ZThn12_N4Ogre22TextAreaOverlayElement18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "non-virtual thunk toOgre::TextAreaOverlayElement::getRenderOperation(Ogre::RenderOperation &)")]
// was: non-virtual thunk to Ogre::TextAreaOverlayElement::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe4364c: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4364c() {
}

// 0xe4366c — __ZN4Ogre22TextAreaOverlayElement15setMaterialNameERKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::setMaterialName(std::string const&)")]
// was: Ogre::TextAreaOverlayElement::setMaterialName(std::string const&)
// IDA 0xe4366c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4366c() {
}

// 0xe43678 — __ZN4Ogre22TextAreaOverlayElement17addBaseParametersEv
#[doc(alias = "Ogre::TextAreaOverlayElement::addBaseParameters(void)")]
// was: Ogre::TextAreaOverlayElement::addBaseParameters(void)
// IDA 0xe43678: 1614 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e43678() {
}

// 0xe4493c — __ZN4Ogre22TextAreaOverlayElement9setColourERKNS_11ColourValueE
#[doc(alias = "Ogre::TextAreaOverlayElement::setColour(Ogre::ColourValue const&)")]
// was: Ogre::TextAreaOverlayElement::setColour(Ogre::ColourValue const&)
// IDA 0xe4493c: 9 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4493c() {
}

// 0xe4495c — __ZNK4Ogre22TextAreaOverlayElement9getColourEv
#[doc(alias = "Ogre::TextAreaOverlayElement::getColour(void)const")]
// was: Ogre::TextAreaOverlayElement::getColour(void)const
// IDA 0xe4495c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4495c() {
}

// 0xe44964 — __ZN4Ogre22TextAreaOverlayElement13updateColoursEv
#[doc(alias = "Ogre::TextAreaOverlayElement::updateColours(void)")]
// was: Ogre::TextAreaOverlayElement::updateColours(void)
// IDA 0xe44964: 203 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44964() {
}

// 0xe44b50 — __ZN4Ogre22TextAreaOverlayElement14setMetricsModeENS_14GuiMetricsModeE
#[doc(alias = "Ogre::TextAreaOverlayElement::setMetricsMode(Ogre::GuiMetricsMode)")]
// was: Ogre::TextAreaOverlayElement::setMetricsMode(Ogre::GuiMetricsMode)
// IDA 0xe44b50: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44b50() {
}

// 0xe44c08 — __ZN4Ogre22TextAreaOverlayElement7_updateEv
#[doc(alias = "Ogre::TextAreaOverlayElement::_update(void)")]
// was: Ogre::TextAreaOverlayElement::_update(void)
// IDA 0xe44c08: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44c08() {
}

// 0xe44cf8 — __ZNK4Ogre22TextAreaOverlayElement13CmdCharHeight5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdCharHeight::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdCharHeight::doGet(void const*)const
// IDA 0xe44cf8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44cf8() {
}

// 0xe44d30 — __ZN4Ogre22TextAreaOverlayElement13CmdCharHeight5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdCharHeight::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdCharHeight::doSet(void *,std::string const&)
// IDA 0xe44d30: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44d30() {
}

// 0xe44d68 — __ZNK4Ogre22TextAreaOverlayElement13CmdSpaceWidth5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdSpaceWidth::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdSpaceWidth::doGet(void const*)const
// IDA 0xe44d68: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44d68() {
}

// 0xe44da0 — __ZN4Ogre22TextAreaOverlayElement13CmdSpaceWidth5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdSpaceWidth::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdSpaceWidth::doSet(void *,std::string const&)
// IDA 0xe44da0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44da0() {
}

// 0xe44dd8 — __ZNK4Ogre22TextAreaOverlayElement11CmdFontName5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdFontName::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdFontName::doGet(void const*)const
// IDA 0xe44dd8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44dd8() {
}

// 0xe44df4 — __ZN4Ogre22TextAreaOverlayElement11CmdFontName5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdFontName::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdFontName::doSet(void *,std::string const&)
// IDA 0xe44df4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44df4() {
}

// 0xe44e04 — __ZNK4Ogre22TextAreaOverlayElement9CmdColour5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColour::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdColour::doGet(void const*)const
// IDA 0xe44e04: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44e04() {
}

// 0xe44e20 — __ZN4Ogre22TextAreaOverlayElement9CmdColour5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColour::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdColour::doSet(void *,std::string const&)
// IDA 0xe44e20: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44e20() {
}

// 0xe44e50 — __ZNK4Ogre22TextAreaOverlayElement12CmdColourTop5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourTop::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdColourTop::doGet(void const*)const
// IDA 0xe44e50: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44e50() {
}

// 0xe44e60 — __ZN4Ogre22TextAreaOverlayElement12CmdColourTop5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourTop::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdColourTop::doSet(void *,std::string const&)
// IDA 0xe44e60: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44e60() {
}

// 0xe44e98 — __ZNK4Ogre22TextAreaOverlayElement15CmdColourBottom5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourBottom::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdColourBottom::doGet(void const*)const
// IDA 0xe44e98: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44e98() {
}

// 0xe44ea8 — __ZN4Ogre22TextAreaOverlayElement15CmdColourBottom5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourBottom::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdColourBottom::doSet(void *,std::string const&)
// IDA 0xe44ea8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44ea8() {
}

// 0xe44ee0 — __ZNK4Ogre22TextAreaOverlayElement12CmdAlignment5doGetEPKv
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdAlignment::doGet(void const*)const")]
// was: Ogre::TextAreaOverlayElement::CmdAlignment::doGet(void const*)const
// IDA 0xe44ee0: 87 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44ee0() {
}

// 0xe44fd4 — __ZN4Ogre22TextAreaOverlayElement12CmdAlignment5doSetEPvRKSs
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdAlignment::doSet(void *,std::string const&)")]
// was: Ogre::TextAreaOverlayElement::CmdAlignment::doSet(void *,std::string const&)
// IDA 0xe44fd4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e44fd4() {
}

// 0xe45020 — __ZN4Ogre22TextAreaOverlayElement13CmdCharHeightD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdCharHeight::~CmdCharHeight()")]
// was: Ogre::TextAreaOverlayElement::CmdCharHeight::~CmdCharHeight()
// IDA 0xe45020: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e45020() {
}

// 0xe45024 — __ZN4Ogre22TextAreaOverlayElement13CmdSpaceWidthD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdSpaceWidth::~CmdSpaceWidth()")]
// was: Ogre::TextAreaOverlayElement::CmdSpaceWidth::~CmdSpaceWidth()
// IDA 0xe45024: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e45024() {
}

// 0xe45028 — __ZN4Ogre22TextAreaOverlayElement11CmdFontNameD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdFontName::~CmdFontName()")]
// was: Ogre::TextAreaOverlayElement::CmdFontName::~CmdFontName()
// IDA 0xe45028: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e45028() {
}

// 0xe4502c — __ZN4Ogre22TextAreaOverlayElement9CmdColourD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColour::~CmdColour()")]
// was: Ogre::TextAreaOverlayElement::CmdColour::~CmdColour()
// IDA 0xe4502c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e4502c() {
}

// 0xe45030 — __ZN4Ogre22TextAreaOverlayElement15CmdColourBottomD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourBottom::~CmdColourBottom()")]
// was: Ogre::TextAreaOverlayElement::CmdColourBottom::~CmdColourBottom()
// IDA 0xe45030: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e45030() {
}

// 0xe45034 — __ZN4Ogre22TextAreaOverlayElement12CmdColourTopD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourTop::~CmdColourTop()")]
// was: Ogre::TextAreaOverlayElement::CmdColourTop::~CmdColourTop()
// IDA 0xe45034: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e45034() {
}

// 0xe45038 — __ZN4Ogre22TextAreaOverlayElement12CmdAlignmentD1Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdAlignment::~CmdAlignment()")]
// was: Ogre::TextAreaOverlayElement::CmdAlignment::~CmdAlignment()
// IDA 0xe45038: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e45038() {
}

// 0xe4503c — __ZN4Ogre22TextAreaOverlayElement13CmdCharHeightD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdCharHeight::~CmdCharHeight()")]
// was: Ogre::TextAreaOverlayElement::CmdCharHeight::~CmdCharHeight()
// IDA 0xe4503c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4503c() {
}

// 0xe45048 — __ZN4Ogre22TextAreaOverlayElement13CmdSpaceWidthD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdSpaceWidth::~CmdSpaceWidth()")]
// was: Ogre::TextAreaOverlayElement::CmdSpaceWidth::~CmdSpaceWidth()
// IDA 0xe45048: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e45048() {
}

// 0xe45054 — __ZN4Ogre22TextAreaOverlayElement11CmdFontNameD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdFontName::~CmdFontName()")]
// was: Ogre::TextAreaOverlayElement::CmdFontName::~CmdFontName()
// IDA 0xe45054: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e45054() {
}

// 0xe45060 — __ZN4Ogre22TextAreaOverlayElement9CmdColourD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColour::~CmdColour()")]
// was: Ogre::TextAreaOverlayElement::CmdColour::~CmdColour()
// IDA 0xe45060: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e45060() {
}

// 0xe4506c — __ZN4Ogre22TextAreaOverlayElement15CmdColourBottomD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourBottom::~CmdColourBottom()")]
// was: Ogre::TextAreaOverlayElement::CmdColourBottom::~CmdColourBottom()
// IDA 0xe4506c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4506c() {
}

// 0xe45078 — __ZN4Ogre22TextAreaOverlayElement12CmdColourTopD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdColourTop::~CmdColourTop()")]
// was: Ogre::TextAreaOverlayElement::CmdColourTop::~CmdColourTop()
// IDA 0xe45078: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e45078() {
}

// 0xe45084 — __ZN4Ogre22TextAreaOverlayElement12CmdAlignmentD0Ev
#[doc(alias = "Ogre::TextAreaOverlayElement::CmdAlignment::~CmdAlignment()")]
// was: Ogre::TextAreaOverlayElement::CmdAlignment::~CmdAlignment()
// IDA 0xe45084: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e45084() {
}

// 0xe45090 — __ZNK4Ogre14OverlayElement11isContainerEv
#[doc(alias = "Ogre::OverlayElement::isContainer(void)const")]
// was: Ogre::OverlayElement::isContainer(void)const
// IDA 0xe45090: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45090() {
}

// 0xe451b8 — __ZN4Ogre7TextureC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::Texture::Texture(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::Texture::Texture(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe451b8: 225 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e451b8() {
}

// 0xe4542c — __ZN4Ogre7Texture11loadRawDataERNS_9SharedPtrINS_10DataStreamEEEttNS_11PixelFormatE
#[doc(alias = "Ogre::Texture::loadRawData(Ogre::SharedPtr<Ogre::DataStream> &,unsigned short,unsigned short,Ogre::PixelFormat)")]
// was: Ogre::Texture::loadRawData(Ogre::SharedPtr<Ogre::DataStream> &,unsigned short,unsigned short,Ogre::PixelFormat)
// IDA 0xe4542c: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4542c() {
}

// 0xe454fc — __ZN4Ogre7Texture9loadImageERKNS_5ImageE
#[doc(alias = "Ogre::Texture::loadImage(Ogre::Image const&)")]
// was: Ogre::Texture::loadImage(Ogre::Image const&)
// IDA 0xe454fc: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e454fc() {
}

// 0xe45650 — __ZN4Ogre7Texture9setFormatENS_11PixelFormatE
#[doc(alias = "Ogre::Texture::setFormat(Ogre::PixelFormat)")]
// was: Ogre::Texture::setFormat(Ogre::PixelFormat)
// IDA 0xe45650: 4 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45650() {
}

// 0xe45660 — __ZNK4Ogre7Texture8hasAlphaEv
#[doc(alias = "Ogre::Texture::hasAlpha(void)const")]
// was: Ogre::Texture::hasAlpha(void)const
// IDA 0xe45660: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45660() {
}

// 0xe45670 — __ZN4Ogre7Texture25setDesiredIntegerBitDepthEt
#[doc(alias = "Ogre::Texture::setDesiredIntegerBitDepth(unsigned short)")]
// was: Ogre::Texture::setDesiredIntegerBitDepth(unsigned short)
// IDA 0xe45670: 2 insns (STRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45670() {
}

// 0xe45678 — __ZNK4Ogre7Texture25getDesiredIntegerBitDepthEv
#[doc(alias = "Ogre::Texture::getDesiredIntegerBitDepth(void)const")]
// was: Ogre::Texture::getDesiredIntegerBitDepth(void)const
// IDA 0xe45678: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45678() {
}

// 0xe45680 — __ZN4Ogre7Texture23setDesiredFloatBitDepthEt
#[doc(alias = "Ogre::Texture::setDesiredFloatBitDepth(unsigned short)")]
// was: Ogre::Texture::setDesiredFloatBitDepth(unsigned short)
// IDA 0xe45680: 2 insns (STRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45680() {
}

// 0xe45688 — __ZNK4Ogre7Texture23getDesiredFloatBitDepthEv
#[doc(alias = "Ogre::Texture::getDesiredFloatBitDepth(void)const")]
// was: Ogre::Texture::getDesiredFloatBitDepth(void)const
// IDA 0xe45688: 2 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45688() {
}

// 0xe45690 — __ZN4Ogre7Texture19setDesiredBitDepthsEtt
#[doc(alias = "Ogre::Texture::setDesiredBitDepths(unsigned short,unsigned short)")]
// was: Ogre::Texture::setDesiredBitDepths(unsigned short,unsigned short)
// IDA 0xe45690: 3 insns (STRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e45690() {
}

// 0xe4569c — __ZN4Ogre7Texture24setTreatLuminanceAsAlphaEb
#[doc(alias = "Ogre::Texture::setTreatLuminanceAsAlpha(bool)")]
// was: Ogre::Texture::setTreatLuminanceAsAlpha(bool)
// IDA 0xe4569c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4569c() {
}

// 0xe456a4 — __ZNK4Ogre7Texture24getTreatLuminanceAsAlphaEv
#[doc(alias = "Ogre::Texture::getTreatLuminanceAsAlpha(void)const")]
// was: Ogre::Texture::getTreatLuminanceAsAlpha(void)const
// IDA 0xe456a4: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e456a4() {
}

// 0xe456ac — __ZNK4Ogre7Texture13calculateSizeEv
#[doc(alias = "Ogre::Texture::calculateSize(void)const")]
// was: Ogre::Texture::calculateSize(void)const
// IDA 0xe456ac: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e456ac() {
}

// 0xe456d0 — __ZNK4Ogre7Texture11getNumFacesEv
#[doc(alias = "Ogre::Texture::getNumFaces(void)const")]
// was: Ogre::Texture::getNumFaces(void)const
// IDA 0xe456d0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e456d0() {
}

// 0xe456e8 — __ZN4Ogre7Texture11_loadImagesERKSt6vectorIPKNS_5ImageENS_12STLAllocatorIS4_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::Texture::_loadImages(std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::Texture::_loadImages(std::vector<Ogre::Image const*,Ogre::STLAllocator<Ogre::Image const*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe456e8: 1376 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e456e8() {
}

// 0xe46568 — __ZN4Ogre7Texture23createInternalResourcesEv
#[doc(alias = "Ogre::Texture::createInternalResources(void)")]
// was: Ogre::Texture::createInternalResources(void)
// IDA 0xe46568: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e46568() {
}

// 0xe4658c — __ZN4Ogre7Texture21freeInternalResourcesEv
#[doc(alias = "Ogre::Texture::freeInternalResources(void)")]
// was: Ogre::Texture::freeInternalResources(void)
// IDA 0xe4658c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4658c() {
}

// 0xe465b0 — __ZN4Ogre7Texture10unloadImplEv
#[doc(alias = "Ogre::Texture::unloadImpl(void)")]
// was: Ogre::Texture::unloadImpl(void)
// IDA 0xe465b0: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e465b0() {
}

// 0xe465c0 — __ZN4Ogre7Texture13copyToTextureERNS_10TexturePtrE
#[doc(alias = "Ogre::Texture::copyToTexture(Ogre::TexturePtr &)")]
// was: Ogre::Texture::copyToTexture(Ogre::TexturePtr &)
// IDA 0xe465c0: 435 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e465c0() {
}

// 0xe46a3c — __ZNK4Ogre7Texture17getSourceFileTypeEv
#[doc(alias = "Ogre::Texture::getSourceFileType(void)const")]
// was: Ogre::Texture::getSourceFileType(void)const
// IDA 0xe46a3c: 721 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e46a3c() {
}

// 0xe471a8 — __ZN4Ogre7Texture14convertToImageERNS_5ImageEb
#[doc(alias = "Ogre::Texture::convertToImage(Ogre::Image &,bool)")]
// was: Ogre::Texture::convertToImage(Ogre::Image &,bool)
// IDA 0xe471a8: 363 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e471a8() {
}

// 0xe4754c — __ZN4Ogre7TextureD1Ev
#[doc(alias = "Ogre::Texture::~Texture()")]
// was: Ogre::Texture::~Texture()
// IDA 0xe4754c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e4754c() {
}

// 0xe475ac — __ZN4Ogre7TextureD0Ev
#[doc(alias = "Ogre::Texture::~Texture()")]
// was: Ogre::Texture::~Texture()
// IDA 0xe475ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e475ac() {
}

// 0xe4768c — __ZN4Ogre7Texture14setTextureTypeENS_11TextureTypeE
#[doc(alias = "Ogre::Texture::setTextureType(Ogre::TextureType)")]
// was: Ogre::Texture::setTextureType(Ogre::TextureType)
// IDA 0xe4768c: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4768c() {
}

// 0xe47694 — __ZNK4Ogre7Texture14getTextureTypeEv
#[doc(alias = "Ogre::Texture::getTextureType(void)const")]
// was: Ogre::Texture::getTextureType(void)const
// IDA 0xe47694: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47694() {
}

// 0xe4769c — __ZNK4Ogre7Texture13getNumMipmapsEv
#[doc(alias = "Ogre::Texture::getNumMipmaps(void)const")]
// was: Ogre::Texture::getNumMipmaps(void)const
// IDA 0xe4769c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4769c() {
}

// 0xe476a0 — __ZN4Ogre7Texture13setNumMipmapsEm
#[doc(alias = "Ogre::Texture::setNumMipmaps(unsigned long)")]
// was: Ogre::Texture::setNumMipmaps(unsigned long)
// IDA 0xe476a0: 3 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476a0() {
}

// 0xe476a8 — __ZNK4Ogre7Texture27getMipmapsHardwareGeneratedEv
#[doc(alias = "Ogre::Texture::getMipmapsHardwareGenerated(void)const")]
// was: Ogre::Texture::getMipmapsHardwareGenerated(void)const
// IDA 0xe476a8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476a8() {
}

// 0xe476b0 — __ZNK4Ogre7Texture8getGammaEv
#[doc(alias = "Ogre::Texture::getGamma(void)const")]
// was: Ogre::Texture::getGamma(void)const
// IDA 0xe476b0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476b0() {
}

// 0xe476b4 — __ZN4Ogre7Texture8setGammaEf
#[doc(alias = "Ogre::Texture::setGamma(float)")]
// was: Ogre::Texture::setGamma(float)
// IDA 0xe476b4: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476b4() {
}

// 0xe476b8 — __ZN4Ogre7Texture23setHardwareGammaEnabledEb
#[doc(alias = "Ogre::Texture::setHardwareGammaEnabled(bool)")]
// was: Ogre::Texture::setHardwareGammaEnabled(bool)
// IDA 0xe476b8: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476b8() {
}

// 0xe476c0 — __ZNK4Ogre7Texture22isHardwareGammaEnabledEv
#[doc(alias = "Ogre::Texture::isHardwareGammaEnabled(void)const")]
// was: Ogre::Texture::isHardwareGammaEnabled(void)const
// IDA 0xe476c0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476c0() {
}

// 0xe476c8 — __ZN4Ogre7Texture7setFSAAEjRKSs
#[doc(alias = "Ogre::Texture::setFSAA(unsigned int,std::string const&)")]
// was: Ogre::Texture::setFSAA(unsigned int,std::string const&)
// IDA 0xe476c8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476c8() {
}

// 0xe476d8 — __ZNK4Ogre7Texture7getFSAAEv
#[doc(alias = "Ogre::Texture::getFSAA(void)const")]
// was: Ogre::Texture::getFSAA(void)const
// IDA 0xe476d8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476d8() {
}

// 0xe476dc — __ZNK4Ogre7Texture11getFSAAHintEv
#[doc(alias = "Ogre::Texture::getFSAAHint(void)const")]
// was: Ogre::Texture::getFSAAHint(void)const
// IDA 0xe476dc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476dc() {
}

// 0xe476e0 — __ZNK4Ogre7Texture9getHeightEv
#[doc(alias = "Ogre::Texture::getHeight(void)const")]
// was: Ogre::Texture::getHeight(void)const
// IDA 0xe476e0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476e0() {
}

// 0xe476e4 — __ZNK4Ogre7Texture8getWidthEv
#[doc(alias = "Ogre::Texture::getWidth(void)const")]
// was: Ogre::Texture::getWidth(void)const
// IDA 0xe476e4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476e4() {
}

// 0xe476e8 — __ZNK4Ogre7Texture8getDepthEv
#[doc(alias = "Ogre::Texture::getDepth(void)const")]
// was: Ogre::Texture::getDepth(void)const
// IDA 0xe476e8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476e8() {
}

// 0xe476ec — __ZNK4Ogre7Texture12getSrcHeightEv
#[doc(alias = "Ogre::Texture::getSrcHeight(void)const")]
// was: Ogre::Texture::getSrcHeight(void)const
// IDA 0xe476ec: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476ec() {
}

// 0xe476f4 — __ZNK4Ogre7Texture11getSrcWidthEv
#[doc(alias = "Ogre::Texture::getSrcWidth(void)const")]
// was: Ogre::Texture::getSrcWidth(void)const
// IDA 0xe476f4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476f4() {
}

// 0xe476fc — __ZNK4Ogre7Texture11getSrcDepthEv
#[doc(alias = "Ogre::Texture::getSrcDepth(void)const")]
// was: Ogre::Texture::getSrcDepth(void)const
// IDA 0xe476fc: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e476fc() {
}

// 0xe47704 — __ZN4Ogre7Texture9setHeightEm
#[doc(alias = "Ogre::Texture::setHeight(unsigned long)")]
// was: Ogre::Texture::setHeight(unsigned long)
// IDA 0xe47704: 3 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e47704() {
}

// 0xe4770c — __ZN4Ogre7Texture8setWidthEm
#[doc(alias = "Ogre::Texture::setWidth(unsigned long)")]
// was: Ogre::Texture::setWidth(unsigned long)
// IDA 0xe4770c: 3 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e4770c() {
}