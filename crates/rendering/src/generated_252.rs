//! rendering shard 252 — 100 stubs EA-sorted asc global gap filler after 0x2d0808 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 27370->27470 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2d08a4 — __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv
// IDA 0x2d08a4: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d08a4() {
}

// 0x2d092c — __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator6createEv
// IDA 0x2d092c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d092c() {
}

// 0x2d0a70 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13AdvLuaDraggerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::AdvLuaDragger>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13AdvLuaDraggerEEEN5boost10shared_ptrIT_EEv
// IDA 0x2d0a70: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0a70() {
}

// 0x2d0b20 — __ZN5boost10shared_ptrIN3RBX13AdvLuaDraggerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::shared_ptr<RBX::AdvLuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13AdvLuaDraggerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2d0b20: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0b20() {
}

// 0x2d0be8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13AdvLuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(rbx_core::SharedPtr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13AdvLuaDraggerES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2d0be8: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0be8() {
}

// 0x2d0cd0 — __ZN5boost6detail12shared_countC2IPN3RBX13AdvLuaDraggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13AdvLuaDraggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2d0cd0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0cd0() {
}

// 0x2d0dd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2d0dd8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2d0dd8() {
}

// 0x2d0ddc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x2d0ddc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d0ddc() {
}

// 0x2d0de0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2d0de0: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0de0() {
}

// 0x2d0e00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2d0e00: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0e00() {
}

// 0x2d0e18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvLuaDraggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2d0e18: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0e18() {
}

// 0x2d0e1c — __ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv
// IDA 0x2d0e1c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d0e1c() {
}

// 0x2d0e20 — __ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v
// IDA 0x2d0e20: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0e20() {
}

// 0x2d0f00 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev
// IDA 0x2d0f00: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0f00() {
}

// 0x2d1144 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv
// IDA 0x2d1144: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d1144() {
}

// 0x2d11b8 — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev
#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")]
// was: __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev
// IDA 0x2d11b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d11b8() {
}

// 0x2d1260 — __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2d1260: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d1260() {
}

// 0x2d1264 — __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2d1264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d1264() {
}

// 0x2d1304 — __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2d1304: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d1304() {
}

// 0x2d130c — __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2d130c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d130c() {
}

// 0x2d13b0 — __ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x2d13b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d13b0() {
}

// 0x2d13b8 — __ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x2d13b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d13b8() {
}

// 0x2d145c — __GLOBAL__I_a_78
#[doc(alias = "global constructor keyed to_a_78")]
// was: __GLOBAL__I_a_78
// IDA 0x2d145c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2d145c() {
}

// 0x2d1a5c — __ZN3RBX14AdvLuaDragToolD0Ev
// type: void __fastcall(RBX::AdvLuaDragTool *__hidden this)
#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
// was: __ZN3RBX14AdvLuaDragToolD0Ev
// IDA 0x2d1a5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d1a5c() {
}

// 0x2d1afc — __ZN3RBX14AdvLuaDragToolD1Ev
// type: void __fastcall(RBX::AdvLuaDragTool *__hidden this)
#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
// was: __ZN3RBX14AdvLuaDragToolD1Ev
// IDA 0x2d1afc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d1afc() {
}

// 0x2d1b00 — __ZThn36_N3RBX14AdvLuaDragToolD0Ev
// type: void __fastcall(RBX::AdvLuaDragTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragTool::~AdvLuaDragTool()")]
// was: __ZThn36_N3RBX14AdvLuaDragToolD0Ev
// IDA 0x2d1b00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d1b00() {
}

// 0x2d1b08 — __ZN3RBX14AdvLuaDragToolD2Ev
// type: void __fastcall(RBX::AdvLuaDragTool *__hidden this)
#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
// was: __ZN3RBX14AdvLuaDragToolD2Ev
// IDA 0x2d1b08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d1b08() {
}

// 0x2d1c48 — __ZThn36_N3RBX14AdvLuaDragToolD1Ev
// type: void __fastcall(RBX::AdvLuaDragTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragTool::~AdvLuaDragTool()")]
// was: __ZThn36_N3RBX14AdvLuaDragToolD1Ev
// IDA 0x2d1c48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d1c48() {
}

// 0x2d1c50 — __ZN3RBX14AdvLuaDragTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvLuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvLuaDragTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX14AdvLuaDragTool11onMouseDownERKNS_7UIEventE
// IDA 0x2d1c50: 172 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d1c50() {
}

// 0x2d1e34 — __ZN3RBX14AdvLuaDragTool11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvLuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvLuaDragTool::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX14AdvLuaDragTool11onMouseMoveERKNS_7UIEventE
// IDA 0x2d1e34: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d1e34() {
}

// 0x2d1edc — __ZN3RBX14AdvLuaDragTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvLuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvLuaDragTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX14AdvLuaDragTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2d1edc: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d1edc() {
}

// 0x2d1f5c — __ZN3RBX14AdvLuaDragTool9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvLuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvLuaDragTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX14AdvLuaDragTool9onMouseUpERKNS_7UIEventE
// IDA 0x2d1f5c: 224 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d1f5c() {
}

// 0x2d21d8 — __ZN3RBX14AdvLuaDragTool9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvLuaDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvLuaDragTool::onKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX14AdvLuaDragTool9onKeyDownERKNS_7UIEventE
// IDA 0x2d21d8: 145 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d21d8() {
}

// 0x2d2374 — __ZN5boost10shared_ptrIN3RBX13AdvLuaDraggerEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::operator=(rbx_core::SharedPtr<RBX::AdvLuaDragger> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13AdvLuaDraggerEEaSERKS3_
// IDA 0x2d2374: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2374() {
}

// 0x2d23ac — __ZN3RBX11shared_fromINS_14AdvLuaDragToolEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::shared_from<RBX::AdvLuaDragTool>(RBX::AdvLuaDragTool*)")]
// was: __ZN3RBX11shared_fromINS_14AdvLuaDragToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2d23ac: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d23ac() {
}

// 0x2d2514 — __ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv
// IDA 0x2d2514: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2514() {
}

// 0x2d253c — __ZNK3RBX14AdvLuaDragTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::AdvLuaDragTool *__hidden this)
#[doc(alias = "RBX::AdvLuaDragTool::getCursorName(void)const")]
// was: __ZNK3RBX14AdvLuaDragTool13getCursorNameEv
// IDA 0x2d253c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d253c() {
}

// 0x2d2548 — __ZN3RBX14AdvLuaDragTool9setCursorESs
#[doc(alias = "RBX::AdvLuaDragTool::setCursor(std::string)")]
// was: __ZN3RBX14AdvLuaDragTool9setCursorESs
// IDA 0x2d2548: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2548() {
}

// 0x2d2550 — __ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv
// IDA 0x2d2550: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d2550() {
}

// 0x2d2554 — __ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v
// IDA 0x2d2554: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2554() {
}

// 0x2d2634 — __GLOBAL__I_a_79
#[doc(alias = "global constructor keyed to_a_79")]
// was: __GLOBAL__I_a_79
// IDA 0x2d2634: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2d2634() {
}

// 0x2d28a4 — __ZN3RBX15AdvMoveToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::AdvMoveToolBase::AdvMoveToolBase(RBX::Workspace *)")]
// was: __ZN3RBX15AdvMoveToolBaseC2EPNS_9WorkspaceE
// IDA 0x2d28a4: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d28a4() {
}

// 0x2d2a94 — __ZN3RBX15AdvMoveToolBase12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveToolBase::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX15AdvMoveToolBase12onMouseHoverERKNS_7UIEventE
// IDA 0x2d2a94: 11 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2a94() {
}

// 0x2d2ab0 — __ZN3RBX15AdvMoveToolBase11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveToolBase::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX15AdvMoveToolBase11onMouseIdleERKNS_7UIEventE
// IDA 0x2d2ab0: 135 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2ab0() {
}

// 0x2d2c3c — __ZN3RBX15AdvMoveToolBase11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveToolBase::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX15AdvMoveToolBase11onMouseDownERKNS_7UIEventE
// IDA 0x2d2c3c: 284 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2c3c() {
}

// 0x2d2f40 — __ZN3RBX15AdvMoveToolBase30saveAndModifyPartsTransparencyEv
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "RBX::AdvMoveToolBase::saveAndModifyPartsTransparency(void)")]
// was: __ZN3RBX15AdvMoveToolBase30saveAndModifyPartsTransparencyEv
// IDA 0x2d2f40: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d2f40() {
}

// 0x2d3174 — __ZN3RBX15AdvMoveToolBase11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveToolBase::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX15AdvMoveToolBase11onMouseMoveERKNS_7UIEventE
// IDA 0x2d3174: 912 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d3174() {
}

// 0x2d421c — __ZN3RBX15AdvMoveToolBase9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveToolBase::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX15AdvMoveToolBase9onMouseUpERKNS_7UIEventE
// IDA 0x2d421c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d421c() {
}

// 0x2d427c — __ZN3RBX15AdvMoveToolBase29restoreSavedPartsTransparencyEv
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "RBX::AdvMoveToolBase::restoreSavedPartsTransparency(void)")]
// was: __ZN3RBX15AdvMoveToolBase29restoreSavedPartsTransparencyEv
// IDA 0x2d427c: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d427c() {
}

// 0x2d43a4 — __ZN3RBX15AdvMoveToolBase9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveToolBase::onKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX15AdvMoveToolBase9onKeyDownERKNS_7UIEventE
// IDA 0x2d43a4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d43a4() {
}

// 0x2d45b8 — __ZNK3RBX15AdvMoveToolBase10getExtentsERNS_7ExtentsE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Extents *)
#[doc(alias = "RBX::AdvMoveToolBase::getExtents(RBX::Extents &)const")]
// was: __ZNK3RBX15AdvMoveToolBase10getExtentsERNS_7ExtentsE
// IDA 0x2d45b8: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d45b8() {
}

// 0x2d496c — __ZN3RBX11AdvMoveTool11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvMoveTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvMoveTool::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX11AdvMoveTool11onMouseDownERKNS_7UIEventE
// IDA 0x2d496c: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d496c() {
}

// 0x2d5218 — __ZNSt3mapIN5boost8weak_ptrIN3RBX12PartInstanceEEEfSt4lessIS4_ESaISt4pairIKS4_fEEEixERS8_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "std::map<rbx_core::WeakPtr<RBX::PartInstance>,float,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::operator[](rbx_core::WeakPtr<RBX::PartInstance> const&)")]
// was: __ZNSt3mapIN5boost8weak_ptrIN3RBX12PartInstanceEEEfSt4lessIS4_ESaISt4pairIKS4_fEEEixERS8_
// IDA 0x2d5218: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5218() {
}

// 0x2d5368 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// IDA 0x2d5368: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5368() {
}

// 0x2d541c — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// IDA 0x2d541c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d541c() {
}

// 0x2d5468 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_insert_unique(std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE16_M_insert_uniqueERKS7_
// IDA 0x2d5468: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5468() {
}

// 0x2d54d0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_create_node(std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float> const&)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE14_M_create_nodeERKS7_
// IDA 0x2d54d0: 106 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d54d0() {
}

// 0x2d55f8 — __GLOBAL__I_a_80
#[doc(alias = "global constructor keyed to_a_80")]
// was: __GLOBAL__I_a_80
// IDA 0x2d55f8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2d55f8() {
}

// 0x2d5fd0 — __ZN3RBX13AdvRotateToolD1Ev
// type: void __fastcall(RBX::AdvRotateTool *__hidden this)
#[doc(alias = "RBX::AdvRotateTool::~AdvRotateTool()")]
// was: __ZN3RBX13AdvRotateToolD1Ev
// IDA 0x2d5fd0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d5fd0() {
}

// 0x2d5fd4 — __ZN3RBX13AdvRotateToolD0Ev
// type: void __fastcall(RBX::AdvRotateTool *__hidden this)
#[doc(alias = "RBX::AdvRotateTool::~AdvRotateTool()")]
// was: __ZN3RBX13AdvRotateToolD0Ev
// IDA 0x2d5fd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d5fd4() {
}

// 0x2d6074 — __ZThn36_N3RBX13AdvRotateToolD1Ev
// type: void __fastcall(RBX::AdvRotateTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvRotateTool::~AdvRotateTool()")]
// was: __ZThn36_N3RBX13AdvRotateToolD1Ev
// IDA 0x2d6074: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d6074() {
}

// 0x2d607c — __ZThn36_N3RBX13AdvRotateToolD0Ev
// type: void __fastcall(RBX::AdvRotateTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvRotateTool::~AdvRotateTool()")]
// was: __ZThn36_N3RBX13AdvRotateToolD0Ev
// IDA 0x2d607c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d607c() {
}

// 0x2d6120 — __GLOBAL__I_a_81
#[doc(alias = "global constructor keyed to_a_81")]
// was: __GLOBAL__I_a_81
// IDA 0x2d6120: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2d6120() {
}

// 0x2d6390 — __ZN3RBX13AdvRunDragger8SnapInfo20updateSurfaceFromHitEv
// type: _DWORD __fastcall(RBX::AdvRunDragger::SnapInfo *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::SnapInfo::updateSurfaceFromHit(void)")]
// was: __ZN3RBX13AdvRunDragger8SnapInfo20updateSurfaceFromHitEv
// IDA 0x2d6390: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d6390() {
}

// 0x2d64ac — __ZN3RBX13AdvRunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::AdvRunDragger::SnapInfo *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::AdvRunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&)")]
// was: __ZN3RBX13AdvRunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE
// IDA 0x2d64ac: 246 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d64ac() {
}

// 0x2d6784 — __ZN3RBX13AdvRunDragger8SnapInfo17hitOutsideExtentsEv
// type: _DWORD __fastcall(RBX::AdvRunDragger::SnapInfo *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::SnapInfo::hitOutsideExtents(void)")]
// was: __ZN3RBX13AdvRunDragger8SnapInfo17hitOutsideExtentsEv
// IDA 0x2d6784: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d6784() {
}

// 0x2d68d8 — __ZN3RBX13AdvRunDraggerC1Ev
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::AdvRunDragger(void)")]
// was: __ZN3RBX13AdvRunDraggerC1Ev
// IDA 0x2d68d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d68d8() {
}

// 0x2d68dc — __ZN3RBX13AdvRunDraggerC2Ev
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::AdvRunDragger(void)")]
// was: __ZN3RBX13AdvRunDraggerC2Ev
// IDA 0x2d68dc: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d68dc() {
}

// 0x2d6ac8 — __ZN3RBX13AdvRunDraggerD1Ev
// type: void __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::~AdvRunDragger()")]
// was: __ZN3RBX13AdvRunDraggerD1Ev
// IDA 0x2d6ac8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d6ac8() {
}

// 0x2d6acc — __ZN3RBX13AdvRunDraggerD2Ev
// type: void __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::~AdvRunDragger()")]
// was: __ZN3RBX13AdvRunDraggerD2Ev
// IDA 0x2d6acc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d6acc() {
}

// 0x2d6c30 — __ZN3RBX13AdvRunDragger20snapInfoFromSnapPartEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::snapInfoFromSnapPart(void)")]
// was: __ZN3RBX13AdvRunDragger20snapInfoFromSnapPartEv
// IDA 0x2d6c30: 247 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d6c30() {
}

// 0x2d6ed8 — __ZN3RBX13AdvRunDragger20snapPartFromSnapInfoEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::snapPartFromSnapInfo(void)")]
// was: __ZN3RBX13AdvRunDragger20snapPartFromSnapInfoEv
// IDA 0x2d6ed8: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d6ed8() {
}

// 0x2d79c8 — __ZN3RBX13AdvRunDragger12moveDragPartEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::moveDragPart(void)")]
// was: __ZN3RBX13AdvRunDragger12moveDragPartEv
// IDA 0x2d79c8: 805 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d79c8() {
}

// 0x2d83d0 — __ZN3RBX13AdvRunDragger19getSnapSurfaceCoordEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::getSnapSurfaceCoord(void)")]
// was: __ZN3RBX13AdvRunDragger19getSnapSurfaceCoordEv
// IDA 0x2d83d0: 142 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d83d0() {
}

// 0x2d8564 — __ZN3RBX13AdvRunDragger12snapDragPartEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::snapDragPart(void)")]
// was: __ZN3RBX13AdvRunDragger12snapDragPartEv
// IDA 0x2d8564: 358 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8564() {
}

// 0x2d8a78 — __ZN3RBX13AdvRunDragger8adjacentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::AdvRunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")]
// was: __ZN3RBX13AdvRunDragger8adjacentEPNS_9PrimitiveES2_
// IDA 0x2d8a78: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8a78() {
}

// 0x2d8de4 — __ZN3RBX13AdvRunDragger11fallOffEdgeEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::fallOffEdge(void)")]
// was: __ZN3RBX13AdvRunDragger11fallOffEdgeEv
// IDA 0x2d8de4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8de4() {
}

// 0x2d8e1c — __ZN3RBX13AdvRunDragger11fallOffPartERb
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this, bool *)
#[doc(alias = "RBX::AdvRunDragger::fallOffPart(bool &)")]
// was: __ZN3RBX13AdvRunDragger11fallOffPartERb
// IDA 0x2d8e1c: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8e1c() {
}

// 0x2d8f8c — __ZN3RBX13AdvRunDragger17rayHitsCloserPartEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::rayHitsCloserPart(void)")]
// was: __ZN3RBX13AdvRunDragger17rayHitsCloserPartEv
// IDA 0x2d8f8c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8f8c() {
}

// 0x2d90dc — __ZN3RBX13AdvRunDragger16tooCloseToCameraEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::tooCloseToCamera(void)")]
// was: __ZN3RBX13AdvRunDragger16tooCloseToCameraEv
// IDA 0x2d90dc: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d90dc() {
}

// 0x2d9430 — __ZN3RBX13AdvRunDragger9findSafeYEv
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this)
#[doc(alias = "RBX::AdvRunDragger::findSafeY(void)")]
// was: __ZN3RBX13AdvRunDragger9findSafeYEv
// IDA 0x2d9430: 400 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d9430() {
}

// 0x2d9ae0 — __ZN3RBX13AdvRunDragger4snapERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::AdvRunDragger::snap(RBX::RbxRay const&)")]
// was: __ZN3RBX13AdvRunDragger4snapERKNS_6RbxRayE
// IDA 0x2d9ae0: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d9ae0() {
}

// 0x2d9d50 — __GLOBAL__I_a_82
#[doc(alias = "global constructor keyed to_a_82")]
// was: __GLOBAL__I_a_82
// IDA 0x2d9d50: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2d9d50() {
}

// 0x2da160 — __ZN3RBX12AxisToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::AxisToolBase::AxisToolBase(RBX::Workspace *)")]
// was: __ZN3RBX12AxisToolBaseC2EPNS_9WorkspaceE
// IDA 0x2da160: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2da160() {
}

// 0x2da2d0 — __ZN3RBX12AxisToolBase12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, const UIEvent *)
#[doc(alias = "RBX::AxisToolBase::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX12AxisToolBase12onMouseHoverERKNS_7UIEventE
// IDA 0x2da2d0: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2da2d0() {
}

// 0x2da2d8 — __ZN3RBX12AxisToolBase11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AxisToolBase::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX12AxisToolBase11onMouseIdleERKNS_7UIEventE
// IDA 0x2da2d8: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2da2d8() {
}

// 0x2da450 — __ZN3RBX12AxisToolBase11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AxisToolBase::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX12AxisToolBase11onMouseDownERKNS_7UIEventE
// IDA 0x2da450: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2da450() {
}

// 0x2da788 — __ZN3RBX12AxisToolBase11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AxisToolBase::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX12AxisToolBase11onMouseMoveERKNS_7UIEventE
// IDA 0x2da788: 380 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2da788() {
}

// 0x2dac1c — __ZN3RBX12AxisToolBase9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AxisToolBase::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX12AxisToolBase9onMouseUpERKNS_7UIEventE
// IDA 0x2dac1c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dac1c() {
}

// 0x2dad94 — __ZNK3RBX12AxisToolBase10getExtentsERNS_7ExtentsE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Extents *)
#[doc(alias = "RBX::AxisToolBase::getExtents(RBX::Extents &)const")]
// was: __ZNK3RBX12AxisToolBase10getExtentsERNS_7ExtentsE
// IDA 0x2dad94: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dad94() {
}

// 0x2db058 — __ZN3RBX11shared_fromINS_12AxisToolBaseEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::AxisToolBase> RBX::shared_from<RBX::AxisToolBase>(RBX::AxisToolBase*)")]
// was: __ZN3RBX11shared_fromINS_12AxisToolBaseEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2db058: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2db058() {
}

// 0x2db1c0 — __ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_
#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::reset(RBX::MegaDragger*)")]
// was: __ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_
// IDA 0x2db1c0: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2db1c0() {
}

// 0x2db274 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_RS9_IPNS4_9PrimitiveESaISG_EEENSD_5list2INS2_3argILi1EEENS2_17reference_wrapperISI_EEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::allocator<RBX::Primitive>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::allocator<RBX::Primitive>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector&<RBX::Primitive *,std::allocator<RBX::Primitive>>),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<std::allocator<RBX::Primitive>>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_RS9_IPNS4_9PrimitiveESaISG_EEENSD_5list2INS2_3argILi1EEENS2_17reference_wrapperISI_EEEEEEET0_T_SU_ST_
// IDA 0x2db274: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2db274() {
}

// 0x2db2bc — __ZN3RBX13DragUtilities14getPrimitives2EN5boost10shared_ptrINS_8InstanceEEERSt6vectorIPNS_9PrimitiveESaIS7_EE
#[doc(alias = "RBX::DragUtilities::getPrimitives2(rbx_core::SharedPtr<RBX::Instance>,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
// was: __ZN3RBX13DragUtilities14getPrimitives2EN5boost10shared_ptrINS_8InstanceEEERSt6vectorIPNS_9PrimitiveESaIS7_EE
// IDA 0x2db2bc: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2db2bc() {
}

// 0x2db2c4 — __GLOBAL__I_a_83
#[doc(alias = "global constructor keyed to_a_83")]
// was: __GLOBAL__I_a_83
// IDA 0x2db2c4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2db2c4() {
}

// 0x2db534 — __GLOBAL__I_a_84
#[doc(alias = "global constructor keyed to_a_84")]
// was: __GLOBAL__I_a_84
// IDA 0x2db534: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2db534() {
}

// 0x2db7a4 — __ZN3RBX9CloneToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CloneTool::CloneTool(RBX::Workspace *)")]
// was: __ZN3RBX9CloneToolC1EPNS_9WorkspaceE
// IDA 0x2db7a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2db7a4() {
}

// 0x2db7a8 — __ZN3RBX9CloneToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::CloneTool::CloneTool(RBX::Workspace *)")]
// was: __ZN3RBX9CloneToolC2EPNS_9WorkspaceE
// IDA 0x2db7a8: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2db7a8() {
}

// 0x2db8b0 — __ZN3RBX9CloneToolD0Ev
// type: void __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "RBX::CloneTool::~CloneTool()")]
// was: __ZN3RBX9CloneToolD0Ev
// IDA 0x2db8b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2db8b0() {
}

// 0x2db950 — __ZN3RBX9CloneToolD1Ev
// type: void __fastcall(RBX::CloneTool *__hidden this)
#[doc(alias = "RBX::CloneTool::~CloneTool()")]
// was: __ZN3RBX9CloneToolD1Ev
// IDA 0x2db950: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2db950() {
}