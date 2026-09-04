//! rendering shard C — next 120 stubs EA-sorted after 0xd260e8 strict Ogre|G3D (filtered 7875 total, 3424 prior claimed in rendering, 4141 remaining global, this batch 0xd266c8..0xd2ce68)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd266c8 — __ZN4Ogre9SharedPtrINS_8MaterialEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::Material>::destroy(void)
// IDA 0xd266c8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d266c8() {
}

// 0xd26700 — __ZN4Ogre9SharedPtrINS_8MaterialEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Material>::swap(Ogre::SharedPtr<Ogre::Material>&)")]
// was: Ogre::SharedPtr<Ogre::Material>::swap(Ogre::SharedPtr<Ogre::Material>&)
// IDA 0xd26700: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26700() {
}

// 0xd26720 — __ZN4Ogre7MeshPtrD0Ev
#[doc(alias = "Ogre::MeshPtr::~MeshPtr()")]
// was: Ogre::MeshPtr::~MeshPtr()
// IDA 0xd26720: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d26720() {
}

// 0xd26a20 — __ZN4Ogre20OptimisedUtilGeneral22softwareVertexSkinningEPKfPfS2_S3_S2_PKhPKPKNS_7Matrix4Emmmmmmmm
#[doc(alias = "Ogre::OptimisedUtilGeneral::softwareVertexSkinning(float const*,float *,float const*,float *,float const*,unsigned char const*,Ogre::Matrix4 const* const*,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long)")]
// was: Ogre::OptimisedUtilGeneral::softwareVertexSkinning(float const*,float *,float const*,float *,float const*,unsigned char const*,Ogre::Matrix4 const* const*,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long,unsigned long)
// IDA 0xd26a20: 188 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26a20() {
}

// 0xd26cb8 — __ZN4Ogre20OptimisedUtilGeneral25concatenateAffineMatricesERKNS_7Matrix4EPS2_PS1_m
#[doc(alias = "Ogre::OptimisedUtilGeneral::concatenateAffineMatrices(Ogre::Matrix4 const&,Ogre::Matrix4 const*,Ogre::Matrix4*,unsigned long)")]
// was: Ogre::OptimisedUtilGeneral::concatenateAffineMatrices(Ogre::Matrix4 const&,Ogre::Matrix4 const*,Ogre::Matrix4*,unsigned long)
// IDA 0xd26cb8: 166 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26cb8() {
}

// 0xd26f44 — __ZN4Ogre20OptimisedUtilGeneral19softwareVertexMorphEfPKfS2_Pfmmmmb
#[doc(alias = "Ogre::OptimisedUtilGeneral::softwareVertexMorph(float,float const*,float const*,float *,unsigned long,unsigned long,unsigned long,unsigned long,bool)")]
// was: Ogre::OptimisedUtilGeneral::softwareVertexMorph(float,float const*,float const*,float *,unsigned long,unsigned long,unsigned long,unsigned long,bool)
// IDA 0xd26f44: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d26f44() {
}

// 0xd27084 — __ZN4Ogre20OptimisedUtilGeneral20calculateFaceNormalsEPKfPKNS_8EdgeData8TriangleEPNS_7Vector4Em
#[doc(alias = "Ogre::OptimisedUtilGeneral::calculateFaceNormals(float const*,Ogre::EdgeData::Triangle const*,Ogre::Vector4 *,unsigned long)")]
// was: Ogre::OptimisedUtilGeneral::calculateFaceNormals(float const*,Ogre::EdgeData::Triangle const*,Ogre::Vector4 *,unsigned long)
// IDA 0xd27084: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27084() {
}

// 0xd27148 — __ZN4Ogre20OptimisedUtilGeneral20calculateLightFacingERKNS_7Vector4EPS2_Pcm
#[doc(alias = "Ogre::OptimisedUtilGeneral::calculateLightFacing(Ogre::Vector4 const&,Ogre::Vector4 const*,char *,unsigned long)")]
// was: Ogre::OptimisedUtilGeneral::calculateLightFacing(Ogre::Vector4 const&,Ogre::Vector4 const*,char *,unsigned long)
// IDA 0xd27148: 34 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27148() {
}

// 0xd271c0 — __ZN4Ogre20OptimisedUtilGeneral15extrudeVerticesERKNS_7Vector4EfPKfPfm
#[doc(alias = "Ogre::OptimisedUtilGeneral::extrudeVertices(Ogre::Vector4 const&,float,float const*,float *,unsigned long)")]
// was: Ogre::OptimisedUtilGeneral::extrudeVertices(Ogre::Vector4 const&,float,float const*,float *,unsigned long)
// IDA 0xd271c0: 89 insns (LDMFD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d271c0() {
}

// 0xd27310 — __ZN4Ogre24_getOptimisedUtilGeneralEv
#[doc(alias = "Ogre::_getOptimisedUtilGeneral(void)")]
// was: Ogre::_getOptimisedUtilGeneral(void)
// IDA 0xd27310: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27310() {
}

// 0xd2734c — __ZN4Ogre20OptimisedUtilGeneralD1Ev
#[doc(alias = "Ogre::OptimisedUtilGeneral::~OptimisedUtilGeneral()")]
// was: Ogre::OptimisedUtilGeneral::~OptimisedUtilGeneral()
// IDA 0xd2734c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2734c() {
}

// 0xd27350 — __ZN4Ogre20OptimisedUtilGeneralD0Ev
#[doc(alias = "Ogre::OptimisedUtilGeneral::~OptimisedUtilGeneral()")]
// was: Ogre::OptimisedUtilGeneral::~OptimisedUtilGeneral()
// IDA 0xd27350: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27350() {
}

// 0xd27390 — __ZN4Ogre7OverlayC1ERKSs
#[doc(alias = "Ogre::Overlay::Overlay(std::string const&)")]
// was: Ogre::Overlay::Overlay(std::string const&)
// IDA 0xd27390: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27390() {
}

// 0xd2739c — __ZN4Ogre7OverlayC2ERKSs
#[doc(alias = "Ogre::Overlay::Overlay(std::string const&)")]
// was: Ogre::Overlay::Overlay(std::string const&)
// IDA 0xd2739c: 201 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2739c() {
}

// 0xd275ac — __ZN4Ogre7OverlayD0Ev
#[doc(alias = "Ogre::Overlay::~Overlay()")]
// was: Ogre::Overlay::~Overlay()
// IDA 0xd275ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d275ac() {
}

// 0xd2763c — __ZN4Ogre7OverlayD1Ev
#[doc(alias = "Ogre::Overlay::~Overlay()")]
// was: Ogre::Overlay::~Overlay()
// IDA 0xd2763c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2763c() {
}

// 0xd27648 — __ZN4Ogre7OverlayD2Ev
#[doc(alias = "Ogre::Overlay::~Overlay()")]
// was: Ogre::Overlay::~Overlay()
// IDA 0xd27648: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27648() {
}

// 0xd27860 — __ZNK4Ogre7Overlay7getNameEv
#[doc(alias = "Ogre::Overlay::getName(void)const")]
// was: Ogre::Overlay::getName(void)const
// IDA 0xd27860: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27860() {
}

// 0xd27864 — __ZN4Ogre7Overlay9setZOrderEt
#[doc(alias = "Ogre::Overlay::setZOrder(unsigned short)")]
// was: Ogre::Overlay::setZOrder(unsigned short)
// IDA 0xd27864: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27864() {
}

// 0xd278a8 — __ZN4Ogre7Overlay5add2DEPNS_16OverlayContainerE
#[doc(alias = "Ogre::Overlay::add2D(Ogre::OverlayContainer *)")]
// was: Ogre::Overlay::add2D(Ogre::OverlayContainer *)
// IDA 0xd278a8: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d278a8() {
}

// 0xd2797c — __ZNK4Ogre7Overlay19_getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Overlay::_getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::Overlay::_getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xd2797c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2797c() {
}

// 0xd279d0 — __ZN4Ogre7Overlay8remove2DEPNS_16OverlayContainerE
#[doc(alias = "Ogre::Overlay::remove2D(Ogre::OverlayContainer *)")]
// was: Ogre::Overlay::remove2D(Ogre::OverlayContainer *)
// IDA 0xd279d0: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d279d0() {
}

// 0xd27a4c — __ZN4Ogre7Overlay6scrollEff
#[doc(alias = "Ogre::Overlay::scroll(float,float)")]
// was: Ogre::Overlay::scroll(float,float)
// IDA 0xd27a4c: 12 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27a4c() {
}

// 0xd27a7c — __ZNK4Ogre7Overlay15updateTransformEv
#[doc(alias = "Ogre::Overlay::updateTransform(void)const")]
// was: Ogre::Overlay::updateTransform(void)const
// IDA 0xd27a7c: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27a7c() {
}

// 0xd27b90 — __ZN4Ogre7Overlay19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueE
#[doc(alias = "Ogre::Overlay::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *)")]
// was: Ogre::Overlay::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *)
// IDA 0xd27b90: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27b90() {
}

// 0xd27cf8 — __ZN4Ogre7Overlay13findElementAtEff
#[doc(alias = "Ogre::Overlay::findElementAt(float,float)")]
// was: Ogre::Overlay::findElementAt(float,float)
// IDA 0xd27cf8: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27cf8() {
}

// 0xd27d9c — __ZN4Ogre16OverlayContainerC2ERKSs
#[doc(alias = "Ogre::OverlayContainer::OverlayContainer(std::string const&)")]
// was: Ogre::OverlayContainer::OverlayContainer(std::string const&)
// IDA 0xd27d9c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d27d9c() {
}

// 0xd27e1c — __ZN4Ogre16OverlayContainerD0Ev
#[doc(alias = "Ogre::OverlayContainer::~OverlayContainer()")]
// was: Ogre::OverlayContainer::~OverlayContainer()
// IDA 0xd27e1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27e1c() {
}

// 0xd27eac — __ZN4Ogre16OverlayContainerD1Ev
#[doc(alias = "Ogre::OverlayContainer::~OverlayContainer()")]
// was: Ogre::OverlayContainer::~OverlayContainer()
// IDA 0xd27eac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27eac() {
}

// 0xd27eb8 — __ZThn12_N4Ogre16OverlayContainerD0Ev
#[doc(alias = "non-virtual thunk toOgre::OverlayContainer::~OverlayContainer()")]
// was: non-virtual thunk to Ogre::OverlayContainer::~OverlayContainer()
// IDA 0xd27eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27eb8() {
}

// 0xd27f4c — __ZN4Ogre16OverlayContainerD2Ev
#[doc(alias = "Ogre::OverlayContainer::~OverlayContainer()")]
// was: Ogre::OverlayContainer::~OverlayContainer()
// IDA 0xd27f4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d27f4c() {
}

// 0xd280bc — __ZThn12_N4Ogre16OverlayContainerD1Ev
#[doc(alias = "non-virtual thunk toOgre::OverlayContainer::~OverlayContainer()")]
// was: non-virtual thunk to Ogre::OverlayContainer::~OverlayContainer()
// IDA 0xd280bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d280bc() {
}

// 0xd280c8 — __ZN4Ogre16OverlayContainer8addChildEPNS_14OverlayElementE
#[doc(alias = "Ogre::OverlayContainer::addChild(Ogre::OverlayElement *)")]
// was: Ogre::OverlayContainer::addChild(Ogre::OverlayElement *)
// IDA 0xd280c8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d280c8() {
}

// 0xd280f0 — __ZN4Ogre16OverlayContainer12addChildImplEPNS_14OverlayElementE
#[doc(alias = "Ogre::OverlayContainer::addChildImpl(Ogre::OverlayElement *)")]
// was: Ogre::OverlayContainer::addChildImpl(Ogre::OverlayElement *)
// IDA 0xd280f0: 405 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d280f0() {
}

// 0xd2858c — __ZN4Ogre16OverlayContainer12addChildImplEPS0_
#[doc(alias = "Ogre::OverlayContainer::addChildImpl(Ogre::OverlayContainer*)")]
// was: Ogre::OverlayContainer::addChildImpl(Ogre::OverlayContainer*)
// IDA 0xd2858c: 104 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2858c() {
}

// 0xd286c0 — __ZN4Ogre16OverlayContainer11removeChildERKSs
#[doc(alias = "Ogre::OverlayContainer::removeChild(std::string const&)")]
// was: Ogre::OverlayContainer::removeChild(std::string const&)
// IDA 0xd286c0: 322 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d286c0() {
}

// 0xd28a68 — __ZN4Ogre16OverlayContainer12_removeChildERKSs
#[doc(alias = "Ogre::OverlayContainer::_removeChild(std::string const&)")]
// was: Ogre::OverlayContainer::_removeChild(std::string const&)
// IDA 0xd28a68: 322 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d28a68() {
}

// 0xd28e10 — __ZN4Ogre16OverlayContainer8getChildERKSs
#[doc(alias = "Ogre::OverlayContainer::getChild(std::string const&)")]
// was: Ogre::OverlayContainer::getChild(std::string const&)
// IDA 0xd28e10: 254 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d28e10() {
}

// 0xd290f0 — __ZN4Ogre16OverlayContainer16getChildIteratorEv
#[doc(alias = "Ogre::OverlayContainer::getChildIterator(void)")]
// was: Ogre::OverlayContainer::getChildIterator(void)
// IDA 0xd290f0: 6 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d290f0() {
}

// 0xd29100 — __ZN4Ogre16OverlayContainer25getChildContainerIteratorEv
#[doc(alias = "Ogre::OverlayContainer::getChildContainerIterator(void)")]
// was: Ogre::OverlayContainer::getChildContainerIterator(void)
// IDA 0xd29100: 6 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29100() {
}

// 0xd29110 — __ZN4Ogre16OverlayContainer10initialiseEv
#[doc(alias = "Ogre::OverlayContainer::initialise(void)")]
// was: Ogre::OverlayContainer::initialise(void)
// IDA 0xd29110: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29110() {
}

// 0xd29154 — __ZN4Ogre16OverlayContainer19_positionsOutOfDateEv
#[doc(alias = "Ogre::OverlayContainer::_positionsOutOfDate(void)")]
// was: Ogre::OverlayContainer::_positionsOutOfDate(void)
// IDA 0xd29154: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29154() {
}

// 0xd2918c — __ZN4Ogre16OverlayContainer7_updateEv
#[doc(alias = "Ogre::OverlayContainer::_update(void)")]
// was: Ogre::OverlayContainer::_update(void)
// IDA 0xd2918c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2918c() {
}

// 0xd291c4 — __ZN4Ogre16OverlayContainer13_notifyZOrderEt
#[doc(alias = "Ogre::OverlayContainer::_notifyZOrder(unsigned short)")]
// was: Ogre::OverlayContainer::_notifyZOrder(unsigned short)
// IDA 0xd291c4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d291c4() {
}

// 0xd29208 — __ZN4Ogre16OverlayContainer22_notifyWorldTransformsERKNS_7Matrix4E
#[doc(alias = "Ogre::OverlayContainer::_notifyWorldTransforms(Ogre::Matrix4 const&)")]
// was: Ogre::OverlayContainer::_notifyWorldTransforms(Ogre::Matrix4 const&)
// IDA 0xd29208: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29208() {
}

// 0xd29240 — __ZN4Ogre16OverlayContainer15_notifyViewportEv
#[doc(alias = "Ogre::OverlayContainer::_notifyViewport(void)")]
// was: Ogre::OverlayContainer::_notifyViewport(void)
// IDA 0xd29240: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29240() {
}

// 0xd29278 — __ZN4Ogre16OverlayContainer13_notifyParentEPS0_PNS_7OverlayE
#[doc(alias = "Ogre::OverlayContainer::_notifyParent(Ogre::OverlayContainer*,Ogre::Overlay *)")]
// was: Ogre::OverlayContainer::_notifyParent(Ogre::OverlayContainer*,Ogre::Overlay *)
// IDA 0xd29278: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29278() {
}

// 0xd292b4 — __ZN4Ogre16OverlayContainer18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::OverlayContainer::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::OverlayContainer::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xd292b4: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d292b4() {
}

// 0xd292f8 — __ZN4Ogre16OverlayContainer13findElementAtEff
#[doc(alias = "Ogre::OverlayContainer::findElementAt(float,float)")]
// was: Ogre::OverlayContainer::findElementAt(float,float)
// IDA 0xd292f8: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d292f8() {
}

// 0xd293b8 — __ZN4Ogre16OverlayContainer16copyFromTemplateEPNS_14OverlayElementE
#[doc(alias = "Ogre::OverlayContainer::copyFromTemplate(Ogre::OverlayElement *)")]
// was: Ogre::OverlayContainer::copyFromTemplate(Ogre::OverlayElement *)
// IDA 0xd293b8: 268 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d293b8() {
}

// 0xd29688 — __ZN4Ogre16OverlayContainer5cloneERKSs
#[doc(alias = "Ogre::OverlayContainer::clone(std::string const&)")]
// was: Ogre::OverlayContainer::clone(std::string const&)
// IDA 0xd29688: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29688() {
}

// 0xd29a90 — __ZN4Ogre14OverlayElementC2ERKSs
#[doc(alias = "Ogre::OverlayElement::OverlayElement(std::string const&)")]
// was: Ogre::OverlayElement::OverlayElement(std::string const&)
// IDA 0xd29a90: 354 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d29a90() {
}

// 0xd29e40 — __ZN4Ogre14OverlayElementD0Ev
#[doc(alias = "Ogre::OverlayElement::~OverlayElement()")]
// was: Ogre::OverlayElement::~OverlayElement()
// IDA 0xd29e40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d29e40() {
}

// 0xd29ed0 — __ZN4Ogre14OverlayElementD1Ev
#[doc(alias = "Ogre::OverlayElement::~OverlayElement()")]
// was: Ogre::OverlayElement::~OverlayElement()
// IDA 0xd29ed0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d29ed0() {
}

// 0xd29edc — __ZThn12_N4Ogre14OverlayElementD0Ev
#[doc(alias = "non-virtual thunk toOgre::OverlayElement::~OverlayElement()")]
// was: non-virtual thunk to Ogre::OverlayElement::~OverlayElement()
// IDA 0xd29edc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d29edc() {
}

// 0xd29f70 — __ZN4Ogre14OverlayElementD2Ev
#[doc(alias = "Ogre::OverlayElement::~OverlayElement()")]
// was: Ogre::OverlayElement::~OverlayElement()
// IDA 0xd29f70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d29f70() {
}

// 0xd2a33c — __ZThn12_N4Ogre14OverlayElementD1Ev
#[doc(alias = "non-virtual thunk toOgre::OverlayElement::~OverlayElement()")]
// was: non-virtual thunk to Ogre::OverlayElement::~OverlayElement()
// IDA 0xd2a33c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2a33c() {
}

// 0xd2a348 — __ZNK4Ogre14OverlayElement7getNameEv
#[doc(alias = "Ogre::OverlayElement::getName(void)const")]
// was: Ogre::OverlayElement::getName(void)const
// IDA 0xd2a348: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a348() {
}

// 0xd2a34c — __ZN4Ogre14OverlayElement4showEv
#[doc(alias = "Ogre::OverlayElement::show(void)")]
// was: Ogre::OverlayElement::show(void)
// IDA 0xd2a34c: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a34c() {
}

// 0xd2a354 — __ZN4Ogre14OverlayElement4hideEv
#[doc(alias = "Ogre::OverlayElement::hide(void)")]
// was: Ogre::OverlayElement::hide(void)
// IDA 0xd2a354: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a354() {
}

// 0xd2a35c — __ZNK4Ogre14OverlayElement9isVisibleEv
#[doc(alias = "Ogre::OverlayElement::isVisible(void)const")]
// was: Ogre::OverlayElement::isVisible(void)const
// IDA 0xd2a35c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a35c() {
}

// 0xd2a364 — __ZN4Ogre14OverlayElement8setWidthEf
#[doc(alias = "Ogre::OverlayElement::setWidth(float)")]
// was: Ogre::OverlayElement::setWidth(float)
// IDA 0xd2a364: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a364() {
}

// 0xd2a394 — __ZNK4Ogre14OverlayElement8getWidthEv
#[doc(alias = "Ogre::OverlayElement::getWidth(void)const")]
// was: Ogre::OverlayElement::getWidth(void)const
// IDA 0xd2a394: 7 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a394() {
}

// 0xd2a3a8 — __ZN4Ogre14OverlayElement9setHeightEf
#[doc(alias = "Ogre::OverlayElement::setHeight(float)")]
// was: Ogre::OverlayElement::setHeight(float)
// IDA 0xd2a3a8: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a3a8() {
}

// 0xd2a3d8 — __ZNK4Ogre14OverlayElement9getHeightEv
#[doc(alias = "Ogre::OverlayElement::getHeight(void)const")]
// was: Ogre::OverlayElement::getHeight(void)const
// IDA 0xd2a3d8: 7 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a3d8() {
}

// 0xd2a3ec — __ZN4Ogre14OverlayElement7setLeftEf
#[doc(alias = "Ogre::OverlayElement::setLeft(float)")]
// was: Ogre::OverlayElement::setLeft(float)
// IDA 0xd2a3ec: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a3ec() {
}

// 0xd2a41c — __ZNK4Ogre14OverlayElement7getLeftEv
#[doc(alias = "Ogre::OverlayElement::getLeft(void)const")]
// was: Ogre::OverlayElement::getLeft(void)const
// IDA 0xd2a41c: 7 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a41c() {
}

// 0xd2a430 — __ZN4Ogre14OverlayElement6setTopEf
#[doc(alias = "Ogre::OverlayElement::setTop(float)")]
// was: Ogre::OverlayElement::setTop(float)
// IDA 0xd2a430: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a430() {
}

// 0xd2a460 — __ZNK4Ogre14OverlayElement6getTopEv
#[doc(alias = "Ogre::OverlayElement::getTop(void)const")]
// was: Ogre::OverlayElement::getTop(void)const
// IDA 0xd2a460: 7 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a460() {
}

// 0xd2a474 — __ZNK4Ogre14OverlayElement15getMaterialNameEv
#[doc(alias = "Ogre::OverlayElement::getMaterialName(void)const")]
// was: Ogre::OverlayElement::getMaterialName(void)const
// IDA 0xd2a474: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a474() {
}

// 0xd2a478 — __ZN4Ogre14OverlayElement15setMaterialNameERKSs
#[doc(alias = "Ogre::OverlayElement::setMaterialName(std::string const&)")]
// was: Ogre::OverlayElement::setMaterialName(std::string const&)
// IDA 0xd2a478: 400 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a478() {
}

// 0xd2a8b4 — __ZNK4Ogre14OverlayElement11getMaterialEv
#[doc(alias = "Ogre::OverlayElement::getMaterial(void)const")]
// was: Ogre::OverlayElement::getMaterial(void)const
// IDA 0xd2a8b4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a8b4() {
}

// 0xd2a8b8 — __ZThn12_NK4Ogre14OverlayElement11getMaterialEv
#[doc(alias = "non-virtual thunk toOgre::OverlayElement::getMaterial(void)const")]
// was: non-virtual thunk to Ogre::OverlayElement::getMaterial(void)const
// IDA 0xd2a8b8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a8b8() {
}

// 0xd2a8bc — __ZNK4Ogre14OverlayElement18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::OverlayElement::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::OverlayElement::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xd2a8bc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a8bc() {
}

// 0xd2a8cc — __ZThn12_NK4Ogre14OverlayElement18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::OverlayElement::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: non-virtual thunk to Ogre::OverlayElement::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xd2a8cc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a8cc() {
}

// 0xd2a8dc — __ZN4Ogre14OverlayElement19_positionsOutOfDateEv
#[doc(alias = "Ogre::OverlayElement::_positionsOutOfDate(void)")]
// was: Ogre::OverlayElement::_positionsOutOfDate(void)
// IDA 0xd2a8dc: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a8dc() {
}

// 0xd2a8e4 — __ZN4Ogre14OverlayElement7_updateEv
#[doc(alias = "Ogre::OverlayElement::_update(void)")]
// was: Ogre::OverlayElement::_update(void)
// IDA 0xd2a8e4: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2a8e4() {
}

// 0xd2aa58 — __ZN4Ogre14OverlayElement17_updateFromParentEv
#[doc(alias = "Ogre::OverlayElement::_updateFromParent(void)")]
// was: Ogre::OverlayElement::_updateFromParent(void)
// IDA 0xd2aa58: 175 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2aa58() {
}

// 0xd2ac8c — __ZN4Ogre14OverlayElement13_notifyParentEPNS_16OverlayContainerEPNS_7OverlayE
#[doc(alias = "Ogre::OverlayElement::_notifyParent(Ogre::OverlayContainer *,Ogre::Overlay *)")]
// was: Ogre::OverlayElement::_notifyParent(Ogre::OverlayContainer *,Ogre::Overlay *)
// IDA 0xd2ac8c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ac8c() {
}

// 0xd2acbc — __ZN4Ogre14OverlayElement15_getDerivedLeftEv
#[doc(alias = "Ogre::OverlayElement::_getDerivedLeft(void)")]
// was: Ogre::OverlayElement::_getDerivedLeft(void)
// IDA 0xd2acbc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2acbc() {
}

// 0xd2acd8 — __ZN4Ogre14OverlayElement14_getDerivedTopEv
#[doc(alias = "Ogre::OverlayElement::_getDerivedTop(void)")]
// was: Ogre::OverlayElement::_getDerivedTop(void)
// IDA 0xd2acd8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2acd8() {
}

// 0xd2acf4 — __ZN4Ogre14OverlayElement17_getRelativeWidthEv
#[doc(alias = "Ogre::OverlayElement::_getRelativeWidth(void)")]
// was: Ogre::OverlayElement::_getRelativeWidth(void)
// IDA 0xd2acf4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2acf4() {
}

// 0xd2acf8 — __ZN4Ogre14OverlayElement18_getRelativeHeightEv
#[doc(alias = "Ogre::OverlayElement::_getRelativeHeight(void)")]
// was: Ogre::OverlayElement::_getRelativeHeight(void)
// IDA 0xd2acf8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2acf8() {
}

// 0xd2acfc — __ZN4Ogre14OverlayElement18_getClippingRegionERNS_9RectangleE
#[doc(alias = "Ogre::OverlayElement::_getClippingRegion(Ogre::Rectangle &)")]
// was: Ogre::OverlayElement::_getClippingRegion(Ogre::Rectangle &)
// IDA 0xd2acfc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2acfc() {
}

// 0xd2ad20 — __ZN4Ogre14OverlayElement13_notifyZOrderEt
#[doc(alias = "Ogre::OverlayElement::_notifyZOrder(unsigned short)")]
// was: Ogre::OverlayElement::_notifyZOrder(unsigned short)
// IDA 0xd2ad20: 4 insns (STRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ad20() {
}

// 0xd2ad2c — __ZN4Ogre14OverlayElement22_notifyWorldTransformsERKNS_7Matrix4E
#[doc(alias = "Ogre::OverlayElement::_notifyWorldTransforms(Ogre::Matrix4 const&)")]
// was: Ogre::OverlayElement::_notifyWorldTransforms(Ogre::Matrix4 const&)
// IDA 0xd2ad2c: 16 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ad2c() {
}

// 0xd2ad68 — __ZN4Ogre14OverlayElement15_notifyViewportEv
#[doc(alias = "Ogre::OverlayElement::_notifyViewport(void)")]
// was: Ogre::OverlayElement::_notifyViewport(void)
// IDA 0xd2ad68: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ad68() {
}

// 0xd2ae7c — __ZN4Ogre14OverlayElement18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::OverlayElement::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::OverlayElement::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xd2ae7c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ae7c() {
}

// 0xd2aea0 — __ZN4Ogre14OverlayElement16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::OverlayElement::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::OverlayElement::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xd2aea0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2aea0() {
}

// 0xd2aec4 — __ZN4Ogre14OverlayElement17addBaseParametersEv
#[doc(alias = "Ogre::OverlayElement::addBaseParameters(void)")]
// was: Ogre::OverlayElement::addBaseParameters(void)
// IDA 0xd2aec4: 2281 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2aec4() {
}

// 0xd2c950 — __ZN4Ogre14OverlayElement10setCaptionERKNS_9UTFStringE
#[doc(alias = "Ogre::OverlayElement::setCaption(Ogre::UTFString const&)")]
// was: Ogre::OverlayElement::setCaption(Ogre::UTFString const&)
// IDA 0xd2c950: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2c950() {
}

// 0xd2c968 — __ZNK4Ogre14OverlayElement10getCaptionEv
#[doc(alias = "Ogre::OverlayElement::getCaption(void)const")]
// was: Ogre::OverlayElement::getCaption(void)const
// IDA 0xd2c968: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2c968() {
}

// 0xd2c96c — __ZN4Ogre14OverlayElement9setColourERKNS_11ColourValueE
#[doc(alias = "Ogre::OverlayElement::setColour(Ogre::ColourValue const&)")]
// was: Ogre::OverlayElement::setColour(Ogre::ColourValue const&)
// IDA 0xd2c96c: 4 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2c96c() {
}

// 0xd2c978 — __ZNK4Ogre14OverlayElement9getColourEv
#[doc(alias = "Ogre::OverlayElement::getColour(void)const")]
// was: Ogre::OverlayElement::getColour(void)const
// IDA 0xd2c978: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2c978() {
}

// 0xd2c97c — __ZN4Ogre14OverlayElement14setMetricsModeENS_14GuiMetricsModeE
#[doc(alias = "Ogre::OverlayElement::setMetricsMode(Ogre::GuiMetricsMode)")]
// was: Ogre::OverlayElement::setMetricsMode(Ogre::GuiMetricsMode)
// IDA 0xd2c97c: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2c97c() {
}

// 0xd2cafc — __ZNK4Ogre14OverlayElement14getMetricsModeEv
#[doc(alias = "Ogre::OverlayElement::getMetricsMode(void)const")]
// was: Ogre::OverlayElement::getMetricsMode(void)const
// IDA 0xd2cafc: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cafc() {
}

// 0xd2cb04 — __ZN4Ogre14OverlayElement22setHorizontalAlignmentENS_22GuiHorizontalAlignmentE
#[doc(alias = "Ogre::OverlayElement::setHorizontalAlignment(Ogre::GuiHorizontalAlignment)")]
// was: Ogre::OverlayElement::setHorizontalAlignment(Ogre::GuiHorizontalAlignment)
// IDA 0xd2cb04: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb04() {
}

// 0xd2cb14 — __ZNK4Ogre14OverlayElement22getHorizontalAlignmentEv
#[doc(alias = "Ogre::OverlayElement::getHorizontalAlignment(void)const")]
// was: Ogre::OverlayElement::getHorizontalAlignment(void)const
// IDA 0xd2cb14: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb14() {
}

// 0xd2cb1c — __ZN4Ogre14OverlayElement20setVerticalAlignmentENS_20GuiVerticalAlignmentE
#[doc(alias = "Ogre::OverlayElement::setVerticalAlignment(Ogre::GuiVerticalAlignment)")]
// was: Ogre::OverlayElement::setVerticalAlignment(Ogre::GuiVerticalAlignment)
// IDA 0xd2cb1c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb1c() {
}

// 0xd2cb2c — __ZNK4Ogre14OverlayElement20getVerticalAlignmentEv
#[doc(alias = "Ogre::OverlayElement::getVerticalAlignment(void)const")]
// was: Ogre::OverlayElement::getVerticalAlignment(void)const
// IDA 0xd2cb2c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb2c() {
}

// 0xd2cb34 — __ZNK4Ogre14OverlayElement8containsEff
#[doc(alias = "Ogre::OverlayElement::contains(float,float)const")]
// was: Ogre::OverlayElement::contains(float,float)const
// IDA 0xd2cb34: 24 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb34() {
}

// 0xd2cb80 — __ZN4Ogre14OverlayElement13findElementAtEff
#[doc(alias = "Ogre::OverlayElement::findElementAt(float,float)")]
// was: Ogre::OverlayElement::findElementAt(float,float)
// IDA 0xd2cb80: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb80() {
}

// 0xd2cb98 — __ZN4Ogre14OverlayElement9getParentEv
#[doc(alias = "Ogre::OverlayElement::getParent(void)")]
// was: Ogre::OverlayElement::getParent(void)
// IDA 0xd2cb98: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cb98() {
}

// 0xd2cba0 — __ZN4Ogre14OverlayElement16copyFromTemplateEPS0_
#[doc(alias = "Ogre::OverlayElement::copyFromTemplate(Ogre::OverlayElement*)")]
// was: Ogre::OverlayElement::copyFromTemplate(Ogre::OverlayElement*)
// IDA 0xd2cba0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cba0() {
}

// 0xd2cbb8 — __ZN4Ogre14OverlayElement5cloneERKSs
#[doc(alias = "Ogre::OverlayElement::clone(std::string const&)")]
// was: Ogre::OverlayElement::clone(std::string const&)
// IDA 0xd2cbb8: 214 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2cbb8() {
}

// 0xd2ce18 — __ZNK4Ogre14OverlayElement9isEnabledEv
#[doc(alias = "Ogre::OverlayElement::isEnabled(void)const")]
// was: Ogre::OverlayElement::isEnabled(void)const
// IDA 0xd2ce18: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ce18() {
}

// 0xd2ce20 — __ZN4Ogre14OverlayElement10setEnabledEb
#[doc(alias = "Ogre::OverlayElement::setEnabled(bool)")]
// was: Ogre::OverlayElement::setEnabled(bool)
// IDA 0xd2ce20: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d2ce20() {
}

// 0xd2ce28 — __ZN4Ogre22OverlayElementCommands7CmdLeftD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdLeft::~CmdLeft()")]
// was: Ogre::OverlayElementCommands::CmdLeft::~CmdLeft()
// IDA 0xd2ce28: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce28() {
}

// 0xd2ce2c — __ZN4Ogre22OverlayElementCommands6CmdTopD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdTop::~CmdTop()")]
// was: Ogre::OverlayElementCommands::CmdTop::~CmdTop()
// IDA 0xd2ce2c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce2c() {
}

// 0xd2ce30 — __ZN4Ogre22OverlayElementCommands8CmdWidthD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdWidth::~CmdWidth()")]
// was: Ogre::OverlayElementCommands::CmdWidth::~CmdWidth()
// IDA 0xd2ce30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce30() {
}

// 0xd2ce34 — __ZN4Ogre22OverlayElementCommands9CmdHeightD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdHeight::~CmdHeight()")]
// was: Ogre::OverlayElementCommands::CmdHeight::~CmdHeight()
// IDA 0xd2ce34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce34() {
}

// 0xd2ce38 — __ZN4Ogre22OverlayElementCommands11CmdMaterialD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdMaterial::~CmdMaterial()")]
// was: Ogre::OverlayElementCommands::CmdMaterial::~CmdMaterial()
// IDA 0xd2ce38: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce38() {
}

// 0xd2ce3c — __ZN4Ogre22OverlayElementCommands10CmdCaptionD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdCaption::~CmdCaption()")]
// was: Ogre::OverlayElementCommands::CmdCaption::~CmdCaption()
// IDA 0xd2ce3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce3c() {
}

// 0xd2ce40 — __ZN4Ogre22OverlayElementCommands14CmdMetricsModeD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdMetricsMode::~CmdMetricsMode()")]
// was: Ogre::OverlayElementCommands::CmdMetricsMode::~CmdMetricsMode()
// IDA 0xd2ce40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce40() {
}

// 0xd2ce44 — __ZN4Ogre22OverlayElementCommands18CmdHorizontalAlignD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdHorizontalAlign::~CmdHorizontalAlign()")]
// was: Ogre::OverlayElementCommands::CmdHorizontalAlign::~CmdHorizontalAlign()
// IDA 0xd2ce44: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce44() {
}

// 0xd2ce48 — __ZN4Ogre22OverlayElementCommands16CmdVerticalAlignD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdVerticalAlign::~CmdVerticalAlign()")]
// was: Ogre::OverlayElementCommands::CmdVerticalAlign::~CmdVerticalAlign()
// IDA 0xd2ce48: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce48() {
}

// 0xd2ce4c — __ZN4Ogre22OverlayElementCommands10CmdVisibleD1Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdVisible::~CmdVisible()")]
// was: Ogre::OverlayElementCommands::CmdVisible::~CmdVisible()
// IDA 0xd2ce4c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d2ce4c() {
}

// 0xd2ce50 — __ZN4Ogre22OverlayElementCommands10CmdVisibleD0Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdVisible::~CmdVisible()")]
// was: Ogre::OverlayElementCommands::CmdVisible::~CmdVisible()
// IDA 0xd2ce50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2ce50() {
}

// 0xd2ce5c — __ZN4Ogre22OverlayElementCommands16CmdVerticalAlignD0Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdVerticalAlign::~CmdVerticalAlign()")]
// was: Ogre::OverlayElementCommands::CmdVerticalAlign::~CmdVerticalAlign()
// IDA 0xd2ce5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2ce5c() {
}

// 0xd2ce68 — __ZN4Ogre22OverlayElementCommands18CmdHorizontalAlignD0Ev
#[doc(alias = "Ogre::OverlayElementCommands::CmdHorizontalAlign::~CmdHorizontalAlign()")]
// was: Ogre::OverlayElementCommands::CmdHorizontalAlign::~CmdHorizontalAlign()
// IDA 0xd2ce68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d2ce68() {
}
