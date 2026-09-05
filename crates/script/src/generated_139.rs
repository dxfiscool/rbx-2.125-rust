// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Script|Lua|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2d08a4..0x2e8704 | 2231->2331 covered, 3070 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv")]
pub fn stub_0x2d08a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator6createEv")]
pub fn stub_0x2d092c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::AdvLuaDragger>(void)")]
pub fn stub_0x2d0a70() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::shared_ptr<RBX::AdvLuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2d0b20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AdvLuaDragger,RBX::AdvLuaDragger>(rbx_core::SharedPtr<RBX::AdvLuaDragger> const*,RBX::AdvLuaDragger *)const")]
pub fn stub_0x2d0be8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragger")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2d0cd0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2d0dd8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2d0ddc]")]
pub fn stub_0x2d0ddc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2d0de0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2d0e00() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2d0e18() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv")]
pub fn stub_0x2d0e1c() -> crate::slot::PortedFn {
// IDA 0x2d0e1c: void RBX::Name::callDoDeclare<RBX::sAdvLuaDragger>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2d0e1c, "void RBX::Name::callDoDeclare<RBX::sAdvLuaDragger>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")]
pub fn stub_0x2d0e20(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAdvLuaDragger>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev")]
pub fn stub_0x2d0f00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv")]
pub fn stub_0x2d1144() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"AdvLuaDragger"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d1260(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d1264(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d1304(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d130c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d13b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d13b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2d17c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>) [0x2d17c8]")]
pub fn stub_0x2d17c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool()")]
pub fn stub_0x2d1a5c(handle: crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool() [0x2d1afc]")]
pub fn stub_0x2d1afc(handle: crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragTool::~AdvLuaDragTool()")]
pub fn stub_0x2d1b00(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AdvLuaDragTool::~AdvLuaDragTool() [0x2d1b08]")]
pub fn stub_0x2d1b08(handle: crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragTool::~AdvLuaDragTool() [0x2d1c48]")]
pub fn stub_0x2d1c48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AdvLuaDragTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2d1c50(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragTool::onMouseMove(RBX::UIEvent const&)")]
pub fn stub_0x2d1e34(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool::onMouseMove(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x2d1edc(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool::onMouseIdle(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2d1f5c(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvLuaDragTool::onKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x2d21d8(handle: &crate::slot::InstanceHandle) {
// RBX::AdvLuaDragTool::onKeyDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragger>::operator=(rbx_core::SharedPtr<RBX::AdvLuaDragger> const&)")]
pub fn stub_0x2d2374(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::shared_from<RBX::AdvLuaDragTool>(RBX::AdvLuaDragTool*)")]
pub fn stub_0x2d23ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvLuaDragTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_16AdvArrowToolBaseELZNS_15sAdvLuaDragToolEEE7getNameEv")]
pub fn stub_0x2d2514(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::AdvLuaDragTool::getCursorName(void)const")]
pub fn stub_0x2d253c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvLuaDragTool getter.
cell.get()
}

#[doc(alias = "RBX::AdvLuaDragTool::setCursor(std::string)")]
pub fn stub_0x2d2548(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::AdvLuaDragTool setter.
cell.set(value)
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAdvLuaDragToolEEEEvv")]
pub fn stub_0x2d2550() -> crate::slot::PortedFn {
// IDA 0x2d2550: void RBX::Name::callDoDeclare<RBX::sAdvLuaDragTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2d2550, "void RBX::Name::callDoDeclare<RBX::sAdvLuaDragTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAdvLuaDragToolEEEERKS0_v")]
pub fn stub_0x2d2554(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAdvLuaDragTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2e08bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool>::shared_ptr<RBX::LuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x2e09ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LuaDragTool,RBX::LuaDragTool>(rbx_core::SharedPtr<RBX::LuaDragTool> const*,RBX::LuaDragTool *)const")]
pub fn stub_0x2e0ab4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x2e0b98() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2e0c90(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x2e0c94]")]
pub fn stub_0x2e0c94(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x2e0c98() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2e0ca8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2e0cc0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::LuaDragger::mouseDownPublic(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
pub fn stub_0x2e51d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::LuaDragger::mouseMove(RBX::RbxRay)")]
pub fn stub_0x2e56d4(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::mouseMove(RBX::RbxRay) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaDragger::mouseUp(void)")]
pub fn stub_0x2e59b4(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::mouseUp() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)")]
pub fn stub_0x2e5b88(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::axisRotate(G3D::Vector3::Axis) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaDragger::LuaDragger(void)")]
pub fn stub_0x2e5c24() -> crate::slot::InstanceHandle {
// RBX::LuaDragger ctor.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "RBX::LuaDragger::~LuaDragger()")]
pub fn stub_0x2e5e10(handle: crate::slot::InstanceHandle) {
// RBX::LuaDragger dtor.
drop(handle);
}

#[doc(alias = "RBX::LuaDragger::~LuaDragger() [0x2e5eb0]")]
pub fn stub_0x2e5eb0(handle: crate::slot::InstanceHandle) {
// RBX::LuaDragger dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger()")]
pub fn stub_0x2e5eb4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger() [0x2e5ebc]")]
pub fn stub_0x2e5ebc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::LuaDragger::~LuaDragger() [0x2e5ec4]")]
pub fn stub_0x2e5ec4(handle: crate::slot::InstanceHandle) {
// RBX::LuaDragger dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger() [0x2e6060]")]
pub fn stub_0x2e6060(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::LuaDragger::~LuaDragger() [0x2e6068]")]
pub fn stub_0x2e6068(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::LuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<Weak<RBX::PartInstance>,std::allocator<Weak<RBX::PartInstance>>>)")]
pub fn stub_0x2e6070() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::LuaDragger::tryStartDragging(RBX::RbxRay const&)")]
pub fn stub_0x2e6314(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::tryStartDragging(RBX::RbxRay const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaDragger::doDrag(RBX::RbxRay const&)")]
pub fn stub_0x2e6524(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::doDrag(RBX::RbxRay const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
pub fn stub_0x2e67a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::LuaDragger getter.
cell.get()
}

#[doc(alias = "RBX::LuaDragger::startDragging(void)")]
pub fn stub_0x2e68c8(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::startDragging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
pub fn stub_0x2e6b88(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::rotateOnSnapFace(G3D::Vector3::Axis, G3D::Matrix3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
pub fn stub_0x2e700c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(RBX::RbxRay),1>::~BoundFuncDesc()")]
pub fn stub_0x2e7010(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x2e7108(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
pub fn stub_0x2e712c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::LuaDragger::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x2e74dc(handle: &crate::slot::InstanceHandle) {
// RBX::LuaDragger::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv")]
pub fn stub_0x2e74e0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E12getClassNameEv")]
pub fn stub_0x2e74f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD1Ev")]
pub fn stub_0x2e7500() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorD2Ev")]
pub fn stub_0x2e7504() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator12getClassNameEv")]
pub fn stub_0x2e75a0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7Creator6createEv")]
pub fn stub_0x2e7628() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger> RBX::Creatable<RBX::Instance>::create<RBX::LuaDragger>(void)")]
pub fn stub_0x2e776c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragger>::shared_ptr<RBX::LuaDragger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2e781c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LuaDragger,RBX::LuaDragger>(rbx_core::SharedPtr<RBX::LuaDragger> const*,RBX::LuaDragger *)const")]
pub fn stub_0x2e78e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaDragger")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2e79cc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2e7ad4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2e7ad8]")]
pub fn stub_0x2e7ad8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2e7adc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2e7afc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaDragger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2e7b14() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sLuaDraggerEEEEvv")]
pub fn stub_0x2e7b18() -> crate::slot::PortedFn {
// IDA 0x2e7b18: void RBX::Name::callDoDeclare<RBX::sLuaDragger>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2e7b18, "void RBX::Name::callDoDeclare<RBX::sLuaDragger>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sLuaDraggerEEEERKS0_v")]
pub fn stub_0x2e7b1c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLuaDragger>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E7CreatorC2Ev")]
pub fn stub_0x2e7bfc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10LuaDraggerENS_8InstanceELZNS_11sLuaDraggerEES2_E17static_getCreatorEv")]
pub fn stub_0x2e7e40() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaDragger"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2e832c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2e8330(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2e83d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2e83d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2e847c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2e8484(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x2e8528() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x2e86d4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::LuaDragger", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc() [0x2e8704]")]
pub fn stub_0x2e8704(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}
