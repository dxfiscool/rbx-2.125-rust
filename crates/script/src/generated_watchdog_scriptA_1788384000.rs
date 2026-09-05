//! script generated_watchdog_scriptA_1788384000 — 100 stubs EA-sorted asc global dedup (Script/Lua filter exhausted, gap filler)
//! Filter: Lua|Script|lapi|lgc|lvm|ldo|StringTable (case-sensitive) -> 3 total, 0 remaining before batch (all already in script crate) — gap filler from remaining EAs
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x74e430..0xa7c414 | EA-sorted asc distinct not yet in global_eas.txt (global dedup)
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0x74e430 — __ZN3RBX9PrismPolyD1Ev
// type: void __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::~PrismPoly()")]
#[doc(alias = "__ZN3RBX9PrismPolyD1Ev")]
pub fn stub_0x74e430(handle: crate::slot::InstanceHandle) {
// RBX::PrismPoly dtor.
drop(handle);
}

// 0x74e454 — __ZN3RBX9PrismPolyD0Ev
// type: void __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::~PrismPoly() [0x74e454]")]
#[doc(alias = "__ZN3RBX9PrismPolyD0Ev")]
pub fn stub_0x74e454(handle: crate::slot::InstanceHandle) {
// RBX::PrismPoly dtor.
drop(handle);
}

// 0x74e508 — __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
// type: int(void)
#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
pub fn stub_0x74e508(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x74e734 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
pub fn stub_0x74e734(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PrismMesh, RBX::Vector3_2IntsComparer>::r~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74e910 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
pub fn stub_0x74e910(handle: crate::slot::InstanceHandle) {
// RBX::GeometryPool dtor.
drop(handle);
}

// 0x74ea6c — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv")]
pub fn stub_0x74ea6c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PrismMesh>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74eaa8 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
pub fn stub_0x74eaa8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x74ead0 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
pub fn stub_0x74ead0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x74eb30 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
pub fn stub_0x74eb30(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x74eb58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
pub fn stub_0x74eb58() -> crate::slot::PortedFn {
// IDA 0x74eb58: std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::P~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x74eb58, "std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3~")
}

// 0x74eb88 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
pub fn stub_0x74eb88() -> crate::slot::PortedFn {
// IDA 0x74eb88: std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::P~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x74eb88, "std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3~")
}

// 0x74ec70 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
pub fn stub_0x74ec70(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x74ed50 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
// type: int __fastcall(int, int, _Rb_tree_node_base *, _QWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
pub fn stub_0x74ed50(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x74edb4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
pub fn stub_0x74edb4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x74ee34 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
pub fn stub_0x74ee34(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PrismMesh, RBX::Vector3_2IntsComparer>::V~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74eef4 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm")]
pub fn stub_0x74eef4(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PrismMesh>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74ef64 — __ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE
#[doc(alias = "RBX::POLY::PrismMesh::PrismMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE")]
pub fn stub_0x74ef64(handle: &crate::slot::InstanceHandle) {
// RBX::POLY::PrismMesh::PrismMesh(RBX::Vector3_2Ints const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74f0b4 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev")]
pub fn stub_0x74f0b4(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PrismMesh>::Allocator() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74f118 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv")]
pub fn stub_0x74f118(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PrismMesh>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74f19c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
pub fn stub_0x74f19c(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

// 0x74f1ec — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
pub fn stub_0x74f1ec(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PrismMesh, RBX::Vector3_2IntsComparer>::s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74f1f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
pub fn stub_0x74f1f0(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PrismMesh, RBX::Vector3_2IntsComparer>::s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74f300 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
pub fn stub_0x74f300(handle: crate::slot::InstanceHandle) {
// RBX::GeometryPool dtor.
drop(handle);
}

// 0x74f4e0 — __ZN3RBX11PyramidPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX11PyramidPoly9buildMeshEv")]
pub fn stub_0x74f4e0(handle: &crate::slot::InstanceHandle) {
// RBX::PyramidPoly::buildMesh() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x74f5c4 — __ZN3RBX11PyramidPoly20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, const std::string *, int)
#[doc(alias = "RBX::PyramidPoly::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX11PyramidPoly20setGeometryParameterERKSsi")]
pub fn stub_0x74f5c4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::PyramidPoly setter.
cell.set(value)
}

// 0x74f668 — __ZNK3RBX11PyramidPoly20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, const std::string *)
#[doc(alias = "RBX::PyramidPoly::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly20getGeometryParameterERKSs")]
pub fn stub_0x74f668(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PyramidPoly getter.
cell.get()
}

// 0x74f6f4 — __ZNK3RBX11PyramidPoly9getMomentEf
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, float)
#[doc(alias = "RBX::PyramidPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly9getMomentEf")]
pub fn stub_0x74f6f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PyramidPoly getter.
cell.get()
}

// 0x74f828 — __ZNK3RBX11PyramidPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly13getCofmOffsetEv")]
pub fn stub_0x74f828(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PyramidPoly getter.
cell.get()
}

// 0x74f85c — __ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, unsigned int)
#[doc(alias = "RBX::PyramidPoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm")]
pub fn stub_0x74f85c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PyramidPoly getter.
cell.get()
}

// 0x74fbc8 — __ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::PyramidPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
pub fn stub_0x74fbc8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PyramidPoly getter.
cell.get()
}

// 0x74fd08 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
pub fn stub_0x74fd08(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GeometryPool getter.
cell.get()
}

// 0x74fe6c — __ZN3RBX11PyramidPolyD1Ev
// type: void __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::~PyramidPoly()")]
#[doc(alias = "__ZN3RBX11PyramidPolyD1Ev")]
pub fn stub_0x74fe6c(handle: crate::slot::InstanceHandle) {
// RBX::PyramidPoly dtor.
drop(handle);
}

// 0x74fe90 — __ZN3RBX11PyramidPolyD0Ev
// type: void __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::~PyramidPoly() [0x74fe90]")]
#[doc(alias = "__ZN3RBX11PyramidPolyD0Ev")]
pub fn stub_0x74fe90(handle: crate::slot::InstanceHandle) {
// RBX::PyramidPoly dtor.
drop(handle);
}

// 0x74ff44 — __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
pub fn stub_0x74ff44(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x750170 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
pub fn stub_0x750170(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PyramidMesh, RBX::Vector3_2IntsComparer>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x75034c — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
pub fn stub_0x75034c(handle: crate::slot::InstanceHandle) {
// RBX::GeometryPool dtor.
drop(handle);
}

// 0x7504a8 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv")]
pub fn stub_0x7504a8(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PyramidMesh>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7504e4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
pub fn stub_0x7504e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x75050c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
pub fn stub_0x75050c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x75056c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
pub fn stub_0x75056c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x750594 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
pub fn stub_0x750594() -> crate::slot::PortedFn {
// IDA 0x750594: std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::P~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x750594, "std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3~")
}

// 0x7505c4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
pub fn stub_0x7505c4() -> crate::slot::PortedFn {
// IDA 0x7505c4: std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::P~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x7505c4, "std::_Rb_tree<RBX::Vector3_2Ints, std::pair<RBX::Vector3_2Ints const, RBX::GeometryPool<RBX::Vector3~")
}

// 0x7505f4 — __ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_
#[doc(alias = "RBX::Vector3_2IntsComparer::operator()(RBX::Vector3_2Ints const&,RBX::Vector3_2Ints const&)const")]
#[doc(alias = "__ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_")]
pub fn stub_0x7505f4(handle: &crate::slot::InstanceHandle) {
// RBX::Vector3_2IntsComparer::operator()(RBX::Vector3_2Ints const&, RBX::Vector3_2Ints const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x75072c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
pub fn stub_0x75072c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x75080c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
pub fn stub_0x75080c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x750870 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
pub fn stub_0x750870(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x7508f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
pub fn stub_0x7508f0(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PyramidMesh, RBX::Vector3_2IntsComparer>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7509b0 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")]
pub fn stub_0x7509b0(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x750a20 — __ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE
#[doc(alias = "RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")]
pub fn stub_0x750a20(handle: &crate::slot::InstanceHandle) {
// RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x750b70 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")]
pub fn stub_0x750b70(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x750bd4 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv")]
pub fn stub_0x750bd4(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::PyramidMesh>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x750c58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
pub fn stub_0x750c58(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

// 0x750ca8 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
pub fn stub_0x750ca8(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PyramidMesh, RBX::Vector3_2IntsComparer>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x750cac — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
pub fn stub_0x750cac(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryPool<RBX::Vector3_2Ints, RBX::POLY::PyramidMesh, RBX::Vector3_2IntsComparer>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x750dbc — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
pub fn stub_0x750dbc(handle: crate::slot::InstanceHandle) {
// RBX::GeometryPool dtor.
drop(handle);
}

// 0x750f9c — __ZN3RBX18RightAngleRampPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX18RightAngleRampPoly9buildMeshEv")]
pub fn stub_0x750f9c(handle: &crate::slot::InstanceHandle) {
// RBX::RightAngleRampPoly::buildMesh() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x751078 — __ZNK3RBX18RightAngleRampPoly9getMomentEf
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this, float)
#[doc(alias = "RBX::RightAngleRampPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly9getMomentEf")]
pub fn stub_0x751078(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RightAngleRampPoly getter.
cell.get()
}

// 0x7511ac — __ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv")]
pub fn stub_0x7511ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RightAngleRampPoly getter.
cell.get()
}

// 0x7511e0 — __ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::RightAngleRampPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
pub fn stub_0x7511e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RightAngleRampPoly getter.
cell.get()
}

// 0x751398 — __ZN3RBX18RightAngleRampPolyD1Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD1Ev")]
pub fn stub_0x751398(handle: crate::slot::InstanceHandle) {
// RBX::RightAngleRampPoly dtor.
drop(handle);
}

// 0x7513bc — __ZN3RBX18RightAngleRampPolyD0Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly() [0x7513bc]")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD0Ev")]
pub fn stub_0x7513bc(handle: crate::slot::InstanceHandle) {
// RBX::RightAngleRampPoly dtor.
drop(handle);
}

// 0x751a00 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")]
pub fn stub_0x751a00(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x75203c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")]
pub fn stub_0x75203c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7521c8 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")]
pub fn stub_0x7521c8(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x75222c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv")]
pub fn stub_0x75222c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::RightAngleRampMesh>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x752648 — __ZN3RBX10RigidJoint9isAlignedEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
#[doc(alias = "RBX::RigidJoint::isAligned(void)")]
#[doc(alias = "__ZN3RBX10RigidJoint9isAlignedEv")]
pub fn stub_0x752648(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RigidJoint getter.
cell.get()
}

// 0x752720 — __ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RigidJoint::align(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_")]
pub fn stub_0x752720(handle: &crate::slot::InstanceHandle) {
// RBX::RigidJoint::align(RBX::Primitive*, RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x752884 — __ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RigidJoint::getChildInParent(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_")]
pub fn stub_0x752884(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RigidJoint getter.
cell.get()
}

// 0x752b14 — __ZNK3RBX10RigidJoint8isBrokenEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
#[doc(alias = "RBX::RigidJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX10RigidJoint8isBrokenEv")]
pub fn stub_0x752b14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RigidJoint getter.
cell.get()
}

// 0x752be0 — __ZN3RBX11RotateJointC1Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC1Ev")]
pub fn stub_0x752be0() -> crate::slot::InstanceHandle {
// RBX::RotateJoint ctor.
crate::slot::InstanceHandle::new("RBX::RotateJoint")
}

// 0x752c04 — __ZN3RBX11RotateJointC2Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::RotateJoint(void) [0x752c04]")]
#[doc(alias = "__ZN3RBX11RotateJointC2Ev")]
pub fn stub_0x752c04() -> crate::slot::InstanceHandle {
// RBX::RotateJoint ctor.
crate::slot::InstanceHandle::new("RBX::RotateJoint")
}

// 0x752c28 — __ZN3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD0Ev")]
pub fn stub_0x752c28(handle: crate::slot::InstanceHandle) {
// RBX::RotateJoint dtor.
drop(handle);
}

// 0x752cc8 — __ZN3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::~RotateJoint() [0x752cc8]")]
#[doc(alias = "__ZN3RBX11RotateJointD1Ev")]
pub fn stub_0x752cc8(handle: crate::slot::InstanceHandle) {
// RBX::RotateJoint dtor.
drop(handle);
}

// 0x752ccc — __ZThn32_N3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD0Ev")]
pub fn stub_0x752ccc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x752cd4 — __ZThn32_N3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint() [0x752cd4]")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD1Ev")]
pub fn stub_0x752cd4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x752cdc — __ZN3RBX11RotateJoint21getAxleWorldDirectionEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getAxleWorldDirection(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint21getAxleWorldDirectionEv")]
pub fn stub_0x752cdc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RotateJoint getter.
cell.get()
}

// 0x752cfc — __ZN3RBX11RotateJoint15getAxleVelocityEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getAxleVelocity(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint15getAxleVelocityEv")]
pub fn stub_0x752cfc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RotateJoint getter.
cell.get()
}

// 0x752fe8 — __ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::RotateJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
pub fn stub_0x752fe8(handle: &crate::slot::InstanceHandle) {
// RBX::RotateJoint::canBuildJoint(RBX::Primitive*, RBX::Primitive*, RBX::NormalId, RBX::Norm~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7537c4 — __ZN3RBX11RotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint16removeFromKernelEv")]
pub fn stub_0x7537c4(handle: &crate::slot::InstanceHandle) {
// RBX::RotateJoint::removeFromKernel() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x753828 — __ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, float *, float *)
#[doc(alias = "RBX::RotateJoint::getPrimitivesTorqueArmLength(float &,float &)")]
#[doc(alias = "__ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_")]
pub fn stub_0x753828(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RotateJoint getter.
cell.get()
}

// 0x753910 — __ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::RotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE")]
pub fn stub_0x753910(handle: &crate::slot::InstanceHandle) {
// RBX::RotateJoint::putInKernel(RBX::Kernel*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x753c00 — __ZN3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD0Ev")]
pub fn stub_0x753c00(handle: crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint dtor.
drop(handle);
}

// 0x753ca0 — __ZN3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint() [0x753ca0]")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD1Ev")]
pub fn stub_0x753ca0(handle: crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint dtor.
drop(handle);
}

// 0x753ca4 — __ZThn32_N3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD0Ev")]
pub fn stub_0x753ca4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x753cac — __ZN3RBX18DynamicRotateJointD2Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint() [0x753cac]")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD2Ev")]
pub fn stub_0x753cac(handle: crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint dtor.
drop(handle);
}

// 0x753dd4 — __ZThn32_N3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint() [0x753dd4]")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD1Ev")]
pub fn stub_0x753dd4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x753ddc — __ZN3RBX18DynamicRotateJoint10setPhysicsEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::setPhysics(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint10setPhysicsEv")]
pub fn stub_0x753ddc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DynamicRotateJoint setter.
cell.set(value)
}

// 0x753dec — __ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::DynamicRotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE")]
pub fn stub_0x753dec(handle: &crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint::putInKernel(RBX::Kernel*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x753f94 — __ZN3RBX18DynamicRotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint16removeFromKernelEv")]
pub fn stub_0x753f94(handle: &crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint::removeFromKernel() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x754060 — __ZN3RBX18DynamicRotateJoint6stepUiEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
#[doc(alias = "RBX::DynamicRotateJoint::stepUi(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint6stepUiEd")]
pub fn stub_0x754060(handle: &crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint::stepUi(double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x75409c — __ZN3RBX18DynamicRotateJoint15getChannelValueEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
#[doc(alias = "RBX::DynamicRotateJoint::getChannelValue(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint15getChannelValueEd")]
pub fn stub_0x75409c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DynamicRotateJoint getter.
cell.get()
}

// 0x754170 — __ZN3RBX12RotatePJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotatePJoint9stepWorldEv")]
pub fn stub_0x754170(handle: &crate::slot::InstanceHandle) {
// RBX::RotatePJoint::stepWorld() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x754184 — __ZN3RBX12RotateVJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotateVJoint9stepWorldEv")]
pub fn stub_0x754184(handle: &crate::slot::InstanceHandle) {
// RBX::RotateVJoint::stepWorld() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x754198 — __ZNK3RBX11RotateJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11RotateJoint12getJointTypeEv")]
pub fn stub_0x754198(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RotateJoint getter.
cell.get()
}

// 0x75419c — __ZNK3RBX18DynamicRotateJoint12canStepWorldEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint12canStepWorldEv")]
pub fn stub_0x75419c(handle: &crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint::canStepWorld() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7541a0 — __ZNK3RBX18DynamicRotateJoint9canStepUiEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint9canStepUiEv")]
pub fn stub_0x7541a0(handle: &crate::slot::InstanceHandle) {
// RBX::DynamicRotateJoint::canStepUi() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x7541a4 — __ZN3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD1Ev")]
pub fn stub_0x7541a4(handle: crate::slot::InstanceHandle) {
// RBX::RotatePJoint dtor.
drop(handle);
}

// 0xa7c2c4 — __ZN6RakNet11StringTableD2Ev
// type: void __fastcall(RakNet::StringTable *__hidden this)
#[doc(alias = "RakNet::StringTable::~StringTable()")]
#[doc(alias = "__ZN6RakNet11StringTableD2Ev")]
pub fn stub_0xa7c2c4() -> crate::slot::PortedFn {
// IDA 0xa7c2c4: RakNet::StringTable::~StringTable().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xa7c2c4, "RakNet::StringTable::~StringTable()")
}

// 0xa7c3dc — __ZN6RakNet11StringTable12AddReferenceEv
// type: int *__fastcall(RakNet::StringTable *this)
#[doc(alias = "RakNet::StringTable::AddReference(void)")]
#[doc(alias = "__ZN6RakNet11StringTable12AddReferenceEv")]
pub fn stub_0xa7c3dc() -> crate::slot::PortedFn {
// IDA 0xa7c3dc: RakNet::StringTable::AddReference().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xa7c3dc, "RakNet::StringTable::AddReference()")
}

// 0xa7c414 — __ZN6RakNet11StringTable15RemoveReferenceEv
// type: void __fastcall(RakNet::StringTable *this)
#[doc(alias = "RakNet::StringTable::RemoveReference(void)")]
#[doc(alias = "__ZN6RakNet11StringTable15RemoveReferenceEv")]
pub fn stub_0xa7c414() -> crate::slot::PortedFn {
// IDA 0xa7c414: RakNet::StringTable::RemoveReference().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xa7c414, "RakNet::StringTable::RemoveReference()")
}
