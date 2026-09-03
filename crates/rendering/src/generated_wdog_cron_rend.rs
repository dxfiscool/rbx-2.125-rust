//! rendering — wdog cron rend — 120 stubs EA-sorted asc Ogre|G3D|Gfx|Render global-deduped
//! Range: 0xb71000..0xbb1c18 (120 stubs, EA-sorted asc, distinct not yet in /tmp/global_eas.txt)
//! Source: ida/export.json (85545 funcs, 15058 Ogre|G3D|Gfx|Render total, 5138 already stubbed, 9920 remaining before -> 9800 after)
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0xb71000 — __ZNK4Ogre16ShadowRenderable19getSquaredViewDepthEPKNS_6CameraE
// type: int()
#[doc(alias = "Ogre::ShadowRenderable::getSquaredViewDepth(Ogre::Camera const*)const")]
#[doc(alias = "__ZNK4Ogre16ShadowRenderable19getSquaredViewDepthEPKNS_6CameraE")]
pub fn stub_b71000() -> ! {
    todo!("0xb71000 Ogre::ShadowRenderable::getSquaredViewDepth(Ogre::Camera const*)const")
}

// 0xb71008 — __ZNK4Ogre16ShadowRenderable9isVisibleEv
// type: int __fastcall(Ogre::ShadowRenderable *this)
#[doc(alias = "Ogre::ShadowRenderable::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre16ShadowRenderable9isVisibleEv")]
pub fn stub_b71008() -> ! {
    todo!("0xb71008 Ogre::ShadowRenderable::isVisible(void)const")
}

// 0xb71010 — __ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb
// type: void()
#[doc(alias = "RBX::GfxBinding::updateChunk(RBX::SpatialRegion::Id const&,bool)")]
#[doc(alias = "__ZN3RBX10GfxBinding11updateChunkERKNS_13SpatialRegion2IdEb")]
pub fn stub_b71010() -> ! {
    todo!("0xb71010 RBX::GfxBinding::updateChunk(RBX::SpatialRegion::Id const&,bool)")
}

// 0xb71018 — __ZN3RBX10GfxBinding13onSizeChangedEv
// type: int __fastcall(RBX::GfxBinding *this)
#[doc(alias = "RBX::GfxBinding::onSizeChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding13onSizeChangedEv")]
pub fn stub_b71018() -> ! {
    todo!("0xb71018 RBX::GfxBinding::onSizeChanged(void)")
}

// 0xb71020 — __ZN3RBX10GfxBinding21onTransparencyChangedEv
// type: int __fastcall(RBX::GfxBinding *this)
#[doc(alias = "RBX::GfxBinding::onTransparencyChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding21onTransparencyChangedEv")]
pub fn stub_b71020() -> ! {
    todo!("0xb71020 RBX::GfxBinding::onTransparencyChanged(void)")
}

// 0xb71028 — __ZN3RBX10GfxBinding21onSpecialShapeChangedEv
// type: int __fastcall(RBX::GfxBinding *this)
#[doc(alias = "RBX::GfxBinding::onSpecialShapeChanged(void)")]
#[doc(alias = "__ZN3RBX10GfxBinding21onSpecialShapeChangedEv")]
pub fn stub_b71028() -> ! {
    todo!("0xb71028 RBX::GfxBinding::onSpecialShapeChanged(void)")
}

// 0xb720f0 — __ZNSt12_Vector_baseIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: int __fastcall(int)
#[doc(alias = "std::_Vector_base<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
pub fn stub_b720f0() -> ! {
    todo!("0xb720f0 std::_Vector_base<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xb731b0 — __ZN3RBX24FastClusterMeshGenerator9getBoundsERKN3G3D7Vector3ES4_
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, const Vector3 *, const G3D::Vector3 *, Ogre *)
#[doc(alias = "RBX::FastClusterMeshGenerator::getBounds(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator9getBoundsERKN3G3D7Vector3ES4_")]
pub fn stub_b731b0() -> ! {
    todo!("0xb731b0 RBX::FastClusterMeshGenerator::getBounds(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0xb7406c — __ZNK3G3D15CoordinateFrame13toObjectSpaceERKS0_
// type: int __fastcall(G3D::CoordinateFrame *this, const G3D::CoordinateFrame *, _DWORD *)
#[doc(alias = "G3D::CoordinateFrame::toObjectSpace(G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3G3D15CoordinateFrame13toObjectSpaceERKS0_")]
pub fn stub_b7406c() -> ! {
    todo!("0xb7406c G3D::CoordinateFrame::toObjectSpace(G3D::CoordinateFrame const&)const")
}

// 0xb74238 — __ZN4Ogre28HardwareIndexBufferSharedPtrD1Ev
// type: void __fastcall(Ogre::HardwareIndexBufferSharedPtr *__hidden this)
#[doc(alias = "Ogre::HardwareIndexBufferSharedPtr::~HardwareIndexBufferSharedPtr()")]
#[doc(alias = "__ZN4Ogre28HardwareIndexBufferSharedPtrD1Ev")]
pub fn stub_b74238() -> ! {
    todo!("0xb74238 Ogre::HardwareIndexBufferSharedPtr::~HardwareIndexBufferSharedPtr()")
}

// 0xb74290 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED0Ev")]
pub fn stub_b74290() -> ! {
    todo!("0xb74290 Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")
}

// 0xb74330 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE7destroyEv
// type: int __fastcall(int, void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE7destroyEv")]
pub fn stub_b74330() -> ! {
    todo!("0xb74330 Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::destroy(void)")
}

// 0xb74368 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareIndexBuffer>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE4swapERS2_")]
pub fn stub_b74368() -> ! {
    todo!("0xb74368 Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareIndexBuffer>&)")
}

// 0xb76a08 — __ZN4Ogre16ShadowRenderableD1Ev
// type: void __fastcall(Ogre::ShadowRenderable *__hidden this)
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "__ZN4Ogre16ShadowRenderableD1Ev")]
pub fn stub_b76a08() -> ! {
    todo!("0xb76a08 Ogre::ShadowRenderable::~ShadowRenderable()")
}

// 0xb9a050 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv
// type: void __fastcall(int)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv")]
pub fn stub_b9a050() -> ! {
    todo!("0xb9a050 Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::destroy(void)")
}

// 0xb9a150 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::swap(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE4swapERS2_")]
pub fn stub_b9a150() -> ! {
    todo!("0xb9a150 Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::swap(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>&)")
}

// 0xb9a170 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::swap(Ogre::SharedPtr<Ogre::GpuNamedConstants>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE4swapERS2_")]
pub fn stub_b9a170() -> ! {
    todo!("0xb9a170 Ogre::SharedPtr<Ogre::GpuNamedConstants>::swap(Ogre::SharedPtr<Ogre::GpuNamedConstants>&)")
}

// 0xb9a190 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev")]
pub fn stub_b9a190() -> ! {
    todo!("0xb9a190 Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")
}

// 0xb9a230 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv
// type: int __fastcall(int, void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv")]
pub fn stub_b9a230() -> ! {
    todo!("0xb9a230 Ogre::SharedPtr<Ogre::GpuSharedParameters>::destroy(void)")
}

// 0xb9a268 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::swap(Ogre::SharedPtr<Ogre::GpuSharedParameters>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_")]
pub fn stub_b9a268() -> ! {
    todo!("0xb9a268 Ogre::SharedPtr<Ogre::GpuSharedParameters>::swap(Ogre::SharedPtr<Ogre::GpuSharedParameters>&)")
}

// 0xb9a284 — __ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE
// type: RBX::ManualObjectMeshGenAdapter *__fastcall(RBX::ManualObjectMeshGenAdapter *this, Ogre::ManualObject *)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE")]
pub fn stub_b9a284() -> ! {
    todo!("0xb9a284 RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)")
}

// 0xb9a914 — __ZN3RBX7MeshGen21pushVerticesTransformERKN3G3D15CoordinateFrameE
// type: void()
#[doc(alias = "RBX::MeshGen::pushVerticesTransform(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX7MeshGen21pushVerticesTransformERKN3G3D15CoordinateFrameE")]
pub fn stub_b9a914() -> ! {
    todo!("0xb9a914 RBX::MeshGen::pushVerticesTransform(G3D::CoordinateFrame const&)")
}

// 0xb9a920 — __ZN4Ogre9SharedPtrINS_4MeshEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEED0Ev")]
pub fn stub_b9a920() -> ! {
    todo!("0xb9a920 Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")
}

// 0xb9a9c0 — __ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv
// type: int __fastcall(int, void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv")]
pub fn stub_b9a9c0() -> ! {
    todo!("0xb9a9c0 Ogre::SharedPtr<Ogre::Mesh>::destroy(void)")
}

// 0xb9a9f8 — __ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::swap(Ogre::SharedPtr<Ogre::Mesh>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_")]
pub fn stub_b9a9f8() -> ! {
    todo!("0xb9a9f8 Ogre::SharedPtr<Ogre::Mesh>::swap(Ogre::SharedPtr<Ogre::Mesh>&)")
}

// 0xb9aa20 — __ZN3RBX5Adorn16finishRenderPassEv
// type: void __fastcall(RBX::Adorn *this)
#[doc(alias = "RBX::Adorn::finishRenderPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn16finishRenderPassEv")]
pub fn stub_b9aa20() -> ! {
    todo!("0xb9aa20 RBX::Adorn::finishRenderPass(void)")
}

// 0xb9aa2c — __ZN4Ogre4Node11setListenerEPNS0_8ListenerE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::Node::setListener(Ogre::Node::Listener *)")]
#[doc(alias = "__ZN4Ogre4Node11setListenerEPNS0_8ListenerE")]
pub fn stub_b9aa2c() -> ! {
    todo!("0xb9aa2c Ogre::Node::setListener(Ogre::Node::Listener *)")
}

// 0xb9aa34 — __ZNK4Ogre4Node11getListenerEv
// type: int __fastcall(Ogre::Node *this)
#[doc(alias = "Ogre::Node::getListener(void)const")]
#[doc(alias = "__ZNK4Ogre4Node11getListenerEv")]
pub fn stub_b9aa34() -> ! {
    todo!("0xb9aa34 Ogre::Node::getListener(void)const")
}

// 0xb9aa3c — __ZN4Ogre4Node10setUserAnyERKNS_3AnyE
// type: void __fastcall(Ogre::Node *this, const Ogre::Any *)
#[doc(alias = "Ogre::Node::setUserAny(Ogre::Any const&)")]
#[doc(alias = "__ZN4Ogre4Node10setUserAnyERKNS_3AnyE")]
pub fn stub_b9aa3c() -> ! {
    todo!("0xb9aa3c Ogre::Node::setUserAny(Ogre::Any const&)")
}

// 0xb9aa44 — __ZNK4Ogre4Node10getUserAnyEv
// type: _DWORD *__fastcall(Ogre::Node *this)
#[doc(alias = "Ogre::Node::getUserAny(void)const")]
#[doc(alias = "__ZNK4Ogre4Node10getUserAnyEv")]
pub fn stub_b9aa44() -> ! {
    todo!("0xb9aa44 Ogre::Node::getUserAny(void)const")
}

// 0xb9aa4c — __ZNK4Ogre9SceneNode14isInSceneGraphEv
// type: int __fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::isInSceneGraph(void)const")]
#[doc(alias = "__ZNK4Ogre9SceneNode14isInSceneGraphEv")]
pub fn stub_b9aa4c() -> ! {
    todo!("0xb9aa4c Ogre::SceneNode::isInSceneGraph(void)const")
}

// 0xb9aa54 — __ZN4Ogre9SceneNode15_notifyRootNodeEv
// type: int __fastcall(int this)
#[doc(alias = "Ogre::SceneNode::_notifyRootNode(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode15_notifyRootNodeEv")]
pub fn stub_b9aa54() -> ! {
    todo!("0xb9aa54 Ogre::SceneNode::_notifyRootNode(void)")
}

// 0xb9aa5c — __ZN4Ogre9SceneNode18getAutoTrackTargetEv
// type: int __fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::getAutoTrackTarget(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode18getAutoTrackTargetEv")]
pub fn stub_b9aa5c() -> ! {
    todo!("0xb9aa5c Ogre::SceneNode::getAutoTrackTarget(void)")
}

// 0xb9aa64 — __ZN4Ogre9SceneNode18getAutoTrackOffsetEv
// type: char *__fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::getAutoTrackOffset(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode18getAutoTrackOffsetEv")]
pub fn stub_b9aa64() -> ! {
    todo!("0xb9aa64 Ogre::SceneNode::getAutoTrackOffset(void)")
}

// 0xb9aa6c — __ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv
// type: char *__fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::getAutoTrackLocalDirection(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv")]
pub fn stub_b9aa6c() -> ! {
    todo!("0xb9aa6c Ogre::SceneNode::getAutoTrackLocalDirection(void)")
}

// 0xb9b3bc — __ZN4Ogre17istreamDataStreamC1EPSib
// type: Ogre::istreamDataStream *__fastcall(Ogre::istreamDataStream *this, std::istream *, bool)
#[doc(alias = "Ogre::istreamDataStream::istreamDataStream(std::istream *,bool)")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamC1EPSib")]
pub fn stub_b9b3bc() -> ! {
    todo!("0xb9b3bc Ogre::istreamDataStream::istreamDataStream(std::istream *,bool)")
}

// 0xb9b52c — __ZN4Ogre17istreamDataStreamD0Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this)
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD0Ev")]
pub fn stub_b9b52c() -> ! {
    todo!("0xb9b52c Ogre::istreamDataStream::~istreamDataStream()")
}

// 0xb9b5e0 — __ZN4Ogre17istreamDataStreamD1Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this)
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD1Ev")]
pub fn stub_b9b5e0() -> ! {
    todo!("0xb9b5e0 Ogre::istreamDataStream::~istreamDataStream()")
}

// 0xb9b5e4 — __ZN4Ogre17istreamDataStreamD2Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this)
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD2Ev")]
pub fn stub_b9b5e4() -> ! {
    todo!("0xb9b5e4 Ogre::istreamDataStream::~istreamDataStream()")
}

// 0xb9b744 — __ZN4Ogre17istreamDataStream4readEPvm
// type: int __fastcall(std::istream **this, char *, int)
#[doc(alias = "Ogre::istreamDataStream::read(void *,unsigned long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4readEPvm")]
pub fn stub_b9b744() -> ! {
    todo!("0xb9b744 Ogre::istreamDataStream::read(void *,unsigned long)")
}

// 0xb9b758 — __ZN4Ogre17istreamDataStream8readLineEPcmRKSs
// type: unsigned int __fastcall(std::istream **this, char *, unsigned int, char **)
#[doc(alias = "Ogre::istreamDataStream::readLine(char *,unsigned long,std::string const&)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream8readLineEPcmRKSs")]
pub fn stub_b9b758() -> ! {
    todo!("0xb9b758 Ogre::istreamDataStream::readLine(char *,unsigned long,std::string const&)")
}

// 0xb9bbd0 — __ZN4Ogre17istreamDataStream4skipEl
// type: int __fastcall(Ogre::istreamDataStream *this, int)
#[doc(alias = "Ogre::istreamDataStream::skip(long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4skipEl")]
pub fn stub_b9bbd0() -> ! {
    todo!("0xb9bbd0 Ogre::istreamDataStream::skip(long)")
}

// 0xb9bbf8 — __ZN4Ogre17istreamDataStream4seekEm
// type: int __fastcall(Ogre::istreamDataStream *this, int)
#[doc(alias = "Ogre::istreamDataStream::seek(unsigned long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4seekEm")]
pub fn stub_b9bbf8() -> ! {
    todo!("0xb9bbf8 Ogre::istreamDataStream::seek(unsigned long)")
}

// 0xb9bc20 — __ZNK4Ogre17istreamDataStream4tellEv
// type: int __fastcall(Ogre::istreamDataStream *this)
#[doc(alias = "Ogre::istreamDataStream::tell(void)const")]
#[doc(alias = "__ZNK4Ogre17istreamDataStream4tellEv")]
pub fn stub_b9bc20() -> ! {
    todo!("0xb9bc20 Ogre::istreamDataStream::tell(void)const")
}

// 0xb9bc64 — __ZNK4Ogre17istreamDataStream3eofEv
// type: int __fastcall(Ogre::istreamDataStream *this)
#[doc(alias = "Ogre::istreamDataStream::eof(void)const")]
#[doc(alias = "__ZNK4Ogre17istreamDataStream3eofEv")]
pub fn stub_b9bc64() -> ! {
    todo!("0xb9bc64 Ogre::istreamDataStream::eof(void)const")
}

// 0xb9bc78 — __ZN4Ogre17istreamDataStream5closeEv
// type: int __fastcall(Ogre::istreamDataStream *this)
#[doc(alias = "Ogre::istreamDataStream::close(void)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream5closeEv")]
pub fn stub_b9bc78() -> ! {
    todo!("0xb9bc78 Ogre::istreamDataStream::close(void)")
}

// 0xb9c320 — __ZN4Ogre7QuadricC1Ev
// type: int __fastcall(int this)
#[doc(alias = "Ogre::Quadric::Quadric(void)")]
#[doc(alias = "__ZN4Ogre7QuadricC1Ev")]
pub fn stub_b9c320() -> ! {
    todo!("0xb9c320 Ogre::Quadric::Quadric(void)")
}

// 0xb9c344 — __ZN4Ogre7Quadric9setOriginERKNS_7Vector3E
// type: int __fastcall(int this, const Vector3 *)
#[doc(alias = "Ogre::Quadric::setOrigin(Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre7Quadric9setOriginERKNS_7Vector3E")]
pub fn stub_b9c344() -> ! {
    todo!("0xb9c344 Ogre::Quadric::setOrigin(Ogre::Vector3 const&)")
}

// 0xb9c358 — __ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, float32_t, float32_t, int, int)
#[doc(alias = "Ogre::Quadric::createCylinder(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii")]
pub fn stub_b9c358() -> ! {
    todo!("0xb9c358 Ogre::Quadric::createCylinder(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,float,int,int)")
}

// 0xb9e7e8 — __ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii
// type: int __fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float, float, int, int)
#[doc(alias = "Ogre::Quadric::createDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii")]
pub fn stub_b9e7e8() -> ! {
    todo!("0xb9e7e8 Ogre::Quadric::createDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int)")
}

// 0xba0b70 — __ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, int, int)
#[doc(alias = "Ogre::Quadric::createSphere(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii")]
pub fn stub_ba0b70() -> ! {
    todo!("0xba0b70 Ogre::Quadric::createSphere(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,int,int)")
}

// 0xba3c7c — __ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: _DWORD *__fastcall(int, char *, __int64 *)
#[doc(alias = "std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_ba3c7c() -> ! {
    todo!("0xba3c7c std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,std::allocator<Ogre::Vector3>>>,Ogre::Vector3 const&)")
}

// 0xba4494 — __ZNK4Ogre10RbxArchive15isCaseSensitiveEv
// type: int __fastcall(Ogre::RbxArchive *this)
#[doc(alias = "Ogre::RbxArchive::isCaseSensitive(void)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive15isCaseSensitiveEv")]
pub fn stub_ba4494() -> ! {
    todo!("0xba4494 Ogre::RbxArchive::isCaseSensitive(void)const")
}

// 0xba4498 — __ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
// type: void __fastcall(int, const char **, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, char, char, char, char, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "Ogre::RbxArchive::doStaticFindFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE")]
pub fn stub_ba4498() -> ! {
    todo!("0xba4498 Ogre::RbxArchive::doStaticFindFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")
}

// 0xba4a18 — __ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
// type: void __fastcall(struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int, int, int)
#[doc(alias = "Ogre::RbxArchive::findFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE")]
pub fn stub_ba4a18() -> ! {
    todo!("0xba4a18 Ogre::RbxArchive::findFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")
}

// 0xba5874 — __ZN4OgreL16concatenate_pathERKSsS1_
// type: void __fastcall(Ogre *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::concatenate_path(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4OgreL16concatenate_pathERKSsS1_")]
pub fn stub_ba5874() -> ! {
    todo!("0xba5874 Ogre::concatenate_path(std::string const&,std::string const&)")
}

// 0xba5a54 — __ZN4Ogre10RbxArchiveD0Ev
// type: void __fastcall(Ogre::RbxArchive *__hidden this)
#[doc(alias = "Ogre::RbxArchive::~RbxArchive()")]
#[doc(alias = "__ZN4Ogre10RbxArchiveD0Ev")]
pub fn stub_ba5a54() -> ! {
    todo!("0xba5a54 Ogre::RbxArchive::~RbxArchive()")
}

// 0xba5af0 — __ZN4Ogre10RbxArchiveD1Ev
// type: void __fastcall(Ogre::RbxArchive *__hidden this)
#[doc(alias = "Ogre::RbxArchive::~RbxArchive()")]
#[doc(alias = "__ZN4Ogre10RbxArchiveD1Ev")]
pub fn stub_ba5af0() -> ! {
    todo!("0xba5af0 Ogre::RbxArchive::~RbxArchive()")
}

// 0xba5b88 — __ZN4Ogre10RbxArchive4loadEv
// type: void __fastcall(Ogre::RbxArchive *this)
#[doc(alias = "Ogre::RbxArchive::load(void)")]
#[doc(alias = "__ZN4Ogre10RbxArchive4loadEv")]
pub fn stub_ba5b88() -> ! {
    todo!("0xba5b88 Ogre::RbxArchive::load(void)")
}

// 0xba5b8c — __ZN4Ogre10RbxArchive6unloadEv
// type: void __fastcall(Ogre::RbxArchive *this)
#[doc(alias = "Ogre::RbxArchive::unload(void)")]
#[doc(alias = "__ZN4Ogre10RbxArchive6unloadEv")]
pub fn stub_ba5b8c() -> ! {
    todo!("0xba5b8c Ogre::RbxArchive::unload(void)")
}

// 0xba5b90 — __ZNK4Ogre10RbxArchive4openERKSsb
// type: void __fastcall(Ogre::RbxArchive *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::RbxArchive::open(std::string const&,bool)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive4openERKSsb")]
pub fn stub_ba5b90() -> ! {
    todo!("0xba5b90 Ogre::RbxArchive::open(std::string const&,bool)const")
}

// 0xba6244 — __ZN4Ogre10RbxArchive4listEbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::list(bool,bool)")]
#[doc(alias = "__ZN4Ogre10RbxArchive4listEbb")]
pub fn stub_ba6244() -> ! {
    todo!("0xba6244 Ogre::RbxArchive::list(bool,bool)")
}

// 0xba6460 — __ZN4Ogre10RbxArchive12listFileInfoEbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::listFileInfo(bool,bool)")]
#[doc(alias = "__ZN4Ogre10RbxArchive12listFileInfoEbb")]
pub fn stub_ba6460() -> ! {
    todo!("0xba6460 Ogre::RbxArchive::listFileInfo(bool,bool)")
}

// 0xba667c — __ZN4Ogre10RbxArchive4findERKSsbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::find(std::string const&,bool,bool)")]
#[doc(alias = "__ZN4Ogre10RbxArchive4findERKSsbb")]
pub fn stub_ba667c() -> ! {
    todo!("0xba667c Ogre::RbxArchive::find(std::string const&,bool,bool)")
}

// 0xba67f8 — __ZNK4Ogre10RbxArchive12findFileInfoERKSsbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::findFileInfo(std::string const&,bool,bool)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive12findFileInfoERKSsbb")]
pub fn stub_ba67f8() -> ! {
    todo!("0xba67f8 Ogre::RbxArchive::findFileInfo(std::string const&,bool,bool)const")
}

// 0xba6974 — __ZN4Ogre10RbxArchive12makeFullPathERKSs
// type: void __fastcall(Ogre::RbxArchive *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::RbxArchive::makeFullPath(std::string const&)")]
#[doc(alias = "__ZN4Ogre10RbxArchive12makeFullPathERKSs")]
pub fn stub_ba6974() -> ! {
    todo!("0xba6974 Ogre::RbxArchive::makeFullPath(std::string const&)")
}

// 0xba6f04 — __ZN4Ogre10RbxArchive6existsERKSs
// type: bool __fastcall(Ogre::RbxArchive *this, const std::string *)
#[doc(alias = "Ogre::RbxArchive::exists(std::string const&)")]
#[doc(alias = "__ZN4Ogre10RbxArchive6existsERKSs")]
pub fn stub_ba6f04() -> ! {
    todo!("0xba6f04 Ogre::RbxArchive::exists(std::string const&)")
}

// 0xba7028 — __ZN4Ogre10RbxArchive15getModifiedTimeERKSs
// type: __darwin_time_t __fastcall(Ogre::RbxArchive *this, const std::string *)
#[doc(alias = "Ogre::RbxArchive::getModifiedTime(std::string const&)")]
#[doc(alias = "__ZN4Ogre10RbxArchive15getModifiedTimeERKSs")]
pub fn stub_ba7028() -> ! {
    todo!("0xba7028 Ogre::RbxArchive::getModifiedTime(std::string const&)")
}

// 0xba7150 — __ZN4Ogre17RbxArchiveFactoryC2EPN3RBX15ContentProviderE
// type: Ogre::RbxArchiveFactory *__fastcall(Ogre::RbxArchiveFactory *this, RBX::ContentProvider *)
#[doc(alias = "Ogre::RbxArchiveFactory::RbxArchiveFactory(RBX::ContentProvider *)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactoryC2EPN3RBX15ContentProviderE")]
pub fn stub_ba7150() -> ! {
    todo!("0xba7150 Ogre::RbxArchiveFactory::RbxArchiveFactory(RBX::ContentProvider *)")
}

// 0xba72c4 — __ZN4Ogre17RbxArchiveFactory18getArchiveTypeNameEPN3RBX15ContentProviderE
// type: void __fastcall(Ogre::RbxArchiveFactory *this, RBX::ContentProvider *)
#[doc(alias = "Ogre::RbxArchiveFactory::getArchiveTypeName(RBX::ContentProvider *)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory18getArchiveTypeNameEPN3RBX15ContentProviderE")]
pub fn stub_ba72c4() -> ! {
    todo!("0xba72c4 Ogre::RbxArchiveFactory::getArchiveTypeName(RBX::ContentProvider *)")
}

// 0xba74cc — __ZNK4Ogre17RbxArchiveFactory7getTypeEv
// type: char *__fastcall(Ogre::RbxArchiveFactory *this)
#[doc(alias = "Ogre::RbxArchiveFactory::getType(void)const")]
#[doc(alias = "__ZNK4Ogre17RbxArchiveFactory7getTypeEv")]
pub fn stub_ba74cc() -> ! {
    todo!("0xba74cc Ogre::RbxArchiveFactory::getType(void)const")
}

// 0xba74d0 — __ZN4Ogre17RbxArchiveFactory9singletonEv
// type: int __fastcall(Ogre::RbxArchiveFactory *this)
#[doc(alias = "Ogre::RbxArchiveFactory::singleton(void)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory9singletonEv")]
pub fn stub_ba74d0() -> ! {
    todo!("0xba74d0 Ogre::RbxArchiveFactory::singleton(void)")
}

// 0xba75a8 — __ZN4Ogre17RbxArchiveFactoryD0Ev
// type: void __fastcall(Ogre::RbxArchiveFactory *__hidden this)
#[doc(alias = "Ogre::RbxArchiveFactory::~RbxArchiveFactory()")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactoryD0Ev")]
pub fn stub_ba75a8() -> ! {
    todo!("0xba75a8 Ogre::RbxArchiveFactory::~RbxArchiveFactory()")
}

// 0xba7618 — __ZN4Ogre17RbxArchiveFactoryD1Ev
// type: void __fastcall(Ogre::RbxArchiveFactory *__hidden this)
#[doc(alias = "Ogre::RbxArchiveFactory::~RbxArchiveFactory()")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactoryD1Ev")]
pub fn stub_ba7618() -> ! {
    todo!("0xba7618 Ogre::RbxArchiveFactory::~RbxArchiveFactory()")
}

// 0xba7684 — __ZN4Ogre17RbxArchiveFactory10destroyAllEv
// type: int __fastcall(Ogre::RbxArchiveFactory *this)
#[doc(alias = "Ogre::RbxArchiveFactory::destroyAll(void)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory10destroyAllEv")]
pub fn stub_ba7684() -> ! {
    todo!("0xba7684 Ogre::RbxArchiveFactory::destroyAll(void)")
}

// 0xba7f20 — __ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerE
// type: Ogre::RbxCullableSceneNode *__fastcall(Ogre::RbxCullableSceneNode *this, Ogre::SceneManager *)
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerE")]
pub fn stub_ba7f20() -> ! {
    todo!("0xba7f20 Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *)")
}

// 0xba7f94 — __ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerERKSs
// type: _DWORD __fastcall(Ogre::RbxCullableSceneNode *__hidden this, Ogre::SceneManager *, const std::string *)
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerERKSs")]
pub fn stub_ba7f94() -> ! {
    todo!("0xba7f94 Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")
}

// 0xba8008 — __ZN4Ogre20RbxCullableSceneNodeC2EPNS_12SceneManagerERKSs
// type: Ogre::RbxCullableSceneNode *__fastcall(Ogre::RbxCullableSceneNode *this, Ogre::SceneManager *, const std::string *)
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeC2EPNS_12SceneManagerERKSs")]
pub fn stub_ba8008() -> ! {
    todo!("0xba8008 Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")
}

// 0xba807c — __ZN4Ogre20RbxCullableSceneNodeD0Ev
// type: void __fastcall(Ogre::RbxCullableSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeD0Ev")]
pub fn stub_ba807c() -> ! {
    todo!("0xba807c Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")
}

// 0xba8130 — __ZN4Ogre20RbxCullableSceneNodeD1Ev
// type: void __fastcall(Ogre::RbxCullableSceneNode *this, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeD1Ev")]
pub fn stub_ba8130() -> ! {
    todo!("0xba8130 Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")
}

// 0xba8134 — __ZN4Ogre20RbxCullableSceneNodeD2Ev
// type: void __fastcall(Ogre::RbxCullableSceneNode *this, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeD2Ev")]
pub fn stub_ba8134() -> ! {
    todo!("0xba8134 Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")
}

// 0xba827c — __ZN4Ogre20RbxCullableSceneNode27calculateSqDistanceToCameraEPKNS_6CameraE
// type: __int32 __fastcall(Ogre::RbxCullableSceneNode *this, const Ogre::Camera *)
#[doc(alias = "Ogre::RbxCullableSceneNode::calculateSqDistanceToCamera(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode27calculateSqDistanceToCameraEPKNS_6CameraE")]
pub fn stub_ba827c() -> ! {
    todo!("0xba827c Ogre::RbxCullableSceneNode::calculateSqDistanceToCamera(Ogre::Camera const*)")
}

// 0xba841c — __ZN4Ogre20RbxCullableSceneNode8IsCulledEPKNS_6CameraEb
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, const Ogre::Camera *, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::IsCulled(Ogre::Camera const*,bool)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode8IsCulledEPKNS_6CameraEb")]
pub fn stub_ba841c() -> ! {
    todo!("0xba841c Ogre::RbxCullableSceneNode::IsCulled(Ogre::Camera const*,bool)")
}

// 0xba854c — __ZN4Ogre20RbxCullableSceneNode17ShouldCastShadowsEPKNS_6CameraE
// type: bool __fastcall(Ogre::RbxCullableSceneNode *this, const Ogre::Camera *)
#[doc(alias = "Ogre::RbxCullableSceneNode::ShouldCastShadows(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode17ShouldCastShadowsEPKNS_6CameraE")]
pub fn stub_ba854c() -> ! {
    todo!("0xba854c Ogre::RbxCullableSceneNode::ShouldCastShadows(Ogre::Camera const*)")
}

// 0xba8594 — __ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbN3RBX15IntersectResultE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool,RBX::IntersectResult)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbN3RBX15IntersectResultE")]
pub fn stub_ba8594() -> ! {
    todo!("0xba8594 Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool,RBX::IntersectResult)")
}

// 0xba85e4 — __ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, int, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb")]
pub fn stub_ba85e4() -> ! {
    todo!("0xba85e4 Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")
}

// 0xba8750 — __ZN4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv")]
pub fn stub_ba8750() -> ! {
    todo!("0xba8750 Ogre::RbxCullableSceneNode::getFastFuzzyExtents(void)")
}

// 0xba876c — __ZThn392_N4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, int)
#[doc(alias = "non-virtual thunk toOgre::RbxCullableSceneNode::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZThn392_N4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv")]
pub fn stub_ba876c() -> ! {
    todo!("0xba876c non-virtual thunk toOgre::RbxCullableSceneNode::getFastFuzzyExtents(void)")
}

// 0xba8e18 — __ZN4Ogre9RbxEntityC1Ev
// type: Ogre::RbxEntity *__fastcall(Ogre::RbxEntity *this)
#[doc(alias = "Ogre::RbxEntity::RbxEntity(void)")]
#[doc(alias = "__ZN4Ogre9RbxEntityC1Ev")]
pub fn stub_ba8e18() -> ! {
    todo!("0xba8e18 Ogre::RbxEntity::RbxEntity(void)")
}

// 0xba8eb4 — __ZN4Ogre9RbxEntity7setMeshENS_7MeshPtrE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "Ogre::RbxEntity::setMesh(Ogre::MeshPtr)")]
#[doc(alias = "__ZN4Ogre9RbxEntity7setMeshENS_7MeshPtrE")]
pub fn stub_ba8eb4() -> ! {
    todo!("0xba8eb4 Ogre::RbxEntity::setMesh(Ogre::MeshPtr)")
}

// 0xba8f9c — __ZN4Ogre9RbxEntity16clearSubEntitiesEv
// type: int __fastcall(int this)
#[doc(alias = "Ogre::RbxEntity::clearSubEntities(void)")]
#[doc(alias = "__ZN4Ogre9RbxEntity16clearSubEntitiesEv")]
pub fn stub_ba8f9c() -> ! {
    todo!("0xba8f9c Ogre::RbxEntity::clearSubEntities(void)")
}

// 0xba8fa8 — __ZN4Ogre9RbxEntity15appendSubEntityEPNS_12RbxSubEntityE
// type: int __fastcall(int this, Ogre::RbxSubEntity *)
#[doc(alias = "Ogre::RbxEntity::appendSubEntity(Ogre::RbxSubEntity *)")]
#[doc(alias = "__ZN4Ogre9RbxEntity15appendSubEntityEPNS_12RbxSubEntityE")]
pub fn stub_ba8fa8() -> ! {
    todo!("0xba8fa8 Ogre::RbxEntity::appendSubEntity(Ogre::RbxSubEntity *)")
}

// 0xba8fdc — __ZN4Ogre9RbxEntity18_updateRenderQueueEPNS_11RenderQueueE
// type: int __fastcall(Ogre::RbxEntity *this, Ogre::RenderQueue *)
#[doc(alias = "Ogre::RbxEntity::_updateRenderQueue(Ogre::RenderQueue *)")]
#[doc(alias = "__ZN4Ogre9RbxEntity18_updateRenderQueueEPNS_11RenderQueueE")]
pub fn stub_ba8fdc() -> ! {
    todo!("0xba8fdc Ogre::RbxEntity::_updateRenderQueue(Ogre::RenderQueue *)")
}

// 0xba905c — __ZN4Ogre9RbxEntity20_notifyCurrentCameraEPNS_6CameraE
// type: int __fastcall(unsigned __int8 *)
#[doc(alias = "Ogre::RbxEntity::_notifyCurrentCamera(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre9RbxEntity20_notifyCurrentCameraEPNS_6CameraE")]
pub fn stub_ba905c() -> ! {
    todo!("0xba905c Ogre::RbxEntity::_notifyCurrentCamera(Ogre::Camera *)")
}

// 0xba9080 — __ZN4Ogre9RbxEntity13setVisibleAllEb
// type: int __fastcall(Ogre::RbxEntity *this, int)
#[doc(alias = "Ogre::RbxEntity::setVisibleAll(bool)")]
#[doc(alias = "__ZN4Ogre9RbxEntity13setVisibleAllEb")]
pub fn stub_ba9080() -> ! {
    todo!("0xba9080 Ogre::RbxEntity::setVisibleAll(bool)")
}

// 0xba90b4 — __ZN4Ogre9RbxEntity4cullEPKNS_6CameraE
// type: void __fastcall(Ogre::RbxEntity *this, const Ogre::Camera *)
#[doc(alias = "Ogre::RbxEntity::cull(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre9RbxEntity4cullEPKNS_6CameraE")]
pub fn stub_ba90b4() -> ! {
    todo!("0xba90b4 Ogre::RbxEntity::cull(Ogre::Camera const*)")
}

// 0xba92b8 — __ZN4Ogre9RbxEntityD0Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZN4Ogre9RbxEntityD0Ev")]
pub fn stub_ba92b8() -> ! {
    todo!("0xba92b8 Ogre::RbxEntity::~RbxEntity()")
}

// 0xba936c — __ZN4Ogre9RbxEntityD1Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZN4Ogre9RbxEntityD1Ev")]
pub fn stub_ba936c() -> ! {
    todo!("0xba936c Ogre::RbxEntity::~RbxEntity()")
}

// 0xba9370 — __ZThn4_N4Ogre9RbxEntityD0Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZThn4_N4Ogre9RbxEntityD0Ev")]
pub fn stub_ba9370() -> ! {
    todo!("0xba9370 non-virtual thunk toOgre::RbxEntity::~RbxEntity()")
}

// 0xba9428 — __ZThn188_N4Ogre9RbxEntityD0Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZThn188_N4Ogre9RbxEntityD0Ev")]
pub fn stub_ba9428() -> ! {
    todo!("0xba9428 non-virtual thunk toOgre::RbxEntity::~RbxEntity()")
}

// 0xba94e0 — __ZN4Ogre9RbxEntityD2Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZN4Ogre9RbxEntityD2Ev")]
pub fn stub_ba94e0() -> ! {
    todo!("0xba94e0 Ogre::RbxEntity::~RbxEntity()")
}

// 0xba9694 — __ZThn4_N4Ogre9RbxEntityD1Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZThn4_N4Ogre9RbxEntityD1Ev")]
pub fn stub_ba9694() -> ! {
    todo!("0xba9694 non-virtual thunk toOgre::RbxEntity::~RbxEntity()")
}

// 0xba969c — __ZThn188_N4Ogre9RbxEntityD1Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZThn188_N4Ogre9RbxEntityD1Ev")]
pub fn stub_ba969c() -> ! {
    todo!("0xba969c non-virtual thunk toOgre::RbxEntity::~RbxEntity()")
}

// 0xba96a4 — __ZNSt6vectorIN4Ogre9BlockSortESaIS1_EE7reserveEm
// type: unsigned int __fastcall(void **, unsigned int)
#[doc(alias = "std::vector<Ogre::BlockSort,std::allocator<Ogre::BlockSort>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre9BlockSortESaIS1_EE7reserveEm")]
pub fn stub_ba96a4() -> ! {
    todo!("0xba96a4 std::vector<Ogre::BlockSort,std::allocator<Ogre::BlockSort>>::reserve(unsigned long)")
}

// 0xba9dbc — __ZN4Ogre11RootManager9GetOrInitENS_11GraphicsAPIERKSs
// type: Ogre::NedPoolingImpl *__fastcall(Ogre::NedPoolingImpl **, Ogre::NedPoolingImpl *)
#[doc(alias = "Ogre::RootManager::GetOrInit(Ogre::GraphicsAPI,std::string const&)")]
#[doc(alias = "__ZN4Ogre11RootManager9GetOrInitENS_11GraphicsAPIERKSs")]
pub fn stub_ba9dbc() -> ! {
    todo!("0xba9dbc Ogre::RootManager::GetOrInit(Ogre::GraphicsAPI,std::string const&)")
}

// 0xbaae98 — __ZN4Ogre11RootManagerD2Ev
// type: void __fastcall(Ogre::RootManager *__hidden this)
#[doc(alias = "Ogre::RootManager::~RootManager()")]
#[doc(alias = "__ZN4Ogre11RootManagerD2Ev")]
pub fn stub_baae98() -> ! {
    todo!("0xbaae98 Ogre::RootManager::~RootManager()")
}

// 0xbab8f8 — __ZN4Ogre11RootManager16cleanUpResourcesERNS_15ResourceManagerERKSsS4_RKSt6vectorISsSaISsEE
// type: void __fastcall(_DWORD *, const std::string *, const void **, _DWORD *)
#[doc(alias = "Ogre::RootManager::cleanUpResources(Ogre::ResourceManager &,std::string const&,std::string const&,std::vector<std::string,std::allocator<std::string>> const&)")]
#[doc(alias = "__ZN4Ogre11RootManager16cleanUpResourcesERNS_15ResourceManagerERKSsS4_RKSt6vectorISsSaISsEE")]
pub fn stub_bab8f8() -> ! {
    todo!("0xbab8f8 Ogre::RootManager::cleanUpResources(Ogre::ResourceManager &,std::string const&,std::string const&,std::vector<std::string,std::allocator<std::string>> const&)")
}

// 0xbac7fc — __ZN4Ogre11RootManager14printResourcesERNS_15ResourceManagerERKSs
// type: void __fastcall(Ogre::RootManager *this, Ogre::ResourceManager *, const std::string *)
#[doc(alias = "Ogre::RootManager::printResources(Ogre::ResourceManager &,std::string const&)")]
#[doc(alias = "__ZN4Ogre11RootManager14printResourcesERNS_15ResourceManagerERKSs")]
pub fn stub_bac7fc() -> ! {
    todo!("0xbac7fc Ogre::RootManager::printResources(Ogre::ResourceManager &,std::string const&)")
}

// 0xbacfcc — __ZN4Ogre12VisualEngineC1Ev
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngineC1Ev")]
pub fn stub_bacfcc() -> ! {
    todo!("0xbacfcc Ogre::VisualEngine::VisualEngine(void)")
}

// 0xbacfd0 — __ZN4Ogre12VisualEngineC2Ev
// type: Ogre::VisualEngine *__fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngineC2Ev")]
pub fn stub_bacfd0() -> ! {
    todo!("0xbacfd0 Ogre::VisualEngine::VisualEngine(void)")
}

// 0xbad2c8 — __ZN4Ogre12VisualEngineD0Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
#[doc(alias = "__ZN4Ogre12VisualEngineD0Ev")]
pub fn stub_bad2c8() -> ! {
    todo!("0xbad2c8 Ogre::VisualEngine::~VisualEngine()")
}

// 0xbad368 — __ZN4Ogre12VisualEngineD1Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
#[doc(alias = "__ZN4Ogre12VisualEngineD1Ev")]
pub fn stub_bad368() -> ! {
    todo!("0xbad368 Ogre::VisualEngine::~VisualEngine()")
}

// 0xbad36c — __ZN4Ogre12VisualEngineD2Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
#[doc(alias = "__ZN4Ogre12VisualEngineD2Ev")]
pub fn stub_bad36c() -> ! {
    todo!("0xbad36c Ogre::VisualEngine::~VisualEngine()")
}

// 0xbadb50 — __ZN4Ogre12VisualEngine7setViewEPN3RBX8ViewBaseE
// type: void __fastcall(Ogre::VisualEngine *this, RBX::ViewBase *)
#[doc(alias = "Ogre::VisualEngine::setView(RBX::ViewBase *)")]
#[doc(alias = "__ZN4Ogre12VisualEngine7setViewEPN3RBX8ViewBaseE")]
pub fn stub_badb50() -> ! {
    todo!("0xbadb50 Ogre::VisualEngine::setView(RBX::ViewBase *)")
}

// 0xbadd64 — __ZN4Ogre14SaveRBXDbgInfoEPKNS_24RenderSystemCapabilitiesE
// type: void __fastcall(Ogre *this, const Ogre::RenderSystemCapabilities *)
#[doc(alias = "Ogre::SaveRBXDbgInfo(Ogre::RenderSystemCapabilities const*)")]
#[doc(alias = "__ZN4Ogre14SaveRBXDbgInfoEPKNS_24RenderSystemCapabilitiesE")]
pub fn stub_badd64() -> ! {
    todo!("0xbadd64 Ogre::SaveRBXDbgInfo(Ogre::RenderSystemCapabilities const*)")
}

// 0xbae0f8 — __ZN4Ogre12VisualEngine14initializeBaseENS_11GraphicsAPIEmiiPN3RBX15CRenderSettingsERKSsmPNS2_9OSContextE
// type: int __fastcall(_DWORD *, int, int, int (*)(const char *, ...), int, int, int, int, const char **)
#[doc(alias = "Ogre::VisualEngine::initializeBase(Ogre::GraphicsAPI,unsigned long,int,int,RBX::CRenderSettings *,std::string const&,unsigned long,RBX::OSContext *)")]
#[doc(alias = "__ZN4Ogre12VisualEngine14initializeBaseENS_11GraphicsAPIEmiiPN3RBX15CRenderSettingsERKSsmPNS2_9OSContextE")]
pub fn stub_bae0f8() -> ! {
    todo!("0xbae0f8 Ogre::VisualEngine::initializeBase(Ogre::GraphicsAPI,unsigned long,int,int,RBX::CRenderSettings *,std::string const&,unsigned long,RBX::OSContext *)")
}

// 0xbb0120 — __ZN4Ogre12VisualEngine14initializeLoadEii
// type: void __fastcall(Ogre::VisualEngine *this, int, int)
#[doc(alias = "Ogre::VisualEngine::initializeLoad(int,int)")]
#[doc(alias = "__ZN4Ogre12VisualEngine14initializeLoadEii")]
pub fn stub_bb0120() -> ! {
    todo!("0xbb0120 Ogre::VisualEngine::initializeLoad(int,int)")
}

// 0xbb039c — __ZN4Ogre12VisualEngine14setupResourcesEv
// type: void __fastcall(Ogre::VisualEngine *this, Ogre::VisualEngine *)
#[doc(alias = "Ogre::VisualEngine::setupResources(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngine14setupResourcesEv")]
pub fn stub_bb039c() -> ! {
    todo!("0xbb039c Ogre::VisualEngine::setupResources(void)")
}

// 0xbb1160 — __ZN4Ogre12VisualEngine10setupSceneEii
// type: void __fastcall(Ogre::VisualEngine *this, int, int)
#[doc(alias = "Ogre::VisualEngine::setupScene(int,int)")]
#[doc(alias = "__ZN4Ogre12VisualEngine10setupSceneEii")]
pub fn stub_bb1160() -> ! {
    todo!("0xbb1160 Ogre::VisualEngine::setupScene(int,int)")
}

// 0xbb1c18 — __ZN4Ogre12VisualEngine17checkMaterialCapsEv
// type: void __fastcall(Ogre::Root **this)
#[doc(alias = "Ogre::VisualEngine::checkMaterialCaps(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngine17checkMaterialCapsEv")]
pub fn stub_bb1c18() -> ! {
    todo!("0xbb1c18 Ogre::VisualEngine::checkMaterialCaps(void)")
}
