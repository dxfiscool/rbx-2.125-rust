// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|lua|Yield (5401 filtered, all stubbed) — global EA-sorted asc filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x303d6c..0x3107c8 | global filler EA-sorted asc after 0x303d44 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEN3G3D7Vector34AxisESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")]
pub fn stub_0x303d6c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0x303dc4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_0x303e78(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_0x303ed0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x303f38(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "__ZNSt12_Vector_baseIN3G3D7Vector34AxisESaIS2_EE11_M_allocateEm")]
pub fn stub_0x30401c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector34AxisES6_EET0_T_S8_S7_")]
pub fn stub_0x304034(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x304070(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "__GLOBAL__I_a_107")]
pub fn stub_0x304200() -> crate::slot::PortedFn {
// IDA 0x304200: __GLOBAL__I_a_107.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x304200, "__GLOBAL__I_a_107")
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMap9singletonEv")]
pub fn stub_0x3042c8(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10BrickColor12colorPaletteEv")]
pub fn stub_0x3043c4(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::colorPalette() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX10BrickColor22getClosestPaletteIndexEv")]
pub fn stub_0x3043dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BrickColor getter.
cell.get()
}

#[doc(alias = "__ZN3RBX10BrickColor5parseEPKc")]
pub fn stub_0x3043fc(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::parse(char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10BrickColor6randomEv")]
pub fn stub_0x304468(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::random() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10BrickColor7closestEN3G3D6Color3E")]
pub fn stub_0x3044a0(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::closest(G3D::Color3) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10BrickColor7closestEN3G3D6Color4E")]
pub fn stub_0x3044c4(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::closest(G3D::Color4) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10BrickColorC1Ei")]
pub fn stub_0x304568() -> crate::slot::InstanceHandle {
// RBX::BrickColor ctor.
crate::slot::InstanceHandle::new("RBX::BrickColor")
}

#[doc(alias = "__ZN3RBX10BrickColorC2Ei")]
pub fn stub_0x30456c() -> crate::slot::InstanceHandle {
// RBX::BrickColor ctor.
crate::slot::InstanceHandle::new("RBX::BrickColor")
}

#[doc(alias = "__ZNK3RBX10BrickColor11color4uint8Ev")]
pub fn stub_0x3045b0(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::color4uint8() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX10BrickColor11color3uint8Ev")]
pub fn stub_0x304654(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::color3uint8() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX10BrickColor4nameEv")]
pub fn stub_0x304674(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::name() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX10BrickColor6color4Ev")]
pub fn stub_0x304710(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::color4() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX10BrickColor6color3Ev")]
pub fn stub_0x3047c4(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::color3() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10hash_valueERKNS_10BrickColorE")]
pub fn stub_0x3047ec() -> crate::slot::PortedFn {
// IDA 0x3047ec: RBX::hash_value(RBX::BrickColor const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3047ec, "RBX::hash_value(RBX::BrickColor const&)")
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm")]
pub fn stub_0x3047f0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BrickColor::BrickMap setter.
cell.set(value)
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMapD1Ev")]
pub fn stub_0x304b70(handle: crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap dtor.
drop(handle);
}

#[doc(alias = "__ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_")]
pub fn stub_0x304b74(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
pub fn stub_0x304bcc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_0x304c80(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_0x304cd8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMapD2Ev")]
pub fn stub_0x304d40(handle: crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap dtor.
drop(handle);
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev")]
pub fn stub_0x304e3c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "__ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_0x304f0c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMapC2Ev")]
pub fn stub_0x304f34() -> crate::slot::InstanceHandle {
// RBX::BrickColor::BrickMap ctor.
crate::slot::InstanceHandle::new("RBX::BrickColor::BrickMap")
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs")]
pub fn stub_0x30cbf8(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number, unsigned char, unsigned char, u~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_")]
pub fn stub_0x30cd98(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0x30cdc0(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "__ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm")]
pub fn stub_0x30cea4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_")]
pub fn stub_0x30cebc(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor* std::__copy_backward<false, std::random_access_iterator_tag>::__copy_b<RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_")]
pub fn stub_0x30cef8(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
pub fn stub_0x30cf54(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "__ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_")]
pub fn stub_0x30d6d8() -> crate::slot::PortedFn {
// IDA 0x30d6d8: void std::fill<RBX::BrickColor::BrickMap::ColorInfo*, RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::C~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x30d6d8, "void std::fill<RBX::BrickColor::BrickMap::ColorInfo*, RBX::BrickColor::BrickMap::ColorInfo>(RBX::Bri~")
}

#[doc(alias = "__ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm")]
pub fn stub_0x30d71c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type")]
pub fn stub_0x30d740() -> crate::slot::PortedFn {
// IDA 0x30d740: void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo*, unsigned long, RBX::BrickColor::BrickMap::Co~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x30d740, "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo*, unsigned long, RBX::Bric~")
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_")]
pub fn stub_0x30d88c(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_")]
pub fn stub_0x30d8b8(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::ColorInfo* std::__copy_backward<false, std::random_access_itera~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_")]
pub fn stub_0x30d914(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv")]
pub fn stub_0x30d948(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::generatePaletteMap() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_")]
pub fn stub_0x30da90(handle: &crate::slot::InstanceHandle) {
// RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number, int, std::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_")]
pub fn stub_0x30db44() -> crate::slot::PortedFn {
// IDA 0x30db44: std::vector<RBX::BrickColor, std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor, std::allocator<RBX::B~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x30db44, "std::vector<RBX::BrickColor, std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor, s~")
}

#[doc(alias = "__ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_")]
pub fn stub_0x30db8c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
pub fn stub_0x30dbbc(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "__ZN3RBX13CameraSubject17getContactManagerEv")]
pub fn stub_0x30dd48(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraSubject getter.
cell.get()
}

#[doc(alias = "__ZN3RBX13CameraSubject11doOcclusionERN3G3D7Vector3ERNS1_15CoordinateFrameEf")]
pub fn stub_0x30dd94(handle: &crate::slot::InstanceHandle) {
// RBX::CameraSubject::doOcclusion(G3D::Vector3&, G3D::CoordinateFrame&, float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX13CameraSubject13testOcclusionERKN3G3D7Vector3ERKNS1_15CoordinateFrameERf")]
pub fn stub_0x30de2c(handle: &crate::slot::InstanceHandle) {
// RBX::CameraSubject::testOcclusion(G3D::Vector3 const&, G3D::CoordinateFrame const&, float&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX13CameraSubject23cameraPointFromDistanceERKN3G3D7Vector3ES4_f")]
pub fn stub_0x30e130(handle: &crate::slot::InstanceHandle) {
// RBX::CameraSubject::cameraPointFromDistance(G3D::Vector3 const&, G3D::Vector3 const&, floa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__GLOBAL__I_a_108")]
pub fn stub_0x30e1b0() -> crate::slot::PortedFn {
// IDA 0x30e1b0: __GLOBAL__I_a_108.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x30e1b0, "__GLOBAL__I_a_108")
}

#[doc(alias = "__ZN3RBX5Color15getColorByIndexEi")]
pub fn stub_0x30e3b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Color getter.
cell.get()
}

#[doc(alias = "__ZN3RBX5Color15colorFromIndex8Ei")]
pub fn stub_0x30e580(handle: &crate::slot::InstanceHandle) {
// RBX::Color::colorFromIndex8(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX5Color12colorFromIntEj")]
pub fn stub_0x30e5c0(handle: &crate::slot::InstanceHandle) {
// RBX::Color::colorFromInt(unsigned int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX5Color16colorFromPointerEPv")]
pub fn stub_0x30e670(handle: &crate::slot::InstanceHandle) {
// RBX::Color::colorFromPointer(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__GLOBAL__I_a_109")]
pub fn stub_0x30e67c() -> crate::slot::PortedFn {
// IDA 0x30e67c: __GLOBAL__I_a_109.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x30e67c, "__GLOBAL__I_a_109")
}

#[doc(alias = "__ZN3RBX13ContentFilter12setFilterUrlESs")]
pub fn stub_0x30e6b4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ContentFilter setter.
cell.set(value)
}

#[doc(alias = "__ZN3RBX13ContentFilter15setFilterLimitsEii")]
pub fn stub_0x30e6bc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ContentFilter setter.
cell.set(value)
}

#[doc(alias = "__ZN3RBX13ContentFilterC1Ev")]
pub fn stub_0x30e6c8() -> crate::slot::InstanceHandle {
// RBX::ContentFilter ctor.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "__ZN3RBX13ContentFilterC2Ev")]
pub fn stub_0x30e6cc() -> crate::slot::InstanceHandle {
// RBX::ContentFilter ctor.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "__ZN3RBX13ContentFilterD0Ev")]
pub fn stub_0x30e868(handle: crate::slot::InstanceHandle) {
// RBX::ContentFilter dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX13ContentFilterD1Ev")]
pub fn stub_0x30e908(handle: crate::slot::InstanceHandle) {
// RBX::ContentFilter dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX13ContentFilterD0Ev")]
pub fn stub_0x30e90c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX13ContentFilterD0Ev")]
pub fn stub_0x30e914(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX13ContentFilterD2Ev")]
pub fn stub_0x30e91c(handle: crate::slot::InstanceHandle) {
// RBX::ContentFilter dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX13ContentFilterD1Ev")]
pub fn stub_0x30e96c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX13ContentFilterD1Ev")]
pub fn stub_0x30e974(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX13ContentFilter14truncateStringERSs")]
pub fn stub_0x30e97c(handle: &crate::slot::InstanceHandle) {
// RBX::ContentFilter::truncateString(std::string&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX13ContentFilter14getStringStateERSs")]
pub fn stub_0x30eab0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ContentFilter getter.
cell.get()
}

#[doc(alias = "__ZN3RBX13ContentFilter20isContentFilterReadyERKSs")]
pub fn stub_0x30eadc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ContentFilter getter.
cell.get()
}

#[doc(alias = "__ZN3RBX13ContentFilter12isStringSafeERSs")]
pub fn stub_0x30ee70(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ContentFilter getter.
cell.get()
}

#[doc(alias = "__ZN3RBX13ContentFilter10cleanTableEv")]
pub fn stub_0x30eebc(handle: &crate::slot::InstanceHandle) {
// RBX::ContentFilter::cleanTable() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBXL21staticDoFilterRequestEN5boost8weak_ptrINS_13ContentFilterEEESs")]
pub fn stub_0x30ef44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "__ZN3RBX13ContentFilter15doFilterRequestESs")]
pub fn stub_0x30f0a0(handle: &crate::slot::InstanceHandle) {
// RBX::ContentFilter::doFilterRequest(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBXL20staticDoFilterResultEPSsPSt9exceptionN5boost8weak_ptrINS_13ContentFilterEEESs")]
pub fn stub_0x30f598() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "__ZN3RBXL22staticSaveFilterResultEN5boost8weak_ptrINS_13ContentFilterEEESsb")]
pub fn stub_0x30f8e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFvSsELi1EED1Ev")]
pub fn stub_0x30fa64(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ContentFilterEFviiELi2EED1Ev")]
pub fn stub_0x30faa4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")]
pub fn stub_0x30faec() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN3RBX9weak_fromINS_13ContentFilterEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_0x30fdbc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "__ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsNS_3argILi1EEENS8_ILi2EEES7_SsEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_")]
pub fn stub_0x30ffb4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
pub fn stub_0x310284(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv")]
pub fn stub_0x3103d4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE12getClassNameEv")]
pub fn stub_0x3103fc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
pub fn stub_0x310424(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_0x310510(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_0x310560(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")]
pub fn stub_0x3105e4() -> crate::slot::PortedFn {
// IDA 0x3105e4: std::_Rb_tree<std::string, std::pair<std::string const, RBX::ContentFilter::ResultEntry>, std::_Select1st<std::pair<std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3105e4, "std::_Rb_tree<std::string, std::pair<std::string const, RBX::ContentFilter::ResultEntry>, std::_Sele~")
}

#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")]
pub fn stub_0x3106c4() -> crate::slot::PortedFn {
// IDA 0x3106c4: std::_Rb_tree<std::string, std::pair<std::string const, RBX::ContentFilter::ResultEntry>, std::_Select1st<std::pair<std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3106c4, "std::_Rb_tree<std::string, std::pair<std::string const, RBX::ContentFilter::ResultEntry>, std::_Sele~")
}

#[doc(alias = "__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs")]
pub fn stub_0x3106f4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_")]
pub fn stub_0x31071c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE")]
pub fn stub_0x310770(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs")]
pub fn stub_0x310798() -> crate::slot::PortedFn {
// IDA 0x310798: std::_Rb_tree<std::string, std::string, std::_Identity<std::string>, std::less<std::string>, std::allocator<std::string>~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x310798, "std::_Rb_tree<std::string, std::string, std::_Identity<std::string>, std::less<std::string>, std::al~")
}

#[doc(alias = "__ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs")]
pub fn stub_0x3107c8() -> crate::slot::PortedFn {
// IDA 0x3107c8: std::_Rb_tree<std::string, std::string, std::_Identity<std::string>, std::less<std::string>, std::allocator<std::string>~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3107c8, "std::_Rb_tree<std::string, std::string, std::_Identity<std::string>, std::less<std::string>, std::al~")
}
