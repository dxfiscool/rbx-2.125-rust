// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x4fed38 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4fef4c..0x506dac | existing ~9841 -> ~9941 total (union; filler 0x4fef4c ascending, global remaining)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12GameSettingsENS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x4fef4c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GameSettings"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v")]
pub fn stub_0x4ff25c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sGameSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sGameSettingsEEEEvv")]
pub fn stub_0x4ff2a0() -> crate::slot::PortedFn {
// IDA 0x4ff2a0: void RBX::Name::callDoDeclare<RBX::sGameSettings>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4ff2a0, "void RBX::Name::callDoDeclare<RBX::sGameSettings>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v")]
pub fn stub_0x4ff2a4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGameSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(RBX::ProfanityFilter *)")]
pub fn stub_0x4ff388() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ProfanityFilter")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ProfanityFilter>(RBX::ProfanityFilter *)")]
pub fn stub_0x4ff45c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::~sp_counted_impl_p()")]
pub fn stub_0x4ff568(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::~sp_counted_impl_p() [0x4ff56c]")]
pub fn stub_0x4ff56c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::dispose(void)")]
pub fn stub_0x4ff570() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4ff614() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ProfanityFilter>::get_untyped_deleter(void)")]
pub fn stub_0x4ff618() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(rbx_core::Weak<RBX::ProfanityFilter> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x4ff61c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ProfanityFilter")
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_s_instance(void)")]
pub fn stub_0x4ff698(handle: &crate::slot::InstanceHandle) {
// RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_s_instance() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_s_instance(void)")]
pub fn stub_0x4ff69c(handle: &crate::slot::InstanceHandle) {
// RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_s_instance() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::Weak<RBX::ProfanityFilter>::~weak_ptr()")]
pub fn stub_0x4ff700(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_sync(void)")]
pub fn stub_0x4ff714(handle: &crate::slot::InstanceHandle) {
// RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_sync() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_sync(void)")]
pub fn stub_0x4ff718(handle: &crate::slot::InstanceHandle) {
// RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_sync() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::mutex::~mutex()")]
pub fn stub_0x4ff808(handle: crate::slot::InstanceHandle) {
// RBX::mutex dtor.
drop(handle);
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x4ff818(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "RBX::NullVerb::~NullVerb()")]
pub fn stub_0x4ff9dc(handle: crate::slot::InstanceHandle) {
// RBX::NullVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::NullVerb::~NullVerb() [0x4ff9e0]")]
pub fn stub_0x4ff9e0(handle: crate::slot::InstanceHandle) {
// RBX::NullVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::NullVerb::isEnabled(void)const")]
pub fn stub_0x4ffa80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NullVerb getter.
cell.get()
}

#[doc(alias = "RBX::Verb::isChecked(void)const")]
pub fn stub_0x4ffa84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Verb getter.
cell.get()
}

#[doc(alias = "RBX::Verb::isSelected(void)const")]
pub fn stub_0x4ffa88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Verb getter.
cell.get()
}

#[doc(alias = "RBX::Verb::getText(void)const")]
pub fn stub_0x4ffa8c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Verb getter.
cell.get()
}

#[doc(alias = "RBX::NullVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x4ffaa0(handle: &crate::slot::InstanceHandle) {
// RBX::NullVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_200")]
pub fn stub_0x500254() -> crate::slot::PortedFn {
// IDA 0x500254: __GLOBAL__I_a_200.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x500254, "__GLOBAL__I_a_200")
}

#[doc(alias = "RBX::GameSettings::setVideoQualitySetting(RBX::GameSettings::VideoQuality)")]
pub fn stub_0x500bcc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::GameSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::GameSettings::setPostImageSetting(RBX::GameSettings::UploadSetting)")]
pub fn stub_0x500bec(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::GameSettings setter.
cell.set(value)
}

#[doc(alias = "RBX::GameSettings::GameSettings(void)")]
pub fn stub_0x500c0c() -> crate::slot::InstanceHandle {
// RBX::GameSettings ctor.
crate::slot::InstanceHandle::new("RBX::GameSettings")
}

#[doc(alias = "RBX::GameSettings::GameSettings(void) [0x500c10]")]
pub fn stub_0x500c10() -> crate::slot::InstanceHandle {
// RBX::GameSettings ctor.
crate::slot::InstanceHandle::new("RBX::GameSettings")
}

#[doc(alias = "RBX::GameSettings::getVideoQualitySetting(void)const")]
pub fn stub_0x50158c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GameSettings getter.
cell.get()
}

#[doc(alias = "RBX::GameSettings::getPostImageSetting(void)const")]
pub fn stub_0x5015b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GameSettings getter.
cell.get()
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev")]
pub fn stub_0x501608() -> crate::slot::InstanceHandle {
// settings-item ctor.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettingsItem")
}

#[doc(alias = "RBX::GameSettings::~GameSettings()")]
pub fn stub_0x501878(handle: crate::slot::InstanceHandle) {
// RBX::GameSettings dtor.
drop(handle);
}

#[doc(alias = "RBX::GameSettings::~GameSettings() [0x501a00]")]
pub fn stub_0x501a00(handle: crate::slot::InstanceHandle) {
// RBX::GameSettings dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12GameSettingsENS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x501aa0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GameSettings"
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings()")]
pub fn stub_0x501ab0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings() [0x501c34]")]
pub fn stub_0x501c34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12GameSettingsENS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x501dd0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GameSettings"
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings() [0x501de0]")]
pub fn stub_0x501de0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings() [0x501f64]")]
pub fn stub_0x501f64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12GameSettingsENS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x502100() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GameSettings"
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
pub fn stub_0x502174(handle: crate::slot::InstanceHandle) {
// settings-item dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
pub fn stub_0x5021b4(handle: crate::slot::InstanceHandle) {
// settings-item dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
pub fn stub_0x502294(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
pub fn stub_0x5022d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED1Ev")]
pub fn stub_0x5022e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
pub fn stub_0x502324(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12GameSettingsELZNS_13sGameSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x50232c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12GameSettingsELZNS_13sGameSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x502330(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12GameSettingsELZNS_13sGameSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5023d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12GameSettingsELZNS_13sGameSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5023d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12GameSettingsELZNS_13sGameSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x50247c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12GameSettingsELZNS_13sGameSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_13sGameSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x502484(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameSettings::UploadSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x50402c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
pub fn stub_0x504084(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
pub fn stub_0x504138(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
pub fn stub_0x504190(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::resize(unsigned long,RBX::GameSettings::UploadSetting)")]
pub fn stub_0x5041f8(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::push_back(RBX::GameSettings::UploadSetting const&)")]
pub fn stub_0x50422c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,RBX::GameSettings::UploadSetting const&)")]
pub fn stub_0x504254(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_allocate(unsigned long)")]
pub fn stub_0x504338() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::GameSettings::UploadSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *>(RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *)")]
pub fn stub_0x504350(handle: &crate::slot::InstanceHandle) {
// RBX::GameSettings::UploadSetting* std::__copy_backward<false, std::random_access_iterator_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,unsigned long,RBX::GameSettings::UploadSetting const&)")]
pub fn stub_0x50438c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameSettings::VideoQuality,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x50451c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
pub fn stub_0x504574(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
pub fn stub_0x504628(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
pub fn stub_0x504680(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::resize(unsigned long,RBX::GameSettings::VideoQuality)")]
pub fn stub_0x5046e8(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::push_back(RBX::GameSettings::VideoQuality const&)")]
pub fn stub_0x50471c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,RBX::GameSettings::VideoQuality const&)")]
pub fn stub_0x504744(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_allocate(unsigned long)")]
pub fn stub_0x504828() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::GameSettings::VideoQuality * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *>(RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *)")]
pub fn stub_0x504840(handle: &crate::slot::InstanceHandle) {
// RBX::GameSettings::VideoQuality* std::__copy_backward<false, std::random_access_iterator_t~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,unsigned long,RBX::GameSettings::VideoQuality const&)")]
pub fn stub_0x50487c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "global constructor keyed to_a_201")]
pub fn stub_0x504a0c() -> crate::slot::PortedFn {
// IDA 0x504a0c: __GLOBAL__I_a_201.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x504a0c, "__GLOBAL__I_a_201")
}

#[doc(alias = "RBX::GeometryService::GeometryService(void)")]
pub fn stub_0x505018() -> crate::slot::InstanceHandle {
// RBX::GeometryService ctor.
crate::slot::InstanceHandle::new("RBX::GeometryService")
}

#[doc(alias = "RBX::GeometryService::GeometryService(void) [0x50501c]")]
pub fn stub_0x50501c() -> crate::slot::InstanceHandle {
// RBX::GeometryService ctor.
crate::slot::InstanceHandle::new("RBX::GeometryService")
}

#[doc(alias = "RBX::GeometryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x505a48(handle: &crate::slot::InstanceHandle) {
// RBX::GeometryService::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilterDescendents::~FilterDescendents()")]
pub fn stub_0x505e40(handle: crate::slot::InstanceHandle) {
// RBX::FilterDescendents dtor.
drop(handle);
}

#[doc(alias = "RBX::GeometryService::~GeometryService()")]
pub fn stub_0x505e64(handle: crate::slot::InstanceHandle) {
// RBX::GeometryService dtor.
drop(handle);
}

#[doc(alias = "RBX::GeometryService::~GeometryService() [0x505f48]")]
pub fn stub_0x505f48(handle: crate::slot::InstanceHandle) {
// RBX::GeometryService dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE12getClassNameEv")]
pub fn stub_0x50603c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService()")]
pub fn stub_0x506068(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService() [0x506148]")]
pub fn stub_0x506148(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE12getClassNameEv")]
pub fn stub_0x506240() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService() [0x506268]")]
pub fn stub_0x506268(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService() [0x506348]")]
pub fn stub_0x506348(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v")]
pub fn stub_0x506440(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGeometryService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::delete_buckets(void)")]
pub fn stub_0x506898(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Primitive const*>>(RBX::Primitive const* const&,boost::unordered::detail::emplace_args1<RBX::Primitive const*> const&)")]
pub fn stub_0x5068e8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x506a78(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::create_buckets(unsigned long)")]
pub fn stub_0x506ac8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x506bf0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::rehash_impl(unsigned long)")]
pub fn stub_0x506c80(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x506cac(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>>::construct(void)")]
pub fn stub_0x506d00() -> crate::slot::PortedFn {
// IDA 0x506d00: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>>::c~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x506d00, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Pr~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::find_node_impl<RBX::Primitive const*,std::equal_to<RBX::Primitive const*>>(unsigned long,RBX::Primitive const* const&,std::equal_to<RBX::Primitive const*> const&)const")]
pub fn stub_0x506d38(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x506da8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15GeometryServiceELZNS_16sGeometryServiceEENS_17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x506dac(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}
