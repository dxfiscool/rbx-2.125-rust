//! rendering generated_54 — Ogre::|G3D:: strict 13333 total, 6568 prior, 100 this batch — 0xdb7d50..0xdba958
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Filter: demangled contains Ogre::|G3D:: (EA-sorted ascending after 0xdb7ba0)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xdb7d50 — __ZN4Ogre24VisibleObjectsBoundsInfo28mergeNonRenderedButInFrustumERKNS_14AxisAlignedBoxERKNS_6SphereEPKNS_6CameraE
// type: int __fastcall(int, int, int, Ogre::Camera *this)
#[doc(alias = "Ogre::VisibleObjectsBoundsInfo::mergeNonRenderedButInFrustum(Ogre::AxisAlignedBox const&,Ogre::Sphere const&,Ogre::Camera const*)")]
// was: Ogre::VisibleObjectsBoundsInfo::mergeNonRenderedButInFrustum(Ogre::AxisAlignedBox const&,Ogre::Sphere const&,Ogre::Camera const*)
// IDA 0xdb7d50: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db7d50() {
}

// 0xdb7e90 — __ZN4Ogre12SceneManager31SceneMgrQueuedRenderableVisitorD1Ev
// type: void __fastcall(Ogre::SceneManager::SceneMgrQueuedRenderableVisitor *__hidden this)
#[doc(alias = "Ogre::SceneManager::SceneMgrQueuedRenderableVisitor::~SceneMgrQueuedRenderableVisitor()")]
// was: Ogre::SceneManager::SceneMgrQueuedRenderableVisitor::~SceneMgrQueuedRenderableVisitor()
// IDA 0xdb7e90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_db7e90() {
}

// 0xdb7e94 — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::ShadowCameraSetup>::~SharedPtr()
// IDA 0xdb7e94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_db7e94() {
}

// 0xdb7f84 — __ZNSt3mapIPKN4Ogre6CameraENS0_24VisibleObjectsBoundsInfoESt4lessIS3_ENS0_12STLAllocatorISt4pairIKS3_S4_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS9_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int)
#[doc(alias = "std::map<Ogre::Camera const*,Ogre::VisibleObjectsBoundsInfo,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::VisibleObjectsBoundsInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Camera const* const&)")]
// was: std::map<Ogre::Camera const*,Ogre::VisibleObjectsBoundsInfo,std::less<Ogre::Camera const*>,Ogre::STLAllocator<std::pair<Ogre::Camera const* const,Ogre::VisibleObjectsBoundsInfo>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Camera const* const&)
// IDA 0xdb7f84: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db7f84() {
}

// 0xdb8104 — __ZN4Ogre24VisibleObjectsBoundsInfoD1Ev
// type: void __fastcall(Ogre::VisibleObjectsBoundsInfo *__hidden this)
#[doc(alias = "Ogre::VisibleObjectsBoundsInfo::~VisibleObjectsBoundsInfo()")]
// was: Ogre::VisibleObjectsBoundsInfo::~VisibleObjectsBoundsInfo()
// IDA 0xdb8104: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_db8104() {
}

// 0xdb81a8 — __ZN4Ogre13NameGenerator8generateEv
// type: _DWORD __fastcall(Ogre::NameGenerator *__hidden this)
#[doc(alias = "Ogre::NameGenerator::generate(void)")]
// was: Ogre::NameGenerator::generate(void)
// IDA 0xdb81a8: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db81a8() {
}

// 0xdb838c — __ZNSt3mapISsPN4Ogre9SceneNodeESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::SceneNode *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneNode *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::SceneNode *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneNode *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xdb838c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db838c() {
}

// 0xdb8548 — __ZNSt3mapISsPN4Ogre13MovableObjectESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::MovableObject *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::MovableObject *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xdb8548: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8548() {
}

// 0xdb8704 — __ZNSt6vectorIN4Ogre12SceneManager9LightInfoENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias = "std::vector<Ogre::SceneManager::LightInfo,Ogre::STLAllocator<Ogre::SceneManager::LightInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// was: std::vector<Ogre::SceneManager::LightInfo,Ogre::STLAllocator<Ogre::SceneManager::LightInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)
// IDA 0xdb8704: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8704() {
}

// 0xdb8798 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::operator=(Ogre::SharedPtr<Ogre::GpuProgramParameters> const&)")]
// was: Ogre::SharedPtr<Ogre::GpuProgramParameters>::operator=(Ogre::SharedPtr<Ogre::GpuProgramParameters> const&)
// IDA 0xdb8798: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8798() {
}

// 0xdb88a4 — __ZN4Ogre9SharedPtrINS_17ShadowCameraSetupEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::ShadowCameraSetup>::operator=(Ogre::SharedPtr<Ogre::ShadowCameraSetup> const&)")]
// was: Ogre::SharedPtr<Ogre::ShadowCameraSetup>::operator=(Ogre::SharedPtr<Ogre::ShadowCameraSetup> const&)
// IDA 0xdb88a4: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db88a4() {
}

// 0xdb8a24 — __ZNSt3mapISsPN4Ogre14StaticGeometryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::StaticGeometry *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xdb8a24: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8a24() {
}

// 0xdb8be0 — __ZNSt3mapISsPN4Ogre17InstancedGeometryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::InstancedGeometry *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::InstancedGeometry *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xdb8be0: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8be0() {
}

// 0xdb8d9c — __ZNSt3mapISsPN4Ogre15InstanceManagerESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::InstanceManager *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::InstanceManager *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xdb8d9c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8d9c() {
}

// 0xdb8f58 — __ZNSt3mapISsPN4Ogre12SceneManager23MovableObjectCollectionESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<std::string,Ogre::SceneManager::MovableObjectCollection *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::SceneManager::MovableObjectCollection *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xdb8f58: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db8f58() {
}

// 0xdb9114 — __ZN4Ogre14AxisAlignedBox5mergeERKS0_
#[doc(alias = "Ogre::AxisAlignedBox::merge(Ogre::AxisAlignedBox const&)")]
// was: Ogre::AxisAlignedBox::merge(Ogre::AxisAlignedBox const&)
// IDA 0xdb9114: 73 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9114() {
}

// 0xdb91f0 — __ZN4Ogre12SceneManager31SceneMgrQueuedRenderableVisitorD0Ev
// type: void __fastcall(Ogre::SceneManager::SceneMgrQueuedRenderableVisitor *__hidden this)
#[doc(alias = "Ogre::SceneManager::SceneMgrQueuedRenderableVisitor::~SceneMgrQueuedRenderableVisitor()")]
// was: Ogre::SceneManager::SceneMgrQueuedRenderableVisitor::~SceneMgrQueuedRenderableVisitor()
// IDA 0xdb91f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_db91f0() {
}

// 0xdb91fc — __ZN4Ogre12SceneManager30ShadowCasterSceneQueryListenerD1Ev
// type: void __fastcall(Ogre::SceneManager::ShadowCasterSceneQueryListener *__hidden this)
#[doc(alias = "Ogre::SceneManager::ShadowCasterSceneQueryListener::~ShadowCasterSceneQueryListener()")]
// was: Ogre::SceneManager::ShadowCasterSceneQueryListener::~ShadowCasterSceneQueryListener()
// IDA 0xdb91fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_db91fc() {
}

// 0xdb9200 — __ZN4Ogre12SceneManager30ShadowCasterSceneQueryListenerD0Ev
// type: void __fastcall(Ogre::SceneManager::ShadowCasterSceneQueryListener *__hidden this)
#[doc(alias = "Ogre::SceneManager::ShadowCasterSceneQueryListener::~ShadowCasterSceneQueryListener()")]
// was: Ogre::SceneManager::ShadowCasterSceneQueryListener::~ShadowCasterSceneQueryListener()
// IDA 0xdb9200: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_db9200() {
}

// 0xdb928c — __ZNK4Ogre12SceneManager25createAutoParamDataSourceEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::createAutoParamDataSource(void)const")]
// was: Ogre::SceneManager::createAutoParamDataSource(void)const
// IDA 0xdb928c: 65 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db928c() {
}

// 0xdb9348 — __ZN4Ogre12SceneManager21estimateWorldGeometryERKSs
#[doc(alias = "Ogre::SceneManager::estimateWorldGeometry(std::string const&)")]
// was: Ogre::SceneManager::estimateWorldGeometry(std::string const&)
// IDA 0xdb9348: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9348() {
}

// 0xdb934c — __ZN4Ogre12SceneManager21estimateWorldGeometryERNS_9SharedPtrINS_10DataStreamEEERKSs
#[doc(alias = "Ogre::SceneManager::estimateWorldGeometry(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)")]
// was: Ogre::SceneManager::estimateWorldGeometry(Ogre::SharedPtr<Ogre::DataStream> &,std::string const&)
// IDA 0xdb934c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db934c() {
}

// 0xdb9350 — __ZN4Ogre12SceneManager9setOptionERKSsPKv
#[doc(alias = "Ogre::SceneManager::setOption(std::string const&,void const*)")]
// was: Ogre::SceneManager::setOption(std::string const&,void const*)
// IDA 0xdb9350: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9350() {
}

// 0xdb9354 — __ZN4Ogre12SceneManager9getOptionERKSsPv
#[doc(alias = "Ogre::SceneManager::getOption(std::string const&,void *)")]
// was: Ogre::SceneManager::getOption(std::string const&,void *)
// IDA 0xdb9354: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9354() {
}

// 0xdb9358 — __ZNK4Ogre12SceneManager9hasOptionERKSs
#[doc(alias = "Ogre::SceneManager::hasOption(std::string const&)const")]
// was: Ogre::SceneManager::hasOption(std::string const&)const
// IDA 0xdb9358: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9358() {
}

// 0xdb935c — __ZN4Ogre12SceneManager15getOptionValuesERKSsRSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::SceneManager::getOptionValues(std::string const&,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// was: Ogre::SceneManager::getOptionValues(std::string const&,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)
// IDA 0xdb935c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db935c() {
}

// 0xdb9360 — __ZN4Ogre12SceneManager13getOptionKeysERSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::SceneManager::getOptionKeys(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// was: Ogre::SceneManager::getOptionKeys(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)
// IDA 0xdb9360: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9360() {
}

// 0xdb9364 — __ZN4Ogre12SceneManager18setSkyPlaneEnabledEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setSkyPlaneEnabled(bool)")]
// was: Ogre::SceneManager::setSkyPlaneEnabled(bool)
// IDA 0xdb9364: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9364() {
}

// 0xdb936c — __ZNK4Ogre12SceneManager17isSkyPlaneEnabledEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isSkyPlaneEnabled(void)const")]
// was: Ogre::SceneManager::isSkyPlaneEnabled(void)const
// IDA 0xdb936c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db936c() {
}

// 0xdb9374 — __ZNK4Ogre12SceneManager15getSkyPlaneNodeEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getSkyPlaneNode(void)const")]
// was: Ogre::SceneManager::getSkyPlaneNode(void)const
// IDA 0xdb9374: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9374() {
}

// 0xdb937c — __ZNK4Ogre12SceneManager24getSkyPlaneGenParametersEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getSkyPlaneGenParameters(void)const")]
// was: Ogre::SceneManager::getSkyPlaneGenParameters(void)const
// IDA 0xdb937c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db937c() {
}

// 0xdb9384 — __ZN4Ogre12SceneManager16setSkyBoxEnabledEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setSkyBoxEnabled(bool)")]
// was: Ogre::SceneManager::setSkyBoxEnabled(bool)
// IDA 0xdb9384: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9384() {
}

// 0xdb938c — __ZNK4Ogre12SceneManager15isSkyBoxEnabledEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isSkyBoxEnabled(void)const")]
// was: Ogre::SceneManager::isSkyBoxEnabled(void)const
// IDA 0xdb938c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db938c() {
}

// 0xdb9394 — __ZNK4Ogre12SceneManager13getSkyBoxNodeEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getSkyBoxNode(void)const")]
// was: Ogre::SceneManager::getSkyBoxNode(void)const
// IDA 0xdb9394: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9394() {
}

// 0xdb939c — __ZNK4Ogre12SceneManager22getSkyBoxGenParametersEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getSkyBoxGenParameters(void)const")]
// was: Ogre::SceneManager::getSkyBoxGenParameters(void)const
// IDA 0xdb939c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db939c() {
}

// 0xdb93a4 — __ZN4Ogre12SceneManager17setSkyDomeEnabledEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setSkyDomeEnabled(bool)")]
// was: Ogre::SceneManager::setSkyDomeEnabled(bool)
// IDA 0xdb93a4: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93a4() {
}

// 0xdb93ac — __ZNK4Ogre12SceneManager16isSkyDomeEnabledEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isSkyDomeEnabled(void)const")]
// was: Ogre::SceneManager::isSkyDomeEnabled(void)const
// IDA 0xdb93ac: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93ac() {
}

// 0xdb93b4 — __ZNK4Ogre12SceneManager14getSkyDomeNodeEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getSkyDomeNode(void)const")]
// was: Ogre::SceneManager::getSkyDomeNode(void)const
// IDA 0xdb93b4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93b4() {
}

// 0xdb93bc — __ZNK4Ogre12SceneManager23getSkyDomeGenParametersEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getSkyDomeGenParameters(void)const")]
// was: Ogre::SceneManager::getSkyDomeGenParameters(void)const
// IDA 0xdb93bc: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93bc() {
}

// 0xdb93c4 — __ZNK4Ogre12SceneManager20getDisplaySceneNodesEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getDisplaySceneNodes(void)const")]
// was: Ogre::SceneManager::getDisplaySceneNodes(void)const
// IDA 0xdb93c4: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93c4() {
}

// 0xdb93cc — __ZNK4Ogre12SceneManager18getShadowTechniqueEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowTechnique(void)const")]
// was: Ogre::SceneManager::getShadowTechnique(void)const
// IDA 0xdb93cc: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93cc() {
}

// 0xdb93d4 — __ZN4Ogre12SceneManager19setShowDebugShadowsEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setShowDebugShadows(bool)")]
// was: Ogre::SceneManager::setShowDebugShadows(bool)
// IDA 0xdb93d4: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93d4() {
}

// 0xdb93dc — __ZNK4Ogre12SceneManager19getShowDebugShadowsEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShowDebugShadows(void)const")]
// was: Ogre::SceneManager::getShowDebugShadows(void)const
// IDA 0xdb93dc: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93dc() {
}

// 0xdb93e4 — __ZNK4Ogre12SceneManager20getShadowFarDistanceEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowFarDistance(void)const")]
// was: Ogre::SceneManager::getShadowFarDistance(void)const
// IDA 0xdb93e4: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93e4() {
}

// 0xdb93ec — __ZNK4Ogre12SceneManager27getShadowFarDistanceSquaredEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowFarDistanceSquared(void)const")]
// was: Ogre::SceneManager::getShadowFarDistanceSquared(void)const
// IDA 0xdb93ec: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93ec() {
}

// 0xdb93f4 — __ZNK4Ogre12SceneManager24getShadowIndexBufferSizeEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowIndexBufferSize(void)const")]
// was: Ogre::SceneManager::getShadowIndexBufferSize(void)const
// IDA 0xdb93f4: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93f4() {
}

// 0xdb93fc — __ZN4Ogre12SceneManager30setShadowDirLightTextureOffsetEf
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, float)
#[doc(alias = "Ogre::SceneManager::setShadowDirLightTextureOffset(float)")]
// was: Ogre::SceneManager::setShadowDirLightTextureOffset(float)
// IDA 0xdb93fc: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db93fc() {
}

// 0xdb9404 — __ZNK4Ogre12SceneManager30getShadowDirLightTextureOffsetEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowDirLightTextureOffset(void)const")]
// was: Ogre::SceneManager::getShadowDirLightTextureOffset(void)const
// IDA 0xdb9404: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9404() {
}

// 0xdb940c — __ZN4Ogre12SceneManager25setShadowTextureFadeStartEf
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, float)
#[doc(alias = "Ogre::SceneManager::setShadowTextureFadeStart(float)")]
// was: Ogre::SceneManager::setShadowTextureFadeStart(float)
// IDA 0xdb940c: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db940c() {
}

// 0xdb9414 — __ZN4Ogre12SceneManager23setShadowTextureFadeEndEf
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, float)
#[doc(alias = "Ogre::SceneManager::setShadowTextureFadeEnd(float)")]
// was: Ogre::SceneManager::setShadowTextureFadeEnd(float)
// IDA 0xdb9414: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9414() {
}

// 0xdb941c — __ZNK4Ogre12SceneManager26getShadowTextureSelfShadowEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowTextureSelfShadow(void)const")]
// was: Ogre::SceneManager::getShadowTextureSelfShadow(void)const
// IDA 0xdb941c: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db941c() {
}

// 0xdb9424 — __ZN4Ogre12SceneManager30setShadowCasterRenderBackFacesEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setShadowCasterRenderBackFaces(bool)")]
// was: Ogre::SceneManager::setShadowCasterRenderBackFaces(bool)
// IDA 0xdb9424: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9424() {
}

// 0xdb942c — __ZNK4Ogre12SceneManager30getShadowCasterRenderBackFacesEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowCasterRenderBackFaces(void)const")]
// was: Ogre::SceneManager::getShadowCasterRenderBackFaces(void)const
// IDA 0xdb942c: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db942c() {
}

// 0xdb9434 — __ZN4Ogre12SceneManager28setShadowUseInfiniteFarPlaneEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setShadowUseInfiniteFarPlane(bool)")]
// was: Ogre::SceneManager::setShadowUseInfiniteFarPlane(bool)
// IDA 0xdb9434: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9434() {
}

// 0xdb943c — __ZNK4Ogre12SceneManager29isShadowTechniqueStencilBasedEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isShadowTechniqueStencilBased(void)const")]
// was: Ogre::SceneManager::isShadowTechniqueStencilBased(void)const
// IDA 0xdb943c: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db943c() {
}

// 0xdb944c — __ZNK4Ogre12SceneManager29isShadowTechniqueTextureBasedEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isShadowTechniqueTextureBased(void)const")]
// was: Ogre::SceneManager::isShadowTechniqueTextureBased(void)const
// IDA 0xdb944c: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db944c() {
}

// 0xdb945c — __ZNK4Ogre12SceneManager27isShadowTechniqueModulativeEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isShadowTechniqueModulative(void)const")]
// was: Ogre::SceneManager::isShadowTechniqueModulative(void)const
// IDA 0xdb945c: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db945c() {
}

// 0xdb946c — __ZNK4Ogre12SceneManager25isShadowTechniqueAdditiveEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isShadowTechniqueAdditive(void)const")]
// was: Ogre::SceneManager::isShadowTechniqueAdditive(void)const
// IDA 0xdb946c: 4 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db946c() {
}

// 0xdb9478 — __ZNK4Ogre12SceneManager27isShadowTechniqueIntegratedEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isShadowTechniqueIntegrated(void)const")]
// was: Ogre::SceneManager::isShadowTechniqueIntegrated(void)const
// IDA 0xdb9478: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9478() {
}

// 0xdb9488 — __ZNK4Ogre12SceneManager22isShadowTechniqueInUseEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isShadowTechniqueInUse(void)const")]
// was: Ogre::SceneManager::isShadowTechniqueInUse(void)const
// IDA 0xdb9488: 6 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9488() {
}

// 0xdb9498 — __ZN4Ogre12SceneManager27setShadowUseLightClipPlanesEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setShadowUseLightClipPlanes(bool)")]
// was: Ogre::SceneManager::setShadowUseLightClipPlanes(bool)
// IDA 0xdb9498: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9498() {
}

// 0xdb94a0 — __ZNK4Ogre12SceneManager27getShadowUseLightClipPlanesEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getShadowUseLightClipPlanes(void)const")]
// was: Ogre::SceneManager::getShadowUseLightClipPlanes(void)const
// IDA 0xdb94a0: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94a0() {
}

// 0xdb94a8 — __ZN4Ogre12SceneManager25_setActiveCompositorChainEPNS_15CompositorChainE
#[doc(alias = "Ogre::SceneManager::_setActiveCompositorChain(Ogre::CompositorChain *)")]
// was: Ogre::SceneManager::_setActiveCompositorChain(Ogre::CompositorChain *)
// IDA 0xdb94a8: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94a8() {
}

// 0xdb94b0 — __ZN4Ogre12SceneManager24setLateMaterialResolvingEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setLateMaterialResolving(bool)")]
// was: Ogre::SceneManager::setLateMaterialResolving(bool)
// IDA 0xdb94b0: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94b0() {
}

// 0xdb94b8 — __ZNK4Ogre12SceneManager23isLateMaterialResolvingEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::isLateMaterialResolving(void)const")]
// was: Ogre::SceneManager::isLateMaterialResolving(void)const
// IDA 0xdb94b8: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94b8() {
}

// 0xdb94c0 — __ZNK4Ogre12SceneManager25_getActiveCompositorChainEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::_getActiveCompositorChain(void)const")]
// was: Ogre::SceneManager::_getActiveCompositorChain(void)const
// IDA 0xdb94c0: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94c0() {
}

// 0xdb94c8 — __ZN4Ogre12SceneManager17setVisibilityMaskEj
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::SceneManager::setVisibilityMask(unsigned int)")]
// was: Ogre::SceneManager::setVisibilityMask(unsigned int)
// IDA 0xdb94c8: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94c8() {
}

// 0xdb94d0 — __ZN4Ogre12SceneManager17getVisibilityMaskEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getVisibilityMask(void)")]
// was: Ogre::SceneManager::getVisibilityMask(void)
// IDA 0xdb94d0: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94d0() {
}

// 0xdb94d8 — __ZN4Ogre12SceneManager21setFindVisibleObjectsEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setFindVisibleObjects(bool)")]
// was: Ogre::SceneManager::setFindVisibleObjects(bool)
// IDA 0xdb94d8: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94d8() {
}

// 0xdb94e0 — __ZN4Ogre12SceneManager21getFindVisibleObjectsEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getFindVisibleObjects(void)")]
// was: Ogre::SceneManager::getFindVisibleObjects(void)
// IDA 0xdb94e0: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94e0() {
}

// 0xdb94e8 — __ZN4Ogre12SceneManager26setNormaliseNormalsOnScaleEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setNormaliseNormalsOnScale(bool)")]
// was: Ogre::SceneManager::setNormaliseNormalsOnScale(bool)
// IDA 0xdb94e8: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94e8() {
}

// 0xdb94f0 — __ZNK4Ogre12SceneManager26getNormaliseNormalsOnScaleEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getNormaliseNormalsOnScale(void)const")]
// was: Ogre::SceneManager::getNormaliseNormalsOnScale(void)const
// IDA 0xdb94f0: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94f0() {
}

// 0xdb94f8 — __ZN4Ogre12SceneManager29setFlipCullingOnNegativeScaleEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setFlipCullingOnNegativeScale(bool)")]
// was: Ogre::SceneManager::setFlipCullingOnNegativeScale(bool)
// IDA 0xdb94f8: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db94f8() {
}

// 0xdb9500 — __ZNK4Ogre12SceneManager29getFlipCullingOnNegativeScaleEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getFlipCullingOnNegativeScale(void)const")]
// was: Ogre::SceneManager::getFlipCullingOnNegativeScale(void)const
// IDA 0xdb9500: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9500() {
}

// 0xdb9508 — __ZNK4Ogre12SceneManager32_areRenderStateChangesSuppressedEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::_areRenderStateChangesSuppressed(void)const")]
// was: Ogre::SceneManager::_areRenderStateChangesSuppressed(void)const
// IDA 0xdb9508: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9508() {
}

// 0xdb9510 — __ZNK4Ogre12SceneManager21_areShadowsSuppressedEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::_areShadowsSuppressed(void)const")]
// was: Ogre::SceneManager::_areShadowsSuppressed(void)const
// IDA 0xdb9510: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9510() {
}

// 0xdb9518 — __ZN4Ogre12SceneManager26setCameraRelativeRenderingEb
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, bool)
#[doc(alias = "Ogre::SceneManager::setCameraRelativeRendering(bool)")]
// was: Ogre::SceneManager::setCameraRelativeRendering(bool)
// IDA 0xdb9518: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9518() {
}

// 0xdb9520 — __ZNK4Ogre12SceneManager26getCameraRelativeRenderingEv
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this)
#[doc(alias = "Ogre::SceneManager::getCameraRelativeRendering(void)const")]
// was: Ogre::SceneManager::getCameraRelativeRendering(void)const
// IDA 0xdb9520: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9520() {
}

// 0xdb9528 — __ZNSt6vectorIN4Ogre29EntityMaterialLodChangedEventENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::EntityMaterialLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMaterialLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EntityMaterialLodChangedEvent*,std::vector<Ogre::EntityMaterialLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMaterialLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EntityMaterialLodChangedEvent const&)")]
// was: std::vector<Ogre::EntityMaterialLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMaterialLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EntityMaterialLodChangedEvent*,std::vector<Ogre::EntityMaterialLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMaterialLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EntityMaterialLodChangedEvent const&)
// IDA 0xdb9528: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_db9528() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xdb964c — __ZNSt6vectorIN4Ogre25EntityMeshLodChangedEventENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::EntityMeshLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMeshLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EntityMeshLodChangedEvent*,std::vector<Ogre::EntityMeshLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMeshLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EntityMeshLodChangedEvent const&)")]
// was: std::vector<Ogre::EntityMeshLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMeshLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::EntityMeshLodChangedEvent*,std::vector<Ogre::EntityMeshLodChangedEvent,Ogre::STLAllocator<Ogre::EntityMeshLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::EntityMeshLodChangedEvent const&)
// IDA 0xdb964c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_db964c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xdb9770 — __ZNSt6vectorIN4Ogre28MovableObjectLodChangedEventENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::MovableObjectLodChangedEvent,Ogre::STLAllocator<Ogre::MovableObjectLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MovableObjectLodChangedEvent*,std::vector<Ogre::MovableObjectLodChangedEvent,Ogre::STLAllocator<Ogre::MovableObjectLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MovableObjectLodChangedEvent const&)")]
// was: std::vector<Ogre::MovableObjectLodChangedEvent,Ogre::STLAllocator<Ogre::MovableObjectLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MovableObjectLodChangedEvent*,std::vector<Ogre::MovableObjectLodChangedEvent,Ogre::STLAllocator<Ogre::MovableObjectLodChangedEvent,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MovableObjectLodChangedEvent const&)
// IDA 0xdb9770: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_db9770() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xdb988c — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre13MovableObjectEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObject *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObject *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObject *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObject *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xdb988c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db988c() {
}

// 0xdb9930 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre13MovableObjectEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObject *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObject *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MovableObject *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObject *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObject *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::MovableObject *>> *)
// IDA 0xdb9930: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9930() {
}

// 0xdb99a8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre13MovableObjectEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObject *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObject *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::MovableObject *>,std::_Select1st<std::pair<std::string const,Ogre::MovableObject *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::MovableObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xdb99a8: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db99a8() {
}

// 0xdb9a4c — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre12SceneManager23MovableObjectCollectionEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xdb9a4c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9a4c() {
}

// 0xdb9af0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12SceneManager23MovableObjectCollectionEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *> const&)
// IDA 0xdb9af0: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9af0() {
}

// 0xdb9cd0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12SceneManager23MovableObjectCollectionEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *> const&)
// IDA 0xdb9cd0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9cd0() {
}

// 0xdb9e24 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12SceneManager23MovableObjectCollectionEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *> const&)
// IDA 0xdb9e24: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9e24() {
}

// 0xdb9f08 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12SceneManager23MovableObjectCollectionEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,std::_Select1st<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SceneManager::MovableObjectCollection *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xdb9f08: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9f08() {
}

// 0xdb9fac — __ZNSt6vectorIPN4Ogre15InstanceManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_range_insertIN9__gnu_cxx17__normal_iteratorIPS2_S8_EEEEvSD_T_SE_St20forward_iterator_tag
// type: int __fastcall(int, void *__dst, void *__src)
#[doc(alias = "void std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_range_insert<__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::forward_iterator_tag)")]
// was: void std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_range_insert<__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::forward_iterator_tag)
// IDA 0xdb9fac: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_db9fac() {
}

// 0xdba16c — __ZNSt6vectorIPN4Ogre15InstanceManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: _DWORD *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstanceManager * const&)")]
// was: std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstanceManager **,std::vector<Ogre::InstanceManager *,Ogre::STLAllocator<Ogre::InstanceManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstanceManager * const&)
// IDA 0xdba16c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_dba16c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xdba264 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15InstanceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstanceManager *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstanceManager *>> *)
// IDA 0xdba264: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba264() {
}

// 0xdba2dc — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre15InstanceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xdba2dc: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba2dc() {
}

// 0xdba380 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15InstanceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstanceManager *>>,std::pair<std::string const,Ogre::InstanceManager *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstanceManager *>>,std::pair<std::string const,Ogre::InstanceManager *> const&)
// IDA 0xdba380: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba380() {
}

// 0xdba560 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15InstanceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstanceManager *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstanceManager *> const&)
// IDA 0xdba560: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba560() {
}

// 0xdba6b4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15InstanceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstanceManager *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstanceManager *> const&)
// IDA 0xdba6b4: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba6b4() {
}

// 0xdba798 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre15InstanceManagerEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstanceManager *>,std::_Select1st<std::pair<std::string const,Ogre::InstanceManager *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstanceManager *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xdba798: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba798() {
}

// 0xdba83c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstancedGeometry *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstancedGeometry *>> *)
// IDA 0xdba83c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba83c() {
}

// 0xdba8b4 — __ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const
// IDA 0xdba8b4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba8b4() {
}

// 0xdba958 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::pair<std::string const,Ogre::InstancedGeometry *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry *>>,std::pair<std::string const,Ogre::InstancedGeometry *> const&)
// IDA 0xdba958: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_dba958() {
}
