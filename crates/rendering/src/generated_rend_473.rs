//! rendering shard 473 — 100 stubs 0x7420e0..0x74775c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 50890->50990 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7420e0 — __ZN3RBX12Motor6DJoint9isAlignedEv
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::isAligned(void)")]
#[doc(alias = "__ZN3RBX12Motor6DJoint9isAlignedEv")]
pub fn stub_7420e0() -> ! {
    todo!("0x7420e0 RBX::Motor6DJoint::isAligned(void)")
}

// 0x7421d8 — __ZN3RBX9AllocatorINS_6D6LinkEEnwEm
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::D6Link>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_6D6LinkEEnwEm")]
pub fn stub_7421d8() -> ! {
    todo!("0x7421d8 RBX::Allocator<RBX::D6Link>::operator new(unsigned long)")
}

// 0x742248 — __ZN3RBX9AllocatorINS_6D6LinkEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::D6Link>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_6D6LinkEEdlEPv")]
pub fn stub_742248() -> ! {
    todo!("0x742248 RBX::Allocator<RBX::D6Link>::operator delete(void *)")
}

// 0x74231c — __ZNK3RBX12Motor6DJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12Motor6DJoint12getJointTypeEv")]
pub fn stub_74231c() -> ! {
    todo!("0x74231c RBX::Motor6DJoint::getJointType(void)const")
}

// 0x742320 — __ZNK3RBX12Motor6DJoint8isBrokenEv
// type: _DWORD __fastcall(RBX::Motor6DJoint *__hidden this)
#[doc(alias = "RBX::Motor6DJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX12Motor6DJoint8isBrokenEv")]
pub fn stub_742320() -> ! {
    todo!("0x742320 RBX::Motor6DJoint::isBroken(void)const")
}

// 0x742324 — __ZNK3RBX12Motor6DJoint9canStepUiEv
// type: int __fastcall(RBX::Motor6DJoint *this)
#[doc(alias = "RBX::Motor6DJoint::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX12Motor6DJoint9canStepUiEv")]
pub fn stub_742324() -> ! {
    todo!("0x742324 RBX::Motor6DJoint::canStepUi(void)const")
}

// 0x742328 — __ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: void *()
#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
pub fn stub_742328() -> ! {
    todo!("0x742328 boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")
}

// 0x742374 — __ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_742374() -> ! {
    todo!("0x742374 boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x7423ac — __ZN3RBX9AllocatorINS_6D6LinkEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Allocator<RBX::D6Link>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_6D6LinkEEC2Ev")]
pub fn stub_7423ac() -> ! {
    todo!("0x7423ac RBX::Allocator<RBX::D6Link>::Allocator(void)")
}

// 0x742410 — __ZN3RBX9AllocatorINS_6D6LinkEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::D6Link>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_6D6LinkEE13releaseMemoryEv")]
pub fn stub_742410() -> ! {
    todo!("0x742410 RBX::Allocator<RBX::D6Link>::releaseMemory(void)")
}

// 0x74242c — __ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_74242c() -> ! {
    todo!("0x74242c boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x74245c — __GLOBAL__I_a_332
// was: global constructor keyed to_a_332
#[doc(alias = "global constructor keyed to_a_332")]
#[doc(alias = "__GLOBAL__I_a_332")]
pub fn stub_74245c() -> ! {
    todo!("0x74245c global constructor keyed to_a_332")
}

// 0x74258c — __ZN3RBX10MotorJointC1Ev
// type: int __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::MotorJoint(void)")]
#[doc(alias = "__ZN3RBX10MotorJointC1Ev")]
pub fn stub_74258c() -> ! {
    todo!("0x74258c RBX::MotorJoint::MotorJoint(void)")
}

// 0x742590 — __ZN3RBX10MotorJointC2Ev
// type: RBX::MotorJoint *__fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::MotorJoint(void)")]
#[doc(alias = "__ZN3RBX10MotorJointC2Ev")]
pub fn stub_742590() -> ! {
    todo!("0x742590 RBX::MotorJoint::MotorJoint(void)")
}

// 0x7426ac — __ZN3RBX10MotorJointD0Ev
// type: void __fastcall(RBX::MotorJoint *__hidden this)
#[doc(alias = "RBX::MotorJoint::~MotorJoint()")]
#[doc(alias = "__ZN3RBX10MotorJointD0Ev")]
pub fn stub_7426ac() -> ! {
    todo!("0x7426ac RBX::MotorJoint::~MotorJoint()")
}

// 0x74274c — __ZN3RBX10MotorJointD1Ev
// type: void __fastcall(RBX::MotorJoint *__hidden this)
#[doc(alias = "RBX::MotorJoint::~MotorJoint()")]
#[doc(alias = "__ZN3RBX10MotorJointD1Ev")]
pub fn stub_74274c() -> ! {
    todo!("0x74274c RBX::MotorJoint::~MotorJoint()")
}

// 0x742750 — __ZThn32_N3RBX10MotorJointD0Ev
// type: void __fastcall(RBX::MotorJoint *__hidden this)
// was: non-virtual thunk toRBX::MotorJoint::~MotorJoint()
#[doc(alias = "non-virtual thunk toRBX::MotorJoint::~MotorJoint()")]
#[doc(alias = "__ZThn32_N3RBX10MotorJointD0Ev")]
pub fn stub_742750() -> ! {
    todo!("0x742750 non-virtual thunk toRBX::MotorJoint::~MotorJoint()")
}

// 0x742758 — __ZN3RBX10MotorJointD2Ev
// type: void __fastcall(RBX::MotorJoint *__hidden this)
#[doc(alias = "RBX::MotorJoint::~MotorJoint()")]
#[doc(alias = "__ZN3RBX10MotorJointD2Ev")]
pub fn stub_742758() -> ! {
    todo!("0x742758 RBX::MotorJoint::~MotorJoint()")
}

// 0x742854 — __ZThn32_N3RBX10MotorJointD1Ev
// type: void __fastcall(RBX::MotorJoint *__hidden this)
// was: non-virtual thunk toRBX::MotorJoint::~MotorJoint()
#[doc(alias = "non-virtual thunk toRBX::MotorJoint::~MotorJoint()")]
#[doc(alias = "__ZThn32_N3RBX10MotorJointD1Ev")]
pub fn stub_742854() -> ! {
    todo!("0x742854 non-virtual thunk toRBX::MotorJoint::~MotorJoint()")
}

// 0x74285c — __ZNK3RBX10MotorJoint11getParentIdEv
// type: bool __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::getParentId(void)const")]
#[doc(alias = "__ZNK3RBX10MotorJoint11getParentIdEv")]
pub fn stub_74285c() -> ! {
    todo!("0x74285c RBX::MotorJoint::getParentId(void)const")
}

// 0x7428d0 — __ZN3RBX10MotorJoint13setJointAngleEf
// type: int __fastcall(RBX::MotorJoint *this, float, int)
#[doc(alias = "RBX::MotorJoint::setJointAngle(float)")]
#[doc(alias = "__ZN3RBX10MotorJoint13setJointAngleEf")]
pub fn stub_7428d0() -> ! {
    todo!("0x7428d0 RBX::MotorJoint::setJointAngle(float)")
}

// 0x742940 — __ZN3RBX10MotorJoint9resetLinkEv
// type: int __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::resetLink(void)")]
#[doc(alias = "__ZN3RBX10MotorJoint9resetLinkEv")]
pub fn stub_742940() -> ! {
    todo!("0x742940 RBX::MotorJoint::resetLink(void)")
}

// 0x74297c — __ZN3RBX10MotorJoint6stepUiEd
// type: int __fastcall(RBX::MotorJoint *this, double)
#[doc(alias = "RBX::MotorJoint::stepUi(double)")]
#[doc(alias = "__ZN3RBX10MotorJoint6stepUiEd")]
pub fn stub_74297c() -> ! {
    todo!("0x74297c RBX::MotorJoint::stepUi(double)")
}

// 0x742a4c — __ZN3RBX10MotorJoint15setCurrentAngleEf
// type: int __fastcall(RBX::MotorJoint *this, float)
#[doc(alias = "RBX::MotorJoint::setCurrentAngle(float)")]
#[doc(alias = "__ZN3RBX10MotorJoint15setCurrentAngleEf")]
pub fn stub_742a4c() -> ! {
    todo!("0x742a4c RBX::MotorJoint::setCurrentAngle(float)")
}

// 0x742aa4 — __ZN3RBX10MotorJoint9applyPoseEfff
// type: int __fastcall(int this, float32_t, float32_t, unsigned int)
#[doc(alias = "RBX::MotorJoint::applyPose(float,float,float)")]
#[doc(alias = "__ZN3RBX10MotorJoint9applyPoseEfff")]
pub fn stub_742aa4() -> ! {
    todo!("0x742aa4 RBX::MotorJoint::applyPose(float,float,float)")
}

// 0x742ac8 — __ZN3RBX10MotorJoint9isAlignedEv
// type: int __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::isAligned(void)")]
#[doc(alias = "__ZN3RBX10MotorJoint9isAlignedEv")]
pub fn stub_742ac8() -> ! {
    todo!("0x742ac8 RBX::MotorJoint::isAligned(void)")
}

// 0x742bc0 — __ZN3RBX9AllocatorINS_12RevoluteLinkEEnwEm
// type: int __fastcall(unsigned int)
#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_12RevoluteLinkEEnwEm")]
pub fn stub_742bc0() -> ! {
    todo!("0x742bc0 RBX::Allocator<RBX::RevoluteLink>::operator new(unsigned long)")
}

// 0x742c30 — __ZN3RBX9AllocatorINS_12RevoluteLinkEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_12RevoluteLinkEEdlEPv")]
pub fn stub_742c30() -> ! {
    todo!("0x742c30 RBX::Allocator<RBX::RevoluteLink>::operator delete(void *)")
}

// 0x742c6c — __ZNK3RBX10MotorJoint12getJointTypeEv
// type: int __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX10MotorJoint12getJointTypeEv")]
pub fn stub_742c6c() -> ! {
    todo!("0x742c6c RBX::MotorJoint::getJointType(void)const")
}

// 0x742c70 — __ZNK3RBX10MotorJoint8isBrokenEv
// type: int __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX10MotorJoint8isBrokenEv")]
pub fn stub_742c70() -> ! {
    todo!("0x742c70 RBX::MotorJoint::isBroken(void)const")
}

// 0x742c74 — __ZNK3RBX10MotorJoint9canStepUiEv
// type: int __fastcall(RBX::MotorJoint *this)
#[doc(alias = "RBX::MotorJoint::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX10MotorJoint9canStepUiEv")]
pub fn stub_742c74() -> ! {
    todo!("0x742c74 RBX::MotorJoint::canStepUi(void)const")
}

// 0x742c78 — __ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_742c78() -> ! {
    todo!("0x742c78 boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x742cb0 — __ZN3RBX12RevoluteLinkC2Ev
// type: RBX::RevoluteLink *__fastcall(RBX::RevoluteLink *this)
#[doc(alias = "RBX::RevoluteLink::RevoluteLink(void)")]
#[doc(alias = "__ZN3RBX12RevoluteLinkC2Ev")]
pub fn stub_742cb0() -> ! {
    todo!("0x742cb0 RBX::RevoluteLink::RevoluteLink(void)")
}

// 0x742d7c — __ZN3RBX9AllocatorINS_12RevoluteLinkEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_12RevoluteLinkEEC2Ev")]
pub fn stub_742d7c() -> ! {
    todo!("0x742d7c RBX::Allocator<RBX::RevoluteLink>::Allocator(void)")
}

// 0x742de0 — __ZN3RBX9AllocatorINS_12RevoluteLinkEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_12RevoluteLinkEE13releaseMemoryEv")]
pub fn stub_742de0() -> ! {
    todo!("0x742de0 RBX::Allocator<RBX::RevoluteLink>::releaseMemory(void)")
}

// 0x742dfc — __ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_742dfc() -> ! {
    todo!("0x742dfc boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x742e2c — __GLOBAL__I_a_333
// was: global constructor keyed to_a_333
#[doc(alias = "global constructor keyed to_a_333")]
#[doc(alias = "__GLOBAL__I_a_333")]
pub fn stub_742e2c() -> ! {
    todo!("0x742e2c global constructor keyed to_a_333")
}

// 0x742f28 — __ZN3RBX19MovingAssemblyStageC1EPNS_6IStageEPNS_5WorldE
// type: int __fastcall(RBX::MovingAssemblyStage *this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::MovingAssemblyStage::MovingAssemblyStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_742f28() -> ! {
    todo!("0x742f28 RBX::MovingAssemblyStage::MovingAssemblyStage(RBX::IStage *,RBX::World *)")
}

// 0x742f2c — __ZN3RBX19MovingAssemblyStageC2EPNS_6IStageEPNS_5WorldE
// type: RBX::MovingAssemblyStage *__fastcall(RBX::MovingAssemblyStage *this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::MovingAssemblyStage::MovingAssemblyStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_742f2c() -> ! {
    todo!("0x742f2c RBX::MovingAssemblyStage::MovingAssemblyStage(RBX::IStage *,RBX::World *)")
}

// 0x74301c — __ZN3RBX19MovingAssemblyStageD0Ev
// type: void __fastcall(RBX::MovingAssemblyStage *__hidden this)
#[doc(alias = "RBX::MovingAssemblyStage::~MovingAssemblyStage()")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStageD0Ev")]
pub fn stub_74301c() -> ! {
    todo!("0x74301c RBX::MovingAssemblyStage::~MovingAssemblyStage()")
}

// 0x7430bc — __ZN3RBX19MovingAssemblyStageD1Ev
// type: void __fastcall(RBX::MovingAssemblyStage *__hidden this)
#[doc(alias = "RBX::MovingAssemblyStage::~MovingAssemblyStage()")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStageD1Ev")]
pub fn stub_7430bc() -> ! {
    todo!("0x7430bc RBX::MovingAssemblyStage::~MovingAssemblyStage()")
}

// 0x7430c0 — __ZN3RBX19MovingAssemblyStageD2Ev
// type: void __fastcall(RBX::MovingAssemblyStage *this, int, int)
#[doc(alias = "RBX::MovingAssemblyStage::~MovingAssemblyStage()")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStageD2Ev")]
pub fn stub_7430c0() -> ! {
    todo!("0x7430c0 RBX::MovingAssemblyStage::~MovingAssemblyStage()")
}

// 0x743290 — __ZN3RBX19MovingAssemblyStage28removeMovingGroundedAssemblyEPNS_8AssemblyE
// type: int __fastcall(RBX::MovingAssemblyStage *this, RBX::Assembly *)
#[doc(alias = "RBX::MovingAssemblyStage::removeMovingGroundedAssembly(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage28removeMovingGroundedAssemblyEPNS_8AssemblyE")]
pub fn stub_743290() -> ! {
    todo!("0x743290 RBX::MovingAssemblyStage::removeMovingGroundedAssembly(RBX::Assembly *)")
}

// 0x7432a4 — __ZN3RBX19MovingAssemblyStage11removeJointEPNS_5JointE
// type: int __fastcall(RBX::MovingAssemblyStage *this, RBX::Joint *)
#[doc(alias = "RBX::MovingAssemblyStage::removeJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage11removeJointEPNS_5JointE")]
pub fn stub_7432a4() -> ! {
    todo!("0x7432a4 RBX::MovingAssemblyStage::removeJoint(RBX::Joint *)")
}

// 0x743344 — __ZN3RBX19MovingAssemblyStage23onSimulateAssemblyAddedEPNS_8AssemblyE
// type: int __fastcall(RBX::StepJointsStage **this, RBX::Assembly *)
#[doc(alias = "RBX::MovingAssemblyStage::onSimulateAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage23onSimulateAssemblyAddedEPNS_8AssemblyE")]
pub fn stub_743344() -> ! {
    todo!("0x743344 RBX::MovingAssemblyStage::onSimulateAssemblyAdded(RBX::Assembly *)")
}

// 0x743360 — __ZN3RBX19MovingAssemblyStage26onSimulateAssemblyRemovingEPNS_8AssemblyE
// type: int __fastcall(RBX::StepJointsStage **this, RBX::Assembly *)
#[doc(alias = "RBX::MovingAssemblyStage::onSimulateAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage26onSimulateAssemblyRemovingEPNS_8AssemblyE")]
pub fn stub_743360() -> ! {
    todo!("0x743360 RBX::MovingAssemblyStage::onSimulateAssemblyRemoving(RBX::Assembly *)")
}

// 0x743388 — __ZN3RBX19MovingAssemblyStage11onEdgeAddedEPNS_4EdgeE
// type: int __fastcall(RBX::MovingAssemblyStage *this, RBX::Edge *)
#[doc(alias = "RBX::MovingAssemblyStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_743388() -> ! {
    todo!("0x743388 RBX::MovingAssemblyStage::onEdgeAdded(RBX::Edge *)")
}

// 0x7433d4 — __ZN3RBX19MovingAssemblyStage14onEdgeRemovingEPNS_4EdgeE
// type: int __fastcall(RBX::MovingAssemblyStage *this, RBX::Edge *)
#[doc(alias = "RBX::MovingAssemblyStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_7433d4() -> ! {
    todo!("0x7433d4 RBX::MovingAssemblyStage::onEdgeRemoving(RBX::Edge *)")
}

// 0x743410 — __ZN3RBX19MovingAssemblyStage12jointsStepUiEd
// type: int __fastcall(int this, double)
#[doc(alias = "RBX::MovingAssemblyStage::jointsStepUi(double)")]
#[doc(alias = "__ZN3RBX19MovingAssemblyStage12jointsStepUiEd")]
pub fn stub_743410() -> ! {
    todo!("0x743410 RBX::MovingAssemblyStage::jointsStepUi(double)")
}

// 0x74350c — __ZNK3RBX19MovingAssemblyStage12getStageTypeEv
// type: int __fastcall(RBX::MovingAssemblyStage *this)
#[doc(alias = "RBX::MovingAssemblyStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX19MovingAssemblyStage12getStageTypeEv")]
pub fn stub_74350c() -> ! {
    todo!("0x74350c RBX::MovingAssemblyStage::getStageType(void)const")
}

// 0x743510 — __GLOBAL__I_a_334
// was: global constructor keyed to_a_334
#[doc(alias = "global constructor keyed to_a_334")]
#[doc(alias = "__GLOBAL__I_a_334")]
pub fn stub_743510() -> ! {
    todo!("0x743510 global constructor keyed to_a_334")
}

// 0x743640 — __ZN3RBX11MovingStageC1EPNS_6IStageEPNS_5WorldE
// type: int __fastcall(RBX::MovingStage *this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::MovingStage::MovingStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX11MovingStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_743640() -> ! {
    todo!("0x743640 RBX::MovingStage::MovingStage(RBX::IStage *,RBX::World *)")
}

// 0x743644 — __ZN3RBX11MovingStageC2EPNS_6IStageEPNS_5WorldE
// type: RBX::MovingStage *__fastcall(RBX::MovingStage *this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::MovingStage::MovingStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX11MovingStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_743644() -> ! {
    todo!("0x743644 RBX::MovingStage::MovingStage(RBX::IStage *,RBX::World *)")
}

// 0x743718 — __ZN3RBX11MovingStageD0Ev
// type: void __fastcall(RBX::MovingStage *__hidden this)
#[doc(alias = "RBX::MovingStage::~MovingStage()")]
#[doc(alias = "__ZN3RBX11MovingStageD0Ev")]
pub fn stub_743718() -> ! {
    todo!("0x743718 RBX::MovingStage::~MovingStage()")
}

// 0x7437d0 — __ZN3RBX11MovingStageD1Ev
// type: void __fastcall(RBX::MovingStage *__hidden this)
#[doc(alias = "RBX::MovingStage::~MovingStage()")]
#[doc(alias = "__ZN3RBX11MovingStageD1Ev")]
pub fn stub_7437d0() -> ! {
    todo!("0x7437d0 RBX::MovingStage::~MovingStage()")
}

// 0x7437f4 — __ZN3RBX20assertNotInPipeline2EPNS_8AssemblyE
// type: int __fastcall(RBX *this, RBX::Assembly *, int)
#[doc(alias = "RBX::assertNotInPipeline2(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX20assertNotInPipeline2EPNS_8AssemblyE")]
pub fn stub_7437f4() -> ! {
    todo!("0x7437f4 RBX::assertNotInPipeline2(RBX::Assembly *)")
}

// 0x743858 — __ZN3RBX11MovingStage16onMechanismAddedEPNS_9MechanismE
// type: int __fastcall(RBX::SpatialFilter **this, RBX::Mechanism *)
#[doc(alias = "RBX::MovingStage::onMechanismAdded(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX11MovingStage16onMechanismAddedEPNS_9MechanismE")]
pub fn stub_743858() -> ! {
    todo!("0x743858 RBX::MovingStage::onMechanismAdded(RBX::Mechanism *)")
}

// 0x743940 — __ZN3RBX11MovingStage19onMechanismRemovingEPNS_9MechanismE
// type: int __fastcall(RBX::SpatialFilter **this, RBX::Mechanism *)
#[doc(alias = "RBX::MovingStage::onMechanismRemoving(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX11MovingStage19onMechanismRemovingEPNS_9MechanismE")]
pub fn stub_743940() -> ! {
    todo!("0x743940 RBX::MovingStage::onMechanismRemoving(RBX::Mechanism *)")
}

// 0x743a34 — __ZNK3RBX11MovingStage12getStageTypeEv
// type: int __fastcall(RBX::MovingStage *this)
#[doc(alias = "RBX::MovingStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX11MovingStage12getStageTypeEv")]
pub fn stub_743a34() -> ! {
    todo!("0x743a34 RBX::MovingStage::getStageType(void)const")
}

// 0x743a38 — __GLOBAL__I_a_335
// was: global constructor keyed to_a_335
#[doc(alias = "global constructor keyed to_a_335")]
#[doc(alias = "__GLOBAL__I_a_335")]
pub fn stub_743a38() -> ! {
    todo!("0x743a38 global constructor keyed to_a_335")
}

// 0x743b00 — __ZN3RBX10MultiJointC2Ei
// type: RBX::MultiJoint *__fastcall(RBX::MultiJoint *this, int)
#[doc(alias = "RBX::MultiJoint::MultiJoint(int)")]
#[doc(alias = "__ZN3RBX10MultiJointC2Ei")]
pub fn stub_743b00() -> ! {
    todo!("0x743b00 RBX::MultiJoint::MultiJoint(int)")
}

// 0x743ba0 — __ZN3RBX10MultiJointD0Ev
// type: void __fastcall(RBX::MultiJoint *__hidden this)
#[doc(alias = "RBX::MultiJoint::~MultiJoint()")]
#[doc(alias = "__ZN3RBX10MultiJointD0Ev")]
pub fn stub_743ba0() -> ! {
    todo!("0x743ba0 RBX::MultiJoint::~MultiJoint()")
}

// 0x743c40 — __ZN3RBX10MultiJointD1Ev
// type: void __fastcall(RBX::MultiJoint *__hidden this)
#[doc(alias = "RBX::MultiJoint::~MultiJoint()")]
#[doc(alias = "__ZN3RBX10MultiJointD1Ev")]
pub fn stub_743c40() -> ! {
    todo!("0x743c40 RBX::MultiJoint::~MultiJoint()")
}

// 0x743c44 — __ZThn32_N3RBX10MultiJointD0Ev
// type: void __fastcall(RBX::MultiJoint *__hidden this)
// was: non-virtual thunk toRBX::MultiJoint::~MultiJoint()
#[doc(alias = "non-virtual thunk toRBX::MultiJoint::~MultiJoint()")]
#[doc(alias = "__ZThn32_N3RBX10MultiJointD0Ev")]
pub fn stub_743c44() -> ! {
    todo!("0x743c44 non-virtual thunk toRBX::MultiJoint::~MultiJoint()")
}

// 0x743c4c — __ZN3RBX10MultiJointD2Ev
// type: void __fastcall(RBX::MultiJoint *this, int, int)
#[doc(alias = "RBX::MultiJoint::~MultiJoint()")]
#[doc(alias = "__ZN3RBX10MultiJointD2Ev")]
pub fn stub_743c4c() -> ! {
    todo!("0x743c4c RBX::MultiJoint::~MultiJoint()")
}

// 0x743dc8 — __ZThn32_N3RBX10MultiJointD1Ev
// type: void __fastcall(RBX::MultiJoint *this, int, int)
// was: non-virtual thunk toRBX::MultiJoint::~MultiJoint()
#[doc(alias = "non-virtual thunk toRBX::MultiJoint::~MultiJoint()")]
#[doc(alias = "__ZThn32_N3RBX10MultiJointD1Ev")]
pub fn stub_743dc8() -> ! {
    todo!("0x743dc8 non-virtual thunk toRBX::MultiJoint::~MultiJoint()")
}

// 0x743dd0 — __ZN3RBX10MultiJoint11putInKernelEPNS_6KernelE
// type: int __fastcall(RBX::MultiJoint *this, RBX::Kernel *)
#[doc(alias = "RBX::MultiJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX10MultiJoint11putInKernelEPNS_6KernelE")]
pub fn stub_743dd0() -> ! {
    todo!("0x743dd0 RBX::MultiJoint::putInKernel(RBX::Kernel *)")
}

// 0x743dd4 — __ZN3RBX10MultiJoint9getJointKEv
// type: int __fastcall(RBX::MultiJoint *this)
#[doc(alias = "RBX::MultiJoint::getJointK(void)")]
#[doc(alias = "__ZN3RBX10MultiJoint9getJointKEv")]
pub fn stub_743dd4() -> ! {
    todo!("0x743dd4 RBX::MultiJoint::getJointK(void)")
}

// 0x743e60 — __ZN3RBX10MultiJoint15addToMultiJointEPNS_5PointES2_PNS_9ConnectorE
// type: int __fastcall(RBX::MultiJoint *this, RBX::Point *, RBX::Point *, RBX::Connector *)
#[doc(alias = "RBX::MultiJoint::addToMultiJoint(RBX::Point *,RBX::Point *,RBX::Connector *)")]
#[doc(alias = "__ZN3RBX10MultiJoint15addToMultiJointEPNS_5PointES2_PNS_9ConnectorE")]
pub fn stub_743e60() -> ! {
    todo!("0x743e60 RBX::MultiJoint::addToMultiJoint(RBX::Point *,RBX::Point *,RBX::Connector *)")
}

// 0x743f00 — __ZN3RBX10MultiJoint16removeFromKernelEv
// type: int __fastcall(RBX::MultiJoint *this, int, int)
#[doc(alias = "RBX::MultiJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX10MultiJoint16removeFromKernelEv")]
pub fn stub_743f00() -> ! {
    todo!("0x743f00 RBX::MultiJoint::removeFromKernel(void)")
}

// 0x744078 — __ZNK3RBX10MultiJoint8isBrokenEv
// type: int __fastcall(RBX::MultiJoint *this, int, int)
#[doc(alias = "RBX::MultiJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX10MultiJoint8isBrokenEv")]
pub fn stub_744078() -> ! {
    todo!("0x744078 RBX::MultiJoint::isBroken(void)const")
}

// 0x74419c — __GLOBAL__I_a_336
// was: global constructor keyed to_a_336
#[doc(alias = "global constructor keyed to_a_336")]
#[doc(alias = "__GLOBAL__I_a_336")]
pub fn stub_74419c() -> ! {
    todo!("0x74419c global constructor keyed to_a_336")
}

// 0x744368 — __ZN3RBX16ParallelRampPoly9buildMeshEv
// type: void __fastcall(RBX::ParallelRampPoly *this, int, int, int)
#[doc(alias = "RBX::ParallelRampPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX16ParallelRampPoly9buildMeshEv")]
pub fn stub_744368() -> ! {
    todo!("0x744368 RBX::ParallelRampPoly::buildMesh(void)")
}

// 0x744444 — __ZNK3RBX16ParallelRampPoly9getMomentEf
// type: int __fastcall(RBX::ParallelRampPoly *this, float, float)
#[doc(alias = "RBX::ParallelRampPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX16ParallelRampPoly9getMomentEf")]
pub fn stub_744444() -> ! {
    todo!("0x744444 RBX::ParallelRampPoly::getMoment(float)const")
}

// 0x744578 — __ZNK3RBX16ParallelRampPoly13getCofmOffsetEv
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::ParallelRampPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX16ParallelRampPoly13getCofmOffsetEv")]
pub fn stub_744578() -> ! {
    todo!("0x744578 RBX::ParallelRampPoly::getCofmOffset(void)const")
}

// 0x744748 — __ZN3RBX16ParallelRampPolyD1Ev
// type: void __fastcall(RBX::ParallelRampPoly *__hidden this)
#[doc(alias = "RBX::ParallelRampPoly::~ParallelRampPoly()")]
#[doc(alias = "__ZN3RBX16ParallelRampPolyD1Ev")]
pub fn stub_744748() -> ! {
    todo!("0x744748 RBX::ParallelRampPoly::~ParallelRampPoly()")
}

// 0x74476c — __ZN3RBX16ParallelRampPolyD0Ev
// type: void __fastcall(RBX::ParallelRampPoly *__hidden this)
#[doc(alias = "RBX::ParallelRampPoly::~ParallelRampPoly()")]
#[doc(alias = "__ZN3RBX16ParallelRampPolyD0Ev")]
pub fn stub_74476c() -> ! {
    todo!("0x74476c RBX::ParallelRampPoly::~ParallelRampPoly()")
}

// 0x744db0 — __ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEdlEPv")]
pub fn stub_744db0() -> ! {
    todo!("0x744db0 RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator delete(void *)")
}

// 0x7453ec — __ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEnwEm
// type: int __fastcall(unsigned int)
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEnwEm")]
pub fn stub_7453ec() -> ! {
    todo!("0x7453ec RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator new(unsigned long)")
}

// 0x745578 — __ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEC2Ev")]
pub fn stub_745578() -> ! {
    todo!("0x745578 RBX::Allocator<RBX::POLY::ParallelRampMesh>::Allocator(void)")
}

// 0x7455dc — __ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEE13releaseMemoryEv")]
pub fn stub_7455dc() -> ! {
    todo!("0x7455dc RBX::Allocator<RBX::POLY::ParallelRampMesh>::releaseMemory(void)")
}

// 0x7455f8 — __ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
pub fn stub_7455f8() -> ! {
    todo!("0x7455f8 boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")
}

// 0x745628 — __ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
pub fn stub_745628() -> ! {
    todo!("0x745628 boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")
}

// 0x7458c8 — __GLOBAL__I_a_337
// was: global constructor keyed to_a_337
#[doc(alias = "global constructor keyed to_a_337")]
#[doc(alias = "__GLOBAL__I_a_337")]
pub fn stub_7458c8() -> ! {
    todo!("0x7458c8 global constructor keyed to_a_337")
}

// 0x745a7c — __ZNK3RBX4Poly13getCofmOffsetEv
// type: int __fastcall(RBX::Poly *this)
#[doc(alias = "RBX::Poly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX4Poly13getCofmOffsetEv")]
pub fn stub_745a7c() -> ! {
    todo!("0x745a7c RBX::Poly::getCofmOffset(void)const")
}

// 0x745c78 — __ZNK3RBX4Poly19getPlaneFromSurfaceEm
// type: _QWORD *__fastcall(RBX::Poly *this, unsigned int, int)
#[doc(alias = "RBX::Poly::getPlaneFromSurface(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Poly19getPlaneFromSurfaceEm")]
pub fn stub_745c78() -> ! {
    todo!("0x745c78 RBX::Poly::getPlaneFromSurface(unsigned long)const")
}

// 0x745d3c — __ZNK3RBX4Poly22getSurfaceNormalInBodyEm
// type: int __fastcall(RBX::Poly *this, unsigned int, int)
#[doc(alias = "RBX::Poly::getSurfaceNormalInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Poly22getSurfaceNormalInBodyEm")]
pub fn stub_745d3c() -> ! {
    todo!("0x745d3c RBX::Poly::getSurfaceNormalInBody(unsigned long)const")
}

// 0x745dd4 — __ZNK3RBX4Poly20getSurfaceVertInBodyEmi
// type: int __fastcall(RBX::Poly *this, unsigned int, int, int)
#[doc(alias = "RBX::Poly::getSurfaceVertInBody(unsigned long,int)const")]
#[doc(alias = "__ZNK3RBX4Poly20getSurfaceVertInBodyEmi")]
pub fn stub_745dd4() -> ! {
    todo!("0x745dd4 RBX::Poly::getSurfaceVertInBody(unsigned long,int)const")
}

// 0x745fa0 — __ZNK3RBX4Poly20getNumVertsInSurfaceEm
// type: int __fastcall(RBX::Poly *this, unsigned int, int)
#[doc(alias = "RBX::Poly::getNumVertsInSurface(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Poly20getNumVertsInSurfaceEm")]
pub fn stub_745fa0() -> ! {
    todo!("0x745fa0 RBX::Poly::getNumVertsInSurface(unsigned long)const")
}

// 0x746098 — __ZNK3RBX4Poly21getSurfaceCoordInBodyEm
// type: char *__fastcall(RBX::Poly *this, unsigned int, int)
#[doc(alias = "RBX::Poly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4Poly21getSurfaceCoordInBodyEm")]
pub fn stub_746098() -> ! {
    todo!("0x746098 RBX::Poly::getSurfaceCoordInBody(unsigned long)const")
}

// 0x7471dc — __GLOBAL__I_a_338
// was: global constructor keyed to_a_338
#[doc(alias = "global constructor keyed to_a_338")]
#[doc(alias = "__GLOBAL__I_a_338")]
pub fn stub_7471dc() -> ! {
    todo!("0x7471dc global constructor keyed to_a_338")
}

// 0x7473a8 — __ZN3RBX11PolyContactD0Ev
// type: void __fastcall(RBX::PolyContact *__hidden this)
#[doc(alias = "RBX::PolyContact::~PolyContact()")]
#[doc(alias = "__ZN3RBX11PolyContactD0Ev")]
pub fn stub_7473a8() -> ! {
    todo!("0x7473a8 RBX::PolyContact::~PolyContact()")
}

// 0x747448 — __ZN3RBX11PolyContactD1Ev
// type: void __fastcall(RBX::PolyContact *__hidden this)
#[doc(alias = "RBX::PolyContact::~PolyContact()")]
#[doc(alias = "__ZN3RBX11PolyContactD1Ev")]
pub fn stub_747448() -> ! {
    todo!("0x747448 RBX::PolyContact::~PolyContact()")
}

// 0x74744c — __ZN3RBX11PolyContactD2Ev
// type: void __fastcall(RBX::PolyContact *__hidden this)
#[doc(alias = "RBX::PolyContact::~PolyContact()")]
#[doc(alias = "__ZN3RBX11PolyContactD2Ev")]
pub fn stub_74744c() -> ! {
    todo!("0x74744c RBX::PolyContact::~PolyContact()")
}

// 0x74757c — __ZN3RBX11PolyContact16deleteConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: int __fastcall(RBX::PolyContact *, int)
#[doc(alias = "RBX::PolyContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX11PolyContact16deleteConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
pub fn stub_74757c() -> ! {
    todo!("0x74757c RBX::PolyContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")
}

// 0x747640 — __ZN3RBX11PolyContact12getConnectorEi
// type: int __fastcall(RBX::PolyContact *this, int)
#[doc(alias = "RBX::PolyContact::getConnector(int)")]
#[doc(alias = "__ZN3RBX11PolyContact12getConnectorEi")]
pub fn stub_747640() -> ! {
    todo!("0x747640 RBX::PolyContact::getConnector(int)")
}

// 0x747648 — __ZN3RBX11PolyContact19deleteAllConnectorsEv
// type: int __fastcall(RBX::PolyContact *this)
#[doc(alias = "RBX::PolyContact::deleteAllConnectors(void)")]
#[doc(alias = "__ZN3RBX11PolyContact19deleteAllConnectorsEv")]
pub fn stub_747648() -> ! {
    todo!("0x747648 RBX::PolyContact::deleteAllConnectors(void)")
}

// 0x747650 — __ZN3RBX11PolyContact29removeAllConnectorsFromKernelEv
// type: unsigned int __fastcall(RBX::PolyContact *this)
#[doc(alias = "RBX::PolyContact::removeAllConnectorsFromKernel(void)")]
#[doc(alias = "__ZN3RBX11PolyContact29removeAllConnectorsFromKernelEv")]
pub fn stub_747650() -> ! {
    todo!("0x747650 RBX::PolyContact::removeAllConnectorsFromKernel(void)")
}

// 0x7476c0 — __ZN3RBX11PolyContact24putAllConnectorsInKernelEv
// type: unsigned int __fastcall(RBX::PolyContact *this)
#[doc(alias = "RBX::PolyContact::putAllConnectorsInKernel(void)")]
#[doc(alias = "__ZN3RBX11PolyContact24putAllConnectorsInKernelEv")]
pub fn stub_7476c0() -> ! {
    todo!("0x7476c0 RBX::PolyContact::putAllConnectorsInKernel(void)")
}

// 0x74775c — __ZN3RBX11PolyContact11stepContactEv
// type: int __fastcall(RBX::PolyContact *this)
#[doc(alias = "RBX::PolyContact::stepContact(void)")]
#[doc(alias = "__ZN3RBX11PolyContact11stepContactEv")]
pub fn stub_74775c() -> ! {
    todo!("0x74775c RBX::PolyContact::stepContact(void)")
}
