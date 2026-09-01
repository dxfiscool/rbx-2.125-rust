//! rendering shard 297 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32240->32340 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32240 before -> 32340 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x417168

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x41716c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_41716c() -> ! {
    todo!("0x41716c boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x417170 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_417170() -> ! {
    todo!("0x417170 boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x417190 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_417190() -> ! {
    todo!("0x417190 boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4171a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ConfigurationENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4171a8() -> ! {
    todo!("0x4171a8 boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4171ac — __ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv
pub fn stub_4171ac() -> ! {
    todo!("0x4171ac __ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv")
}

// 0x4171b0 — __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v
pub fn stub_4171b0() -> ! {
    todo!("0x4171b0 __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")
}

// 0x417290 — __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev
pub fn stub_417290() -> ! {
    todo!("0x417290 __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev")
}

// 0x4174d4 — __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv
pub fn stub_4174d4() -> ! {
    todo!("0x4174d4 __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv")
}

// 0x417548 — __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_417548() -> ! {
    todo!("0x417548 __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x41754c — __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_41754c() -> ! {
    todo!("0x41754c __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x4175ec — __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_4175ec() -> ! {
    todo!("0x4175ec __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x4175f4 — __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_4175f4() -> ! {
    todo!("0x4175f4 __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x417698 — __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_417698() -> ! {
    todo!("0x417698 __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x4176a0 — __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_4176a0() -> ! {
    todo!("0x4176a0 __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x417744 — __GLOBAL__I_a_175
#[doc(alias = "global constructor keyed to_a_175")]
// was: __GLOBAL__I_a_175
pub fn stub_417744() -> ! {
    todo!("0x417744 `global constructor keyed to'_a_175")
}

// 0x417a70 — __ZN3RBX19CornerWedgeInstanceC1Ev
// type: RBX::Instance *__fastcall(RBX::CornerWedgeInstance *this)
#[doc(alias = "RBX::CornerWedgeInstance::CornerWedgeInstance(void)")]
// was: __ZN3RBX19CornerWedgeInstanceC1Ev
pub fn stub_417a70() -> ! {
    todo!("0x417a70 RBX::CornerWedgeInstance::CornerWedgeInstance(void)")
}

// 0x417d78 — __ZN3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "RBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZN3RBX19CornerWedgeInstanceD0Ev
pub fn stub_417d78() -> ! {
    todo!("0x417d78 RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e28 — __ZN3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "RBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZN3RBX19CornerWedgeInstanceD1Ev
pub fn stub_417e28() -> ! {
    todo!("0x417e28 RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e38 — __ZThn32_N3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZThn32_N3RBX19CornerWedgeInstanceD0Ev
pub fn stub_417e38() -> ! {
    todo!("0x417e38 `non-virtual thunk to'RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e40 — __ZThn36_N3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZThn36_N3RBX19CornerWedgeInstanceD0Ev
pub fn stub_417e40() -> ! {
    todo!("0x417e40 `non-virtual thunk to'RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e48 — __ZThn132_N3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZThn132_N3RBX19CornerWedgeInstanceD0Ev
pub fn stub_417e48() -> ! {
    todo!("0x417e48 `non-virtual thunk to'RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e50 — __ZThn32_N3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZThn32_N3RBX19CornerWedgeInstanceD1Ev
pub fn stub_417e50() -> ! {
    todo!("0x417e50 `non-virtual thunk to'RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e64 — __ZThn36_N3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZThn36_N3RBX19CornerWedgeInstanceD1Ev
pub fn stub_417e64() -> ! {
    todo!("0x417e64 `non-virtual thunk to'RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e78 — __ZThn132_N3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
// was: __ZThn132_N3RBX19CornerWedgeInstanceD1Ev
pub fn stub_417e78() -> ! {
    todo!("0x417e78 `non-virtual thunk to'RBX::CornerWedgeInstance::~CornerWedgeInstance()")
}

// 0x417e8c — __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv
pub fn stub_417e8c() -> ! {
    todo!("0x417e8c __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv")
}

// 0x417e9c — __ZNK3RBX19CornerWedgeInstance11getPartTypeEv
// type: int __fastcall(RBX::CornerWedgeInstance *this)
#[doc(alias = "RBX::CornerWedgeInstance::getPartType(void)const")]
// was: __ZNK3RBX19CornerWedgeInstance11getPartTypeEv
pub fn stub_417e9c() -> ! {
    todo!("0x417e9c RBX::CornerWedgeInstance::getPartType(void)const")
}

// 0x417ea0 — __ZThn32_NK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv
pub fn stub_417ea0() -> ! {
    todo!("0x417ea0 __ZThn32_NK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE12getClassNameEv")
}

// 0x417eb0 — __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_417eb0() -> ! {
    todo!("0x417eb0 __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x417ec4 — __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_417ec4() -> ! {
    todo!("0x417ec4 __ZN3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x417f74 — __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_417f74() -> ! {
    todo!("0x417f74 __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x417f88 — __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_417f88() -> ! {
    todo!("0x417f88 __ZThn132_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x41803c — __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_41803c() -> ! {
    todo!("0x41803c __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x418050 — __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_418050() -> ! {
    todo!("0x418050 __ZN3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x418100 — __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_418100() -> ! {
    todo!("0x418100 __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x418114 — __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_418114() -> ! {
    todo!("0x418114 __ZThn132_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x4181c8 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
pub fn stub_4181c8() -> ! {
    todo!("0x4181c8 __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")
}

// 0x4181dc — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
// type: void __fastcall(RBX::PartInstance *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
pub fn stub_4181dc() -> ! {
    todo!("0x4181dc __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")
}

// 0x41828c — __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")]
// was: __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
pub fn stub_41828c() -> ! {
    todo!("0x41828c __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")
}

// 0x4182a0 — __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")]
// was: __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
pub fn stub_4182a0() -> ! {
    todo!("0x4182a0 __ZThn132_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")
}

// 0x4182a8 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD1Ev
pub fn stub_4182a8() -> ! {
    todo!("0x4182a8 __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD1Ev")
}

// 0x4182ac — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD2Ev
pub fn stub_4182ac() -> ! {
    todo!("0x4182ac __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorD2Ev")
}

// 0x418348 — __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_418348() -> ! {
    todo!("0x418348 __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x4183d0 — __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator6createEv
pub fn stub_4183d0() -> ! {
    todo!("0x4183d0 __ZNK3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7Creator6createEv")
}

// 0x418514 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19CornerWedgeInstanceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CornerWedgeInstance> RBX::Creatable<RBX::Instance>::create<RBX::CornerWedgeInstance>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_19CornerWedgeInstanceEEEN5boost10shared_ptrIT_EEv
pub fn stub_418514() -> ! {
    todo!("0x418514 rbx_core::SharedPtr<RBX::CornerWedgeInstance> RBX::Creatable<RBX::Instance>::create<RBX::CornerWedgeInstance>(void)")
}

// 0x4185c8 — __ZN5boost10shared_ptrIN3RBX19CornerWedgeInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CornerWedgeInstance>::shared_ptr<RBX::CornerWedgeInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX19CornerWedgeInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_4185c8() -> ! {
    todo!("0x4185c8 rbx_core::SharedPtr<RBX::CornerWedgeInstance>::shared_ptr<RBX::CornerWedgeInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x418690 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CornerWedgeInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CornerWedgeInstance,RBX::CornerWedgeInstance>(rbx_core::SharedPtr<RBX::CornerWedgeInstance> const*,RBX::CornerWedgeInstance *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CornerWedgeInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_418690() -> ! {
    todo!("0x418690 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CornerWedgeInstance,RBX::CornerWedgeInstance>(rbx_core::SharedPtr<RBX::CornerWedgeInstance> const*,RBX::CornerWedgeInstance *)const")
}

// 0x418778 — __ZN5boost6detail12shared_countC2IPN3RBX19CornerWedgeInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX19CornerWedgeInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_418778() -> ! {
    todo!("0x418778 boost::detail::shared_count::shared_count<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x418880 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_418880() -> ! {
    todo!("0x418880 boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x418884 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_418884() -> ! {
    todo!("0x418884 boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x418888 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_418888() -> ! {
    todo!("0x418888 boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4188a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4188a8() -> ! {
    todo!("0x4188a8 boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4188c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CornerWedgeInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4188c0() -> ! {
    todo!("0x4188c0 boost::detail::sp_counted_impl_pd<RBX::CornerWedgeInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4188c4 — __ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv
pub fn stub_4188c4() -> ! {
    todo!("0x4188c4 __ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv")
}

// 0x4188c8 — __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v
pub fn stub_4188c8() -> ! {
    todo!("0x4188c8 __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v")
}

// 0x4189a8 — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorC2Ev
pub fn stub_4189a8() -> ! {
    todo!("0x4189a8 __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE7CreatorC2Ev")
}

// 0x418bec — __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE17static_getCreatorEv
pub fn stub_418bec() -> ! {
    todo!("0x418bec __ZN3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEE17static_getCreatorEv")
}

// 0x418c60 — __ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
pub fn stub_418c60() -> ! {
    todo!("0x418c60 __ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")
}

// 0x418c74 — __ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev
pub fn stub_418c74() -> ! {
    todo!("0x418c74 __ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED1Ev")
}

// 0x418c88 — __ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
pub fn stub_418c88() -> ! {
    todo!("0x418c88 __ZThn32_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")
}

// 0x418c90 — __ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev
pub fn stub_418c90() -> ! {
    todo!("0x418c90 __ZThn36_N3RBX14FactoryProductINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEED0Ev")
}

// 0x418c98 — __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_418c98() -> ! {
    todo!("0x418c98 __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x418cac — __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_418cac() -> ! {
    todo!("0x418cac __ZThn32_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x418d60 — __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_418d60() -> ! {
    todo!("0x418d60 __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x418d74 — __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_418d74() -> ! {
    todo!("0x418d74 __ZThn36_N3RBX18DescribedCreatableINS_19CornerWedgeInstanceENS_12PartInstanceELZNS_12sCornerWedgeEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x418e94 — __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_418e94() -> ! {
    todo!("0x418e94 __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x418ea8 — __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_418ea8() -> ! {
    todo!("0x418ea8 __ZThn32_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x418f5c — __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_418f5c() -> ! {
    todo!("0x418f5c __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x418f70 — __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_418f70() -> ! {
    todo!("0x418f70 __ZThn36_N3RBX10Reflection9DescribedINS_19CornerWedgeInstanceELZNS_12sCornerWedgeEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sCornerWedgeEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x419024 — __GLOBAL__I_a_176
#[doc(alias = "global constructor keyed to_a_176")]
// was: __GLOBAL__I_a_176
pub fn stub_419024() -> ! {
    todo!("0x419024 `global constructor keyed to'_a_176")
}

// 0x419344 — __GLOBAL__I_a_177
#[doc(alias = "global constructor keyed to_a_177")]
// was: __GLOBAL__I_a_177
pub fn stub_419344() -> ! {
    todo!("0x419344 `global constructor keyed to'_a_177")
}

// 0x419518 — __ZN3RBXL11dummyLoaderEPNS_9DataModelE
// type: void __fastcall(RBX *this, RBX::DataModel *)
#[doc(alias = "RBX::dummyLoader(RBX::DataModel *)")]
// was: __ZN3RBXL11dummyLoaderEPNS_9DataModelE
pub fn stub_419518() -> ! {
    todo!("0x419518 RBX::dummyLoader(RBX::DataModel *)")
}

// 0x41951c — __ZN3RBX9DataModel3getENS_9ContentIdE
// type: void __fastcall(boost::detail::sp_counted_base *, int, const std::string *, int)
#[doc(alias = "RBX::DataModel::get(RBX::ContentId)")]
// was: __ZN3RBX9DataModel3getENS_9ContentIdE
pub fn stub_41951c() -> ! {
    todo!("0x41951c RBX::DataModel::get(RBX::ContentId)")
}

// 0x419894 — __ZN3RBX9DataModel9loadWorldEi
// type: void __fastcall(RBX::DataModel *this, int, bool)
#[doc(alias = "RBX::DataModel::loadWorld(int)")]
// was: __ZN3RBX9DataModel9loadWorldEi
pub fn stub_419894() -> ! {
    todo!("0x419894 RBX::DataModel::loadWorld(int)")
}

// 0x419a60 — __ZN3RBX9DataModel8loadGameEi
// type: void __fastcall(RBX::DataModel *this, int, bool)
#[doc(alias = "RBX::DataModel::loadGame(int)")]
// was: __ZN3RBX9DataModel8loadGameEi
pub fn stub_419a60() -> ! {
    todo!("0x419a60 RBX::DataModel::loadGame(int)")
}

// 0x419be0 — __ZN3RBX9DataModel11loadContentENS_9ContentIdE
// type: void __fastcall(int, const std::string *)
#[doc(alias = "RBX::DataModel::loadContent(RBX::ContentId)")]
// was: __ZN3RBX9DataModel11loadContentENS_9ContentIdE
pub fn stub_419be0() -> ! {
    todo!("0x419be0 RBX::DataModel::loadContent(RBX::ContentId)")
}

// 0x419e98 — __ZN3RBX9DataModel4saveENS_9ContentIdE
// type: void __fastcall(RBX::DataModel *, const std::string *)
#[doc(alias = "RBX::DataModel::save(RBX::ContentId)")]
// was: __ZN3RBX9DataModel4saveENS_9ContentIdE
pub fn stub_419e98() -> ! {
    todo!("0x419e98 RBX::DataModel::save(RBX::ContentId)")
}

// 0x419fd8 — __ZN3RBX9DataModel18setRemoteBuildModeEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RBX::DataModel::setRemoteBuildMode(bool)")]
// was: __ZN3RBX9DataModel18setRemoteBuildModeEb
pub fn stub_419fd8() -> ! {
    todo!("0x419fd8 RBX::DataModel::setRemoteBuildMode(bool)")
}

// 0x419fe0 — __ZN3RBX9DataModel18getRemoteBuildModeEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getRemoteBuildMode(void)")]
// was: __ZN3RBX9DataModel18getRemoteBuildModeEv
pub fn stub_419fe0() -> ! {
    todo!("0x419fe0 RBX::DataModel::getRemoteBuildMode(void)")
}

// 0x419fe8 — __ZN3RBX9DataModel16setServerSaveUrlESs
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::setServerSaveUrl(std::string)")]
// was: __ZN3RBX9DataModel16setServerSaveUrlESs
pub fn stub_419fe8() -> ! {
    todo!("0x419fe8 RBX::DataModel::setServerSaveUrl(std::string)")
}

// 0x419ff0 — __ZN3RBX9DataModel12httpGetAsyncESsN5boost8functionIFvSsEEES4_
// type: void __fastcall(int, std::string *, int, int)
#[doc(alias = "RBX::DataModel::httpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel12httpGetAsyncESsN5boost8functionIFvSsEEES4_
pub fn stub_419ff0() -> ! {
    todo!("0x419ff0 RBX::DataModel::httpGetAsync(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x41a210 — __ZN3RBX9DataModel13httpPostAsyncESsSsN5boost8functionIFvSsEEES4_
// type: void __fastcall(int, std::string *, int, int, int)
#[doc(alias = "RBX::DataModel::httpPostAsync(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel13httpPostAsyncESsSsN5boost8functionIFvSsEEES4_
pub fn stub_41a210() -> ! {
    todo!("0x41a210 RBX::DataModel::httpPostAsync(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x41a430 — __ZN3RBX9DataModel7httpGetESsb
// type: void __fastcall(RBX::DataModel *, int, const std::string *, int)
#[doc(alias = "RBX::DataModel::httpGet(std::string,bool)")]
// was: __ZN3RBX9DataModel7httpGetESsb
pub fn stub_41a430() -> ! {
    todo!("0x41a430 RBX::DataModel::httpGet(std::string,bool)")
}

// 0x41a7a0 — __ZN3RBX9DataModel8httpPostESsSsb
// type: void __fastcall(RBX::DataModel *, int, const std::string *, const std::string *, int)
#[doc(alias = "RBX::DataModel::httpPost(std::string,std::string,bool)")]
// was: __ZN3RBX9DataModel8httpPostESsSsb
pub fn stub_41a7a0() -> ! {
    todo!("0x41a7a0 RBX::DataModel::httpPost(std::string,std::string,bool)")
}

// 0x41abf8 — __ZN3RBX9DataModel11getJobsInfoEv
// type: int __fastcall(RBX::DataModel *this, int)
#[doc(alias = "RBX::DataModel::getJobsInfo(void)")]
// was: __ZN3RBX9DataModel11getJobsInfoEv
pub fn stub_41abf8() -> ! {
    todo!("0x41abf8 RBX::DataModel::getJobsInfo(void)")
}

// 0x41b47c — __ZN3RBX9DataModel17reportMeasurementESsSsSsSsSs
// type: int __fastcall(int, const std::string *, const std::string *, const std::string *, const std::string *, const std::string *)
#[doc(alias = "RBX::DataModel::reportMeasurement(std::string,std::string,std::string,std::string,std::string)")]
// was: __ZN3RBX9DataModel17reportMeasurementESsSsSsSsSs
pub fn stub_41b47c() -> ! {
    todo!("0x41b47c RBX::DataModel::reportMeasurement(std::string,std::string,std::string,std::string,std::string)")
}

// 0x41b498 — __ZN3RBX9DataModel13clearContentsEb
// type: void __fastcall(RBX::DataModel *this, int, int, int)
#[doc(alias = "RBX::DataModel::clearContents(bool)")]
// was: __ZN3RBX9DataModel13clearContentsEb
pub fn stub_41b498() -> ! {
    todo!("0x41b498 RBX::DataModel::clearContents(bool)")
}

// 0x41bac0 — __ZN3RBX9DataModel5closeEv
// type: void __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::close(void)")]
// was: __ZN3RBX9DataModel5closeEv
pub fn stub_41bac0() -> ! {
    todo!("0x41bac0 RBX::DataModel::close(void)")
}

// 0x41bcbc — __ZN3RBX9DataModel14toggleToolsOffEv
// type: void __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::toggleToolsOff(void)")]
// was: __ZN3RBX9DataModel14toggleToolsOffEv
pub fn stub_41bcbc() -> ! {
    todo!("0x41bcbc RBX::DataModel::toggleToolsOff(void)")
}

// 0x41be10 — __ZNK3RBX9DataModel12canSaveLocalEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::canSaveLocal(void)const")]
// was: __ZNK3RBX9DataModel12canSaveLocalEv
pub fn stub_41be10() -> ! {
    todo!("0x41be10 RBX::DataModel::canSaveLocal(void)const")
}

// 0x41be14 — __ZN3RBX9DataModel12saveToRobloxEN5boost8functionIFvbEEENS2_IFvSsEEE
// type: void __fastcall(RBX::DataModel *, const RBX::Instance *)
#[doc(alias = "RBX::DataModel::saveToRoblox(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel12saveToRobloxEN5boost8functionIFvbEEENS2_IFvSsEEE
pub fn stub_41be14() -> ! {
    todo!("0x41be14 RBX::DataModel::saveToRoblox(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")
}

// 0x41c07c — __ZN3RBX9DataModel16completeShutdownEb
// type: void __fastcall(RBX::DataModel *this, int)
#[doc(alias = "RBX::DataModel::completeShutdown(bool)")]
// was: __ZN3RBX9DataModel16completeShutdownEb
pub fn stub_41c07c() -> ! {
    todo!("0x41c07c RBX::DataModel::completeShutdown(bool)")
}

// 0x41c284 — __ZN3RBX9DataModel12setUiMessageESs
// type: int __fastcall(int)
#[doc(alias = "RBX::DataModel::setUiMessage(std::string)")]
// was: __ZN3RBX9DataModel12setUiMessageESs
pub fn stub_41c284() -> ! {
    todo!("0x41c284 RBX::DataModel::setUiMessage(std::string)")
}

// 0x41c28c — __ZN3RBX9DataModel14clearUiMessageEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::clearUiMessage(void)")]
// was: __ZN3RBX9DataModel14clearUiMessageEv
pub fn stub_41c28c() -> ! {
    todo!("0x41c28c RBX::DataModel::clearUiMessage(void)")
}

// 0x41c2a0 — __ZN3RBX9DataModel20getJobsExtendedStatsEv
// type: int __fastcall(RBX::DataModel *this, int)
#[doc(alias = "RBX::DataModel::getJobsExtendedStats(void)")]
// was: __ZN3RBX9DataModel20getJobsExtendedStatsEv
pub fn stub_41c2a0() -> ! {
    todo!("0x41c2a0 RBX::DataModel::getJobsExtendedStats(void)")
}

// 0x41cd40 — __ZN3RBX9DataModel22getJobTimePeakFractionESsd
// type: __int64 __fastcall(RBX::TaskScheduler *, const std::string *, __int64)
#[doc(alias = "RBX::DataModel::getJobTimePeakFraction(std::string,double)")]
// was: __ZN3RBX9DataModel22getJobTimePeakFractionESsd
pub fn stub_41cd40() -> ! {
    todo!("0x41cd40 RBX::DataModel::getJobTimePeakFraction(std::string,double)")
}

// 0x41cf9c — __ZN3RBX9DataModel26getJobIntervalPeakFractionESsd
// type: __int64 __fastcall(RBX::TaskScheduler *, const std::string *, __int64)
#[doc(alias = "RBX::DataModel::getJobIntervalPeakFraction(std::string,double)")]
// was: __ZN3RBX9DataModel26getJobIntervalPeakFractionESsd
pub fn stub_41cf9c() -> ! {
    todo!("0x41cf9c RBX::DataModel::getJobIntervalPeakFraction(std::string,double)")
}

// 0x41d1f8 — __ZN3RBX9DataModel26setJobsExtendedStatsWindowEd
// type: int __fastcall(RBX::DataModel *this, double)
#[doc(alias = "RBX::DataModel::setJobsExtendedStatsWindow(double)")]
// was: __ZN3RBX9DataModel26setJobsExtendedStatsWindowEd
pub fn stub_41d1f8() -> ! {
    todo!("0x41d1f8 RBX::DataModel::setJobsExtendedStatsWindow(double)")
}

// 0x41d210 — __ZN3RBX9DataModel15setPlaceVersionEi
// type: char *__fastcall(RBX::DataModel *this, char *, int, const void *)
#[doc(alias = "RBX::DataModel::setPlaceVersion(int)")]
// was: __ZN3RBX9DataModel15setPlaceVersionEi
pub fn stub_41d210() -> ! {
    todo!("0x41d210 RBX::DataModel::setPlaceVersion(int)")
}

// 0x41d260 — __ZN3RBX9DataModel10setPlaceIDEib
// type: _DWORD __fastcall(RBX::DataModel *__hidden this, char *, bool)
#[doc(alias = "RBX::DataModel::setPlaceID(int,bool)")]
// was: __ZN3RBX9DataModel10setPlaceIDEib
pub fn stub_41d260() -> ! {
    todo!("0x41d260 RBX::DataModel::setPlaceID(int,bool)")
}

// 0x41d2c8 — __ZN3RBX9DataModel28activateExperimentalFeaturesEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::DataModel::activateExperimentalFeatures(void)")]
// was: __ZN3RBX9DataModel28activateExperimentalFeaturesEv
pub fn stub_41d2c8() -> ! {
    todo!("0x41d2c8 RBX::DataModel::activateExperimentalFeatures(void)")
}
