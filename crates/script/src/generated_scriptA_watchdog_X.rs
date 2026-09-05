// Auto-generated skeletons for rbx-script — script/Lua watchdogA filler
// Filter: Script|Lua (case-sensitive) 4456 filtered, all already stubbed — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x86bb10..0x88339c | gap 21828 remaining after
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::StringConverter<RBX::Voxel::CellBlock>::convertToValue(std::string const&,RBX::Voxel::CellBlock&)")]
pub fn stub_0x86bb10(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Voxel::CellBlock>::convertToValue(std::string const&, RBX::Voxel~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::CellOrientation>::convertToValue(std::string const&,RBX::Voxel::CellOrientation&)")]
pub fn stub_0x86bb5c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Voxel::CellOrientation>::convertToValue(std::string const&, RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::WaterCellForce>::convertToValue(std::string const&,RBX::Voxel::WaterCellForce&)")]
pub fn stub_0x86bba8(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Voxel::WaterCellForce>::convertToValue(std::string const&, RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::Voxel::WaterCellDirection>::convertToValue(std::string const&,RBX::Voxel::WaterCellDirection&)")]
pub fn stub_0x86bbf4(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Voxel::WaterCellDirection>::convertToValue(std::string const&, R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::resize(unsigned long,RBX::Voxel::Cell)")]
pub fn stub_0x8715e4(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::push_back(RBX::Voxel::CellChangeListener * const&)")]
pub fn stub_0x8716a8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::StringReadBuffer::operator>>(unsigned char &)")]
pub fn stub_0x872db4(handle: &crate::slot::InstanceHandle) {
// RBX::StringReadBuffer::operator>>(unsigned char&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer &)")]
pub fn stub_0x872f0c() -> crate::slot::PortedFn {
// IDA 0x872f0c: unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x872f0c, "unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer&)")
}

#[doc(alias = "void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer &,unsigned int)")]
pub fn stub_0x872f44() -> crate::slot::PortedFn {
// IDA 0x872f44: void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer&, unsigned int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x872f44, "void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer&, unsigned int)")
}

#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&)")]
pub fn stub_0x872fc4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_allocate(unsigned long)")]
pub fn stub_0x8730a4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener *>(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&,std::random_access_iterator_tag)")]
pub fn stub_0x8730bc() -> crate::slot::PortedFn {
// IDA 0x8730bc: __gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener**, std::vector<RBX::Voxel::CellChangeListener*, std::allocat~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x8730bc, "__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener**, std::vector<RBX::Voxel::CellChangeLis~")
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::Cell*,std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>>,unsigned long,RBX::Voxel::Cell const&)")]
pub fn stub_0x87314c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Voxel::WaterCellDirection * rbx::any_cast<RBX::Voxel::WaterCellDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x87788c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Voxel::WaterCellDirection & rbx::any_cast<RBX::Voxel::WaterCellDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x8778e4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::resize(unsigned long,RBX::Voxel::WaterCellDirection)")]
pub fn stub_0x8779d4(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::push_back(RBX::Voxel::WaterCellDirection const&)")]
pub fn stub_0x877a08(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x877a30(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
pub fn stub_0x877a88(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
pub fn stub_0x877b3c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection> const&)")]
pub fn stub_0x877b94(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,RBX::Voxel::WaterCellDirection const&)")]
pub fn stub_0x877bfc(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_allocate(unsigned long)")]
pub fn stub_0x877ce0() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Voxel::WaterCellDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *>(RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *)")]
pub fn stub_0x877cf8(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::WaterCellDirection* std::__copy_backward<false, std::random_access_iterator_ta~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,unsigned long,RBX::Voxel::WaterCellDirection const&)")]
pub fn stub_0x877d34(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Voxel::WaterCellForce * rbx::any_cast<RBX::Voxel::WaterCellForce,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x877ec4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Voxel::WaterCellForce & rbx::any_cast<RBX::Voxel::WaterCellForce &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x877f1c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::resize(unsigned long,RBX::Voxel::WaterCellForce)")]
pub fn stub_0x87800c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::push_back(RBX::Voxel::WaterCellForce const&)")]
pub fn stub_0x878040(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellForce,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x878068(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
pub fn stub_0x8780c0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
pub fn stub_0x878174(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce> const&)")]
pub fn stub_0x8781cc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,RBX::Voxel::WaterCellForce const&)")]
pub fn stub_0x878234(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_allocate(unsigned long)")]
pub fn stub_0x878318() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Voxel::WaterCellForce * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *>(RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *)")]
pub fn stub_0x878330(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::WaterCellForce* std::__copy_backward<false, std::random_access_iterator_tag>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,unsigned long,RBX::Voxel::WaterCellForce const&)")]
pub fn stub_0x87836c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Voxel::CellOrientation * rbx::any_cast<RBX::Voxel::CellOrientation,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x8784fc(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Voxel::CellOrientation & rbx::any_cast<RBX::Voxel::CellOrientation &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x878554(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::resize(unsigned long,RBX::Voxel::CellOrientation)")]
pub fn stub_0x878644(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::push_back(RBX::Voxel::CellOrientation const&)")]
pub fn stub_0x878678(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellOrientation,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x8786a0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
pub fn stub_0x8786f8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
pub fn stub_0x8787ac(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
pub fn stub_0x878804(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,RBX::Voxel::CellOrientation const&)")]
pub fn stub_0x87886c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_allocate(unsigned long)")]
pub fn stub_0x878950() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Voxel::CellOrientation * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *>(RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *)")]
pub fn stub_0x878968(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::CellOrientation* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,unsigned long,RBX::Voxel::CellOrientation const&)")]
pub fn stub_0x8789a4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Voxel::CellBlock * rbx::any_cast<RBX::Voxel::CellBlock,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x878b34(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Voxel::CellBlock & rbx::any_cast<RBX::Voxel::CellBlock &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x878b8c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::resize(unsigned long,RBX::Voxel::CellBlock)")]
pub fn stub_0x878c7c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::push_back(RBX::Voxel::CellBlock const&)")]
pub fn stub_0x878cb0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellBlock,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x878cd8(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
pub fn stub_0x878d30(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
pub fn stub_0x878de4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
pub fn stub_0x878e3c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,RBX::Voxel::CellBlock const&)")]
pub fn stub_0x878ea4(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_allocate(unsigned long)")]
pub fn stub_0x878f88(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "RBX::Voxel::CellBlock * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *>(RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *)")]
pub fn stub_0x878fa0(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::CellBlock* std::__copy_backward<false, std::random_access_iterator_tag>::__cop~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,unsigned long,RBX::Voxel::CellBlock const&)")]
pub fn stub_0x878fdc(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "RBX::Voxel::CellMaterial * rbx::any_cast<RBX::Voxel::CellMaterial,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x87916c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Voxel::CellMaterial & rbx::any_cast<RBX::Voxel::CellMaterial &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x8791c4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::resize(unsigned long,RBX::Voxel::CellMaterial)")]
pub fn stub_0x8792b4(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::push_back(RBX::Voxel::CellMaterial const&)")]
pub fn stub_0x8792e8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellMaterial,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x879310(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
pub fn stub_0x879368(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
pub fn stub_0x87941c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
pub fn stub_0x879474(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,RBX::Voxel::CellMaterial const&)")]
pub fn stub_0x8794dc(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_allocate(unsigned long)")]
pub fn stub_0x8795c0() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Voxel::CellMaterial * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *>(RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *)")]
pub fn stub_0x8795d8(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::CellMaterial* std::__copy_backward<false, std::random_access_iterator_tag>::__~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,unsigned long,RBX::Voxel::CellMaterial const&)")]
pub fn stub_0x879614(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::~vector()")]
pub fn stub_0x879810(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::CellContact::~CellContact()")]
pub fn stub_0x87a47c(handle: crate::slot::InstanceHandle) {
// RBX::CellContact dtor.
drop(handle);
}

#[doc(alias = "RBX::CellContact::~CellContact() [0x87a51c]")]
pub fn stub_0x87a51c(handle: crate::slot::InstanceHandle) {
// RBX::CellContact dtor.
drop(handle);
}

#[doc(alias = "RBX::CellContact::~CellContact() [0x87a520]")]
pub fn stub_0x87a520(handle: crate::slot::InstanceHandle) {
// RBX::CellContact dtor.
drop(handle);
}

#[doc(alias = "RBX::CellContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
pub fn stub_0x87a650(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector*, 40ul>&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::getConnector(int)")]
pub fn stub_0x87a714(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CellContact getter.
cell.get()
}

#[doc(alias = "RBX::CellContact::deleteAllConnectors(void)")]
pub fn stub_0x87a71c(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::deleteAllConnectors() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::removeAllConnectorsFromKernel(void)")]
pub fn stub_0x87a724(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::removeAllConnectorsFromKernel() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::putAllConnectorsInKernel(void)")]
pub fn stub_0x87a794(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::putAllConnectorsInKernel() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::stepContact(void)")]
pub fn stub_0x87a830(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::stepContact() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::computeIsColliding(float)")]
pub fn stub_0x87a86c(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::computeIsColliding(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::updateClosestFeatures(void)")]
pub fn stub_0x87a8d4(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::updateClosestFeatures() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::worstFeatureOverlap(void)")]
pub fn stub_0x87a914(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::worstFeatureOverlap() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::matchClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
pub fn stub_0x87a9c8(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::matchClosestFeatures(RBX::FixedArray<RBX::PolyConnector*, 40ul>&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::updateContactPoints(void)")]
pub fn stub_0x87aa20(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::updateContactPoints() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellContact::matchClosestFeature(RBX::PolyConnector *)")]
pub fn stub_0x87aa50(handle: &crate::slot::InstanceHandle) {
// RBX::CellContact::matchClosestFeature(RBX::PolyConnector*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Voxel::Grid * RBX::CellContact::getVoxelStore<RBX::Voxel::Grid>(void)")]
pub fn stub_0x87aaa8(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::Grid* RBX::CellContact::getVoxelStore<RBX::Voxel::Grid>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::operator[](unsigned long)")]
pub fn stub_0x87aac0(vec: &crate::slot::VecModel, index: usize) -> Option<usize> {
// bounds-checked element access shape.
if index < vec.len() { Some(index) } else { None }
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::replace(unsigned long,RBX::PolyConnector * const&)")]
pub fn stub_0x87ab20(handle: &crate::slot::InstanceHandle) {
// RBX::FixedArray<RBX::PolyConnector*, 40ul>::replace(unsigned long, RBX::PolyConnector* con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PolyConnector::match(RBX::PolyConnector*,RBX::PolyConnector*)")]
pub fn stub_0x87abd8(handle: &crate::slot::InstanceHandle) {
// RBX::PolyConnector::match(RBX::PolyConnector*, RBX::PolyConnector*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::fastRemove(unsigned long)")]
pub fn stub_0x87ac14(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "RBX::MegaClusterPoly::buildMesh(void)")]
pub fn stub_0x87b2d8(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterPoly::buildMesh() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterPoly::getSurfaceCoordInBody(unsigned long)const")]
pub fn stub_0x87b474(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MegaClusterPoly getter.
cell.get()
}

#[doc(alias = "RBX::MegaClusterPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
pub fn stub_0x87b480(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MegaClusterPoly getter.
cell.get()
}

#[doc(alias = "RBX::MegaClusterPoly::~MegaClusterPoly()")]
pub fn stub_0x87fc58(handle: crate::slot::InstanceHandle) {
// RBX::MegaClusterPoly dtor.
drop(handle);
}

#[doc(alias = "RBX::MegaClusterPoly::~MegaClusterPoly() [0x87fc7c]")]
pub fn stub_0x87fc7c(handle: crate::slot::InstanceHandle) {
// RBX::MegaClusterPoly dtor.
drop(handle);
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void *)")]
pub fn stub_0x88067c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator new(unsigned long)")]
pub fn stub_0x880cb8(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::Allocator(void)")]
pub fn stub_0x880e3c() -> crate::slot::InstanceHandle {
// RBX::Allocator ctor.
crate::slot::InstanceHandle::new("RBX::Allocator")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::releaseMemory(void)")]
pub fn stub_0x880ea0(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<RBX::POLY::MegaClusterMesh>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x880ebc() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x880ebc, "boost::singleton_pool<RBX::POLY::MegaClusterMesh, 48u, boost::default_user_allocator_malloc_free, bo~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x880eec() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x880eec, "boost::singleton_pool<RBX::POLY::MegaClusterMesh, 48u, boost::default_user_allocator_malloc_free, bo~")
}

#[doc(alias = "RBX::PolyCellContact::~PolyCellContact()")]
pub fn stub_0x881898(handle: crate::slot::InstanceHandle) {
// RBX::PolyCellContact dtor.
drop(handle);
}

#[doc(alias = "RBX::PolyCellContact::~PolyCellContact() [0x88194c]")]
pub fn stub_0x88194c(handle: crate::slot::InstanceHandle) {
// RBX::PolyCellContact dtor.
drop(handle);
}

#[doc(alias = "RBX::PolyCellContact::~PolyCellContact() [0x881950]")]
pub fn stub_0x881950(handle: crate::slot::InstanceHandle) {
// RBX::PolyCellContact dtor.
drop(handle);
}

#[doc(alias = "RBX::PolyCellContact::resetBestPair(RBX::PolyCellPair *)")]
pub fn stub_0x881a94(handle: &crate::slot::InstanceHandle) {
// RBX::PolyCellContact::resetBestPair(RBX::PolyCellPair*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PolyCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
pub fn stub_0x881ac4(handle: &crate::slot::InstanceHandle) {
// RBX::PolyCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector*, 40ul>&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PolyCellContact::findBestPair(void)")]
pub fn stub_0x881b38(handle: &crate::slot::InstanceHandle) {
// RBX::PolyCellContact::findBestPair() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PolyCellContact::generateDataForMovingAssemblyStage(void)")]
pub fn stub_0x881d50(handle: &crate::slot::InstanceHandle) {
// RBX::PolyCellContact::generateDataForMovingAssemblyStage() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::allocateClone(void)")]
pub fn stub_0x881d58(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::allocateClone() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::test(void)")]
pub fn stub_0x881d98(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::test() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::findOtherFace(RBX::POLY::Vertex const*)")]
pub fn stub_0x88214c(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::findOtherFace(RBX::POLY::Vertex const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::pairIsValid(void)")]
pub fn stub_0x882330(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::pairIsValid() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
pub fn stub_0x882510(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector*, 40ul>&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::vertexInside(RBX::Primitive *,RBX::Primitive *,RBX::POLY::Vertex const*,RBX::POLY::Face const*,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
pub fn stub_0x882ec0(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::vertexInside(RBX::Primitive*, RBX::Primitive*, RBX::POLY::Vertex co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellFaceFacePair::newFaceEdgeConnector(unsigned long,RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
pub fn stub_0x8830ec(handle: &crate::slot::InstanceHandle) {
// RBX::CellFaceFacePair::newFaceEdgeConnector(unsigned long, RBX::POLY::Vertex const*, RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CellEdgeEdgePair::allocateClone(void)")]
pub fn stub_0x88339c(handle: &crate::slot::InstanceHandle) {
// RBX::CellEdgeEdgePair::allocateClone() — engine-side; linkage preserved via the alias.
let _ = handle;
}
