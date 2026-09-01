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
pub fn stub_5ae2ac() -> ! {
    todo!("0x5ae2ac boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5ae2b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5ae2b0() -> ! {
    todo!("0x5ae2b0 boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5ae2b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5ae2b4() -> ! {
    todo!("0x5ae2b4 boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5ae2d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5ae2d4() -> ! {
    todo!("0x5ae2d4 boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5ae2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7RotatePENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5ae2ec() -> ! {
    todo!("0x5ae2ec boost::detail::sp_counted_impl_pd<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5ae2f0 — __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v
pub fn stub_5ae2f0() -> ! {
    todo!("0x5ae2f0 __ZN3RBX4Name7declareILZNS_8sRotatePEEEERKS0_v")
}

// 0x5ae334 — __ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv
pub fn stub_5ae334() -> ! {
    todo!("0x5ae334 __ZN3RBX4Name13callDoDeclareILZNS_8sRotatePEEEEvv")
}

// 0x5ae338 — __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v
pub fn stub_5ae338() -> ! {
    todo!("0x5ae338 __ZN3RBX4Name9doDeclareILZNS_8sRotatePEEEERKS0_v")
}

// 0x5ae41c — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev
pub fn stub_5ae41c() -> ! {
    todo!("0x5ae41c __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5ae644 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev
pub fn stub_5ae644() -> ! {
    todo!("0x5ae644 __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorD2Ev")
}

// 0x5ae6e0 — __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_5ae6e0() -> ! {
    todo!("0x5ae6e0 __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x5ae74c — __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv
pub fn stub_5ae74c() -> ! {
    todo!("0x5ae74c __ZNK3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7Creator6createEv")
}

// 0x5ae890 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv
pub fn stub_5ae890() -> ! {
    todo!("0x5ae890 boost::shared_ptr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)")
}

// 0x5ae940 — __ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5ae940() -> ! {
    todo!("0x5ae940 boost::shared_ptr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5aea08 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rotate,RBX::Rotate>(rbx_core::SharedPtr<RBX::Rotate> const*,RBX::Rotate *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RotateES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5aea08() -> ! {
    todo!("0x5aea08 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rotate,RBX::Rotate>(boost::shared_ptr<RBX::Rotate> const*,RBX::Rotate *)const")
}

// 0x5aeaf0 — __ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5aeaf0() -> ! {
    todo!("0x5aeaf0 boost::detail::shared_count::shared_count<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5aebf8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5aebf8() -> ! {
    todo!("0x5aebf8 boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5aebfc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5aebfc() -> ! {
    todo!("0x5aebfc boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5aec00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5aec00() -> ! {
    todo!("0x5aec00 boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5aec20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5aec20() -> ! {
    todo!("0x5aec20 boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5aec38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RotateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5aec38() -> ! {
    todo!("0x5aec38 boost::detail::sp_counted_impl_pd<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5aec3c — __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v
pub fn stub_5aec3c() -> ! {
    todo!("0x5aec3c __ZN3RBX4Name7declareILZNS_7sRotateEEEERKS0_v")
}

// 0x5aec80 — __ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv
pub fn stub_5aec80() -> ! {
    todo!("0x5aec80 __ZN3RBX4Name13callDoDeclareILZNS_7sRotateEEEEvv")
}

// 0x5aec84 — __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v
pub fn stub_5aec84() -> ! {
    todo!("0x5aec84 __ZN3RBX4Name9doDeclareILZNS_7sRotateEEEERKS0_v")
}

// 0x5aed68 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev
pub fn stub_5aed68() -> ! {
    todo!("0x5aed68 __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5aef90 — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev
pub fn stub_5aef90() -> ! {
    todo!("0x5aef90 __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorD2Ev")
}

// 0x5af02c — __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_5af02c() -> ! {
    todo!("0x5af02c __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x5af098 — __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv
pub fn stub_5af098() -> ! {
    todo!("0x5af098 __ZNK3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7Creator6createEv")
}

// 0x5af1dc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv
pub fn stub_5af1dc() -> ! {
    todo!("0x5af1dc boost::shared_ptr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)")
}

// 0x5af28c — __ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5af28c() -> ! {
    todo!("0x5af28c boost::shared_ptr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5af354 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Glue,RBX::Glue>(rbx_core::SharedPtr<RBX::Glue> const*,RBX::Glue *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4GlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5af354() -> ! {
    todo!("0x5af354 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Glue,RBX::Glue>(boost::shared_ptr<RBX::Glue> const*,RBX::Glue *)const")
}

// 0x5af43c — __ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5af43c() -> ! {
    todo!("0x5af43c boost::detail::shared_count::shared_count<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5af544 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5af544() -> ! {
    todo!("0x5af544 boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5af548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5af548() -> ! {
    todo!("0x5af548 boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5af54c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5af54c() -> ! {
    todo!("0x5af54c boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5af56c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5af56c() -> ! {
    todo!("0x5af56c boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5af584 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4GlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5af584() -> ! {
    todo!("0x5af584 boost::detail::sp_counted_impl_pd<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5af588 — __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v
pub fn stub_5af588() -> ! {
    todo!("0x5af588 __ZN3RBX4Name7declareILZNS_5sGlueEEEERKS0_v")
}

// 0x5af5cc — __ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv
pub fn stub_5af5cc() -> ! {
    todo!("0x5af5cc __ZN3RBX4Name13callDoDeclareILZNS_5sGlueEEEEvv")
}

// 0x5af5d0 — __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v
pub fn stub_5af5d0() -> ! {
    todo!("0x5af5d0 __ZN3RBX4Name9doDeclareILZNS_5sGlueEEEERKS0_v")
}

// 0x5af6b4 — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev
pub fn stub_5af6b4() -> ! {
    todo!("0x5af6b4 __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5af8dc — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev
pub fn stub_5af8dc() -> ! {
    todo!("0x5af8dc __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorD2Ev")
}

// 0x5af978 — __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_5af978() -> ! {
    todo!("0x5af978 __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x5af9e4 — __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv
pub fn stub_5af9e4() -> ! {
    todo!("0x5af9e4 __ZNK3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7Creator6createEv")
}

// 0x5afb28 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv
pub fn stub_5afb28() -> ! {
    todo!("0x5afb28 boost::shared_ptr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)")
}

// 0x5afbd8 — __ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5afbd8() -> ! {
    todo!("0x5afbd8 boost::shared_ptr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5afca0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Snap,RBX::Snap>(rbx_core::SharedPtr<RBX::Snap> const*,RBX::Snap *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SnapES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5afca0() -> ! {
    todo!("0x5afca0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Snap,RBX::Snap>(boost::shared_ptr<RBX::Snap> const*,RBX::Snap *)const")
}

// 0x5afd88 — __ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5afd88() -> ! {
    todo!("0x5afd88 boost::detail::shared_count::shared_count<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5afe90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5afe90() -> ! {
    todo!("0x5afe90 boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5afe94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5afe94() -> ! {
    todo!("0x5afe94 boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5afe98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5afe98() -> ! {
    todo!("0x5afe98 boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5afeb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5afeb8() -> ! {
    todo!("0x5afeb8 boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5afed0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5afed0() -> ! {
    todo!("0x5afed0 boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5afed4 — __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v
pub fn stub_5afed4() -> ! {
    todo!("0x5afed4 __ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v")
}

// 0x5aff18 — __ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv
pub fn stub_5aff18() -> ! {
    todo!("0x5aff18 __ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv")
}

// 0x5aff1c — __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v
pub fn stub_5aff1c() -> ! {
    todo!("0x5aff1c __ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v")
}

// 0x5b0000 — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev
pub fn stub_5b0000() -> ! {
    todo!("0x5b0000 __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev")
}

// 0x5b0228 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::insert(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
pub fn stub_5b0228() -> ! {
    todo!("0x5b0228 rbx::signals::signal<void ()(RBX::Joint *)>::insert(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")
}

// 0x5b0434 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
pub fn stub_5b0434() -> ! {
    todo!("0x5b0434 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")
}

// 0x5b0458 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
pub fn stub_5b0458() -> ! {
    todo!("0x5b0458 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")
}

// 0x5b047c — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE22safe_static_init_mutexEv
pub fn stub_5b047c() -> ! {
    todo!("0x5b047c rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_init_mutex(void)")
}

// 0x5b0480 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
pub fn stub_5b0480() -> ! {
    todo!("0x5b0480 rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_do_get_mutex(void)")
}

// 0x5b0578 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
pub fn stub_5b0578() -> ! {
    todo!("0x5b0578 rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x5b05a4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
pub fn stub_5b05a4() -> ! {
    todo!("0x5b05a4 rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x5b0678 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot10disconnectEv
pub fn stub_5b0678() -> ! {
    todo!("0x5b0678 rbx::signals::signal<void ()(RBX::Joint *)>::slot::disconnect(void)")
}

// 0x5b0788 — __ZNK3rbx7signals6signalIFvPN3RBX5JointEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvPN3RBX5JointEEE4slot9connectedEv
pub fn stub_5b0788() -> ! {
    todo!("0x5b0788 rbx::signals::signal<void ()(RBX::Joint *)>::slot::connected(void)const")
}

// 0x5b0794 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_5b0794() -> ! {
    todo!("0x5b0794 rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")
}

// 0x5b07a8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
pub fn stub_5b07a8() -> ! {
    todo!("0x5b07a8 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")
}

// 0x5b07bc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>::operator()<RBX::Joint *>(RBX::Joint * &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
pub fn stub_5b07bc() -> ! {
    todo!("0x5b07bc void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>::operator()<RBX::Joint *>(RBX::Joint * &)")
}

// 0x5b07d4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::remove(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
pub fn stub_5b07d4() -> ! {
    todo!("0x5b07d4 rbx::signals::signal<void ()(RBX::Joint *)>::remove(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")
}

// 0x5b08c4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot22safe_static_init_mutexEv
pub fn stub_5b08c4() -> ! {
    todo!("0x5b08c4 rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_init_mutex(void)")
}

// 0x5b08c8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
pub fn stub_5b08c8() -> ! {
    todo!("0x5b08c8 rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_do_get_mutex(void)")
}

// 0x5b09b8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev
pub fn stub_5b09b8() -> ! {
    todo!("0x5b09b8 rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")
}

// 0x5b09e4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev
pub fn stub_5b09e4() -> ! {
    todo!("0x5b09e4 rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")
}

// 0x5b0ab8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
pub fn stub_5b0ab8() -> ! {
    todo!("0x5b0ab8 rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")
}

// 0x5b0ae4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
pub fn stub_5b0ae4() -> ! {
    todo!("0x5b0ae4 rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")
}

// 0x5b0bb8 — __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5b0bb8() -> ! {
    todo!("0x5b0bb8 __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5b0bbc — __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5b0bbc() -> ! {
    todo!("0x5b0bbc __ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5b0c5c — __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5b0c5c() -> ! {
    todo!("0x5b0c5c __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5b0c64 — __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5b0c64() -> ! {
    todo!("0x5b0c64 __ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5b0d08 — __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5b0d08() -> ! {
    todo!("0x5b0d08 __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5b0d10 — __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5b0d10() -> ! {
    todo!("0x5b0d10 __ZThn36_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5b0db4 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::BoundFuncDesc(void (RBX::JointsService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5b0db4() -> ! {
    todo!("0x5b0db4 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::BoundFuncDesc(void (RBX::JointsService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5b0eb8 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED0Ev
pub fn stub_5b0eb8() -> ! {
    todo!("0x5b0eb8 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc()")
}

// 0x5b0f6c — __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5b0f6c() -> ! {
    todo!("0x5b0f6c RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5b0f8c — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5b0f8c() -> ! {
    todo!("0x5b0f8c RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5b1124 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_5b1124() -> ! {
    todo!("0x5b1124 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x5b1154 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
pub fn stub_5b1154() -> ! {
    todo!("0x5b1154 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x5b1270 — __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5b1270() -> ! {
    todo!("0x5b1270 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5b1354 — __ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
pub fn stub_5b1354() -> ! {
    todo!("0x5b1354 RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)")
}

// 0x5b143c — __ZN3RBX13JointsServiceD2Ev
// type: void __fastcall(RBX::JointsService *__hidden this)
#[doc(alias = "RBX::JointsService::~JointsService()")]
// was: __ZN3RBX13JointsServiceD2Ev
pub fn stub_5b143c() -> ! {
    todo!("0x5b143c RBX::JointsService::~JointsService()")
}

// 0x5b16c0 — __GLOBAL__I_a_219
#[doc(alias = "global constructor keyed to_a_219")]
// was: __GLOBAL__I_a_219
pub fn stub_5b16c0() -> ! {
    todo!("0x5b16c0 global constructor keyed to_a_219")
}

// 0x5b1b9c — __ZN3RBX8Keyframe8getPosesEv
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::getPoses(void)")]
// was: __ZN3RBX8Keyframe8getPosesEv
pub fn stub_5b1b9c() -> ! {
    todo!("0x5b1b9c RBX::Keyframe::getPoses(void)")
}

// 0x5b1bb0 — __ZN3RBX8Keyframe7addPoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Keyframe::addPose(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8Keyframe7addPoseEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b1bb0() -> ! {
    todo!("0x5b1bb0 RBX::Keyframe::addPose(boost::shared_ptr<RBX::Instance>)")
}

// 0x5b1bbc — __ZN3RBX8Keyframe10removePoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Keyframe::removePose(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8Keyframe10removePoseEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b1bbc() -> ! {
    todo!("0x5b1bbc RBX::Keyframe::removePose(boost::shared_ptr<RBX::Instance>)")
}

// 0x5b1bd0 — __ZN3RBX8Keyframe7setTimeEf
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, float)
#[doc(alias = "RBX::Keyframe::setTime(float)")]
// was: __ZN3RBX8Keyframe7setTimeEf
pub fn stub_5b1bd0() -> ! {
    todo!("0x5b1bd0 RBX::Keyframe::setTime(float)")
}

// 0x5b1c0c — __ZN3RBX8KeyframeC2Ev
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::Keyframe(void)")]
// was: __ZN3RBX8KeyframeC2Ev
pub fn stub_5b1c0c() -> ! {
    todo!("0x5b1c0c RBX::Keyframe::Keyframe(void)")
}

// 0x5b1e08 — __ZN3RBX8Keyframe10invalidateEv
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::invalidate(void)")]
// was: __ZN3RBX8Keyframe10invalidateEv
pub fn stub_5b1e08() -> ! {
    todo!("0x5b1e08 RBX::Keyframe::invalidate(void)")
}

// 0x5b1e44 — __ZNK3RBX8Keyframe17verifySetAncestorEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Keyframe::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
// was: __ZNK3RBX8Keyframe17verifySetAncestorEPKNS_8InstanceES3_
pub fn stub_5b1e44() -> ! {
    todo!("0x5b1e44 RBX::Keyframe::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")
}

// 0x5b1f34 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
pub fn stub_5b1f34() -> ! {
    todo!("0x5b1f34 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}