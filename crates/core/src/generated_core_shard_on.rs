//! core shard on — 100 core stubs EA-sorted, 0x7541a8..0x75a100 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 after global_eas.txt dedup).
//! Source: ida/export.json filtered where demangled/mangled contains RBX and demangled excludes Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 with EA not in /tmp/global_eas.txt.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes/backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
// 0x7541a8 — __ZN3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_0x7541a8() -> ! {
    todo!("0x7541a8 __ZN3RBX12RotatePJointD0Ev")
}

#[doc(alias = "RBX::RotatePJoint::getJointType(void)const")]
// 0x754248 — __ZNK3RBX12RotatePJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_0x754248() -> ! {
    todo!("0x754248 __ZNK3RBX12RotatePJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
// 0x75424c — __ZThn32_N3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_0x75424c() -> ! {
    todo!("0x75424c __ZThn32_N3RBX12RotatePJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
// 0x754254 — __ZThn32_N3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_0x754254() -> ! {
    todo!("0x754254 __ZThn32_N3RBX12RotatePJointD0Ev")
}

#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
// 0x7542f8 — __ZN3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_0x7542f8() -> ! {
    todo!("0x7542f8 __ZN3RBX12RotateVJointD1Ev")
}

#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
// 0x7542fc — __ZN3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_0x7542fc() -> ! {
    todo!("0x7542fc __ZN3RBX12RotateVJointD0Ev")
}

#[doc(alias = "RBX::RotateVJoint::getJointType(void)const")]
// 0x75439c — __ZNK3RBX12RotateVJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_0x75439c() -> ! {
    todo!("0x75439c __ZNK3RBX12RotateVJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
// 0x7543a0 — __ZThn32_N3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_0x7543a0() -> ! {
    todo!("0x7543a0 __ZThn32_N3RBX12RotateVJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
// 0x7543a8 — __ZThn32_N3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_0x7543a8() -> ! {
    todo!("0x7543a8 __ZThn32_N3RBX12RotateVJointD0Ev")
}

#[doc(alias = "RBX::JointConnector::getConnectorKernelType(void)const")]
// 0x75444c — __ZNK3RBX14JointConnector22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::JointConnector *__hidden this)
pub fn stub_0x75444c() -> ! {
    todo!("0x75444c __ZNK3RBX14JointConnector22getConnectorKernelTypeEv")
}

#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
// 0x754450 — __ZN3RBX26PointToPointBreakConnectorD1Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
pub fn stub_0x754450() -> ! {
    todo!("0x754450 __ZN3RBX26PointToPointBreakConnectorD1Ev")
}

#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
// 0x754454 — __ZN3RBX26PointToPointBreakConnectorD0Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
pub fn stub_0x754454() -> ! {
    todo!("0x754454 __ZN3RBX26PointToPointBreakConnectorD0Ev")
}

#[doc(alias = "RBX::PointToPointBreakConnector::getBroken(void)")]
// 0x754458 — __ZN3RBX26PointToPointBreakConnector9getBrokenEv
// type: _DWORD __fastcall(RBX::PointToPointBreakConnector *__hidden this)
pub fn stub_0x754458() -> ! {
    todo!("0x754458 __ZN3RBX26PointToPointBreakConnector9getBrokenEv")
}

#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
// 0x75462c — __ZN3RBX11SendPhysicsC1Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_0x75462c() -> ! {
    todo!("0x75462c __ZN3RBX11SendPhysicsC1Ev")
}

#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
// 0x754630 — __ZN3RBX11SendPhysicsC2Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_0x754630() -> ! {
    todo!("0x754630 __ZN3RBX11SendPhysicsC2Ev")
}

#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
// 0x754824 — __ZN3RBX11SendPhysicsD1Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_0x754824() -> ! {
    todo!("0x754824 __ZN3RBX11SendPhysicsD1Ev")
}

#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
// 0x754828 — __ZN3RBX11SendPhysicsD2Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_0x754828() -> ! {
    todo!("0x754828 __ZN3RBX11SendPhysicsD2Ev")
}

#[doc(alias = "RBX::SendPhysics::buildSimJob(RBX::SimJob *)")]
// 0x754abc — __ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
pub fn stub_0x754abc() -> ! {
    todo!("0x754abc __ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SendPhysics::destroySimJob(RBX::SimJob *)")]
// 0x754b34 — __ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
pub fn stub_0x754b34() -> ! {
    todo!("0x754b34 __ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootAdded(RBX::Assembly *)")]
// 0x754bd0 — __ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
pub fn stub_0x754bd0() -> ! {
    todo!("0x754bd0 __ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootRemoving(RBX::Assembly *)")]
// 0x754d1c — __ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
pub fn stub_0x754d1c() -> ! {
    todo!("0x754d1c __ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SendPhysics::nextSimJob(RBX::SimJob *)")]
// 0x754e00 — __ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
pub fn stub_0x754e00() -> ! {
    todo!("0x754e00 __ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SimJob::getConstSimJobFromPrimitive(RBX::Primitive const*)")]
// 0x754f3c — __ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, const RBX::Primitive *)
pub fn stub_0x754f3c() -> ! {
    todo!("0x754f3c __ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::SimJobTracker::stopTracking(void)")]
// 0x754f54 — __ZN3RBX13SimJobTracker12stopTrackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
pub fn stub_0x754f54() -> ! {
    todo!("0x754f54 __ZN3RBX13SimJobTracker12stopTrackingEv")
}

#[doc(alias = "RBX::SimJobTracker::tracking(void)")]
// 0x755034 — __ZN3RBX13SimJobTracker8trackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
pub fn stub_0x755034() -> ! {
    todo!("0x755034 __ZN3RBX13SimJobTracker8trackingEv")
}

#[doc(alias = "RBX::SimJobTracker::setSimJob(RBX::SimJob *)")]
// 0x7550bc — __ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *)
pub fn stub_0x7550bc() -> ! {
    todo!("0x7550bc __ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SimJobTracker::getSimJob(void)")]
// 0x7551a8 — __ZN3RBX13SimJobTracker9getSimJobEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
pub fn stub_0x7551a8() -> ! {
    todo!("0x7551a8 __ZN3RBX13SimJobTracker9getSimJobEv")
}

#[doc(alias = "RBX::SimJobTracker::transferTrackers(RBX::SimJob *,RBX::SimJob *)")]
// 0x755264 — __ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *, RBX::SimJob *)
pub fn stub_0x755264() -> ! {
    todo!("0x755264 __ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_")
}

#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
// 0x755310 — __ZN3RBX6SimJobC1EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
pub fn stub_0x755310() -> ! {
    todo!("0x755310 __ZN3RBX6SimJobC1EPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
// 0x755314 — __ZN3RBX6SimJobC2EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
pub fn stub_0x755314() -> ! {
    todo!("0x755314 __ZN3RBX6SimJobC2EPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimJob::~SimJob()")]
// 0x755424 — __ZN3RBX6SimJobD1Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
pub fn stub_0x755424() -> ! {
    todo!("0x755424 __ZN3RBX6SimJobD1Ev")
}

#[doc(alias = "RBX::SimJob::~SimJob()")]
// 0x755428 — __ZN3RBX6SimJobD2Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
pub fn stub_0x755428() -> ! {
    todo!("0x755428 __ZN3RBX6SimJobD2Ev")
}

#[doc(alias = "unsigned long RBX::fastRemoveShort<RBX::SimJobTracker *>(std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>> &,RBX::SimJobTracker * const&)")]
// 0x755580 — __ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_
pub fn stub_0x755580() -> ! {
    todo!("0x755580 __ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::push_back(RBX::SimJobTracker * const&)")]
// 0x7556dc — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_
pub fn stub_0x7556dc() -> ! {
    todo!("0x7556dc __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&)")]
// 0x755708 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0x755708() -> ! {
    todo!("0x755708 __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_allocate(unsigned long)")]
// 0x7557e8 — __ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm
pub fn stub_0x7557e8() -> ! {
    todo!("0x7557e8 __ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::resize(unsigned long,RBX::SimJobTracker *)")]
// 0x755800 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_
pub fn stub_0x755800() -> ! {
    todo!("0x755800 __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")]
// 0x755834 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0x755834() -> ! {
    todo!("0x755834 __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker *>(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&,std::random_access_iterator_tag)")]
// 0x75599c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_0x75599c() -> ! {
    todo!("0x75599c __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
// 0x755af4 — __ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x755af4() -> ! {
    todo!("0x755af4 __ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
// 0x755af8 — __ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x755af8() -> ! {
    todo!("0x755af8 __ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
// 0x755bf0 — __ZN3RBX13SimulateStageD0Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
pub fn stub_0x755bf0() -> ! {
    todo!("0x755bf0 __ZN3RBX13SimulateStageD0Ev")
}

#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
// 0x755c90 — __ZN3RBX13SimulateStageD1Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
pub fn stub_0x755c90() -> ! {
    todo!("0x755c90 __ZN3RBX13SimulateStageD1Ev")
}

#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
// 0x755c94 — __ZN3RBX13SimulateStageD2Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
pub fn stub_0x755c94() -> ! {
    todo!("0x755c94 __ZN3RBX13SimulateStageD2Ev")
}

#[doc(alias = "RBX::SimulateStage::onAssemblyAdded(RBX::Assembly *)")]
// 0x755f34 — __ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
pub fn stub_0x755f34() -> ! {
    todo!("0x755f34 __ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimulateStage::putFirstMovingRootInSendPhysics(RBX::Assembly *)")]
// 0x756070 — __ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
pub fn stub_0x756070() -> ! {
    todo!("0x756070 __ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimulateStage::onAssemblyRemoving(RBX::Assembly *)")]
// 0x756130 — __ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
pub fn stub_0x756130() -> ! {
    todo!("0x756130 __ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimulateStage::removeLastMovingRootFromSendPhysics(RBX::Assembly *)")]
// 0x7561ac — __ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
pub fn stub_0x7561ac() -> ! {
    todo!("0x7561ac __ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimulateStage::removeFromSendPhysics(RBX::Assembly *)")]
// 0x75627c — __ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
pub fn stub_0x75627c() -> ! {
    todo!("0x75627c __ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimulateStage::onEdgeAdded(RBX::Edge *)")]
// 0x7562f8 — __ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
pub fn stub_0x7562f8() -> ! {
    todo!("0x7562f8 __ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::SimulateStage::onEdgeRemoving(RBX::Edge *)")]
// 0x756320 — __ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
pub fn stub_0x756320() -> ! {
    todo!("0x756320 __ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "RBX::Assembly * RBX::IndexedTree::getOneBelowRoot<RBX::Assembly>(void)")]
// 0x75633c — __ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v
pub fn stub_0x75633c() -> ! {
    todo!("0x75633c __ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v")
}

#[doc(alias = "RBX::SimulateStage::getStageType(void)const")]
// 0x7563a8 — __ZNK3RBX13SimulateStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this)
pub fn stub_0x7563a8() -> ! {
    todo!("0x7563a8 __ZNK3RBX13SimulateStage12getStageTypeEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert_unique(std::pair<RBX::Assembly * const,int> const&)")]
// 0x7563ac — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x7563ac() -> ! {
    todo!("0x7563ac __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Assembly * const,int> const&)")]
// 0x756414 — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x756414() -> ! {
    todo!("0x756414 __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,int>> *)")]
// 0x75646c — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x75646c() -> ! {
    todo!("0x75646c __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
// 0x75655c — __ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x75655c() -> ! {
    todo!("0x75655c __ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
// 0x756560 — __ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
pub fn stub_0x756560() -> ! {
    todo!("0x756560 __ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::SleepStage::~SleepStage()")]
// 0x756a74 — __ZN3RBX10SleepStageD0Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
pub fn stub_0x756a74() -> ! {
    todo!("0x756a74 __ZN3RBX10SleepStageD0Ev")
}

#[doc(alias = "RBX::SleepStage::~SleepStage()")]
// 0x756b14 — __ZN3RBX10SleepStageD1Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
pub fn stub_0x756b14() -> ! {
    todo!("0x756b14 __ZN3RBX10SleepStageD1Ev")
}

#[doc(alias = "RBX::SleepStage::~SleepStage()")]
// 0x756b18 — __ZN3RBX10SleepStageD2Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
pub fn stub_0x756b18() -> ! {
    todo!("0x756b18 __ZN3RBX10SleepStageD2Ev")
}

#[doc(alias = "RBX::SleepStage::stepSleepStage(int,int,bool)")]
// 0x7573d8 — __ZN3RBX10SleepStage14stepSleepStageEiib
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, int, int, bool)
pub fn stub_0x7573d8() -> ! {
    todo!("0x7573d8 __ZN3RBX10SleepStage14stepSleepStageEiib")
}

#[doc(alias = "RBX::SleepStage::doContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> (&)[2])")]
// 0x7578ac — __ZN3RBX10SleepStage10doContactsERA2_NS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE
pub fn stub_0x7578ac() -> ! {
    todo!("0x7578ac __ZN3RBX10SleepStage10doContactsERA2_NS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE")
}

#[doc(alias = "RBX::SleepStage::stepAssembliesAwake(void)")]
// 0x75798c — __ZN3RBX10SleepStage19stepAssembliesAwakeEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
pub fn stub_0x75798c() -> ! {
    todo!("0x75798c __ZN3RBX10SleepStage19stepAssembliesAwakeEv")
}

#[doc(alias = "RBX::SleepStage::stepAssembliesSleepingChecking(void)")]
// 0x757c2c — __ZN3RBX10SleepStage30stepAssembliesSleepingCheckingEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
pub fn stub_0x757c2c() -> ! {
    todo!("0x757c2c __ZN3RBX10SleepStage30stepAssembliesSleepingCheckingEv")
}

#[doc(alias = "RBX::SleepStage::stepJoints(void)")]
// 0x757ea8 — __ZN3RBX10SleepStage10stepJointsEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
pub fn stub_0x757ea8() -> ! {
    todo!("0x757ea8 __ZN3RBX10SleepStage10stepJointsEv")
}

#[doc(alias = "RBX::SleepStage::stepContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> &)")]
// 0x7580dc — __ZN3RBX10SleepStage12stepContactsERNS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE
pub fn stub_0x7580dc() -> ! {
    todo!("0x7580dc __ZN3RBX10SleepStage12stepContactsERNS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE")
}

#[doc(alias = "RBX::SleepStage::wakeAssemblies(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> &,int,RBX::Sim::AssemblyState)")]
// 0x758624 — __ZN3RBX10SleepStage14wakeAssembliesERSt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEiNS_3Sim13AssemblyStateE
// type: int __fastcall(int, int, int, void *)
pub fn stub_0x758624() -> ! {
    todo!("0x758624 __ZN3RBX10SleepStage14wakeAssembliesERSt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEiNS_3Sim13AssemblyStateE")
}

#[doc(alias = "RBX::SleepStage::traverse(RBX::Assembly *,std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>> &,int)")]
// 0x758958 — __ZN3RBX10SleepStage8traverseEPNS_8AssemblyERSt5dequeIS2_SaIS2_EEi
// type: int __fastcall(int, RBX::Assembly *this)
pub fn stub_0x758958() -> ! {
    todo!("0x758958 __ZN3RBX10SleepStage8traverseEPNS_8AssemblyERSt5dequeIS2_SaIS2_EEi")
}

#[doc(alias = "RBX::canThrottle(RBX::Edge *)")]
// 0x758adc — __ZN3RBX11canThrottleEPNS_4EdgeE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Edge *)
pub fn stub_0x758adc() -> ! {
    todo!("0x758adc __ZN3RBX11canThrottleEPNS_4EdgeE")
}

#[doc(alias = "RBX::SleepStage::changeContactState(std::vector<RBX::Contact *,std::allocator<RBX::Contact *>> const&,RBX::Sim::EdgeState)")]
// 0x758b40 — __ZN3RBX10SleepStage18changeContactStateERKSt6vectorIPNS_7ContactESaIS3_EENS_3Sim9EdgeStateE
pub fn stub_0x758b40() -> ! {
    todo!("0x758b40 __ZN3RBX10SleepStage18changeContactStateERKSt6vectorIPNS_7ContactESaIS3_EENS_3Sim9EdgeStateE")
}

#[doc(alias = "RBX::SleepStage::changeJointState(std::vector<RBX::Joint *,std::allocator<RBX::Joint *>> const&,RBX::Sim::EdgeState)")]
// 0x758b78 — __ZN3RBX10SleepStage16changeJointStateERKSt6vectorIPNS_5JointESaIS3_EENS_3Sim9EdgeStateE
pub fn stub_0x758b78() -> ! {
    todo!("0x758b78 __ZN3RBX10SleepStage16changeJointStateERKSt6vectorIPNS_5JointESaIS3_EENS_3Sim9EdgeStateE")
}

#[doc(alias = "RBX::SleepStage::computeStateFromNeighbors(RBX::Assembly *)")]
// 0x758bb0 — __ZN3RBX10SleepStage25computeStateFromNeighborsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
pub fn stub_0x758bb0() -> ! {
    todo!("0x758bb0 __ZN3RBX10SleepStage25computeStateFromNeighborsEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SleepStage::changeAssemblyState(std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>> const&,RBX::Sim::AssemblyState)")]
// 0x758c4c — __ZN3RBX10SleepStage19changeAssemblyStateERKSt6vectorIPNS_8AssemblyESaIS3_EENS_3Sim13AssemblyStateE
pub fn stub_0x758c4c() -> ! {
    todo!("0x758c4c __ZN3RBX10SleepStage19changeAssemblyStateERKSt6vectorIPNS_8AssemblyESaIS3_EENS_3Sim13AssemblyStateE")
}

#[doc(alias = "RBX::SleepStage::changeAssemblyState(RBX::Assembly *,RBX::Sim::AssemblyState)")]
// 0x758c84 — __ZN3RBX10SleepStage19changeAssemblyStateEPNS_8AssemblyENS_3Sim13AssemblyStateE
// type: int __fastcall(RBX::IStage *, RBX::IPipelined *this)
pub fn stub_0x758c84() -> ! {
    todo!("0x758c84 __ZN3RBX10SleepStage19changeAssemblyStateEPNS_8AssemblyENS_3Sim13AssemblyStateE")
}

#[doc(alias = "RBX::SleepStage::wakeEdge(RBX::Edge *)")]
// 0x758f40 — __ZN3RBX10SleepStage8wakeEdgeEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
pub fn stub_0x758f40() -> ! {
    todo!("0x758f40 __ZN3RBX10SleepStage8wakeEdgeEPNS_4EdgeE")
}

#[doc(alias = "RBX::SleepStage::isAffecting(RBX::Edge *)")]
// 0x759080 — __ZN3RBX10SleepStage11isAffectingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
pub fn stub_0x759080() -> ! {
    todo!("0x759080 __ZN3RBX10SleepStage11isAffectingEPNS_4EdgeE")
}

#[doc(alias = "RBX::SleepStage::changeContactState(RBX::Contact *,RBX::Sim::EdgeState)")]
// 0x759144 — __ZN3RBX10SleepStage18changeContactStateEPNS_7ContactENS_3Sim9EdgeStateE
pub fn stub_0x759144() -> ! {
    todo!("0x759144 __ZN3RBX10SleepStage18changeContactStateEPNS_7ContactENS_3Sim9EdgeStateE")
}

#[doc(alias = "RBX::SleepStage::changeJointState(RBX::Joint *,RBX::Sim::EdgeState)")]
// 0x75942c — __ZN3RBX10SleepStage16changeJointStateEPNS_5JointENS_3Sim9EdgeStateE
// type: int __fastcall(int, int, int)
pub fn stub_0x75942c() -> ! {
    todo!("0x75942c __ZN3RBX10SleepStage16changeJointStateEPNS_5JointENS_3Sim9EdgeStateE")
}

#[doc(alias = "RBX::SleepStage::wakeEvent(RBX::Edge *)")]
// 0x759578 — __ZN3RBX10SleepStage9wakeEventEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
pub fn stub_0x759578() -> ! {
    todo!("0x759578 __ZN3RBX10SleepStage9wakeEventEPNS_4EdgeE")
}

#[doc(alias = "RBX::SleepStage::wakeEvent(RBX::Assembly *)")]
// 0x75959c — __ZN3RBX10SleepStage9wakeEventEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
pub fn stub_0x75959c() -> ! {
    todo!("0x75959c __ZN3RBX10SleepStage9wakeEventEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SleepStage::recursiveWakeEvent(RBX::Contact *)")]
// 0x7595c0 — __ZN3RBX10SleepStage18recursiveWakeEventEPNS_7ContactE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Contact *)
pub fn stub_0x7595c0() -> ! {
    todo!("0x7595c0 __ZN3RBX10SleepStage18recursiveWakeEventEPNS_7ContactE")
}

#[doc(alias = "RBX::SleepStage::recursiveWakeEvent(RBX::Assembly *)")]
// 0x7595e4 — __ZN3RBX10SleepStage18recursiveWakeEventEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
pub fn stub_0x7595e4() -> ! {
    todo!("0x7595e4 __ZN3RBX10SleepStage18recursiveWakeEventEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SleepStage::highVelocityNewTouch(RBX::Contact *)")]
// 0x759608 — __ZN3RBX10SleepStage20highVelocityNewTouchEPNS_7ContactE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Contact *)
pub fn stub_0x759608() -> ! {
    todo!("0x759608 __ZN3RBX10SleepStage20highVelocityNewTouchEPNS_7ContactE")
}

#[doc(alias = "RBX::SleepStage::stateToSet(RBX::Sim::AssemblyState)")]
// 0x7596f0 — __ZN3RBX10SleepStage10stateToSetENS_3Sim13AssemblyStateE
pub fn stub_0x7596f0() -> ! {
    todo!("0x7596f0 __ZN3RBX10SleepStage10stateToSetENS_3Sim13AssemblyStateE")
}

#[doc(alias = "RBX::SleepStage::onExternalTickleAssembly(RBX::Assembly *,bool)")]
// 0x759778 — __ZN3RBX10SleepStage24onExternalTickleAssemblyEPNS_8AssemblyEb
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *, bool)
pub fn stub_0x759778() -> ! {
    todo!("0x759778 __ZN3RBX10SleepStage24onExternalTickleAssemblyEPNS_8AssemblyEb")
}

#[doc(alias = "RBX::SleepStage::onAssemblyAdded(RBX::Assembly *)")]
// 0x7597c0 — __ZN3RBX10SleepStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
pub fn stub_0x7597c0() -> ! {
    todo!("0x7597c0 __ZN3RBX10SleepStage15onAssemblyAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SleepStage::onAssemblyRemoving(RBX::Assembly *)")]
// 0x7599fc — __ZN3RBX10SleepStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
pub fn stub_0x7599fc() -> ! {
    todo!("0x7599fc __ZN3RBX10SleepStage18onAssemblyRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SleepStage::onEdgeAdded(RBX::Edge *)")]
// 0x759b68 — __ZN3RBX10SleepStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
pub fn stub_0x759b68() -> ! {
    todo!("0x759b68 __ZN3RBX10SleepStage11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::SleepStage::onEdgeRemoving(RBX::Edge *)")]
// 0x759d40 — __ZN3RBX10SleepStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
pub fn stub_0x759d40() -> ! {
    todo!("0x759d40 __ZN3RBX10SleepStage14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "RBX::SleepStage::getMetric(RBX::IWorldStage::MetricType)")]
// 0x759ed8 — __ZN3RBX10SleepStage9getMetricENS_11IWorldStage10MetricTypeE
pub fn stub_0x759ed8() -> ! {
    todo!("0x759ed8 __ZN3RBX10SleepStage9getMetricENS_11IWorldStage10MetricTypeE")
}

#[doc(alias = "RBX::IPipelined::inOrDownstreamOfStage(RBX::IStage *)const")]
// 0x759efc — __ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this, RBX::IStage *)
pub fn stub_0x759efc() -> ! {
    todo!("0x759efc __ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE")
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
// 0x759fbc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
pub fn stub_0x759fbc() -> ! {
    todo!("0x759fbc __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::pop_front(void)")]
// 0x759fdc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv
pub fn stub_0x759fdc() -> ! {
    todo!("0x759fdc __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv")
}

#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::push_back(RBX::Contact * const&)")]
// 0x75a00c — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_
pub fn stub_0x75a00c() -> ! {
    todo!("0x75a00c __ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::resize(unsigned long,RBX::Contact *)")]
// 0x75a038 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_
pub fn stub_0x75a038() -> ! {
    todo!("0x75a038 __ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::resize(unsigned long,RBX::Joint *)")]
// 0x75a06c — __ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_
pub fn stub_0x75a06c() -> ! {
    todo!("0x75a06c __ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
// 0x75a0a0 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
pub fn stub_0x75a0a0() -> ! {
    todo!("0x75a0a0 __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::resize(unsigned long,RBX::Assembly *)")]
// 0x75a0cc — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_
pub fn stub_0x75a0cc() -> ! {
    todo!("0x75a0cc __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc>::fastRemove(RBX::Contact*)")]
// 0x75a100 — __ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_
pub fn stub_0x75a100() -> ! {
    todo!("0x75a100 __ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_")
}
