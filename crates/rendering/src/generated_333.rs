//! rendering shard 333 — 100 stubs 0x5b1f58..0x5b61f4 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36260->36360 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36260 before -> 36360 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5b1f34 (lowest remaining 0x5b1f58..0x5b61f4, next lowest 0x5b61fc if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x5b1f58 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// IDA 0x5b1f58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b1f58() {
}

// 0x5b2064 — __ZNK3RBX8Keyframe7getTimeEv
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::getTime(void)const")]
// was: __ZNK3RBX8Keyframe7getTimeEv
// IDA 0x5b2064: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2064() {
}

// 0x5b2068 — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED1Ev
// IDA 0x5b2068: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2068() {
}

// 0x5b208c — __ZN3RBX8KeyframeD1Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::~Keyframe()")]
// was: __ZN3RBX8KeyframeD1Ev
// IDA 0x5b208c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b208c() {
}

// 0x5b2090 — __ZN3RBX8KeyframeD0Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::~Keyframe()")]
// was: __ZN3RBX8KeyframeD0Ev
// IDA 0x5b2090: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2090() {
}

// 0x5b2130 — __ZNK3RBX8Keyframe11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Keyframe::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX8Keyframe11askAddChildEPKNS_8InstanceE
// IDA 0x5b2130: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2130() {
}

// 0x5b216c — __ZN3RBX8Keyframe12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Keyframe::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX8Keyframe12onChildAddedEPNS_8InstanceE
// IDA 0x5b216c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b216c() {
}

// 0x5b2170 — __ZN3RBX8Keyframe14onChildRemovedEPNS_8InstanceE
// type: int __fastcall(RBX::Keyframe *this, RBX::Instance *)
#[doc(alias = "RBX::Keyframe::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX8Keyframe14onChildRemovedEPNS_8InstanceE
// IDA 0x5b2170: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b2170() {
}

// 0x5b2174 — __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
// IDA 0x5b2174: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2174() {
}

// 0x5b2184 — __ZThn32_N3RBX8KeyframeD1Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn32_N3RBX8KeyframeD1Ev
// IDA 0x5b2184: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2184() {
}

// 0x5b218c — __ZThn32_N3RBX8KeyframeD0Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn32_N3RBX8KeyframeD0Ev
// IDA 0x5b218c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b218c() {
}

// 0x5b2230 — __ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
// IDA 0x5b2230: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2230() {
}

// 0x5b2240 — __ZThn36_N3RBX8KeyframeD1Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn36_N3RBX8KeyframeD1Ev
// IDA 0x5b2240: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2240() {
}

// 0x5b2248 — __ZThn36_N3RBX8KeyframeD0Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn36_N3RBX8KeyframeD0Ev
// IDA 0x5b2248: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2248() {
}

// 0x5b22ec — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev
// IDA 0x5b22ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b22ec() {
}

// 0x5b22f0 — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev
// IDA 0x5b22f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b22f0() {
}

// 0x5b238c — __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv
// IDA 0x5b238c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b238c() {
}

// 0x5b2414 — __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv
// IDA 0x5b2414: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2414() {
}

// 0x5b2558 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv
// IDA 0x5b2558: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2558() {
}

// 0x5b2608 — __ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5b2608: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2608() {
}

// 0x5b26d0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Keyframe,RBX::Keyframe>(rbx_core::SharedPtr<RBX::Keyframe> const*,RBX::Keyframe *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5b26d0: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b26d0() {
}

// 0x5b27b8 — __ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5b27b8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b27b8() {
}

// 0x5b28c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5b28c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5b28c0() {
}

// 0x5b28c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5b28c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b28c4() {
}

// 0x5b28c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5b28c8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b28c8() {
}

// 0x5b28e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5b28e8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b28e8() {
}

// 0x5b2900 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5b2900: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2900() {
}

// 0x5b2904 — __ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv
// IDA 0x5b2904: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b2904() {
}

// 0x5b2908 — __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v
// IDA 0x5b2908: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2908() {
}

// 0x5b29e8 — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev
// IDA 0x5b29e8: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b29e8() {
}

// 0x5b2c2c — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv
// IDA 0x5b2c2c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2c2c() {
}

// 0x5b2ca0 — __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5b2ca0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b2ca0() {
}

// 0x5b2ca4 — __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5b2ca4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2ca4() {
}

// 0x5b2d44 — __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5b2d44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2d44() {
}

// 0x5b2d4c — __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5b2d4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2d4c() {
}

// 0x5b2df0 — __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5b2df0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2df0() {
}

// 0x5b2df8 — __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5b2df8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2df8() {
}

// 0x5b2e9c — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::PropDescriptor<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>(char const*,char const*,float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5b2e9c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2e9c() {
}

// 0x5b2fb0 — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED0Ev
// IDA 0x5b2fb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b2fb0() {
}

// 0x5b2fdc — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// IDA 0x5b2fdc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2fdc() {
}

// 0x5b2fe0 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// IDA 0x5b2fe0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2fe0() {
}

// 0x5b2fe4 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5b2fe4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b2fe4() {
}

// 0x5b3004 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x5b3004: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3004() {
}

// 0x5b3028 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5b3028: 154 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3028() {
}

// 0x5b31c0 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x5b31c0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b31c0() {
}

// 0x5b31f0 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// IDA 0x5b31f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b31f0() {
}

// 0x5b330c — __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5b330c: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b330c() {
}

// 0x5b33f0 — __ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
// IDA 0x5b33f0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b33f0() {
}

// 0x5b34d8 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5b34d8: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b34d8() {
}

// 0x5b35dc — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
// IDA 0x5b35dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b35dc() {
}

// 0x5b3690 — __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5b3690: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3690() {
}

// 0x5b36b4 — __ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Keyframe*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
// IDA 0x5b36b4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b36b4() {
}

// 0x5b379c — __GLOBAL__I_a_220
#[doc(alias = "global constructor keyed to_a_220")]
// was: __GLOBAL__I_a_220
// IDA 0x5b379c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5b379c() {
}

// 0x5b3b08 — __ZN3RBX16KeyframeSequence12getKeyframesEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getKeyframes(void)")]
// was: __ZN3RBX16KeyframeSequence12getKeyframesEv
// IDA 0x5b3b08: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3b08() {
}

// 0x5b3b1c — __ZN3RBX16KeyframeSequence11addKeyframeEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::addKeyframe(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX16KeyframeSequence11addKeyframeEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b3b1c: 5 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3b1c() {
}

// 0x5b3b28 — __ZN3RBX16KeyframeSequence14removeKeyframeEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::removeKeyframe(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX16KeyframeSequence14removeKeyframeEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b3b28: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3b28() {
}

// 0x5b3b3c — __ZN3RBX16KeyframeSequence7setLoopEb
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, bool)
#[doc(alias = "RBX::KeyframeSequence::setLoop(bool)")]
// was: __ZN3RBX16KeyframeSequence7setLoopEb
// IDA 0x5b3b3c: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3b3c() {
}

// 0x5b3b5c — __ZN3RBX16KeyframeSequence11setPriorityENS0_8PriorityE
#[doc(alias = "RBX::KeyframeSequence::setPriority(RBX::KeyframeSequence::Priority)")]
// was: __ZN3RBX16KeyframeSequence11setPriorityENS0_8PriorityE
// IDA 0x5b3b5c: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3b5c() {
}

// 0x5b3b7c — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC1Ev
// IDA 0x5b3b7c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b3b7c() {
}

// 0x5b3b80 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC2Ev
// IDA 0x5b3b80: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3b80() {
}

// 0x5b3d70 — __ZN3RBX16KeyframeSequenceC1Ev
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::KeyframeSequence(void)")]
// was: __ZN3RBX16KeyframeSequenceC1Ev
// IDA 0x5b3d70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b3d70() {
}

// 0x5b3d74 — __ZN3RBX16KeyframeSequenceC2Ev
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::KeyframeSequence(void)")]
// was: __ZN3RBX16KeyframeSequenceC2Ev
// IDA 0x5b3d74: 237 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b3d74() {
}

// 0x5b401c — __ZN3RBX16KeyframeSequence20copyKeyframeSequenceEPS0_
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::KeyframeSequence *)
#[doc(alias = "RBX::KeyframeSequence::copyKeyframeSequence(RBX::KeyframeSequence*)")]
// was: __ZN3RBX16KeyframeSequence20copyKeyframeSequenceEPS0_
// IDA 0x5b401c: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b401c() {
}

// 0x5b4174 — __ZN3RBXL9CopyChildEN5boost10shared_ptrINS_8InstanceEEEPS2_
#[doc(alias = "RBX::CopyChild(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*)")]
// was: __ZN3RBXL9CopyChildEN5boost10shared_ptrINS_8InstanceEEEPS2_
// IDA 0x5b4174: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4174() {
}

// 0x5b417c — __ZNK3RBX16KeyframeSequence9cacheDataEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::cacheData(void)const")]
// was: __ZNK3RBX16KeyframeSequence9cacheDataEv
// IDA 0x5b417c: 185 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b417c() {
}

// 0x5b4364 — __ZNK3RBX16KeyframeSequence11getDurationEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getDuration(void)const")]
// was: __ZNK3RBX16KeyframeSequence11getDurationEv
// IDA 0x5b4364: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4364() {
}

// 0x5b437c — __ZNK3RBX16KeyframeSequence5applyERSt6vectorINS_15PoseAccumulatorESaIS2_EEddf
// type: int __fastcall(int, int, int, int, double, float, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::KeyframeSequence::apply(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double,double,float)const")]
// was: __ZNK3RBX16KeyframeSequence5applyERSt6vectorINS_15PoseAccumulatorESaIS2_EEddf
// IDA 0x5b437c: 344 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b437c() {
}

// 0x5b4774 — __ZN3RBX10CachedPose16interpolatePosesERKS0_S2_ff
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this, const RBX::CachedPose *, const RBX::CachedPose *, float, float)
#[doc(alias = "RBX::CachedPose::interpolatePoses(RBX::CachedPose const&,RBX::CachedPose const&,float,float)")]
// was: __ZN3RBX10CachedPose16interpolatePosesERKS0_S2_ff
// IDA 0x5b4774: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4774() {
}

// 0x5b48b0 — __ZN3RBX10CachedPose10blendPosesERKS0_S2_
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this, const RBX::CachedPose *, const RBX::CachedPose *)
#[doc(alias = "RBX::CachedPose::blendPoses(RBX::CachedPose const&,RBX::CachedPose const&)")]
// was: __ZN3RBX10CachedPose10blendPosesERKS0_S2_
// IDA 0x5b48b0: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b48b0() {
}

// 0x5b496c — __ZN3RBX16KeyframeSequence12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX16KeyframeSequence12onChildAddedEPNS_8InstanceE
// IDA 0x5b496c: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b496c() {
}

// 0x5b4974 — __ZN3RBX16KeyframeSequence15invalidateCacheEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::invalidateCache(void)")]
// was: __ZN3RBX16KeyframeSequence15invalidateCacheEv
// IDA 0x5b4974: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4974() {
}

// 0x5b497c — __ZN3RBX16KeyframeSequence14onChildRemovedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX16KeyframeSequence14onChildRemovedEPNS_8InstanceE
// IDA 0x5b497c: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b497c() {
}

// 0x5b4984 — __ZNK3RBX16KeyframeSequence15AppendPosePass0ERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::AppendPosePass0(rbx_core::SharedPtr<RBX::Instance> const&)const")]
// was: __ZNK3RBX16KeyframeSequence15AppendPosePass0ERKN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b4984: 213 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4984() {
}

// 0x5b4bf8 — __ZNK3RBX16KeyframeSequence15AppendPosePass1ERKN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIPNS_10CachedPoseESaIS9_EE
#[doc(alias = "RBX::KeyframeSequence::AppendPosePass1(rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *)const")]
// was: __ZNK3RBX16KeyframeSequence15AppendPosePass1ERKN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIPNS_10CachedPoseESaIS9_EE
// IDA 0x5b4bf8: 252 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4bf8() {
}

// 0x5b4f24 — __ZNK3RBX16KeyframeSequence12makeKeyframeEPNS_8KeyframeE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Keyframe *)
#[doc(alias = "RBX::KeyframeSequence::makeKeyframe(RBX::Keyframe *)const")]
// was: __ZNK3RBX16KeyframeSequence12makeKeyframeEPNS_8KeyframeE
// IDA 0x5b4f24: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b4f24() {
}

// 0x5b50a4 — __ZNK3RBX16KeyframeSequence18cacheKeyframePass0ERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::KeyframeSequence::cacheKeyframePass0(rbx_core::SharedPtr<RBX::Instance> const&)const")]
// was: __ZNK3RBX16KeyframeSequence18cacheKeyframePass0ERKN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b50a4: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b50a4() {
}

// 0x5b520c — __ZNK3RBX16KeyframeSequence18cacheKeyframePass1ERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::cacheKeyframePass1(rbx_core::SharedPtr<RBX::Instance> const&)const")]
// was: __ZNK3RBX16KeyframeSequence18cacheKeyframePass1ERKN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b520c: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b520c() {
}

// 0x5b52fc — __ZNK3RBX10CachedPose9getCFrameEv
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this)
#[doc(alias = "RBX::CachedPose::getCFrame(void)const")]
// was: __ZNK3RBX10CachedPose9getCFrameEv
// IDA 0x5b52fc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b52fc() {
}

// 0x5b560c — __ZNK3RBX16KeyframeSequence17verifySetAncestorEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
// was: __ZNK3RBX16KeyframeSequence17verifySetAncestorEPKNS_8InstanceES3_
// IDA 0x5b560c: 75 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b560c() {
}

// 0x5b56fc — __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
// IDA 0x5b56fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b56fc() {
}

// 0x5b5720 — __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// IDA 0x5b5720: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b5720() {
}

// 0x5b582c — __ZNK3RBX16KeyframeSequence7getLoopEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getLoop(void)const")]
// was: __ZNK3RBX16KeyframeSequence7getLoopEv
// IDA 0x5b582c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b582c() {
}

// 0x5b5834 — __ZN3RBX10Reflection14PropDescriptorINS_16KeyframeSequenceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::KeyframeSequence,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_16KeyframeSequenceEbED1Ev
// IDA 0x5b5834: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b5834() {
}

// 0x5b5858 — __ZNK3RBX16KeyframeSequence11getPriorityEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getPriority(void)const")]
// was: __ZNK3RBX16KeyframeSequence11getPriorityEv
// IDA 0x5b5858: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5858() {
}

// 0x5b5860 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED1Ev
// IDA 0x5b5860: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b5860() {
}

// 0x5b5884 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::addPair(RBX::KeyframeSequence::Priority,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE7addPairES3_PKc
// IDA 0x5b5884: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5884() {
}

// 0x5b5be4 — __ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_
// type: unsigned int __fastcall(const std::string **, std::string *)
#[doc(alias = "unsigned long RBX::findOrAdd<std::string>(std::vector<std::string,std::allocator<std::string>> &,std::string const&)")]
// was: __ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_
// IDA 0x5b5be4: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5be4() {
}

// 0x5b5c28 — __ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
// type: int(void)
#[doc(alias = "unsigned long RBX::findOrAdd<std::pair<unsigned long,unsigned long>>(std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>> &,std::pair<unsigned long,unsigned long> const&)")]
// was: __ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
// IDA 0x5b5c28: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5c28() {
}

// 0x5b5c7c — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")]
// was: __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
// IDA 0x5b5c7c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5b5c7c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5b5cb8 — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::resize(unsigned long,RBX::CachedPose *)")]
// was: __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
// IDA 0x5b5cb8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5cb8() {
}

// 0x5b5cec — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
// IDA 0x5b5cec: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5cec() {
}

// 0x5b5df4 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
// IDA 0x5b5df4: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5df4() {
}

// 0x5b5ef0 — __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX::KeyframeSequence::CachedKeyframe const&)")]
// was: __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
// IDA 0x5b5ef0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5b5ef0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5b5f40 — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::reserve(unsigned long)")]
// was: __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
// IDA 0x5b5f40: 49 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b5f40() {
}

// 0x5b5fcc — __ZN3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
// was: __ZN3RBX16KeyframeSequenceD1Ev
// IDA 0x5b5fcc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b5fcc() {
}

// 0x5b6104 — __ZN3RBX16KeyframeSequenceD0Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
// was: __ZN3RBX16KeyframeSequenceD0Ev
// IDA 0x5b6104: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b6104() {
}

// 0x5b61a4 — __ZNK3RBX16KeyframeSequence11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX16KeyframeSequence11askAddChildEPKNS_8InstanceE
// IDA 0x5b61a4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b61a4() {
}

// 0x5b61e0 — __ZNK3RBX16KeyframeSequence12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX16KeyframeSequence12askSetParentEPKNS_8InstanceE
// IDA 0x5b61e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b61e0() {
}

// 0x5b61e4 — __ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv
// IDA 0x5b61e4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b61e4() {
}

// 0x5b61f4 — __ZThn32_N3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequence::~KeyframeSequence()")]
// was: __ZThn32_N3RBX16KeyframeSequenceD1Ev
// IDA 0x5b61f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b61f4() {
}