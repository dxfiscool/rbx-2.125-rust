// Auto-generated skeletons for rbx-script — Lua/Script gap filler EA-sorted asc
// Filter: Lua|Script demangled else gap filler EA-sorted asc next 120 not yet in script
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x562328..0x56a508 | script gap filler EA-asc
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x562328(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
pub fn stub_0x5623a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x5623c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Rocket,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_0x5624a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
pub fn stub_0x5624ac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
pub fn stub_0x5624b0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x5624b4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Rocket,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::Rocket::*)(void)const,void (RBX::Rocket::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
pub fn stub_0x5624d4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "boost::shared_ptr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)")]
pub fn stub_0x562ad0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rocket")
}

#[doc(alias = "boost::shared_ptr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x562b84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rocket")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rocket,RBX::Rocket>(boost::shared_ptr<RBX::Rocket> const*,RBX::Rocket *)const")]
pub fn stub_0x562c4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Rocket")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x562d34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x562e3c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x562e40]")]
pub fn stub_0x562e40(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x562e44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x562e64() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x562e7c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)")]
pub fn stub_0x5632a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyThrust")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x563358() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyThrust")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyThrust,RBX::BodyThrust>(boost::shared_ptr<RBX::BodyThrust> const*,RBX::BodyThrust *)const")]
pub fn stub_0x563420() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyThrust")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x563508() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x563610(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x563614]")]
pub fn stub_0x563614(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x563618() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x563638() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x563650() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)")]
pub fn stub_0x563a78() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyForce")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x563b2c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyForce")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyForce,RBX::BodyForce>(boost::shared_ptr<RBX::BodyForce> const*,RBX::BodyForce *)const")]
pub fn stub_0x563bf4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyForce")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x563cdc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x563de4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x563de8]")]
pub fn stub_0x563de8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x563dec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x563e0c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x563e24() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)")]
pub fn stub_0x56424c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyAngularVelocity")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x564300() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyAngularVelocity")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyAngularVelocity,RBX::BodyAngularVelocity>(boost::shared_ptr<RBX::BodyAngularVelocity> const*,RBX::BodyAngularVelocity *)const")]
pub fn stub_0x5643c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyAngularVelocity")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x5644b0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x5645b8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5645bc]")]
pub fn stub_0x5645bc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x5645c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x5645e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x5645f8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)")]
pub fn stub_0x564a20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyVelocity")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x564ad4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyVelocity")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyVelocity,RBX::BodyVelocity>(boost::shared_ptr<RBX::BodyVelocity> const*,RBX::BodyVelocity *)const")]
pub fn stub_0x564b9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyVelocity")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x564c84() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x564d8c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x564d90]")]
pub fn stub_0x564d90(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x564d94() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x564db4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x564dcc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)")]
pub fn stub_0x5651f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyPosition")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x5652a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyPosition")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyPosition,RBX::BodyPosition>(boost::shared_ptr<RBX::BodyPosition> const*,RBX::BodyPosition *)const")]
pub fn stub_0x565370() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyPosition")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x565458() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x565560(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x565564]")]
pub fn stub_0x565564(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x565568() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x565588() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x5655a0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::shared_ptr<RBX::BodyGyro> RBX::Creatable<RBX::Instance>::create<RBX::BodyGyro>(void)")]
pub fn stub_0x5659c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyGyro")
}

#[doc(alias = "boost::shared_ptr<RBX::BodyGyro>::shared_ptr<RBX::BodyGyro,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x565a7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyGyro")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyGyro,RBX::BodyGyro>(boost::shared_ptr<RBX::BodyGyro> const*,RBX::BodyGyro *)const")]
pub fn stub_0x565b44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BodyGyro")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x565c2c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x565d34(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x565d38]")]
pub fn stub_0x565d38(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x565d3c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x565d5c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyGyro *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x565d74() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Handles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x5673ac(handle: &crate::slot::InstanceHandle) {
// RBX::Handles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Handles::getHandleType(void)const")]
pub fn stub_0x567694(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Handles getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Handles,RBX::Handles::VisualStyle>::~EnumPropDescriptor()")]
pub fn stub_0x5676b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Handles,RBX::Faces>::~PropDescriptor()")]
pub fn stub_0x5676e4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId),rbx::remote_signal<void ()(RBX::NormalId)>>::~RemoteEventDesc()")]
pub fn stub_0x567708(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::~RemoteEventDesc()")]
pub fn stub_0x56772c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x567a10(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::Handles, void (RBX::NormalId)>::onPropertyChanged(RBX::Refle~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x567a70(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::Handles, void (RBX::NormalId, float)>::onPropertyChanged(RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::shared_ptr<RBX::Handles> RBX::Creatable<RBX::Instance>::create<RBX::Handles>(void)")]
pub fn stub_0x5681f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Handles")
}

#[doc(alias = "boost::shared_ptr<RBX::Handles>::shared_ptr<RBX::Handles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x5682a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Handles")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Handles,RBX::Handles>(boost::shared_ptr<RBX::Handles> const*,RBX::Handles *)const")]
pub fn stub_0x568370() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Handles")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x568458() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x568560(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x568564]")]
pub fn stub_0x568564(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x568568() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x568588() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Handles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x5685a0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)")]
pub fn stub_0x568940() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::NormalId)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")]
pub fn stub_0x568aa0(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)")]
pub fn stub_0x568ac8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)")]
pub fn stub_0x568be8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::NormalId, float)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")]
pub fn stub_0x568d48(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)")]
pub fn stub_0x568d70(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
pub fn stub_0x568e90() -> crate::slot::SlotConnection {
// IDA 0x568e90: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
pub fn stub_0x568f50(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot() [0x568f7c]")]
pub fn stub_0x568f7c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x569050(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x569050: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x569058(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x569058: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
pub fn stub_0x569060() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_0x569078(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x569078: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable() [0x5690a4]")]
pub fn stub_0x5690a4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5690a4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
pub fn stub_0x569178() -> crate::slot::SlotConnection {
// IDA 0x569178: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
pub fn stub_0x569238(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot() [0x569264]")]
pub fn stub_0x569264(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x569338(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x569338: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x569340(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x569340: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
pub fn stub_0x569348() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_0x569360(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x569360: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable() [0x56938c]")]
pub fn stub_0x56938c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x56938c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
pub fn stub_0x569afc() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::Handles")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::Handles,void ()(RBX::NormalId,float),rbx::remote_signal<void ()(RBX::NormalId,float)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::NormalId,float)")]
pub fn stub_0x569b7c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<2, RBX::Handles, void (RBX::NormalId, float), rbx::re~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_0x569ce8() -> crate::slot::SlotConnection {
// IDA 0x569ce8: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)")]
pub fn stub_0x569f68(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0x569f8c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x569fb8]")]
pub fn stub_0x569fb8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
pub fn stub_0x56a1a8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x56a1a8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
pub fn stub_0x56a1d0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x56a1d0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list2<RBX::NormalId&,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,boost::_bi::list2<RBX::NormalId&,float &> &,int)")]
pub fn stub_0x56a1f8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x56a1f8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
pub fn stub_0x56a508(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x56a508: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}
