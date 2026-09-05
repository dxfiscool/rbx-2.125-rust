// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x410014..0x413640 | script 24452->24552 distinct (filler 0x410014 asc, not-in-script 61293->61193)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x410014(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::GlueTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlueTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GlueTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4100e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GlueTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv")]
pub fn stub_0x4101b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::GlueTool::isSticky(void)const")]
pub fn stub_0x4101bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GlueTool getter.
cell.get()
}

#[doc(alias = "RBX::GlueTool::getCursorName(void)const")]
pub fn stub_0x410284(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GlueTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v")]
pub fn stub_0x410578(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sGlueTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv")]
pub fn stub_0x4105bc() -> crate::slot::PortedFn {
// IDA 0x4105bc: void RBX::Name::callDoDeclare<RBX::sGlueTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4105bc, "void RBX::Name::callDoDeclare<RBX::sGlueTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v")]
pub fn stub_0x4105c0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sGlueTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x4106a4() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::~TToolVerb() [0x410828]")]
pub fn stub_0x410828(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x4108c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x410900(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::FlatTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x410a14(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::FlatTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlatTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FlatTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x410ae0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FlatTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv")]
pub fn stub_0x410bb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::FlatTool::isSticky(void)const")]
pub fn stub_0x410bbc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FlatTool getter.
cell.get()
}

#[doc(alias = "RBX::FlatTool::getCursorName(void)const")]
pub fn stub_0x410c84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FlatTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v")]
pub fn stub_0x410f78(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sFlatTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv")]
pub fn stub_0x410fbc() -> crate::slot::PortedFn {
// IDA 0x410fbc: void RBX::Name::callDoDeclare<RBX::sFlatTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x410fbc, "void RBX::Name::callDoDeclare<RBX::sFlatTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v")]
pub fn stub_0x410fc0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFlatTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x4110a4() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::~TToolVerb() [0x411228]")]
pub fn stub_0x411228(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x4112c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x4112fc(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AdvArrowTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x411410(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AdvArrowTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x4114dc() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::~TToolVerb() [0x411660]")]
pub fn stub_0x411660(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x411700(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x411738(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AdvRotateTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x41184c(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AdvRotateTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvRotateTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x411918() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvRotateTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv")]
pub fn stub_0x4119fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::AdvRotateTool::isSticky(void)const")]
pub fn stub_0x411a00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvRotateTool getter.
cell.get()
}

#[doc(alias = "RBX::AdvMoveToolBase::drawConnectors(void)const")]
pub fn stub_0x411ac8(handle: &crate::slot::InstanceHandle) {
// RBX::AdvMoveToolBase::drawConnectors() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AdvMoveToolBase::getCursorName(void)const")]
pub fn stub_0x411acc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvMoveToolBase getter.
cell.get()
}

#[doc(alias = "RBX::AdvMoveToolBase::setCursor(std::string)")]
pub fn stub_0x411ad8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::AdvMoveToolBase setter.
cell.set(value)
}

#[doc(alias = "RBX::AdvRotateTool::getHandleColor(void)const")]
pub fn stub_0x411ae0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvRotateTool getter.
cell.get()
}

#[doc(alias = "RBX::AdvRotateTool::getDragType(void)const")]
pub fn stub_0x411af8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvRotateTool getter.
cell.get()
}

#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
pub fn stub_0x411afc(handle: crate::slot::InstanceHandle) {
// RBX::AdvMoveToolBase dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase() [0x411c14]")]
pub fn stub_0x411c14(handle: crate::slot::InstanceHandle) {
// RBX::AdvMoveToolBase dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase() [0x411c18]")]
pub fn stub_0x411c18(handle: crate::slot::InstanceHandle) {
// RBX::AdvMoveToolBase dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()")]
pub fn stub_0x411cb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase() [0x411cc0]")]
pub fn stub_0x411cc0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::~auto_ptr()")]
pub fn stub_0x411cc8() -> crate::slot::PortedFn {
// IDA 0x411cc8: std::auto_ptr<RBX::MegaDragger>::~auto_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x411cc8, "std::auto_ptr<RBX::MegaDragger>::~auto_ptr()")
}

#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::PartInstance>,std::pair<rbx_core::Weak<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::Weak<RBX::PartInstance> const,float>>,std::less<rbx_core::Weak<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::Weak<RBX::PartInstance> const,float>>>::_M_erase(std::_Rb_tree_node<std::pair<rbx_core::Weak<RBX::PartInstance> const,float>> *)")]
pub fn stub_0x411d70() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::PartInstance>,std::pair<rbx_core::Weak<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::Weak<RBX::PartInstance> const,float>>,std::less<rbx_core::Weak<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::Weak<RBX::PartInstance> const,float>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<rbx_core::Weak<RBX::PartInstance> const,float>> *)")]
pub fn stub_0x411d98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool>::shared_ptr<RBX::AdvRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x411db4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvRotateTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvRotateTool,RBX::AdvRotateTool>(rbx_core::SharedPtr<RBX::AdvRotateTool> const*,RBX::AdvRotateTool *)const")]
pub fn stub_0x411e7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvRotateTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x411f60() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x412058(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x41205c]")]
pub fn stub_0x41205c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x412060() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x412070() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x412088() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v")]
pub fn stub_0x41208c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sAdvRotateTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv")]
pub fn stub_0x4120d0() -> crate::slot::PortedFn {
// IDA 0x4120d0: void RBX::Name::callDoDeclare<RBX::sAdvRotateTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4120d0, "void RBX::Name::callDoDeclare<RBX::sAdvRotateTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v")]
pub fn stub_0x4120d4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAdvRotateTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x4121b8() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::~TToolVerb() [0x41233c]")]
pub fn stub_0x41233c(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x4123dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x412414(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AdvMoveTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x412528(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AdvMoveTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvMoveTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4125f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvMoveTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv")]
pub fn stub_0x4126e4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool()")]
pub fn stub_0x4126e8(handle: crate::slot::InstanceHandle) {
// RBX::AdvMoveTool dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool() [0x4126ec]")]
pub fn stub_0x4126ec(handle: crate::slot::InstanceHandle) {
// RBX::AdvMoveTool dtor.
drop(handle);
}

#[doc(alias = "RBX::AdvMoveTool::isSticky(void)const")]
pub fn stub_0x41278c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvMoveTool getter.
cell.get()
}

#[doc(alias = "RBX::AdvMoveTool::getHandleColor(void)const")]
pub fn stub_0x412854(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvMoveTool getter.
cell.get()
}

#[doc(alias = "RBX::AdvMoveTool::getDragType(void)const")]
pub fn stub_0x41286c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AdvMoveTool getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()")]
pub fn stub_0x412870(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool() [0x412878]")]
pub fn stub_0x412878(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool>::shared_ptr<RBX::AdvMoveTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x412880() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvMoveTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvMoveTool,RBX::AdvMoveTool>(rbx_core::SharedPtr<RBX::AdvMoveTool> const*,RBX::AdvMoveTool *)const")]
pub fn stub_0x412948() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvMoveTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x412a2c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x412b24(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x412b28]")]
pub fn stub_0x412b28(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x412b2c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x412b3c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x412b54() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v")]
pub fn stub_0x412b58(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sAdvMoveTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv")]
pub fn stub_0x412b9c() -> crate::slot::PortedFn {
// IDA 0x412b9c: void RBX::Name::callDoDeclare<RBX::sAdvMoveTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x412b9c, "void RBX::Name::callDoDeclare<RBX::sAdvMoveTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v")]
pub fn stub_0x412ba0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAdvMoveTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x412c84() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::~TToolVerb() [0x412e08]")]
pub fn stub_0x412e08(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x412ea8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x412ee0(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::MoveResizeJoinTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x412ff4(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::MoveResizeJoinTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MoveResizeJoinTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x4130c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MoveResizeJoinTool")
}

#[doc(alias = "RBX::MoveResizeJoinTool::MoveResizeJoinTool(RBX::Workspace *)")]
pub fn stub_0x413174() -> crate::slot::InstanceHandle {
// RBX::MoveResizeJoinTool ctor.
crate::slot::InstanceHandle::new("RBX::MoveResizeJoinTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12AdvArrowToolELZNS_19sMoveResizeJoinToolEEE7getNameEv")]
pub fn stub_0x4132b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::MoveResizeJoinTool::isSticky(void)const")]
pub fn stub_0x4132b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MoveResizeJoinTool getter.
cell.get()
}

#[doc(alias = "RBX::MoveResizeJoinTool::drawConnectors(void)const")]
pub fn stub_0x41337c(handle: &crate::slot::InstanceHandle) {
// RBX::MoveResizeJoinTool::drawConnectors() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveResizeJoinTool::getCursorName(void)const")]
pub fn stub_0x413380(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MoveResizeJoinTool getter.
cell.get()
}

#[doc(alias = "RBX::MoveResizeJoinTool::setCursor(std::string)")]
pub fn stub_0x41338c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::MoveResizeJoinTool setter.
cell.set(value)
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool>::shared_ptr<RBX::MoveResizeJoinTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x413394() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MoveResizeJoinTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MoveResizeJoinTool,RBX::MoveResizeJoinTool>(rbx_core::SharedPtr<RBX::MoveResizeJoinTool> const*,RBX::MoveResizeJoinTool *)const")]
pub fn stub_0x41345c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MoveResizeJoinTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x413540() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x413638(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x41363c]")]
pub fn stub_0x41363c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x413640() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
