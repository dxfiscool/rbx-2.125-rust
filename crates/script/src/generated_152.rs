// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|lua|Yield (5401 filtered, all stubbed) — global EA-sorted asc filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x32366c..0x356878 | global filler EA-sorted asc after 0x3234d4 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::HandleHitTest::hitTestHandleWorld(RBX::Extents const&,RBX::HandleType,RBX::RbxRay const&,G3D::Vector3 &,RBX::NormalId &,int)")]
pub fn stub_0x32366c(handle: &crate::slot::InstanceHandle) {
// RBX::HandleHitTest::hitTestHandleWorld(RBX::Extents const&, RBX::HandleType, RBX::RbxRay c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::HandleHitTest::hitTestHandleLocal(RBX::Extents const&,G3D::CoordinateFrame const&,RBX::HandleType,RBX::RbxRay const&,G3D::Vector3 &,RBX::NormalId &,int)")]
pub fn stub_0x323768(handle: crate::slot::InstanceHandle) {
// RBX::HandleHitTest dtor.
drop(handle);
}

#[doc(alias = "RBX::HandleHitTest::hitTestMoveHandleWorld(RBX::Extents const&,RBX::RbxRay const&,G3D::Vector3 &,RBX::NormalId &,int)")]
pub fn stub_0x3238e8(handle: &crate::slot::InstanceHandle) {
// RBX::HandleHitTest::hitTestMoveHandleWorld(RBX::Extents const&, RBX::RbxRay const&, G3D::V~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_120")]
pub fn stub_0x323b78() -> crate::slot::PortedFn {
// IDA 0x323b78: __GLOBAL__I_a_120.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x323b78, "__GLOBAL__I_a_120")
}

#[doc(alias = "RBX::IndexBox::IndexBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub fn stub_0x323de8() -> crate::slot::InstanceHandle {
// RBX::IndexBox ctor.
crate::slot::InstanceHandle::new("RBX::IndexBox")
}

#[doc(alias = "RBX::IndexBox::IndexBox(G3D::Vector3 const&,G3D::Vector3 const&) [0x323dec]")]
pub fn stub_0x323dec() -> crate::slot::InstanceHandle {
// RBX::IndexBox ctor.
crate::slot::InstanceHandle::new("RBX::IndexBox")
}

#[doc(alias = "RBX::IndexBox::getFaceCorners(int,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
pub fn stub_0x323efc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::IndexBox getter.
cell.get()
}

#[doc(alias = "RBX::IndexBox::~IndexBox()")]
pub fn stub_0x323fd8(handle: crate::slot::InstanceHandle) {
// RBX::IndexBox dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexBox::~IndexBox() [0x323fdc]")]
pub fn stub_0x323fdc(handle: crate::slot::InstanceHandle) {
// RBX::IndexBox dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_121")]
pub fn stub_0x323fe0() -> crate::slot::PortedFn {
// IDA 0x323fe0: __GLOBAL__I_a_121.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x323fe0, "__GLOBAL__I_a_121")
}

#[doc(alias = "RBX::IndexedMesh::IndexedMesh(void)")]
pub fn stub_0x324018() -> crate::slot::InstanceHandle {
// RBX::IndexedMesh ctor.
crate::slot::InstanceHandle::new("RBX::IndexedMesh")
}

#[doc(alias = "RBX::IndexedMesh::~IndexedMesh()")]
pub fn stub_0x32403c(handle: crate::slot::InstanceHandle) {
// RBX::IndexedMesh dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexedMesh::~IndexedMesh() [0x3240dc]")]
pub fn stub_0x3240dc(handle: crate::slot::InstanceHandle) {
// RBX::IndexedMesh dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexedMesh::~IndexedMesh() [0x3240e0]")]
pub fn stub_0x3240e0(handle: crate::slot::InstanceHandle) {
// RBX::IndexedMesh dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexedMesh::setComputedUpper(RBX::IndexedMesh*)")]
pub fn stub_0x324354(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::IndexedMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::IndexedMesh::setUpper(RBX::IndexedMesh*)")]
pub fn stub_0x32438c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::IndexedMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::IndexedMesh::setLower(RBX::IndexedMesh*)")]
pub fn stub_0x3243d4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::IndexedMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::IndexedMesh::getComputedUpper(void)")]
pub fn stub_0x3244b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::IndexedMesh getter.
cell.get()
}

#[doc(alias = "RBX::IndexedMesh::getIndexedMeshParent(void)")]
pub fn stub_0x3244bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::IndexedMesh getter.
cell.get()
}

#[doc(alias = "RBX::IndexedMesh::attachChildren(RBX::IndexedMesh*)")]
pub fn stub_0x3244c0(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedMesh::attachChildren(RBX::IndexedMesh*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::IndexedMesh::onParentChanged(RBX::IndexedTree *)")]
pub fn stub_0x3244fc(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedMesh::onParentChanged(RBX::IndexedTree*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::IndexedMesh::severeChildren(RBX::IndexedMesh*)")]
pub fn stub_0x324550(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedMesh::severeChildren(RBX::IndexedMesh*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::IndexedMesh::getConstIndexedMeshParent(void)const")]
pub fn stub_0x32458c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::IndexedMesh getter.
cell.get()
}

#[doc(alias = "RBX::IndexedMesh::getConstComputedUpper(void)const")]
pub fn stub_0x3245f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::IndexedMesh getter.
cell.get()
}

#[doc(alias = "RBX::IndexedMesh::isUpperRoot(RBX::IndexedMesh const*)")]
pub fn stub_0x3245fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::IndexedMesh getter.
cell.get()
}

#[doc(alias = "RBX::IndexedMesh * RBX::IndexedTree::getTypedChild<RBX::IndexedMesh>(int)")]
pub fn stub_0x324684(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedMesh* RBX::IndexedTree::getTypedChild<RBX::IndexedMesh>(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::IndexedMesh::lowersChanged(void)")]
pub fn stub_0x3246ec(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedMesh::lowersChanged() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_122")]
pub fn stub_0x324710() -> crate::slot::PortedFn {
// IDA 0x324710: __GLOBAL__I_a_122.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x324710, "__GLOBAL__I_a_122")
}

#[doc(alias = "RBX::IndexedTree::IndexedTree(void)")]
pub fn stub_0x3247d8() -> crate::slot::InstanceHandle {
// RBX::IndexedTree ctor.
crate::slot::InstanceHandle::new("RBX::IndexedTree")
}

#[doc(alias = "RBX::IndexedTree::~IndexedTree()")]
pub fn stub_0x324800(handle: crate::slot::InstanceHandle) {
// RBX::IndexedTree dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexedTree::~IndexedTree() [0x3248a0]")]
pub fn stub_0x3248a0(handle: crate::slot::InstanceHandle) {
// RBX::IndexedTree dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexedTree::~IndexedTree() [0x3248a4]")]
pub fn stub_0x3248a4(handle: crate::slot::InstanceHandle) {
// RBX::IndexedTree dtor.
drop(handle);
}

#[doc(alias = "RBX::IndexedTree::setIndexedTreeParent(RBX::IndexedTree*)")]
pub fn stub_0x324a74(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::IndexedTree setter.
cell.set(value)
}

#[doc(alias = "RBX::IndexArray<RBX::IndexedTree,&RBX::IndexedTree::getIndex>::fastRemove(RBX::IndexedTree*)")]
pub fn stub_0x324c14(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "RBX::IndexedTree::onParentChanged(RBX::IndexedTree*)")]
pub fn stub_0x324ce8(handle: &crate::slot::InstanceHandle) {
// RBX::IndexedTree::onParentChanged(RBX::IndexedTree*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::append(RBX::IndexedTree * const&)")]
pub fn stub_0x324cec(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::resize(int,bool)")]
pub fn stub_0x324d48(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::realloc(int)")]
pub fn stub_0x324e00(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::~Array()")]
pub fn stub_0x324fe8(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "G3D::Array<RBX::IndexedTree *,10,32ul>::Array(void)")]
pub fn stub_0x3250bc() -> crate::slot::PortedFn {
// IDA 0x3250bc: G3D::Array<RBX::IndexedTree*, 10, 32ul>::Array().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3250bc, "G3D::Array<RBX::IndexedTree*, 10, 32ul>::Array()")
}

#[doc(alias = "global constructor keyed to_a_123")]
pub fn stub_0x3251ac() -> crate::slot::PortedFn {
// IDA 0x3251ac: __GLOBAL__I_a_123.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3251ac, "__GLOBAL__I_a_123")
}

#[doc(alias = "RBX::InterpolatedCFrame::clearHistory(void)")]
pub fn stub_0x325278(handle: &crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame::clearHistory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::InterpolatedCFrame::setValue(RBX::PartInstance *,G3D::CoordinateFrame const&,RBX::RemoteTime const&)")]
pub fn stub_0x3252f8(handle: crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame dtor.
drop(handle);
}

#[doc(alias = "RBX::InterpolatedCFrame::interpolate(RBX::Time const&,RBX::Time const&,unsigned int const&)")]
pub fn stub_0x325538(handle: &crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame::interpolate(RBX::Time const&, RBX::Time const&, unsigned int cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::InterpolatedCFrame::computeSampleTargetTime(RBX::Time const&)")]
pub fn stub_0x3258e4(handle: &crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame::computeSampleTargetTime(RBX::Time const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::InterpolatedCFrame::computeValue(RBX::PartInstance *)")]
pub fn stub_0x325998(handle: &crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame::computeValue(RBX::PartInstance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::InterpolatedCFrame::setRenderedFrame(G3D::CoordinateFrame const&)")]
pub fn stub_0x325b08(handle: crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame dtor.
drop(handle);
}

#[doc(alias = "RBX::InterpolatedCFrame::setRenderedFrame(G3D::CoordinateFrame const&,RBX::RemoteTime const&)")]
pub fn stub_0x325b4c(handle: crate::slot::InstanceHandle) {
// RBX::InterpolatedCFrame dtor.
drop(handle);
}

#[doc(alias = "RBX::InterpolatedCFrame::getSampleInterval(void)const")]
pub fn stub_0x325b98(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::InterpolatedCFrame getter.
cell.get()
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::push_back(RBX::InterpolatedCFrame::FrameInfo const&)")]
pub fn stub_0x325c38() -> crate::slot::PortedFn {
// IDA 0x325c38: boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo, std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::push_bac~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x325c38, "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo, std::allocator<RBX::InterpolatedCFrame::F~")
}

#[doc(alias = "global constructor keyed to_a_124")]
pub fn stub_0x326108() -> crate::slot::PortedFn {
// IDA 0x326108: __GLOBAL__I_a_124.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x326108, "__GLOBAL__I_a_124")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void)")]
pub fn stub_0x326378() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::EnumDesc(void) [0x32637c]")]
pub fn stub_0x32637c() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::addPair(RBX::KeywordFilterType,char const*)")]
pub fn stub_0x32653c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::addPair(RBX::KeywordFilterType, char co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::resize(unsigned long,RBX::KeywordFilterType)")]
pub fn stub_0x32689c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::push_back(RBX::KeywordFilterType const&)")]
pub fn stub_0x3268d0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::KeywordFilterType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x3268f8(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
pub fn stub_0x326950(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
pub fn stub_0x326a04(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
pub fn stub_0x326a5c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,RBX::KeywordFilterType const&)")]
pub fn stub_0x326ac4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_allocate(unsigned long)")]
pub fn stub_0x326ba8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::KeywordFilterType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeywordFilterType *,RBX::KeywordFilterType *>(RBX::KeywordFilterType *,RBX::KeywordFilterType *,RBX::KeywordFilterType *)")]
pub fn stub_0x326bc0(handle: &crate::slot::InstanceHandle) {
// RBX::KeywordFilterType* std::__copy_backward<false, std::random_access_iterator_tag>::__co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,unsigned long,RBX::KeywordFilterType const&)")]
pub fn stub_0x326bfc(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "global constructor keyed to_a_125")]
pub fn stub_0x326d8c() -> crate::slot::PortedFn {
// IDA 0x326d8c: __GLOBAL__I_a_125.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x326d8c, "__GLOBAL__I_a_125")
}

#[doc(alias = "RBX::LegacyContentTable::LegacyContentTable(void)")]
pub fn stub_0x326e54() -> crate::slot::InstanceHandle {
// RBX::LegacyContentTable ctor.
crate::slot::InstanceHandle::new("RBX::LegacyContentTable")
}

#[doc(alias = "RBX::LegacyContentTable::LegacyContentTable(void) [0x326e58]")]
pub fn stub_0x326e58() -> crate::slot::InstanceHandle {
// RBX::LegacyContentTable ctor.
crate::slot::InstanceHandle::new("RBX::LegacyContentTable")
}

#[doc(alias = "sub_3378C6")]
pub fn stub_0x3378c6() -> crate::slot::PortedFn {
// IDA 0x3378c6: sub_3378C6.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3378c6, "sub_3378C6")
}

#[doc(alias = "RBX::LegacyContentTable::AddEntry(std::string const&,std::string const&)")]
pub fn stub_0x34581c(handle: &crate::slot::InstanceHandle) {
// RBX::LegacyContentTable::AddEntry(std::string const&, std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "anonymous namespace::normalizeUrl(std::string &)")]
pub fn stub_0x345950() -> crate::slot::PortedFn {
// IDA 0x345950: (anonymous namespace)::normalizeUrl(std::string&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x345950, "(anonymous namespace)::normalizeUrl(std::string&)")
}

#[doc(alias = "RBX::LegacyContentTable::FindEntry(std::string const&)")]
pub fn stub_0x3459d4(handle: &crate::slot::InstanceHandle) {
// RBX::LegacyContentTable::FindEntry(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x345b48(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> const&)")]
pub fn stub_0x345bb4() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
pub fn stub_0x345c20() -> crate::slot::PortedFn {
// IDA 0x345c20: RBX::findLocalFile(std::string const&, std::string*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x345c20, "RBX::findLocalFile(std::string const&, std::string*)")
}

#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list_av_1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::type> boost::bind<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_0x347fc8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)const")]
pub fn stub_0x34f17c(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x34f28c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "__ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS4_5list1INS4_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x350670() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3507b4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>)")]
pub fn stub_0x3508fc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x350a54(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
pub fn stub_0x350a70(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x350a8c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x350bd4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x350d18(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::operator()<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> &,boost::_bi::list1<std::string &> &,int)")]
pub fn stub_0x350e0c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x350e0c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x350edc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&)")]
pub fn stub_0x351078(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::list1(boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)")]
pub fn stub_0x3510a8() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>::bind_t(boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>> const&)")]
pub fn stub_0x351188() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)const")]
pub fn stub_0x351f58(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> rbx::any_cast<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x35206c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "Weak<RBX::AsyncHttpQueue>::expired(void)const")]
pub fn stub_0x352dc8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "boost::detail::function::functor_manager<bool (*)(std::string const&,std::string *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x356328(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::function1<void,bool>::clear(void)")]
pub fn stub_0x356588(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)")]
pub fn stub_0x3565b8(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "global constructor keyed to_a_126")]
pub fn stub_0x3565e4() -> crate::slot::PortedFn {
// IDA 0x3565e4: __GLOBAL__I_a_126.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3565e4, "__GLOBAL__I_a_126")
}

#[doc(alias = "RBX::Math::sumDeltaAxis(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
pub fn stub_0x35677c(handle: &crate::slot::InstanceHandle) {
// RBX::Math::sumDeltaAxis(G3D::Matrix3 const&, G3D::Matrix3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Math::mulMatrixDiagVector(G3D::Matrix3 const&,G3D::Vector3 const&,G3D::Matrix3&)")]
pub fn stub_0x3567e0(handle: &crate::slot::InstanceHandle) {
// RBX::Math::mulMatrixDiagVector(G3D::Matrix3 const&, G3D::Vector3 const&, G3D::Matrix3&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Math::mulMatrixMatrixTranspose(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")]
pub fn stub_0x356878(handle: &crate::slot::InstanceHandle) {
// RBX::Math::mulMatrixMatrixTranspose(G3D::Matrix3 const&, G3D::Matrix3 const&, G3D::Matrix3~ — engine-side; linkage preserved via the alias.
let _ = handle;
}
