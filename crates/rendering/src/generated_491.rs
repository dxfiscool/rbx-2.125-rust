//! rendering shard 491 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xcb5c6c..0xcb9a38, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|Gfx|Render|G3D), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcb5c6c — __ZN4Ogre21HardwareBufferManagerC2EPNS_25HardwareBufferManagerBaseE
#[doc(alias = "Ogre::HardwareBufferManager::HardwareBufferManager(Ogre::HardwareBufferManagerBase *)")]
// was: __ZN4Ogre21HardwareBufferManagerC2EPNS_25HardwareBufferManagerBaseE
// IDA 0xcb5c6c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5c6c() {
}


// 0xcb5d70 — __ZN4Ogre25HardwareBufferManagerBaseC2Ev
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::HardwareBufferManagerBase(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBaseC2Ev
// IDA 0xcb5d70: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb5d70() {
}


// 0xcb5e60 — __ZN4Ogre25HardwareBufferManagerBaseD2Ev
// type: void __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()")]
// was: __ZN4Ogre25HardwareBufferManagerBaseD2Ev
// IDA 0xcb5e60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb5e60() {
}


// 0xcb605c — __ZN4Ogre21HardwareBufferManagerD0Ev
// type: void __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::~HardwareBufferManager()")]
// was: __ZN4Ogre21HardwareBufferManagerD0Ev
// IDA 0xcb605c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb605c() {
}


// 0xcb60fc — __ZN4Ogre21HardwareBufferManagerD1Ev
// type: void __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::~HardwareBufferManager()")]
// was: __ZN4Ogre21HardwareBufferManagerD1Ev
// IDA 0xcb60fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb60fc() {
}


// 0xcb6114 — __ZN4Ogre21HardwareBufferManagerD2Ev
// type: void __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::~HardwareBufferManager()")]
// was: __ZN4Ogre21HardwareBufferManagerD2Ev
// IDA 0xcb6114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb6114() {
}


// 0xcb612c — __ZN4Ogre25HardwareBufferManagerBaseD0Ev
// type: void __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()")]
// was: __ZN4Ogre25HardwareBufferManagerBaseD0Ev
// IDA 0xcb612c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb612c() {
}


// 0xcb61bc — __ZN4Ogre25HardwareBufferManagerBaseD1Ev
// type: void __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::~HardwareBufferManagerBase()")]
// was: __ZN4Ogre25HardwareBufferManagerBaseD1Ev
// IDA 0xcb61bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb61bc() {
}


// 0xcb61c8 — __ZN4Ogre25HardwareBufferManagerBase23createVertexDeclarationEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexDeclaration(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase23createVertexDeclarationEv
// IDA 0xcb61c8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb61c8() {
}


// 0xcb61ec — __ZN4Ogre25HardwareBufferManagerBase24destroyVertexDeclarationEPNS_17VertexDeclarationE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::VertexDeclaration *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyVertexDeclaration(Ogre::VertexDeclaration *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase24destroyVertexDeclarationEPNS_17VertexDeclarationE
// IDA 0xcb61ec: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb61ec() {
}


// 0xcb624c — __ZN4Ogre25HardwareBufferManagerBase25createVertexBufferBindingEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexBufferBinding(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase25createVertexBufferBindingEv
// IDA 0xcb624c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb624c() {
}


// 0xcb6270 — __ZN4Ogre25HardwareBufferManagerBase26destroyVertexBufferBindingEPNS_19VertexBufferBindingE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::VertexBufferBinding *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyVertexBufferBinding(Ogre::VertexBufferBinding *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase26destroyVertexBufferBindingEPNS_19VertexBufferBindingE
// IDA 0xcb6270: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6270() {
}


// 0xcb62d0 — __ZN4Ogre25HardwareBufferManagerBase27createVertexDeclarationImplEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexDeclarationImpl(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase27createVertexDeclarationImplEv
// IDA 0xcb62d0: 65 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb62d0() {
}


// 0xcb6388 — __ZN4Ogre25HardwareBufferManagerBase28destroyVertexDeclarationImplEPNS_17VertexDeclarationE
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyVertexDeclarationImpl(Ogre::VertexDeclaration *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase28destroyVertexDeclarationImplEPNS_17VertexDeclarationE
// IDA 0xcb6388: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6388() {
}


// 0xcb639c — __ZN4Ogre25HardwareBufferManagerBase29createVertexBufferBindingImplEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::createVertexBufferBindingImpl(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase29createVertexBufferBindingImplEv
// IDA 0xcb639c: 65 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb639c() {
}


// 0xcb6454 — __ZN4Ogre25HardwareBufferManagerBase30destroyVertexBufferBindingImplEPNS_19VertexBufferBindingE
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyVertexBufferBindingImpl(Ogre::VertexBufferBinding *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase30destroyVertexBufferBindingImplEPNS_19VertexBufferBindingE
// IDA 0xcb6454: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6454() {
}


// 0xcb6468 — __ZN4Ogre25HardwareBufferManagerBase22destroyAllDeclarationsEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyAllDeclarations(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase22destroyAllDeclarationsEv
// IDA 0xcb6468: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6468() {
}


// 0xcb64a4 — __ZN4Ogre25HardwareBufferManagerBase18destroyAllBindingsEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::destroyAllBindings(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase18destroyAllBindingsEv
// IDA 0xcb64a4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb64a4() {
}


// 0xcb64e0 — __ZN4Ogre25HardwareBufferManagerBase33registerVertexBufferSourceAndCopyERKNS_29HardwareVertexBufferSharedPtrES3_
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, const Ogre::HardwareVertexBufferSharedPtr *, const Ogre::HardwareVertexBufferSharedPtr *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::registerVertexBufferSourceAndCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: __ZN4Ogre25HardwareBufferManagerBase33registerVertexBufferSourceAndCopyERKNS_29HardwareVertexBufferSharedPtrES3_
// IDA 0xcb64e0: 222 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb64e0() {
}


// 0xcb6704 — __ZN4Ogre25HardwareBufferManagerBase24allocateVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS0_17BufferLicenseTypeEPNS_22HardwareBufferLicenseeEb
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::HardwareBufferManagerBase::allocateVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBufferManagerBase::BufferLicenseType,Ogre::HardwareBufferLicensee *,bool)")]
// was: __ZN4Ogre25HardwareBufferManagerBase24allocateVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS0_17BufferLicenseTypeEPNS_22HardwareBufferLicenseeEb
// IDA 0xcb6704: 604 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6704() {
}


// 0xcb6cc0 — __ZN4Ogre25HardwareBufferManagerBase23releaseVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, const Ogre::HardwareVertexBufferSharedPtr *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::releaseVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: __ZN4Ogre25HardwareBufferManagerBase23releaseVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrE
// IDA 0xcb6cc0: 261 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6cc0() {
}


// 0xcb6f8c — __ZN4Ogre25HardwareBufferManagerBase23_freeUnusedBufferCopiesEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManagerBase::_freeUnusedBufferCopies(void)")]
// was: __ZN4Ogre25HardwareBufferManagerBase23_freeUnusedBufferCopiesEv
// IDA 0xcb6f8c: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6f8c() {
}


// 0xcb72c8 — __ZN4Ogre25HardwareBufferManagerBase20_releaseBufferCopiesEb
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, bool)
#[doc(alias = "Ogre::HardwareBufferManagerBase::_releaseBufferCopies(bool)")]
// was: __ZN4Ogre25HardwareBufferManagerBase20_releaseBufferCopiesEb
// IDA 0xcb72c8: 296 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb72c8() {
}


// 0xcb75ac — __ZN4Ogre25HardwareBufferManagerBase25_forceReleaseBufferCopiesERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManagerBase::_forceReleaseBufferCopies(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: __ZN4Ogre25HardwareBufferManagerBase25_forceReleaseBufferCopiesERKNS_29HardwareVertexBufferSharedPtrE
// IDA 0xcb75ac: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb75ac() {
}


// 0xcb75bc — __ZN4Ogre25HardwareBufferManagerBase25_forceReleaseBufferCopiesEPNS_20HardwareVertexBufferE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::HardwareVertexBuffer *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::_forceReleaseBufferCopies(Ogre::HardwareVertexBuffer *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase25_forceReleaseBufferCopiesEPNS_20HardwareVertexBufferE
// IDA 0xcb75bc: 234 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb75bc() {
}


// 0xcb7810 — __ZN4Ogre25HardwareBufferManagerBase28_notifyVertexBufferDestroyedEPNS_20HardwareVertexBufferE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::HardwareVertexBuffer *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::_notifyVertexBufferDestroyed(Ogre::HardwareVertexBuffer *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase28_notifyVertexBufferDestroyedEPNS_20HardwareVertexBufferE
// IDA 0xcb7810: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7810() {
}


// 0xcb7868 — __ZN4Ogre25HardwareBufferManagerBase27_notifyIndexBufferDestroyedEPNS_19HardwareIndexBufferE
// type: _DWORD __fastcall(Ogre::HardwareBufferManagerBase *__hidden this, Ogre::HardwareIndexBuffer *)
#[doc(alias = "Ogre::HardwareBufferManagerBase::_notifyIndexBufferDestroyed(Ogre::HardwareIndexBuffer *)")]
// was: __ZN4Ogre25HardwareBufferManagerBase27_notifyIndexBufferDestroyedEPNS_19HardwareIndexBufferE
// IDA 0xcb7868: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7868() {
}


// 0xcb78b4 — __ZN4Ogre25HardwareBufferManagerBase14makeBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::HardwareBufferManagerBase::makeBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBuffer::Usage,bool)")]
// was: __ZN4Ogre25HardwareBufferManagerBase14makeBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS_14HardwareBuffer5UsageEb
// IDA 0xcb78b4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb78b4() {
}


// 0xcb78e0 — __ZN4Ogre21TempBlendedBufferInfoD0Ev
// type: void __fastcall(Ogre::TempBlendedBufferInfo *__hidden this)
#[doc(alias = "Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()")]
// was: __ZN4Ogre21TempBlendedBufferInfoD0Ev
// IDA 0xcb78e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb78e0() {
}


// 0xcb7970 — __ZN4Ogre21TempBlendedBufferInfoD1Ev
// type: void __fastcall(Ogre::TempBlendedBufferInfo *__hidden this)
#[doc(alias = "Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()")]
// was: __ZN4Ogre21TempBlendedBufferInfoD1Ev
// IDA 0xcb7970: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb7970() {
}


// 0xcb797c — __ZN4Ogre21TempBlendedBufferInfoD2Ev
// type: void __fastcall(Ogre::TempBlendedBufferInfo *__hidden this)
#[doc(alias = "Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()")]
// was: __ZN4Ogre21TempBlendedBufferInfoD2Ev
// IDA 0xcb797c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb797c() {
}


// 0xcb7dc0 — __ZN4Ogre21TempBlendedBufferInfo11extractFromEPKNS_10VertexDataE
// type: _DWORD __fastcall(Ogre::TempBlendedBufferInfo *__hidden this, const Ogre::VertexData *)
#[doc(alias = "Ogre::TempBlendedBufferInfo::extractFrom(Ogre::VertexData const*)")]
// was: __ZN4Ogre21TempBlendedBufferInfo11extractFromEPKNS_10VertexDataE
// IDA 0xcb7dc0: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7dc0() {
}


// 0xcb7e94 — __ZN4Ogre21TempBlendedBufferInfo18checkoutTempCopiesEbb
// type: _DWORD __fastcall(Ogre::TempBlendedBufferInfo *__hidden this, bool, bool)
#[doc(alias = "Ogre::TempBlendedBufferInfo::checkoutTempCopies(bool,bool)")]
// was: __ZN4Ogre21TempBlendedBufferInfo18checkoutTempCopiesEbb
// IDA 0xcb7e94: 287 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7e94() {
}


// 0xcb815c — __ZNK4Ogre21TempBlendedBufferInfo17buffersCheckedOutEbb
// type: _DWORD __fastcall(Ogre::TempBlendedBufferInfo *__hidden this, bool, bool)
#[doc(alias = "Ogre::TempBlendedBufferInfo::buffersCheckedOut(bool,bool)const")]
// was: __ZNK4Ogre21TempBlendedBufferInfo17buffersCheckedOutEbb
// IDA 0xcb815c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb815c() {
}


// 0xcb81ac — __ZN4Ogre21TempBlendedBufferInfo14bindTempCopiesEPNS_10VertexDataEb
#[doc(alias = "Ogre::TempBlendedBufferInfo::bindTempCopies(Ogre::VertexData *,bool)")]
// was: __ZN4Ogre21TempBlendedBufferInfo14bindTempCopiesEPNS_10VertexDataEb
// IDA 0xcb81ac: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb81ac() {
}


// 0xcb8214 — __ZN4Ogre21TempBlendedBufferInfo14licenseExpiredEPNS_14HardwareBufferE
#[doc(alias = "Ogre::TempBlendedBufferInfo::licenseExpired(Ogre::HardwareBuffer *)")]
// was: __ZN4Ogre21TempBlendedBufferInfo14licenseExpiredEPNS_14HardwareBufferE
// IDA 0xcb8214: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8214() {
}


// 0xcb8270 — __ZN4Ogre21HardwareBufferManager18createVertexBufferEmmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::HardwareBufferManager::createVertexBuffer(unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: __ZN4Ogre21HardwareBufferManager18createVertexBufferEmmNS_14HardwareBuffer5UsageEb
// IDA 0xcb8270: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8270() {
}


// 0xcb8290 — __ZN4Ogre21HardwareBufferManager17createIndexBufferENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::HardwareBufferManager::createIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: __ZN4Ogre21HardwareBufferManager17createIndexBufferENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageEb
// IDA 0xcb8290: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8290() {
}


// 0xcb82b0 — __ZN4Ogre21HardwareBufferManager26createRenderToVertexBufferEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::createRenderToVertexBuffer(void)")]
// was: __ZN4Ogre21HardwareBufferManager26createRenderToVertexBufferEv
// IDA 0xcb82b0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82b0() {
}


// 0xcb82c0 — __ZN4Ogre21HardwareBufferManager23createVertexDeclarationEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::createVertexDeclaration(void)")]
// was: __ZN4Ogre21HardwareBufferManager23createVertexDeclarationEv
// IDA 0xcb82c0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82c0() {
}


// 0xcb82d0 — __ZN4Ogre21HardwareBufferManager24destroyVertexDeclarationEPNS_17VertexDeclarationE
#[doc(alias = "Ogre::HardwareBufferManager::destroyVertexDeclaration(Ogre::VertexDeclaration *)")]
// was: __ZN4Ogre21HardwareBufferManager24destroyVertexDeclarationEPNS_17VertexDeclarationE
// IDA 0xcb82d0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82d0() {
}


// 0xcb82e0 — __ZN4Ogre21HardwareBufferManager25createVertexBufferBindingEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::createVertexBufferBinding(void)")]
// was: __ZN4Ogre21HardwareBufferManager25createVertexBufferBindingEv
// IDA 0xcb82e0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82e0() {
}


// 0xcb82f0 — __ZN4Ogre21HardwareBufferManager26destroyVertexBufferBindingEPNS_19VertexBufferBindingE
#[doc(alias = "Ogre::HardwareBufferManager::destroyVertexBufferBinding(Ogre::VertexBufferBinding *)")]
// was: __ZN4Ogre21HardwareBufferManager26destroyVertexBufferBindingEPNS_19VertexBufferBindingE
// IDA 0xcb82f0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82f0() {
}


// 0xcb8300 — __ZN4Ogre21HardwareBufferManager33registerVertexBufferSourceAndCopyERKNS_29HardwareVertexBufferSharedPtrES3_
#[doc(alias = "Ogre::HardwareBufferManager::registerVertexBufferSourceAndCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: __ZN4Ogre21HardwareBufferManager33registerVertexBufferSourceAndCopyERKNS_29HardwareVertexBufferSharedPtrES3_
// IDA 0xcb8300: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8300() {
}


// 0xcb8310 — __ZN4Ogre21HardwareBufferManager24allocateVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS_25HardwareBufferManagerBase17BufferLicenseTypeEPNS_22HardwareBufferLicenseeEb
#[doc(alias = "Ogre::HardwareBufferManager::allocateVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBufferManagerBase::BufferLicenseType,Ogre::HardwareBufferLicensee *,bool)")]
// was: __ZN4Ogre21HardwareBufferManager24allocateVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS_25HardwareBufferManagerBase17BufferLicenseTypeEPNS_22HardwareBufferLicenseeEb
// IDA 0xcb8310: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8310() {
}


// 0xcb8330 — __ZN4Ogre21HardwareBufferManager23releaseVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManager::releaseVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: __ZN4Ogre21HardwareBufferManager23releaseVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrE
// IDA 0xcb8330: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8330() {
}


// 0xcb8350 — __ZN4Ogre21HardwareBufferManager23_freeUnusedBufferCopiesEv
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this)
#[doc(alias = "Ogre::HardwareBufferManager::_freeUnusedBufferCopies(void)")]
// was: __ZN4Ogre21HardwareBufferManager23_freeUnusedBufferCopiesEv
// IDA 0xcb8350: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8350() {
}


// 0xcb8360 — __ZN4Ogre21HardwareBufferManager20_releaseBufferCopiesEb
// type: _DWORD __fastcall(Ogre::HardwareBufferManager *__hidden this, bool)
#[doc(alias = "Ogre::HardwareBufferManager::_releaseBufferCopies(bool)")]
// was: __ZN4Ogre21HardwareBufferManager20_releaseBufferCopiesEb
// IDA 0xcb8360: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8360() {
}


// 0xcb8370 — __ZN4Ogre21HardwareBufferManager25_forceReleaseBufferCopiesERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManager::_forceReleaseBufferCopies(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: __ZN4Ogre21HardwareBufferManager25_forceReleaseBufferCopiesERKNS_29HardwareVertexBufferSharedPtrE
// IDA 0xcb8370: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8370() {
}


// 0xcb8380 — __ZN4Ogre21HardwareBufferManager25_forceReleaseBufferCopiesEPNS_20HardwareVertexBufferE
#[doc(alias = "Ogre::HardwareBufferManager::_forceReleaseBufferCopies(Ogre::HardwareVertexBuffer *)")]
// was: __ZN4Ogre21HardwareBufferManager25_forceReleaseBufferCopiesEPNS_20HardwareVertexBufferE
// IDA 0xcb8380: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8380() {
}


// 0xcb8390 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_
// IDA 0xcb8390: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8390() {
}


// 0xcb83f8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
// IDA 0xcb83f8: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb83f8() {
}


// 0xcb84f4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
// IDA 0xcb84f4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb84f4() {
}


// 0xcb851c — __ZNSt10_List_baseIN4Ogre29HardwareVertexBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::HardwareVertexBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwareVertexBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIN4Ogre29HardwareVertexBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
// IDA 0xcb851c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb851c() {
}


// 0xcb8520 — __ZNSt10_List_baseIN4Ogre29HardwareVertexBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::HardwareVertexBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwareVertexBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: __ZNSt10_List_baseIN4Ogre29HardwareVertexBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
// IDA 0xcb8520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8520() {
}


// 0xcb852c — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
// IDA 0xcb852c: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb852c() {
}


// 0xcb8628 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
// IDA 0xcb8628: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8628() {
}


// 0xcb8694 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
// IDA 0xcb8694: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8694() {
}


// 0xcb874c — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0xcb874c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb874c() {
}


// 0xcb8774 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0xcb8774: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8774() {
}


// 0xcb879c — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)")]
// was: __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// IDA 0xcb879c: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb879c() {
}


// 0xcb8800 — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: _DWORD *__fastcall(char *, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// IDA 0xcb8800: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8800() {
}


// 0xcb88f8 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)")]
// was: __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// IDA 0xcb88f8: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb88f8() {
}


// 0xcb895c — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: _DWORD *__fastcall(char *, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)")]
// was: __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// IDA 0xcb895c: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb895c() {
}


// 0xcb8a54 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0xcb8a54: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8a54() {
}


// 0xcb8a7c — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(int result, Ogre::NedPoolingImpl *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0xcb8a7c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8a7c() {
}


// 0xcb8aa4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
// IDA 0xcb8aa4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8aa4() {
}


// 0xcb8aa8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
// IDA 0xcb8aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8aa8() {
}


// 0xcb8ab4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
// IDA 0xcb8ab4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ab4() {
}


// 0xcb8ab8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
// IDA 0xcb8ab8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ab8() {
}


// 0xcb8ac4 — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexBufferBinding *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// IDA 0xcb8ac4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ac4() {
}


// 0xcb8ac8 — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexBufferBinding *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// IDA 0xcb8ac8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ac8() {
}


// 0xcb8ad4 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexDeclaration *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// IDA 0xcb8ad4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ad4() {
}


// 0xcb8ad8 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexDeclaration *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// IDA 0xcb8ad8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ad8() {
}


// 0xcb8ae4 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareIndexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// IDA 0xcb8ae4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ae4() {
}


// 0xcb8ae8 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareIndexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// IDA 0xcb8ae8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ae8() {
}


// 0xcb8af4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// type: void()
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
// IDA 0xcb8af4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8af4() {
}


// 0xcb8af8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
// IDA 0xcb8af8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8af8() {
}


// 0xcb8b04 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
// was: __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0xcb8b04: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8b04() {
}


// 0xcb8b60 — __ZN4Ogre19HardwareIndexBufferC2EPNS_25HardwareBufferManagerBaseENS0_9IndexTypeEmNS_14HardwareBuffer5UsageEbb
// type: Ogre::HardwareIndexBuffer *__fastcall(int, int, int, int, int, char, int)
#[doc(alias = "Ogre::HardwareIndexBuffer::HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: __ZN4Ogre19HardwareIndexBufferC2EPNS_25HardwareBufferManagerBaseENS0_9IndexTypeEmNS_14HardwareBuffer5UsageEbb
// IDA 0xcb8b60: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8b60() {
}


// 0xcb8ccc — __ZN4Ogre19HardwareIndexBufferD0Ev
// type: void __fastcall(Ogre::HardwareIndexBuffer *this, void *)
#[doc(alias = "Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()")]
// was: __ZN4Ogre19HardwareIndexBufferD0Ev
// IDA 0xcb8ccc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ccc() {
}


// 0xcb8d88 — __ZN4Ogre19HardwareIndexBufferD1Ev
// type: void __fastcall(Ogre::HardwareIndexBuffer *__hidden this)
#[doc(alias = "Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()")]
// was: __ZN4Ogre19HardwareIndexBufferD1Ev
// IDA 0xcb8d88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8d88() {
}


// 0xcb8e38 — __ZN4Ogre19HardwareIndexBufferD2Ev
// type: void __fastcall(Ogre::HardwareIndexBuffer *__hidden this)
#[doc(alias = "Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()")]
// was: __ZN4Ogre19HardwareIndexBufferD2Ev
// IDA 0xcb8e38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8e38() {
}


// 0xcb8ee8 — __ZN4Ogre28HardwareIndexBufferSharedPtrC1EPNS_19HardwareIndexBufferE
// type: int __fastcall(__int64 this)
#[doc(alias = "Ogre::HardwareIndexBufferSharedPtr::HardwareIndexBufferSharedPtr(Ogre::HardwareIndexBuffer *)")]
// was: __ZN4Ogre28HardwareIndexBufferSharedPtrC1EPNS_19HardwareIndexBufferE
// IDA 0xcb8ee8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8ee8() {
}


// 0xcb8f70 — __ZN4Ogre22HardwareOcclusionQueryC2Ev
// type: int __fastcall(int this)
#[doc(alias = "Ogre::HardwareOcclusionQuery::HardwareOcclusionQuery(void)")]
// was: __ZN4Ogre22HardwareOcclusionQueryC2Ev
// IDA 0xcb8f70: 8 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8f70() {
}


// 0xcb8f88 — __ZN4Ogre22HardwareOcclusionQueryD0Ev
// type: void __fastcall(Ogre::HardwareOcclusionQuery *this, void *)
#[doc(alias = "Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()")]
// was: __ZN4Ogre22HardwareOcclusionQueryD0Ev
// IDA 0xcb8f88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8f88() {
}


// 0xcb9014 — __ZN4Ogre22HardwareOcclusionQueryD1Ev
// type: void __fastcall(Ogre::HardwareOcclusionQuery *__hidden this)
#[doc(alias = "Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()")]
// was: __ZN4Ogre22HardwareOcclusionQueryD1Ev
// IDA 0xcb9014: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb9014() {
}


// 0xcb9018 — __ZN4Ogre22HardwareOcclusionQueryD2Ev
// type: void __fastcall(Ogre::HardwareOcclusionQuery *__hidden this)
#[doc(alias = "Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()")]
// was: __ZN4Ogre22HardwareOcclusionQueryD2Ev
// IDA 0xcb9018: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb9018() {
}


// 0xcb9050 — __ZN4Ogre19HardwarePixelBufferC2EmmmNS_11PixelFormatENS_14HardwareBuffer5UsageEbb
// type: int __fastcall(int, int, int, int, int, int, char, int)
#[doc(alias = "Ogre::HardwarePixelBuffer::HardwarePixelBuffer(unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: __ZN4Ogre19HardwarePixelBufferC2EmmmNS_11PixelFormatENS_14HardwareBuffer5UsageEbb
// IDA 0xcb9050: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9050() {
}


// 0xcb90f4 — __ZN4Ogre19HardwarePixelBufferD0Ev
// type: void __fastcall(Ogre::HardwarePixelBuffer *this, void *)
#[doc(alias = "Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()")]
// was: __ZN4Ogre19HardwarePixelBufferD0Ev
// IDA 0xcb90f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb90f4() {
}


// 0xcb9190 — __ZN4Ogre19HardwarePixelBufferD1Ev
// type: void __fastcall(Ogre::HardwarePixelBuffer *__hidden this)
#[doc(alias = "Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()")]
// was: __ZN4Ogre19HardwarePixelBufferD1Ev
// IDA 0xcb9190: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb9190() {
}


// 0xcb91a0 — __ZN4Ogre19HardwarePixelBufferD2Ev
// type: void __fastcall(Ogre::HardwarePixelBuffer *__hidden this)
#[doc(alias = "Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()")]
// was: __ZN4Ogre19HardwarePixelBufferD2Ev
// IDA 0xcb91a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb91a0() {
}


// 0xcb91b0 — __ZN4Ogre19HardwarePixelBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "Ogre::HardwarePixelBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre19HardwarePixelBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
// IDA 0xcb91b0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb91b0() {
}


// 0xcb91e4 — __ZN4Ogre19HardwarePixelBuffer4lockERKNS_3BoxENS_14HardwareBuffer11LockOptionsE
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "Ogre::HardwarePixelBuffer::lock(Ogre::Box const&,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre19HardwarePixelBuffer4lockERKNS_3BoxENS_14HardwareBuffer11LockOptionsE
// IDA 0xcb91e4: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb91e4() {
}


// 0xcb9280 — __ZN4Ogre19HardwarePixelBuffer14getCurrentLockEv
// type: char *__fastcall(Ogre::HardwarePixelBuffer *this)
#[doc(alias = "Ogre::HardwarePixelBuffer::getCurrentLock(void)")]
// was: __ZN4Ogre19HardwarePixelBuffer14getCurrentLockEv
// IDA 0xcb9280: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9280() {
}


// 0xcb9284 — __ZN4Ogre19HardwarePixelBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
// type: void __noreturn()
#[doc(alias = "Ogre::HardwarePixelBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: __ZN4Ogre19HardwarePixelBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
// IDA 0xcb9284: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9284() {
}


// 0xcb9434 — __ZN4Ogre19HardwarePixelBuffer4blitERKNS_28HardwarePixelBufferSharedPtrERKNS_3BoxES6_
// type: void __fastcall(int, int, int, _DWORD *)
#[doc(alias = "Ogre::HardwarePixelBuffer::blit(Ogre::HardwarePixelBufferSharedPtr const&,Ogre::Box const&,Ogre::Box const&)")]
// was: __ZN4Ogre19HardwarePixelBuffer4blitERKNS_28HardwarePixelBufferSharedPtrERKNS_3BoxES6_
// IDA 0xcb9434: 374 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9434() {
}


// 0xcb983c — __ZN4Ogre19HardwarePixelBuffer4blitERKNS_28HardwarePixelBufferSharedPtrE
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "Ogre::HardwarePixelBuffer::blit(Ogre::HardwarePixelBufferSharedPtr const&)")]
// was: __ZN4Ogre19HardwarePixelBuffer4blitERKNS_28HardwarePixelBufferSharedPtrE
// IDA 0xcb983c: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb983c() {
}


// 0xcb9888 — __ZN4Ogre19HardwarePixelBuffer8readDataEmmPv
// type: void __fastcall __noreturn(Ogre::HardwarePixelBuffer *this, unsigned int, unsigned int, void *)
#[doc(alias = "Ogre::HardwarePixelBuffer::readData(unsigned long,unsigned long,void *)")]
// was: __ZN4Ogre19HardwarePixelBuffer8readDataEmmPv
// IDA 0xcb9888: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9888() {
}


// 0xcb9a38 — __ZN4Ogre19HardwarePixelBuffer9writeDataEmmPKvb
// type: void __fastcall __noreturn(Ogre::HardwarePixelBuffer *this, unsigned int, unsigned int, const void *, bool)
#[doc(alias = "Ogre::HardwarePixelBuffer::writeData(unsigned long,unsigned long,void const*,bool)")]
// was: __ZN4Ogre19HardwarePixelBuffer9writeDataEmmPKvb
// IDA 0xcb9a38: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9a38() {
}

