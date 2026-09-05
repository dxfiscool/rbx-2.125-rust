// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x475bd4..0x479d28 | script 26072->26172 distinct (filler 0x475bd4 asc, not-in-script 59473->59373)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x475bd4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x475bd8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x475c78(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x475c80(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x475d24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x475d2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x475dd0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::~PropDescriptor() [0x475ee4]")]
pub fn stub_0x475ee4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_0x475f10(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_0x475f14(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x475f18(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
pub fn stub_0x475f4c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::EnumPropDescriptor<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>(char const*,char const*,RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x475f70(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::~EnumPropDescriptor() [0x476124]")]
pub fn stub_0x476124(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::isReadOnly(void)const")]
pub fn stub_0x476150(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::isWriteOnly(void)const")]
pub fn stub_0x476160(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x476170(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x476198(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4761bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x476308(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::hasStringValue(void)const")]
pub fn stub_0x47632c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x476330(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x476354(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x476394(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4763b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4765f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x476610(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x476644(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x47664c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x476698(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4766b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::convertToIndex(RBX::DataModelMesh::LODType)const")]
pub fn stub_0x4766ec(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DataModelMesh::LODType>::convertToIndex(RBX::DataModelMesh:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x47675c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::isReadOnly(void)const")]
pub fn stub_0x47679c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::isWriteOnly(void)const")]
pub fn stub_0x4767a0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4767a4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModelMesh,RBX::DataModelMesh::LODType>::GetSetImpl<RBX::DataModelMesh::LODType (RBX::DataModelMesh::*)(void)const,void (RBX::DataModelMesh::*)(RBX::DataModelMesh::LODType)>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModelMesh::LODType const&)const")]
pub fn stub_0x4767c4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::resize(unsigned long,RBX::DataModelMesh::LODType)")]
pub fn stub_0x4767e8(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::push_back(RBX::DataModelMesh::LODType const&)")]
pub fn stub_0x47681c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModelMesh::LODType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x476844(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
pub fn stub_0x47689c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
pub fn stub_0x476950(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModelMesh::LODType> const&)")]
pub fn stub_0x4769a8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModelMesh::LODType*,std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>>,RBX::DataModelMesh::LODType const&)")]
pub fn stub_0x476a10(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_allocate(unsigned long)")]
pub fn stub_0x476af4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DataModelMesh::LODType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *>(RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *,RBX::DataModelMesh::LODType *)")]
pub fn stub_0x476b0c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModelMesh::LODType* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModelMesh::LODType*,std::vector<RBX::DataModelMesh::LODType,std::allocator<RBX::DataModelMesh::LODType>>>,unsigned long,RBX::DataModelMesh::LODType const&)")]
pub fn stub_0x476b48(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::DebrisService::setMaxItems(int)")]
pub fn stub_0x4770dc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebrisService setter.
cell.set(value)
}

#[doc(alias = "RBX::DebrisService::addItem(rbx_core::SharedPtr<RBX::Instance>,double)")]
pub fn stub_0x477264() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::DebrisService::setLegacyMaxItems(bool)")]
pub fn stub_0x477410(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DebrisService setter.
cell.set(value)
}

#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
pub fn stub_0x477418() -> crate::slot::InstanceHandle {
// RBX::DebrisService ctor.
crate::slot::InstanceHandle::new("RBX::DebrisService")
}

#[doc(alias = "RBX::DebrisService::DebrisService(void) [0x47741c]")]
pub fn stub_0x47741c() -> crate::slot::InstanceHandle {
// RBX::DebrisService ctor.
crate::slot::InstanceHandle::new("RBX::DebrisService")
}

#[doc(alias = "RBX::DebrisService::cleanup(void)")]
pub fn stub_0x4775e4(handle: &crate::slot::InstanceHandle) {
// RBX::DebrisService::cleanup() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "cleanup(rbx_core::Weak<RBX::Instance>)")]
pub fn stub_0x477738() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::DebrisService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x477864(handle: &crate::slot::InstanceHandle) {
// RBX::DebrisService::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DebrisService::getMaxItems(void)const")]
pub fn stub_0x477a0c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DebrisService getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::~PropDescriptor()")]
pub fn stub_0x477a14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(rbx_core::SharedPtr<RBX::Instance>,double),2>::~BoundFuncDesc()")]
pub fn stub_0x477a38(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_0x477b50(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list_av_1<rbx_core::Weak<RBX::Instance>>::type> boost::bind<void,rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance>>(void (*)(rbx_core::Weak<RBX::Instance>),rbx_core::Weak<RBX::Instance>)")]
pub fn stub_0x477b90() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "RBX::DebrisService::~DebrisService()")]
pub fn stub_0x477ed8(handle: crate::slot::InstanceHandle) {
// RBX::DebrisService dtor.
drop(handle);
}

#[doc(alias = "RBX::DebrisService::~DebrisService() [0x477fe4]")]
pub fn stub_0x477fe4(handle: crate::slot::InstanceHandle) {
// RBX::DebrisService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
pub fn stub_0x478128(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService() [0x478234]")]
pub fn stub_0x478234(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService() [0x47837c]")]
pub fn stub_0x47837c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService() [0x478484]")]
pub fn stub_0x478484(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::push_back(rbx_core::Weak<RBX::Instance> const&)")]
pub fn stub_0x4785a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_push_back_aux(rbx_core::Weak<RBX::Instance> const&)")]
pub fn stub_0x478630() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x478814() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x478830() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Deque_base<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x478908() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>)")]
pub fn stub_0x478a4c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x478b84(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x478ba0(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x478bb4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x478cd4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>::operator()<void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Instance>) &,boost::_bi::list0 &,int)")]
pub fn stub_0x478e50(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x478e50: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x478f60(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::Instance>>>::list1(boost::_bi::value<rbx_core::Weak<RBX::Instance>>)")]
pub fn stub_0x479038() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::pop_front(void)")]
pub fn stub_0x479180() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_pop_front_aux(void)")]
pub fn stub_0x4791ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::deque(std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>> const&)")]
pub fn stub_0x4791d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Deque_base<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::~_Deque_base()")]
pub fn stub_0x4792fc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance>&,rbx_core::Weak<RBX::Instance>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance> const&,rbx_core::Weak<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance>&,rbx_core::Weak<RBX::Instance>*>>(std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance> const&,rbx_core::Weak<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance> const&,rbx_core::Weak<RBX::Instance> const*>,std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance>&,rbx_core::Weak<RBX::Instance>*>,std::__false_type)")]
pub fn stub_0x479328() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Deque_base<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x479510() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Deque_base<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_create_nodes(rbx_core::Weak<RBX::Instance>**,rbx_core::Weak<RBX::Instance>**)")]
pub fn stub_0x479668() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::~deque()")]
pub fn stub_0x47975c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::deque<rbx_core::Weak<RBX::Instance>,std::allocator<rbx_core::Weak<RBX::Instance>>>::_M_destroy_data_aux(std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance>&,rbx_core::Weak<RBX::Instance>*>,std::_Deque_iterator<rbx_core::Weak<RBX::Instance>,rbx_core::Weak<RBX::Instance>&,rbx_core::Weak<RBX::Instance>*>)")]
pub fn stub_0x479844() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x479984(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x479988(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x479a28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x479a30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x479ad4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebrisServiceELZNS_14sDebrisServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x479adc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::BoundFuncDesc(void (RBX::DebrisService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x479b80() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DebrisService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x479cf8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DebrisService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::~BoundFuncDesc() [0x479d28]")]
pub fn stub_0x479d28(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}
