//! rendering — next 120 Ogre/G3D stubs (EA-sorted filter Ogre/G3D)
//! Filter: Ogre|G3D (13663 total, 1816 prior stubbed, +120 this batch) — 0xcc66c0..0xccd04c after 0xcc658c (remaining 11727 after batch)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcc66c0 — __ZN4Ogre17InstancedGeometry5resetEv
#[doc(alias = "Ogre::InstancedGeometry::reset(void)")]
// was: Ogre::InstancedGeometry::reset(void)
// IDA 0xcc66c0: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc66c0() {
}

// 0xcc68c0 — __ZN4Ogre17InstancedGeometry10setVisibleEb
#[doc(alias = "Ogre::InstancedGeometry::setVisible(bool)")]
// was: Ogre::InstancedGeometry::setVisible(bool)
// IDA 0xcc68c0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc68c0() {
}

// 0xcc68f0 — __ZN4Ogre17InstancedGeometry14setCastShadowsEb
#[doc(alias = "Ogre::InstancedGeometry::setCastShadows(bool)")]
// was: Ogre::InstancedGeometry::setCastShadows(bool)
// IDA 0xcc68f0: 15 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc68f0() {
}

// 0xcc6918 — __ZN4Ogre17InstancedGeometry19setRenderQueueGroupEh
#[doc(alias = "Ogre::InstancedGeometry::setRenderQueueGroup(unsigned char)")]
// was: Ogre::InstancedGeometry::setRenderQueueGroup(unsigned char)
// IDA 0xcc6918: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc6918() {
}

// 0xcc694c — __ZNK4Ogre17InstancedGeometry19getRenderQueueGroupEv
#[doc(alias = "Ogre::InstancedGeometry::getRenderQueueGroup(void)const")]
// was: Ogre::InstancedGeometry::getRenderQueueGroup(void)const
// IDA 0xcc694c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc694c() {
}

// 0xcc6954 — __ZNK4Ogre17InstancedGeometry4dumpERKSs
#[doc(alias = "Ogre::InstancedGeometry::dump(std::string const&)const")]
// was: Ogre::InstancedGeometry::dump(std::string const&)const
// IDA 0xcc6954: 528 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc6954() {
}

// 0xcc6f38 — __ZNK4Ogre17InstancedGeometry13BatchInstance4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
// was: Ogre::InstancedGeometry::BatchInstance::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xcc6f38: 252 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc6f38() {
}

// 0xcc721c — __ZN4Ogre17InstancedGeometry23setProvideWorldInversesEb
#[doc(alias = "Ogre::InstancedGeometry::setProvideWorldInverses(bool)")]
// was: Ogre::InstancedGeometry::setProvideWorldInverses(bool)
// IDA 0xcc721c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc721c() {
}

// 0xcc7224 — __ZN4Ogre17InstancedGeometry15InstancedObjectC2EtPNS_16SkeletonInstanceEPNS_17AnimationStateSetE
#[doc(alias = "Ogre::InstancedGeometry::InstancedObject::InstancedObject(unsigned short,Ogre::SkeletonInstance *,Ogre::AnimationStateSet *)")]
// was: Ogre::InstancedGeometry::InstancedObject::InstancedObject(unsigned short,Ogre::SkeletonInstance *,Ogre::AnimationStateSet *)
// IDA 0xcc7224: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7224() {
}

// 0xcc74bc — __ZN4Ogre17InstancedGeometry15InstancedObjectD2Ev
#[doc(alias = "Ogre::InstancedGeometry::InstancedObject::~InstancedObject()")]
// was: Ogre::InstancedGeometry::InstancedObject::~InstancedObject()
// IDA 0xcc74bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc74bc() {
}

// 0xcc75b0 — __ZN4Ogre17InstancedGeometry15InstancedObject15updateAnimationEv
#[doc(alias = "Ogre::InstancedGeometry::InstancedObject::updateAnimation(void)")]
// was: Ogre::InstancedGeometry::InstancedObject::updateAnimation(void)
// IDA 0xcc75b0: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc75b0() {
}

// 0xcc7678 — __ZN4Ogre17InstancedGeometry13BatchInstanceD0Ev
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::~BatchInstance()")]
// was: Ogre::InstancedGeometry::BatchInstance::~BatchInstance()
// IDA 0xcc7678: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7678() {
}

// 0xcc7708 — __ZN4Ogre17InstancedGeometry13BatchInstanceD1Ev
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::~BatchInstance()")]
// was: Ogre::InstancedGeometry::BatchInstance::~BatchInstance()
// IDA 0xcc7708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7708() {
}

// 0xcc7714 — __ZThn4_N4Ogre17InstancedGeometry13BatchInstanceD0Ev
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::BatchInstance::~BatchInstance()")]
// was: non-virtual thunk to Ogre::InstancedGeometry::BatchInstance::~BatchInstance()
// IDA 0xcc7714: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7714() {
}

// 0xcc77a8 — __ZN4Ogre17InstancedGeometry13BatchInstanceD2Ev
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::~BatchInstance()")]
// was: Ogre::InstancedGeometry::BatchInstance::~BatchInstance()
// IDA 0xcc77a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc77a8() {
}

// 0xcc7a28 — __ZThn4_N4Ogre17InstancedGeometry13BatchInstanceD1Ev
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::BatchInstance::~BatchInstance()")]
// was: non-virtual thunk to Ogre::InstancedGeometry::BatchInstance::~BatchInstance()
// IDA 0xcc7a28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7a28() {
}

// 0xcc7a34 — __ZN4Ogre17InstancedGeometry9LODBucket6assignEPNS0_13QueuedSubMeshEt
#[doc(alias = "Ogre::InstancedGeometry::LODBucket::assign(Ogre::InstancedGeometry::QueuedSubMesh *,unsigned short)")]
// was: Ogre::InstancedGeometry::LODBucket::assign(Ogre::InstancedGeometry::QueuedSubMesh *,unsigned short)
// IDA 0xcc7a34: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7a34() {
}

// 0xcc7be8 — __ZNK4Ogre17InstancedGeometry13BatchInstance14getMovableTypeEv
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::getMovableType(void)const")]
// was: Ogre::InstancedGeometry::BatchInstance::getMovableType(void)const
// IDA 0xcc7be8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7be8() {
}

// 0xcc7cdc — __ZN4Ogre17InstancedGeometry13BatchInstance20_notifyCurrentCameraEPNS_6CameraE
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::_notifyCurrentCamera(Ogre::Camera *)")]
// was: Ogre::InstancedGeometry::BatchInstance::_notifyCurrentCamera(Ogre::Camera *)
// IDA 0xcc7cdc: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7cdc() {
}

// 0xcc7d34 — __ZNK4Ogre17InstancedGeometry13BatchInstance14getBoundingBoxEv
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::getBoundingBox(void)const")]
// was: Ogre::InstancedGeometry::BatchInstance::getBoundingBox(void)const
// IDA 0xcc7d34: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7d34() {
}

// 0xcc7d3c — __ZNK4Ogre17InstancedGeometry13BatchInstance17getBoundingRadiusEv
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::getBoundingRadius(void)const")]
// was: Ogre::InstancedGeometry::BatchInstance::getBoundingRadius(void)const
// IDA 0xcc7d3c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7d3c() {
}

// 0xcc7d44 — __ZN4Ogre17InstancedGeometry13BatchInstance18_updateRenderQueueEPNS_11RenderQueueE
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::_updateRenderQueue(Ogre::RenderQueue *)")]
// was: Ogre::InstancedGeometry::BatchInstance::_updateRenderQueue(Ogre::RenderQueue *)
// IDA 0xcc7d44: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7d44() {
}

// 0xcc7dac — __ZN4Ogre17InstancedGeometry13BatchInstance16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::InstancedGeometry::BatchInstance::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xcc7dac: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7dac() {
}

// 0xcc7e18 — __ZNK4Ogre17InstancedGeometry13BatchInstance9isVisibleEv
#[doc(alias = "Ogre::InstancedGeometry::BatchInstance::isVisible(void)const")]
// was: Ogre::InstancedGeometry::BatchInstance::isVisible(void)const
// IDA 0xcc7e18: 11 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7e18() {
}

// 0xcc7e30 — __ZNK4Ogre17InstancedGeometry9LODBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::InstancedGeometry::LODBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
// was: Ogre::InstancedGeometry::LODBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xcc7e30: 104 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc7e30() {
}

// 0xcc7f58 — __ZN4Ogre17InstancedGeometry9LODBucketD0Ev
#[doc(alias = "Ogre::InstancedGeometry::LODBucket::~LODBucket()")]
// was: Ogre::InstancedGeometry::LODBucket::~LODBucket()
// IDA 0xcc7f58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7f58() {
}

// 0xcc7fe8 — __ZN4Ogre17InstancedGeometry9LODBucketD1Ev
#[doc(alias = "Ogre::InstancedGeometry::LODBucket::~LODBucket()")]
// was: Ogre::InstancedGeometry::LODBucket::~LODBucket()
// IDA 0xcc7fe8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7fe8() {
}

// 0xcc7ff4 — __ZN4Ogre17InstancedGeometry9LODBucketD2Ev
#[doc(alias = "Ogre::InstancedGeometry::LODBucket::~LODBucket()")]
// was: Ogre::InstancedGeometry::LODBucket::~LODBucket()
// IDA 0xcc7ff4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc7ff4() {
}

// 0xcc8160 — __ZN4Ogre17InstancedGeometry14MaterialBucket6assignEPNS0_14QueuedGeometryE
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::assign(Ogre::InstancedGeometry::QueuedGeometry *)")]
// was: Ogre::InstancedGeometry::MaterialBucket::assign(Ogre::InstancedGeometry::QueuedGeometry *)
// IDA 0xcc8160: 291 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc8160() {
}

// 0xcc8490 — __ZN4Ogre17InstancedGeometry14MaterialBucket5buildEv
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::build(void)")]
// was: Ogre::InstancedGeometry::MaterialBucket::build(void)
// IDA 0xcc8490: 427 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc8490() {
}

// 0xcc891c — __ZN4Ogre17InstancedGeometry14MaterialBucket14addRenderablesEPNS_11RenderQueueEhf
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::addRenderables(Ogre::RenderQueue *,unsigned char,float)")]
// was: Ogre::InstancedGeometry::MaterialBucket::addRenderables(Ogre::RenderQueue *,unsigned char,float)
// IDA 0xcc891c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc891c() {
}

// 0xcc8998 — __ZNK4Ogre17InstancedGeometry14MaterialBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
// was: Ogre::InstancedGeometry::MaterialBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xcc8998: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc8998() {
}

// 0xcc8a88 — __ZN4Ogre17InstancedGeometry14MaterialBucketC2EPNS0_9LODBucketERKSs
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::MaterialBucket(Ogre::InstancedGeometry::LODBucket *,std::string const&)")]
// was: Ogre::InstancedGeometry::MaterialBucket::MaterialBucket(Ogre::InstancedGeometry::LODBucket *,std::string const&)
// IDA 0xcc8a88: 345 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc8a88() {
}

// 0xcc8df4 — __ZN4Ogre17InstancedGeometry14MaterialBucketD0Ev
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::~MaterialBucket()")]
// was: Ogre::InstancedGeometry::MaterialBucket::~MaterialBucket()
// IDA 0xcc8df4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc8df4() {
}

// 0xcc8e84 — __ZN4Ogre17InstancedGeometry14MaterialBucketD1Ev
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::~MaterialBucket()")]
// was: Ogre::InstancedGeometry::MaterialBucket::~MaterialBucket()
// IDA 0xcc8e84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc8e84() {
}

// 0xcc8e90 — __ZN4Ogre17InstancedGeometry14MaterialBucketD2Ev
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::~MaterialBucket()")]
// was: Ogre::InstancedGeometry::MaterialBucket::~MaterialBucket()
// IDA 0xcc8e90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cc8e90() {
}

// 0xcc9010 — __ZN4Ogre17InstancedGeometry14MaterialBucket23getGeometryFormatStringEPNS0_22SubMeshLodGeometryLinkE
#[doc(alias = "Ogre::InstancedGeometry::MaterialBucket::getGeometryFormatString(Ogre::InstancedGeometry::SubMeshLodGeometryLink *)")]
// was: Ogre::InstancedGeometry::MaterialBucket::getGeometryFormatString(Ogre::InstancedGeometry::SubMeshLodGeometryLink *)
// IDA 0xcc9010: 252 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc9010() {
}

// 0xcc92e4 — __ZN4Ogre17InstancedGeometry14GeometryBucket6assignEPNS0_14QueuedGeometryE
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::assign(Ogre::InstancedGeometry::QueuedGeometry *)")]
// was: Ogre::InstancedGeometry::GeometryBucket::assign(Ogre::InstancedGeometry::QueuedGeometry *)
// IDA 0xcc92e4: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc92e4() {
}

// 0xcc9360 — __ZN4Ogre17InstancedGeometry14GeometryBucket5buildEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::build(void)")]
// was: Ogre::InstancedGeometry::GeometryBucket::build(void)
// IDA 0xcc9360: 1313 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cc9360() {
}

// 0xcca044 — __ZNK4Ogre17InstancedGeometry14GeometryBucket4dumpERSt14basic_ofstreamIcSt11char_traitsIcEE
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::dump(std::basic_ofstream<char,std::char_traits<char>> &)const
// IDA 0xcca044: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cca044() {
}

// 0xcca14c — __ZN4Ogre17InstancedGeometry14GeometryBucketC2EPNS0_14MaterialBucketERKSsPKNS_10VertexDataEPKNS_9IndexDataE
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::GeometryBucket(Ogre::InstancedGeometry::MaterialBucket *,std::string const&,Ogre::VertexData const*,Ogre::IndexData const*)")]
// was: Ogre::InstancedGeometry::GeometryBucket::GeometryBucket(Ogre::InstancedGeometry::MaterialBucket *,std::string const&,Ogre::VertexData const*,Ogre::IndexData const*)
// IDA 0xcca14c: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cca14c() {
}

// 0xcca350 — __ZN4Ogre17InstancedGeometry14GeometryBucket19_initGeometryBucketEPKNS_10VertexDataEPKNS_9IndexDataE
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::_initGeometryBucket(Ogre::VertexData const*,Ogre::IndexData const*)")]
// was: Ogre::InstancedGeometry::GeometryBucket::_initGeometryBucket(Ogre::VertexData const*,Ogre::IndexData const*)
// IDA 0xcca350: 421 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cca350() {
}

// 0xcca778 — __ZN4Ogre17InstancedGeometry14GeometryBucketD0Ev
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xcca778: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cca778() {
}

// 0xcca808 — __ZN4Ogre17InstancedGeometry14GeometryBucketD1Ev
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xcca808: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cca808() {
}

// 0xcca814 — __ZThn4_N4Ogre17InstancedGeometry14GeometryBucketD0Ev
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xcca814: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cca814() {
}

// 0xcca8a8 — __ZThn188_N4Ogre17InstancedGeometry14GeometryBucketD0Ev
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xcca8a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cca8a8() {
}

// 0xcca93c — __ZN4Ogre17InstancedGeometry14GeometryBucketD2Ev
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xcca93c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cca93c() {
}

// 0xccaa50 — __ZThn4_N4Ogre17InstancedGeometry14GeometryBucketD1Ev
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xccaa50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccaa50() {
}

// 0xccaa5c — __ZThn188_N4Ogre17InstancedGeometry14GeometryBucketD1Ev
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::~GeometryBucket()")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::~GeometryBucket()
// IDA 0xccaa5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccaa5c() {
}

// 0xccaa68 — __ZNK4Ogre17InstancedGeometry14GeometryBucket17getBoundingRadiusEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getBoundingRadius(void)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getBoundingRadius(void)const
// IDA 0xccaa68: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccaa68() {
}

// 0xccaa70 — __ZNK4Ogre17InstancedGeometry14GeometryBucket11getMaterialEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getMaterial(void)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getMaterial(void)const
// IDA 0xccaa70: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccaa70() {
}

// 0xccaa78 — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket11getMaterialEv
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getMaterial(void)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getMaterial(void)const
// IDA 0xccaa78: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccaa78() {
}

// 0xccaa80 — __ZNK4Ogre17InstancedGeometry14GeometryBucket12getTechniqueEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getTechnique(void)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getTechnique(void)const
// IDA 0xccaa80: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccaa80() {
}

// 0xccaa88 — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket12getTechniqueEv
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getTechnique(void)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getTechnique(void)const
// IDA 0xccaa88: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccaa88() {
}

// 0xccaa90 — __ZNK4Ogre17InstancedGeometry14GeometryBucket18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xccaa90: 291 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccaa90() {
}

// 0xccae04 — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xccae04: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccae04() {
}

// 0xccae10 — __ZNK4Ogre17InstancedGeometry14GeometryBucket21getNumWorldTransformsEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getNumWorldTransforms(void)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getNumWorldTransforms(void)const
// IDA 0xccae10: 250 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccae10() {
}

// 0xccb078 — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket21getNumWorldTransformsEv
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getNumWorldTransforms(void)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getNumWorldTransforms(void)const
// IDA 0xccb078: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb078() {
}

// 0xccb084 — __ZNK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xccb084: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb084() {
}

// 0xccb0cc — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xccb0cc: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb0cc() {
}

// 0xccb114 — __ZNK4Ogre17InstancedGeometry14GeometryBucket9getLightsEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getLights(void)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getLights(void)const
// IDA 0xccb114: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb114() {
}

// 0xccb12c — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket9getLightsEv
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getLights(void)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getLights(void)const
// IDA 0xccb12c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb12c() {
}

// 0xccb144 — __ZNK4Ogre17InstancedGeometry14GeometryBucket15getCastsShadowsEv
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getCastsShadows(void)const")]
// was: Ogre::InstancedGeometry::GeometryBucket::getCastsShadows(void)const
// IDA 0xccb144: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb144() {
}

// 0xccb158 — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket15getCastsShadowsEv
#[doc(alias = "non-virtual thunk toOgre::InstancedGeometry::GeometryBucket::getCastsShadows(void)const")]
// was: non-virtual thunk to Ogre::InstancedGeometry::GeometryBucket::getCastsShadows(void)const
// IDA 0xccb158: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb158() {
}

// 0xccb16c — __ZN4Ogre17InstancedGeometry14GeometryBucket16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::InstancedGeometry::GeometryBucket::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xccb16c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb16c() {
}

// 0xccb194 — __ZNSt3mapISsPN4Ogre17InstancedGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<std::string,Ogre::InstancedGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::InstancedGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xccb194: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb194() {
}

// 0xccb350 — __ZNSt3mapISsPN4Ogre17InstancedGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<std::string,Ogre::InstancedGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::InstancedGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xccb350: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb350() {
}

// 0xccb50c — __ZN4Ogre17InstancedGeometry20setRenderingDistanceEf
#[doc(alias = "Ogre::InstancedGeometry::setRenderingDistance(float)")]
// was: Ogre::InstancedGeometry::setRenderingDistance(float)
// IDA 0xccb50c: 6 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb50c() {
}

// 0xccb520 — __ZNK4Ogre17InstancedGeometry20getRenderingDistanceEv
#[doc(alias = "Ogre::InstancedGeometry::getRenderingDistance(void)const")]
// was: Ogre::InstancedGeometry::getRenderingDistance(void)const
// IDA 0xccb520: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb520() {
}

// 0xccb524 — __ZNK4Ogre17InstancedGeometry27getSquaredRenderingDistanceEv
#[doc(alias = "Ogre::InstancedGeometry::getSquaredRenderingDistance(void)const")]
// was: Ogre::InstancedGeometry::getSquaredRenderingDistance(void)const
// IDA 0xccb524: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb524() {
}

// 0xccb528 — __ZNK4Ogre17InstancedGeometry9isVisibleEv
#[doc(alias = "Ogre::InstancedGeometry::isVisible(void)const")]
// was: Ogre::InstancedGeometry::isVisible(void)const
// IDA 0xccb528: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb528() {
}

// 0xccb530 — __ZN4Ogre17InstancedGeometry14getCastShadowsEv
#[doc(alias = "Ogre::InstancedGeometry::getCastShadows(void)")]
// was: Ogre::InstancedGeometry::getCastShadows(void)
// IDA 0xccb530: 2 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb530() {
}

// 0xccb534 — __ZN4Ogre17InstancedGeometry26setBatchInstanceDimensionsERKNS_7Vector3E
#[doc(alias = "Ogre::InstancedGeometry::setBatchInstanceDimensions(Ogre::Vector3 const&)")]
// was: Ogre::InstancedGeometry::setBatchInstanceDimensions(Ogre::Vector3 const&)
// IDA 0xccb534: 18 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb534() {
}

// 0xccb570 — __ZNK4Ogre17InstancedGeometry26getBatchInstanceDimensionsEv
#[doc(alias = "Ogre::InstancedGeometry::getBatchInstanceDimensions(void)const")]
// was: Ogre::InstancedGeometry::getBatchInstanceDimensions(void)const
// IDA 0xccb570: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb570() {
}

// 0xccb574 — __ZN4Ogre17InstancedGeometry9setOriginERKNS_7Vector3E
#[doc(alias = "Ogre::InstancedGeometry::setOrigin(Ogre::Vector3 const&)")]
// was: Ogre::InstancedGeometry::setOrigin(Ogre::Vector3 const&)
// IDA 0xccb574: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb574() {
}

// 0xccb584 — __ZNK4Ogre17InstancedGeometry9getOriginEv
#[doc(alias = "Ogre::InstancedGeometry::getOrigin(void)const")]
// was: Ogre::InstancedGeometry::getOrigin(void)const
// IDA 0xccb584: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb584() {
}

// 0xccb588 — __ZNK4Ogre17InstancedGeometry23getProvideWorldInversesEv
#[doc(alias = "Ogre::InstancedGeometry::getProvideWorldInverses(void)const")]
// was: Ogre::InstancedGeometry::getProvideWorldInverses(void)const
// IDA 0xccb588: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb588() {
}

// 0xccb590 — __ZNSt6vectorIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderOperation **,std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderOperation * const&)")]
// was: std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderOperation **,std::vector<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderOperation * const&)
// IDA 0xccb590: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ccb590() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xccb688 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xccb688: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb688() {
}

// 0xccb72c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xccb72c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccb72c() {
}

// 0xccb730 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xccb730: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccb730() {
}

// 0xccb73c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xccb73c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb73c() {
}

// 0xccb7e0 — __ZNSt6vectorIPN4Ogre17InstancedGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::QueuedGeometry **,std::vector<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::QueuedGeometry * const&)")]
// was: std::vector<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::QueuedGeometry **,std::vector<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::QueuedGeometry * const&)
// IDA 0xccb7e0: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ccb7e0() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xccb8d8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>> *)
// IDA 0xccb8d8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb8d8() {
}

// 0xccb950 — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccb950: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccb950() {
}

// 0xccb954 — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccb954: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccb954() {
}

// 0xccb960 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xccb960: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccb960() {
}

// 0xccb964 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xccb964: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccb964() {
}

// 0xccb970 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *> const&)
// IDA 0xccb970: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccb970() {
}

// 0xccbb78 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *> const&)
// IDA 0xccbb78: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccbb78() {
}

// 0xccbc74 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>> *)
// IDA 0xccbc74: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccbc74() {
}

// 0xccbc9c — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccbc9c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccbc9c() {
}

// 0xccbca0 — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccbca0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccbca0() {
}

// 0xccbcac — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xccbcac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccbcac() {
}

// 0xccbcb0 — __ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xccbcb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccbcb0() {
}

// 0xccbcbc — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccbcbc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccbcbc() {
}

// 0xccbcc0 — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccbcc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccbcc0() {
}

// 0xccbccc — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISF_E
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)
// IDA 0xccbccc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccbccc() {
}

// 0xccbcf4 — __ZNSt12_Vector_baseIN4Ogre17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccbcf4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccbcf4() {
}

// 0xccbcf8 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>> *)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>> *)
// IDA 0xccbcf8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccbcf8() {
}

// 0xccbd20 — __ZNSt6vectorIPN4Ogre17InstancedGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::GeometryBucket **,std::vector<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::GeometryBucket * const&)")]
// was: std::vector<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::GeometryBucket **,std::vector<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::GeometryBucket * const&)
// IDA 0xccbd20: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ccbd20() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xccbe18 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)
// IDA 0xccbe18: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccbe18() {
}

// 0xccbff8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)
// IDA 0xccbff8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccbff8() {
}

// 0xccc14c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)
// IDA 0xccc14c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccc14c() {
}

// 0xccc230 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)
// IDA 0xccc230: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccc230() {
}

// 0xccc410 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)
// IDA 0xccc410: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccc410() {
}

// 0xccc564 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)
// IDA 0xccc564: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccc564() {
}

// 0xccc648 — __ZNSt6vectorIPN4Ogre17InstancedGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::LODBucket **,std::vector<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::LODBucket * const&)")]
// was: std::vector<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::LODBucket **,std::vector<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::LODBucket * const&)
// IDA 0xccc648: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_ccc648() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xccc740 — __ZNSt6vectorIN4Ogre17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::InstancedGeometry::SubMeshLodGeometryLink const&)")]
// was: std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::InstancedGeometry::SubMeshLodGeometryLink const&)
// IDA 0xccc740: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccc740() {
}

// 0xccc928 — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISF_ERKSF_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xccc928: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ccc928() {
}

// 0xcccb30 — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueERKSF_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xcccb30: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cccb30() {
}

// 0xcccc2c — __ZNSt12_Vector_baseIN4Ogre17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xcccc2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cccc2c() {
}

// 0xcccc38 — __ZNSt6vectorIPN4Ogre17InstancedGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::QueuedSubMesh **,std::vector<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::QueuedSubMesh * const&)")]
// was: std::vector<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::QueuedSubMesh **,std::vector<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::QueuedSubMesh * const&)
// IDA 0xcccc38: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_cccc38() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xcccd30 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *> const&)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *> const&)
// IDA 0xcccd30: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cccd30() {
}

// 0xcccf38 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *> const&)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *> const&)
// IDA 0xcccf38: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cccf38() {
}

// 0xccd034 — __ZNSt12_Vector_baseIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccd034: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccd034() {
}

// 0xccd038 — __ZNSt12_Vector_baseIPN4Ogre17InstancedGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccd038: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccd038() {
}

// 0xccd03c — __ZNSt12_Vector_baseIPN4Ogre15RenderOperationENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RenderOperation *,Ogre::STLAllocator<Ogre::RenderOperation *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xccd03c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccd03c() {
}

// 0xccd048 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()
// IDA 0xccd048: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ccd048() {
}

// 0xccd04c — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()
// IDA 0xccd04c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ccd04c() {
}