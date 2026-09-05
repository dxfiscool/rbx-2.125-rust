// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3ab954..0x3af674 | script 22903->23003 distinct (filler 0x3ab954 asc, not-in-script 62843->62743)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x3ab954(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
pub fn stub_0x3aba44() -> crate::lua::LuaVector3 {
// G3D::Vector3 ctor — zero.
crate::lua::LuaVector3 { x: 0.0, y: 0.0, z: 0.0 }
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot() [0x3aba70]")]
pub fn stub_0x3aba70() -> crate::lua::LuaVector3 {
// G3D::Vector3 ctor — zero.
crate::lua::LuaVector3 { x: 0.0, y: 0.0, z: 0.0 }
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
pub fn stub_0x3abb44(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3abb44: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable() [0x3abb70]")]
pub fn stub_0x3abb70(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3abb70: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_0x3abc44(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::ArcHandles, void (G3D::Vector3::Axis)>::connectSignalListene~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::HandlesBase::getHandleType(void)const")]
pub fn stub_0x3abc48(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::HandlesBase getter.
cell.get()
}

#[doc(alias = "RBX::HandlesBase::getHandlesNormalIdMask(void)const")]
pub fn stub_0x3abc4c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::HandlesBase getter.
cell.get()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3abc50(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3abc54(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3abcf4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3abcfc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3abda0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3abda8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3abe4c(handle: crate::slot::InstanceHandle) {
// RBX::HandlesBase dtor.
drop(handle);
}

#[doc(alias = "RBX::HandlesBase::~HandlesBase() [0x3ac098]")]
pub fn stub_0x3ac098(handle: crate::slot::InstanceHandle) {
// RBX::HandlesBase dtor.
drop(handle);
}

#[doc(alias = "RBX::HandlesBase::~HandlesBase() [0x3ac09c]")]
pub fn stub_0x3ac09c(handle: crate::slot::InstanceHandle) {
// RBX::HandlesBase dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
pub fn stub_0x3ac13c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PartAdornment"
}

#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase()")]
pub fn stub_0x3ac140(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase() [0x3ac148]")]
pub fn stub_0x3ac148(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
pub fn stub_0x3ac1ec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"PartAdornment"
}

#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase() [0x3ac1f0]")]
pub fn stub_0x3ac1f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::HandlesBase::~HandlesBase() [0x3ac1f8]")]
pub fn stub_0x3ac1f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")]
pub fn stub_0x3ac29c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sHandlesBase>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")]
pub fn stub_0x3ac2e0() -> crate::slot::PortedFn {
// IDA 0x3ac2e0: void RBX::Name::callDoDeclare<RBX::sHandlesBase>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3ac2e0, "void RBX::Name::callDoDeclare<RBX::sHandlesBase>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")]
pub fn stub_0x3ac2e4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sHandlesBase>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartAdornment::~PartAdornment()")]
pub fn stub_0x3ac3c8(handle: crate::slot::InstanceHandle) {
// RBX::PartAdornment dtor.
drop(handle);
}

#[doc(alias = "RBX::PartAdornment::~PartAdornment() [0x3ac510]")]
pub fn stub_0x3ac510(handle: crate::slot::InstanceHandle) {
// RBX::PartAdornment dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
pub fn stub_0x3ac5b0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase3d"
}

#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment()")]
pub fn stub_0x3ac5b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment() [0x3ac6fc]")]
pub fn stub_0x3ac6fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
pub fn stub_0x3ac858() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"GuiBase3d"
}

#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment() [0x3ac85c]")]
pub fn stub_0x3ac85c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::PartAdornment::~PartAdornment() [0x3ac9a4]")]
pub fn stub_0x3ac9a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")]
pub fn stub_0x3acb00(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sPartAdornment>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")]
pub fn stub_0x3acb44() -> crate::slot::PortedFn {
// IDA 0x3acb44: void RBX::Name::callDoDeclare<RBX::sPartAdornment>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3acb44, "void RBX::Name::callDoDeclare<RBX::sPartAdornment>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")]
pub fn stub_0x3acb48(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sPartAdornment>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ArcHandles>(char const*,char const*,int RBX::ArcHandles::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3acc2c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::isReadOnly(void)const")]
pub fn stub_0x3acdbc() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::isWriteOnly(void)const")]
pub fn stub_0x3acdc0() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3acdc4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ArcHandles>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x3acdd0() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc() [0x3ace20]")]
pub fn stub_0x3ace20(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3aced4() -> crate::slot::SlotConnection {
// IDA 0x3aced4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isBroadcast(void)const")]
pub fn stub_0x3ad040() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ad048(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<3, RBX::ArcHandles, void (G3D::Vector3::Axis, float, float)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ad0f0() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3ad100(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::ArcHandles, void (G3D::Vector3::Axis, float, float), r~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x3ad114() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<G3D::Vector3::Axis,float,float>(G3D::Vector3::Axis const&,float const&,float const&)")]
pub fn stub_0x3ad230() -> crate::slot::PortedFn {
// IDA 0x3ad230: void RBX::Reflection::GenericSlotWrapper::execute3<G3D::Vector3::Axis, float, float>(G3D::Vector3::Axis const&, float co~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3ad230, "void RBX::Reflection::GenericSlotWrapper::execute3<G3D::Vector3::Axis, float, float>(G3D::Vector3::A~")
}

#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::clear(void)")]
pub fn stub_0x3ad3bc(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvN3G3D7Vector34AxisEffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSH_EENS7_5list4INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3ad3e8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IvN3G3D7Vector34AxisEffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKS3_RKfSG_EENS6_5list4INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3ad4cc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x3ad5b4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3ad6ac(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,G3D::Vector3::Axis,float,float>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3ad6c8(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3ad6f0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3ad7d8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,G3D::Vector3::Axis,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3ad8bc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_0x3ad990(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3ad990: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3ad9b8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::function<void ()(G3D::Vector3::Axis,float,float)>>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&)")]
pub fn stub_0x3adb10() -> crate::slot::SlotConnection {
// IDA 0x3adb10: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*>(boost::function<void ()(G3D::Vector3::Axis,float,float)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>*)")]
pub fn stub_0x3adc04(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3adc04: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot()")]
pub fn stub_0x3add00(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis,float,float)>>::~callable_slot() [0x3ade10]")]
pub fn stub_0x3ade10(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3adf40(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3adf40: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3adf48(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3adf48: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::operator()(G3D::Vector3::Axis,float,float)const")]
pub fn stub_0x3adf50(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_0x3ae028(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ae028: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::function<void ()(G3D::Vector3::Axis,float,float)>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable() [0x3ae138]")]
pub fn stub_0x3ae138(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ae138: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,G3D::Vector3::Axis,float,float>::assign_to_own(boost::function3<void,G3D::Vector3::Axis,float,float> const&)")]
pub fn stub_0x3ae268(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::EventDesc(rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3ae298() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc()")]
pub fn stub_0x3ae4f4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>,rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)> RBX::ArcHandles::*>::~EventDesc() [0x3ae518]")]
pub fn stub_0x3ae518(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc() [0x3ae5cc]")]
pub fn stub_0x3ae5cc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3ae680() -> crate::slot::SlotConnection {
// IDA 0x3ae680: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isBroadcast(void)const")]
pub fn stub_0x3ae7ec() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ae7f4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::ArcHandles, void (G3D::Vector3::Axis), rbx::remote_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ae880() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>,rbx::remote_signal<void ()(G3D::Vector3::Axis)> RBX::ArcHandles::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3ae890(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::ArcHandles, void (G3D::Vector3::Axis), rbx::remote_sig~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3::Axis const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
pub fn stub_0x3ae8a4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")]
pub fn stub_0x3ae9c0() -> crate::slot::PortedFn {
// IDA 0x3ae9c0: void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3ae9c0, "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")
}

#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::clear(void)")]
pub fn stub_0x3aeb04(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvN3G3D7Vector34AxisEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3aeb30() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvN3G3D7Vector34AxisEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS3_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3aec14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
pub fn stub_0x3aecfc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3aedf4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3::Axis>::invoke(boost::detail::function::function_buffer &,G3D::Vector3::Axis)")]
pub fn stub_0x3aee10(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3aee24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3aef0c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,G3D::Vector3::Axis>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3aeff0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
pub fn stub_0x3af0c4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3::Axis const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3af0dc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::function<void ()(G3D::Vector3::Axis)>>(boost::function<void ()(G3D::Vector3::Axis)> const&)")]
pub fn stub_0x3af234() -> crate::slot::SlotConnection {
// IDA 0x3af234: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>*>(boost::function<void ()(G3D::Vector3::Axis)> const&,rbx::signals::signal<void ()(G3D::Vector3::Axis)>*)")]
pub fn stub_0x3af328(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3af328: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot()")]
pub fn stub_0x3af424(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::function<void ()(G3D::Vector3::Axis)>>::~callable_slot() [0x3af534]")]
pub fn stub_0x3af534(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_0x3af664(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3af664: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::function<void ()(G3D::Vector3::Axis)>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_0x3af66c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3af66c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function1<void,G3D::Vector3::Axis>::operator()(G3D::Vector3::Axis)const")]
pub fn stub_0x3af674(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}
