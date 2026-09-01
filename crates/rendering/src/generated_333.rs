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
pub fn stub_5b1f58() -> ! {
    todo!("0x5b1f58 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x5b2064 — __ZNK3RBX8Keyframe7getTimeEv
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::getTime(void)const")]
// was: __ZNK3RBX8Keyframe7getTimeEv
pub fn stub_5b2064() -> ! {
    todo!("0x5b2064 RBX::Keyframe::getTime(void)const")
}

// 0x5b2068 — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED1Ev
pub fn stub_5b2068() -> ! {
    todo!("0x5b2068 RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")
}

// 0x5b208c — __ZN3RBX8KeyframeD1Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::~Keyframe()")]
// was: __ZN3RBX8KeyframeD1Ev
pub fn stub_5b208c() -> ! {
    todo!("0x5b208c RBX::Keyframe::~Keyframe()")
}

// 0x5b2090 — __ZN3RBX8KeyframeD0Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "RBX::Keyframe::~Keyframe()")]
// was: __ZN3RBX8KeyframeD0Ev
pub fn stub_5b2090() -> ! {
    todo!("0x5b2090 RBX::Keyframe::~Keyframe()")
}

// 0x5b2130 — __ZNK3RBX8Keyframe11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Keyframe::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX8Keyframe11askAddChildEPKNS_8InstanceE
pub fn stub_5b2130() -> ! {
    todo!("0x5b2130 RBX::Keyframe::askAddChild(RBX::Instance const*)const")
}

// 0x5b216c — __ZN3RBX8Keyframe12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Keyframe::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX8Keyframe12onChildAddedEPNS_8InstanceE
pub fn stub_5b216c() -> ! {
    todo!("0x5b216c RBX::Keyframe::onChildAdded(RBX::Instance *)")
}

// 0x5b2170 — __ZN3RBX8Keyframe14onChildRemovedEPNS_8InstanceE
// type: int __fastcall(RBX::Keyframe *this, RBX::Instance *)
#[doc(alias = "RBX::Keyframe::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX8Keyframe14onChildRemovedEPNS_8InstanceE
pub fn stub_5b2170() -> ! {
    todo!("0x5b2170 RBX::Keyframe::onChildRemoved(RBX::Instance *)")
}

// 0x5b2174 — __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
pub fn stub_5b2174() -> ! {
    todo!("0x5b2174 __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv")
}

// 0x5b2184 — __ZThn32_N3RBX8KeyframeD1Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn32_N3RBX8KeyframeD1Ev
pub fn stub_5b2184() -> ! {
    todo!("0x5b2184 `non-virtual thunk to'RBX::Keyframe::~Keyframe()")
}

// 0x5b218c — __ZThn32_N3RBX8KeyframeD0Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn32_N3RBX8KeyframeD0Ev
pub fn stub_5b218c() -> ! {
    todo!("0x5b218c `non-virtual thunk to'RBX::Keyframe::~Keyframe()")
}

// 0x5b2230 — __ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv
pub fn stub_5b2230() -> ! {
    todo!("0x5b2230 __ZThn32_NK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E12getClassNameEv")
}

// 0x5b2240 — __ZThn36_N3RBX8KeyframeD1Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn36_N3RBX8KeyframeD1Ev
pub fn stub_5b2240() -> ! {
    todo!("0x5b2240 `non-virtual thunk to'RBX::Keyframe::~Keyframe()")
}

// 0x5b2248 — __ZThn36_N3RBX8KeyframeD0Ev
// type: void __fastcall(RBX::Keyframe *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Keyframe::~Keyframe()")]
// was: __ZThn36_N3RBX8KeyframeD0Ev
pub fn stub_5b2248() -> ! {
    todo!("0x5b2248 `non-virtual thunk to'RBX::Keyframe::~Keyframe()")
}

// 0x5b22ec — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev
pub fn stub_5b22ec() -> ! {
    todo!("0x5b22ec __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD1Ev")
}

// 0x5b22f0 — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev
pub fn stub_5b22f0() -> ! {
    todo!("0x5b22f0 __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorD2Ev")
}

// 0x5b238c — __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv
pub fn stub_5b238c() -> ! {
    todo!("0x5b238c __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator12getClassNameEv")
}

// 0x5b2414 — __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv
pub fn stub_5b2414() -> ! {
    todo!("0x5b2414 __ZNK3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7Creator6createEv")
}

// 0x5b2558 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv
pub fn stub_5b2558() -> ! {
    todo!("0x5b2558 rbx_core::SharedPtr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)")
}

// 0x5b2608 — __ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_5b2608() -> ! {
    todo!("0x5b2608 rbx_core::SharedPtr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5b26d0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Keyframe,RBX::Keyframe>(rbx_core::SharedPtr<RBX::Keyframe> const*,RBX::Keyframe *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_5b26d0() -> ! {
    todo!("0x5b26d0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Keyframe,RBX::Keyframe>(rbx_core::SharedPtr<RBX::Keyframe> const*,RBX::Keyframe *)const")
}

// 0x5b27b8 — __ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5b27b8() -> ! {
    todo!("0x5b27b8 boost::detail::shared_count::shared_count<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x5b28c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5b28c0() -> ! {
    todo!("0x5b28c0 boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5b28c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5b28c4() -> ! {
    todo!("0x5b28c4 boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x5b28c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5b28c8() -> ! {
    todo!("0x5b28c8 boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x5b28e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5b28e8() -> ! {
    todo!("0x5b28e8 boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x5b2900 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5b2900() -> ! {
    todo!("0x5b2900 boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x5b2904 — __ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv
pub fn stub_5b2904() -> ! {
    todo!("0x5b2904 __ZN3RBX4Name13callDoDeclareILZNS_9sKeyframeEEEEvv")
}

// 0x5b2908 — __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v
pub fn stub_5b2908() -> ! {
    todo!("0x5b2908 __ZN3RBX4Name9doDeclareILZNS_9sKeyframeEEEERKS0_v")
}

// 0x5b29e8 — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev
pub fn stub_5b29e8() -> ! {
    todo!("0x5b29e8 __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E7CreatorC2Ev")
}

// 0x5b2c2c — __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv
pub fn stub_5b2c2c() -> ! {
    todo!("0x5b2c2c __ZN3RBX14FactoryProductINS_8KeyframeENS_8InstanceELZNS_9sKeyframeEES2_E17static_getCreatorEv")
}

// 0x5b2ca0 — __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5b2ca0() -> ! {
    todo!("0x5b2ca0 __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5b2ca4 — __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5b2ca4() -> ! {
    todo!("0x5b2ca4 __ZN3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5b2d44 — __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5b2d44() -> ! {
    todo!("0x5b2d44 __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5b2d4c — __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5b2d4c() -> ! {
    todo!("0x5b2d4c __ZThn32_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5b2df0 — __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_5b2df0() -> ! {
    todo!("0x5b2df0 __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x5b2df8 — __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_5b2df8() -> ! {
    todo!("0x5b2df8 __ZThn36_N3RBX10Reflection9DescribedINS_8KeyframeELZNS_9sKeyframeEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sKeyframeEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x5b2e9c — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::PropDescriptor<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>(char const*,char const*,float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_5b2e9c() -> ! {
    todo!("0x5b2e9c RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::PropDescriptor<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>(char const*,char const*,float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x5b2fb0 — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED0Ev
pub fn stub_5b2fb0() -> ! {
    todo!("0x5b2fb0 RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")
}

// 0x5b2fdc — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
pub fn stub_5b2fdc() -> ! {
    todo!("0x5b2fdc RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isReadOnly(void)const")
}

// 0x5b2fe0 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
pub fn stub_5b2fe0() -> ! {
    todo!("0x5b2fe0 RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isWriteOnly(void)const")
}

// 0x5b2fe4 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_5b2fe4() -> ! {
    todo!("0x5b2fe4 RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5b3004 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
pub fn stub_5b3004() -> ! {
    todo!("0x5b3004 RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")
}

// 0x5b3028 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5b3028() -> ! {
    todo!("0x5b3028 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5b31c0 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_5b31c0() -> ! {
    todo!("0x5b31c0 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x5b31f0 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
pub fn stub_5b31f0() -> ! {
    todo!("0x5b31f0 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x5b330c — __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5b330c() -> ! {
    todo!("0x5b330c RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5b33f0 — __ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
pub fn stub_5b33f0() -> ! {
    todo!("0x5b33f0 RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x5b34d8 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_5b34d8() -> ! {
    todo!("0x5b34d8 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x5b35dc — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
pub fn stub_5b35dc() -> ! {
    todo!("0x5b35dc RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x5b3690 — __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_5b3690() -> ! {
    todo!("0x5b3690 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5b36b4 — __ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Keyframe*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
pub fn stub_5b36b4() -> ! {
    todo!("0x5b36b4 RBX::Reflection::Call0Helper<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Keyframe*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)")
}

// 0x5b379c — __GLOBAL__I_a_220
#[doc(alias = "global constructor keyed to_a_220")]
// was: __GLOBAL__I_a_220
pub fn stub_5b379c() -> ! {
    todo!("0x5b379c `global constructor keyed to'_a_220")
}

// 0x5b3b08 — __ZN3RBX16KeyframeSequence12getKeyframesEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getKeyframes(void)")]
// was: __ZN3RBX16KeyframeSequence12getKeyframesEv
pub fn stub_5b3b08() -> ! {
    todo!("0x5b3b08 RBX::KeyframeSequence::getKeyframes(void)")
}

// 0x5b3b1c — __ZN3RBX16KeyframeSequence11addKeyframeEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::addKeyframe(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX16KeyframeSequence11addKeyframeEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b3b1c() -> ! {
    todo!("0x5b3b1c RBX::KeyframeSequence::addKeyframe(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x5b3b28 — __ZN3RBX16KeyframeSequence14removeKeyframeEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::removeKeyframe(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX16KeyframeSequence14removeKeyframeEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b3b28() -> ! {
    todo!("0x5b3b28 RBX::KeyframeSequence::removeKeyframe(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x5b3b3c — __ZN3RBX16KeyframeSequence7setLoopEb
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, bool)
#[doc(alias = "RBX::KeyframeSequence::setLoop(bool)")]
// was: __ZN3RBX16KeyframeSequence7setLoopEb
pub fn stub_5b3b3c() -> ! {
    todo!("0x5b3b3c RBX::KeyframeSequence::setLoop(bool)")
}

// 0x5b3b5c — __ZN3RBX16KeyframeSequence11setPriorityENS0_8PriorityE
#[doc(alias = "RBX::KeyframeSequence::setPriority(RBX::KeyframeSequence::Priority)")]
// was: __ZN3RBX16KeyframeSequence11setPriorityENS0_8PriorityE
pub fn stub_5b3b5c() -> ! {
    todo!("0x5b3b5c RBX::KeyframeSequence::setPriority(RBX::KeyframeSequence::Priority)")
}

// 0x5b3b7c — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC1Ev
pub fn stub_5b3b7c() -> ! {
    todo!("0x5b3b7c RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")
}

// 0x5b3b80 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEEC2Ev
pub fn stub_5b3b80() -> ! {
    todo!("0x5b3b80 RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::EnumDesc(void)")
}

// 0x5b3d70 — __ZN3RBX16KeyframeSequenceC1Ev
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::KeyframeSequence(void)")]
// was: __ZN3RBX16KeyframeSequenceC1Ev
pub fn stub_5b3d70() -> ! {
    todo!("0x5b3d70 RBX::KeyframeSequence::KeyframeSequence(void)")
}

// 0x5b3d74 — __ZN3RBX16KeyframeSequenceC2Ev
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::KeyframeSequence(void)")]
// was: __ZN3RBX16KeyframeSequenceC2Ev
pub fn stub_5b3d74() -> ! {
    todo!("0x5b3d74 RBX::KeyframeSequence::KeyframeSequence(void)")
}

// 0x5b401c — __ZN3RBX16KeyframeSequence20copyKeyframeSequenceEPS0_
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::KeyframeSequence *)
#[doc(alias = "RBX::KeyframeSequence::copyKeyframeSequence(RBX::KeyframeSequence*)")]
// was: __ZN3RBX16KeyframeSequence20copyKeyframeSequenceEPS0_
pub fn stub_5b401c() -> ! {
    todo!("0x5b401c RBX::KeyframeSequence::copyKeyframeSequence(RBX::KeyframeSequence*)")
}

// 0x5b4174 — __ZN3RBXL9CopyChildEN5boost10shared_ptrINS_8InstanceEEEPS2_
#[doc(alias = "RBX::CopyChild(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*)")]
// was: __ZN3RBXL9CopyChildEN5boost10shared_ptrINS_8InstanceEEEPS2_
pub fn stub_5b4174() -> ! {
    todo!("0x5b4174 RBX::CopyChild(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*)")
}

// 0x5b417c — __ZNK3RBX16KeyframeSequence9cacheDataEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::cacheData(void)const")]
// was: __ZNK3RBX16KeyframeSequence9cacheDataEv
pub fn stub_5b417c() -> ! {
    todo!("0x5b417c RBX::KeyframeSequence::cacheData(void)const")
}

// 0x5b4364 — __ZNK3RBX16KeyframeSequence11getDurationEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getDuration(void)const")]
// was: __ZNK3RBX16KeyframeSequence11getDurationEv
pub fn stub_5b4364() -> ! {
    todo!("0x5b4364 RBX::KeyframeSequence::getDuration(void)const")
}

// 0x5b437c — __ZNK3RBX16KeyframeSequence5applyERSt6vectorINS_15PoseAccumulatorESaIS2_EEddf
// type: int __fastcall(int, int, int, int, double, float, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::KeyframeSequence::apply(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double,double,float)const")]
// was: __ZNK3RBX16KeyframeSequence5applyERSt6vectorINS_15PoseAccumulatorESaIS2_EEddf
pub fn stub_5b437c() -> ! {
    todo!("0x5b437c RBX::KeyframeSequence::apply(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double,double,float)const")
}

// 0x5b4774 — __ZN3RBX10CachedPose16interpolatePosesERKS0_S2_ff
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this, const RBX::CachedPose *, const RBX::CachedPose *, float, float)
#[doc(alias = "RBX::CachedPose::interpolatePoses(RBX::CachedPose const&,RBX::CachedPose const&,float,float)")]
// was: __ZN3RBX10CachedPose16interpolatePosesERKS0_S2_ff
pub fn stub_5b4774() -> ! {
    todo!("0x5b4774 RBX::CachedPose::interpolatePoses(RBX::CachedPose const&,RBX::CachedPose const&,float,float)")
}

// 0x5b48b0 — __ZN3RBX10CachedPose10blendPosesERKS0_S2_
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this, const RBX::CachedPose *, const RBX::CachedPose *)
#[doc(alias = "RBX::CachedPose::blendPoses(RBX::CachedPose const&,RBX::CachedPose const&)")]
// was: __ZN3RBX10CachedPose10blendPosesERKS0_S2_
pub fn stub_5b48b0() -> ! {
    todo!("0x5b48b0 RBX::CachedPose::blendPoses(RBX::CachedPose const&,RBX::CachedPose const&)")
}

// 0x5b496c — __ZN3RBX16KeyframeSequence12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX16KeyframeSequence12onChildAddedEPNS_8InstanceE
pub fn stub_5b496c() -> ! {
    todo!("0x5b496c RBX::KeyframeSequence::onChildAdded(RBX::Instance *)")
}

// 0x5b4974 — __ZN3RBX16KeyframeSequence15invalidateCacheEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::invalidateCache(void)")]
// was: __ZN3RBX16KeyframeSequence15invalidateCacheEv
pub fn stub_5b4974() -> ! {
    todo!("0x5b4974 RBX::KeyframeSequence::invalidateCache(void)")
}

// 0x5b497c — __ZN3RBX16KeyframeSequence14onChildRemovedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::onChildRemoved(RBX::Instance *)")]
// was: __ZN3RBX16KeyframeSequence14onChildRemovedEPNS_8InstanceE
pub fn stub_5b497c() -> ! {
    todo!("0x5b497c RBX::KeyframeSequence::onChildRemoved(RBX::Instance *)")
}

// 0x5b4984 — __ZNK3RBX16KeyframeSequence15AppendPosePass0ERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::AppendPosePass0(rbx_core::SharedPtr<RBX::Instance> const&)const")]
// was: __ZNK3RBX16KeyframeSequence15AppendPosePass0ERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b4984() -> ! {
    todo!("0x5b4984 RBX::KeyframeSequence::AppendPosePass0(rbx_core::SharedPtr<RBX::Instance> const&)const")
}

// 0x5b4bf8 — __ZNK3RBX16KeyframeSequence15AppendPosePass1ERKN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIPNS_10CachedPoseESaIS9_EE
#[doc(alias = "RBX::KeyframeSequence::AppendPosePass1(rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *)const")]
// was: __ZNK3RBX16KeyframeSequence15AppendPosePass1ERKN5boost10shared_ptrINS_8InstanceEEEPSt6vectorIPNS_10CachedPoseESaIS9_EE
pub fn stub_5b4bf8() -> ! {
    todo!("0x5b4bf8 RBX::KeyframeSequence::AppendPosePass1(rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *)const")
}

// 0x5b4f24 — __ZNK3RBX16KeyframeSequence12makeKeyframeEPNS_8KeyframeE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, RBX::Keyframe *)
#[doc(alias = "RBX::KeyframeSequence::makeKeyframe(RBX::Keyframe *)const")]
// was: __ZNK3RBX16KeyframeSequence12makeKeyframeEPNS_8KeyframeE
pub fn stub_5b4f24() -> ! {
    todo!("0x5b4f24 RBX::KeyframeSequence::makeKeyframe(RBX::Keyframe *)const")
}

// 0x5b50a4 — __ZNK3RBX16KeyframeSequence18cacheKeyframePass0ERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::KeyframeSequence::cacheKeyframePass0(rbx_core::SharedPtr<RBX::Instance> const&)const")]
// was: __ZNK3RBX16KeyframeSequence18cacheKeyframePass0ERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b50a4() -> ! {
    todo!("0x5b50a4 RBX::KeyframeSequence::cacheKeyframePass0(rbx_core::SharedPtr<RBX::Instance> const&)const")
}

// 0x5b520c — __ZNK3RBX16KeyframeSequence18cacheKeyframePass1ERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::cacheKeyframePass1(rbx_core::SharedPtr<RBX::Instance> const&)const")]
// was: __ZNK3RBX16KeyframeSequence18cacheKeyframePass1ERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_5b520c() -> ! {
    todo!("0x5b520c RBX::KeyframeSequence::cacheKeyframePass1(rbx_core::SharedPtr<RBX::Instance> const&)const")
}

// 0x5b52fc — __ZNK3RBX10CachedPose9getCFrameEv
// type: _DWORD __fastcall(RBX::CachedPose *__hidden this)
#[doc(alias = "RBX::CachedPose::getCFrame(void)const")]
// was: __ZNK3RBX10CachedPose9getCFrameEv
pub fn stub_5b52fc() -> ! {
    todo!("0x5b52fc RBX::CachedPose::getCFrame(void)const")
}

// 0x5b560c — __ZNK3RBX16KeyframeSequence17verifySetAncestorEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
// was: __ZNK3RBX16KeyframeSequence17verifySetAncestorEPKNS_8InstanceES3_
pub fn stub_5b560c() -> ! {
    todo!("0x5b560c RBX::KeyframeSequence::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")
}

// 0x5b56fc — __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
pub fn stub_5b56fc() -> ! {
    todo!("0x5b56fc RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x5b5720 — __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
pub fn stub_5b5720() -> ! {
    todo!("0x5b5720 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x5b582c — __ZNK3RBX16KeyframeSequence7getLoopEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getLoop(void)const")]
// was: __ZNK3RBX16KeyframeSequence7getLoopEv
pub fn stub_5b582c() -> ! {
    todo!("0x5b582c RBX::KeyframeSequence::getLoop(void)const")
}

// 0x5b5834 — __ZN3RBX10Reflection14PropDescriptorINS_16KeyframeSequenceEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::KeyframeSequence,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_16KeyframeSequenceEbED1Ev
pub fn stub_5b5834() -> ! {
    todo!("0x5b5834 RBX::Reflection::PropDescriptor<RBX::KeyframeSequence,bool>::~PropDescriptor()")
}

// 0x5b5858 — __ZNK3RBX16KeyframeSequence11getPriorityEv
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::getPriority(void)const")]
// was: __ZNK3RBX16KeyframeSequence11getPriorityEv
pub fn stub_5b5858() -> ! {
    todo!("0x5b5858 RBX::KeyframeSequence::getPriority(void)const")
}

// 0x5b5860 — __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_16KeyframeSequenceENS2_8PriorityEED1Ev
pub fn stub_5b5860() -> ! {
    todo!("0x5b5860 RBX::Reflection::EnumPropDescriptor<RBX::KeyframeSequence,RBX::KeyframeSequence::Priority>::~EnumPropDescriptor()")
}

// 0x5b5884 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::addPair(RBX::KeyframeSequence::Priority,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE7addPairES3_PKc
pub fn stub_5b5884() -> ! {
    todo!("0x5b5884 RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::addPair(RBX::KeyframeSequence::Priority,char const*)")
}

// 0x5b5be4 — __ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_
// type: unsigned int __fastcall(const std::string **, std::string *)
#[doc(alias = "unsigned long RBX::findOrAdd<std::string>(std::vector<std::string,std::allocator<std::string>> &,std::string const&)")]
// was: __ZN3RBX9findOrAddISsEEmRSt6vectorIT_SaIS2_EERKS2_
pub fn stub_5b5be4() -> ! {
    todo!("0x5b5be4 unsigned long RBX::findOrAdd<std::string>(std::vector<std::string,std::allocator<std::string>> &,std::string const&)")
}

// 0x5b5c28 — __ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
// type: int(void)
#[doc(alias = "unsigned long RBX::findOrAdd<std::pair<unsigned long,unsigned long>>(std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>> &,std::pair<unsigned long,unsigned long> const&)")]
// was: __ZN3RBX9findOrAddISt4pairImmEEEmRSt6vectorIT_SaIS4_EERKS4_
pub fn stub_5b5c28() -> ! {
    todo!("0x5b5c28 unsigned long RBX::findOrAdd<std::pair<unsigned long,unsigned long>>(std::vector<std::pair<unsigned long,unsigned long>,std::allocator<std::pair<unsigned long,unsigned long>>> &,std::pair<unsigned long,unsigned long> const&)")
}

// 0x5b5c7c — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")]
// was: __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE9push_backERKS1_
pub fn stub_5b5c7c() -> ! {
    todo!("0x5b5c7c std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::push_back(RBX::CachedPose const&)")
}

// 0x5b5cb8 — __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::resize(unsigned long,RBX::CachedPose *)")]
// was: __ZNSt6vectorIPN3RBX10CachedPoseESaIS2_EE6resizeEmS2_
pub fn stub_5b5cb8() -> ! {
    todo!("0x5b5cb8 std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>>::resize(unsigned long,RBX::CachedPose *)")
}

// 0x5b5cec — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
pub fn stub_5b5cec() -> ! {
    todo!("0x5b5cec void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")
}

// 0x5b5df4 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
pub fn stub_5b5df4() -> ! {
    todo!("0x5b5df4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")
}

// 0x5b5ef0 — __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX::KeyframeSequence::CachedKeyframe const&)")]
// was: __ZNSt6vectorIN3RBX16KeyframeSequence14CachedKeyframeESaIS2_EE9push_backERKS2_
pub fn stub_5b5ef0() -> ! {
    todo!("0x5b5ef0 std::vector<RBX::KeyframeSequence::CachedKeyframe,std::allocator<RBX::KeyframeSequence::CachedKeyframe>>::push_back(RBX::KeyframeSequence::CachedKeyframe const&)")
}

// 0x5b5f40 — __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
// type: int(void)
#[doc(alias = "std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::reserve(unsigned long)")]
// was: __ZNSt6vectorIN3RBX10CachedPoseESaIS1_EE7reserveEm
pub fn stub_5b5f40() -> ! {
    todo!("0x5b5f40 std::vector<RBX::CachedPose,std::allocator<RBX::CachedPose>>::reserve(unsigned long)")
}

// 0x5b5fcc — __ZN3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
// was: __ZN3RBX16KeyframeSequenceD1Ev
pub fn stub_5b5fcc() -> ! {
    todo!("0x5b5fcc RBX::KeyframeSequence::~KeyframeSequence()")
}

// 0x5b6104 — __ZN3RBX16KeyframeSequenceD0Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "RBX::KeyframeSequence::~KeyframeSequence()")]
// was: __ZN3RBX16KeyframeSequenceD0Ev
pub fn stub_5b6104() -> ! {
    todo!("0x5b6104 RBX::KeyframeSequence::~KeyframeSequence()")
}

// 0x5b61a4 — __ZNK3RBX16KeyframeSequence11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX16KeyframeSequence11askAddChildEPKNS_8InstanceE
pub fn stub_5b61a4() -> ! {
    todo!("0x5b61a4 RBX::KeyframeSequence::askAddChild(RBX::Instance const*)const")
}

// 0x5b61e0 — __ZNK3RBX16KeyframeSequence12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::KeyframeSequence *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequence::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX16KeyframeSequence12askSetParentEPKNS_8InstanceE
pub fn stub_5b61e0() -> ! {
    todo!("0x5b61e0 RBX::KeyframeSequence::askSetParent(RBX::Instance const*)const")
}

// 0x5b61e4 — __ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv
pub fn stub_5b61e4() -> ! {
    todo!("0x5b61e4 __ZNK3RBX14FactoryProductINS_16KeyframeSequenceENS_8InstanceELZNS_17sKeyframeSequenceEES2_E12getClassNameEv")
}

// 0x5b61f4 — __ZThn32_N3RBX16KeyframeSequenceD1Ev
// type: void __fastcall(RBX::KeyframeSequence *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KeyframeSequence::~KeyframeSequence()")]
// was: __ZThn32_N3RBX16KeyframeSequenceD1Ev
pub fn stub_5b61f4() -> ! {
    todo!("0x5b61f4 `non-virtual thunk to'RBX::KeyframeSequence::~KeyframeSequence()")
}