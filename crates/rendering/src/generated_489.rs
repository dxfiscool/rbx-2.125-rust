//! rendering shard 489 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xc7aae4..0xc8162c, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|Gfx|Render|G3D), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xc7aae4 — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE7destroyEv
// IDA 0xc7aae4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7aae4() {
}


// 0xc7ab1c — __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE4swapERS3_
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerFunction<float>>::swap(Ogre::SharedPtr<Ogre::ControllerFunction<float>>&)")]
// was: __ZN4Ogre9SharedPtrINS_18ControllerFunctionIfEEE4swapERS3_
// IDA 0xc7ab1c: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ab1c() {
}


// 0xc7ab38 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::~SharedPtr()")]
// was: __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEED0Ev
// IDA 0xc7ab38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ab38() {
}


// 0xc7ac2c — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::destroy(void)")]
// was: __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE7destroyEv
// IDA 0xc7ac2c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ac2c() {
}


// 0xc7ac64 — __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE4swapERS3_
#[doc(alias = "Ogre::SharedPtr<Ogre::ControllerValue<float>>::swap(Ogre::SharedPtr<Ogre::ControllerValue<float>>&)")]
// was: __ZN4Ogre9SharedPtrINS_15ControllerValueIfEEE4swapERS3_
// IDA 0xc7ac64: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ac64() {
}


// 0xc7ac80 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Controller<float> *> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
// IDA 0xc7ac80: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ac80() {
}


// 0xc7aca8 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Controller<float> * const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_
// IDA 0xc7aca8: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7aca8() {
}


// 0xc7ada0 — __ZN4Ogre10ControllerIfEC2ERKNS_9SharedPtrINS_15ControllerValueIfEEEES7_RKNS2_INS_18ControllerFunctionIfEEEE
#[doc(alias = "Ogre::Controller<float>::Controller(Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerValue<float>> const&,Ogre::SharedPtr<Ogre::ControllerFunction<float>> const&)")]
// was: __ZN4Ogre10ControllerIfEC2ERKNS_9SharedPtrINS_15ControllerValueIfEEEES7_RKNS2_INS_18ControllerFunctionIfEEEE
// IDA 0xc7ada0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ada0() {
}


// 0xc7ae5c — __ZN4Ogre10ControllerIfED1Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
// was: __ZN4Ogre10ControllerIfED1Ev
// IDA 0xc7ae5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ae5c() {
}


// 0xc7ae68 — __ZN4Ogre10ControllerIfED0Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
// was: __ZN4Ogre10ControllerIfED0Ev
// IDA 0xc7ae68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ae68() {
}


// 0xc7aef8 — __ZN4Ogre10ControllerIfED2Ev
#[doc(alias = "Ogre::Controller<float>::~Controller()")]
// was: __ZN4Ogre10ControllerIfED2Ev
// IDA 0xc7aef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7aef8() {
}


// 0xc7b0cc — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED1Ev
// IDA 0xc7b0cc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7b0cc() {
}


// 0xc7b0d0 — __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Controller<float> *,Ogre::Controller<float> *,std::_Identity<Ogre::Controller<float> *>,std::less<Ogre::Controller<float> *>,Ogre::STLAllocator<Ogre::Controller<float> *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Controller<float> *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre10ControllerIfEES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS7_Lb0EED0Ev
// IDA 0xc7b0d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7b0d0() {
}


// 0xc7b110 — __ZN4Ogre10ConvexBody15_initialisePoolEv
// type: _DWORD __fastcall(Ogre::ConvexBody *__hidden this)
#[doc(alias = "Ogre::ConvexBody::_initialisePool(void)")]
// was: __ZN4Ogre10ConvexBody15_initialisePoolEv
// IDA 0xc7b110: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b110() {
}


// 0xc7b16c — __ZN4Ogre10ConvexBody12_destroyPoolEv
// type: _DWORD __fastcall(Ogre::ConvexBody *__hidden this)
#[doc(alias = "Ogre::ConvexBody::_destroyPool(void)")]
// was: __ZN4Ogre10ConvexBody12_destroyPoolEv
// IDA 0xc7b16c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b16c() {
}


// 0xc7b1a0 — __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
// was: __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
// IDA 0xc7b1a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7b1a0() {
}


// 0xc7b234 — __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Polygon **,std::vector<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Polygon * const&)")]
// was: __ZNSt6vectorIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// IDA 0xc7b234: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7b234() {
}


// 0xc7b3dc — __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xc7b3dc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7b3dc() {
}


// 0xc7b3e0 — __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Polygon *,Ogre::STLAllocator<Ogre::Polygon *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre7PolygonENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xc7b3e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7b3e0() {
}


// 0xc7cdc4 — __ZN4Ogre8DDSCodec7startupEv
// type: _DWORD __fastcall(Ogre::DDSCodec *__hidden this)
#[doc(alias = "Ogre::DDSCodec::startup(void)")]
// was: __ZN4Ogre8DDSCodec7startupEv
// IDA 0xc7cdc4: 162 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cdc4() {
}


// 0xc7cf9c — __ZN4Ogre8DDSCodec8shutdownEv
// type: _DWORD __fastcall(Ogre::DDSCodec *__hidden this)
#[doc(alias = "Ogre::DDSCodec::shutdown(void)")]
// was: __ZN4Ogre8DDSCodec8shutdownEv
// IDA 0xc7cf9c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7cf9c() {
}


// 0xc7d8b8 — __ZNK4Ogre8DDSCodec18convertPixelFormatEjjjjj
// type: _DWORD __fastcall(Ogre::DDSCodec *__hidden this, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int)
#[doc(alias = "Ogre::DDSCodec::convertPixelFormat(unsigned int,unsigned int,unsigned int,unsigned int,unsigned int)const")]
// was: __ZNK4Ogre8DDSCodec18convertPixelFormatEjjjjj
// IDA 0xc7d8b8: 150 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7d8b8() {
}


// 0xc7db10 — __ZNK4Ogre8DDSCodec15unpackDXTColourENS_11PixelFormatERKNS_14DXTColourBlockEPNS_11ColourValueE
#[doc(alias = "Ogre::DDSCodec::unpackDXTColour(Ogre::PixelFormat,Ogre::DXTColourBlock const&,Ogre::ColourValue *)const")]
// was: __ZNK4Ogre8DDSCodec15unpackDXTColourENS_11PixelFormatERKNS_14DXTColourBlockEPNS_11ColourValueE
// IDA 0xc7db10: 219 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7db10() {
}


// 0xc7ddc4 — __ZNK4Ogre8DDSCodec14unpackDXTAlphaERKNS_21DXTExplicitAlphaBlockEPNS_11ColourValueE
#[doc(alias = "Ogre::DDSCodec::unpackDXTAlpha(Ogre::DXTExplicitAlphaBlock const&,Ogre::ColourValue *)const")]
// was: __ZNK4Ogre8DDSCodec14unpackDXTAlphaERKNS_21DXTExplicitAlphaBlockEPNS_11ColourValueE
// IDA 0xc7ddc4: 86 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ddc4() {
}


// 0xc7df0c — __ZNK4Ogre8DDSCodec14unpackDXTAlphaERKNS_25DXTInterpolatedAlphaBlockEPNS_11ColourValueE
#[doc(alias = "Ogre::DDSCodec::unpackDXTAlpha(Ogre::DXTInterpolatedAlphaBlock const&,Ogre::ColourValue *)const")]
// was: __ZNK4Ogre8DDSCodec14unpackDXTAlphaERKNS_25DXTInterpolatedAlphaBlockEPNS_11ColourValueE
// IDA 0xc7df0c: 119 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7df0c() {
}


// 0xc7eea8 — __ZNK4Ogre8DDSCodec7getTypeEv
// type: _DWORD __fastcall(Ogre::DDSCodec *__hidden this)
#[doc(alias = "Ogre::DDSCodec::getType(void)const")]
// was: __ZNK4Ogre8DDSCodec7getTypeEv
// IDA 0xc7eea8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7eea8() {
}


// 0xc7eeb4 — __ZNK4Ogre8DDSCodec20magicNumberToFileExtEPKcm
// type: _DWORD __fastcall(Ogre::DDSCodec *__hidden this, const char *, unsigned int)
#[doc(alias = "Ogre::DDSCodec::magicNumberToFileExt(char const*,unsigned long)const")]
// was: __ZNK4Ogre8DDSCodec20magicNumberToFileExtEPKcm
// IDA 0xc7eeb4: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7eeb4() {
}


// 0xc7eef4 — __ZN4Ogre8DDSCodecD1Ev
// type: void __fastcall(Ogre::DDSCodec *__hidden this)
#[doc(alias = "Ogre::DDSCodec::~DDSCodec()")]
// was: __ZN4Ogre8DDSCodecD1Ev
// IDA 0xc7eef4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7eef4() {
}


// 0xc7ef54 — __ZN4Ogre8DDSCodecD0Ev
// type: void __fastcall(Ogre::DDSCodec *__hidden this)
#[doc(alias = "Ogre::DDSCodec::~DDSCodec()")]
// was: __ZN4Ogre8DDSCodecD0Ev
// IDA 0xc7ef54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ef54() {
}


// 0xc7f068 — __ZN4Ogre27DefaultHardwareVertexBufferC1EPNS_25HardwareBufferManagerBaseEmmNS_14HardwareBuffer5UsageE
// type: int __fastcall(int, int, int, int, int, Ogre::HardwareVertexBuffer *, int, int, int)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::DefaultHardwareVertexBuffer(Ogre::HardwareBufferManagerBase *,unsigned long,unsigned long,Ogre::HardwareBuffer::Usage)")]
// was: __ZN4Ogre27DefaultHardwareVertexBufferC1EPNS_25HardwareBufferManagerBaseEmmNS_14HardwareBuffer5UsageE
// IDA 0xc7f068: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f068() {
}


// 0xc7f134 — __ZN4Ogre27DefaultHardwareVertexBufferD0Ev
// type: void __fastcall(Ogre::DefaultHardwareVertexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::~DefaultHardwareVertexBuffer()")]
// was: __ZN4Ogre27DefaultHardwareVertexBufferD0Ev
// IDA 0xc7f134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f134() {
}


// 0xc7f1f8 — __ZN4Ogre27DefaultHardwareVertexBufferD1Ev
// type: void __fastcall(Ogre::DefaultHardwareVertexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::~DefaultHardwareVertexBuffer()")]
// was: __ZN4Ogre27DefaultHardwareVertexBufferD1Ev
// IDA 0xc7f1f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f1f8() {
}


// 0xc7f2ac — __ZN4Ogre27DefaultHardwareVertexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre27DefaultHardwareVertexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
// IDA 0xc7f2ac: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2ac() {
}


// 0xc7f2b4 — __ZN4Ogre27DefaultHardwareVertexBuffer10unlockImplEv
// type: _DWORD __fastcall(Ogre::DefaultHardwareVertexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::unlockImpl(void)")]
// was: __ZN4Ogre27DefaultHardwareVertexBuffer10unlockImplEv
// IDA 0xc7f2b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7f2b4() {
}


// 0xc7f2b8 — __ZN4Ogre27DefaultHardwareVertexBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre27DefaultHardwareVertexBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
// IDA 0xc7f2b8: 5 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2b8() {
}


// 0xc7f2c4 — __ZN4Ogre27DefaultHardwareVertexBuffer6unlockEv
// type: _DWORD __fastcall(Ogre::DefaultHardwareVertexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::unlock(void)")]
// was: __ZN4Ogre27DefaultHardwareVertexBuffer6unlockEv
// IDA 0xc7f2c4: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2c4() {
}


// 0xc7f2cc — __ZN4Ogre27DefaultHardwareVertexBuffer8readDataEmmPv
// type: _DWORD __fastcall(Ogre::DefaultHardwareVertexBuffer *__hidden this, unsigned int, unsigned int, void *__dst)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: __ZN4Ogre27DefaultHardwareVertexBuffer8readDataEmmPv
// IDA 0xc7f2cc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2cc() {
}


// 0xc7f2dc — __ZN4Ogre27DefaultHardwareVertexBuffer9writeDataEmmPKvb
// type: _DWORD __fastcall(Ogre::DefaultHardwareVertexBuffer *__hidden this, unsigned int, unsigned int, const void *__src, bool)
#[doc(alias = "Ogre::DefaultHardwareVertexBuffer::writeData(unsigned long,unsigned long,void const*,bool)")]
// was: __ZN4Ogre27DefaultHardwareVertexBuffer9writeDataEmmPKvb
// IDA 0xc7f2dc: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2dc() {
}


// 0xc7f2ec — __ZN4Ogre26DefaultHardwareIndexBufferC1ENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageE
// type: int __fastcall(int, int, int, int, int, Ogre::HardwareIndexBuffer *, int, int, int)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::DefaultHardwareIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage)")]
// was: __ZN4Ogre26DefaultHardwareIndexBufferC1ENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageE
// IDA 0xc7f2ec: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f2ec() {
}


// 0xc7f3bc — __ZN4Ogre26DefaultHardwareIndexBufferD0Ev
// type: void __fastcall(Ogre::DefaultHardwareIndexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::~DefaultHardwareIndexBuffer()")]
// was: __ZN4Ogre26DefaultHardwareIndexBufferD0Ev
// IDA 0xc7f3bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f3bc() {
}


// 0xc7f47c — __ZN4Ogre26DefaultHardwareIndexBufferD1Ev
// type: void __fastcall(Ogre::DefaultHardwareIndexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::~DefaultHardwareIndexBuffer()")]
// was: __ZN4Ogre26DefaultHardwareIndexBufferD1Ev
// IDA 0xc7f47c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f47c() {
}


// 0xc7f530 — __ZN4Ogre26DefaultHardwareIndexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre26DefaultHardwareIndexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
// IDA 0xc7f530: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f530() {
}


// 0xc7f538 — __ZN4Ogre26DefaultHardwareIndexBuffer10unlockImplEv
// type: _DWORD __fastcall(Ogre::DefaultHardwareIndexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::unlockImpl(void)")]
// was: __ZN4Ogre26DefaultHardwareIndexBuffer10unlockImplEv
// IDA 0xc7f538: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c7f538() {
}


// 0xc7f53c — __ZN4Ogre26DefaultHardwareIndexBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre26DefaultHardwareIndexBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
// IDA 0xc7f53c: 5 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f53c() {
}


// 0xc7f548 — __ZN4Ogre26DefaultHardwareIndexBuffer6unlockEv
// type: _DWORD __fastcall(Ogre::DefaultHardwareIndexBuffer *__hidden this)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::unlock(void)")]
// was: __ZN4Ogre26DefaultHardwareIndexBuffer6unlockEv
// IDA 0xc7f548: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f548() {
}


// 0xc7f550 — __ZN4Ogre26DefaultHardwareIndexBuffer8readDataEmmPv
// type: _DWORD __fastcall(Ogre::DefaultHardwareIndexBuffer *__hidden this, unsigned int, unsigned int, void *__dst)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: __ZN4Ogre26DefaultHardwareIndexBuffer8readDataEmmPv
// IDA 0xc7f550: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f550() {
}


// 0xc7f560 — __ZN4Ogre26DefaultHardwareIndexBuffer9writeDataEmmPKvb
// type: _DWORD __fastcall(Ogre::DefaultHardwareIndexBuffer *__hidden this, unsigned int, unsigned int, const void *__src, bool)
#[doc(alias = "Ogre::DefaultHardwareIndexBuffer::writeData(unsigned long,unsigned long,void const*,bool)")]
// was: __ZN4Ogre26DefaultHardwareIndexBuffer9writeDataEmmPKvb
// IDA 0xc7f560: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f560() {
}


// 0xc7f5a4 — __ZN4Ogre29DefaultIntersectionSceneQueryC1EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::DefaultIntersectionSceneQuery *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::DefaultIntersectionSceneQuery::DefaultIntersectionSceneQuery(Ogre::SceneManager *)")]
// was: __ZN4Ogre29DefaultIntersectionSceneQueryC1EPNS_12SceneManagerE
// IDA 0xc7f5a4: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f5a4() {
}


// 0xc7f66c — __ZN4Ogre29DefaultIntersectionSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultIntersectionSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()")]
// was: __ZN4Ogre29DefaultIntersectionSceneQueryD0Ev
// IDA 0xc7f66c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f66c() {
}


// 0xc7f6fc — __ZN4Ogre29DefaultIntersectionSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultIntersectionSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()")]
// was: __ZN4Ogre29DefaultIntersectionSceneQueryD1Ev
// IDA 0xc7f6fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f6fc() {
}


// 0xc7f708 — __ZThn48_N4Ogre29DefaultIntersectionSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultIntersectionSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()")]
// was: __ZThn48_N4Ogre29DefaultIntersectionSceneQueryD0Ev
// IDA 0xc7f708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f708() {
}


// 0xc7f79c — __ZThn48_N4Ogre29DefaultIntersectionSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultIntersectionSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultIntersectionSceneQuery::~DefaultIntersectionSceneQuery()")]
// was: __ZThn48_N4Ogre29DefaultIntersectionSceneQueryD1Ev
// IDA 0xc7f79c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7f79c() {
}


// 0xc7f7a8 — __ZN4Ogre29DefaultIntersectionSceneQuery7executeEPNS_30IntersectionSceneQueryListenerE
#[doc(alias = "Ogre::DefaultIntersectionSceneQuery::execute(Ogre::IntersectionSceneQueryListener *)")]
// was: __ZN4Ogre29DefaultIntersectionSceneQuery7executeEPNS_30IntersectionSceneQueryListenerE
// IDA 0xc7f7a8: 253 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7f7a8() {
}


// 0xc7fa48 — __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryC1EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::DefaultAxisAlignedBoxSceneQuery *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::DefaultAxisAlignedBoxSceneQuery(Ogre::SceneManager *)")]
// was: __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryC1EPNS_12SceneManagerE
// IDA 0xc7fa48: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7fa48() {
}


// 0xc7fb10 — __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultAxisAlignedBoxSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()")]
// was: __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryD0Ev
// IDA 0xc7fb10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fb10() {
}


// 0xc7fba0 — __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultAxisAlignedBoxSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()")]
// was: __ZN4Ogre31DefaultAxisAlignedBoxSceneQueryD1Ev
// IDA 0xc7fba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fba0() {
}


// 0xc7fbac — __ZThn48_N4Ogre31DefaultAxisAlignedBoxSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultAxisAlignedBoxSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()")]
// was: __ZThn48_N4Ogre31DefaultAxisAlignedBoxSceneQueryD0Ev
// IDA 0xc7fbac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fbac() {
}


// 0xc7fc40 — __ZThn48_N4Ogre31DefaultAxisAlignedBoxSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultAxisAlignedBoxSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultAxisAlignedBoxSceneQuery::~DefaultAxisAlignedBoxSceneQuery()")]
// was: __ZThn48_N4Ogre31DefaultAxisAlignedBoxSceneQueryD1Ev
// IDA 0xc7fc40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fc40() {
}


// 0xc7fc4c — __ZN4Ogre31DefaultAxisAlignedBoxSceneQuery7executeEPNS_18SceneQueryListenerE
#[doc(alias = "Ogre::DefaultAxisAlignedBoxSceneQuery::execute(Ogre::SceneQueryListener *)")]
// was: __ZN4Ogre31DefaultAxisAlignedBoxSceneQuery7executeEPNS_18SceneQueryListenerE
// IDA 0xc7fc4c: 117 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7fc4c() {
}


// 0xc7fd80 — __ZN4Ogre20DefaultRaySceneQueryC1EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::DefaultRaySceneQuery *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::DefaultRaySceneQuery::DefaultRaySceneQuery(Ogre::SceneManager *)")]
// was: __ZN4Ogre20DefaultRaySceneQueryC1EPNS_12SceneManagerE
// IDA 0xc7fd80: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7fd80() {
}


// 0xc7fe48 — __ZN4Ogre20DefaultRaySceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultRaySceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: __ZN4Ogre20DefaultRaySceneQueryD0Ev
// IDA 0xc7fe48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fe48() {
}


// 0xc7fed8 — __ZN4Ogre20DefaultRaySceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultRaySceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: __ZN4Ogre20DefaultRaySceneQueryD1Ev
// IDA 0xc7fed8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fed8() {
}


// 0xc7fee4 — __ZThn48_N4Ogre20DefaultRaySceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultRaySceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: __ZThn48_N4Ogre20DefaultRaySceneQueryD0Ev
// IDA 0xc7fee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7fee4() {
}


// 0xc7ff78 — __ZThn48_N4Ogre20DefaultRaySceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultRaySceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultRaySceneQuery::~DefaultRaySceneQuery()")]
// was: __ZThn48_N4Ogre20DefaultRaySceneQueryD1Ev
// IDA 0xc7ff78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c7ff78() {
}


// 0xc7ff84 — __ZN4Ogre20DefaultRaySceneQuery7executeEPNS_21RaySceneQueryListenerE
#[doc(alias = "Ogre::DefaultRaySceneQuery::execute(Ogre::RaySceneQueryListener *)")]
// was: __ZN4Ogre20DefaultRaySceneQuery7executeEPNS_21RaySceneQueryListenerE
// IDA 0xc7ff84: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c7ff84() {
}


// 0xc8004c — __ZN4Ogre23DefaultSphereSceneQueryC1EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::DefaultSphereSceneQuery *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::DefaultSphereSceneQuery::DefaultSphereSceneQuery(Ogre::SceneManager *)")]
// was: __ZN4Ogre23DefaultSphereSceneQueryC1EPNS_12SceneManagerE
// IDA 0xc8004c: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8004c() {
}


// 0xc80114 — __ZN4Ogre23DefaultSphereSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultSphereSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: __ZN4Ogre23DefaultSphereSceneQueryD0Ev
// IDA 0xc80114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80114() {
}


// 0xc801a4 — __ZN4Ogre23DefaultSphereSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultSphereSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: __ZN4Ogre23DefaultSphereSceneQueryD1Ev
// IDA 0xc801a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c801a4() {
}


// 0xc801b0 — __ZThn48_N4Ogre23DefaultSphereSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultSphereSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: __ZThn48_N4Ogre23DefaultSphereSceneQueryD0Ev
// IDA 0xc801b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c801b0() {
}


// 0xc80244 — __ZThn48_N4Ogre23DefaultSphereSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultSphereSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultSphereSceneQuery::~DefaultSphereSceneQuery()")]
// was: __ZThn48_N4Ogre23DefaultSphereSceneQueryD1Ev
// IDA 0xc80244: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80244() {
}


// 0xc80250 — __ZN4Ogre23DefaultSphereSceneQuery7executeEPNS_18SceneQueryListenerE
#[doc(alias = "Ogre::DefaultSphereSceneQuery::execute(Ogre::SceneQueryListener *)")]
// was: __ZN4Ogre23DefaultSphereSceneQuery7executeEPNS_18SceneQueryListenerE
// IDA 0xc80250: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80250() {
}


// 0xc80384 — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryC1EPNS_12SceneManagerE
// type: _DWORD __fastcall(Ogre::DefaultPlaneBoundedVolumeListSceneQuery *__hidden this, Ogre::SceneManager *)
#[doc(alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::DefaultPlaneBoundedVolumeListSceneQuery(Ogre::SceneManager *)")]
// was: __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryC1EPNS_12SceneManagerE
// IDA 0xc80384: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80384() {
}


// 0xc8044c — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultPlaneBoundedVolumeListSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()")]
// was: __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD0Ev
// IDA 0xc8044c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8044c() {
}


// 0xc804dc — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultPlaneBoundedVolumeListSceneQuery *__hidden this)
#[doc(alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()")]
// was: __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD1Ev
// IDA 0xc804dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c804dc() {
}


// 0xc804e8 — __ZThn48_N4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD0Ev
// type: void __fastcall(Ogre::DefaultPlaneBoundedVolumeListSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()")]
// was: __ZThn48_N4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD0Ev
// IDA 0xc804e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c804e8() {
}


// 0xc8057c — __ZThn48_N4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD1Ev
// type: void __fastcall(Ogre::DefaultPlaneBoundedVolumeListSceneQuery *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::DefaultPlaneBoundedVolumeListSceneQuery::~DefaultPlaneBoundedVolumeListSceneQuery()")]
// was: __ZThn48_N4Ogre39DefaultPlaneBoundedVolumeListSceneQueryD1Ev
// IDA 0xc8057c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8057c() {
}


// 0xc80588 — __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQuery7executeEPNS_18SceneQueryListenerE
#[doc(alias = "Ogre::DefaultPlaneBoundedVolumeListSceneQuery::execute(Ogre::SceneQueryListener *)")]
// was: __ZN4Ogre39DefaultPlaneBoundedVolumeListSceneQuery7executeEPNS_18SceneQueryListenerE
// IDA 0xc80588: 150 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80588() {
}


// 0xc80728 — __ZNSt8_Rb_treeIN4Ogre10SceneQuery17WorldFragmentTypeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::SceneQuery::WorldFragmentType,Ogre::SceneQuery::WorldFragmentType,std::_Identity<Ogre::SceneQuery::WorldFragmentType>,std::less<Ogre::SceneQuery::WorldFragmentType>,Ogre::STLAllocator<Ogre::SceneQuery::WorldFragmentType,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::SceneQuery::WorldFragmentType const&)")]
// was: __ZNSt8_Rb_treeIN4Ogre10SceneQuery17WorldFragmentTypeES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// IDA 0xc80728: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80728() {
}


// 0xc80854 — __ZN4Ogre19DistanceLodStrategy15getSingletonPtrEv
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this)
#[doc(alias = "Ogre::DistanceLodStrategy::getSingletonPtr(void)")]
// was: __ZN4Ogre19DistanceLodStrategy15getSingletonPtrEv
// IDA 0xc80854: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80854() {
}


// 0xc80864 — __ZN4Ogre19DistanceLodStrategyC1Ev
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this)
#[doc(alias = "Ogre::DistanceLodStrategy::DistanceLodStrategy(void)")]
// was: __ZN4Ogre19DistanceLodStrategyC1Ev
// IDA 0xc80864: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80864() {
}


// 0xc80870 — __ZN4Ogre19DistanceLodStrategyC2Ev
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this)
#[doc(alias = "Ogre::DistanceLodStrategy::DistanceLodStrategy(void)")]
// was: __ZN4Ogre19DistanceLodStrategyC2Ev
// IDA 0xc80870: 116 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80870() {
}


// 0xc809cc — __ZNK4Ogre19DistanceLodStrategy12getValueImplEPKNS_13MovableObjectEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this, const Ogre::MovableObject *, const Ogre::Camera *)
#[doc(alias = "Ogre::DistanceLodStrategy::getValueImpl(Ogre::MovableObject const*,Ogre::Camera const*)const")]
// was: __ZNK4Ogre19DistanceLodStrategy12getValueImplEPKNS_13MovableObjectEPKNS_6CameraE
// IDA 0xc809cc: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c809cc() {
}


// 0xc80ad8 — __ZNK4Ogre19DistanceLodStrategy12getBaseValueEv
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this)
#[doc(alias = "Ogre::DistanceLodStrategy::getBaseValue(void)const")]
// was: __ZNK4Ogre19DistanceLodStrategy12getBaseValueEv
// IDA 0xc80ad8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80ad8() {
}


// 0xc80adc — __ZNK4Ogre19DistanceLodStrategy13transformBiasEf
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this, float)
#[doc(alias = "Ogre::DistanceLodStrategy::transformBias(float)const")]
// was: __ZNK4Ogre19DistanceLodStrategy13transformBiasEf
// IDA 0xc80adc: 5 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80adc() {
}


// 0xc80af0 — __ZNK4Ogre19DistanceLodStrategy18transformUserValueEf
// type: _DWORD __fastcall(Ogre::DistanceLodStrategy *__hidden this, float)
#[doc(alias = "Ogre::DistanceLodStrategy::transformUserValue(float)const")]
// was: __ZNK4Ogre19DistanceLodStrategy18transformUserValueEf
// IDA 0xc80af0: 4 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80af0() {
}


// 0xc80b00 — __ZNK4Ogre19DistanceLodStrategy8getIndexEfRKSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::DistanceLodStrategy::getIndex(float,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const")]
// was: __ZNK4Ogre19DistanceLodStrategy8getIndexEfRKSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xc80b00: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b00() {
}


// 0xc80b10 — __ZNK4Ogre19DistanceLodStrategy8getIndexEfRKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::DistanceLodStrategy::getIndex(float,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const")]
// was: __ZNK4Ogre19DistanceLodStrategy8getIndexEfRKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xc80b10: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b10() {
}


// 0xc80b20 — __ZNK4Ogre19DistanceLodStrategy8isSortedERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::DistanceLodStrategy::isSorted(std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)const")]
// was: __ZNK4Ogre19DistanceLodStrategy8isSortedERKSt6vectorIfNS_12STLAllocatorIfNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xc80b20: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b20() {
}


// 0xc80b2c — __ZNK4Ogre19DistanceLodStrategy4sortERSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::DistanceLodStrategy::sort(std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)const")]
// was: __ZNK4Ogre19DistanceLodStrategy4sortERSt6vectorINS_12MeshLodUsageENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// IDA 0xc80b2c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80b2c() {
}


// 0xc80b38 — __ZN4Ogre19DistanceLodStrategyD1Ev
// type: void __fastcall(Ogre::DistanceLodStrategy *__hidden this)
#[doc(alias = "Ogre::DistanceLodStrategy::~DistanceLodStrategy()")]
// was: __ZN4Ogre19DistanceLodStrategyD1Ev
// IDA 0xc80b38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80b38() {
}


// 0xc80b50 — __ZN4Ogre19DistanceLodStrategyD0Ev
// type: void __fastcall(Ogre::DistanceLodStrategy *__hidden this)
#[doc(alias = "Ogre::DistanceLodStrategy::~DistanceLodStrategy()")]
// was: __ZN4Ogre19DistanceLodStrategyD0Ev
// IDA 0xc80b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80b50() {
}


// 0xc80c24 — __ZN4Ogre6DynLibC1ERKSs
// type: _DWORD __fastcall(Ogre::DynLib *__hidden this, const std::string *)
#[doc(alias = "Ogre::DynLib::DynLib(std::string const&)")]
// was: __ZN4Ogre6DynLibC1ERKSs
// IDA 0xc80c24: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80c24() {
}


// 0xc80d10 — __ZN4Ogre6DynLibD1Ev
// type: void __fastcall(Ogre::DynLib *__hidden this)
#[doc(alias = "Ogre::DynLib::~DynLib()")]
// was: __ZN4Ogre6DynLibD1Ev
// IDA 0xc80d10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c80d10() {
}


// 0xc80d5c — __ZN4Ogre6DynLib4loadEv
// type: _DWORD __fastcall(Ogre::DynLib *__hidden this)
#[doc(alias = "Ogre::DynLib::load(void)")]
// was: __ZN4Ogre6DynLib4loadEv
// IDA 0xc80d5c: 223 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c80d5c() {
}


// 0xc812a0 — __ZN4Ogre6DynLib6unloadEv
// type: _DWORD __fastcall(Ogre::DynLib *__hidden this)
#[doc(alias = "Ogre::DynLib::unload(void)")]
// was: __ZN4Ogre6DynLib6unloadEv
// IDA 0xc812a0: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c812a0() {
}


// 0xc8141c — __ZNK4Ogre6DynLib9getSymbolERKSs
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "Ogre::DynLib::getSymbol(std::string const&)const")]
// was: __ZNK4Ogre6DynLib9getSymbolERKSs
// IDA 0xc8141c: 51 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8141c() {
}


// 0xc814e4 — __ZN4Ogre13DynLibManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::DynLibManager *__hidden this)
#[doc(alias = "Ogre::DynLibManager::getSingleton(void)")]
// was: __ZN4Ogre13DynLibManager12getSingletonEv
// IDA 0xc814e4: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c814e4() {
}


// 0xc814f4 — __ZN4Ogre13DynLibManagerC1Ev
// type: _DWORD __fastcall(Ogre::DynLibManager *__hidden this)
#[doc(alias = "Ogre::DynLibManager::DynLibManager(void)")]
// was: __ZN4Ogre13DynLibManagerC1Ev
// IDA 0xc814f4: 22 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c814f4() {
}


// 0xc81538 — __ZN4Ogre13DynLibManager4loadERKSs
// type: _DWORD __fastcall(Ogre::DynLibManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::DynLibManager::load(std::string const&)")]
// was: __ZN4Ogre13DynLibManager4loadERKSs
// IDA 0xc81538: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c81538() {
}


// 0xc8162c — __ZN4Ogre13DynLibManager6unloadEPNS_6DynLibE
// type: _DWORD __fastcall(Ogre::DynLibManager *__hidden this, Ogre::DynLib *)
#[doc(alias = "Ogre::DynLibManager::unload(Ogre::DynLib *)")]
// was: __ZN4Ogre13DynLibManager6unloadEPNS_6DynLibE
// IDA 0xc8162c: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8162c() {
}
