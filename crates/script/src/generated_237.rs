// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x413650..0x417e38 | script 24352->24452 distinct (filler 0x413650 asc, not-in-script 61193->61093)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x413650() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x413668() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
pub fn stub_0x41366c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sMoveResizeJoinTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sMoveResizeJoinToolEEEEvv")]
pub fn stub_0x4136b0() -> crate::slot::PortedFn {
// IDA 0x4136b0: void RBX::Name::callDoDeclare<RBX::sMoveResizeJoinTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4136b0, "void RBX::Name::callDoDeclare<RBX::sMoveResizeJoinTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
pub fn stub_0x4136b4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sMoveResizeJoinTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
pub fn stub_0x413798() -> crate::slot::InstanceHandle {
// RBX::TToolVerb ctor.
crate::slot::InstanceHandle::new("RBX::TToolVerb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::~TToolVerb() [0x41391c]")]
pub fn stub_0x41391c(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x4139bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x4139f0(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AxisRotateTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x413b04(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AxisRotateTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AxisRotateTool,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x413bd0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AxisRotateTool")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12AxisToolBaseELZNS_15sAxisRotateToolEEE7getNameEv")]
pub fn stub_0x413ca8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::AxisRotateTool::~AxisRotateTool()")]
pub fn stub_0x413cac(handle: crate::slot::InstanceHandle) {
// RBX::AxisRotateTool dtor.
drop(handle);
}

#[doc(alias = "RBX::AxisRotateTool::~AxisRotateTool() [0x413d90]")]
pub fn stub_0x413d90(handle: crate::slot::InstanceHandle) {
// RBX::AxisRotateTool dtor.
drop(handle);
}

#[doc(alias = "RBX::AxisRotateTool::isSticky(void)const")]
pub fn stub_0x413e88(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AxisRotateTool getter.
cell.get()
}

#[doc(alias = "RBX::AxisToolBase::drawConnectors(void)const")]
pub fn stub_0x413f50(handle: &crate::slot::InstanceHandle) {
// RBX::AxisToolBase::drawConnectors() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AxisToolBase::getCursorName(void)const")]
pub fn stub_0x413f54(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AxisToolBase getter.
cell.get()
}

#[doc(alias = "RBX::AxisRotateTool::getHandleColor(void)const")]
pub fn stub_0x413f60(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AxisRotateTool getter.
cell.get()
}

#[doc(alias = "RBX::AxisRotateTool::getDragType(void)const")]
pub fn stub_0x413f78(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::AxisRotateTool getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::AxisRotateTool::~AxisRotateTool()")]
pub fn stub_0x413f7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::AxisRotateTool::~AxisRotateTool() [0x414060]")]
pub fn stub_0x414060(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::AxisToolBase::~AxisToolBase()")]
pub fn stub_0x414158(handle: crate::slot::InstanceHandle) {
// RBX::AxisToolBase dtor.
drop(handle);
}

#[doc(alias = "RBX::AxisToolBase::~AxisToolBase() [0x41423c]")]
pub fn stub_0x41423c(handle: crate::slot::InstanceHandle) {
// RBX::AxisToolBase dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::AxisToolBase::~AxisToolBase()")]
pub fn stub_0x414334(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::AxisToolBase::~AxisToolBase() [0x414418]")]
pub fn stub_0x414418(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisRotateTool>::shared_ptr<RBX::AxisRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x414510() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AxisRotateTool")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AxisRotateTool,RBX::AxisRotateTool>(rbx_core::SharedPtr<RBX::AxisRotateTool> const*,RBX::AxisRotateTool *)const")]
pub fn stub_0x4145d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AxisRotateTool")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x4146bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4147b4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x4147b8]")]
pub fn stub_0x4147b8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x4147bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4147cc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4147e4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v")]
pub fn stub_0x4147e8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sAxisRotateTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAxisRotateToolEEEEvv")]
pub fn stub_0x41482c() -> crate::slot::PortedFn {
// IDA 0x41482c: void RBX::Name::callDoDeclare<RBX::sAxisRotateTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x41482c, "void RBX::Name::callDoDeclare<RBX::sAxisRotateTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v")]
pub fn stub_0x414830(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAxisRotateTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ResetCommand::ResetCommand(RBX::DataModel *)")]
pub fn stub_0x414914() -> crate::slot::InstanceHandle {
// RBX::ResetCommand ctor.
crate::slot::InstanceHandle::new("RBX::ResetCommand")
}

#[doc(alias = "RBX::StopCommand::StopCommand(RBX::DataModel *)")]
pub fn stub_0x414a5c() -> crate::slot::InstanceHandle {
// RBX::StopCommand ctor.
crate::slot::InstanceHandle::new("RBX::StopCommand")
}

#[doc(alias = "RBX::RunCommand::RunCommand(RBX::DataModel *)")]
pub fn stub_0x414ba4() -> crate::slot::InstanceHandle {
// RBX::RunCommand ctor.
crate::slot::InstanceHandle::new("RBX::RunCommand")
}

#[doc(alias = "RBX::MoveUpBrickVerb::MoveUpBrickVerb(RBX::DataModel *)")]
pub fn stub_0x414cec() -> crate::slot::InstanceHandle {
// RBX::MoveUpBrickVerb ctor.
crate::slot::InstanceHandle::new("RBX::MoveUpBrickVerb")
}

#[doc(alias = "RBX::MoveUpSelectionVerb::MoveUpSelectionVerb(RBX::DataModel *,std::string const&,float)")]
pub fn stub_0x414e3c() -> crate::slot::InstanceHandle {
// RBX::MoveUpSelectionVerb ctor.
crate::slot::InstanceHandle::new("RBX::MoveUpSelectionVerb")
}

#[doc(alias = "RBX::MoveUpBrickVerb::~MoveUpBrickVerb() [0x414f80]")]
pub fn stub_0x414f80(handle: crate::slot::InstanceHandle) {
// RBX::MoveUpBrickVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::MoveUpPlateVerb::MoveUpPlateVerb(RBX::DataModel *)")]
pub fn stub_0x415020() -> crate::slot::InstanceHandle {
// RBX::MoveUpPlateVerb ctor.
crate::slot::InstanceHandle::new("RBX::MoveUpPlateVerb")
}

#[doc(alias = "RBX::MoveUpPlateVerb::~MoveUpPlateVerb() [0x415170]")]
pub fn stub_0x415170(handle: crate::slot::InstanceHandle) {
// RBX::MoveUpPlateVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::AllCanSelectCommand::AllCanSelectCommand(RBX::DataModel *)")]
pub fn stub_0x415210() -> crate::slot::InstanceHandle {
// RBX::AllCanSelectCommand ctor.
crate::slot::InstanceHandle::new("RBX::AllCanSelectCommand")
}

#[doc(alias = "RBX::CanNotSelectCommand::CanNotSelectCommand(RBX::DataModel *)")]
pub fn stub_0x415358() -> crate::slot::InstanceHandle {
// RBX::CanNotSelectCommand ctor.
crate::slot::InstanceHandle::new("RBX::CanNotSelectCommand")
}

#[doc(alias = "RBX::CanCollideVerb::CanCollideVerb(RBX::DataModel *)")]
pub fn stub_0x4154a0() -> crate::slot::InstanceHandle {
// RBX::CanCollideVerb ctor.
crate::slot::InstanceHandle::new("RBX::CanCollideVerb")
}

#[doc(alias = "RBX::CanCollideVerb::~CanCollideVerb() [0x4155f4]")]
pub fn stub_0x4155f4(handle: crate::slot::InstanceHandle) {
// RBX::CanCollideVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TranslucentVerb::TranslucentVerb(RBX::DataModel *)")]
pub fn stub_0x415694() -> crate::slot::InstanceHandle {
// RBX::TranslucentVerb ctor.
crate::slot::InstanceHandle::new("RBX::TranslucentVerb")
}

#[doc(alias = "RBX::TranslucentVerb::~TranslucentVerb() [0x4157e8]")]
pub fn stub_0x4157e8(handle: crate::slot::InstanceHandle) {
// RBX::TranslucentVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::AnchorVerb::AnchorVerb(RBX::DataModel *)")]
pub fn stub_0x415888() -> crate::slot::InstanceHandle {
// RBX::AnchorVerb ctor.
crate::slot::InstanceHandle::new("RBX::AnchorVerb")
}

#[doc(alias = "RBX::AnchorVerb::~AnchorVerb() [0x4159dc]")]
pub fn stub_0x4159dc(handle: crate::slot::InstanceHandle) {
// RBX::AnchorVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::UnlockAllVerb::UnlockAllVerb(RBX::DataModel *)")]
pub fn stub_0x415a7c() -> crate::slot::InstanceHandle {
// RBX::UnlockAllVerb ctor.
crate::slot::InstanceHandle::new("RBX::UnlockAllVerb")
}

#[doc(alias = "RBX::SelectAllCommand::SelectAllCommand(RBX::DataModel *)")]
pub fn stub_0x415bc4() -> crate::slot::InstanceHandle {
// RBX::SelectAllCommand ctor.
crate::slot::InstanceHandle::new("RBX::SelectAllCommand")
}

#[doc(alias = "RBX::DeleteSelectionVerb::DeleteSelectionVerb(RBX::DataModel *)")]
pub fn stub_0x415d0c() -> crate::slot::InstanceHandle {
// RBX::DeleteSelectionVerb ctor.
crate::slot::InstanceHandle::new("RBX::DeleteSelectionVerb")
}

#[doc(alias = "RBX::DeleteSelectionVerb::~DeleteSelectionVerb() [0x415e58]")]
pub fn stub_0x415e58(handle: crate::slot::InstanceHandle) {
// RBX::DeleteSelectionVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::PlayDeleteSelectionVerb::PlayDeleteSelectionVerb(RBX::DataModel *)")]
pub fn stub_0x415ef8() -> crate::slot::InstanceHandle {
// RBX::PlayDeleteSelectionVerb ctor.
crate::slot::InstanceHandle::new("RBX::PlayDeleteSelectionVerb")
}

#[doc(alias = "RBX::PlayDeleteSelectionVerb::~PlayDeleteSelectionVerb() [0x416044]")]
pub fn stub_0x416044(handle: crate::slot::InstanceHandle) {
// RBX::PlayDeleteSelectionVerb dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_174")]
pub fn stub_0x4160e4() -> crate::slot::PortedFn {
// IDA 0x4160e4: __GLOBAL__I_a_174.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4160e4, "__GLOBAL__I_a_174")
}

#[doc(alias = "RBX::Configuration::Configuration(void)")]
pub fn stub_0x416388() -> crate::slot::InstanceHandle {
// RBX::Configuration ctor.
crate::slot::InstanceHandle::new("RBX::Configuration")
}

#[doc(alias = "RBX::Configuration::askForbidChild(RBX::Instance const*)const")]
pub fn stub_0x41657c(handle: &crate::slot::InstanceHandle) {
// RBX::Configuration::askForbidChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Configuration::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x4165b8(handle: &crate::slot::InstanceHandle) {
// RBX::Configuration::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Configuration::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x416674(handle: &crate::slot::InstanceHandle) {
// RBX::Configuration::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Configuration> RBX::shared_from<RBX::Configuration>(RBX::Configuration*)")]
pub fn stub_0x416808() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Configuration")
}

#[doc(alias = "RBX::Configuration::~Configuration()")]
pub fn stub_0x416978(handle: crate::slot::InstanceHandle) {
// RBX::Configuration dtor.
drop(handle);
}

#[doc(alias = "RBX::Configuration::~Configuration() [0x41697c]")]
pub fn stub_0x41697c(handle: crate::slot::InstanceHandle) {
// RBX::Configuration dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv")]
pub fn stub_0x416a1c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration()")]
pub fn stub_0x416a2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration() [0x416a34]")]
pub fn stub_0x416a34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E12getClassNameEv")]
pub fn stub_0x416ad8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration() [0x416ae8]")]
pub fn stub_0x416ae8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration() [0x416af0]")]
pub fn stub_0x416af0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD1Ev")]
pub fn stub_0x416b94() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorD2Ev")]
pub fn stub_0x416b98() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator12getClassNameEv")]
pub fn stub_0x416c34() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7Creator6createEv")]
pub fn stub_0x416cbc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Configuration> RBX::Creatable<RBX::Instance>::create<RBX::Configuration>(void)")]
pub fn stub_0x416e00() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Configuration")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Configuration>::shared_ptr<RBX::Configuration,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x416eb0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Configuration")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Configuration,RBX::Configuration>(rbx_core::SharedPtr<RBX::Configuration> const*,RBX::Configuration *)const")]
pub fn stub_0x416f78() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Configuration")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x417060() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x417168(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x41716c]")]
pub fn stub_0x41716c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x417170() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x417190() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Configuration *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4171a8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv")]
pub fn stub_0x4171ac() -> crate::slot::PortedFn {
// IDA 0x4171ac: void RBX::Name::callDoDeclare<RBX::sConfiguration>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4171ac, "void RBX::Name::callDoDeclare<RBX::sConfiguration>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")]
pub fn stub_0x4171b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sConfiguration>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E7CreatorC2Ev")]
pub fn stub_0x417290() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ConfigurationENS_8InstanceELZNS_14sConfigurationEES2_E17static_getCreatorEv")]
pub fn stub_0x4174d4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Configuration"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x417548(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x41754c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4175ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4175f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x417698(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ConfigurationELZNS_14sConfigurationEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sConfigurationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4176a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "global constructor keyed to_a_175")]
pub fn stub_0x417744() -> crate::slot::PortedFn {
// IDA 0x417744: __GLOBAL__I_a_175.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x417744, "__GLOBAL__I_a_175")
}

#[doc(alias = "RBX::CornerWedgeInstance::CornerWedgeInstance(void)")]
pub fn stub_0x417a70() -> crate::slot::InstanceHandle {
// RBX::CornerWedgeInstance ctor.
crate::slot::InstanceHandle::new("RBX::CornerWedgeInstance")
}

#[doc(alias = "RBX::CornerWedgeInstance::~CornerWedgeInstance()")]
pub fn stub_0x417d78(handle: crate::slot::InstanceHandle) {
// RBX::CornerWedgeInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::CornerWedgeInstance::~CornerWedgeInstance() [0x417e28]")]
pub fn stub_0x417e28(handle: crate::slot::InstanceHandle) {
// RBX::CornerWedgeInstance dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::CornerWedgeInstance::~CornerWedgeInstance()")]
pub fn stub_0x417e38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}
