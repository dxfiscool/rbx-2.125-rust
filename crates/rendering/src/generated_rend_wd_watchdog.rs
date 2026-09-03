//! rendering shard rend_wd_watchdog — 120 stubs 0x7d2e5c..0x7d75c8 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render 15112 total filtered, 0 uncovered -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc Ogre/G3D/Render-filtered then global gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7d2e5c — __ZThn4_N3RBX5HUMAN7JumpingD0Ev
// type: void __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Jumping::~Jumping()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN7JumpingD0Ev")]
// IDA 0x7d2e5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d2e5c() {
}


// 0x7d2f00 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sJumpingEEEEvv")]
// IDA 0x7d2f00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d2f00() {
}


// 0x7d2f04 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sJumpingEEEERKS0_v")]
// IDA 0x7d2f04: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2f04() {
}


// 0x7d2fe4 — __GLOBAL__I_a_378
#[doc(alias = "global constructor keyed to_a_378")]
#[doc(alias = "__GLOBAL__I_a_378")]
// IDA 0x7d2fe4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d2fe4() {
}


// 0x7d32bc — __ZN3RBX5HUMAN19MovingNoPhysicsBaseC2EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, RBX::HUMAN::HumanoidState *, rbx::signals::connection *, int, int, int, int)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::MovingNoPhysicsBase(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBaseC2EPNS_8HumanoidENS0_9StateTypeE")]
// was: RBX::HUMAN::MovingNoPhysicsBase::MovingNoPhysicsBase(RBX::Humanoid *,RBX::HUMAN::StateType)
// IDA 0x7d32bc: 236 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d32bc() {
}


// 0x7d3544 — __ZN3RBX5HUMAN19MovingNoPhysicsBase28onEvent_TorsoAncestryChangedEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::onEvent_TorsoAncestryChanged(void)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBase28onEvent_TorsoAncestryChangedEv")]
// IDA 0x7d3544: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3544() {
}


// 0x7d35ac — __ZN3RBX5HUMAN19MovingNoPhysicsBaseD0Ev
// type: void __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBaseD0Ev")]
// IDA 0x7d35ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d35ac() {
}


// 0x7d364c — __ZN3RBX5HUMAN19MovingNoPhysicsBaseD1Ev
// type: void __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBaseD1Ev")]
// IDA 0x7d364c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d364c() {
}


// 0x7d3650 — __ZThn4_N3RBX5HUMAN19MovingNoPhysicsBaseD0Ev
// type: void __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN19MovingNoPhysicsBaseD0Ev")]
// IDA 0x7d3650: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d3650() {
}


// 0x7d3658 — __ZN3RBX5HUMAN19MovingNoPhysicsBaseD2Ev
// type: void __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBaseD2Ev")]
// IDA 0x7d3658: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d3658() {
}


// 0x7d386c — __ZThn4_N3RBX5HUMAN19MovingNoPhysicsBaseD1Ev
// type: void __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN19MovingNoPhysicsBaseD1Ev")]
// IDA 0x7d386c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d386c() {
}


// 0x7d3874 — __ZN3RBX5HUMAN19MovingNoPhysicsBase15disconnectTorsoEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::disconnectTorso(void)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBase15disconnectTorsoEv")]
// IDA 0x7d3874: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3874() {
}


// 0x7d390c — __ZN3RBX5HUMAN19MovingNoPhysicsBase18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBase18onComputeForceImplEv")]
// IDA 0x7d390c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7d390c() {
}


// 0x7d3910 — __ZN3RBX5HUMAN19MovingNoPhysicsBase19applyImpulseToFloorEf
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this, float)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::applyImpulseToFloor(float)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBase19applyImpulseToFloorEf")]
// IDA 0x7d3910: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3910() {
}


// 0x7d3a88 — __ZN3RBX5HUMAN19MovingNoPhysicsBase19onSimulatorStepImplEf
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this, float)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBase19onSimulatorStepImplEf")]
// IDA 0x7d3a88: 269 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3a88() {
}


// 0x7d3e38 — __ZN3RBX5HUMAN19MovingNoPhysicsBase10fireEventsEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::fireEvents(void)")]
#[doc(alias = "__ZN3RBX5HUMAN19MovingNoPhysicsBase10fireEventsEv")]
// IDA 0x7d3e38: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3e38() {
}


// 0x7d3e88 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_5HUMAN19MovingNoPhysicsBaseEEENSA_5list1INSA_5valueIPSF_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_5HUMAN19MovingNoPhysicsBaseEEENSA_5list1INSA_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>> const&)
// IDA 0x7d3e88: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3e88() {
}


// 0x7d3efc — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_20sMovingNoPhysicsBaseEEE7getNameEv")]
// IDA 0x7d3efc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3efc() {
}


// 0x7d3f24 — __ZNK3RBX5HUMAN19MovingNoPhysicsBase12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN19MovingNoPhysicsBase12getStateTypeEv")]
// IDA 0x7d3f24: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3f24() {
}


// 0x7d3f28 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEEvv")]
// IDA 0x7d3f28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d3f28() {
}


// 0x7d3f2c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN20sMovingNoPhysicsBaseEEEERKS0_v")]
// IDA 0x7d3f2c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d3f2c() {
}


// 0x7d400c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_5HUMAN19MovingNoPhysicsBaseEEENSA_5list1INSA_5valueIPSF_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_5HUMAN19MovingNoPhysicsBaseEEENSA_5list1INSA_5valueIPSF_EEEEEEED1Ev")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>>::~callable_slot()
// IDA 0x7d400c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d400c() {
}


// 0x7d4038 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_5HUMAN19MovingNoPhysicsBaseEEENSA_5list1INSA_5valueIPSF_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf0IvNS4_5HUMAN19MovingNoPhysicsBaseEEENSA_5list1INSA_5valueIPSF_EEEEEEED0Ev")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>>::~callable_slot()
// IDA 0x7d4038: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4038() {
}


// 0x7d410c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_E4callES7_S7_")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
// IDA 0x7d410c: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d410c() {
}


// 0x7d4124 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_E4callES7_S7_")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
// IDA 0x7d4124: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4124() {
}


// 0x7d413c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_ED1Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
// IDA 0x7d413c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d413c() {
}


// 0x7d4168 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf0IvNS5_5HUMAN19MovingNoPhysicsBaseEEENSB_5list1INSB_5valueIPSG_EEEEEELi2ES8_ED0Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HUMAN::MovingNoPhysicsBase>,boost::_bi::list1<boost::_bi::value<RBX::HUMAN::MovingNoPhysicsBase*>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
// IDA 0x7d4168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4168() {
}


// 0x7d423c — __GLOBAL__I_a_379
#[doc(alias = "global constructor keyed to_a_379")]
#[doc(alias = "__GLOBAL__I_a_379")]
// IDA 0x7d423c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d423c() {
}


// 0x7d4514 — __ZN3RBX5HUMAN7RunningC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Running::Running(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN7RunningC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d4514: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4514() {
}


// 0x7d4538 — __ZN3RBX5HUMAN7Running10fireEventsEv
// type: _DWORD __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "RBX::HUMAN::Running::fireEvents(void)")]
#[doc(alias = "__ZN3RBX5HUMAN7Running10fireEventsEv")]
// IDA 0x7d4538: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4538() {
}


// 0x7d4588 — __ZN3RBX5HUMAN7Running18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "RBX::HUMAN::Running::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN7Running18onComputeForceImplEv")]
// IDA 0x7d4588: 46 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4588() {
}


// 0x7d4628 — __ZN3RBX5HUMAN12RunningSlaveC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::RunningSlave::RunningSlave(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN12RunningSlaveC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d4628: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4628() {
}


// 0x7d4650 — __ZN3RBX5HUMAN6LandedC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Landed::Landed(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN6LandedC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d4650: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4650() {
}


// 0x7d4694 — __ZN3RBX5HUMAN8Climbing10fireEventsEv
// type: _DWORD __fastcall(RBX::HUMAN::Climbing *__hidden this)
#[doc(alias = "RBX::HUMAN::Climbing::fireEvents(void)")]
#[doc(alias = "__ZN3RBX5HUMAN8Climbing10fireEventsEv")]
// IDA 0x7d4694: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4694() {
}


// 0x7d46c0 — __ZN3RBX5HUMAN8ClimbingD1Ev
// type: void __fastcall(RBX::HUMAN::Climbing *__hidden this)
#[doc(alias = "RBX::HUMAN::Climbing::~Climbing()")]
#[doc(alias = "__ZN3RBX5HUMAN8ClimbingD1Ev")]
// IDA 0x7d46c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d46c0() {
}


// 0x7d46c4 — __ZN3RBX5HUMAN8ClimbingD0Ev
// type: void __fastcall(RBX::HUMAN::Climbing *__hidden this)
#[doc(alias = "RBX::HUMAN::Climbing::~Climbing()")]
#[doc(alias = "__ZN3RBX5HUMAN8ClimbingD0Ev")]
// IDA 0x7d46c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d46c4() {
}


// 0x7d4764 — __ZThn4_N3RBX5HUMAN8ClimbingD1Ev
// type: void __fastcall(RBX::HUMAN::Climbing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Climbing::~Climbing()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN8ClimbingD1Ev")]
// IDA 0x7d4764: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4764() {
}


// 0x7d476c — __ZThn4_N3RBX5HUMAN8ClimbingD0Ev
// type: void __fastcall(RBX::HUMAN::Climbing *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Climbing::~Climbing()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN8ClimbingD0Ev")]
// IDA 0x7d476c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d476c() {
}


// 0x7d4810 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_8sRunningEEE7getNameEv")]
// IDA 0x7d4810: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4810() {
}


// 0x7d4838 — __ZN3RBX5HUMAN7RunningD1Ev
// type: void __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "RBX::HUMAN::Running::~Running()")]
#[doc(alias = "__ZN3RBX5HUMAN7RunningD1Ev")]
// IDA 0x7d4838: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d4838() {
}


// 0x7d483c — __ZN3RBX5HUMAN7RunningD0Ev
// type: void __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "RBX::HUMAN::Running::~Running()")]
#[doc(alias = "__ZN3RBX5HUMAN7RunningD0Ev")]
// IDA 0x7d483c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d483c() {
}


// 0x7d48dc — __ZNK3RBX5HUMAN7Running12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "RBX::HUMAN::Running::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN7Running12getStateTypeEv")]
// IDA 0x7d48dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d48dc() {
}


// 0x7d48e0 — __ZThn4_N3RBX5HUMAN7RunningD1Ev
// type: void __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Running::~Running()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN7RunningD1Ev")]
// IDA 0x7d48e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d48e0() {
}


// 0x7d48e8 — __ZThn4_N3RBX5HUMAN7RunningD0Ev
// type: void __fastcall(RBX::HUMAN::Running *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Running::~Running()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN7RunningD0Ev")]
// IDA 0x7d48e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d48e8() {
}


// 0x7d498c — __ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN7RunningELZNS1_13sRunningSlaveEEE7getNameEv")]
// IDA 0x7d498c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d498c() {
}


// 0x7d49b4 — __ZN3RBX5HUMAN12RunningSlaveD1Ev
// type: void __fastcall(RBX::HUMAN::RunningSlave *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningSlave::~RunningSlave()")]
#[doc(alias = "__ZN3RBX5HUMAN12RunningSlaveD1Ev")]
// IDA 0x7d49b4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d49b4() {
}


// 0x7d49b8 — __ZN3RBX5HUMAN12RunningSlaveD0Ev
// type: void __fastcall(RBX::HUMAN::RunningSlave *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningSlave::~RunningSlave()")]
#[doc(alias = "__ZN3RBX5HUMAN12RunningSlaveD0Ev")]
// IDA 0x7d49b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d49b8() {
}


// 0x7d4a58 — __ZThn4_N3RBX5HUMAN12RunningSlaveD1Ev
// type: void __fastcall(RBX::HUMAN::RunningSlave *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::RunningSlave::~RunningSlave()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN12RunningSlaveD1Ev")]
// IDA 0x7d4a58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4a58() {
}


// 0x7d4a60 — __ZThn4_N3RBX5HUMAN12RunningSlaveD0Ev
// type: void __fastcall(RBX::HUMAN::RunningSlave *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::RunningSlave::~RunningSlave()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN12RunningSlaveD0Ev")]
// IDA 0x7d4a60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4a60() {
}


// 0x7d4b04 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_7sLandedEEE7getNameEv")]
// IDA 0x7d4b04: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4b04() {
}


// 0x7d4b2c — __ZN3RBX5HUMAN6LandedD1Ev
// type: void __fastcall(RBX::HUMAN::Landed *__hidden this)
#[doc(alias = "RBX::HUMAN::Landed::~Landed()")]
#[doc(alias = "__ZN3RBX5HUMAN6LandedD1Ev")]
// IDA 0x7d4b2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d4b2c() {
}


// 0x7d4b30 — __ZN3RBX5HUMAN6LandedD0Ev
// type: void __fastcall(RBX::HUMAN::Landed *__hidden this)
#[doc(alias = "RBX::HUMAN::Landed::~Landed()")]
#[doc(alias = "__ZN3RBX5HUMAN6LandedD0Ev")]
// IDA 0x7d4b30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4b30() {
}


// 0x7d4bd0 — __ZNK3RBX5HUMAN6Landed12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Landed *__hidden this)
#[doc(alias = "RBX::HUMAN::Landed::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN6Landed12getStateTypeEv")]
// IDA 0x7d4bd0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4bd0() {
}


// 0x7d4bd4 — __ZThn4_N3RBX5HUMAN6LandedD1Ev
// type: void __fastcall(RBX::HUMAN::Landed *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Landed::~Landed()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN6LandedD1Ev")]
// IDA 0x7d4bd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4bd4() {
}


// 0x7d4bdc — __ZThn4_N3RBX5HUMAN6LandedD0Ev
// type: void __fastcall(RBX::HUMAN::Landed *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Landed::~Landed()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN6LandedD0Ev")]
// IDA 0x7d4bdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d4bdc() {
}


// 0x7d4c80 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sLandedEEEEvv")]
// IDA 0x7d4c80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d4c80() {
}


// 0x7d4c84 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sLandedEEEERKS0_v")]
// IDA 0x7d4c84: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4c84() {
}


// 0x7d4d64 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN13sRunningSlaveEEEEvv")]
// IDA 0x7d4d64: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d4d64() {
}


// 0x7d4d68 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN13sRunningSlaveEEEERKS0_v")]
// IDA 0x7d4d68: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4d68() {
}


// 0x7d4e48 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN8sRunningEEEEvv")]
// IDA 0x7d4e48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d4e48() {
}


// 0x7d4e4c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN8sRunningEEEERKS0_v")]
// IDA 0x7d4e4c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d4e4c() {
}


// 0x7d4f2c — __GLOBAL__I_a_380
#[doc(alias = "global constructor keyed to_a_380")]
#[doc(alias = "__GLOBAL__I_a_380")]
// IDA 0x7d4f2c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d4f2c() {
}


// 0x7d5168 — __ZN3RBX5HUMAN11RunningBaseC2EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::HUMAN::RunningBase::RunningBase(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBaseC2EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d5168: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d5168() {
}


// 0x7d5320 — __ZN3RBX5HUMAN11RunningBaseC2EPNS_8HumanoidENS0_9StateTypeEff
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, RBX::HUMAN::HumanoidState *, int, int, int, int)
#[doc(alias = "RBX::HUMAN::RunningBase::RunningBase(RBX::Humanoid *,RBX::HUMAN::StateType,float,float)")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBaseC2EPNS_8HumanoidENS0_9StateTypeEff")]
// IDA 0x7d5320: 152 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d5320() {
}


// 0x7d54e4 — __ZN3RBX5HUMAN11RunningBase18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBase18onComputeForceImplEv")]
// IDA 0x7d54e4: 542 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d54e4() {
}


// 0x7d5c58 — __ZN3RBX5HUMAN11RunningBase19onSimulatorStepImplEf
// type: _DWORD __fastcall(RBX::HUMAN::RunningBase *__hidden this, float)
#[doc(alias = "RBX::HUMAN::RunningBase::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBase19onSimulatorStepImplEf")]
// IDA 0x7d5c58: 229 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d5c58() {
}


// 0x7d5fa8 — __ZN3RBX5HUMAN11RunningBase29onCFrameChangedFromReflectionEv
// type: _DWORD __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::onCFrameChangedFromReflection(void)")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBase29onCFrameChangedFromReflectionEv")]
// IDA 0x7d5fa8: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d5fa8() {
}


// 0x7d60fc — __ZN3RBX5HUMAN13HumanoidState12maxMoveForceEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::maxMoveForce(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState12maxMoveForceEv")]
// IDA 0x7d60fc: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d60fc() {
}


// 0x7d6160 — __ZNK3RBX5HUMAN13HumanoidState20getFloorTouchInWorldEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorTouchInWorld(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState20getFloorTouchInWorldEv")]
// IDA 0x7d6160: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6160() {
}


// 0x7d6200 — __ZN3RBX5HUMAN13HumanoidState12minMoveForceEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::minMoveForce(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState12minMoveForceEv")]
// IDA 0x7d6200: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6200() {
}


// 0x7d625c — __ZN3RBX5HUMAN11RunningBaseD1Ev
// type: void __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::~RunningBase()")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBaseD1Ev")]
// IDA 0x7d625c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d625c() {
}


// 0x7d6260 — __ZN3RBX5HUMAN11RunningBaseD0Ev
// type: void __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::~RunningBase()")]
#[doc(alias = "__ZN3RBX5HUMAN11RunningBaseD0Ev")]
// IDA 0x7d6260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6260() {
}


// 0x7d6300 — __ZThn4_N3RBX5HUMAN11RunningBaseD1Ev
// type: void __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::RunningBase::~RunningBase()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN11RunningBaseD1Ev")]
// IDA 0x7d6300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6300() {
}


// 0x7d6308 — __ZThn4_N3RBX5HUMAN11RunningBaseD0Ev
// type: void __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::RunningBase::~RunningBase()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN11RunningBaseD0Ev")]
// IDA 0x7d6308: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6308() {
}


// 0x7d63ac — __GLOBAL__I_a_381
#[doc(alias = "global constructor keyed to_a_381")]
#[doc(alias = "__GLOBAL__I_a_381")]
// IDA 0x7d63ac: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d63ac() {
}


// 0x7d6684 — __ZN3RBX5HUMAN16RunningNoPhysicsC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::RunningNoPhysics::RunningNoPhysics(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN16RunningNoPhysicsC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d6684: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d6684() {
}


// 0x7d6688 — __ZN3RBX5HUMAN16RunningNoPhysicsC2EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::HUMAN::RunningNoPhysics::RunningNoPhysics(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN16RunningNoPhysicsC2EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d6688: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6688() {
}


// 0x7d6760 — __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_17sRunningNoPhysicsEEE7getNameEv")]
// IDA 0x7d6760: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6760() {
}


// 0x7d6788 — __ZNK3RBX5HUMAN19MovingNoPhysicsBase17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN19MovingNoPhysicsBase17armsShouldCollideEv")]
// IDA 0x7d6788: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6788() {
}


// 0x7d678c — __ZNK3RBX5HUMAN19MovingNoPhysicsBase17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::MovingNoPhysicsBase *__hidden this)
#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN19MovingNoPhysicsBase17legsShouldCollideEv")]
// IDA 0x7d678c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d678c() {
}


// 0x7d6790 — __ZN3RBX5HUMAN16RunningNoPhysicsD1Ev
// type: void __fastcall(RBX::HUMAN::RunningNoPhysics *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningNoPhysics::~RunningNoPhysics()")]
#[doc(alias = "__ZN3RBX5HUMAN16RunningNoPhysicsD1Ev")]
// IDA 0x7d6790: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d6790() {
}


// 0x7d6794 — __ZN3RBX5HUMAN16RunningNoPhysicsD0Ev
// type: void __fastcall(RBX::HUMAN::RunningNoPhysics *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningNoPhysics::~RunningNoPhysics()")]
#[doc(alias = "__ZN3RBX5HUMAN16RunningNoPhysicsD0Ev")]
// IDA 0x7d6794: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6794() {
}


// 0x7d6834 — __ZNK3RBX5HUMAN16RunningNoPhysics12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::RunningNoPhysics *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningNoPhysics::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN16RunningNoPhysics12getStateTypeEv")]
// IDA 0x7d6834: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6834() {
}


// 0x7d6838 — __ZThn4_N3RBX5HUMAN16RunningNoPhysicsD1Ev
// type: void __fastcall(RBX::HUMAN::RunningNoPhysics *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::RunningNoPhysics::~RunningNoPhysics()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN16RunningNoPhysicsD1Ev")]
// IDA 0x7d6838: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6838() {
}


// 0x7d6840 — __ZThn4_N3RBX5HUMAN16RunningNoPhysicsD0Ev
// type: void __fastcall(RBX::HUMAN::RunningNoPhysics *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::RunningNoPhysics::~RunningNoPhysics()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN16RunningNoPhysicsD0Ev")]
// IDA 0x7d6840: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6840() {
}


// 0x7d68e4 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEEvv")]
// IDA 0x7d68e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d68e4() {
}


// 0x7d68e8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sRunningNoPhysicsEEEERKS0_v")]
// IDA 0x7d68e8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d68e8() {
}


// 0x7d69c8 — __GLOBAL__I_a_382
#[doc(alias = "global constructor keyed to_a_382")]
#[doc(alias = "__GLOBAL__I_a_382")]
// IDA 0x7d69c8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d69c8() {
}


// 0x7d6c04 — __ZN3RBX5HUMAN6SeatedC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Seated::Seated(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN6SeatedC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d6c04: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d6c04() {
}


// 0x7d6c08 — __ZN3RBX5HUMAN6SeatedC2EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Seated::Seated(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN6SeatedC2EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d6c08: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6c08() {
}


// 0x7d6cdc — __ZN3RBX5HUMAN6SeatedD0Ev
// type: void __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::~Seated()")]
#[doc(alias = "__ZN3RBX5HUMAN6SeatedD0Ev")]
// IDA 0x7d6cdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6cdc() {
}


// 0x7d6d7c — __ZN3RBX5HUMAN6SeatedD1Ev
// type: void __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::~Seated()")]
#[doc(alias = "__ZN3RBX5HUMAN6SeatedD1Ev")]
// IDA 0x7d6d7c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d6d7c() {
}


// 0x7d6d80 — __ZThn4_N3RBX5HUMAN6SeatedD0Ev
// type: void __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Seated::~Seated()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN6SeatedD0Ev")]
// IDA 0x7d6d80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6d80() {
}


// 0x7d6d88 — __ZN3RBX5HUMAN6SeatedD2Ev
// type: void __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::~Seated()")]
#[doc(alias = "__ZN3RBX5HUMAN6SeatedD2Ev")]
// IDA 0x7d6d88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6d88() {
}


// 0x7d6e7c — __ZThn4_N3RBX5HUMAN6SeatedD1Ev
// type: void __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Seated::~Seated()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN6SeatedD1Ev")]
// IDA 0x7d6e7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6e7c() {
}


// 0x7d6e84 — __ZN3RBX5HUMAN16PlatformStandingC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::PlatformStanding::PlatformStanding(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN16PlatformStandingC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d6e84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d6e84() {
}


// 0x7d6e88 — __ZN3RBX5HUMAN16PlatformStandingC2EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::PlatformStanding::PlatformStanding(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN16PlatformStandingC2EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d6e88: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d6e88() {
}


// 0x7d6f5c — __ZN3RBX5HUMAN16PlatformStandingD0Ev
// type: void __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::~PlatformStanding()")]
#[doc(alias = "__ZN3RBX5HUMAN16PlatformStandingD0Ev")]
// IDA 0x7d6f5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d6f5c() {
}


// 0x7d6ffc — __ZN3RBX5HUMAN16PlatformStandingD1Ev
// type: void __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::~PlatformStanding()")]
#[doc(alias = "__ZN3RBX5HUMAN16PlatformStandingD1Ev")]
// IDA 0x7d6ffc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d6ffc() {
}


// 0x7d7000 — __ZThn4_N3RBX5HUMAN16PlatformStandingD0Ev
// type: void __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::PlatformStanding::~PlatformStanding()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN16PlatformStandingD0Ev")]
// IDA 0x7d7000: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d7000() {
}


// 0x7d7008 — __ZN3RBX5HUMAN16PlatformStandingD2Ev
// type: void __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::~PlatformStanding()")]
#[doc(alias = "__ZN3RBX5HUMAN16PlatformStandingD2Ev")]
// IDA 0x7d7008: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d7008() {
}


// 0x7d70fc — __ZThn4_N3RBX5HUMAN16PlatformStandingD1Ev
// type: void __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::PlatformStanding::~PlatformStanding()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN16PlatformStandingD1Ev")]
// IDA 0x7d70fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d70fc() {
}


// 0x7d7104 — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_7sSeatedEEE7getNameEv")]
// IDA 0x7d7104: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7104() {
}


// 0x7d712c — __ZN3RBX5HUMAN6Seated18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN6Seated18onComputeForceImplEv")]
// IDA 0x7d712c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7d712c() {
}


// 0x7d7130 — __ZNK3RBX5HUMAN6Seated17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN6Seated17armsShouldCollideEv")]
// IDA 0x7d7130: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7130() {
}


// 0x7d7134 — __ZNK3RBX5HUMAN6Seated17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN6Seated17legsShouldCollideEv")]
// IDA 0x7d7134: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7134() {
}


// 0x7d7138 — __ZNK3RBX5HUMAN6Seated12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Seated *__hidden this)
#[doc(alias = "RBX::HUMAN::Seated::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN6Seated12getStateTypeEv")]
// IDA 0x7d7138: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7138() {
}


// 0x7d713c — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_17sPlatformStandingEEE7getNameEv")]
// IDA 0x7d713c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d713c() {
}


// 0x7d7164 — __ZN3RBX5HUMAN16PlatformStanding18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN16PlatformStanding18onComputeForceImplEv")]
// IDA 0x7d7164: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7d7164() {
}


// 0x7d7168 — __ZNK3RBX5HUMAN16PlatformStanding17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN16PlatformStanding17armsShouldCollideEv")]
// IDA 0x7d7168: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7168() {
}


// 0x7d716c — __ZNK3RBX5HUMAN16PlatformStanding17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN16PlatformStanding17legsShouldCollideEv")]
// IDA 0x7d716c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d716c() {
}


// 0x7d7170 — __ZNK3RBX5HUMAN16PlatformStanding12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::PlatformStanding *__hidden this)
#[doc(alias = "RBX::HUMAN::PlatformStanding::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN16PlatformStanding12getStateTypeEv")]
// IDA 0x7d7170: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7170() {
}


// 0x7d7174 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN17sPlatformStandingEEEEvv")]
// IDA 0x7d7174: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d7174() {
}


// 0x7d7178 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN17sPlatformStandingEEEERKS0_v")]
// IDA 0x7d7178: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7178() {
}


// 0x7d7258 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sSeatedEEEEvv")]
// IDA 0x7d7258: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d7258() {
}


// 0x7d725c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sSeatedEEEERKS0_v")]
// IDA 0x7d725c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d725c() {
}


// 0x7d733c — __GLOBAL__I_a_383
#[doc(alias = "global constructor keyed to_a_383")]
#[doc(alias = "__GLOBAL__I_a_383")]
// IDA 0x7d733c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d733c() {
}


// 0x7d7578 — __ZN3RBX5HUMAN17StrafingNoPhysicsC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::StrafingNoPhysics::StrafingNoPhysics(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN17StrafingNoPhysicsC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7d7578: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d7578() {
}


// 0x7d75a0 — __ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN19MovingNoPhysicsBaseELZNS1_18sStrafingNoPhysicsEEE7getNameEv")]
// IDA 0x7d75a0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d75a0() {
}


// 0x7d75c8 — __ZN3RBX5HUMAN17StrafingNoPhysicsD1Ev
// type: void __fastcall(RBX::HUMAN::StrafingNoPhysics *__hidden this)
#[doc(alias = "RBX::HUMAN::StrafingNoPhysics::~StrafingNoPhysics()")]
#[doc(alias = "__ZN3RBX5HUMAN17StrafingNoPhysicsD1Ev")]
// IDA 0x7d75c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d75c8() {
}
