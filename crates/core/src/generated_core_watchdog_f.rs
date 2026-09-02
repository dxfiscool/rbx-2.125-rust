//! core watchdog f — 100 core stubs EA-sorted, gap filler after 0xf6fb4c (global fallback exhausted).
//! Source: ida/export.json (85545 funcs) EA-sorted asc fallback filter excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound — 0 fallback uncovered before, chose global fallback exhausted 100 not yet in rbx_core.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv")]
// 0x3f3940 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv
pub fn stub_0x3f3940() -> ! {
    todo!("0x3f3940 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3f437c — __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0x3f437c() -> ! {
    todo!("0x3f437c __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3f4380 — __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0x3f4380() -> ! {
    todo!("0x3f4380 __ZN3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3f4420 — __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x3f4420() -> ! {
    todo!("0x3f4420 __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3f4428 — __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x3f4428() -> ! {
    todo!("0x3f4428 __ZThn32_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3f44cc — __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x3f44cc() -> ! {
    todo!("0x3f44cc __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3f44d4 — __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x3f44d4() -> ! {
    todo!("0x3f44d4 __ZThn36_N3RBX10Reflection9DescribedINS_17CollectionServiceELZNS_18sCollectionServiceEENS_17NonFactoryProductINS_8InstanceELZNS_18sCollectionServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZN3RBX17RotateAxisCommand15rotateAboutAxisERKN3G3D7Matrix3ERKSt6vectorIPNS_10PVInstanceESaIS7_EE")]
// 0x3faac8 — __ZN3RBX17RotateAxisCommand15rotateAboutAxisERKN3G3D7Matrix3ERKSt6vectorIPNS_10PVInstanceESaIS7_EE
// type: void __fastcall(int, const G3D::Matrix3 *, _DWORD *)
pub fn stub_0x3faac8() -> ! {
    todo!("0x3faac8 __ZN3RBX17RotateAxisCommand15rotateAboutAxisERKN3G3D7Matrix3ERKSt6vectorIPNS_10PVInstanceESaIS7_EE")
}

#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_PKNS_8InstanceE")]
// 0x3fc864 — __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_0x3fc864() -> ! {
    todo!("0x3fc864 __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_PKNS_8InstanceE")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// 0x3ff478 — __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_0x3ff478() -> ! {
    todo!("0x3ff478 __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEE12getSelectionEv")]
// 0x3ff598 — __ZN3RBX17FilteredSelectionINS_8InstanceEE12getSelectionEv
// type: int __fastcall(int)
pub fn stub_0x3ff598() -> ! {
    todo!("0x3ff598 __ZN3RBX17FilteredSelectionINS_8InstanceEE12getSelectionEv")
}

#[doc(alias = "__ZN3RBX9Selection12setSelectionIN9__gnu_cxx17__normal_iteratorIPPNS_8InstanceESt6vectorIS5_SaIS5_EEEEEEvT_SB_")]
// 0x3ff5f0 — __ZN3RBX9Selection12setSelectionIN9__gnu_cxx17__normal_iteratorIPPNS_8InstanceESt6vectorIS5_SaIS5_EEEEEEvT_SB_
// type: int __fastcall(RBX::Selection *, RBX::Instance **, RBX::Instance **)
pub fn stub_0x3ff5f0() -> ! {
    todo!("0x3ff5f0 __ZN3RBX9Selection12setSelectionIN9__gnu_cxx17__normal_iteratorIPPNS_8InstanceESt6vectorIS5_SaIS5_EEEEEEvT_SB_")
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_8InstanceEEEEEPT_v")]
// 0x3ff614 — __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_8InstanceEEEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3ff614() -> ! {
    todo!("0x3ff614 __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_8InstanceEEEEEPT_v")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE15isNullClassNameEv")]
// 0x3ff788 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE15isNullClassNameEv
// type: int()
pub fn stub_0x3ff788() -> ! {
    todo!("0x3ff788 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE15isNullClassNameEv")
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_8InstanceEEEEEvv")]
// 0x3ff954 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_8InstanceEEEEEvv
pub fn stub_0x3ff954() -> ! {
    todo!("0x3ff954 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_8InstanceEEEEEvv")
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_8InstanceEEEEEmv")]
// 0x3ff958 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_8InstanceEEEEEmv
// type: int()
pub fn stub_0x3ff958() -> ! {
    todo!("0x3ff958 __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_8InstanceEEEEEmv")
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_v")]
// 0x3ffa30 — __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3ffa30() -> ! {
    todo!("0x3ffa30 __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_v")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEEC2Ev")]
// 0x3ffcdc — __ZN3RBX17FilteredSelectionINS_8InstanceEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x3ffcdc() -> ! {
    todo!("0x3ffcdc __ZN3RBX17FilteredSelectionINS_8InstanceEEC2Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEED1Ev")]
// 0x3ffe98 — __ZN3RBX17FilteredSelectionINS_8InstanceEED1Ev
// type: int()
pub fn stub_0x3ffe98() -> ! {
    todo!("0x3ffe98 __ZN3RBX17FilteredSelectionINS_8InstanceEED1Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEED0Ev")]
// 0x3ffe9c — __ZN3RBX17FilteredSelectionINS_8InstanceEED0Ev
// type: void __fastcall(void *)
pub fn stub_0x3ffe9c() -> ! {
    todo!("0x3ffe9c __ZN3RBX17FilteredSelectionINS_8InstanceEED0Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEE17onAncestorChangedERKNS_15AncestorChangedE")]
// 0x3fff3c — __ZN3RBX17FilteredSelectionINS_8InstanceEE17onAncestorChangedERKNS_15AncestorChangedE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x3fff3c() -> ! {
    todo!("0x3fff3c __ZN3RBX17FilteredSelectionINS_8InstanceEE17onAncestorChangedERKNS_15AncestorChangedE")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")]
// 0x40008c — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv
// type: int()
pub fn stub_0x40008c() -> ! {
    todo!("0x40008c __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// 0x400090 — __ZN3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int, int *)
pub fn stub_0x400090() -> ! {
    todo!("0x400090 __ZN3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE")
}

#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED1Ev")]
// 0x4000e4 — __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED1Ev
// type: int __fastcall(int)
pub fn stub_0x4000e4() -> ! {
    todo!("0x4000e4 __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED0Ev")]
// 0x4000ec — __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x4000ec() -> ! {
    todo!("0x4000ec __ZThn32_N3RBX17FilteredSelectionINS_8InstanceEED0Ev")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")]
// 0x4000f4 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv
// type: int()
pub fn stub_0x4000f4() -> ! {
    todo!("0x4000f4 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_18sFilteredSelectionEEE12getClassNameEv")
}

#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED1Ev")]
// 0x4000f8 — __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED1Ev
// type: int __fastcall(int)
pub fn stub_0x4000f8() -> ! {
    todo!("0x4000f8 __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED0Ev")]
// 0x400100 — __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x400100() -> ! {
    todo!("0x400100 __ZThn36_N3RBX17FilteredSelectionINS_8InstanceEED0Ev")
}

#[doc(alias = "__ZThn96_N3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// 0x400108 — __ZThn96_N3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int()
pub fn stub_0x400108() -> ! {
    todo!("0x400108 __ZThn96_N3RBX17FilteredSelectionINS_8InstanceEE18onSelectionChangedERKNS_16SelectionChangedE")
}

#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX8InstanceESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")]
// 0x400110 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX8InstanceESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
pub fn stub_0x400110() -> ! {
    todo!("0x400110 __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX8InstanceESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_8InstanceEED2Ev")]
// 0x4001a0 — __ZN3RBX17FilteredSelectionINS_8InstanceEED2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x4001a0() -> ! {
    todo!("0x4001a0 __ZN3RBX17FilteredSelectionINS_8InstanceEED2Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEE12getSelectionEv")]
// 0x4006f0 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE12getSelectionEv
// type: int __fastcall(int)
pub fn stub_0x4006f0() -> ! {
    todo!("0x4006f0 __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE12getSelectionEv")
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v")]
// 0x4007b4 — __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x4007b4() -> ! {
    todo!("0x4007b4 __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v")
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEvv")]
// 0x400928 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEvv
pub fn stub_0x400928() -> ! {
    todo!("0x400928 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEvv")
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEmv")]
// 0x40092c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEmv
// type: int()
pub fn stub_0x40092c() -> ! {
    todo!("0x40092c __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_13ModelInstanceEEEEEmv")
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v")]
// 0x400a04 — __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x400a04() -> ! {
    todo!("0x400a04 __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_13ModelInstanceEEEEEPT_v")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEEC2Ev")]
// 0x400cb0 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x400cb0() -> ! {
    todo!("0x400cb0 __ZN3RBX17FilteredSelectionINS_13ModelInstanceEEC2Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev")]
// 0x400e6c — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
// type: int()
pub fn stub_0x400e6c() -> ! {
    todo!("0x400e6c __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev")]
// 0x400e70 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
// type: void __fastcall(void *)
pub fn stub_0x400e70() -> ! {
    todo!("0x400e70 __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEE17onAncestorChangedERKNS_15AncestorChangedE")]
// 0x400f10 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x400f10() -> ! {
    todo!("0x400f10 __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE17onAncestorChangedERKNS_15AncestorChangedE")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// 0x401088 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int, int)
pub fn stub_0x401088() -> ! {
    todo!("0x401088 __ZN3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")
}

#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev")]
// 0x401104 — __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
// type: int __fastcall(int)
pub fn stub_0x401104() -> ! {
    todo!("0x401104 __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev")]
// 0x40110c — __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40110c() -> ! {
    todo!("0x40110c __ZThn32_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev")]
// 0x401114 — __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev
// type: int __fastcall(int)
pub fn stub_0x401114() -> ! {
    todo!("0x401114 __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev")]
// 0x40111c — __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x40111c() -> ! {
    todo!("0x40111c __ZThn36_N3RBX17FilteredSelectionINS_13ModelInstanceEED0Ev")
}

#[doc(alias = "__ZThn96_N3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// 0x401124 — __ZThn96_N3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int)
pub fn stub_0x401124() -> ! {
    todo!("0x401124 __ZThn96_N3RBX17FilteredSelectionINS_13ModelInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")
}

#[doc(alias = "__ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE9push_backERKS2_")]
// 0x40112c — __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x40112c() -> ! {
    todo!("0x40112c __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13ModelInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag")]
// 0x401158 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13ModelInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
pub fn stub_0x401158() -> ! {
    todo!("0x401158 __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13ModelInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "__ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x4011e8 — __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, char *__src, _DWORD *)
pub fn stub_0x4011e8() -> ! {
    todo!("0x4011e8 __ZNSt6vectorIPN3RBX13ModelInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX13ModelInstanceESaIS2_EE11_M_allocateEm")]
// 0x4012c8 — __ZNSt12_Vector_baseIPN3RBX13ModelInstanceESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x4012c8() -> ! {
    todo!("0x4012c8 __ZNSt12_Vector_baseIPN3RBX13ModelInstanceESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_13ModelInstanceEED2Ev")]
// 0x4012e0 — __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x4012e0() -> ! {
    todo!("0x4012e0 __ZN3RBX17FilteredSelectionINS_13ModelInstanceEED2Ev")
}

#[doc(alias = "__ZN3RBX8Instance14findCommonNodeEPS0_S1_")]
// 0x4017dc — __ZN3RBX8Instance14findCommonNodeEPS0_S1_
// type: RBX::Instance *__fastcall(RBX::Instance *this, RBX::Instance *, RBX::Instance *)
pub fn stub_0x4017dc() -> ! {
    todo!("0x4017dc __ZN3RBX8Instance14findCommonNodeEPS0_S1_")
}

#[doc(alias = "__ZNK3RBX8Instance11canAddChildEPKS0_")]
// 0x40181c — __ZNK3RBX8Instance11canAddChildEPKS0_
// type: int __fastcall(RBX::Instance *this, const RBX::Instance *)
pub fn stub_0x40181c() -> ! {
    todo!("0x40181c __ZNK3RBX8Instance11canAddChildEPKS0_")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x401cec — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
pub fn stub_0x401cec() -> ! {
    todo!("0x401cec __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEE12getSelectionEv")]
// 0x401e80 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEE12getSelectionEv
// type: int __fastcall(int)
pub fn stub_0x401e80() -> ! {
    todo!("0x401e80 __ZN3RBX17FilteredSelectionINS_10PVInstanceEE12getSelectionEv")
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v")]
// 0x401ed8 — __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x401ed8() -> ! {
    todo!("0x401ed8 __ZNK3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v")
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v")]
// 0x4020a0 — __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x4020a0() -> ! {
    todo!("0x4020a0 __ZNK3RBX15ServiceProvider4findINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_v")
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEvv")]
// 0x4022f8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEvv
pub fn stub_0x4022f8() -> ! {
    todo!("0x4022f8 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEvv")
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEmv")]
// 0x4022fc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEmv
// type: int()
pub fn stub_0x4022fc() -> ! {
    todo!("0x4022fc __ZN3RBX15ServiceProvider15doGetClassIndexINS_17FilteredSelectionINS_10PVInstanceEEEEEmv")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEEC2Ev")]
// 0x4023d4 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x4023d4() -> ! {
    todo!("0x4023d4 __ZN3RBX17FilteredSelectionINS_10PVInstanceEEC2Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEED1Ev")]
// 0x402590 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
// type: int()
pub fn stub_0x402590() -> ! {
    todo!("0x402590 __ZN3RBX17FilteredSelectionINS_10PVInstanceEED1Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEED0Ev")]
// 0x402594 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
// type: void __fastcall(void *)
pub fn stub_0x402594() -> ! {
    todo!("0x402594 __ZN3RBX17FilteredSelectionINS_10PVInstanceEED0Ev")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEE17onAncestorChangedERKNS_15AncestorChangedE")]
// 0x402634 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEE17onAncestorChangedERKNS_15AncestorChangedE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x402634() -> ! {
    todo!("0x402634 __ZN3RBX17FilteredSelectionINS_10PVInstanceEE17onAncestorChangedERKNS_15AncestorChangedE")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// 0x4027ac — __ZN3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int, int)
pub fn stub_0x4027ac() -> ! {
    todo!("0x4027ac __ZN3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")
}

#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev")]
// 0x402828 — __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
// type: int __fastcall(int)
pub fn stub_0x402828() -> ! {
    todo!("0x402828 __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev")]
// 0x402830 — __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x402830() -> ! {
    todo!("0x402830 __ZThn32_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev")]
// 0x402838 — __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev
// type: int __fastcall(int)
pub fn stub_0x402838() -> ! {
    todo!("0x402838 __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev")]
// 0x402840 — __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev
// type: int __fastcall(int)
pub fn stub_0x402840() -> ! {
    todo!("0x402840 __ZThn36_N3RBX17FilteredSelectionINS_10PVInstanceEED0Ev")
}

#[doc(alias = "__ZThn96_N3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")]
// 0x402848 — __ZThn96_N3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE
// type: int __fastcall(int)
pub fn stub_0x402848() -> ! {
    todo!("0x402848 __ZThn96_N3RBX17FilteredSelectionINS_10PVInstanceEE18onSelectionChangedERKNS_16SelectionChangedE")
}

#[doc(alias = "__ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE9push_backERKS2_")]
// 0x402850 — __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x402850() -> ! {
    todo!("0x402850 __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX10PVInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag")]
// 0x40287c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX10PVInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag
// type: _DWORD *__fastcall(_DWORD *, int, int *)
pub fn stub_0x40287c() -> ! {
    todo!("0x40287c __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX10PVInstanceESt6vectorIS4_SaIS4_EEEEPNS2_8InstanceEET_SC_SC_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "__ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x40290c — __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, char *__src, _DWORD *)
pub fn stub_0x40290c() -> ! {
    todo!("0x40290c __ZNSt6vectorIPN3RBX10PVInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX10PVInstanceESaIS2_EE11_M_allocateEm")]
// 0x4029ec — __ZNSt12_Vector_baseIPN3RBX10PVInstanceESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x4029ec() -> ! {
    todo!("0x4029ec __ZNSt12_Vector_baseIPN3RBX10PVInstanceESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "__ZN3RBX17FilteredSelectionINS_10PVInstanceEED2Ev")]
// 0x402a04 — __ZN3RBX17FilteredSelectionINS_10PVInstanceEED2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x402a04() -> ! {
    todo!("0x402a04 __ZN3RBX17FilteredSelectionINS_10PVInstanceEED2Ev")
}

#[doc(alias = "__ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_10PVInstanceEEEE13createServiceEv")]
// 0x402e34 — __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_10PVInstanceEEEE13createServiceEv
// type: int __fastcall(_DWORD *)
pub fn stub_0x402e34() -> ! {
    todo!("0x402e34 __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_10PVInstanceEEEE13createServiceEv")
}

#[doc(alias = "__ZNK3RBX13Configuration14askForbidChildEPKNS_8InstanceE")]
// 0x41657c — __ZNK3RBX13Configuration14askForbidChildEPKNS_8InstanceE
// type: bool __fastcall(RBX::Configuration *this, const RBX::Instance *lpsrc)
pub fn stub_0x41657c() -> ! {
    todo!("0x41657c __ZNK3RBX13Configuration14askForbidChildEPKNS_8InstanceE")
}

#[doc(alias = "__ZNK3RBX13Configuration12askSetParentEPKNS_8InstanceE")]
// 0x4165b8 — __ZNK3RBX13Configuration12askSetParentEPKNS_8InstanceE
// type: int __fastcall(signed int this, const RBX::Instance *, int, int)
pub fn stub_0x4165b8() -> ! {
    todo!("0x4165b8 __ZNK3RBX13Configuration12askSetParentEPKNS_8InstanceE")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv")]
// 0x416a1c — __ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv
// type: int()
pub fn stub_0x416a1c() -> ! {
    todo!("0x416a1c __ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv")]
// 0x416ad8 — __ZThn32_NK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv
// type: int()
pub fn stub_0x416ad8() -> ! {
    todo!("0x416ad8 __ZThn32_NK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD1Ev")]
// 0x416b94 — __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD1Ev
// type: int()
pub fn stub_0x416b94() -> ! {
    todo!("0x416b94 __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD1Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD2Ev")]
// 0x416b98 — __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x416b98() -> ! {
    todo!("0x416b98 __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator12getClassNameEv")]
// 0x416c34 — __ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator12getClassNameEv
// type: int __fastcall(_DWORD)
pub fn stub_0x416c34() -> ! {
    todo!("0x416c34 __ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator6createEv")]
// 0x416cbc — __ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_0x416cbc() -> ! {
    todo!("0x416cbc __ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator6createEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev")]
// 0x417290 — __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_0x417290() -> ! {
    todo!("0x417290 __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv")]
// 0x4174d4 — __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv
// type: void *()
pub fn stub_0x4174d4() -> ! {
    todo!("0x4174d4 __ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x417548 — __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0x417548() -> ! {
    todo!("0x417548 __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x41754c — __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0x41754c() -> ! {
    todo!("0x41754c __ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4175ec — __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x4175ec() -> ! {
    todo!("0x4175ec __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4175f4 — __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x4175f4() -> ! {
    todo!("0x4175f4 __ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x417698 — __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x417698() -> ! {
    todo!("0x417698 __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4176a0 — __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x4176a0() -> ! {
    todo!("0x4176a0 __ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZN3RBX19CornerWedgeInstanceC1Ev")]
// 0x417a70 — __ZN3RBX19CornerWedgeInstanceC1Ev
// type: RBX::Instance *__fastcall(RBX::CornerWedgeInstance *this)
pub fn stub_0x417a70() -> ! {
    todo!("0x417a70 __ZN3RBX19CornerWedgeInstanceC1Ev")
}

#[doc(alias = "__ZN3RBX19CornerWedgeInstanceD0Ev")]
// 0x417d78 — __ZN3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417d78() -> ! {
    todo!("0x417d78 __ZN3RBX19CornerWedgeInstanceD0Ev")
}

#[doc(alias = "__ZN3RBX19CornerWedgeInstanceD1Ev")]
// 0x417e28 — __ZN3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e28() -> ! {
    todo!("0x417e28 __ZN3RBX19CornerWedgeInstanceD1Ev")
}

#[doc(alias = "__ZThn32_N3RBX19CornerWedgeInstanceD0Ev")]
// 0x417e38 — __ZThn32_N3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e38() -> ! {
    todo!("0x417e38 __ZThn32_N3RBX19CornerWedgeInstanceD0Ev")
}

#[doc(alias = "__ZThn36_N3RBX19CornerWedgeInstanceD0Ev")]
// 0x417e40 — __ZThn36_N3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e40() -> ! {
    todo!("0x417e40 __ZThn36_N3RBX19CornerWedgeInstanceD0Ev")
}

#[doc(alias = "__ZThn132_N3RBX19CornerWedgeInstanceD0Ev")]
// 0x417e48 — __ZThn132_N3RBX19CornerWedgeInstanceD0Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e48() -> ! {
    todo!("0x417e48 __ZThn132_N3RBX19CornerWedgeInstanceD0Ev")
}

#[doc(alias = "__ZThn32_N3RBX19CornerWedgeInstanceD1Ev")]
// 0x417e50 — __ZThn32_N3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e50() -> ! {
    todo!("0x417e50 __ZThn32_N3RBX19CornerWedgeInstanceD1Ev")
}

#[doc(alias = "__ZThn36_N3RBX19CornerWedgeInstanceD1Ev")]
// 0x417e64 — __ZThn36_N3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e64() -> ! {
    todo!("0x417e64 __ZThn36_N3RBX19CornerWedgeInstanceD1Ev")
}

#[doc(alias = "__ZThn132_N3RBX19CornerWedgeInstanceD1Ev")]
// 0x417e78 — __ZThn132_N3RBX19CornerWedgeInstanceD1Ev
// type: void __fastcall(RBX::CornerWedgeInstance *__hidden this)
pub fn stub_0x417e78() -> ! {
    todo!("0x417e78 __ZThn132_N3RBX19CornerWedgeInstanceD1Ev")
}

