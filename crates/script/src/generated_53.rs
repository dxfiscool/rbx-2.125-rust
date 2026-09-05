// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x4f9980 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4f9988..0x4fed38 | existing ~9741 -> ~9841 total (union; filler 0x4f9988 ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::~EnumPropDescriptor()")]
pub fn stub_0x4f9988(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::addPair(RBX::Frame::Style,char const*)")]
pub fn stub_0x4f99ac(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::addPair(RBX::Frame::Style, char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f9d0c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f9d10(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f9db0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f9db8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f9e5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f9e64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::resize(unsigned long,RBX::Frame::Style)")]
pub fn stub_0x4f9f08(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::push_back(RBX::Frame::Style const&)")]
pub fn stub_0x4f9f3c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Frame::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x4f9f64(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
pub fn stub_0x4f9fbc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
pub fn stub_0x4fa070(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
pub fn stub_0x4fa0c8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,RBX::Frame::Style const&)")]
pub fn stub_0x4fa130(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_allocate(unsigned long)")]
pub fn stub_0x4fa214() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Frame::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Frame::Style *,RBX::Frame::Style *>(RBX::Frame::Style *,RBX::Frame::Style *,RBX::Frame::Style *)")]
pub fn stub_0x4fa22c(handle: &crate::slot::InstanceHandle) {
// RBX::Frame::Style* std::__copy_backward<false, std::random_access_iterator_tag>::__copy_b<~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,unsigned long,RBX::Frame::Style const&)")]
pub fn stub_0x4fa268(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::EnumPropDescriptor<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>(char const*,char const*,RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4fa3f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::~EnumPropDescriptor() [0x4fa5ac]")]
pub fn stub_0x4fa5ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::isReadOnly(void)const")]
pub fn stub_0x4fa5d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::isWriteOnly(void)const")]
pub fn stub_0x4fa5e8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4fa5f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x4fa620(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x4fa644(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x4fa790(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::hasStringValue(void)const")]
pub fn stub_0x4fa7b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4fa7b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x4fa7dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x4fa81c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x4fa83c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4faa7c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x4faa98(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4faacc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4faad4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4fab20(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x4fab40(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToIndex(RBX::Frame::Style)const")]
pub fn stub_0x4fab74(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToIndex(RBX::Frame::Style) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x4fabe4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Frame,RBX::Frame::Style>::GetSetImpl<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>::isReadOnly(void)const")]
pub fn stub_0x4fac24(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Frame,RBX::Frame::Style>::GetSetImpl<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>::isWriteOnly(void)const")]
pub fn stub_0x4fac28(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Frame,RBX::Frame::Style>::GetSetImpl<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4fac2c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Frame,RBX::Frame::Style>::GetSetImpl<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>::setValue(RBX::Reflection::DescribedBase *,RBX::Frame::Style const&)const")]
pub fn stub_0x4fac4c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "global constructor keyed to_a_199")]
pub fn stub_0x4fac70() -> crate::slot::PortedFn {
// IDA 0x4fac70: __GLOBAL__I_a_199.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4fac70, "__GLOBAL__I_a_199")
}

#[doc(alias = "RBX::SecurePlayerGame::SecurePlayerGame(RBX::Verb *,char const*,bool)")]
pub fn stub_0x4faee8() -> crate::slot::InstanceHandle {
// RBX::SecurePlayerGame ctor.
crate::slot::InstanceHandle::new("RBX::SecurePlayerGame")
}

#[doc(alias = "RBX::SecurePlayerGame::SecurePlayerGame(RBX::Verb *,char const*,bool) [0x4faeec]")]
pub fn stub_0x4faeec() -> crate::slot::InstanceHandle {
// RBX::SecurePlayerGame ctor.
crate::slot::InstanceHandle::new("RBX::SecurePlayerGame")
}

#[doc(alias = "RBX::Game::Game(RBX::Verb *,char const*,bool)")]
pub fn stub_0x4fafc4() -> crate::slot::InstanceHandle {
// RBX::Game ctor.
crate::slot::InstanceHandle::new("RBX::Game")
}

#[doc(alias = "RBX::Game::~Game()")]
pub fn stub_0x4fb85c(handle: crate::slot::InstanceHandle) {
// RBX::Game dtor.
drop(handle);
}

#[doc(alias = "RBX::UnsecuredStudioGame::UnsecuredStudioGame(RBX::Verb *,char const*,bool)")]
pub fn stub_0x4fba28() -> crate::slot::InstanceHandle {
// RBX::UnsecuredStudioGame ctor.
crate::slot::InstanceHandle::new("RBX::UnsecuredStudioGame")
}

#[doc(alias = "RBX::UnsecuredStudioGame::UnsecuredStudioGame(RBX::Verb *,char const*,bool) [0x4fba2c]")]
pub fn stub_0x4fba2c() -> crate::slot::InstanceHandle {
// RBX::UnsecuredStudioGame ctor.
crate::slot::InstanceHandle::new("RBX::UnsecuredStudioGame")
}

#[doc(alias = "RBX::Game::globalInit(void)")]
pub fn stub_0x4fbb04(handle: &crate::slot::InstanceHandle) {
// RBX::Game::globalInit() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Game::setupDataModel(std::string const&)")]
pub fn stub_0x4fbc68(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Game setter.
cell.set(value)
}

#[doc(alias = "RBX::Game::setDataModel(rbx_core::SharedPtr<RBX::DataModel>)")]
pub fn stub_0x4fc0c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::Game::~Game() [0x4fc348]")]
pub fn stub_0x4fc348(handle: crate::slot::InstanceHandle) {
// RBX::Game dtor.
drop(handle);
}

#[doc(alias = "RBX::Game::~Game() [0x4fc3e8]")]
pub fn stub_0x4fc3e8(handle: crate::slot::InstanceHandle) {
// RBX::Game dtor.
drop(handle);
}

#[doc(alias = "RBX::Game::shutdown(void)")]
pub fn stub_0x4fc3ec(handle: &crate::slot::InstanceHandle) {
// RBX::Game::shutdown() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Game::doClearVerbs(void)")]
pub fn stub_0x4fc420(handle: &crate::slot::InstanceHandle) {
// RBX::Game::doClearVerbs() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Game::clearVerbs(bool)")]
pub fn stub_0x4fc548(handle: &crate::slot::InstanceHandle) {
// RBX::Game::clearVerbs(bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Game::shutdownGameDataModel(void)")]
pub fn stub_0x4fc660(handle: &crate::slot::InstanceHandle) {
// RBX::Game::shutdownGameDataModel() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Game::getSuppressNavKeys(void)")]
pub fn stub_0x4fc750(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Game getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::~shared_ptr()")]
pub fn stub_0x4fc774(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::operator=(rbx_core::SharedPtr<RBX::ProfanityFilter> const&)")]
pub fn stub_0x4fc788(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::getInstance(void)")]
pub fn stub_0x4fc7c0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ScopedSingleton getter.
cell.get()
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv")]
pub fn stub_0x4fc998() -> crate::slot::InstanceHandle {
// settings-item ctor.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettingsItem")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::CommonVerbs>::reset<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
pub fn stub_0x4fcd04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CommonVerbs")
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::push_back(RBX::Verb * const&)")]
pub fn stub_0x4fcd30(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::OverlayDataModel>::operator=(rbx_core::SharedPtr<RBX::OverlayDataModel> const&)")]
pub fn stub_0x4fcd5c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::operator=(rbx_core::SharedPtr<RBX::DataModel> const&)")]
pub fn stub_0x4fcd94(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list_av_2<RBX::Game*,std::string>::type> boost::bind<void,RBX::Game,std::string const&,RBX::Game*,std::string>(void (RBX::Game::*)(std::string const&),RBX::Game*,std::string)")]
pub fn stub_0x4fcdcc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::DataModel *)>::operator()(RBX::DataModel *)")]
pub fn stub_0x4fcf84(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (RBX::DataModel*)>::operator()(RBX::DataModel*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void RBX::shutdownDM<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> &)")]
pub fn stub_0x4fd0c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "void RBX::shutdownDM<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &)")]
pub fn stub_0x4fd1e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::OverlayDataModel")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12GameSettingsENS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x4fd300() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GameSettings"
}

#[doc(alias = "RBX::SecurePlayerGame::~SecurePlayerGame()")]
pub fn stub_0x4fd304(handle: crate::slot::InstanceHandle) {
// RBX::SecurePlayerGame dtor.
drop(handle);
}

#[doc(alias = "RBX::SecurePlayerGame::~SecurePlayerGame() [0x4fd308]")]
pub fn stub_0x4fd308(handle: crate::slot::InstanceHandle) {
// RBX::SecurePlayerGame dtor.
drop(handle);
}

#[doc(alias = "RBX::UnsecuredStudioGame::~UnsecuredStudioGame()")]
pub fn stub_0x4fd3a8(handle: crate::slot::InstanceHandle) {
// RBX::UnsecuredStudioGame dtor.
drop(handle);
}

#[doc(alias = "RBX::UnsecuredStudioGame::~UnsecuredStudioGame() [0x4fd3ac]")]
pub fn stub_0x4fd3ac(handle: crate::slot::InstanceHandle) {
// RBX::UnsecuredStudioGame dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> &)")]
pub fn stub_0x4fd44c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::DataModel*)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::on_error(std::exception &)")]
pub fn stub_0x4fd5ac(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS7_5list2INS7_5valueIPSB_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4fd5d4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvNS1_4GameERKSsEENS6_5list2INS6_5valueIPSA_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x4fd70c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>)")]
pub fn stub_0x4fd848(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4fd994(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_0x4fd9b0(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x4fd9c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x4fdb00(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x4fdc38(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>::operator()<RBX::DataModel *>(RBX::DataModel * &)")]
pub fn stub_0x4fdd08() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x4fdd20(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>>::list2(boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>)")]
pub fn stub_0x4fde5c() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Verb **,std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>>,RBX::Verb * const&)")]
pub fn stub_0x4fdf80(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_allocate(unsigned long)")]
pub fn stub_0x4fe060() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CommonVerbs>::shared_ptr<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
pub fn stub_0x4fe078() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CommonVerbs")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
pub fn stub_0x4fe14c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::CommonVerbs::~CommonVerbs()")]
pub fn stub_0x4fe258(handle: crate::slot::InstanceHandle) {
// RBX::CommonVerbs dtor.
drop(handle);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::~sp_counted_impl_p()")]
pub fn stub_0x4fec88(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::~sp_counted_impl_p() [0x4fec8c]")]
pub fn stub_0x4fec8c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::dispose(void)")]
pub fn stub_0x4fec90() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::get_deleter(std::type_info const&)")]
pub fn stub_0x4fed34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CommonVerbs>::get_untyped_deleter(void)")]
pub fn stub_0x4fed38() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
