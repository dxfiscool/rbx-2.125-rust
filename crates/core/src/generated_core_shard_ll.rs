//! core shard ll — 150 core stubs EA-sorted, next uncovered fallback after shard lk (0x41f360..0x45c02c, lowest EA first).
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 41432, 9232->9082 uncovered, 37999->38149 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::DataModel::onlyJobsLeftForThisArbiterAreGenericJobs(void)")]
// 0x41f360 — __ZN3RBX9DataModel40onlyJobsLeftForThisArbiterAreGenericJobsEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x41f360() -> ! {
    todo!("0x41f360 __ZN3RBX9DataModel40onlyJobsLeftForThisArbiterAreGenericJobsEv")
}

#[doc(alias = "RBX::DataModel::DataModel(RBX::Verb *,RBX::DataModel*)")]
// 0x41fd78 — __ZN3RBX9DataModelC2EPNS_4VerbEPS0_
// type: int __fastcall(RBX::DataModel *this, RBX::Verb *, RBX::DataModel *)
pub fn stub_0x41fd78() -> ! {
    todo!("0x41fd78 __ZN3RBX9DataModelC2EPNS_4VerbEPS0_")
}

#[doc(alias = "RBX::DataModel::onRunTransition(RBX::RunTransition)")]
// 0x420fec — __ZN3RBX9DataModel15onRunTransitionENS_13RunTransitionE
// type: int __fastcall(int, int, int)
pub fn stub_0x420fec() -> ! {
    todo!("0x420fec __ZN3RBX9DataModel15onRunTransitionENS_13RunTransitionE")
}

#[doc(alias = "RBX::DataModel::loadCoreScripts(void)")]
// 0x4210d8 — __ZN3RBX9DataModel15loadCoreScriptsEv
// type: void __fastcall(RBX::DataModel *this)
pub fn stub_0x4210d8() -> ! {
    todo!("0x4210d8 __ZN3RBX9DataModel15loadCoreScriptsEv")
}

#[doc(alias = "RBX::DataModel::startCoreScripts(RBX::Adorn *,bool)")]
// 0x421b60 — __ZN3RBX9DataModel16startCoreScriptsEPNS_5AdornEb
// type: void __fastcall(RBX::Workspace **this, RBX::Adorn *, bool)
pub fn stub_0x421b60() -> ! {
    todo!("0x421b60 __ZN3RBX9DataModel16startCoreScriptsEPNS_5AdornEb")
}

#[doc(alias = "RBX::DataModel::~DataModel()")]
// 0x421b80 — __ZN3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421b80() -> ! {
    todo!("0x421b80 __ZN3RBX9DataModelD0Ev")
}

#[doc(alias = "RBX::DataModel::~DataModel()")]
// 0x421c20 — __ZN3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c20() -> ! {
    todo!("0x421c20 __ZN3RBX9DataModelD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x421c24 — __ZThn32_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c24() -> ! {
    todo!("0x421c24 __ZThn32_N3RBX9DataModelD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x421c2c — __ZThn36_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c2c() -> ! {
    todo!("0x421c2c __ZThn36_N3RBX9DataModelD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x421c34 — __ZThn144_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c34() -> ! {
    todo!("0x421c34 __ZThn144_N3RBX9DataModelD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x421c3c — __ZThn180_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c3c() -> ! {
    todo!("0x421c3c __ZThn180_N3RBX9DataModelD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x421c44 — __ZThn184_N3RBX9DataModelD0Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c44() -> ! {
    todo!("0x421c44 __ZThn184_N3RBX9DataModelD0Ev")
}

#[doc(alias = "RBX::DataModel::~DataModel()")]
// 0x421c4c — __ZN3RBX9DataModelD2Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x421c4c() -> ! {
    todo!("0x421c4c __ZN3RBX9DataModelD2Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x4228ac — __ZThn32_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x4228ac() -> ! {
    todo!("0x4228ac __ZThn32_N3RBX9DataModelD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x4228b4 — __ZThn36_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x4228b4() -> ! {
    todo!("0x4228b4 __ZThn36_N3RBX9DataModelD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x4228bc — __ZThn144_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x4228bc() -> ! {
    todo!("0x4228bc __ZThn144_N3RBX9DataModelD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x4228c4 — __ZThn180_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x4228c4() -> ! {
    todo!("0x4228c4 __ZThn180_N3RBX9DataModelD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::~DataModel()")]
// 0x4228cc — __ZThn184_N3RBX9DataModelD1Ev
// type: void __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x4228cc() -> ! {
    todo!("0x4228cc __ZThn184_N3RBX9DataModelD1Ev")
}

#[doc(alias = "RBX::DataModel::getGenericJob(RBX::DataModelJob::TaskType)")]
// 0x4228d4 — __ZN3RBX9DataModel13getGenericJobENS_12DataModelJob8TaskTypeE
// type: void __fastcall(sp_counted_base **, int, int)
pub fn stub_0x4228d4() -> ! {
    todo!("0x4228d4 __ZN3RBX9DataModel13getGenericJobENS_12DataModelJob8TaskTypeE")
}

#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")]
// 0x422c64 — __ZN3RBX9DataModel10LegacyLockC1EPS0_NS_12DataModelJob8TaskTypeE
// type: int()
pub fn stub_0x422c64() -> ! {
    todo!("0x422c64 __ZN3RBX9DataModel10LegacyLockC1EPS0_NS_12DataModelJob8TaskTypeE")
}

#[doc(alias = "RBX::DataModel::LegacyLock::LegacyLock(RBX::DataModel*,RBX::DataModelJob::TaskType)")]
// 0x422c68 — __ZN3RBX9DataModel10LegacyLockC2EPS0_NS_12DataModelJob8TaskTypeE
// type: boost::detail::sp_counted_base **__fastcall(boost::detail::sp_counted_base **, int, int)
pub fn stub_0x422c68() -> ! {
    todo!("0x422c68 __ZN3RBX9DataModel10LegacyLockC2EPS0_NS_12DataModelJob8TaskTypeE")
}

#[doc(alias = "RBX::DataModel::doHttpGet(std::string const&)")]
// 0x4234e4 — __ZN3RBX9DataModel9doHttpGetERKSs
// type: void __fastcall(RBX::DataModel *this, const std::string *)
pub fn stub_0x4234e4() -> ! {
    todo!("0x4234e4 __ZN3RBX9DataModel9doHttpGetERKSs")
}

#[doc(alias = "RBX::DataModel::doHttpPost(std::string const&,std::string const&)")]
// 0x4237e4 — __ZN3RBX9DataModel10doHttpPostERKSsS2_
// type: void __fastcall(RBX::DataModel *this, const std::string *, const std::string *)
pub fn stub_0x4237e4() -> ! {
    todo!("0x4237e4 __ZN3RBX9DataModel10doHttpPostERKSsS2_")
}

#[doc(alias = "RBX::DataModel::loadAssetIdIntoStream(int)")]
// 0x423f90 — __ZN3RBX9DataModel21loadAssetIdIntoStreamEi
// type: int __fastcall(RBX::DataModel *this, int, int)
pub fn stub_0x423f90() -> ! {
    todo!("0x423f90 __ZN3RBX9DataModel21loadAssetIdIntoStreamEi")
}

#[doc(alias = "RBX::DataModel::computeGuiInset(RBX::Adorn *)")]
// 0x4243d8 — __ZN3RBX9DataModel15computeGuiInsetEPNS_5AdornE
// type: _DWORD __fastcall(RBX::DataModel *__hidden this, RBX::Adorn *)
pub fn stub_0x4243d8() -> ! {
    todo!("0x4243d8 __ZN3RBX9DataModel15computeGuiInsetEPNS_5AdornE")
}

#[doc(alias = "RBX::DataModel::renderPlayerGui(RBX::Adorn *)")]
// 0x42442c — __ZN3RBX9DataModel15renderPlayerGuiEPNS_5AdornE
// type: unsigned int __fastcall(RBX::StarterGuiService **this, RBX::Adorn *)
pub fn stub_0x42442c() -> ! {
    todo!("0x42442c __ZN3RBX9DataModel15renderPlayerGuiEPNS_5AdornE")
}

#[doc(alias = "RBX::DataModel::renderGuiRoot(RBX::Adorn *)")]
// 0x4244c0 — __ZN3RBX9DataModel13renderGuiRootEPNS_5AdornE
// type: int __fastcall(RBX::DataModel *this, RBX::Adorn *)
pub fn stub_0x4244c0() -> ! {
    todo!("0x4244c0 __ZN3RBX9DataModel13renderGuiRootEPNS_5AdornE")
}

#[doc(alias = "RBX::DataModel::getUpdatedMessageBoxText(void)")]
// 0x424510 — __ZN3RBX9DataModel24getUpdatedMessageBoxTextEv
// type: void __fastcall(RBX::DataModel *this, RBX::Workspace **)
pub fn stub_0x424510() -> ! {
    todo!("0x424510 __ZN3RBX9DataModel24getUpdatedMessageBoxTextEv")
}

#[doc(alias = "RBX::DataModel::renderMessageBox(RBX::Adorn *)")]
// 0x424ed0 — __ZN3RBX9DataModel16renderMessageBoxEPNS_5AdornE
// type: void __fastcall(RBX::Workspace **this, RBX::Adorn *)
pub fn stub_0x424ed0() -> ! {
    todo!("0x424ed0 __ZN3RBX9DataModel16renderMessageBoxEPNS_5AdornE")
}

#[doc(alias = "RBX::DataModel::renderPass2d(RBX::Adorn *,RBX::IMetric *)")]
// 0x4251d8 — __ZN3RBX9DataModel12renderPass2dEPNS_5AdornEPNS_7IMetricE
// type: int __fastcall(RBX::DataModel *this, RBX::Adorn *, RBX::IMetric *)
pub fn stub_0x4251d8() -> ! {
    todo!("0x4251d8 __ZN3RBX9DataModel12renderPass2dEPNS_5AdornEPNS_7IMetricE")
}

#[doc(alias = "RBX::DataModel::getRenderMouseCursor(void)")]
// 0x4252ec — __ZN3RBX9DataModel20getRenderMouseCursorEv
// type: _DWORD __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x4252ec() -> ! {
    todo!("0x4252ec __ZN3RBX9DataModel20getRenderMouseCursorEv")
}

#[doc(alias = "RBX::DataModel::renderMouse(RBX::Adorn *)")]
// 0x42538c — __ZN3RBX9DataModel11renderMouseEPNS_5AdornE
// type: void __fastcall(RBX::DataModel *this, RBX::Adorn *, int, int)
pub fn stub_0x42538c() -> ! {
    todo!("0x42538c __ZN3RBX9DataModel11renderMouseEPNS_5AdornE")
}

#[doc(alias = "RBX::DataModel::renderPass3dAdorn(RBX::Adorn *)")]
// 0x425590 — __ZN3RBX9DataModel17renderPass3dAdornEPNS_5AdornE
// type: void __fastcall(RBX::DataModel *this, RBX::Adorn *)
pub fn stub_0x425590() -> ! {
    todo!("0x425590 __ZN3RBX9DataModel17renderPass3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::DataModel::physicsStep(float,double,double,int)")]
// 0x4259d0 — __ZN3RBX9DataModel11physicsStepEfddi
// type: void __fastcall(RBX::DataModel *this, float, double, double, int)
pub fn stub_0x4259d0() -> ! {
    todo!("0x4259d0 __ZN3RBX9DataModel11physicsStepEfddi")
}

#[doc(alias = "RBX::DataModel::updatePhysicsInstructions(RBX::Network::GameMode)")]
// 0x425d58 — __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE
// type: int __fastcall(int, int)
pub fn stub_0x425d58() -> ! {
    todo!("0x425d58 __ZN3RBX9DataModel25updatePhysicsInstructionsENS_7Network8GameModeE")
}

#[doc(alias = "RBX::DataModel::processAccelerators(RBX::GuiEvent const&)")]
// 0x4260d8 — __ZN3RBX9DataModel19processAcceleratorsERKNS_8GuiEventE
// type: void __fastcall(RBX::OnScreenProfiler *, int, _DWORD *, int)
pub fn stub_0x4260d8() -> ! {
    todo!("0x4260d8 __ZN3RBX9DataModel19processAcceleratorsERKNS_8GuiEventE")
}

#[doc(alias = "RBX::DataModel::switchViewMode(void)")]
// 0x427054 — __ZN3RBX9DataModel14switchViewModeEv
// type: void __fastcall(RBX::DataModel *this, int, int, int)
pub fn stub_0x427054() -> ! {
    todo!("0x427054 __ZN3RBX9DataModel14switchViewModeEv")
}

#[doc(alias = "RBX::DataModel::processPlayerGui(RBX::GuiEvent const&)")]
// 0x42738c — __ZN3RBX9DataModel16processPlayerGuiERKNS_8GuiEventE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x42738c() -> ! {
    todo!("0x42738c __ZN3RBX9DataModel16processPlayerGuiERKNS_8GuiEventE")
}

#[doc(alias = "RBX::DataModel::processCameraCommands(RBX::GuiEvent const&)")]
// 0x4273c0 — __ZN3RBX9DataModel21processCameraCommandsERKNS_8GuiEventE
// type: void __fastcall(_QWORD *, int, _DWORD *)
pub fn stub_0x4273c0() -> ! {
    todo!("0x4273c0 __ZN3RBX9DataModel21processCameraCommandsERKNS_8GuiEventE")
}

#[doc(alias = "RBX::DataModel::processEvent(RBX::UIEvent const&)")]
// 0x4275e0 — __ZN3RBX9DataModel12processEventERKNS_7UIEventE
// type: int __fastcall(RBX::DataModel *this, const RBX::UIEvent *, int, const void *)
pub fn stub_0x4275e0() -> ! {
    todo!("0x4275e0 __ZN3RBX9DataModel12processEventERKNS_7UIEventE")
}

#[doc(alias = "RBX::DataModel::processWorkspaceEvent(RBX::UIEvent const&)")]
// 0x427b54 — __ZN3RBX9DataModel21processWorkspaceEventERKNS_7UIEventE
// type: int __fastcall(RBX::DataModel *this, const RBX::UIEvent *, int, const void *)
pub fn stub_0x427b54() -> ! {
    todo!("0x427b54 __ZN3RBX9DataModel21processWorkspaceEventERKNS_7UIEventE")
}

#[doc(alias = "RBX::DataModel::processUiEvent(RBX::UIEvent const&)")]
// 0x427bac — __ZN3RBX9DataModel14processUiEventERKNS_7UIEventE
// type: int __fastcall(RBX::DataModel *this, const RBX::UIEvent *, int, int)
pub fn stub_0x427bac() -> ! {
    todo!("0x427bac __ZN3RBX9DataModel14processUiEventERKNS_7UIEventE")
}

#[doc(alias = "RBX::DataModel::setNetworkMetric(RBX::IMetric *)")]
// 0x427db8 — __ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE
// type: int __fastcall(int this, IMetric *)
pub fn stub_0x427db8() -> ! {
    todo!("0x427db8 __ZN3RBX9DataModel16setNetworkMetricEPNS_7IMetricE")
}

#[doc(alias = "RBX::DataModel::getMetricValue(std::string const&)const")]
// 0x427dc0 — __ZNK3RBX9DataModel14getMetricValueERKSs
// type: __int64 __fastcall(RBX::DataModel *this, const std::string *)
pub fn stub_0x427dc0() -> ! {
    todo!("0x427dc0 __ZNK3RBX9DataModel14getMetricValueERKSs")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::getMetricValue(std::string const&)const")]
// 0x4288b0 — __ZThn180_NK3RBX9DataModel14getMetricValueERKSs
// type: __int64 __fastcall(RBX::DataModel *this, const std::string *)
pub fn stub_0x4288b0() -> ! {
    todo!("0x4288b0 __ZThn180_NK3RBX9DataModel14getMetricValueERKSs")
}

#[doc(alias = "RBX::DataModel::getMetric(std::string const&)const")]
// 0x4288b8 — __ZNK3RBX9DataModel9getMetricERKSs
// type: void __fastcall(RBX::DataModel *this, pthread_mutex_t *, std::string *)
pub fn stub_0x4288b8() -> ! {
    todo!("0x4288b8 __ZNK3RBX9DataModel9getMetricERKSs")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::getMetric(std::string const&)const")]
// 0x42fb24 — __ZThn180_NK3RBX9DataModel9getMetricERKSs
// type: void __fastcall(RBX::DataModel *this, const std::string *, std::string *)
pub fn stub_0x42fb24() -> ! {
    todo!("0x42fb24 __ZThn180_NK3RBX9DataModel9getMetricERKSs")
}

#[doc(alias = "RBX::DataModel::gameLoaded(void)")]
// 0x430004 — __ZN3RBX9DataModel10gameLoadedEv
// type: int __fastcall(int this)
pub fn stub_0x430004() -> ! {
    todo!("0x430004 __ZN3RBX9DataModel10gameLoadedEv")
}

#[doc(alias = "RBX::DataModel::getNumPlayers(void)const")]
// 0x430900 — __ZNK3RBX9DataModel13getNumPlayersEv
// type: _DWORD __fastcall(RBX::DataModel *__hidden this)
pub fn stub_0x430900() -> ! {
    todo!("0x430900 __ZNK3RBX9DataModel13getNumPlayersEv")
}

#[doc(alias = "`non-virtual thunk toRBX::DataModel::getNumPlayers(void)const")]
// 0x430924 — __ZThn184_NK3RBX9DataModel13getNumPlayersEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x430924() -> ! {
    todo!("0x430924 __ZThn184_NK3RBX9DataModel13getNumPlayersEv")
}

#[doc(alias = "RBX::DataModel::currentThreadHasWriteLock(void)const")]
// 0x4309f8 — __ZNK3RBX9DataModel25currentThreadHasWriteLockEv
// type: bool __fastcall(RBX::DataModel *this)
pub fn stub_0x4309f8() -> ! {
    todo!("0x4309f8 __ZNK3RBX9DataModel25currentThreadHasWriteLockEv")
}

#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
// 0x430b28 — __ZN3RBX9DataModel20scoped_write_requestD1Ev
// type: void __fastcall(RBX::DataModel::scoped_write_request *__hidden this)
pub fn stub_0x430b28() -> ! {
    todo!("0x430b28 __ZN3RBX9DataModel20scoped_write_requestD1Ev")
}

#[doc(alias = "RBX::DataModel::scoped_write_request::~scoped_write_request()")]
// 0x430b2c — __ZN3RBX9DataModel20scoped_write_requestD2Ev
// type: void __fastcall(RBX::DataModel::scoped_write_request *__hidden this)
pub fn stub_0x430b2c() -> ! {
    todo!("0x430b2c __ZN3RBX9DataModel20scoped_write_requestD2Ev")
}

#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
// 0x430d0c — __ZN3RBX9DataModel19scoped_read_requestD1Ev
// type: void __fastcall(RBX::DataModel::scoped_read_request *__hidden this)
pub fn stub_0x430d0c() -> ! {
    todo!("0x430d0c __ZN3RBX9DataModel19scoped_read_requestD1Ev")
}

#[doc(alias = "RBX::DataModel::scoped_read_request::~scoped_read_request()")]
// 0x430d10 — __ZN3RBX9DataModel19scoped_read_requestD2Ev
// type: void __fastcall(RBX::DataModel::scoped_read_request *__hidden this)
pub fn stub_0x430d10() -> ! {
    todo!("0x430d10 __ZN3RBX9DataModel19scoped_read_requestD2Ev")
}

#[doc(alias = "RBX::DataModel::allHackFlagsOredTogether(void)")]
// 0x430df4 — __ZN3RBX9DataModel24allHackFlagsOredTogetherEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x430df4() -> ! {
    todo!("0x430df4 __ZN3RBX9DataModel24allHackFlagsOredTogetherEv")
}

#[doc(alias = "RBX::DataModel::loadPlugins(void)")]
// 0x431278 — __ZN3RBX9DataModel11loadPluginsEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x431278() -> ! {
    todo!("0x431278 __ZN3RBX9DataModel11loadPluginsEv")
}

#[doc(alias = "RBX::DataModel::getIsPersonalServer(void)const")]
// 0x431618 — __ZNK3RBX9DataModel19getIsPersonalServerEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x431618() -> ! {
    todo!("0x431618 __ZNK3RBX9DataModel19getIsPersonalServerEv")
}

#[doc(alias = "RBX::DataModel::setIsPersonalServer(bool)")]
// 0x431620 — __ZN3RBX9DataModel19setIsPersonalServerEb
// type: int __fastcall(int this, bool)
pub fn stub_0x431620() -> ! {
    todo!("0x431620 __ZN3RBX9DataModel19setIsPersonalServerEb")
}

#[doc(alias = "RBX::DataModel::setUiMessageBrickCount(void)")]
// 0x431768 — __ZN3RBX9DataModel22setUiMessageBrickCountEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x431768() -> ! {
    todo!("0x431768 __ZN3RBX9DataModel22setUiMessageBrickCountEv")
}

#[doc(alias = "RBX::DataModel::getWorkspace(void)const")]
// 0x43191c — __ZNK3RBX9DataModel12getWorkspaceEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x43191c() -> ! {
    todo!("0x43191c __ZNK3RBX9DataModel12getWorkspaceEv")
}

#[doc(alias = "RBX::DataModel::getPlaceID(void)const")]
// 0x43197c — __ZNK3RBX9DataModel10getPlaceIDEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x43197c() -> ! {
    todo!("0x43197c __ZNK3RBX9DataModel10getPlaceIDEv")
}

#[doc(alias = "RBX::DataModel::getPlaceVersion(void)const")]
// 0x4319a8 — __ZNK3RBX9DataModel15getPlaceVersionEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x4319a8() -> ! {
    todo!("0x4319a8 __ZNK3RBX9DataModel15getPlaceVersionEv")
}

#[doc(alias = "RBX::DataModel::getCreatorID(void)const")]
// 0x4319b0 — __ZNK3RBX9DataModel12getCreatorIDEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x4319b0() -> ! {
    todo!("0x4319b0 __ZNK3RBX9DataModel12getCreatorIDEv")
}

#[doc(alias = "RBX::DataModel::getCreatorType(void)const")]
// 0x4319b8 — __ZNK3RBX9DataModel14getCreatorTypeEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x4319b8() -> ! {
    todo!("0x4319b8 __ZNK3RBX9DataModel14getCreatorTypeEv")
}

#[doc(alias = "RBX::DataModel::getGenre(void)const")]
// 0x4319e4 — __ZNK3RBX9DataModel8getGenreEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x4319e4() -> ! {
    todo!("0x4319e4 __ZNK3RBX9DataModel8getGenreEv")
}

#[doc(alias = "RBX::DataModel::getGearGenreSetting(void)const")]
// 0x431a10 — __ZNK3RBX9DataModel19getGearGenreSettingEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x431a10() -> ! {
    todo!("0x431a10 __ZNK3RBX9DataModel19getGearGenreSettingEv")
}

#[doc(alias = "RBX::DataModel::getJobId(void)const")]
// 0x431aa0 — __ZNK3RBX9DataModel8getJobIdEv
// type: int __fastcall(RBX::DataModel *this, int)
pub fn stub_0x431aa0() -> ! {
    todo!("0x431aa0 __ZNK3RBX9DataModel8getJobIdEv")
}

#[doc(alias = "RBX::DataModel::getIsGameLoaded(void)")]
// 0x431af8 — __ZN3RBX9DataModel15getIsGameLoadedEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x431af8() -> ! {
    todo!("0x431af8 __ZN3RBX9DataModel15getIsGameLoadedEv")
}

#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::create<RBX::ScriptService>(void)const")]
// 0x436c10 — __ZNK3RBX15ServiceProvider6createINS_13ScriptServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x436c10() -> ! {
    todo!("0x436c10 __ZNK3RBX15ServiceProvider6createINS_13ScriptServiceEEEPT_v")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")]
// 0x43876c — __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv
pub fn stub_0x43876c() -> ! {
    todo!("0x43876c __ZNK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")]
// 0x438798 — __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv
pub fn stub_0x438798() -> ! {
    todo!("0x438798 __ZThn32_NK3RBX17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEE12getClassNameEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearType>(RBX::DataModel::GearType const&)")]
// 0x439c24 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel8GearTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x439c24() -> ! {
    todo!("0x439c24 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel8GearTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::singleton(void)")]
// 0x439c74 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE9singletonEv
// type: _DWORD *()
pub fn stub_0x439c74() -> ! {
    todo!("0x439c74 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::construct_func(char const*,char *)")]
// 0x439ce0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x439ce0() -> ! {
    todo!("0x439ce0 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearType>::destruct_func(char *)")]
// 0x439cec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE13destruct_funcEPc
// type: void()
pub fn stub_0x439cec() -> ! {
    todo!("0x439cec __ZN3rbx14implementation12typed_holderIN3RBX9DataModel8GearTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DataModel::GearType const& rbx::any_cast<RBX::DataModel::GearType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x439dbc — __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x439dbc() -> ! {
    todo!("0x439dbc __ZN3rbx8any_castIRKN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::GearGenreSetting>(RBX::DataModel::GearGenreSetting const&)")]
// 0x43a29c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x43a29c() -> ! {
    todo!("0x43a29c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel16GearGenreSettingEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::singleton(void)")]
// 0x43a2ec — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv
// type: _DWORD *()
pub fn stub_0x43a2ec() -> ! {
    todo!("0x43a2ec __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::construct_func(char const*,char *)")]
// 0x43a358 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x43a358() -> ! {
    todo!("0x43a358 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::GearGenreSetting>::destruct_func(char *)")]
// 0x43a364 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc
// type: void()
pub fn stub_0x43a364() -> ! {
    todo!("0x43a364 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel16GearGenreSettingEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DataModel::GearGenreSetting const& rbx::any_cast<RBX::DataModel::GearGenreSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x43a434 — __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x43a434() -> ! {
    todo!("0x43a434 __ZN3rbx8any_castIRKN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::Genre>(RBX::DataModel::Genre const&)")]
// 0x43a914 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x43a914() -> ! {
    todo!("0x43a914 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel5GenreEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::singleton(void)")]
// 0x43a964 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv
// type: _DWORD *()
pub fn stub_0x43a964() -> ! {
    todo!("0x43a964 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::construct_func(char const*,char *)")]
// 0x43a9d0 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x43a9d0() -> ! {
    todo!("0x43a9d0 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::Genre>::destruct_func(char *)")]
// 0x43a9dc — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc
// type: void()
pub fn stub_0x43a9dc() -> ! {
    todo!("0x43a9dc __ZN3rbx14implementation12typed_holderIN3RBX9DataModel5GenreEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DataModel::Genre const& rbx::any_cast<RBX::DataModel::Genre const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x43aaac — __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x43aaac() -> ! {
    todo!("0x43aaac __ZN3rbx8any_castIRKN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DataModel::CreatorType>(RBX::DataModel::CreatorType const&)")]
// 0x43af8c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel11CreatorTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x43af8c() -> ! {
    todo!("0x43af8c __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9DataModel11CreatorTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::singleton(void)")]
// 0x43afdc — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE9singletonEv
// type: _DWORD *()
pub fn stub_0x43afdc() -> ! {
    todo!("0x43afdc __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::construct_func(char const*,char *)")]
// 0x43b048 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x43b048() -> ! {
    todo!("0x43b048 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DataModel::CreatorType>::destruct_func(char *)")]
// 0x43b054 — __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE13destruct_funcEPc
// type: void()
pub fn stub_0x43b054() -> ! {
    todo!("0x43b054 __ZN3rbx14implementation12typed_holderIN3RBX9DataModel11CreatorTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DataModel::CreatorType const& rbx::any_cast<RBX::DataModel::CreatorType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x43b124 — __ZN3rbx8any_castIRKN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x43b124() -> ! {
    todo!("0x43b124 __ZN3rbx8any_castIRKN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv")]
// 0x43b464 — __ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv
pub fn stub_0x43b464() -> ! {
    todo!("0x43b464 __ZN3RBX4Name13callDoDeclareILZNS_10sDataModelEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v")]
// 0x43b468 — __ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v
// type: int()
pub fn stub_0x43b468() -> ! {
    todo!("0x43b468 __ZN3RBX4Name9doDeclareILZNS_10sDataModelEEEERKS0_v")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptContext>(void)")]
// 0x43c178 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv
// type: int()
pub fn stub_0x43c178() -> ! {
    todo!("0x43c178 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptContextEEEmv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")]
// 0x44387c — __ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x44387c() -> ! {
    todo!("0x44387c __ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv")]
// 0x4438c0 — __ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv
pub fn stub_0x4438c0() -> ! {
    todo!("0x4438c0 __ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")]
// 0x4438c4 — __ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v
// type: int()
pub fn stub_0x4438c4() -> ! {
    todo!("0x4438c4 __ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")
}

#[doc(alias = "RBX::ServerScriptService * RBX::ServiceProvider::find<RBX::ServerScriptService>(void)const")]
// 0x443bd0 — __ZNK3RBX15ServiceProvider4findINS_19ServerScriptServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x443bd0() -> ! {
    todo!("0x443bd0 __ZNK3RBX15ServiceProvider4findINS_19ServerScriptServiceEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerScriptService>(void)")]
// 0x443e20 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19ServerScriptServiceEEEvv
pub fn stub_0x443e20() -> ! {
    todo!("0x443e20 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_19ServerScriptServiceEEEvv")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerScriptService>(void)")]
// 0x443e24 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_19ServerScriptServiceEEEmv
// type: int()
pub fn stub_0x443e24() -> ! {
    todo!("0x443e24 __ZN3RBX15ServiceProvider15doGetClassIndexINS_19ServerScriptServiceEEEmv")
}

#[doc(alias = "RBX::DataModel::LegacyLock::Implementation::~Implementation()")]
// 0x4484dc — __ZN3RBX9DataModel10LegacyLock14ImplementationD2Ev
// type: void __fastcall(RBX::DataModel::LegacyLock::Implementation *this, int, int, int)
pub fn stub_0x4484dc() -> ! {
    todo!("0x4484dc __ZN3RBX9DataModel10LegacyLock14ImplementationD2Ev")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv")]
// 0x449180 — __ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv
// type: int()
pub fn stub_0x449180() -> ! {
    todo!("0x449180 __ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ScriptService>(void)")]
// 0x449188 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ScriptServiceEEEvv
pub fn stub_0x449188() -> ! {
    todo!("0x449188 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ScriptServiceEEEvv")
}

#[doc(alias = "RBX::ScriptService::~ScriptService()")]
// 0x44918c — __ZN3RBX13ScriptServiceD1Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
pub fn stub_0x44918c() -> ! {
    todo!("0x44918c __ZN3RBX13ScriptServiceD1Ev")
}

#[doc(alias = "RBX::ScriptService::~ScriptService()")]
// 0x449270 — __ZN3RBX13ScriptServiceD0Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
pub fn stub_0x449270() -> ! {
    todo!("0x449270 __ZN3RBX13ScriptServiceD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// 0x44936c — __ZThn32_N3RBX13ScriptServiceD1Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
pub fn stub_0x44936c() -> ! {
    todo!("0x44936c __ZThn32_N3RBX13ScriptServiceD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// 0x449450 — __ZThn32_N3RBX13ScriptServiceD0Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
pub fn stub_0x449450() -> ! {
    todo!("0x449450 __ZThn32_N3RBX13ScriptServiceD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// 0x44954c — __ZThn36_N3RBX13ScriptServiceD1Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
pub fn stub_0x44954c() -> ! {
    todo!("0x44954c __ZThn36_N3RBX13ScriptServiceD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::ScriptService::~ScriptService()")]
// 0x449630 — __ZThn36_N3RBX13ScriptServiceD0Ev
// type: void __fastcall(RBX::ScriptService *__hidden this)
pub fn stub_0x449630() -> ! {
    todo!("0x449630 __ZThn36_N3RBX13ScriptServiceD0Ev")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v")]
// 0x44c420 — __ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v
// type: int(void)
pub fn stub_0x44c420() -> ! {
    todo!("0x44c420 __ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v")]
// 0x44c468 — __ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v
// type: int()
pub fn stub_0x44c468() -> ! {
    todo!("0x44c468 __ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptInformationProvider>(void)")]
// 0x44c550 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv
// type: int()
pub fn stub_0x44c550() -> ! {
    todo!("0x44c550 __ZN3RBX15ServiceProvider15doGetClassIndexINS_25ScriptInformationProviderEEEmv")
}

#[doc(alias = "RBX::DataModel::GearType * rbx::any_cast<RBX::DataModel::GearType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x45ad84 — __ZN3rbx8any_castIN3RBX9DataModel8GearTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45ad84() -> ! {
    todo!("0x45ad84 __ZN3rbx8any_castIN3RBX9DataModel8GearTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::DataModel::GearType & rbx::any_cast<RBX::DataModel::GearType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x45addc — __ZN3rbx8any_castIRN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45addc() -> ! {
    todo!("0x45addc __ZN3rbx8any_castIRN3RBX9DataModel8GearTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::resize(unsigned long,RBX::DataModel::GearType)")]
// 0x45aecc — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x45aecc() -> ! {
    todo!("0x45aecc __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::push_back(RBX::DataModel::GearType const&)")]
// 0x45af00 — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45af00() -> ! {
    todo!("0x45af00 __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::GearType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::operator[](RBX::Name const* const&)")]
// 0x45af28 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel8GearTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45af28() -> ! {
    todo!("0x45af28 __ZNSt3mapIPKN3RBX4NameENS0_9DataModel8GearTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
// 0x45af80 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x45af80() -> ! {
    todo!("0x45af80 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
// 0x45b034 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x45b034() -> ! {
    todo!("0x45b034 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
// 0x45b08c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_0x45b08c() -> ! {
    todo!("0x45b08c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel8GearTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,RBX::DataModel::GearType const&)")]
// 0x45b0f4 — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x45b0f4() -> ! {
    todo!("0x45b0f4 __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_allocate(unsigned long)")]
// 0x45b1d8 — __ZNSt12_Vector_baseIN3RBX9DataModel8GearTypeESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x45b1d8() -> ! {
    todo!("0x45b1d8 __ZNSt12_Vector_baseIN3RBX9DataModel8GearTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DataModel::GearType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearType *,RBX::DataModel::GearType *>(RBX::DataModel::GearType *,RBX::DataModel::GearType *,RBX::DataModel::GearType *)")]
// 0x45b1f0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel8GearTypeES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_0x45b1f0() -> ! {
    todo!("0x45b1f0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel8GearTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,unsigned long,RBX::DataModel::GearType const&)")]
// 0x45b22c — __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
pub fn stub_0x45b22c() -> ! {
    todo!("0x45b22c __ZNSt6vectorIN3RBX9DataModel8GearTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::DataModel::GearGenreSetting * rbx::any_cast<RBX::DataModel::GearGenreSetting,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x45b3bc — __ZN3rbx8any_castIN3RBX9DataModel16GearGenreSettingENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45b3bc() -> ! {
    todo!("0x45b3bc __ZN3rbx8any_castIN3RBX9DataModel16GearGenreSettingENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::DataModel::GearGenreSetting & rbx::any_cast<RBX::DataModel::GearGenreSetting &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x45b414 — __ZN3rbx8any_castIRN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45b414() -> ! {
    todo!("0x45b414 __ZN3rbx8any_castIRN3RBX9DataModel16GearGenreSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::resize(unsigned long,RBX::DataModel::GearGenreSetting)")]
// 0x45b504 — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x45b504() -> ! {
    todo!("0x45b504 __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::push_back(RBX::DataModel::GearGenreSetting const&)")]
// 0x45b538 — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x45b538() -> ! {
    todo!("0x45b538 __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::GearGenreSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::operator[](RBX::Name const* const&)")]
// 0x45b560 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel16GearGenreSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45b560() -> ! {
    todo!("0x45b560 __ZNSt3mapIPKN3RBX4NameENS0_9DataModel16GearGenreSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
// 0x45b5b8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x45b5b8() -> ! {
    todo!("0x45b5b8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
// 0x45b66c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
pub fn stub_0x45b66c() -> ! {
    todo!("0x45b66c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
// 0x45b6c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
pub fn stub_0x45b6c4() -> ! {
    todo!("0x45b6c4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearGenreSetting*,std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>>,RBX::DataModel::GearGenreSetting const&)")]
// 0x45b72c — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x45b72c() -> ! {
    todo!("0x45b72c __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_allocate(unsigned long)")]
// 0x45b810 — __ZNSt12_Vector_baseIN3RBX9DataModel16GearGenreSettingESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
pub fn stub_0x45b810() -> ! {
    todo!("0x45b810 __ZNSt12_Vector_baseIN3RBX9DataModel16GearGenreSettingESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DataModel::GearGenreSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *>(RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *)")]
// 0x45b828 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel16GearGenreSettingES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
pub fn stub_0x45b828() -> ! {
    todo!("0x45b828 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel16GearGenreSettingES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearGenreSetting*,std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>>,unsigned long,RBX::DataModel::GearGenreSetting const&)")]
// 0x45b864 — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x45b864() -> ! {
    todo!("0x45b864 __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::DataModel::Genre * rbx::any_cast<RBX::DataModel::Genre,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x45b9f4 — __ZN3rbx8any_castIN3RBX9DataModel5GenreENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45b9f4() -> ! {
    todo!("0x45b9f4 __ZN3rbx8any_castIN3RBX9DataModel5GenreENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::DataModel::Genre & rbx::any_cast<RBX::DataModel::Genre &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x45ba4c — __ZN3rbx8any_castIRN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45ba4c() -> ! {
    todo!("0x45ba4c __ZN3rbx8any_castIRN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::resize(unsigned long,RBX::DataModel::Genre)")]
// 0x45bb3c — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x45bb3c() -> ! {
    todo!("0x45bb3c __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::push_back(RBX::DataModel::Genre const&)")]
// 0x45bb70 — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45bb70() -> ! {
    todo!("0x45bb70 __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::Genre,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::operator[](RBX::Name const* const&)")]
// 0x45bb98 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel5GenreESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45bb98() -> ! {
    todo!("0x45bb98 __ZNSt3mapIPKN3RBX4NameENS0_9DataModel5GenreESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
// 0x45bbf0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x45bbf0() -> ! {
    todo!("0x45bbf0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
// 0x45bca4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x45bca4() -> ! {
    todo!("0x45bca4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
// 0x45bcfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_0x45bcfc() -> ! {
    todo!("0x45bcfc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,RBX::DataModel::Genre const&)")]
// 0x45bd64 — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
pub fn stub_0x45bd64() -> ! {
    todo!("0x45bd64 __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_allocate(unsigned long)")]
// 0x45be48 — __ZNSt12_Vector_baseIN3RBX9DataModel5GenreESaIS2_EE11_M_allocateEm
// type: int(void)
pub fn stub_0x45be48() -> ! {
    todo!("0x45be48 __ZNSt12_Vector_baseIN3RBX9DataModel5GenreESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DataModel::Genre * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::Genre *,RBX::DataModel::Genre *>(RBX::DataModel::Genre *,RBX::DataModel::Genre *,RBX::DataModel::Genre *)")]
// 0x45be60 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel5GenreES6_EET0_T_S8_S7_
// type: int(void)
pub fn stub_0x45be60() -> ! {
    todo!("0x45be60 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel5GenreES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,unsigned long,RBX::DataModel::Genre const&)")]
// 0x45be9c — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
pub fn stub_0x45be9c() -> ! {
    todo!("0x45be9c __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::DataModel::CreatorType * rbx::any_cast<RBX::DataModel::CreatorType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x45c02c — __ZN3rbx8any_castIN3RBX9DataModel11CreatorTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
pub fn stub_0x45c02c() -> ! {
    todo!("0x45c02c __ZN3rbx8any_castIN3RBX9DataModel11CreatorTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}
