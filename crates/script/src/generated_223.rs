// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3af738..0x3b9914 | script 22802->22902 distinct (filler 0x3af738 asc, not-in-script 62743->62643)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_0x3af738(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3af738: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::~callable() [0x3af848]")]
pub fn stub_0x3af848(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3af848: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::assign_to_own(boost::function1<void,G3D::Vector3::Axis> const&)")]
pub fn stub_0x3af978(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3af9a8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc()")]
pub fn stub_0x3afb2c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::~EventDesc() [0x3afb50]")]
pub fn stub_0x3afb50(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::PropDescriptor<RBX::Axes (RBX::ArcHandles::*)(void)const,void (RBX::ArcHandles::*)(RBX::Axes)>(char const*,char const*,RBX::Axes (RBX::ArcHandles::*)(void)const,void (RBX::ArcHandles::*)(RBX::Axes),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3afc04(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3afd18() -> crate::slot::InstanceHandle {
// RBX::Reflection::TypedPropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::TypedPropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::~PropDescriptor() [0x3afe3c]")]
pub fn stub_0x3afe3c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::isReadOnly(void)const")]
pub fn stub_0x3afe68(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::isWriteOnly(void)const")]
pub fn stub_0x3afe78(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3afe88(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::equalValues(RBX::Reflection::Describe~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x3afeb0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x3afed8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x3b0030(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::copyValue(RBX::Reflection::DescribedB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::~TypedPropertyDescriptor()")]
pub fn stub_0x3b0054(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Axes>::~TypedPropertyDescriptor() [0x3b0078]")]
pub fn stub_0x3b0078(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::GetSetImpl<RBX::Axes (RBX::ArcHandles::*)(void)const,void (RBX::ArcHandles::*)(RBX::Axes)>::isReadOnly(void)const")]
pub fn stub_0x3b00a4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::GetSetImpl<RBX::Axes (RBX::ArcHandles::*)(void)const,void (RBX::ArcHandles::*)(RBX::Axes)>::isWriteOnly(void)const")]
pub fn stub_0x3b00a8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::GetSetImpl<RBX::Axes (RBX::ArcHandles::*)(void)const,void (RBX::ArcHandles::*)(RBX::Axes)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3b00ac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::GetSetImpl<RBX::Axes (RBX::ArcHandles::*)(void)const,void (RBX::ArcHandles::*)(RBX::Axes)>::setValue(RBX::Reflection::DescribedBase *,RBX::Axes const&)const")]
pub fn stub_0x3b00cc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::ArcHandles::~ArcHandles() [0x3b00f0]")]
pub fn stub_0x3b00f0(handle: crate::slot::InstanceHandle) {
// RBX::ArcHandles dtor.
drop(handle);
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::~remote_signal()")]
pub fn stub_0x3b0324(_v: u64) {
// G3D value dtor — host Copy payload needs no release.
let _ = _v;
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::~remote_signal()")]
pub fn stub_0x3b0470(_v: u64) {
// G3D value dtor — host Copy payload needs no release.
let _ = _v;
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
pub fn stub_0x3b05bc(_v: u64) {
// G3D value dtor — host Copy payload needs no release.
let _ = _v;
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
pub fn stub_0x3b06ec(_v: u64) {
// G3D value dtor — host Copy payload needs no release.
let _ = _v;
}

#[doc(alias = "global constructor keyed to_a_159")]
pub fn stub_0x3b081c() -> crate::slot::PortedFn {
// IDA 0x3b081c: __GLOBAL__I_a_159.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3b081c, "__GLOBAL__I_a_159")
}

#[doc(alias = "RBX::Backpack::Backpack(void)")]
pub fn stub_0x3b0e04() -> crate::slot::InstanceHandle {
// RBX::Backpack ctor.
crate::slot::InstanceHandle::new("RBX::Backpack")
}

#[doc(alias = "RBX::Backpack::Backpack(void) [0x3b0e08]")]
pub fn stub_0x3b0e08() -> crate::slot::InstanceHandle {
// RBX::Backpack ctor.
crate::slot::InstanceHandle::new("RBX::Backpack")
}

#[doc(alias = "RBX::Backpack::~Backpack()")]
pub fn stub_0x3b1224(handle: crate::slot::InstanceHandle) {
// RBX::Backpack dtor.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Backpack> RBX::Creatable<RBX::Instance>::create<RBX::Backpack>(void)")]
pub fn stub_0x3b16ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Backpack")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Backpack>::shared_ptr<RBX::Backpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3b175c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Backpack")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Backpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3b1824() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8BackpackELZNS_9sBackpackEENS_14FactoryProductIS2_NS_6HopperELZNS_9sBackpackEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3b1cc8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8BackpackELZNS_9sBackpackEENS_14FactoryProductIS2_NS_6HopperELZNS_9sBackpackEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3b1ccc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8BackpackELZNS_9sBackpackEENS_14FactoryProductIS2_NS_6HopperELZNS_9sBackpackEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3b1d6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8BackpackELZNS_9sBackpackEENS_14FactoryProductIS2_NS_6HopperELZNS_9sBackpackEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3b1d74(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8BackpackELZNS_9sBackpackEENS_14FactoryProductIS2_NS_6HopperELZNS_9sBackpackEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3b1e18(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8BackpackELZNS_9sBackpackEENS_14FactoryProductIS2_NS_6HopperELZNS_9sBackpackEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3b1e20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::BadgeService::userHasBadge(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b21a4(handle: &crate::slot::InstanceHandle) {
// RBX::BadgeService::userHasBadge(int, int, boost::function<void (bool)>, boost::function<vo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BadgeService::awardBadge(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b2908(handle: &crate::slot::InstanceHandle) {
// RBX::BadgeService::awardBadge(int, int, boost::function<void (bool)>, boost::function<void~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BadgeService::isDisabled(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b3160(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BadgeService getter.
cell.get()
}

#[doc(alias = "RBX::BadgeService::isLegal(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b3840(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BadgeService getter.
cell.get()
}

#[doc(alias = "RBX::BadgeService::hasBadgeResultHelper(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b42f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "RBX::BadgeService::hasBadgeResult(int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b4424(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::BadgeService::isDiabledResultHelper(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b4720() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "RBX::BadgeService::isDisabledResult(int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b484c(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::BadgeService::isLegalResultHelper(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b4a20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "RBX::BadgeService::isLegalResult(int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b4b4c(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::BadgeService::awardBadgeResultHelper(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b4d20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "RBX::BadgeService::awardBadgeResult(int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b4e50(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "void RBX::Reflection::resume_adapter<bool>(boost::function<void ()(RBX::Reflection::Variant)>,bool)")]
pub fn stub_0x3b50bc() -> crate::slot::PortedFn {
// IDA 0x3b50bc: void RBX::Reflection::resume_adapter<bool>(boost::function<void (RBX::Reflection::Variant)>, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3b50bc, "void RBX::Reflection::resume_adapter<bool>(boost::function<void (RBX::Reflection::Variant)>, bool)")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_0x3b5298(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_0x3b52d8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_0x3b5318(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_7<rbx_core::Weak<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::Weak<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::Weak<RBX::BadgeService>,int,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b5484() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "rbx_core::Weak<RBX::BadgeService> RBX::weak_from<RBX::BadgeService>(RBX::BadgeService*)")]
pub fn stub_0x3b5728() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list_av_6<rbx_core::Weak<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,rbx_core::Weak<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),rbx_core::Weak<RBX::BadgeService>,int,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x3b5978() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::BadgeService*,std::string)")]
pub fn stub_0x3b5c18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<1, RBX::BadgeService, void (std::string), rbx::remote~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::BadgeService*,std::string)const")]
pub fn stub_0x3b611c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::BadgeService, void (std::string), rbx::remote_signa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
pub fn stub_0x3b6238(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<1, RBX::BadgeService, void (std::string), rbx::remote~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3b6384() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3b6540() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::storage6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0x3b6700() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x3b6880(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3b6a4c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
pub fn stub_0x3b6a68(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3b6a88(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3b6c44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3b6dfc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
pub fn stub_0x3b6ebc(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3b7054(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list6(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x3b7224() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x3b73a4() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage5(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
pub fn stub_0x3b7550() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x3b7698() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>)")]
pub fn stub_0x3b77b4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>)")]
pub fn stub_0x3b78d0() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3b7b94() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3b7d50() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::storage7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0x3b7f10() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x3b8094(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3b8260(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
pub fn stub_0x3b827c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3b829c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3b8458(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3b8610(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
pub fn stub_0x3b86d0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3b8870(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x3b8a40() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x3b8bc8() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
pub fn stub_0x3b8d7c() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x3b8ec8() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)")]
pub fn stub_0x3b8fe4() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)")]
pub fn stub_0x3b9100() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "rbx_core::Weak<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)")]
pub fn stub_0x3b9224() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(rbx_core::Weak<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x3b9738() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BadgeService")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string)>::remote_signal(void)")]
pub fn stub_0x3b97b4() -> crate::slot::PortedFn {
// IDA 0x3b97b4: rbx::remote_signal<void (std::string)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3b97b4, "rbx::remote_signal<void (std::string)>::remote_signal()")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3b9910(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3b9914(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}
