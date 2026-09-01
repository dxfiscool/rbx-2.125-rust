//! rendering shard 301 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32640->32740 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32640 before -> 32740 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0xf6fb4c (lowest remaining 0x4228d4..0x431ad4)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4228d4 — __ZN3RBX9DataModel13getGenericJobENS_12DataModelJob8TaskTypeE
// type: void __fastcall(sp_counted_base **, int, int)
#[doc(alias = "RBX::DataModel::getGenericJob(RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel13getGenericJobENS_12DataModelJob8TaskTypeE
pub fn stub_4228d4() -> ! {
    todo!("0x4228d4 RBX::DataModel::getGenericJob(RBX::DataModelJob::TaskType)")
}

// 0x422ae8 — __ZN3RBX9DataModel10LegacyLockC2EN5boost10shared_ptrIS0_EENS_12DataModelJob8TaskTypeE
// type: boost::detail::sp_counted_base **__fastcall(boost::detail::sp_counted_base **, struct _Unwind_Exception **, int)
#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(rbx_core::SharedPtr<RBX::DataModel>,RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel10LegacyLockC2EN5boost10shared_ptrIS0_EENS_12DataModelJob8TaskTypeE
pub fn stub_422ae8() -> ! {
    todo!("0x422ae8 RBX::DataModel::LegacyLock::LegacyLock(boost::shared_ptr<RBX::DataModel>,RBX::DataModelJob::TaskType)")
}

// 0x422c64 — __ZN3RBX9DataModel10LegacyLockC1EPS0_NS_12DataModelJob8TaskTypeE
// type: int()
#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel10LegacyLockC1EPS0_NS_12DataModelJob8TaskTypeE
pub fn stub_422c64() -> ! {
    todo!("0x422c64 RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")
}

// 0x422c68 — __ZN3RBX9DataModel10LegacyLockC2EPS0_NS_12DataModelJob8TaskTypeE
// type: boost::detail::sp_counted_base **__fastcall(boost::detail::sp_counted_base **, int, int)
#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel10LegacyLockC2EPS0_NS_12DataModelJob8TaskTypeE
pub fn stub_422c68() -> ! {
    todo!("0x422c68 RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")
}

// 0x422dd8 — __ZN3RBX9DataModel10submitTaskEN5boost8functionIFvPS0_EEENS_12DataModelJob8TaskTypeE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DataModel::submitTask(boost::function<void ()(RBX::DataModel*)>,RBX::DataModelJob::TaskType)")]
// was: __ZN3RBX9DataModel10submitTaskEN5boost8functionIFvPS0_EEENS_12DataModelJob8TaskTypeE
pub fn stub_422dd8() -> ! {
    todo!("0x422dd8 RBX::DataModel::submitTask(boost::function<void ()(RBX::DataModel*)>,RBX::DataModelJob::TaskType)")
}

// 0x422ef4 — __ZN3RBX9DataModel10HttpHelperEPSsPSt9exceptionN5boost8functionIFvSsEEES7_
// type: void __fastcall(const std::string *, int, int, int)
#[doc(alias = "RBX::DataModel::HttpHelper(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel10HttpHelperEPSsPSt9exceptionN5boost8functionIFvSsEEES7_
pub fn stub_422ef4() -> ! {
    todo!("0x422ef4 RBX::DataModel::HttpHelper(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x4230c4 — __ZN3RBX9DataModel9doHttpGetERKSsN5boost8functionIFvSsEEES6_
// type: void __fastcall(std::string *, int, int)
#[doc(alias = "RBX::DataModel::doHttpGet(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel9doHttpGetERKSsN5boost8functionIFvSsEEES6_
pub fn stub_4230c4() -> ! {
    todo!("0x4230c4 RBX::DataModel::doHttpGet(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x4234e4 — __ZN3RBX9DataModel9doHttpGetERKSs
// type: void __fastcall(RBX::DataModel *this, const std::string *)
#[doc(alias = "RBX::DataModel::doHttpGet(std::string const&)")]
// was: __ZN3RBX9DataModel9doHttpGetERKSs
pub fn stub_4234e4() -> ! {
    todo!("0x4234e4 RBX::DataModel::doHttpGet(std::string const&)")
}

// 0x4237e4 — __ZN3RBX9DataModel10doHttpPostERKSsS2_
// type: void __fastcall(RBX::DataModel *this, const std::string *, const std::string *)
#[doc(alias = "RBX::DataModel::doHttpPost(std::string const&,std::string const&)")]
// was: __ZN3RBX9DataModel10doHttpPostERKSsS2_
pub fn stub_4237e4() -> ! {
    todo!("0x4237e4 RBX::DataModel::doHttpPost(std::string const&,std::string const&)")
}

// 0x423b58 — __ZN3RBX9DataModel10doHttpPostERKSsS2_N5boost8functionIFvSsEEES6_
// type: void __fastcall(std::string *, _DWORD *, int, int)
#[doc(alias = "RBX::DataModel::doHttpPost(std::string const&,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX9DataModel10doHttpPostERKSsS2_N5boost8functionIFvSsEEES6_
pub fn stub_423b58() -> ! {
    todo!("0x423b58 RBX::DataModel::doHttpPost(std::string const&,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")
}

// 0x423f90 — __ZN3RBX9DataModel21loadAssetIdIntoStreamEi
// type: int __fastcall(RBX::DataModel *this, int, int)
#[doc(alias = "RBX::DataModel::loadAssetIdIntoStream(int)")]
// was: __ZN3RBX9DataModel21loadAssetIdIntoStreamEi
pub fn stub_423f90() -> ! {
    todo!("0x423f90 RBX::DataModel::loadAssetIdIntoStream(int)")
}

// 0x424378 — __ZN3RBX9DataModel12onChildAddedEPNS_8InstanceE
// type: int __fastcall(RBX::Workspace **this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX9DataModel12onChildAddedEPNS_8InstanceE
pub fn stub_424378() -> ! {
    todo!("0x424378 RBX::DataModel::onChildAdded(RBX::Instance *)")
}

// 0x42439c — __ZNK3RBX9DataModel11askAddChildEPKNS_8InstanceE
// type: bool __fastcall(RBX::DataModel *this, const RBX::Instance *lpsrc)
#[doc(alias = "RBX::DataModel::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX9DataModel11askAddChildEPKNS_8InstanceE
pub fn stub_42439c() -> ! {
    todo!("0x42439c RBX::DataModel::askAddChild(RBX::Instance const*)const")
}

// 0x424510 — __ZN3RBX9DataModel24getUpdatedMessageBoxTextEv
// type: void __fastcall(RBX::DataModel *this, RBX::Workspace **)
#[doc(alias = "RBX::DataModel::getUpdatedMessageBoxText(void)")]
// was: __ZN3RBX9DataModel24getUpdatedMessageBoxTextEv
pub fn stub_424510() -> ! {
    todo!("0x424510 RBX::DataModel::getUpdatedMessageBoxText(void)")
}

// 0x4259d0 — __ZN3RBX9DataModel11physicsStepEfddi
// type: void __fastcall(RBX::DataModel *this, float, double, double, int)
#[doc(alias = "RBX::DataModel::physicsStep(float,double,double,int)")]
// was: __ZN3RBX9DataModel11physicsStepEfddi
pub fn stub_4259d0() -> ! {
    todo!("0x4259d0 RBX::DataModel::physicsStep(float,double,double,int)")
}

// 0x425d58 — __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
// was: __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE
pub fn stub_425d58() -> ! {
    todo!("0x425d58 RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")
}

// 0x4260d8 — __ZN3RBX9DataModel19processAcceleratorsERKNS_8GuiEventE
// type: void __fastcall(RBX::OnScreenProfiler *, int, _DWORD *, int)
#[doc(alias = "RBX::DataModel::processAccelerators(RBX::GuiEvent const&)")]
// was: __ZN3RBX9DataModel19processAcceleratorsERKNS_8GuiEventE
pub fn stub_4260d8() -> ! {
    todo!("0x4260d8 RBX::DataModel::processAccelerators(RBX::GuiEvent const&)")
}

// 0x427054 — __ZN3RBX9DataModel14switchViewModeEv
// type: void __fastcall(RBX::DataModel *this, int, int, int)
#[doc(alias = "RBX::DataModel::switchViewMode(void)")]
// was: __ZN3RBX9DataModel14switchViewModeEv
pub fn stub_427054() -> ! {
    todo!("0x427054 RBX::DataModel::switchViewMode(void)")
}

// 0x42738c — __ZN3RBX9DataModel16processPlayerGuiERKNS_8GuiEventE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::DataModel::processPlayerGui(RBX::GuiEvent const&)")]
// was: __ZN3RBX9DataModel16processPlayerGuiERKNS_8GuiEventE
pub fn stub_42738c() -> ! {
    todo!("0x42738c RBX::DataModel::processPlayerGui(RBX::GuiEvent const&)")
}

// 0x4273c0 — __ZN3RBX9DataModel21processCameraCommandsERKNS_8GuiEventE
// type: void __fastcall(_QWORD *, int, _DWORD *)
#[doc(alias = "RBX::DataModel::processCameraCommands(RBX::GuiEvent const&)")]
// was: __ZN3RBX9DataModel21processCameraCommandsERKNS_8GuiEventE
pub fn stub_4273c0() -> ! {
    todo!("0x4273c0 RBX::DataModel::processCameraCommands(RBX::GuiEvent const&)")
}

// 0x4275e0 — __ZN3RBX9DataModel12processEventERKNS_7UIEventE
// type: int __fastcall(RBX::DataModel *this, const RBX::UIEvent *, int, const void *)
#[doc(alias = "RBX::DataModel::processEvent(RBX::UIEvent const&)")]
// was: __ZN3RBX9DataModel12processEventERKNS_7UIEventE
pub fn stub_4275e0() -> ! {
    todo!("0x4275e0 RBX::DataModel::processEvent(RBX::UIEvent const&)")
}

// 0x427b54 — __ZN3RBX9DataModel21processWorkspaceEventERKNS_7UIEventE
// type: int __fastcall(RBX::DataModel *this, const RBX::UIEvent *, int, const void *)
#[doc(alias = "RBX::DataModel::processWorkspaceEvent(RBX::UIEvent const&)")]
// was: __ZN3RBX9DataModel21processWorkspaceEventERKNS_7UIEventE
pub fn stub_427b54() -> ! {
    todo!("0x427b54 RBX::DataModel::processWorkspaceEvent(RBX::UIEvent const&)")
}

// 0x427bac — __ZN3RBX9DataModel14processUiEventERKNS_7UIEventE
// type: int __fastcall(RBX::DataModel *this, const RBX::UIEvent *, int, int)
#[doc(alias = "RBX::DataModel::processUiEvent(RBX::UIEvent const&)")]
// was: __ZN3RBX9DataModel14processUiEventERKNS_7UIEventE
pub fn stub_427bac() -> ! {
    todo!("0x427bac RBX::DataModel::processUiEvent(RBX::UIEvent const&)")
}

// 0x427db8 — __ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE
// type: int __fastcall(int this, IMetric *)
#[doc(alias = "RBX::DataModel::setNetworkMetric(RBX::IMetric *)")]
// was: __ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE
pub fn stub_427db8() -> ! {
    todo!("0x427db8 RBX::DataModel::setNetworkMetric(RBX::IMetric *)")
}

// 0x427dc0 — __ZNK3RBX9DataModel14getMetricValueERKSs
// type: __int64 __fastcall(RBX::DataModel *this, const std::string *)
#[doc(alias = "RBX::DataModel::getMetricValue(std::string const&)const")]
// was: __ZNK3RBX9DataModel14getMetricValueERKSs
pub fn stub_427dc0() -> ! {
    todo!("0x427dc0 RBX::DataModel::getMetricValue(std::string const&)const")
}

// 0x4288b0 — __ZThn180_NK3RBX9DataModel14getMetricValueERKSs
// type: __int64 __fastcall(RBX::DataModel *this, const std::string *)
#[doc(alias = "non-virtual thunk toRBX::DataModel::getMetricValue(std::string const&)const")]
// was: __ZThn180_NK3RBX9DataModel14getMetricValueERKSs
pub fn stub_4288b0() -> ! {
    todo!("0x4288b0 non-virtual thunk toRBX::DataModel::getMetricValue(std::string const&)const")
}

// 0x4288b8 — __ZNK3RBX9DataModel9getMetricERKSs
// type: void __fastcall(RBX::DataModel *this, pthread_mutex_t *, std::string *)
#[doc(alias = "RBX::DataModel::getMetric(std::string const&)const")]
// was: __ZNK3RBX9DataModel9getMetricERKSs
pub fn stub_4288b8() -> ! {
    todo!("0x4288b8 RBX::DataModel::getMetric(std::string const&)const")
}

// 0x42fb24 — __ZThn180_NK3RBX9DataModel9getMetricERKSs
// type: void __fastcall(RBX::DataModel *this, const std::string *, std::string *)
#[doc(alias = "non-virtual thunk toRBX::DataModel::getMetric(std::string const&)const")]
// was: __ZThn180_NK3RBX9DataModel9getMetricERKSs
pub fn stub_42fb24() -> ! {
    todo!("0x42fb24 non-virtual thunk toRBX::DataModel::getMetric(std::string const&)const")
}

// 0x42fb30 — __ZN3RBX9DataModel3getEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::DataModel *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::get(RBX::Instance *)")]
// was: __ZN3RBX9DataModel3getEPNS_8InstanceE
pub fn stub_42fb30() -> ! {
    todo!("0x42fb30 RBX::DataModel::get(RBX::Instance *)")
}

// 0x42fb68 — __ZN3RBXL13appendJobInfoEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE
// type: int __fastcall(int, RBX::TaskScheduler::Job **, int)
#[doc(alias = "RBX::appendJobInfo(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
// was: __ZN3RBXL13appendJobInfoEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE
pub fn stub_42fb68() -> ! {
    todo!("0x42fb68 RBX::appendJobInfo(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")
}

// 0x430004 — __ZN3RBX9DataModel10gameLoadedEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::DataModel::gameLoaded(void)")]
// was: __ZN3RBX9DataModel10gameLoadedEv
pub fn stub_430004() -> ! {
    todo!("0x430004 RBX::DataModel::gameLoaded(void)")
}

// 0x43001c — __ZN3RBXL22appendJobExtendedStatsEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE
// type: int __fastcall(sp_counted_base *, const shared_count **, int)
#[doc(alias = "RBX::appendJobExtendedStats(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")]
// was: __ZN3RBXL22appendJobExtendedStatsEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEEPSt6vectorINS_10Reflection7VariantESaISA_EE
pub fn stub_43001c() -> ! {
    todo!("0x43001c RBX::appendJobExtendedStats(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *)")
}

// 0x43053c — __ZN3RBXL26getJobTimePeakFractionFuncEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEERSsdPd
// type: boost::detail::sp_counted_base *__fastcall(sp_counted_base *, const shared_count **, int, unsigned int, unsigned int, double *)
#[doc(alias = "RBX::getJobTimePeakFractionFunc(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *)")]
// was: __ZN3RBXL26getJobTimePeakFractionFuncEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEERSsdPd
pub fn stub_43053c() -> ! {
    todo!("0x43053c RBX::getJobTimePeakFractionFunc(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *)")
}

// 0x4305b4 — __ZN3RBXL30getJobIntervalPeakFractionFuncEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEERSsdPd
// type: boost::detail::sp_counted_base *__fastcall(sp_counted_base *, const shared_count **, int, unsigned int, unsigned int, double *)
#[doc(alias = "RBX::getJobIntervalPeakFractionFunc(RBX::DataModel *,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::string &,double,double *)")]
// was: __ZN3RBXL30getJobIntervalPeakFractionFuncEPNS_9DataModelEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEERSsdPd
pub fn stub_4305b4() -> ! {
    todo!("0x4305b4 RBX::getJobIntervalPeakFractionFunc(RBX::DataModel *,boost::shared_ptr<RBX::TaskScheduler::Job const>,std::string &,double,double *)")
}

// 0x43062c — __ZN3RBX9DataModel14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
// type: void __fastcall(_DWORD *, int, _DWORD *)
#[doc(alias = "RBX::DataModel::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
// was: __ZN3RBX9DataModel14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
pub fn stub_43062c() -> ! {
    todo!("0x43062c RBX::DataModel::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")
}

// 0x43077c — __ZN3RBX9DataModel17onDescendantAddedEPNS_8InstanceE
// type: int __fastcall(RBX::DataModel *this, RBX::Instance *, int, int)
#[doc(alias = "RBX::DataModel::onDescendantAdded(RBX::Instance *)")]
// was: __ZN3RBX9DataModel17onDescendantAddedEPNS_8InstanceE
pub fn stub_43077c() -> ! {
    todo!("0x43077c RBX::DataModel::onDescendantAdded(RBX::Instance *)")
}

// 0x430840 — __ZN3RBX9DataModel20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "RBX::DataModel::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX9DataModel20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_430840() -> ! {
    todo!("0x430840 RBX::DataModel::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")
}

// 0x430900 — __ZNK3RBX9DataModel13getNumPlayersEv
// type: _DWORD __fastcall(RBX::DataModel *__hidden this)
#[doc(alias = "RBX::DataModel::getNumPlayers(void)const")]
// was: __ZNK3RBX9DataModel13getNumPlayersEv
pub fn stub_430900() -> ! {
    todo!("0x430900 RBX::DataModel::getNumPlayers(void)const")
}

// 0x430924 — __ZThn184_NK3RBX9DataModel13getNumPlayersEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "non-virtual thunk toRBX::DataModel::getNumPlayers(void)const")]
// was: __ZThn184_NK3RBX9DataModel13getNumPlayersEv
pub fn stub_430924() -> ! {
    todo!("0x430924 non-virtual thunk toRBX::DataModel::getNumPlayers(void)const")
}

// 0x430930 — __ZN3RBX9DataModel19ScreenshotReadyTaskEN5boost8weak_ptrIS0_EERKSs
// type: void __fastcall(int, int)
#[doc(alias = "RBX::DataModel::ScreenshotReadyTask(boost::weak_ptr<RBX::DataModel>,std::string const&)")]
// was: __ZN3RBX9DataModel19ScreenshotReadyTaskEN5boost8weak_ptrIS0_EERKSs
pub fn stub_430930() -> ! {
    todo!("0x430930 RBX::DataModel::ScreenshotReadyTask(boost::weak_ptr<RBX::DataModel>,std::string const&)")
}

// 0x4309f8 — __ZNK3RBX9DataModel25currentThreadHasWriteLockEv
// type: bool __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::currentThreadHasWriteLock(void)const")]
// was: __ZNK3RBX9DataModel25currentThreadHasWriteLockEv
pub fn stub_4309f8() -> ! {
    todo!("0x4309f8 RBX::DataModel::currentThreadHasWriteLock(void)const")
}

// 0x430a10 — __ZN3RBX9DataModel20scoped_write_requestC1EPNS_8InstanceE
// type: int __fastcall(RBX::DataModel::scoped_write_request *this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::scoped_write_request::scoped_write_request(RBX::Instance *)")]
// was: __ZN3RBX9DataModel20scoped_write_requestC1EPNS_8InstanceE
pub fn stub_430a10() -> ! {
    todo!("0x430a10 RBX::DataModel::scoped_write_request::scoped_write_request(RBX::Instance *)")
}

// 0x430a14 — __ZN3RBX9DataModel20scoped_write_requestC2EPNS_8InstanceE
// type: RBX::DataModel::scoped_write_request *__fastcall(RBX::DataModel::scoped_write_request *this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::scoped_write_request::scoped_write_request(RBX::Instance *)")]
// was: __ZN3RBX9DataModel20scoped_write_requestC2EPNS_8InstanceE
pub fn stub_430a14() -> ! {
    todo!("0x430a14 RBX::DataModel::scoped_write_request::scoped_write_request(RBX::Instance *)")
}

// 0x430b28 — __ZN3RBX9DataModel20scoped_write_requestD1Ev
// type: void __fastcall(RBX::DataModel::scoped_write_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
// was: __ZN3RBX9DataModel20scoped_write_requestD1Ev
pub fn stub_430b28() -> ! {
    todo!("0x430b28 RBX::DataModel::scoped_write_request::~scoped_write_request()")
}

// 0x430b2c — __ZN3RBX9DataModel20scoped_write_requestD2Ev
// type: void __fastcall(RBX::DataModel::scoped_write_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
// was: __ZN3RBX9DataModel20scoped_write_requestD2Ev
pub fn stub_430b2c() -> ! {
    todo!("0x430b2c RBX::DataModel::scoped_write_request::~scoped_write_request()")
}

// 0x430c18 — __ZN3RBX9DataModel19scoped_read_requestC1EPNS_8InstanceE
// type: int __fastcall(RBX::DataModel::scoped_read_request *this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")]
// was: __ZN3RBX9DataModel19scoped_read_requestC1EPNS_8InstanceE
pub fn stub_430c18() -> ! {
    todo!("0x430c18 RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")
}

// 0x430c1c — __ZN3RBX9DataModel19scoped_read_requestC2EPNS_8InstanceE
// type: RBX::DataModel::scoped_read_request *__fastcall(RBX::DataModel::scoped_read_request *this, RBX::Instance *)
#[doc(alias = "RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")]
// was: __ZN3RBX9DataModel19scoped_read_requestC2EPNS_8InstanceE
pub fn stub_430c1c() -> ! {
    todo!("0x430c1c RBX::DataModel::scoped_read_request::scoped_read_request(RBX::Instance *)")
}

// 0x430d0c — __ZN3RBX9DataModel19scoped_read_requestD1Ev
// type: void __fastcall(RBX::DataModel::scoped_read_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
// was: __ZN3RBX9DataModel19scoped_read_requestD1Ev
pub fn stub_430d0c() -> ! {
    todo!("0x430d0c RBX::DataModel::scoped_read_request::~scoped_read_request()")
}

// 0x430d10 — __ZN3RBX9DataModel19scoped_read_requestD2Ev
// type: void __fastcall(RBX::DataModel::scoped_read_request *__hidden this)
#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
// was: __ZN3RBX9DataModel19scoped_read_requestD2Ev
pub fn stub_430d10() -> ! {
    todo!("0x430d10 RBX::DataModel::scoped_read_request::~scoped_read_request()")
}

// 0x430df4 — __ZN3RBX9DataModel24allHackFlagsOredTogetherEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::allHackFlagsOredTogether(void)")]
// was: __ZN3RBX9DataModel24allHackFlagsOredTogetherEv
pub fn stub_430df4() -> ! {
    todo!("0x430df4 RBX::DataModel::allHackFlagsOredTogether(void)")
}

// 0x430e54 — __ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0
// type: int __fastcall(int, char)
#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0")]
// was: __ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0
pub fn stub_430e54() -> ! {
    todo!("0x430e54 __ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__0")
}

// 0x430fa8 — __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0
// type: int __fastcall(int, const std::string *)
#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0")]
// was: __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0
pub fn stub_430fa8() -> ! {
    todo!("0x430fa8 __ZN3RBX10ReflectionL14resume_adapterISsEEvN5boost8functionIFvNS0_7VariantEEEET__0")
}

// 0x431100 — __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKNS0_5TupleEEEEEvNS2_8functionIFvNS0_7VariantEEEET_
// type: int __fastcall(int, const shared_count *)
#[doc(alias = "void RBX::Reflection::resume_adapter<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
// was: __ZN3RBX10ReflectionL14resume_adapterIN5boost10shared_ptrIKNS0_5TupleEEEEEvNS2_8functionIFvNS0_7VariantEEEET_
pub fn stub_431100() -> ! {
    todo!("0x431100 void RBX::Reflection::resume_adapter<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x431268 — __ZN5boost8functionIFvPN3RBX9DataModelEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::function<void ()(RBX::DataModel *)>::~function()")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEED1Ev
pub fn stub_431268() -> ! {
    todo!("0x431268 boost::function<void ()(RBX::DataModel *)>::~function()")
}

// 0x431278 — __ZN3RBX9DataModel11loadPluginsEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::loadPlugins(void)")]
// was: __ZN3RBX9DataModel11loadPluginsEv
pub fn stub_431278() -> ! {
    todo!("0x431278 RBX::DataModel::loadPlugins(void)")
}

// 0x431288 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvvELi0EED1Ev
pub fn stub_431288() -> ! {
    todo!("0x431288 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(void),0>::~BoundFuncDesc()")
}

// 0x4312ac — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvN5boost10shared_ptrINS_8InstanceEEEPKNS0_18PropertyDescriptorEEN3rbx6signalISA_EEMS2_SD_ED1Ev
pub fn stub_4312ac() -> ! {
    todo!("0x4312ac RBX::Reflection::EventDesc<RBX::DataModel,void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)> RBX::DataModel::*>::~EventDesc()")
}

// 0x4312d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEENS_9ContentIdEELi1EED1Ev
pub fn stub_4312d0() -> ! {
    todo!("0x4312d0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x431310 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFN5boost10shared_ptrIKNS0_5TupleEEENS_8Instance10SaveFilterEES7_Li1EED1Ev
pub fn stub_431310() -> ! {
    todo!("0x431310 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,boost::shared_ptr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),boost::shared_ptr<RBX::Reflection::Tuple const>,1>::~BoundYieldFuncDesc()")
}

// 0x431350 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviELi1EED1Ev
pub fn stub_431350() -> ! {
    todo!("0x431350 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int),1>::~BoundFuncDesc()")
}

// 0x431390 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS_9ContentIdEELi1EED1Ev
pub fn stub_431390() -> ! {
    todo!("0x431390 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::ContentId),1>::~BoundFuncDesc()")
}

// 0x4313d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvbELi1EED1Ev
pub fn stub_4313d0() -> ! {
    todo!("0x4313d0 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(bool),1>::~BoundFuncDesc()")
}

// 0x431410 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbvELi0EED1Ev
pub fn stub_431410() -> ! {
    todo!("0x431410 RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(void),0>::~BoundFuncDesc()")
}

// 0x431434 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsELi1EED1Ev
pub fn stub_431434() -> ! {
    todo!("0x431434 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x431474 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsESsLi1EED1Ev
pub fn stub_431474() -> ! {
    todo!("0x431474 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::~BoundYieldFuncDesc()")
}

// 0x4314b4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFSsSsSsESsLi2EED1Ev
pub fn stub_4314b4() -> ! {
    todo!("0x4314b4 RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::~BoundYieldFuncDesc()")
}

// 0x4314fc — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsbELi2EED1Ev
pub fn stub_4314fc() -> ! {
    todo!("0x4314fc RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,bool),2>::~BoundFuncDesc()")
}

// 0x431544 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFSsSsSsbELi3EED1Ev
pub fn stub_431544() -> ! {
    todo!("0x431544 RBX::Reflection::BoundFuncDesc<RBX::DataModel,std::string ()(std::string,std::string,bool),3>::~BoundFuncDesc()")
}

// 0x431594 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev
pub fn stub_431594() -> ! {
    todo!("0x431594 RBX::Reflection::BoundFuncDesc<RBX::DataModel,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")
}

// 0x4315b8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsSsSsSsELi5EED1Ev
pub fn stub_4315b8() -> ! {
    todo!("0x4315b8 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string,std::string,std::string,std::string),5>::~BoundFuncDesc()")
}

// 0x431618 — __ZNK3RBX9DataModel19getIsPersonalServerEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getIsPersonalServer(void)const")]
// was: __ZNK3RBX9DataModel19getIsPersonalServerEv
pub fn stub_431618() -> ! {
    todo!("0x431618 RBX::DataModel::getIsPersonalServer(void)const")
}

// 0x431620 — __ZN3RBX9DataModel19setIsPersonalServerEb
// type: int __fastcall(int this, bool)
#[doc(alias = "RBX::DataModel::setIsPersonalServer(bool)")]
// was: __ZN3RBX9DataModel19setIsPersonalServerEb
pub fn stub_431620() -> ! {
    todo!("0x431620 RBX::DataModel::setIsPersonalServer(bool)")
}

// 0x431628 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEbED1Ev
pub fn stub_431628() -> ! {
    todo!("0x431628 RBX::Reflection::PropDescriptor<RBX::DataModel,bool>::~PropDescriptor()")
}

// 0x43164c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_9DataModelEFbvEbLi0EED1Ev
pub fn stub_43164c() -> ! {
    todo!("0x43164c RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::~BoundYieldFuncDesc()")
}

// 0x431670 — __ZN3RBX10Reflection17BoundCallbackDescIFbvEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")]
// was: __ZN3RBX10Reflection17BoundCallbackDescIFbvEED1Ev
pub fn stub_431670() -> ! {
    todo!("0x431670 RBX::Reflection::BoundCallbackDesc<bool ()(void)>::~BoundCallbackDesc()")
}

// 0x431768 — __ZN3RBX9DataModel22setUiMessageBrickCountEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::setUiMessageBrickCount(void)")]
// was: __ZN3RBX9DataModel22setUiMessageBrickCountEv
pub fn stub_431768() -> ! {
    todo!("0x431768 RBX::DataModel::setUiMessageBrickCount(void)")
}

// 0x43177c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFdSsdELi2EED1Ev
pub fn stub_43177c() -> ! {
    todo!("0x43177c RBX::Reflection::BoundFuncDesc<RBX::DataModel,double ()(std::string,double),2>::~BoundFuncDesc()")
}

// 0x4317c4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvdELi1EED1Ev
pub fn stub_4317c4() -> ! {
    todo!("0x4317c4 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(double),1>::~BoundFuncDesc()")
}

// 0x431804 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvibELi2EED1Ev
pub fn stub_431804() -> ! {
    todo!("0x431804 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,bool),2>::~BoundFuncDesc()")
}

// 0x43184c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFviNS2_11CreatorTypeEELi2EED1Ev
pub fn stub_43184c() -> ! {
    todo!("0x43184c RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(int,RBX::DataModel::CreatorType),2>::~BoundFuncDesc()")
}

// 0x431894 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_5GenreEELi1EED1Ev
pub fn stub_431894() -> ! {
    todo!("0x431894 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::Genre),1>::~BoundFuncDesc()")
}

// 0x4318d4 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvNS2_16GearGenreSettingEiELi2EED1Ev
pub fn stub_4318d4() -> ! {
    todo!("0x4318d4 RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(RBX::DataModel::GearGenreSetting,int),2>::~BoundFuncDesc()")
}

// 0x43191c — __ZNK3RBX9DataModel12getWorkspaceEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getWorkspace(void)const")]
// was: __ZNK3RBX9DataModel12getWorkspaceEv
pub fn stub_43191c() -> ! {
    todo!("0x43191c RBX::DataModel::getWorkspace(void)const")
}

// 0x431924 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_9WorkspaceEED1Ev
pub fn stub_431924() -> ! {
    todo!("0x431924 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Workspace>::~RefPropDescriptor()")
}

// 0x431950 — __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_9DataModelENS_8InstanceEED1Ev
pub fn stub_431950() -> ! {
    todo!("0x431950 RBX::Reflection::RefPropDescriptor<RBX::DataModel,RBX::Instance>::~RefPropDescriptor()")
}

// 0x43197c — __ZNK3RBX9DataModel10getPlaceIDEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getPlaceID(void)const")]
// was: __ZNK3RBX9DataModel10getPlaceIDEv
pub fn stub_43197c() -> ! {
    todo!("0x43197c RBX::DataModel::getPlaceID(void)const")
}

// 0x431984 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelEiED1Ev
pub fn stub_431984() -> ! {
    todo!("0x431984 RBX::Reflection::PropDescriptor<RBX::DataModel,int>::~PropDescriptor()")
}

// 0x4319a8 — __ZNK3RBX9DataModel15getPlaceVersionEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getPlaceVersion(void)const")]
// was: __ZNK3RBX9DataModel15getPlaceVersionEv
pub fn stub_4319a8() -> ! {
    todo!("0x4319a8 RBX::DataModel::getPlaceVersion(void)const")
}

// 0x4319b0 — __ZNK3RBX9DataModel12getCreatorIDEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getCreatorID(void)const")]
// was: __ZNK3RBX9DataModel12getCreatorIDEv
pub fn stub_4319b0() -> ! {
    todo!("0x4319b0 RBX::DataModel::getCreatorID(void)const")
}

// 0x4319b8 — __ZNK3RBX9DataModel14getCreatorTypeEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getCreatorType(void)const")]
// was: __ZNK3RBX9DataModel14getCreatorTypeEv
pub fn stub_4319b8() -> ! {
    todo!("0x4319b8 RBX::DataModel::getCreatorType(void)const")
}

// 0x4319c0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED1Ev
pub fn stub_4319c0() -> ! {
    todo!("0x4319c0 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")
}

// 0x4319e4 — __ZNK3RBX9DataModel8getGenreEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getGenre(void)const")]
// was: __ZNK3RBX9DataModel8getGenreEv
pub fn stub_4319e4() -> ! {
    todo!("0x4319e4 RBX::DataModel::getGenre(void)const")
}

// 0x4319ec — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED1Ev
pub fn stub_4319ec() -> ! {
    todo!("0x4319ec RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")
}

// 0x431a10 — __ZNK3RBX9DataModel19getGearGenreSettingEv
// type: int __fastcall(RBX::DataModel *this)
#[doc(alias = "RBX::DataModel::getGearGenreSetting(void)const")]
// was: __ZNK3RBX9DataModel19getGearGenreSettingEv
pub fn stub_431a10() -> ! {
    todo!("0x431a10 RBX::DataModel::getGearGenreSetting(void)const")
}

// 0x431a18 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED1Ev
pub fn stub_431a18() -> ! {
    todo!("0x431a18 RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")
}

// 0x431a3c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED1Ev
pub fn stub_431a3c() -> ! {
    todo!("0x431a3c RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")
}

// 0x431a7c — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
pub fn stub_431a7c() -> ! {
    todo!("0x431a7c RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")
}

// 0x431aa0 — __ZNK3RBX9DataModel8getJobIdEv
// type: int __fastcall(RBX::DataModel *this, int)
#[doc(alias = "RBX::DataModel::getJobId(void)const")]
// was: __ZNK3RBX9DataModel8getJobIdEv
pub fn stub_431aa0() -> ! {
    todo!("0x431aa0 RBX::DataModel::getJobId(void)const")
}

// 0x431ab0 — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED1Ev
pub fn stub_431ab0() -> ! {
    todo!("0x431ab0 RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::~PropDescriptor()")
}

// 0x431ad4 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
pub fn stub_431ad4() -> ! {
    todo!("0x431ad4 RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc()")
}