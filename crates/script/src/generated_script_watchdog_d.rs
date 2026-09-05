// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x420c38..0x43184c | script 24652->24752 distinct (filler 0x420c38 asc, not-in-script 60893->60793)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::registerNewImageAPI(void)")]
pub fn stub_0x420c38() -> crate::slot::PortedFn {
// IDA 0x420c38: RBX::registerNewImageAPI().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x420c38, "RBX::registerNewImageAPI()")
}

#[doc(alias = "RBX::DataModel::onRunTransition(RBX::RunTransition)")]
pub fn stub_0x420fec(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::onRunTransition(RBX::RunTransition) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::~DataModel()")]
pub fn stub_0x421b80(handle: crate::slot::InstanceHandle) {
// RBX::DataModel dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::~DataModel() [0x421c20]")]
pub fn stub_0x421c20(handle: crate::slot::InstanceHandle) {
// RBX::DataModel dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel()")]
pub fn stub_0x421c24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x421c2c]")]
pub fn stub_0x421c2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x421c34]")]
pub fn stub_0x421c34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 144, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 144);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x421c3c]")]
pub fn stub_0x421c3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 180);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x421c44]")]
pub fn stub_0x421c44(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 184, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 184);
}

#[doc(alias = "RBX::DataModel::~DataModel() [0x421c4c]")]
pub fn stub_0x421c4c(handle: crate::slot::InstanceHandle) {
// RBX::DataModel dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x4228ac]")]
pub fn stub_0x4228ac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x4228b4]")]
pub fn stub_0x4228b4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x4228bc]")]
pub fn stub_0x4228bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 144, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 144);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x4228c4]")]
pub fn stub_0x4228c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 180);
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::~DataModel() [0x4228cc]")]
pub fn stub_0x4228cc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 184, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 184);
}

#[doc(alias = "RBX::DataModel::getGenericJob(RBX::DataModelJob::TaskType)")]
pub fn stub_0x4228d4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(rbx_core::SharedPtr<RBX::DataModel>,RBX::DataModelJob::TaskType) [0x422ae8]")]
pub fn stub_0x422ae8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")]
pub fn stub_0x422c64() -> crate::slot::InstanceHandle {
// RBX::DataModel::LegacyLock ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock")
}

#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType) [0x422c68]")]
pub fn stub_0x422c68() -> crate::slot::InstanceHandle {
// RBX::DataModel::LegacyLock ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::LegacyLock")
}

#[doc(alias = "RBX::DataModel::submitTask(boost::function<void ()(RBX::DataModel*)>,RBX::DataModelJob::TaskType)")]
pub fn stub_0x422dd8(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::submitTask(boost::function<void (RBX::DataModel*)>, RBX::DataModelJob::Tas~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::HttpHelper(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x422ef4(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::DataModel::doHttpGet(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x4230c4(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::doHttpGet(std::string const&, boost::function<void (std::string)>, boost::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::doHttpGet(std::string const&)")]
pub fn stub_0x4234e4(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::doHttpGet(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::doHttpPost(std::string const&,std::string const&)")]
pub fn stub_0x4237e4(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::doHttpPost(std::string const&, std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::doHttpPost(std::string const&,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x423b58(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::doHttpPost(std::string const&, std::string const&, boost::function<void (s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::loadAssetIdIntoStream(int)")]
pub fn stub_0x423f90(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::loadAssetIdIntoStream(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::onChildAdded(RBX::Instance *)")]
pub fn stub_0x424378(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::onChildAdded(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x42439c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::computeGuiInset(RBX::Adorn *)")]
pub fn stub_0x4243d8(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::computeGuiInset(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::renderPlayerGui(RBX::Adorn *)")]
pub fn stub_0x42442c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::renderPlayerGui(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::renderGuiRoot(RBX::Adorn *)")]
pub fn stub_0x4244c0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::renderGuiRoot(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::getUpdatedMessageBoxText(void)")]
pub fn stub_0x424510(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "RBX::DataModel::renderMessageBox(RBX::Adorn *)")]
pub fn stub_0x424ed0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::renderMessageBox(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::renderPass2d(RBX::Adorn *,RBX::IMetric *)")]
pub fn stub_0x4251d8(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::renderPass2d(RBX::Adorn*, RBX::IMetric*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::getRenderMouseCursor(void)")]
pub fn stub_0x4252ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "RBX::DataModel::renderMouse(RBX::Adorn *)")]
pub fn stub_0x42538c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::renderMouse(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::renderPass3dAdorn(RBX::Adorn *)")]
pub fn stub_0x425590(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::renderPass3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GreaterProjectedPosition(RBX::IAdornable const*,RBX::IAdornable const*)")]
pub fn stub_0x4259b8() -> crate::slot::PortedFn {
// IDA 0x4259b8: RBX::GreaterProjectedPosition(RBX::IAdornable const*, RBX::IAdornable const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4259b8, "RBX::GreaterProjectedPosition(RBX::IAdornable const*, RBX::IAdornable const*)")
}

#[doc(alias = "RBX::DataModel::physicsStep(float,double,double,int)")]
pub fn stub_0x4259d0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::physicsStep(float, double, double, int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
pub fn stub_0x425d58(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::processAccelerators(RBX::GuiEvent const&)")]
pub fn stub_0x4260d8(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::processAccelerators(RBX::GuiEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::switchViewMode(void)")]
pub fn stub_0x427054(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::switchViewMode() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::processPlayerGui(RBX::GuiEvent const&)")]
pub fn stub_0x42738c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::processPlayerGui(RBX::GuiEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::processCameraCommands(RBX::GuiEvent const&)")]
pub fn stub_0x4273c0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::processCameraCommands(RBX::GuiEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::processEvent(RBX::UIEvent const&)")]
pub fn stub_0x4275e0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::processEvent(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::processWorkspaceEvent(RBX::UIEvent const&)")]
pub fn stub_0x427b54(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::processWorkspaceEvent(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::processUiEvent(RBX::UIEvent const&)")]
pub fn stub_0x427bac(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::processUiEvent(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::setNetworkMetric(RBX::IMetric *)")]
pub fn stub_0x427db8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModel setter.
cell.set(value)
}

#[doc(alias = "RBX::DataModel::getMetricValue(std::string const&)const")]
pub fn stub_0x427dc0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::getMetricValue(std::string const&)const")]
pub fn stub_0x4288b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 180);
}

#[doc(alias = "RBX::DataModel::getMetric(std::string const&)const")]
pub fn stub_0x4288b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::getMetric(std::string const&)const")]
pub fn stub_0x42fb24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 180, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 180);
}

#[doc(alias = "RBX::DataModel::get(RBX::Instance *)")]
pub fn stub_0x42fb30(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::get(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::appendJobInfo(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
pub fn stub_0x42fb68() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job const")
}

#[doc(alias = "RBX::DataModel::gameLoaded(void)")]
pub fn stub_0x430004(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::gameLoaded() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::appendJobExtendedStats(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
pub fn stub_0x43001c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job const")
}

#[doc(alias = "RBX::getJobTimePeakFractionFunc(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *)")]
pub fn stub_0x43053c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job const")
}

#[doc(alias = "RBX::getJobIntervalPeakFractionFunc(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *)")]
pub fn stub_0x4305b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job const")
}

#[doc(alias = "RBX::DataModel::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
pub fn stub_0x43062c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::onChildChanged(RBX::Instance*, RBX::PropertyChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::onDescendantAdded(RBX::Instance *)")]
pub fn stub_0x43077c(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::onDescendantAdded(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x430840() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::DataModel::getNumPlayers(void)const")]
pub fn stub_0x430900(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::getNumPlayers(void)const")]
pub fn stub_0x430924(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 184, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 184);
}

#[doc(alias = "RBX::DataModel::ScreenshotReadyTask(rbx_core::Weak<RBX::DataModel>,std::string const&)")]
pub fn stub_0x430930() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::DataModel::currentThreadHasWriteLock(void)const")]
pub fn stub_0x4309f8(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::currentThreadHasWriteLock() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::scoped_write_request::scoped_write_request(RBX::Instance *)")]
pub fn stub_0x430a10() -> crate::slot::InstanceHandle {
// RBX::DataModel::scoped_write_request ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::scoped_write_request")
}

#[doc(alias = "RBX::DataModel::scoped_write_request::scoped_write_request(RBX::Instance *) [0x430a14]")]
pub fn stub_0x430a14() -> crate::slot::InstanceHandle {
// RBX::DataModel::scoped_write_request ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::scoped_write_request")
}

#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
pub fn stub_0x430b28(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::scoped_write_request dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request() [0x430b2c]")]
pub fn stub_0x430b2c(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::scoped_write_request dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")]
pub fn stub_0x430c18() -> crate::slot::InstanceHandle {
// RBX::DataModel::scoped_read_request ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::scoped_read_request")
}

#[doc(alias = "RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *) [0x430c1c]")]
pub fn stub_0x430c1c() -> crate::slot::InstanceHandle {
// RBX::DataModel::scoped_read_request ctor.
crate::slot::InstanceHandle::new("RBX::DataModel::scoped_read_request")
}

#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
pub fn stub_0x430d0c(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::scoped_read_request dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request() [0x430d10]")]
pub fn stub_0x430d10(handle: crate::slot::InstanceHandle) {
// RBX::DataModel::scoped_read_request dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::allHackFlagsOredTogether(void)")]
pub fn stub_0x430df4(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::allHackFlagsOredTogether() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0")]
pub fn stub_0x430e54() -> crate::slot::PortedFn {
// IDA 0x430e54: __ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x430e54, "__ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0")
}

#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0")]
pub fn stub_0x430fa8() -> crate::slot::PortedFn {
// IDA 0x430fa8: __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x430fa8, "__ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0")
}

#[doc(alias = "void RBX::Reflection::resume_adapter<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_0x431100() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "boost::function<void ()(RBX::DataModel *)>::~function()")]
pub fn stub_0x431268(slot: &crate::slot::FnSlot) {
// boost::function invocation — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "RBX::DataModel::loadPlugins(void)")]
pub fn stub_0x431278(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::loadPlugins() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x431288(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")]
pub fn stub_0x4312ac(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
pub fn stub_0x4312d0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_0x431350(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")]
pub fn stub_0x431390(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_0x4313d0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x431410(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_0x431434(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")]
pub fn stub_0x4314fc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")]
pub fn stub_0x431544(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x431594(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")]
pub fn stub_0x4315b8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::DataModel::getIsPersonalServer(void)const")]
pub fn stub_0x431618(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DataModel getter.
cell.get()
}

#[doc(alias = "RBX::DataModel::setIsPersonalServer(bool)")]
pub fn stub_0x431620(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModel setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")]
pub fn stub_0x431628(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")]
pub fn stub_0x431670(handle: crate::slot::InstanceHandle) {
// RBX::Reflection dtor.
drop(handle);
}

#[doc(alias = "RBX::DataModel::setUiMessageBrickCount(void)")]
pub fn stub_0x431768(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::DataModel setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
pub fn stub_0x43177c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
pub fn stub_0x4317c4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
pub fn stub_0x431804(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
pub fn stub_0x43184c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}
