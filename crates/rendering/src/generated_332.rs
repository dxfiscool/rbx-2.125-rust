//! rendering shard 332 — 100 stubs 0x5ae2ac..0x5b1f34 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36160->36260 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36160 before -> 36260 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5ae1a4 (lowest remaining 0x5ae2ac..0x5b1f34, next lowest 0x5b1f58 if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x5ae2ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5ae2ac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ae2ac() {
}

// 0x5ae2b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5ae2b0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ae2b0() {
}

// 0x5ae2b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5ae2b4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae2b4() {
}

// 0x5ae2d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5ae2d4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae2d4() {
}

// 0x5ae2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5ae2ec: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae2ec() {
}

// 0x5ae2f0 — __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v
// IDA 0x5ae2f0: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae2f0() {
}

// 0x5ae334 — __ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv
// IDA 0x5ae334: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ae334() {
}

// 0x5ae338 — __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v
// IDA 0x5ae338: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae338() {
}

// 0x5ae41c — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5ae41c: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae41c() {
}

// 0x5ae644 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5ae644: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ae644() {
}

// 0x5ae6e0 — __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5ae6e0: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae6e0() {
}

// 0x5ae74c — __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv
// IDA 0x5ae74c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae74c() {
}

// 0x5ae890 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv
// IDA 0x5ae890: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae890() {
}

// 0x5ae940 — __ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5ae940: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ae940() {
}

// 0x5aea08 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rotate,RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const*,RBX::Rotate *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5aea08: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aea08() {
}

// 0x5aeaf0 — __ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5aeaf0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aeaf0() {
}

// 0x5aebf8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5aebf8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5aebf8() {
}

// 0x5aebfc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5aebfc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5aebfc() {
}

// 0x5aec00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5aec00: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aec00() {
}

// 0x5aec20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5aec20: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aec20() {
}

// 0x5aec38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5aec38: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aec38() {
}

// 0x5aec3c — __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v
// IDA 0x5aec3c: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aec3c() {
}

// 0x5aec80 — __ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv
// IDA 0x5aec80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5aec80() {
}

// 0x5aec84 — __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v
// IDA 0x5aec84: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aec84() {
}

// 0x5aed68 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5aed68: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aed68() {
}

// 0x5aef90 — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5aef90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5aef90() {
}

// 0x5af02c — __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5af02c: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af02c() {
}

// 0x5af098 — __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv
// IDA 0x5af098: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af098() {
}

// 0x5af1dc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv
// IDA 0x5af1dc: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af1dc() {
}

// 0x5af28c — __ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5af28c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af28c() {
}

// 0x5af354 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Glue,RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const*,RBX::Glue *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5af354: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af354() {
}

// 0x5af43c — __ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5af43c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af43c() {
}

// 0x5af544 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5af544: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5af544() {
}

// 0x5af548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5af548: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5af548() {
}

// 0x5af54c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5af54c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af54c() {
}

// 0x5af56c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5af56c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af56c() {
}

// 0x5af584 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5af584: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af584() {
}

// 0x5af588 — __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v
// IDA 0x5af588: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af588() {
}

// 0x5af5cc — __ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv
// IDA 0x5af5cc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5af5cc() {
}

// 0x5af5d0 — __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v
// IDA 0x5af5d0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af5d0() {
}

// 0x5af6b4 — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5af6b4: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af6b4() {
}

// 0x5af8dc — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5af8dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5af8dc() {
}

// 0x5af978 — __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5af978: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af978() {
}

// 0x5af9e4 — __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv
// IDA 0x5af9e4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5af9e4() {
}

// 0x5afb28 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv
// IDA 0x5afb28: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afb28() {
}

// 0x5afbd8 — __ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5afbd8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afbd8() {
}

// 0x5afca0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Snap,RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const*,RBX::Snap *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5afca0: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afca0() {
}

// 0x5afd88 — __ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5afd88: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afd88() {
}

// 0x5afe90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5afe90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5afe90() {
}

// 0x5afe94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5afe94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5afe94() {
}

// 0x5afe98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5afe98: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afe98() {
}

// 0x5afeb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5afeb8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afeb8() {
}

// 0x5afed0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5afed0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afed0() {
}

// 0x5afed4 — __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v
// IDA 0x5afed4: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5afed4() {
}

// 0x5aff18 — __ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv
// IDA 0x5aff18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5aff18() {
}

// 0x5aff1c — __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v
// IDA 0x5aff1c: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5aff1c() {
}

// 0x5b0000 — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5b0000: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0000() {
}

// 0x5b0228 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::insert(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
// IDA 0x5b0228: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0228() {
}

// 0x5b0434 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// IDA 0x5b0434: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0434() {
}

// 0x5b0458 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// IDA 0x5b0458: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0458() {
}

// 0x5b047c — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE22safe_static_init_mutexEv
// IDA 0x5b047c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b047c() {
}

// 0x5b0480 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
// IDA 0x5b0480: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0480() {
}

// 0x5b0578 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
// IDA 0x5b0578: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0578() {
}

// 0x5b05a4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
// IDA 0x5b05a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b05a4() {
}

// 0x5b0678 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot10disconnectEv
// IDA 0x5b0678: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0678() {
}

// 0x5b0788 — __ZNK3rbx7signals6signalIFvPN3RBX5JointEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvPN3RBX5JointEEE4slot9connectedEv
// IDA 0x5b0788: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0788() {
}

// 0x5b0794 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// IDA 0x5b0794: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0794() {
}

// 0x5b07a8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// IDA 0x5b07a8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b07a8() {
}

// 0x5b07bc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>::operator()<RBX::Joint *>(RBX::Joint * &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// IDA 0x5b07bc: 9 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b07bc() {
}

// 0x5b07d4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::remove(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
// IDA 0x5b07d4: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b07d4() {
}

// 0x5b08c4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot22safe_static_init_mutexEv
// IDA 0x5b08c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b08c4() {
}

// 0x5b08c8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
// IDA 0x5b08c8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b08c8() {
}

// 0x5b09b8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev
// IDA 0x5b09b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b09b8() {
}

// 0x5b09e4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev
// IDA 0x5b09e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b09e4() {
}

// 0x5b0ab8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// IDA 0x5b0ab8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0ab8() {
}

// 0x5b0ae4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// IDA 0x5b0ae4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0ae4() {
}

// 0x5b0bb8 — __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5b0bb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5b0bb8() {
}

// 0x5b0bbc — __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5b0bbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0bbc() {
}

// 0x5b0c5c — __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5b0c5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0c5c() {
}

// 0x5b0c64 — __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5b0c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0c64() {
}

// 0x5b0d08 — __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5b0d08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0d08() {
}

// 0x5b0d10 — __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5b0d10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0d10() {
}

// 0x5b0db4 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::BoundFuncDesc(void (RBX::JointsService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5b0db4: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0db4() {
}

// 0x5b0eb8 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED0Ev
// IDA 0x5b0eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b0eb8() {
}

// 0x5b0f6c — __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5b0f6c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0f6c() {
}

// 0x5b0f8c — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x5b0f8c: 154 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b0f8c() {
}

// 0x5b1124 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x5b1124: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1124() {
}

// 0x5b1154 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// IDA 0x5b1154: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b1154() {
}

// 0x5b1270 — __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5b1270: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1270() {
}

// 0x5b1354 — __ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
// IDA 0x5b1354: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1354() {
}

// 0x5b143c — __ZN3RBX13JointsServiceD2Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
// was: __ZN3RBX13JointsServiceD2Ev
// IDA 0x5b143c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b143c() {
}

// 0x5b16c0 — __GLOBAL__I_a_219
#[doc(alias = "global constructor keyed to_a_219")]
// was: __GLOBAL__I_a_219
// IDA 0x5b16c0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5b16c0() {
}

// 0x5b1b9c — __ZN3RBX8Keyframe8getPosesEv
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::getPoses(void)")]
// was: __ZN3RBX8Keyframe8getPosesEv
// IDA 0x5b1b9c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1b9c() {
}

// 0x5b1bb0 — __ZN3RBX8Keyframe7addPoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Keyframe::addPose(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8Keyframe7addPoseEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b1bb0: 5 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1bb0() {
}

// 0x5b1bbc — __ZN3RBX8Keyframe10removePoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Keyframe::removePose(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8Keyframe10removePoseEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x5b1bbc: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1bbc() {
}

// 0x5b1bd0 — __ZN3RBX8Keyframe7setTimeEf
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, float)
#[doc(alias = "RBX::Keyframe::setTime(float)")]
// was: __ZN3RBX8Keyframe7setTimeEf
// IDA 0x5b1bd0: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1bd0() {
}

// 0x5b1c0c — __ZN3RBX8KeyframeC2Ev
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::Keyframe(void)")]
// was: __ZN3RBX8KeyframeC2Ev
// IDA 0x5b1c0c: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1c0c() {
}

// 0x5b1e08 — __ZN3RBX8Keyframe10invalidateEv
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::invalidate(void)")]
// was: __ZN3RBX8Keyframe10invalidateEv
// IDA 0x5b1e08: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1e08() {
}

// 0x5b1e44 — __ZNK3RBX8Keyframe17verifySetAncestorEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Keyframe::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
// was: __ZNK3RBX8Keyframe17verifySetAncestorEPKNS_8InstanceES3_
// IDA 0x5b1e44: 75 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5b1e44() {
}

// 0x5b1f34 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
// IDA 0x5b1f34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5b1f34() {
}