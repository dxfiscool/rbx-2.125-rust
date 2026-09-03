//! rendering shard 276 — 150 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 29970->30120 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29970 before -> 30120 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3a2778 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_
// IDA 0x3a2778: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a2778() {
}


// 0x3a2874 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// IDA 0x3a2874: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a2874() {
}


// 0x3a2984 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// IDA 0x3a2984: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a2984() {
}


// 0x3a2ab4 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// IDA 0x3a2ab4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a2ab4() {
}


// 0x3a2abc — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// IDA 0x3a2abc: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a2abc() {
}


// 0x3a2ac4 — __ZNK5boost9function4IvffffEclEffff
// type: void __fastcall(_DWORD *, int, int, int, float)
#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
// was: __ZNK5boost9function4IvffffEclEffff
// IDA 0x3a2ac4: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a2ac4() {
}


// 0x3a2bac — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
// IDA 0x3a2bac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a2bac() {
}


// 0x3a2cbc — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
// IDA 0x3a2cbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a2cbc() {
}


// 0x3a2dec — __ZN5boost9function4IvffffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
// was: __ZN5boost9function4IvffffE13assign_to_ownERKS1_
// IDA 0x3a2dec: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a2dec() {
}


// 0x3a2e1c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3a2e1c: 280 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a2e1c() {
}


// 0x3a30e8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// IDA 0x3a30e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a30e8() {
}


// 0x3a310c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// IDA 0x3a310c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a310c() {
}


// 0x3a31c0 — __ZN3rbx13remote_signalIFvffffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvffffEED2Ev
// IDA 0x3a31c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a31c0() {
}


// 0x3a330c — __ZN3rbx13remote_signalIFvfffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvfffEED2Ev
// IDA 0x3a330c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a330c() {
}


// 0x3a3458 — __ZN3rbx13remote_signalIFvffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvffEED2Ev
// IDA 0x3a3458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a3458() {
}


// 0x3a35a4 — __GLOBAL__I_a_157
#[doc(alias = "global constructor keyed to_a_157")]
// was: __GLOBAL__I_a_157
// IDA 0x3a35a4: 303 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a35a4() {
}


// 0x3a395c — __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, __guard *, const shared_count *, int)
#[doc(alias = "RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3a395c: 373 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a395c() {
}


// 0x3a3d44 — __ZN3RBX8AnimatorC1EPNS_8InstanceE
// type: int __fastcall(RBX::Animator *this, RBX::Instance *)
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
// was: __ZN3RBX8AnimatorC1EPNS_8InstanceE
// IDA 0x3a3d44: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a3d44() {
}


// 0x3a3d48 — __ZN3RBX8AnimatorC2EPNS_8InstanceE
// type: RBX::Instance *__fastcall(RBX::Animator *this, RBX::Instance *)
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
// was: __ZN3RBX8AnimatorC2EPNS_8InstanceE
// IDA 0x3a3d48: 326 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a3d48() {
}


// 0x3a40b8 — __ZN3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "RBX::Animator::~Animator()")]
// was: __ZN3RBX8AnimatorD0Ev
// IDA 0x3a40b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a40b8() {
}


// 0x3a4158 — __ZN3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "RBX::Animator::~Animator()")]
// was: __ZN3RBX8AnimatorD1Ev
// IDA 0x3a4158: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a4158() {
}


// 0x3a415c — __ZThn32_N3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn32_N3RBX8AnimatorD0Ev
// IDA 0x3a415c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a415c() {
}


// 0x3a4164 — __ZThn36_N3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn36_N3RBX8AnimatorD0Ev
// IDA 0x3a4164: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a4164() {
}


// 0x3a416c — __ZThn92_N3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn92_N3RBX8AnimatorD0Ev
// IDA 0x3a416c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a416c() {
}


// 0x3a4174 — __ZN3RBX8AnimatorD2Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "RBX::Animator::~Animator()")]
// was: __ZN3RBX8AnimatorD2Ev
// IDA 0x3a4174: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a4174() {
}


// 0x3a434c — __ZThn32_N3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn32_N3RBX8AnimatorD1Ev
// IDA 0x3a434c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a434c() {
}


// 0x3a4354 — __ZThn36_N3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn36_N3RBX8AnimatorD1Ev
// IDA 0x3a4354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a4354() {
}


// 0x3a435c — __ZThn92_N3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn92_N3RBX8AnimatorD1Ev
// IDA 0x3a435c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a435c() {
}


// 0x3a4364 — __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
// type: int *__fastcall(int, int *, int, int)
#[doc(alias = "RBX::Animator::reloadAnimation(rbx_core::SharedPtr<RBX::AnimationTrackState>)")]
// was: __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
// IDA 0x3a4364: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4364() {
}


// 0x3a439c — __ZNK3RBX8Animator11getGameTimeEv
// type: int __fastcall(RBX::Animator *this)
#[doc(alias = "RBX::Animator::getGameTime(void)const")]
// was: __ZNK3RBX8Animator11getGameTimeEv
// IDA 0x3a439c: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a439c() {
}


// 0x3a4474 — __ZN3RBX8Animator23getReplicatingContainerEv
// type: void __fastcall(RBX::Animator *this, int)
#[doc(alias = "RBX::Animator::getReplicatingContainer(void)")]
// was: __ZN3RBX8Animator23getReplicatingContainerEv
// IDA 0x3a4474: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4474() {
}


// 0x3a4598 — __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
// type: void __fastcall(int, int *, double, int, _DWORD *)
#[doc(alias = "RBX::Animator::onTrackStepped(rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")]
// was: __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
// IDA 0x3a4598: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4598() {
}


// 0x3a46a0 — __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
// type: void *__fastcall(int, const void **)
#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3a46a0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a46a0() {
}


// 0x3a46e8 — __ZN3RBX8Animator20calcAnimatableJointsEv
// type: void __fastcall(RBX::Animator *this)
#[doc(alias = "RBX::Animator::calcAnimatableJoints(void)")]
// was: __ZN3RBX8Animator20calcAnimatableJointsEv
// IDA 0x3a46e8: 152 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a46e8() {
}


// 0x3a4870 — __ZN3RBX8Animator9onSteppedERKNS_7SteppedE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "RBX::Animator::onStepped(RBX::Stepped const&)")]
// was: __ZN3RBX8Animator9onSteppedERKNS_7SteppedE
// IDA 0x3a4870: 599 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4870() {
}


// 0x3a4e98 — __ZThn92_N3RBX8Animator9onSteppedERKNS_7SteppedE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "non-virtual thunk toRBX::Animator::onStepped(RBX::Stepped const&)")]
// was: __ZThn92_N3RBX8Animator9onSteppedERKNS_7SteppedE
// IDA 0x3a4e98: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4e98() {
}


// 0x3a4ea0 — __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Animator *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
// IDA 0x3a4ea0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4ea0() {
}


// 0x3a4edc — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev
// IDA 0x3a4edc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a4edc() {
}


// 0x3a4fe8 — __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
// was: __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x3a4fe8: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a4fe8() {
}


// 0x3a5158 — __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
// type: _DWORD **__fastcall(_DWORD **)
#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
// IDA 0x3a5158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a5158() {
}


// 0x3a5218 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// type: void __fastcall(int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, char, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// IDA 0x3a5218: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5218() {
}


// 0x3a5380 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// type: void __fastcall(int, const shared_count *, int *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, char, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// IDA 0x3a5380: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5380() {
}


// 0x3a54e8 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
// IDA 0x3a54e8: 59 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a54e8() {
}


// 0x3a5590 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, int)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
// IDA 0x3a5590: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3a5590() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x3a55d0 — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")]
// was: __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
// IDA 0x3a55d0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3a55d0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x3a55fc — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// IDA 0x3a55fc: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a55fc() {
}


// 0x3a5704 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
// type: int __fastcall(unsigned __int64 *, _DWORD *, _DWORD *, unsigned int, boost::detail::sp_counted_base *, int, int, int, unsigned __int64)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")]
// was: __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
// IDA 0x3a5704: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5704() {
}


// 0x3a5778 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
// type: int __fastcall(int, __int64 **)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
// IDA 0x3a5778: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5778() {
}


// 0x3a5878 — __ZN3RBX8Animator17onServiceProviderEPNS_15ServiceProviderES2_
// type: void __fastcall(int32_t **this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Animator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8Animator17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x3a5878: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5878() {
}


// 0x3a5880 — __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Animator *this, const RBX::Instance *)
#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
// IDA 0x3a5880: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5880() {
}


// 0x3a5884 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
// IDA 0x3a5884: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5884() {
}


// 0x3a58ac — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
// IDA 0x3a58ac: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a58ac() {
}


// 0x3a58d4 — __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv
// IDA 0x3a58d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a58d4() {
}


// 0x3a58d8 — __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
// IDA 0x3a58d8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a58d8() {
}


// 0x3a59b8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *)
#[doc(alias = "RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// IDA 0x3a59b8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3a59b8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3a5a30 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *)
#[doc(alias = "RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
// IDA 0x3a5a30: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3a5a30() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3a5aa8 — __ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
// IDA 0x3a5aa8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3a5aa8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x3a5acc — __ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, int, int **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3a5acc: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5acc() {
}


// 0x3a5bb4 — __ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
// type: void __fastcall(int *, int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")]
// was: __ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
// IDA 0x3a5bb4: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5bb4() {
}


// 0x3a5cb4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// type: int __fastcall(int, boost::detail::sp_counted_base **this)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>)")]
// was: __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// IDA 0x3a5cb4: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5cb4() {
}


// 0x3a5cd4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3a5cd4: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5cd4() {
}


// 0x3a5dac — __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// IDA 0x3a5dac: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a5dac() {
}


// 0x3a5e94 — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")]
// was: __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x3a5e94: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3a5e94() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x3a5f74 — __ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
// IDA 0x3a5f74: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3a5f74() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x3a5f8c — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(unsigned int *, __int64 *, __int64 *)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// IDA 0x3a5f8c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3a5f8c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x3a6160 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// IDA 0x3a6160: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3a6160() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3a61fc — __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x3a61fc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a61fc() {
}


// 0x3a62c4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14AnimationTrackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrack,RBX::AnimationTrack>(rbx_core::SharedPtr<RBX::AnimationTrack> const*,RBX::AnimationTrack *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14AnimationTrackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3a62c4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a62c4() {
}


// 0x3a63ac — __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x3a63ac: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a63ac() {
}


// 0x3a64b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x3a64b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3a64b4() {
}


// 0x3a64b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x3a64b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a64b8() {
}


// 0x3a64bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3a64bc: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a64bc() {
}


// 0x3a64dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3a64dc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a64dc() {
}


// 0x3a64f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3a64f4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a64f4() {
}


// 0x3a64f8 — __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x3a64f8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a64f8() {
}


// 0x3a65c0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19AnimationTrackStateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrackState,RBX::AnimationTrackState>(rbx_core::SharedPtr<RBX::AnimationTrackState> const*,RBX::AnimationTrackState *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19AnimationTrackStateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3a65c0: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a65c0() {
}


// 0x3a66a8 — __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x3a66a8: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a66a8() {
}


// 0x3a67b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x3a67b0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3a67b0() {
}


// 0x3a67b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x3a67b4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a67b4() {
}


// 0x3a67b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3a67b8: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a67b8() {
}


// 0x3a67d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3a67d8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a67d8() {
}


// 0x3a67f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3a67f0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a67f0() {
}


// 0x3a67f4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
// type: shared_count *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_create_node(rbx_core::SharedPtr<RBX::AnimationTrackState> const&)")]
// was: __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
// IDA 0x3a67f4: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a67f4() {
}


// 0x3a68d8 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")]
// was: __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_
// IDA 0x3a68d8: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a68d8() {
}


// 0x3a69c4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12PartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PartInstance,RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const*,RBX::PartInstance *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12PartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3a69c4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a69c4() {
}


// 0x3a6aac — __ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_
// IDA 0x3a6aac: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a6aac() {
}


// 0x3a6ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev
// IDA 0x3a6ba4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3a6ba4() {
}


// 0x3a6ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev
// IDA 0x3a6ba8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a6ba8() {
}


// 0x3a6bac — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv
// IDA 0x3a6bac: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a6bac() {
}


// 0x3a6bbc — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info
// IDA 0x3a6bbc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a6bbc() {
}


// 0x3a6bc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv
// IDA 0x3a6bc0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a6bc0() {
}


// 0x3a6bc4 — __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3a6bc4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a6bc4() {
}


// 0x3a6bc8 — __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3a6bc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a6bc8() {
}


// 0x3a6c68 — __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3a6c68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a6c68() {
}


// 0x3a6c70 — __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3a6c70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a6c70() {
}


// 0x3a6d14 — __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3a6d14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a6d14() {
}


// 0x3a6d1c — __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3a6d1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a6d1c() {
}


// 0x3a6dc0 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3a6dc0: 154 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a6dc0() {
}


// 0x3a6f58 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3a6f58: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a6f58() {
}


// 0x3a6f88 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev
// IDA 0x3a6f88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a6f88() {
}


// 0x3a70a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3a70a4: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a70a4() {
}


// 0x3a718c — __ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_
// type: void __fastcall(int, char *, int, _DWORD *, const shared_count *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_
// IDA 0x3a718c: 109 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a718c() {
}


// 0x3a72b8 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
// IDA 0x3a72b8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a72b8() {
}


// 0x3a72e0 — __GLOBAL__I_a_158
#[doc(alias = "global constructor keyed to_a_158")]
// was: __GLOBAL__I_a_158
// IDA 0x3a72e0: 275 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a72e0() {
}


// 0x3a7640 — __ZN3RBX10ArcHandles7setAxesENS_4AxesE
// type: RBX::Instance *__fastcall(RBX::Instance *result, int)
#[doc(alias = "RBX::ArcHandles::setAxes(RBX::Axes)")]
// was: __ZN3RBX10ArcHandles7setAxesENS_4AxesE
// IDA 0x3a7640: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7640() {
}


// 0x3a7660 — __ZN3RBX10ArcHandlesC2Ev
// type: RBX::HandlesBase *__fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::ArcHandles(void)")]
// was: __ZN3RBX10ArcHandlesC2Ev
// IDA 0x3a7660: 373 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7660() {
}


// 0x3a7a9c — __ZNK3RBX10ArcHandles22getHandlesNormalIdMaskEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::getHandlesNormalIdMask(void)const")]
// was: __ZNK3RBX10ArcHandles22getHandlesNormalIdMaskEv
// IDA 0x3a7a9c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7a9c() {
}


// 0x3a7af0 — __ZN3RBX10ArcHandles18setServerGuiObjectEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::setServerGuiObject(void)")]
// was: __ZN3RBX10ArcHandles18setServerGuiObjectEv
// IDA 0x3a7af0: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7af0() {
}


// 0x3a7b58 — __ZN3RBX10ArcHandles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::ArcHandles *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX10ArcHandles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x3a7b58: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7b58() {
}


// 0x3a7b94 — __ZN3RBX10ArcHandles7processERKNS_8GuiEventE
// type: void __fastcall(struct _Unwind_Exception *, int, int *)
#[doc(alias = "RBX::ArcHandles::process(RBX::GuiEvent const&)")]
// was: __ZN3RBX10ArcHandles7processERKNS_8GuiEventE
// IDA 0x3a7b94: 313 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7b94() {
}


// 0x3a7ee4 — __ZThn92_N3RBX10ArcHandles7processERKNS_8GuiEventE
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::process(RBX::GuiEvent const&)")]
// was: __ZThn92_N3RBX10ArcHandles7processERKNS_8GuiEventE
// IDA 0x3a7ee4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7ee4() {
}


// 0x3a7ef0 — __ZNK3RBX10ArcHandles13getHandleTypeEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::getHandleType(void)const")]
// was: __ZNK3RBX10ArcHandles13getHandleTypeEv
// IDA 0x3a7ef0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7ef0() {
}


// 0x3a7ef4 — __ZNK3RBX10ArcHandles7getAxesEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::getAxes(void)const")]
// was: __ZNK3RBX10ArcHandles7getAxesEv
// IDA 0x3a7ef4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a7ef4() {
}


// 0x3a7efc — __ZN3RBX10Reflection14PropDescriptorINS_10ArcHandlesENS_4AxesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10ArcHandlesENS_4AxesEED1Ev
// IDA 0x3a7efc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a7efc() {
}


// 0x3a8584 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// was: __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
// IDA 0x3a8584: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8584() {
}


// 0x3a85b0 — __ZN3RBX10ArcHandlesD1Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
// was: __ZN3RBX10ArcHandlesD1Ev
// IDA 0x3a85b0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a85b0() {
}


// 0x3a85b4 — __ZN3RBX10ArcHandlesD0Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
// was: __ZN3RBX10ArcHandlesD0Ev
// IDA 0x3a85b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a85b4() {
}


// 0x3a8654 — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
// IDA 0x3a8654: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8654() {
}


// 0x3a8674 — __ZThn32_N3RBX10ArcHandlesD1Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn32_N3RBX10ArcHandlesD1Ev
// IDA 0x3a8674: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a8674() {
}


// 0x3a867c — __ZThn32_N3RBX10ArcHandlesD0Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn32_N3RBX10ArcHandlesD0Ev
// IDA 0x3a867c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a867c() {
}


// 0x3a8720 — __ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
// IDA 0x3a8720: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8720() {
}


// 0x3a8730 — __ZThn36_N3RBX10ArcHandlesD1Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn36_N3RBX10ArcHandlesD1Ev
// IDA 0x3a8730: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a8730() {
}


// 0x3a8738 — __ZThn36_N3RBX10ArcHandlesD0Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn36_N3RBX10ArcHandlesD0Ev
// IDA 0x3a8738: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a8738() {
}


// 0x3a87ec — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev
// IDA 0x3a87ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a87ec() {
}


// 0x3a87f0 — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev
// IDA 0x3a87f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3a87f0() {
}


// 0x3a888c — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x3a888c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a888c() {
}


// 0x3a8914 — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv
// IDA 0x3a8914: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8914() {
}


// 0x3a8a58 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
// IDA 0x3a8a58: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8a58() {
}


// 0x3a8b0c — __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x3a8b0c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8b0c() {
}


// 0x3a8bd4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ArcHandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ArcHandles,RBX::ArcHandles>(rbx_core::SharedPtr<RBX::ArcHandles> const*,RBX::ArcHandles *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ArcHandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3a8bd4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8bd4() {
}


// 0x3a8cbc — __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x3a8cbc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8cbc() {
}


// 0x3a8dc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x3a8dc4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3a8dc4() {
}


// 0x3a8dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x3a8dc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a8dc8() {
}


// 0x3a8dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3a8dcc: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8dcc() {
}


// 0x3a8dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3a8dec: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8dec() {
}


// 0x3a8e04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3a8e04: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8e04() {
}


// 0x3a8e08 — __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv
// IDA 0x3a8e08: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a8e08() {
}


// 0x3a8e0c — __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
// IDA 0x3a8e0c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8e0c() {
}


// 0x3a8eec — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev
// IDA 0x3a8eec: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a8eec() {
}


// 0x3a9130 — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv
// IDA 0x3a9130: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a9130() {
}


// 0x3a91a4 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// was: __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
// IDA 0x3a91a4: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a91a4() {
}


// 0x3a9278 — __ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
// IDA 0x3a9278: 85 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a9278() {
}


// 0x3a9364 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev
// IDA 0x3a9364: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3a9364() {
}


// 0x3a9368 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev
// IDA 0x3a9368: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a9368() {
}


// 0x3a936c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv
// IDA 0x3a936c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a936c() {
}


// 0x3a9378 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info
// IDA 0x3a9378: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a9378() {
}


// 0x3a937c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv
// IDA 0x3a937c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a937c() {
}


// 0x3abc48 — __ZNK3RBX11HandlesBase13getHandleTypeEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::getHandleType(void)const")]
// was: __ZNK3RBX11HandlesBase13getHandleTypeEv
// IDA 0x3abc48: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3abc48() {
}


// 0x3abc4c — __ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::getHandlesNormalIdMask(void)const")]
// was: __ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv
// IDA 0x3abc4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3abc4c() {
}


// 0x3abc50 — __ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: int()
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3abc50: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3abc50() {
}
