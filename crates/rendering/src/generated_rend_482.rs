//! rendering shard 482 — 120 stubs 0x93c5f0..0xf2e714 EA-sorted rendering filter Ogre|G3D|Render|Adorn|Mesh
//! Next 120 uncovered rendering Filtered EA asc (remaining 361 -> 120 batch).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc rendering filter Ogre|G3D|Render|Adorn|Mesh

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x93c5f0 — __ZN3RBX20ExtrusionMeshBuilder5buildERKNS0_12DetailParamsE
// type: int __fastcall(_DWORD, _DWORD)
// was: RBX::ExtrusionMeshBuilder::build(RBX::ExtrusionMeshBuilder::DetailParams const&)
#[doc(alias = "RBX::ExtrusionMeshBuilder::build(RBX::ExtrusionMeshBuilder::DetailParams const&)")]
#[doc(alias = "__ZN3RBX20ExtrusionMeshBuilder5buildERKNS0_12DetailParamsE")]
pub fn stub_93c5f0() -> ! {
    todo!("0x93c5f0 RBX::ExtrusionMeshBuilder::build(RBX::ExtrusionMeshBuilder::DetailParams const&)")
}


// 0x93d1d0 — __ZNK3RBX18HumanoidIdentifier15getRelevantMeshEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::HumanoidIdentifier *__hidden this, RBX::PartInstance *)
// was: RBX::HumanoidIdentifier::getRelevantMesh(RBX::PartInstance *)const
#[doc(alias = "RBX::HumanoidIdentifier::getRelevantMesh(RBX::PartInstance *)const")]
#[doc(alias = "__ZNK3RBX18HumanoidIdentifier15getRelevantMeshEPNS_12PartInstanceE")]
pub fn stub_93d1d0() -> ! {
    todo!("0x93d1d0 RBX::HumanoidIdentifier::getRelevantMesh(RBX::PartInstance *)const")
}


// 0xa24000 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator6createEv")]
pub fn stub_a24000() -> ! {
    todo!("0xa24000 __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator6createEv")
}


// 0xa242b8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(RBX::BevelMesh **, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
// was: boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)
#[doc(alias = "boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_a242b8() -> ! {
    todo!("0xa242b8 boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")
}


// 0xa24538 — __ZThn32_N3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn32_N3RBX12CylinderMeshD1Ev")]
pub fn stub_a24538() -> ! {
    todo!("0xa24538 non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")
}


// 0xa24548 — __ZThn36_N3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn36_N3RBX12CylinderMeshD0Ev")]
pub fn stub_a24548() -> ! {
    todo!("0xa24548 non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")
}


// 0xa245f0 — __ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_a245f0() -> ! {
    todo!("0xa245f0 __ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0xa24690 — __ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_a24690() -> ! {
    todo!("0xa24690 __ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0xa246a0 — __ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_a246a0() -> ! {
    todo!("0xa246a0 __ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0xa246b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_a246b0() -> ! {
    todo!("0xa246b0 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0xa246b8 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev
// type: _Rb_tree_node_base *__fastcall(_Rb_tree_node_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_a246b8() -> ! {
    todo!("0xa246b8 __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev")
}


// 0xa24bc8 — __ZN3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: RBX::BlockMesh::~BlockMesh()
#[doc(alias = "RBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZN3RBX9BlockMeshD0Ev")]
pub fn stub_a24bc8() -> ! {
    todo!("0xa24bc8 RBX::BlockMesh::~BlockMesh()")
}


// 0xa24c68 — __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv")]
pub fn stub_a24c68() -> ! {
    todo!("0xa24c68 __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv")
}


// 0xa24cd8 — __ZThn32_N3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn32_N3RBX9BlockMeshD0Ev")]
pub fn stub_a24cd8() -> ! {
    todo!("0xa24cd8 non-virtual thunk toRBX::BlockMesh::~BlockMesh()")
}


// 0xa24d80 — __ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_a24d80() -> ! {
    todo!("0xa24d80 __ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0xb10b78 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_b10b78() -> ! {
    todo!("0xb10b78 __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")
}


// 0xb10d08 — __ZN3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZN3RBX12CylinderMeshD0Ev")]
pub fn stub_b10d08() -> ! {
    todo!("0xb10d08 RBX::CylinderMesh::~CylinderMesh()")
}


// 0xb10da8 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
pub fn stub_b10da8() -> ! {
    todo!("0xb10da8 __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")
}


// 0xb10e18 — __ZThn32_N3RBX12CylinderMeshD0Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn32_N3RBX12CylinderMeshD0Ev")]
pub fn stub_b10e18() -> ! {
    todo!("0xb10e18 non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")
}


// 0xb10ec0 — __ZThn36_N3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: `non-virtual thunk to'RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZThn36_N3RBX12CylinderMeshD1Ev")]
pub fn stub_b10ec0() -> ! {
    todo!("0xb10ec0 non-virtual thunk toRBX::CylinderMesh::~CylinderMesh()")
}


// 0xb10ed0 — __ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_b10ed0() -> ! {
    todo!("0xb10ed0 __ZN3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0xb10ee0 — __ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_b10ee0() -> ! {
    todo!("0xb10ee0 __ZThn36_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0xb10f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_b10f88() -> ! {
    todo!("0xb10f88 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0xb10f90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_b10f90() -> ! {
    todo!("0xb10f90 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0xb10fb0 — __ZN3RBX4Name13callDoDeclareILZNS_13sCylinderMeshEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sCylinderMeshEEEEvv")]
pub fn stub_b10fb0() -> ! {
    todo!("0xb10fb0 __ZN3RBX4Name13callDoDeclareILZNS_13sCylinderMeshEEEEvv")
}


// 0xb11088 — __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(__guard *, int, int, int (*)(const char *, ...), pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator6createEv")]
pub fn stub_b11088() -> ! {
    todo!("0xb11088 __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator6createEv")
}


// 0xb11340 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_b11340() -> ! {
    todo!("0xb11340 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const")
}


// 0xb6f5f8 — __ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE")]
pub fn stub_b6f5f8() -> ! {
    todo!("0xb6f5f8 RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")
}


// 0xb6f678 — __ZN3RBX24FastClusterMeshGenerator16isPartCompositedEPNS_12PartInstanceE
// type: int __fastcall(RBX::PartInstance **this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::isPartComposited(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isPartComposited(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator16isPartCompositedEPNS_12PartInstanceE")]
pub fn stub_b6f678() -> ! {
    todo!("0xb6f678 RBX::FastClusterMeshGenerator::isPartComposited(RBX::PartInstance *)")
}


// 0xb6f800 — __ZN3RBX24FastClusterMeshGenerator10isPartHeadEPNS_12PartInstanceE
// type: int __fastcall(RBX::PartInstance **this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::isPartHead(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isPartHead(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator10isPartHeadEPNS_12PartInstanceE")]
pub fn stub_b6f800() -> ! {
    todo!("0xb6f800 RBX::FastClusterMeshGenerator::isPartHead(RBX::PartInstance *)")
}


// 0xb6f9bc — __ZN3RBX24FastClusterMeshGenerator11addInstanceEmPNS_12PartInstanceEPNS_5DecalEjNS_22eShadowCullingPriorityEPNS_11AsyncResultE
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int)
// was: RBX::FastClusterMeshGenerator::addInstance(unsigned long,RBX::PartInstance *,RBX::Decal *,unsigned int,RBX::eShadowCullingPriority,RBX::AsyncResult *)
#[doc(alias = "RBX::FastClusterMeshGenerator::addInstance(unsigned long,RBX::PartInstance *,RBX::Decal *,unsigned int,RBX::eShadowCullingPriority,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator11addInstanceEmPNS_12PartInstanceEPNS_5DecalEjNS_22eShadowCullingPriorityEPNS_11AsyncResultE")]
pub fn stub_b6f9bc() -> ! {
    todo!("0xb6f9bc RBX::FastClusterMeshGenerator::addInstance(unsigned long,RBX::PartInstance *,RBX::Decal *,unsigned int,RBX::eShadowCullingPriority,RBX::AsyncResult *)")
}


// 0xb70210 — __ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE
// type: int __fastcall(int, bool *, int, _DWORD *)
// was: RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)
#[doc(alias = "RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE")]
pub fn stub_b70210() -> ! {
    todo!("0xb70210 RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")
}


// 0xb70d90 — __ZN3RBX24FastClusterMeshGeneratorD1Ev
// type: void __fastcall(RBX::FastClusterMeshGenerator *__hidden this)
// was: RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()
#[doc(alias = "RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGeneratorD1Ev")]
pub fn stub_b70d90() -> ! {
    todo!("0xb70d90 RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")
}


// 0xb730b8 — __ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev
// type: _DWORD **__fastcall(_DWORD **)
// was: std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()
#[doc(alias = "std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")]
#[doc(alias = "__ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev")]
pub fn stub_b730b8() -> ! {
    todo!("0xb730b8 std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")
}


// 0xb73f30 — __ZN3RBX24FastClusterMeshGenerator20getRelativeTransformEPNS_12PartInstanceES2_
// type: void __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *, RBX::PartInstance *, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::getRelativeTransform(RBX::PartInstance *,RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::getRelativeTransform(RBX::PartInstance *,RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20getRelativeTransformEPNS_12PartInstanceES2_")]
pub fn stub_b73f30() -> ! {
    todo!("0xb73f30 RBX::FastClusterMeshGenerator::getRelativeTransform(RBX::PartInstance *,RBX::PartInstance *)")
}


// 0xb74384 — __ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, int)
// was: RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)
#[doc(alias = "RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb")]
pub fn stub_b74384() -> ! {
    todo!("0xb74384 RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")
}


// 0xb75100 — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void __fastcall(int *, int, int)
// was: std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance const&)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_b75100() -> ! {
    todo!("0xb75100 std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance const&)")
}


// 0xb755a8 — __ZSt24__uninitialized_copy_auxIPN3RBX24FastClusterMeshGenerator13BatchInstanceES3_ET0_T_S5_S4_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
// was: RBX::FastClusterMeshGenerator::BatchInstance * std::__uninitialized_copy_aux<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,std::__false_type)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance * std::__uninitialized_copy_aux<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX24FastClusterMeshGenerator13BatchInstanceES3_ET0_T_S5_S4_St12__false_type")]
pub fn stub_b755a8() -> ! {
    todo!("0xb755a8 RBX::FastClusterMeshGenerator::BatchInstance * std::__uninitialized_copy_aux<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,std::__false_type)")
}


// 0xb75794 — __ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX24FastClusterMeshGenerator13BatchInstanceES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
// was: RBX::FastClusterMeshGenerator::BatchInstance * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *)")]
#[doc(alias = "__ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX24FastClusterMeshGenerator13BatchInstanceES5_EET0_T_S7_S6_")]
pub fn stub_b75794() -> ! {
    todo!("0xb75794 RBX::FastClusterMeshGenerator::BatchInstance * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *)")
}


// 0xb758a8 — __ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_
// type: void __fastcall(int, std::_List_node_base *, int, int, void *, int)
// was: std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)
#[doc(alias = "std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "__ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_")]
pub fn stub_b758a8() -> ! {
    todo!("0xb758a8 std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")
}


// 0xb75984 — __ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_
// type: int __fastcall(int, __int64 *)
// was: RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)
#[doc(alias = "RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_")]
pub fn stub_b75984() -> ! {
    todo!("0xb75984 RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")
}


// 0xb75b3c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EEC2ERKS4_
// type: int __fastcall(int *, int *, int, int)
// was: std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::vector(std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>> const&)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::vector(std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EEC2ERKS4_")]
pub fn stub_b75b3c() -> ! {
    todo!("0xb75b3c std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::vector(std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>> const&)")
}


// 0xb75c48 — __ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX24FastClusterMeshGenerator13BatchInstanceESt6vectorIS4_SaIS4_EEEEPS4_ET0_T_SD_SC_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
// was: RBX::FastClusterMeshGenerator::BatchInstance* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*>(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*,std::__false_type)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*>(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX24FastClusterMeshGenerator13BatchInstanceESt6vectorIS4_SaIS4_EEEEPS4_ET0_T_SD_SC_St12__false_type")]
pub fn stub_b75c48() -> ! {
    todo!("0xb75c48 RBX::FastClusterMeshGenerator::BatchInstance* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*>(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*,std::__false_type)")
}


// 0xb76338 — __ZN3RBX24FastClusterMeshGenerator20isBodyPartCompositedEPNS_12PartInstanceE
// type: int __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *)
// was: RBX::FastClusterMeshGenerator::isBodyPartComposited(RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isBodyPartComposited(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX24FastClusterMeshGenerator20isBodyPartCompositedEPNS_12PartInstanceE")]
pub fn stub_b76338() -> ! {
    todo!("0xb76338 RBX::FastClusterMeshGenerator::isBodyPartComposited(RBX::PartInstance *)")
}


// 0xb7654c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, __int64 *, char **)
// was: std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_b7654c() -> ! {
    todo!("0xb7654c std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")
}


// 0xb7695c — __ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm
// type: unsigned int __fastcall(void **, unsigned int)
// was: std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm")]
pub fn stub_b7695c() -> ! {
    todo!("0xb7695c std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")
}


// 0xb77140 — __ZN3RBX17GeometryGenerator11addFileMeshEPNS_12FileMeshDataEPNS_13DataModelMeshEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsE
// type: void __fastcall(int, _DWORD *, int, RBX::PartInstance *, int, int)
// was: RBX::GeometryGenerator::addFileMesh(RBX::FileMeshData *,RBX::DataModelMesh *,RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&)
#[doc(alias = "RBX::GeometryGenerator::addFileMesh(RBX::FileMeshData *,RBX::DataModelMesh *,RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&)")]
#[doc(alias = "__ZN3RBX17GeometryGenerator11addFileMeshEPNS_12FileMeshDataEPNS_13DataModelMeshEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsE")]
pub fn stub_b77140() -> ! {
    todo!("0xb77140 RBX::GeometryGenerator::addFileMesh(RBX::FileMeshData *,RBX::DataModelMesh *,RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&)")
}


// 0xb77bcc — __ZN3RBXL8getColorEPNS_12PartInstanceEPNS_5DecalEPNS_13DataModelMeshERKNS_17GeometryGenerator7OptionsEjb
// type: int __fastcall(_BYTE *, RBX::PartInstance *this, int, int, int *, char, int)
// was: RBX::getColor(RBX::PartInstance *,RBX::Decal *,RBX::DataModelMesh *,RBX::GeometryGenerator::Options const&,unsigned int,bool)
#[doc(alias = "RBX::getColor(RBX::PartInstance *,RBX::Decal *,RBX::DataModelMesh *,RBX::GeometryGenerator::Options const&,unsigned int,bool)")]
#[doc(alias = "__ZN3RBXL8getColorEPNS_12PartInstanceEPNS_5DecalEPNS_13DataModelMeshERKNS_17GeometryGenerator7OptionsEjb")]
pub fn stub_b77bcc() -> ! {
    todo!("0xb77bcc RBX::getColor(RBX::PartInstance *,RBX::Decal *,RBX::DataModelMesh *,RBX::GeometryGenerator::Options const&,unsigned int,bool)")
}


// 0xb7ba5c — __ZN3RBX17GeometryGenerator11addPartImplEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE
// type: void __fastcall(float *, unsigned int, int, int *, _DWORD **)
// was: RBX::GeometryGenerator::addPartImpl(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)
#[doc(alias = "RBX::GeometryGenerator::addPartImpl(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)")]
#[doc(alias = "__ZN3RBX17GeometryGenerator11addPartImplEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE")]
pub fn stub_b7ba5c() -> ! {
    todo!("0xb7ba5c RBX::GeometryGenerator::addPartImpl(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)")
}


// 0xb7e120 — __ZN3RBXL9fetchMeshERKNS_6MeshIdEPNS_8InstanceEPNS_11AsyncResultE
// type: void __fastcall(int, _DWORD *, int, pthread_mutex_t *)
// was: RBX::fetchMesh(RBX::MeshId const&,RBX::Instance *,RBX::AsyncResult *)
#[doc(alias = "RBX::fetchMesh(RBX::MeshId const&,RBX::Instance *,RBX::AsyncResult *)")]
#[doc(alias = "__ZN3RBXL9fetchMeshERKNS_6MeshIdEPNS_8InstanceEPNS_11AsyncResultE")]
pub fn stub_b7e120() -> ! {
    todo!("0xb7e120 RBX::fetchMesh(RBX::MeshId const&,RBX::Instance *,RBX::AsyncResult *)")
}


// 0xb7e74c — __ZN3RBX17GeometryGenerator11addInstanceEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE
// type: int __fastcall(int, unsigned int, int, int *, _DWORD **)
// was: RBX::GeometryGenerator::addInstance(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)
#[doc(alias = "RBX::GeometryGenerator::addInstance(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)")]
#[doc(alias = "__ZN3RBX17GeometryGenerator11addInstanceEPNS_12PartInstanceEPNS_5DecalERKNS0_7OptionsERKN5boost10shared_ptrINS_12FileMeshDataEEE")]
pub fn stub_b7e74c() -> ! {
    todo!("0xb7e74c RBX::GeometryGenerator::addInstance(RBX::PartInstance *,RBX::Decal *,RBX::GeometryGenerator::Options const&,boost::shared_ptr<RBX::FileMeshData> const&)")
}


// 0xb7e7c8 — __ZN5boost10shared_ptrIN3RBX12FileMeshDataEED1Ev
// type: int __fastcall(int)
// was: boost::shared_ptr<RBX::FileMeshData>::~shared_ptr()
#[doc(alias = "boost::shared_ptr<RBX::FileMeshData>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12FileMeshDataEED1Ev")]
pub fn stub_b7e7c8() -> ! {
    todo!("0xb7e7c8 boost::shared_ptr<RBX::FileMeshData>::~shared_ptr()")
}


// 0xb8c108 — __ZN12_GLOBAL__N_118getExtraSlotMeshIdEPN3RBX12PartInstanceERKNS0_18HumanoidIdentifierERA4_KNS_16AccoutrementMeshE
// type: void __fastcall(std::string *, int, int, int *)
// was: `anonymous namespace'::getExtraSlotMeshId(RBX::PartInstance *,RBX::HumanoidIdentifier const&,`anonymous namespace'::AccoutrementMesh const(&)[4])
#[doc(alias = "anonymous namespace::getExtraSlotMeshId(RBX::PartInstance *,RBX::HumanoidIdentifier const&,anonymous namespace::AccoutrementMesh const(&)[4])")]
#[doc(alias = "__ZN12_GLOBAL__N_118getExtraSlotMeshIdEPN3RBX12PartInstanceERKNS0_18HumanoidIdentifierERA4_KNS_16AccoutrementMeshE")]
pub fn stub_b8c108() -> ! {
    todo!("0xb8c108 anonymous namespace::getExtraSlotMeshId(RBX::PartInstance *,RBX::HumanoidIdentifier const&,anonymous namespace::AccoutrementMesh const(&)[4])")
}


// 0xb8c644 — __ZSt16__introsort_loopIPN12_GLOBAL__N_116AccoutrementMeshEiNS0_28AccoutrementMeshIdComparatorEEvT_S4_T0_T1_
// type: int __fastcall(int result, __int64 *, int)
// was: void std::__introsort_loop<`anonymous namespace'::AccoutrementMesh *,int,`anonymous namespace'::AccoutrementMeshIdComparator>(`anonymous namespace'::AccoutrementMesh *,`anonymous namespace'::AccoutrementMesh *,int,`anonymous namespace'::AccoutrementMeshIdComparator)
#[doc(alias = "void std::__introsort_loop<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator)")]
#[doc(alias = "__ZSt16__introsort_loopIPN12_GLOBAL__N_116AccoutrementMeshEiNS0_28AccoutrementMeshIdComparatorEEvT_S4_T0_T1_")]
pub fn stub_b8c644() -> ! {
    todo!("0xb8c644 void std::__introsort_loop<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMeshIdComparator)")
}


// 0xb8c808 — __ZSt13__adjust_heapIPN12_GLOBAL__N_116AccoutrementMeshEiS1_NS0_28AccoutrementMeshIdComparatorEEvT_T0_S5_T1_T2_
// type: int __fastcall(int, int, int, int, int, int)
// was: void std::__adjust_heap<`anonymous namespace'::AccoutrementMesh *,int,`anonymous namespace'::AccoutrementMesh,`anonymous namespace'::AccoutrementMeshIdComparator>(`anonymous namespace'::AccoutrementMesh *,int,int,`anonymous namespace'::AccoutrementMesh,`anonymous namespace'::AccoutrementMeshIdComparator)
#[doc(alias = "void std::__adjust_heap<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,int,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator)")]
#[doc(alias = "__ZSt13__adjust_heapIPN12_GLOBAL__N_116AccoutrementMeshEiS1_NS0_28AccoutrementMeshIdComparatorEEvT_T0_S5_T1_T2_")]
pub fn stub_b8c808() -> ! {
    todo!("0xb8c808 void std::__adjust_heap<anonymous namespace::AccoutrementMesh *,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator>(anonymous namespace::AccoutrementMesh *,int,int,anonymous namespace::AccoutrementMesh,anonymous namespace::AccoutrementMeshIdComparator)")
}


// 0xb8c91c — __ZN3RBX13DataModelUtil11getFileMeshEPNS_13DataModelMeshE
// type: RBX::DataModelUtil *__fastcall(RBX::Reflection::ClassDescriptor **this, RBX::DataModelMesh *)
// was: RBX::DataModelUtil::getFileMesh(RBX::DataModelMesh *)
#[doc(alias = "RBX::DataModelUtil::getFileMesh(RBX::DataModelMesh *)")]
#[doc(alias = "__ZN3RBX13DataModelUtil11getFileMeshEPNS_13DataModelMeshE")]
pub fn stub_b8c91c() -> ! {
    todo!("0xb8c91c RBX::DataModelUtil::getFileMesh(RBX::DataModelMesh *)")
}


// 0xb9a3e4 — __ZN3RBX7MeshGen12addRefVertexEi
// type: int __fastcall(RBX::MeshGen *this, int)
// was: RBX::MeshGen::addRefVertex(int)
#[doc(alias = "RBX::MeshGen::addRefVertex(int)")]
#[doc(alias = "__ZN3RBX7MeshGen12addRefVertexEi")]
pub fn stub_b9a3e4() -> ! {
    todo!("0xb9a3e4 RBX::MeshGen::addRefVertex(int)")
}


// 0xb9a3e8 — __ZN3RBX7MeshGen13releaseVertexEi
// type: void __fastcall(RBX::MeshGen *this, int)
// was: RBX::MeshGen::releaseVertex(int)
#[doc(alias = "RBX::MeshGen::releaseVertex(int)")]
#[doc(alias = "__ZN3RBX7MeshGen13releaseVertexEi")]
pub fn stub_b9a3e8() -> ! {
    todo!("0xb9a3e8 RBX::MeshGen::releaseVertex(int)")
}


// 0xb9a3ec — __ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this, int)
// was: RBX::ManualObjectMeshGenAdapter::getVertex(int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getVertex(int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter9getVertexEi")]
pub fn stub_b9a3ec() -> ! {
    todo!("0xb9a3ec RBX::ManualObjectMeshGenAdapter::getVertex(int)")
}


// 0xb9a50c — __ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, unsigned int)
// was: RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter18reserveVertexRangeEm")]
pub fn stub_b9a50c() -> ! {
    todo!("0xb9a50c RBX::ManualObjectMeshGenAdapter::reserveVertexRange(unsigned long)")
}


// 0xb9a524 — __ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE
// type: int __fastcall(int, _DWORD *)
// was: RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter11allocVertexERKNS_7MeshGen6VertexE")]
pub fn stub_b9a524() -> ! {
    todo!("0xb9a524 RBX::ManualObjectMeshGenAdapter::allocVertex(RBX::MeshGen::Vertex const&)")
}


// 0xb9a558 — __ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, unsigned int)
// was: RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter17reserveIndexRangeEm")]
pub fn stub_b9a558() -> ! {
    todo!("0xb9a558 RBX::ManualObjectMeshGenAdapter::reserveIndexRange(unsigned long)")
}


// 0xb9a568 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, int, int, int)
// was: RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiii")]
pub fn stub_b9a568() -> ! {
    todo!("0xb9a568 RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int)")
}


// 0xb9a580 — __ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this, int, int, int, int)
// was: RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter27appendQuadFromVertexIndicesEiiii")]
pub fn stub_b9a580() -> ! {
    todo!("0xb9a580 RBX::ManualObjectMeshGenAdapter::appendQuadFromVertexIndices(int,int,int,int)")
}


// 0xb9a5b0 — __ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this, int, int)
// was: RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter19duplicateIndexRangeEii")]
pub fn stub_b9a5b0() -> ! {
    todo!("0xb9a5b0 RBX::ManualObjectMeshGenAdapter::duplicateIndexRange(int,int)")
}


// 0xb9a6d0 — __ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv
// type: int __fastcall(RBX::ManualObjectMeshGenAdapter *this)
// was: RBX::ManualObjectMeshGenAdapter::getIndexCount(void)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getIndexCount(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter13getIndexCountEv")]
pub fn stub_b9a6d0() -> ! {
    todo!("0xb9a6d0 RBX::ManualObjectMeshGenAdapter::getIndexCount(void)")
}


// 0xb9a6d4 — __ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this)
// was: RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter20getShadowVertexArrayEv")]
pub fn stub_b9a6d4() -> ! {
    todo!("0xb9a6d4 RBX::ManualObjectMeshGenAdapter::getShadowVertexArray(void)")
}


// 0xb9a7f4 — __ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv
// type: void __fastcall __noreturn(RBX::ManualObjectMeshGenAdapter *this)
// was: RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)")]
#[doc(alias = "__ZN3RBX26ManualObjectMeshGenAdapter19getShadowIndexArrayEv")]
pub fn stub_b9a7f4() -> ! {
    todo!("0xb9a7f4 RBX::ManualObjectMeshGenAdapter::getShadowIndexArray(void)")
}


// 0xb9a918 — __ZN3RBX7MeshGen20popVerticesTransformEv
// type: void __fastcall(RBX::MeshGen *this)
// was: RBX::MeshGen::popVerticesTransform(void)
#[doc(alias = "RBX::MeshGen::popVerticesTransform(void)")]
#[doc(alias = "__ZN3RBX7MeshGen20popVerticesTransformEv")]
pub fn stub_b9a918() -> ! {
    todo!("0xb9a918 RBX::MeshGen::popVerticesTransform(void)")
}


// 0xbef8d8 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_bef8d8() -> ! {
    todo!("0xbef8d8 __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD1Ev")
}


// 0xbef8e0 — __ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")]
pub fn stub_bef8e0() -> ! {
    todo!("0xbef8e0 __ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")
}


// 0xc2c0a0 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_c2c0a0() -> ! {
    todo!("0xc2c0a0 __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev")
}


// 0xc2c220 — __ZN3RBX12CylinderMeshD1Ev
// type: void __fastcall(RBX::CylinderMesh *__hidden this)
// was: RBX::CylinderMesh::~CylinderMesh()
#[doc(alias = "RBX::CylinderMesh::~CylinderMesh()")]
#[doc(alias = "__ZN3RBX12CylinderMeshD1Ev")]
pub fn stub_c2c220() -> ! {
    todo!("0xc2c220 RBX::CylinderMesh::~CylinderMesh()")
}


// 0xc2c260 — __ZThn32_NK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
pub fn stub_c2c260() -> ! {
    todo!("0xc2c260 __ZThn32_NK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")
}


// 0xc2cf20 — __ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_c2cf20() -> ! {
    todo!("0xc2cf20 __ZThn32_N3RBX10Reflection9DescribedINS_12CylinderMeshELZNS_13sCylinderMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0xc2cfd0 — __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_c2cfd0() -> ! {
    todo!("0xc2cfd0 __ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}


// 0xc2d3b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_c2d3b0() -> ! {
    todo!("0xc2d3b0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const")
}


// 0xc2d520 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_c2d520() -> ! {
    todo!("0xc2d520 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0xc2d530 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_c2d530() -> ! {
    todo!("0xc2d530 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0xc2d670 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_c2d670() -> ! {
    todo!("0xc2d670 __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev")
}


// 0xc2d7f0 — __ZThn32_N3RBX9BlockMeshD1Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn32_N3RBX9BlockMeshD1Ev")]
pub fn stub_c2d7f0() -> ! {
    todo!("0xc2d7f0 non-virtual thunk toRBX::BlockMesh::~BlockMesh()")
}


// 0xc2d800 — __ZThn36_N3RBX9BlockMeshD1Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
// was: `non-virtual thunk to'RBX::BlockMesh::~BlockMesh()
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
#[doc(alias = "__ZThn36_N3RBX9BlockMeshD1Ev")]
pub fn stub_c2d800() -> ! {
    todo!("0xc2d800 non-virtual thunk toRBX::BlockMesh::~BlockMesh()")
}


// 0xc2d810 — __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_c2d810() -> ! {
    todo!("0xc2d810 __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0xc2d8b0 — __ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_c2d8b0() -> ! {
    todo!("0xc2d8b0 __ZThn32_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0xc2d8c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_c2d8c0() -> ! {
    todo!("0xc2d8c0 boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0xc2d8d0 — __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")]
pub fn stub_c2d8d0() -> ! {
    todo!("0xc2d8d0 __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")
}


// 0xc2d9f0 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_c2d9f0() -> ! {
    todo!("0xc2d9f0 __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev")
}


// 0xc304a0 — __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_c304a0() -> ! {
    todo!("0xc304a0 __ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}


// 0xf20488 — __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim")]
pub fn stub_f20488() -> ! {
    todo!("0xf20488 __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim")
}


// 0xf2077c — __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")]
pub fn stub_f2077c() -> ! {
    todo!("0xf2077c __ZNK3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")
}


// 0xf20788 — __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev$shim
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_f20788() -> ! {
    todo!("0xf20788 __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorD2Ev$shim")
}


// 0xf20794 — __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim")]
pub fn stub_f20794() -> ! {
    todo!("0xf20794 __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim")
}


// 0xf207b8 — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
#[doc(alias = "__ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub fn stub_f207b8() -> ! {
    todo!("0xf207b8 __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")
}


// 0xf207c4 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_$shim")]
pub fn stub_f207c4() -> ! {
    todo!("0xf207c4 __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_$shim")
}


// 0xf21250 — __ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v$shim")]
pub fn stub_f21250() -> ! {
    todo!("0xf21250 __ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v$shim")
}


// 0xf2125c — __ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_$shim")]
pub fn stub_f2125c() -> ! {
    todo!("0xf2125c __ZNK3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEE14convertToIndexES3_$shim")
}


// 0xf21268 — __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub fn stub_f21268() -> ! {
    todo!("0xf21268 __ZNSt6vectorIN3RBX13DataModelMesh7LODTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")
}


// 0xf212c8 — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev$shim
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_f212c8() -> ! {
    todo!("0xf212c8 __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev$shim")
}


// 0xf21340 — __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim")]
pub fn stub_f21340() -> ! {
    todo!("0xf21340 __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim")
}


// 0xf2134c — __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator12getClassNameEv$shim
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")]
pub fn stub_f2134c() -> ! {
    todo!("0xf2134c __ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")
}


// 0xf21358 — __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim")]
pub fn stub_f21358() -> ! {
    todo!("0xf21358 __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim")
}


// 0xf219b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim")]
pub fn stub_f219b8() -> ! {
    todo!("0xf219b8 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim")
}


// 0xf219c4 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim")]
pub fn stub_f219c4() -> ! {
    todo!("0xf219c4 __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim")
}


// 0xf21a30 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a30() -> ! {
    todo!("0xf21a30 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim")
}


// 0xf21a3c — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim")]
pub fn stub_f21a3c() -> ! {
    todo!("0xf21a3c __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim")
}


// 0xf21c4c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DataModelMesh7LODTypeEEEE14doGetSingletonEv$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DataModelMesh7LODTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21c4c() -> ! {
    todo!("0xf21c4c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DataModelMesh7LODTypeEEEE14doGetSingletonEv$shim")
}


// 0xf21c58 — __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEED2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEED2Ev$shim")]
pub fn stub_f21c58() -> ! {
    todo!("0xf21c58 __ZN3RBX10Reflection8EnumDescINS_13DataModelMesh7LODTypeEED2Ev$shim")
}


// 0xf232d8 — __ZNK3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7Creator12getClassNameEv$shim")]
pub fn stub_f232d8() -> ! {
    todo!("0xf232d8 __ZNK3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7Creator12getClassNameEv$shim")
}


// 0xf232e4 — __ZN3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_f232e4() -> ! {
    todo!("0xf232e4 __ZN3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7CreatorD2Ev$shim")
}


// 0xf232f0 — __ZN3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_f232f0() -> ! {
    todo!("0xf232f0 __ZN3RBX14FactoryProductINS_12SpecialShapeENS_8FileMeshELZNS_13sSpecialShapeEENS_8InstanceEE7CreatorD2Ev$shim")
}


// 0xf23308 — __ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim
// type: int()
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim")]
pub fn stub_f23308() -> ! {
    todo!("0xf23308 __ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim")
}


// 0xf23314 — __ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim")]
pub fn stub_f23314() -> ! {
    todo!("0xf23314 __ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim")
}


// 0xf23320 — __ZNK3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7Creator12getClassNameEv$shim
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")]
pub fn stub_f23320() -> ! {
    todo!("0xf23320 __ZNK3RBX14FactoryProductINS_8FileMeshENS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEE7Creator12getClassNameEv$shim")
}


// 0xf2332c — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim")]
pub fn stub_f2332c() -> ! {
    todo!("0xf2332c __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim")
}


// 0xf23338 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
#[doc(alias = "__ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub fn stub_f23338() -> ! {
    todo!("0xf23338 __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")
}


// 0xf24208 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")]
pub fn stub_f24208() -> ! {
    todo!("0xf24208 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")
}


// 0xf24214 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim
// type: int()
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")]
pub fn stub_f24214() -> ! {
    todo!("0xf24214 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv$shim")
}


// 0xf25984 — __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev$shim
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_f25984() -> ! {
    todo!("0xf25984 __ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev$shim")
}


// 0xf25a20 — __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv$shim
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv$shim")]
pub fn stub_f25a20() -> ! {
    todo!("0xf25a20 __ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv$shim")
}


// 0xf2e714 — j___ZN3RBX11IndexedMesh13lowersChangedEv
// type: _DWORD __fastcall(RBX::IndexedMesh *__hidden this)
// was: RBX::IndexedMesh::lowersChanged(void)
#[doc(alias = "RBX::IndexedMesh::lowersChanged(void)")]
#[doc(alias = "j___ZN3RBX11IndexedMesh13lowersChangedEv")]
pub fn stub_f2e714() -> ! {
    todo!("0xf2e714 RBX::IndexedMesh::lowersChanged(void)")
}
