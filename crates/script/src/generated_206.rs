// Auto-generated skeletons for rbx-script — shard 206 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x2e81b8..0x2f71d4 | script 17691->17841 distinct (filler 0x2e81b8 asc, not-in-script 67854->67704)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::~auto_ptr()")]
pub fn stub_0x2e81b8() -> crate::slot::PortedFn {
// IDA 0x2e81b8: std::auto_ptr<RBX::RunDragger>::~auto_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2e81b8, "std::auto_ptr<RBX::RunDragger>::~auto_ptr()")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Joint>,std::allocator<rbx_core::SharedPtr<RBX::Joint>>>::~vector()")]
pub fn stub_0x2e8260(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x2e880c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")]
pub fn stub_0x2e899c() -> crate::slot::PortedFn {
// IDA 0x2e899c: bool RBX::Reflection::ArgHelper::try_enum<1, G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments&, G3D::V~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2e899c, "bool RBX::Reflection::ArgHelper::try_enum<1, G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor~")
}

#[doc(alias = "G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x2e94dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x2e96ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector*<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector*<rbx_core::Weak<RBX::PartInstance>,std::allocator<RBX::PartInstance>>>>>)")]
pub fn stub_0x2e9870() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>> *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>> *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>> *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_0x2e98b8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2e98b8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "global constructor keyed to_a_95")]
pub fn stub_0x2e9b08() -> crate::slot::PortedFn {
// IDA 0x2e9b08: __GLOBAL__I_a_95.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2e9b08, "__GLOBAL__I_a_95")
}

#[doc(alias = "global constructor keyed to_a_96")]
pub fn stub_0x2eac30() -> crate::slot::PortedFn {
// IDA 0x2eac30: __GLOBAL__I_a_96.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2eac30, "__GLOBAL__I_a_96")
}

#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
pub fn stub_0x2eaea0() -> crate::slot::InstanceHandle {
// RBX::MegaDragger ctor.
crate::slot::InstanceHandle::new("RBX::MegaDragger")
}

#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<RBX::PVInstance *,std::allocator<RBX::PVInstance *>> const&,RBX::RootInstance *,RBX::DRAG::JoinType) [0x2eaea4]")]
pub fn stub_0x2eaea4() -> crate::slot::InstanceHandle {
// RBX::MegaDragger ctor.
crate::slot::InstanceHandle::new("RBX::MegaDragger")
}

#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType)")]
pub fn stub_0x2eafd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::MegaDragger::MegaDragger(RBX::PartInstance *,std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>> const&,RBX::RootInstance *,RBX::DRAG::JoinType) [0x2eafd8]")]
pub fn stub_0x2eafd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::MegaDragger::~MegaDragger()")]
pub fn stub_0x2eb0e8(handle: crate::slot::InstanceHandle) {
// RBX::MegaDragger dtor.
drop(handle);
}

#[doc(alias = "RBX::MegaDragger::~MegaDragger() [0x2eb0ec]")]
pub fn stub_0x2eb0ec(handle: crate::slot::InstanceHandle) {
// RBX::MegaDragger dtor.
drop(handle);
}

#[doc(alias = "RBX::MegaDragger::startDragging(void)")]
pub fn stub_0x2eb224(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::startDragging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::continueDragging(void)")]
pub fn stub_0x2eb248(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::continueDragging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::finishDragging(void)")]
pub fn stub_0x2eb2b4(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::finishDragging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::alignAndCleanParts(void)")]
pub fn stub_0x2eb380(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::alignAndCleanParts() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::mousePartAlive(void)")]
pub fn stub_0x2eb540(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::mousePartAlive() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::getPartsForDrag(G3D::Array<RBX::Primitive *,10,32ul> &)")]
pub fn stub_0x2eb680(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MegaDragger getter.
cell.get()
}

#[doc(alias = "RBX::MegaDragger::rotateDragParts(G3D::Matrix3 const&,bool)")]
pub fn stub_0x2ebf24(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::rotateDragParts(G3D::Matrix3 const&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaDragger::anyDragPartAlive(void)")]
pub fn stub_0x2ebf7c(handle: &crate::slot::InstanceHandle) {
// RBX::MegaDragger::anyDragPartAlive() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_97")]
pub fn stub_0x2ebf88() -> crate::slot::PortedFn {
// IDA 0x2ebf88: __GLOBAL__I_a_97.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2ebf88, "__GLOBAL__I_a_97")
}

#[doc(alias = "RBX::MoveResizeJoinTool::findTargetPV(RBX::UIEvent const&)")]
pub fn stub_0x2ec2fc(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::findTargetPV(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2ec558(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2ec7e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "RBX::MoveResizeJoinTool::render2d(RBX::Adorn *)")]
pub fn stub_0x2ec7ec(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::render2d(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::MoveResizeJoinTool::render2d(RBX::Adorn *)")]
pub fn stub_0x2ed9d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_0x2ed9dc(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::onMouseHover(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x2eda60(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::onMouseIdle(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2edbcc(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::moveIncrement(void)")]
pub fn stub_0x2ede04(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::moveIncrement() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::onKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x2edf9c(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::onKeyDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseMove(RBX::UIEvent const&)")]
pub fn stub_0x2ee084(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::onMouseMove(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::capturedDrag(float)")]
pub fn stub_0x2ee324(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::capturedDrag(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2ee4e4(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*)")]
pub fn stub_0x2ee6b0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MoveResizeJoinTool")
}

#[doc(alias = "RBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
pub fn stub_0x2ee818(handle: crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool dtor.
drop(handle);
}

#[doc(alias = "RBX::MoveResizeJoinTool::~MoveResizeJoinTool() [0x2ee900]")]
pub fn stub_0x2ee900(handle: crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
pub fn stub_0x2ee9f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::MoveResizeJoinTool::~MoveResizeJoinTool() [0x2eeadc]")]
pub fn stub_0x2eeadc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_98")]
pub fn stub_0x2eebd8() -> crate::slot::PortedFn {
// IDA 0x2eebd8: __GLOBAL__I_a_98.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2eebd8, "__GLOBAL__I_a_98")
}

#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
pub fn stub_0x2eee88() -> crate::slot::InstanceHandle {
// RBX::NullTool ctor.
crate::slot::InstanceHandle::new("RBX::NullTool")
}

#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *) [0x2eee8c]")]
pub fn stub_0x2eee8c() -> crate::slot::InstanceHandle {
// RBX::NullTool ctor.
crate::slot::InstanceHandle::new("RBX::NullTool")
}

#[doc(alias = "RBX::NullTool::~NullTool()")]
pub fn stub_0x2eef84(handle: crate::slot::InstanceHandle) {
// RBX::NullTool dtor.
drop(handle);
}

#[doc(alias = "RBX::NullTool::~NullTool() [0x2ef024]")]
pub fn stub_0x2ef024(handle: crate::slot::InstanceHandle) {
// RBX::NullTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::NullTool::~NullTool()")]
pub fn stub_0x2ef028(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::NullTool::~NullTool() [0x2ef030]")]
pub fn stub_0x2ef030(handle: crate::slot::InstanceHandle) {
// RBX::NullTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::NullTool::~NullTool() [0x2ef124]")]
pub fn stub_0x2ef124(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
pub fn stub_0x2ef12c() -> crate::slot::InstanceHandle {
// RBX::NewNullTool ctor.
crate::slot::InstanceHandle::new("RBX::NewNullTool")
}

#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *) [0x2ef130]")]
pub fn stub_0x2ef130() -> crate::slot::InstanceHandle {
// RBX::NewNullTool ctor.
crate::slot::InstanceHandle::new("RBX::NewNullTool")
}

#[doc(alias = "RBX::NewNullTool::~NewNullTool()")]
pub fn stub_0x2ef22c(handle: crate::slot::InstanceHandle) {
// RBX::NewNullTool dtor.
drop(handle);
}

#[doc(alias = "RBX::NewNullTool::~NewNullTool() [0x2ef2f4]")]
pub fn stub_0x2ef2f4(handle: crate::slot::InstanceHandle) {
// RBX::NewNullTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::NewNullTool::~NewNullTool()")]
pub fn stub_0x2ef328(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::NewNullTool::~NewNullTool() [0x2ef330]")]
pub fn stub_0x2ef330(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)")]
pub fn stub_0x2ef364(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NewNullTool getter.
cell.get()
}

#[doc(alias = "RBX::NewNullTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x2ef48c(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::onMouseIdle(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::updateClickDetectorHover(RBX::UIEvent const&)")]
pub fn stub_0x2ef694(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::updateClickDetectorHover(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_0x2ef888(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::onMouseHover(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::onRightMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2efb14(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::onRightMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2efc0c(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::onRightMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2efd44(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::onRightMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2efef0(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::NewNullTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2efef4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*)")]
pub fn stub_0x2efef8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::NewNullTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv")]
pub fn stub_0x2f0060(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::NullTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2f0088(handle: &crate::slot::InstanceHandle) {
// RBX::NullTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NullTool::isSticky(void)const")]
pub fn stub_0x2f015c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NullTool getter.
cell.get()
}

#[doc(alias = "RBX::NullTool::getCursorName(void)const")]
pub fn stub_0x2f0224(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NullTool getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv")]
pub fn stub_0x2f0240(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::NewNullTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2f0268(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::NewNullTool::isSticky(void)const")]
pub fn stub_0x2f033c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NewNullTool getter.
cell.get()
}

#[doc(alias = "RBX::NewNullTool::getCursorName(void)const")]
pub fn stub_0x2f0404(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NewNullTool getter.
cell.get()
}

#[doc(alias = "RBX::NewNullTool::shouldRender3dAdorn(void)const")]
pub fn stub_0x2f0410(handle: &crate::slot::InstanceHandle) {
// RBX::NewNullTool::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::NewNullTool::shouldRender3dAdorn(void)const")]
pub fn stub_0x2f0414(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv")]
pub fn stub_0x2f0418() -> crate::slot::PortedFn {
// IDA 0x2f0418: void RBX::Name::callDoDeclare<RBX::sNewNullTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2f0418, "void RBX::Name::callDoDeclare<RBX::sNewNullTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")]
pub fn stub_0x2f041c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sNewNullTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*)")]
pub fn stub_0x2f04fc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::NullTool")
}

#[doc(alias = "global constructor keyed to_a_99")]
pub fn stub_0x2f0664() -> crate::slot::PortedFn {
// IDA 0x2f0664: __GLOBAL__I_a_99.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2f0664, "__GLOBAL__I_a_99")
}

#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2f0948() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>) [0x2f094c]")]
pub fn stub_0x2f094c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::PartDragTool::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2f0bb8(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::onMouseMove(RBX::UIEvent const&)")]
pub fn stub_0x2f0cb0(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::onMouseMove(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&)")]
pub fn stub_0x2f0d60(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x2f0ecc(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2f0f68(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::onKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x2f1134(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::onKeyDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2f12c0(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2f13d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
pub fn stub_0x2f13d8(handle: crate::slot::InstanceHandle) {
// RBX::PartDragTool dtor.
drop(handle);
}

#[doc(alias = "RBX::PartDragTool::~PartDragTool() [0x2f1478]")]
pub fn stub_0x2f1478(handle: crate::slot::InstanceHandle) {
// RBX::PartDragTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
pub fn stub_0x2f147c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::PartDragTool::~PartDragTool() [0x2f1484]")]
pub fn stub_0x2f1484(handle: crate::slot::InstanceHandle) {
// RBX::PartDragTool dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::PartDragTool::~PartDragTool() [0x2f15e4]")]
pub fn stub_0x2f15e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::shared_from<RBX::PartDragTool>(RBX::PartDragTool*)")]
pub fn stub_0x2f15ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartDragTool")
}

#[doc(alias = "RBX::MegaDragger::getMousePart(void)")]
pub fn stub_0x2f1754(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MegaDragger getter.
cell.get()
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv")]
pub fn stub_0x2f1808(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::PartDragTool::drawConnectors(void)const")]
pub fn stub_0x2f1830(handle: &crate::slot::InstanceHandle) {
// RBX::PartDragTool::drawConnectors() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartDragTool::getCursorName(void)const")]
pub fn stub_0x2f1834(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PartDragTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv")]
pub fn stub_0x2f1864() -> crate::slot::PortedFn {
// IDA 0x2f1864: void RBX::Name::callDoDeclare<RBX::sPartDragTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2f1864, "void RBX::Name::callDoDeclare<RBX::sPartDragTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")]
pub fn stub_0x2f1868(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sPartDragTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_100")]
pub fn stub_0x2f1948() -> crate::slot::PortedFn {
// IDA 0x2f1948: __GLOBAL__I_a_100.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2f1948, "__GLOBAL__I_a_100")
}

#[doc(alias = "global constructor keyed to_a_101")]
pub fn stub_0x2f1c20() -> crate::slot::PortedFn {
// IDA 0x2f1c20: __GLOBAL__I_a_101.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2f1c20, "__GLOBAL__I_a_101")
}

#[doc(alias = "RBX::RunDragger::SnapInfo::updateSurfaceFromHit(void)")]
pub fn stub_0x2f1ef8(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::SnapInfo::updateSurfaceFromHit() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&)")]
pub fn stub_0x2f1fb8(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::SnapInfo::hitOutsideExtents(void)")]
pub fn stub_0x2f229c(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::SnapInfo::hitOutsideExtents() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::RunDragger(void)")]
pub fn stub_0x2f23f0() -> crate::slot::InstanceHandle {
// RBX::RunDragger ctor.
crate::slot::InstanceHandle::new("RBX::RunDragger")
}

#[doc(alias = "RBX::RunDragger::RunDragger(void) [0x2f23f4]")]
pub fn stub_0x2f23f4() -> crate::slot::InstanceHandle {
// RBX::RunDragger ctor.
crate::slot::InstanceHandle::new("RBX::RunDragger")
}

#[doc(alias = "RBX::RunDragger::~RunDragger()")]
pub fn stub_0x2f25ac(handle: crate::slot::InstanceHandle) {
// RBX::RunDragger dtor.
drop(handle);
}

#[doc(alias = "RBX::RunDragger::~RunDragger() [0x2f25b0]")]
pub fn stub_0x2f25b0(handle: crate::slot::InstanceHandle) {
// RBX::RunDragger dtor.
drop(handle);
}

#[doc(alias = "RBX::RunDragger::snapInfoFromSnapPart(void)")]
pub fn stub_0x2f26a8(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::snapInfoFromSnapPart() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::snapPartFromSnapInfo(void)")]
pub fn stub_0x2f2a54(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::snapPartFromSnapInfo() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::initLocal(RBX::Workspace *,rbx_core::Weak<RBX::PartInstance>,G3D::Vector3 const&)")]
pub fn stub_0x2f2bf0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::RunDragger::turnUpright(RBX::PartInstance *)")]
pub fn stub_0x2f2f3c(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::turnUpright(RBX::PartInstance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::init(RBX::Workspace *,rbx_core::Weak<RBX::PartInstance>,G3D::Vector3 const&)")]
pub fn stub_0x2f2ff8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::RunDragger::moveDragPart(void)")]
pub fn stub_0x2f37c8(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::moveDragPart() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::getSnapSurfaceCoord(void)")]
pub fn stub_0x2f41c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RunDragger getter.
cell.get()
}

#[doc(alias = "RBX::RunDragger::snapDragPart(void)")]
pub fn stub_0x2f4340(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::snapDragPart() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")]
pub fn stub_0x2f46c0(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::adjacent(RBX::Primitive*, RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)")]
pub fn stub_0x2f4700(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::rayHitsPart(G3D::Array<RBX::Primitive*, 10, 32ul> const&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))")]
pub fn stub_0x2f495c(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::bestProximatePart(G3D::Array<RBX::Primitive*, 10, 32ul> const&, bool (RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::fallOffEdge(void)")]
pub fn stub_0x2f4ae0(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::fallOffEdge() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::fallOffPart(bool &)")]
pub fn stub_0x2f4b14(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::fallOffPart(bool&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::rayHitsCloserPart(void)")]
pub fn stub_0x2f4c88(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::rayHitsCloserPart() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::tooCloseToCamera(void)")]
pub fn stub_0x2f4dd8(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::tooCloseToCamera() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::findSafeY(void)")]
pub fn stub_0x2f5168(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::findSafeY() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)")]
pub fn stub_0x2f53f4(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")]
pub fn stub_0x2f5410(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis, float const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunDragger::snap(RBX::RbxRay const&)")]
pub fn stub_0x2f5610(handle: &crate::slot::InstanceHandle) {
// RBX::RunDragger::snap(RBX::RbxRay const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_102")]
pub fn stub_0x2f5d3c() -> crate::slot::PortedFn {
// IDA 0x2f5d3c: __GLOBAL__I_a_102.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2f5d3c, "__GLOBAL__I_a_102")
}

#[doc(alias = "RBX::ArrowToolBase::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_0x2f614c(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::onMouseHover(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArrowToolBase::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x2f6154(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::onMouseIdle(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArrowToolBase::getCursorName(void)const")]
pub fn stub_0x2f6190(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ArrowToolBase getter.
cell.get()
}

#[doc(alias = "RBX::ArrowToolBase::findDecal(RBX::PartInstance *,RBX::UIEvent const&)")]
pub fn stub_0x2f61c0(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::findDecal(RBX::PartInstance*, RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2f6254(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArrowToolBase::onPeekKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x2f6610(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::onPeekKeyDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2f6850(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArrowToolBase::renderHoverOver(RBX::Adorn *,bool)")]
pub fn stub_0x2f6858(handle: &crate::slot::InstanceHandle) {
// RBX::ArrowToolBase::renderHoverOver(RBX::Adorn*, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x2f68c8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "RBX::AdvArrowToolBase::getCursorName(void)const")]
pub fn stub_0x2f68d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvArrowToolBase getter.
cell.get()
}

#[doc(alias = "RBX::AdvArrowToolBase::onKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x2f6900(handle: &crate::slot::InstanceHandle) {
// RBX::AdvArrowToolBase::onKeyDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2f6954(handle: &crate::slot::InstanceHandle) {
// RBX::AdvArrowToolBase::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvArrowToolBase::onMouseMove(RBX::UIEvent const&)")]
pub fn stub_0x2f6d04(handle: &crate::slot::InstanceHandle) {
// RBX::AdvArrowToolBase::onMouseMove(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvArrowToolBase::determineManualJointConditions(void)")]
pub fn stub_0x2f6d18(handle: &crate::slot::InstanceHandle) {
// RBX::AdvArrowToolBase::determineManualJointConditions() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvArrowToolBase::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x2f6fb8(handle: &crate::slot::InstanceHandle) {
// RBX::AdvArrowToolBase::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)")]
pub fn stub_0x2f6ff4() -> crate::slot::InstanceHandle {
// RBX::BoxSelectCommand ctor.
crate::slot::InstanceHandle::new("RBX::BoxSelectCommand")
}

#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
pub fn stub_0x2f7134(handle: crate::slot::InstanceHandle) {
// RBX::BoxSelectCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand() [0x2f71d4]")]
pub fn stub_0x2f71d4(handle: crate::slot::InstanceHandle) {
// RBX::BoxSelectCommand dtor.
drop(handle);
}
