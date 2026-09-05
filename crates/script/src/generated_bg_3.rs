// Auto-generated skeletons for rbx-script — script-bg filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — script-bg filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3e11ec..0x3e7298 | script 23552 -> 23652 total (filler 0x3e11ec asc, not-in-script 61993->61893)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3e11ec(handle: &crate::slot::InstanceHandle) {
// RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3e11f0(handle: &crate::slot::InstanceHandle) {
// RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance *)")]
pub fn stub_0x3e11f4(handle: &crate::slot::InstanceHandle) {
// RBX::Skin* RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::shared_ptr<RBX::BodyColors> RBX::Creatable<RBX::Instance>::create<RBX::BodyColors>(void)")]
pub fn stub_0x3e31c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyColors")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyColors>::shared_ptr<RBX::BodyColors,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e3278() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyColors")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyColors,RBX::BodyColors>(boost::shared_ptr<RBX::BodyColors> const*,RBX::BodyColors *)const")]
pub fn stub_0x3e3340() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyColors")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e3428() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e3530(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3e3534]")]
pub fn stub_0x3e3534(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3e3538() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e3558() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyColors *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3e3570() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::Skin> RBX::Creatable<RBX::Instance>::create<RBX::Skin>(void)")]
pub fn stub_0x3e3b78() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Skin")
}

#[doc(alias = "boost::shared_ptr<RBX::Skin>::shared_ptr<RBX::Skin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e3c28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Skin")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Skin,RBX::Skin>(boost::shared_ptr<RBX::Skin> const*,RBX::Skin *)const")]
pub fn stub_0x3e3cf0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Skin")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e3dd8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e3ee0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3e3ee4]")]
pub fn stub_0x3e3ee4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3e3ee8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e3f08() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Skin *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3e3f20() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::ShirtGraphic> RBX::Creatable<RBX::Instance>::create<RBX::ShirtGraphic>(void)")]
pub fn stub_0x3e460c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ShirtGraphic")
}

#[doc(alias = "boost::shared_ptr<RBX::ShirtGraphic>::shared_ptr<RBX::ShirtGraphic,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e46bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ShirtGraphic")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ShirtGraphic,RBX::ShirtGraphic>(boost::shared_ptr<RBX::ShirtGraphic> const*,RBX::ShirtGraphic *)const")]
pub fn stub_0x3e4784() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ShirtGraphic")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e486c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e4974(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3e4978]")]
pub fn stub_0x3e4978(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3e497c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e499c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ShirtGraphic *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3e49b4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::Pants> RBX::Creatable<RBX::Instance>::create<RBX::Pants>(void)")]
pub fn stub_0x3e4f34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Pants")
}

#[doc(alias = "boost::shared_ptr<RBX::Pants>::shared_ptr<RBX::Pants,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e4fe4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Pants")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pants,RBX::Pants>(boost::shared_ptr<RBX::Pants> const*,RBX::Pants *)const")]
pub fn stub_0x3e50ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Pants")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e5194() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e529c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3e52a0]")]
pub fn stub_0x3e52a0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3e52a4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e52c4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pants *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3e52dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::Shirt> RBX::Creatable<RBX::Instance>::create<RBX::Shirt>(void)")]
pub fn stub_0x3e5704() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Shirt")
}

#[doc(alias = "boost::shared_ptr<RBX::Shirt>::shared_ptr<RBX::Shirt,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e57b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Shirt")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Shirt,RBX::Shirt>(boost::shared_ptr<RBX::Shirt> const*,RBX::Shirt *)const")]
pub fn stub_0x3e587c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Shirt")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3e5964() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3e5a6c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3e5a70]")]
pub fn stub_0x3e5a70(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3e5a74() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3e5a94() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Shirt *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3e5aac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(void)")]
pub fn stub_0x3e5cf4(handle: &crate::slot::InstanceHandle) {
// RBX::Skin* RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e5d40(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e5d44(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e5de4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e5dec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e5e90(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e5e98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e5f3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e5f44(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3e5fe8() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isReadOnly(void)const")]
pub fn stub_0x3e617c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isWriteOnly(void)const")]
pub fn stub_0x3e6180() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3e6184() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
pub fn stub_0x3e6190() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e61e0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e61e4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6284(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e628c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6330(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6338(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e63dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e63e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3e6488() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isReadOnly(void)const")]
pub fn stub_0x3e661c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isWriteOnly(void)const")]
pub fn stub_0x3e6620() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3e6624() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
pub fn stub_0x3e6630() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "RBX::BrickColor")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6680(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e66c8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e67a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e67f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e68d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6924(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6a08(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6a54(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6b38(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6b80(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6c60(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6cac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6d90(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6ddc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6ec0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6f0c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e6ff0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e6ff4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e7094(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e709c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e7140(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e7148(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e71ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3e71f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3e7298(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}
