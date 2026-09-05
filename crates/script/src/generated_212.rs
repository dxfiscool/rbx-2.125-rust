// Auto-generated skeletons for rbx-script — shard 212 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x37be48..0x381534 | script 21552->21702 distinct (filler 0x37be48 asc, not-in-script 63993->63843)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*,char *)")]
pub fn stub_0x37be48(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*, c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char *)")]
pub fn stub_0x37be64(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_0x37be68() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService::SoundJob")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
pub fn stub_0x37bf50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService::SoundJob")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
pub fn stub_0x37c034() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
pub fn stub_0x37c12c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p() [0x37c130]")]
pub fn stub_0x37c130(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::dispose(void)")]
pub fn stub_0x37c134() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x37c144() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_untyped_deleter(void)")]
pub fn stub_0x37c148() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_0x37c14c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_0x37c200() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_0x37c24c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
pub fn stub_0x37c2b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x37c3a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x37c440() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x37c4c8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")]
pub fn stub_0x37c60c() -> crate::slot::PortedFn {
// IDA 0x37c60c: void RBX::Name::callDoDeclare<RBX::sStockSound>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x37c60c, "void RBX::Name::callDoDeclare<RBX::sStockSound>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")]
pub fn stub_0x37c610(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sStockSound>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x37c6f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "RBX::StockSound::~StockSound()")]
pub fn stub_0x37c934(handle: crate::slot::InstanceHandle) {
// RBX::StockSound dtor.
drop(handle);
}

#[doc(alias = "RBX::StockSound::~StockSound() [0x37c938]")]
pub fn stub_0x37c938(handle: crate::slot::InstanceHandle) {
// RBX::StockSound dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x37c9d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
pub fn stub_0x37c9e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound() [0x37c9f0]")]
pub fn stub_0x37c9f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x37ca94() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound() [0x37caa4]")]
pub fn stub_0x37caa4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound() [0x37caac]")]
pub fn stub_0x37caac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x37cb50() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x37cbc4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x37cbc8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x37cc68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x37cc70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x37cd14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10StockSoundELZNS_11sStockSoundEENS_14FactoryProductIS2_NS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x37cd1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x37cdc0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StockSound")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StockSound,RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const*,RBX::StockSound *)const")]
pub fn stub_0x37ce88() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StockSound")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x37cf74() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x37d07c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x37d080]")]
pub fn stub_0x37d080(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x37d084() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x37d0a4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x37d0bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>> *)")]
pub fn stub_0x37d0c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)")]
pub fn stub_0x37d0f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_0x37d1b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
pub fn stub_0x37d1dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
pub fn stub_0x37d1f8() -> crate::slot::PortedFn {
// IDA 0x37d1f8: boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x37d1f8, "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x37d2a0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x37d2a4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x37d344(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x37d34c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x37d3f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10Soundscape12SoundServiceELZNS2_13sSoundServiceEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundServiceEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x37d3f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
pub fn stub_0x37d49c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
pub fn stub_0x37d4d0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x37d4f8(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_0x37d550(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_0x37d604(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
pub fn stub_0x37d65c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
pub fn stub_0x37d6c4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
pub fn stub_0x37d7a8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
pub fn stub_0x37d7c0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::ReverbType* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
pub fn stub_0x37d7fc(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
pub fn stub_0x37d98c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("SoundServiceStatsItem")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
pub fn stub_0x37da40(handle: &crate::slot::InstanceHandle) {
// RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*, int const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x37dbf4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x37dbf8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<int>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<int>,int const*>(int const*)")]
pub fn stub_0x37dc18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::TypedStatsItem<int>")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x37dd20(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int const&,int const& (*)(int const*),boost::_bi::list1<boost::_bi::value<int const*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x37dd80(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x37dd90() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
pub fn stub_0x37de98() -> crate::slot::PortedFn {
// IDA 0x37de98: SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x37de98, "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")
}

#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
pub fn stub_0x37e05c() -> crate::slot::PortedFn {
// IDA 0x37e05c: SoundServiceStatsItem::~SoundServiceStatsItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x37e05c, "SoundServiceStatsItem::~SoundServiceStatsItem()")
}

#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem() [0x37e098]")]
pub fn stub_0x37e098() -> crate::slot::PortedFn {
// IDA 0x37e098: SoundServiceStatsItem::~SoundServiceStatsItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x37e098, "SoundServiceStatsItem::~SoundServiceStatsItem()")
}

#[doc(alias = "SoundServiceStatsItem::update(void)")]
pub fn stub_0x37e16c() -> crate::slot::PortedFn {
// IDA 0x37e16c: SoundServiceStatsItem::update().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x37e16c, "SoundServiceStatsItem::update()")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
pub fn stub_0x37e344(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem() [0x37e384]")]
pub fn stub_0x37e384(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem() [0x37e458]")]
pub fn stub_0x37e458(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem() [0x37e498]")]
pub fn stub_0x37e498(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x37e56c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("SoundServiceStatsItem")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<SoundServiceStatsItem,SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const*,SoundServiceStatsItem *)const")]
pub fn stub_0x37e634() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("SoundServiceStatsItem")
}

#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x37e720() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x37e828(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x37e82c]")]
pub fn stub_0x37e82c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x37e830() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x37e850() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x37e868() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
pub fn stub_0x37e86c() -> crate::slot::InstanceHandle {
// RBX::Soundscape::SoundService::SoundJob ctor.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService::SoundJob")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
pub fn stub_0x37e9c4(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::SoundJob dtor.
drop(handle);
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob() [0x37e9c8]")]
pub fn stub_0x37e9c8(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::SoundJob dtor.
drop(handle);
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x37ea68(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x37ea84(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x37eaa0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::SoundJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
pub fn stub_0x37eab0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "global constructor keyed to_a_138")]
pub fn stub_0x37ead8() -> crate::slot::PortedFn {
// IDA 0x37ead8: __GLOBAL__I_a_138.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x37ead8, "__GLOBAL__I_a_138")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void)")]
pub fn stub_0x37f4d8() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::EnumDesc(void) [0x37f4dc]")]
pub fn stub_0x37f4dc() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::convert<RBX::SoundType>(void)")]
pub fn stub_0x37f7c8(handle: &crate::slot::InstanceHandle) {
// RBX::SoundType& RBX::Reflection::Variant::convert<RBX::SoundType>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")]
pub fn stub_0x37f7cc(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&, RBX::SoundType&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType,char const*)")]
pub fn stub_0x37f818(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::SoundType>::addPair(RBX::SoundType, char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SoundType & RBX::Reflection::Variant::genericConvert<RBX::SoundType>(void)")]
pub fn stub_0x37fb78(handle: &crate::slot::InstanceHandle) {
// RBX::SoundType& RBX::Reflection::Variant::genericConvert<RBX::SoundType>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x37fd64(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x37fdbc(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
pub fn stub_0x37feac(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
pub fn stub_0x37fee0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x37ff08(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_0x37ff60(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_0x380014(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
pub fn stub_0x38006c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
pub fn stub_0x3800d4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
pub fn stub_0x3801b8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
pub fn stub_0x3801d0(handle: &crate::slot::InstanceHandle) {
// RBX::SoundType* std::__copy_backward<false, std::random_access_iterator_tag>::__copy_b<RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
pub fn stub_0x38020c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "global constructor keyed to_a_139")]
pub fn stub_0x38039c() -> crate::slot::PortedFn {
// IDA 0x38039c: __GLOBAL__I_a_139.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38039c, "__GLOBAL__I_a_139")
}

#[doc(alias = "RBX::SpanningEdge::getConstChildSpanningNode(void)const")]
pub fn stub_0x380464(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpanningEdge getter.
cell.get()
}

#[doc(alias = "RBX::SpanningEdge::getChildSpanningNode(void)")]
pub fn stub_0x3804e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpanningEdge getter.
cell.get()
}

#[doc(alias = "RBX::SpanningEdge::getParentSpanningNode(void)")]
pub fn stub_0x3804e4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpanningEdge getter.
cell.get()
}

#[doc(alias = "RBX::SpanningEdge::removeFromSpanningTree(void)")]
pub fn stub_0x3804fc(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningEdge::removeFromSpanningTree() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode *)")]
pub fn stub_0x380568(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningEdge::addToSpanningTree(RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningEdge::inSpanningTree(void)const")]
pub fn stub_0x3806bc(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningEdge::inSpanningTree() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_140")]
pub fn stub_0x3806e4() -> crate::slot::PortedFn {
// IDA 0x3806e4: __GLOBAL__I_a_140.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3806e4, "__GLOBAL__I_a_140")
}

#[doc(alias = "RBX::SpanningNode::setEdgeToParent(RBX::SpanningEdge *)")]
pub fn stub_0x3807ac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::SpanningNode setter.
cell.set(value)
}

#[doc(alias = "global constructor keyed to_a_141")]
pub fn stub_0x3807b0() -> crate::slot::PortedFn {
// IDA 0x3807b0: __GLOBAL__I_a_141.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3807b0, "__GLOBAL__I_a_141")
}

#[doc(alias = "RBX::SpanningTree::SpanningTree(void)")]
pub fn stub_0x380878() -> crate::slot::InstanceHandle {
// RBX::SpanningTree ctor.
crate::slot::InstanceHandle::new("RBX::SpanningTree")
}

#[doc(alias = "RBX::SpanningTree::~SpanningTree()")]
pub fn stub_0x38089c(handle: crate::slot::InstanceHandle) {
// RBX::SpanningTree dtor.
drop(handle);
}

#[doc(alias = "RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge *)")]
pub fn stub_0x3809c4(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::insertSpanningTreeEdge(RBX::SpanningEdge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge *,RBX::SpanningEdge *&,int &)")]
pub fn stub_0x380a6c(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::findLightestUpstream(RBX::SpanningEdge*, RBX::SpanningEdge*&, int&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::swapTree(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
pub fn stub_0x380abc(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::swapTree(RBX::SpanningEdge*, RBX::SpanningEdge*, RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge *)")]
pub fn stub_0x380b30(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::removeSpanningTreeEdge(RBX::SpanningEdge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode *,RBX::SpanningNode *&)")]
pub fn stub_0x380bac(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::findHeaviestDownstream(RBX::SpanningNode*, RBX::SpanningNode*&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::swap(RBX::SpanningEdge *,RBX::SpanningEdge *,RBX::SpanningNode *)")]
pub fn stub_0x380cdc(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::swap(RBX::SpanningEdge*, RBX::SpanningEdge*, RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::removeEdge(RBX::SpanningEdge *)")]
pub fn stub_0x380d50(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::removeEdge(RBX::SpanningEdge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::addEdge(RBX::SpanningEdge *,RBX::SpanningNode *)")]
pub fn stub_0x380e34(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::addEdge(RBX::SpanningEdge*, RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode *,RBX::SpanningEdge *,G3D::Array<RBX::SpanningEdge *,10,32ul> &)")]
pub fn stub_0x380f1c(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode*, RBX::SpanningEdge*, G3D::Arr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::activateEdges(RBX::SpanningNode *,G3D::Array<RBX::SpanningEdge *,10,32ul> const&)")]
pub fn stub_0x38103c(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::activateEdges(RBX::SpanningNode*, G3D::Array<RBX::SpanningEdge*, 10, 32~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode *,RBX::SpanningNode *,int,int,RBX::SpanningEdge *&,int &)")]
pub fn stub_0x381120(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::findLightestUpstream(RBX::SpanningNode*, RBX::SpanningNode*, int, int, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode *,std::set<RBX::SpanningNode *,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>> &)")]
pub fn stub_0x38120c(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::buildDownstreamTree(RBX::SpanningNode*, std::set<RBX::SpanningNode*, st~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::append(RBX::SpanningEdge * const&)")]
pub fn stub_0x3812ac(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
pub fn stub_0x381308(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpanningNode getter.
cell.get()
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
pub fn stub_0x381328(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::onSpanningEdgeAdding(RBX::SpanningEdge*, RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
pub fn stub_0x38132c(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::onSpanningEdgeAdded(RBX::SpanningEdge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
pub fn stub_0x381330(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::onSpanningEdgeRemoving(RBX::SpanningEdge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
pub fn stub_0x381334(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge*, RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpanningTree::validateTree(RBX::SpanningNode *)")]
pub fn stub_0x381338(handle: &crate::slot::InstanceHandle) {
// RBX::SpanningTree::validateTree(RBX::SpanningNode*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
pub fn stub_0x38133c(handle: &crate::slot::InstanceHandle) {
// RBX::FindHeaviest::operator()(RBX::SpanningNode*, RBX::SpanningEdge*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
pub fn stub_0x3813bc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
pub fn stub_0x381424(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::resize(int,bool)")]
pub fn stub_0x38147c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::realloc(int)")]
pub fn stub_0x381534(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}
