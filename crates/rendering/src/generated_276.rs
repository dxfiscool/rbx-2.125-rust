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
pub fn stub_3a2778() -> ! {
    todo!("0x3a2778 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")
}


// 0x3a2874 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_3a2874() -> ! {
    todo!("0x3a2874 rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")
}


// 0x3a2984 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_3a2984() -> ! {
    todo!("0x3a2984 rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")
}


// 0x3a2ab4 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
pub fn stub_3a2ab4() -> ! {
    todo!("0x3a2ab4 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}


// 0x3a2abc — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
pub fn stub_3a2abc() -> ! {
    todo!("0x3a2abc non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}


// 0x3a2ac4 — __ZNK5boost9function4IvffffEclEffff
// type: void __fastcall(_DWORD *, int, int, int, float)
#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
// was: __ZNK5boost9function4IvffffEclEffff
pub fn stub_3a2ac4() -> ! {
    todo!("0x3a2ac4 boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")
}


// 0x3a2bac — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
pub fn stub_3a2bac() -> ! {
    todo!("0x3a2bac rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")
}


// 0x3a2cbc — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
pub fn stub_3a2cbc() -> ! {
    todo!("0x3a2cbc rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")
}


// 0x3a2dec — __ZN5boost9function4IvffffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
// was: __ZN5boost9function4IvffffE13assign_to_ownERKS1_
pub fn stub_3a2dec() -> ! {
    todo!("0x3a2dec boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")
}


// 0x3a2e1c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3a2e1c() -> ! {
    todo!("0x3a2e1c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}


// 0x3a30e8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
pub fn stub_3a30e8() -> ! {
    todo!("0x3a30e8 RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}


// 0x3a310c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
pub fn stub_3a310c() -> ! {
    todo!("0x3a310c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")
}


// 0x3a31c0 — __ZN3rbx13remote_signalIFvffffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvffffEED2Ev
pub fn stub_3a31c0() -> ! {
    todo!("0x3a31c0 rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")
}


// 0x3a330c — __ZN3rbx13remote_signalIFvfffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvfffEED2Ev
pub fn stub_3a330c() -> ! {
    todo!("0x3a330c rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")
}


// 0x3a3458 — __ZN3rbx13remote_signalIFvffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvffEED2Ev
pub fn stub_3a3458() -> ! {
    todo!("0x3a3458 rbx::remote_signal<void ()(float,float)>::~remote_signal()")
}


// 0x3a35a4 — __GLOBAL__I_a_157
#[doc(alias = "global constructor keyed to_a_157")]
// was: __GLOBAL__I_a_157
pub fn stub_3a35a4() -> ! {
    todo!("0x3a35a4 global constructor keyed to_a_157")
}


// 0x3a395c — __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, __guard *, const shared_count *, int)
#[doc(alias = "RBX::Animator::loadAnimation(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3a395c() -> ! {
    todo!("0x3a395c RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")
}


// 0x3a3d44 — __ZN3RBX8AnimatorC1EPNS_8InstanceE
// type: int __fastcall(RBX::Animator *this, RBX::Instance *)
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
// was: __ZN3RBX8AnimatorC1EPNS_8InstanceE
pub fn stub_3a3d44() -> ! {
    todo!("0x3a3d44 RBX::Animator::Animator(RBX::Instance *)")
}


// 0x3a3d48 — __ZN3RBX8AnimatorC2EPNS_8InstanceE
// type: RBX::Instance *__fastcall(RBX::Animator *this, RBX::Instance *)
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
// was: __ZN3RBX8AnimatorC2EPNS_8InstanceE
pub fn stub_3a3d48() -> ! {
    todo!("0x3a3d48 RBX::Animator::Animator(RBX::Instance *)")
}


// 0x3a40b8 — __ZN3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "RBX::Animator::~Animator()")]
// was: __ZN3RBX8AnimatorD0Ev
pub fn stub_3a40b8() -> ! {
    todo!("0x3a40b8 RBX::Animator::~Animator()")
}


// 0x3a4158 — __ZN3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "RBX::Animator::~Animator()")]
// was: __ZN3RBX8AnimatorD1Ev
pub fn stub_3a4158() -> ! {
    todo!("0x3a4158 RBX::Animator::~Animator()")
}


// 0x3a415c — __ZThn32_N3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn32_N3RBX8AnimatorD0Ev
pub fn stub_3a415c() -> ! {
    todo!("0x3a415c non-virtual thunk toRBX::Animator::~Animator()")
}


// 0x3a4164 — __ZThn36_N3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn36_N3RBX8AnimatorD0Ev
pub fn stub_3a4164() -> ! {
    todo!("0x3a4164 non-virtual thunk toRBX::Animator::~Animator()")
}


// 0x3a416c — __ZThn92_N3RBX8AnimatorD0Ev
// type: void __fastcall(RBX::Animator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn92_N3RBX8AnimatorD0Ev
pub fn stub_3a416c() -> ! {
    todo!("0x3a416c non-virtual thunk toRBX::Animator::~Animator()")
}


// 0x3a4174 — __ZN3RBX8AnimatorD2Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "RBX::Animator::~Animator()")]
// was: __ZN3RBX8AnimatorD2Ev
pub fn stub_3a4174() -> ! {
    todo!("0x3a4174 RBX::Animator::~Animator()")
}


// 0x3a434c — __ZThn32_N3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn32_N3RBX8AnimatorD1Ev
pub fn stub_3a434c() -> ! {
    todo!("0x3a434c non-virtual thunk toRBX::Animator::~Animator()")
}


// 0x3a4354 — __ZThn36_N3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn36_N3RBX8AnimatorD1Ev
pub fn stub_3a4354() -> ! {
    todo!("0x3a4354 non-virtual thunk toRBX::Animator::~Animator()")
}


// 0x3a435c — __ZThn92_N3RBX8AnimatorD1Ev
// type: void __fastcall(RBX::Animator *this, int, int, const void *)
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
// was: __ZThn92_N3RBX8AnimatorD1Ev
pub fn stub_3a435c() -> ! {
    todo!("0x3a435c non-virtual thunk toRBX::Animator::~Animator()")
}


// 0x3a4364 — __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
// type: int *__fastcall(int, int *, int, int)
#[doc(alias = "RBX::Animator::reloadAnimation(boost::shared_ptr<RBX::AnimationTrackState>)")]
// was: __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
pub fn stub_3a4364() -> ! {
    todo!("0x3a4364 RBX::Animator::reloadAnimation(rbx_core::SharedPtr<RBX::AnimationTrackState>)")
}


// 0x3a439c — __ZNK3RBX8Animator11getGameTimeEv
// type: int __fastcall(RBX::Animator *this)
#[doc(alias = "RBX::Animator::getGameTime(void)const")]
// was: __ZNK3RBX8Animator11getGameTimeEv
pub fn stub_3a439c() -> ! {
    todo!("0x3a439c RBX::Animator::getGameTime(void)const")
}


// 0x3a4474 — __ZN3RBX8Animator23getReplicatingContainerEv
// type: void __fastcall(RBX::Animator *this, int)
#[doc(alias = "RBX::Animator::getReplicatingContainer(void)")]
// was: __ZN3RBX8Animator23getReplicatingContainerEv
pub fn stub_3a4474() -> ! {
    todo!("0x3a4474 RBX::Animator::getReplicatingContainer(void)")
}


// 0x3a4598 — __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
// type: void __fastcall(int, int *, double, int, _DWORD *)
#[doc(alias = "RBX::Animator::onTrackStepped(boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")]
// was: __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
pub fn stub_3a4598() -> ! {
    todo!("0x3a4598 RBX::Animator::onTrackStepped(rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")
}


// 0x3a46a0 — __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
// type: void *__fastcall(int, const void **)
#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3a46a0() -> ! {
    todo!("0x3a46a0 RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")
}


// 0x3a46e8 — __ZN3RBX8Animator20calcAnimatableJointsEv
// type: void __fastcall(RBX::Animator *this)
#[doc(alias = "RBX::Animator::calcAnimatableJoints(void)")]
// was: __ZN3RBX8Animator20calcAnimatableJointsEv
pub fn stub_3a46e8() -> ! {
    todo!("0x3a46e8 RBX::Animator::calcAnimatableJoints(void)")
}


// 0x3a4870 — __ZN3RBX8Animator9onSteppedERKNS_7SteppedE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "RBX::Animator::onStepped(RBX::Stepped const&)")]
// was: __ZN3RBX8Animator9onSteppedERKNS_7SteppedE
pub fn stub_3a4870() -> ! {
    todo!("0x3a4870 RBX::Animator::onStepped(RBX::Stepped const&)")
}


// 0x3a4e98 — __ZThn92_N3RBX8Animator9onSteppedERKNS_7SteppedE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "non-virtual thunk toRBX::Animator::onStepped(RBX::Stepped const&)")]
// was: __ZThn92_N3RBX8Animator9onSteppedERKNS_7SteppedE
pub fn stub_3a4e98() -> ! {
    todo!("0x3a4e98 non-virtual thunk toRBX::Animator::onStepped(RBX::Stepped const&)")
}


// 0x3a4ea0 — __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Animator *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
pub fn stub_3a4ea0() -> ! {
    todo!("0x3a4ea0 RBX::Animator::askAddChild(RBX::Instance const*)const")
}


// 0x3a4edc — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev
pub fn stub_3a4edc() -> ! {
    todo!("0x3a4edc RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}


// 0x3a4fe8 — __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
// was: __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_3a4fe8() -> ! {
    todo!("0x3a4fe8 rbx_core::SharedPtr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")
}


// 0x3a5158 — __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
// type: _DWORD **__fastcall(_DWORD **)
#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
pub fn stub_3a5158() -> ! {
    todo!("0x3a5158 rbx_core::SharedPtr<RBX::AnimatableRootJoint>::~scoped_ptr()")
}


// 0x3a5218 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// type: void __fastcall(int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, char, char, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
pub fn stub_3a5218() -> ! {
    todo!("0x3a5218 rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")
}


// 0x3a5380 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// type: void __fastcall(int, const shared_count *, int *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, char, char, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
pub fn stub_3a5380() -> ! {
    todo!("0x3a5380 rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")
}


// 0x3a54e8 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
pub fn stub_3a54e8() -> ! {
    todo!("0x3a54e8 std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")
}


// 0x3a5590 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, int)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
pub fn stub_3a5590() -> ! {
    todo!("0x3a5590 std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")
}


// 0x3a55d0 — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")]
// was: __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
pub fn stub_3a55d0() -> ! {
    todo!("0x3a55d0 std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")
}


// 0x3a55fc — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
pub fn stub_3a55fc() -> ! {
    todo!("0x3a55fc void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")
}


// 0x3a5704 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
// type: int __fastcall(unsigned __int64 *, _DWORD *, _DWORD *, unsigned int, boost::detail::sp_counted_base *, int, int, int, unsigned __int64)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")]
// was: __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
pub fn stub_3a5704() -> ! {
    todo!("0x3a5704 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")
}


// 0x3a5778 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
// type: int __fastcall(int, __int64 **)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
pub fn stub_3a5778() -> ! {
    todo!("0x3a5778 std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")
}


// 0x3a5878 — __ZN3RBX8Animator17onServiceProviderEPNS_15ServiceProviderES2_
// type: void __fastcall(int32_t **this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Animator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8Animator17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_3a5878() -> ! {
    todo!("0x3a5878 RBX::Animator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}


// 0x3a5880 — __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Animator *this, const RBX::Instance *)
#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
pub fn stub_3a5880() -> ! {
    todo!("0x3a5880 RBX::Animator::askSetParent(RBX::Instance const*)const")
}


// 0x3a5884 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
pub fn stub_3a5884() -> ! {
    todo!("0x3a5884 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")
}


// 0x3a58ac — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
pub fn stub_3a58ac() -> ! {
    todo!("0x3a58ac __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")
}


// 0x3a58d4 — __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv
pub fn stub_3a58d4() -> ! {
    todo!("0x3a58d4 __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")
}


// 0x3a58d8 — __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
pub fn stub_3a58d8() -> ! {
    todo!("0x3a58d8 __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")
}


// 0x3a59b8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *)
#[doc(alias = "RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
pub fn stub_3a59b8() -> ! {
    todo!("0x3a59b8 RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")
}


// 0x3a5a30 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *)
#[doc(alias = "RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
pub fn stub_3a5a30() -> ! {
    todo!("0x3a5a30 RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")
}


// 0x3a5aa8 — __ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
pub fn stub_3a5aa8() -> ! {
    todo!("0x3a5aa8 std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")
}


// 0x3a5acc — __ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, int, int **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3a5acc() -> ! {
    todo!("0x3a5acc void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&> &,int)")
}


// 0x3a5bb4 — __ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
// type: void __fastcall(int *, int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")]
// was: __ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
pub fn stub_3a5bb4() -> ! {
    todo!("0x3a5bb4 boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")
}


// 0x3a5cb4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// type: int __fastcall(int, boost::detail::sp_counted_base **this)
#[doc(alias = "std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>)")]
// was: __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
pub fn stub_3a5cb4() -> ! {
    todo!("0x3a5cb4 std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>)")
}


// 0x3a5cd4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3a5cd4() -> ! {
    todo!("0x3a5cd4 void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}


// 0x3a5dac — __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Animator*,boost::shared_ptr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
pub fn stub_3a5dac() -> ! {
    todo!("0x3a5dac boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")
}


// 0x3a5e94 — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")]
// was: __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_3a5e94() -> ! {
    todo!("0x3a5e94 std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")
}


// 0x3a5f74 — __ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
pub fn stub_3a5f74() -> ! {
    todo!("0x3a5f74 std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")
}


// 0x3a5f8c — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(unsigned int *, __int64 *, __int64 *)
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")]
// was: __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_3a5f8c() -> ! {
    todo!("0x3a5f8c std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")
}


// 0x3a6160 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
pub fn stub_3a6160() -> ! {
    todo!("0x3a6160 RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")
}


// 0x3a61fc — __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3a61fc() -> ! {
    todo!("0x3a61fc rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x3a62c4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14AnimationTrackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrack,RBX::AnimationTrack>(boost::shared_ptr<RBX::AnimationTrack> const*,RBX::AnimationTrack *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14AnimationTrackES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3a62c4() -> ! {
    todo!("0x3a62c4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrack,RBX::AnimationTrack>(rbx_core::SharedPtr<RBX::AnimationTrack> const*,RBX::AnimationTrack *)const")
}


// 0x3a63ac — __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3a63ac() -> ! {
    todo!("0x3a63ac boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x3a64b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_3a64b4() -> ! {
    todo!("0x3a64b4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x3a64b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3a64b8() -> ! {
    todo!("0x3a64b8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x3a64bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3a64bc() -> ! {
    todo!("0x3a64bc boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x3a64dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3a64dc() -> ! {
    todo!("0x3a64dc boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x3a64f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3a64f4() -> ! {
    todo!("0x3a64f4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x3a64f8 — __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3a64f8() -> ! {
    todo!("0x3a64f8 rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x3a65c0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19AnimationTrackStateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrackState,RBX::AnimationTrackState>(boost::shared_ptr<RBX::AnimationTrackState> const*,RBX::AnimationTrackState *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19AnimationTrackStateES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3a65c0() -> ! {
    todo!("0x3a65c0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrackState,RBX::AnimationTrackState>(rbx_core::SharedPtr<RBX::AnimationTrackState> const*,RBX::AnimationTrackState *)const")
}


// 0x3a66a8 — __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3a66a8() -> ! {
    todo!("0x3a66a8 boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x3a67b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_3a67b0() -> ! {
    todo!("0x3a67b0 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x3a67b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3a67b4() -> ! {
    todo!("0x3a67b4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x3a67b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3a67b8() -> ! {
    todo!("0x3a67b8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x3a67d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3a67d8() -> ! {
    todo!("0x3a67d8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x3a67f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3a67f0() -> ! {
    todo!("0x3a67f0 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x3a67f4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
// type: shared_count *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_create_node(boost::shared_ptr<RBX::AnimationTrackState> const&)")]
// was: __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
pub fn stub_3a67f4() -> ! {
    todo!("0x3a67f4 std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_create_node(rbx_core::SharedPtr<RBX::AnimationTrackState> const&)")
}


// 0x3a68d8 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")]
// was: __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_
pub fn stub_3a68d8() -> ! {
    todo!("0x3a68d8 rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")
}


// 0x3a69c4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12PartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PartInstance,RBX::PartInstance>(boost::shared_ptr<RBX::PartInstance> const*,RBX::PartInstance *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12PartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3a69c4() -> ! {
    todo!("0x3a69c4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PartInstance,RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const*,RBX::PartInstance *)const")
}


// 0x3a6aac — __ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_
pub fn stub_3a6aac() -> ! {
    todo!("0x3a6aac boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")
}


// 0x3a6ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev
pub fn stub_3a6ba4() -> ! {
    todo!("0x3a6ba4 boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")
}


// 0x3a6ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev
pub fn stub_3a6ba8() -> ! {
    todo!("0x3a6ba8 boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")
}


// 0x3a6bac — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv
pub fn stub_3a6bac() -> ! {
    todo!("0x3a6bac boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")
}


// 0x3a6bbc — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info
pub fn stub_3a6bbc() -> ! {
    todo!("0x3a6bbc boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")
}


// 0x3a6bc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv
pub fn stub_3a6bc0() -> ! {
    todo!("0x3a6bc0 boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")
}


// 0x3a6bc4 — __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3a6bc4() -> ! {
    todo!("0x3a6bc4 __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3a6bc8 — __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3a6bc8() -> ! {
    todo!("0x3a6bc8 __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3a6c68 — __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3a6c68() -> ! {
    todo!("0x3a6c68 __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3a6c70 — __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3a6c70() -> ! {
    todo!("0x3a6c70 __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3a6d14 — __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3a6d14() -> ! {
    todo!("0x3a6d14 __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}


// 0x3a6d1c — __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3a6d1c() -> ! {
    todo!("0x3a6d1c __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}


// 0x3a6dc0 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Animator::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_3a6dc0() -> ! {
    todo!("0x3a6dc0 RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}


// 0x3a6f58 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_3a6f58() -> ! {
    todo!("0x3a6f58 RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}


// 0x3a6f88 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev
pub fn stub_3a6f88() -> ! {
    todo!("0x3a6f88 RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")
}


// 0x3a70a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_3a70a4() -> ! {
    todo!("0x3a70a4 RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}


// 0x3a718c — __ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_
// type: void __fastcall(int, char *, int, _DWORD *, const shared_count *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Animator,boost::shared_ptr<RBX::Instance> (RBX::Animator::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::call(RBX::Animator*,boost::shared_ptr<RBX::Instance> (RBX::Animator::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_
pub fn stub_3a718c() -> ! {
    todo!("0x3a718c RBX::Reflection::Call1Helper<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}


// 0x3a72b8 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
pub fn stub_3a72b8() -> ! {
    todo!("0x3a72b8 std::_List_base<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_clear(void)")
}


// 0x3a72e0 — __GLOBAL__I_a_158
#[doc(alias = "global constructor keyed to_a_158")]
// was: __GLOBAL__I_a_158
pub fn stub_3a72e0() -> ! {
    todo!("0x3a72e0 global constructor keyed to_a_158")
}


// 0x3a7640 — __ZN3RBX10ArcHandles7setAxesENS_4AxesE
// type: RBX::Instance *__fastcall(RBX::Instance *result, int)
#[doc(alias = "RBX::ArcHandles::setAxes(RBX::Axes)")]
// was: __ZN3RBX10ArcHandles7setAxesENS_4AxesE
pub fn stub_3a7640() -> ! {
    todo!("0x3a7640 RBX::ArcHandles::setAxes(RBX::Axes)")
}


// 0x3a7660 — __ZN3RBX10ArcHandlesC2Ev
// type: RBX::HandlesBase *__fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::ArcHandles(void)")]
// was: __ZN3RBX10ArcHandlesC2Ev
pub fn stub_3a7660() -> ! {
    todo!("0x3a7660 RBX::ArcHandles::ArcHandles(void)")
}


// 0x3a7a9c — __ZNK3RBX10ArcHandles22getHandlesNormalIdMaskEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::getHandlesNormalIdMask(void)const")]
// was: __ZNK3RBX10ArcHandles22getHandlesNormalIdMaskEv
pub fn stub_3a7a9c() -> ! {
    todo!("0x3a7a9c RBX::ArcHandles::getHandlesNormalIdMask(void)const")
}


// 0x3a7af0 — __ZN3RBX10ArcHandles18setServerGuiObjectEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::setServerGuiObject(void)")]
// was: __ZN3RBX10ArcHandles18setServerGuiObjectEv
pub fn stub_3a7af0() -> ! {
    todo!("0x3a7af0 RBX::ArcHandles::setServerGuiObject(void)")
}


// 0x3a7b58 — __ZN3RBX10ArcHandles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::ArcHandles *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX10ArcHandles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3a7b58() -> ! {
    todo!("0x3a7b58 RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}


// 0x3a7b94 — __ZN3RBX10ArcHandles7processERKNS_8GuiEventE
// type: void __fastcall(struct _Unwind_Exception *, int, int *)
#[doc(alias = "RBX::ArcHandles::process(RBX::GuiEvent const&)")]
// was: __ZN3RBX10ArcHandles7processERKNS_8GuiEventE
pub fn stub_3a7b94() -> ! {
    todo!("0x3a7b94 RBX::ArcHandles::process(RBX::GuiEvent const&)")
}


// 0x3a7ee4 — __ZThn92_N3RBX10ArcHandles7processERKNS_8GuiEventE
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::process(RBX::GuiEvent const&)")]
// was: __ZThn92_N3RBX10ArcHandles7processERKNS_8GuiEventE
pub fn stub_3a7ee4() -> ! {
    todo!("0x3a7ee4 non-virtual thunk toRBX::ArcHandles::process(RBX::GuiEvent const&)")
}


// 0x3a7ef0 — __ZNK3RBX10ArcHandles13getHandleTypeEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::getHandleType(void)const")]
// was: __ZNK3RBX10ArcHandles13getHandleTypeEv
pub fn stub_3a7ef0() -> ! {
    todo!("0x3a7ef0 RBX::ArcHandles::getHandleType(void)const")
}


// 0x3a7ef4 — __ZNK3RBX10ArcHandles7getAxesEv
// type: int __fastcall(RBX::ArcHandles *this)
#[doc(alias = "RBX::ArcHandles::getAxes(void)const")]
// was: __ZNK3RBX10ArcHandles7getAxesEv
pub fn stub_3a7ef4() -> ! {
    todo!("0x3a7ef4 RBX::ArcHandles::getAxes(void)const")
}


// 0x3a7efc — __ZN3RBX10Reflection14PropDescriptorINS_10ArcHandlesENS_4AxesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_10ArcHandlesENS_4AxesEED1Ev
pub fn stub_3a7efc() -> ! {
    todo!("0x3a7efc RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::~PropDescriptor()")
}


// 0x3a8584 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// was: __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
pub fn stub_3a8584() -> ! {
    todo!("0x3a8584 void rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")
}


// 0x3a85b0 — __ZN3RBX10ArcHandlesD1Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
// was: __ZN3RBX10ArcHandlesD1Ev
pub fn stub_3a85b0() -> ! {
    todo!("0x3a85b0 RBX::ArcHandles::~ArcHandles()")
}


// 0x3a85b4 — __ZN3RBX10ArcHandlesD0Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
// was: __ZN3RBX10ArcHandlesD0Ev
pub fn stub_3a85b4() -> ! {
    todo!("0x3a85b4 RBX::ArcHandles::~ArcHandles()")
}


// 0x3a8654 — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
pub fn stub_3a8654() -> ! {
    todo!("0x3a8654 __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")
}


// 0x3a8674 — __ZThn32_N3RBX10ArcHandlesD1Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn32_N3RBX10ArcHandlesD1Ev
pub fn stub_3a8674() -> ! {
    todo!("0x3a8674 non-virtual thunk toRBX::ArcHandles::~ArcHandles()")
}


// 0x3a867c — __ZThn32_N3RBX10ArcHandlesD0Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn32_N3RBX10ArcHandlesD0Ev
pub fn stub_3a867c() -> ! {
    todo!("0x3a867c non-virtual thunk toRBX::ArcHandles::~ArcHandles()")
}


// 0x3a8720 — __ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
pub fn stub_3a8720() -> ! {
    todo!("0x3a8720 __ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")
}


// 0x3a8730 — __ZThn36_N3RBX10ArcHandlesD1Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn36_N3RBX10ArcHandlesD1Ev
pub fn stub_3a8730() -> ! {
    todo!("0x3a8730 non-virtual thunk toRBX::ArcHandles::~ArcHandles()")
}


// 0x3a8738 — __ZThn36_N3RBX10ArcHandlesD0Ev
// type: void __fastcall(RBX::ArcHandles *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
// was: __ZThn36_N3RBX10ArcHandlesD0Ev
pub fn stub_3a8738() -> ! {
    todo!("0x3a8738 non-virtual thunk toRBX::ArcHandles::~ArcHandles()")
}


// 0x3a87ec — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev
pub fn stub_3a87ec() -> ! {
    todo!("0x3a87ec __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev")
}


// 0x3a87f0 — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev
pub fn stub_3a87f0() -> ! {
    todo!("0x3a87f0 __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev")
}


// 0x3a888c — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3a888c() -> ! {
    todo!("0x3a888c __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")
}


// 0x3a8914 — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv
pub fn stub_3a8914() -> ! {
    todo!("0x3a8914 __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")
}


// 0x3a8a58 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
pub fn stub_3a8a58() -> ! {
    todo!("0x3a8a58 rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")
}


// 0x3a8b0c — __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3a8b0c() -> ! {
    todo!("0x3a8b0c rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x3a8bd4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ArcHandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ArcHandles,RBX::ArcHandles>(boost::shared_ptr<RBX::ArcHandles> const*,RBX::ArcHandles *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ArcHandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3a8bd4() -> ! {
    todo!("0x3a8bd4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ArcHandles,RBX::ArcHandles>(rbx_core::SharedPtr<RBX::ArcHandles> const*,RBX::ArcHandles *)const")
}


// 0x3a8cbc — __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3a8cbc() -> ! {
    todo!("0x3a8cbc boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x3a8dc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_3a8dc4() -> ! {
    todo!("0x3a8dc4 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x3a8dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_3a8dc8() -> ! {
    todo!("0x3a8dc8 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x3a8dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3a8dcc() -> ! {
    todo!("0x3a8dcc boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x3a8dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3a8dec() -> ! {
    todo!("0x3a8dec boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x3a8e04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3a8e04() -> ! {
    todo!("0x3a8e04 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x3a8e08 — __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv
pub fn stub_3a8e08() -> ! {
    todo!("0x3a8e08 __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")
}


// 0x3a8e0c — __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
pub fn stub_3a8e0c() -> ! {
    todo!("0x3a8e0c __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")
}


// 0x3a8eec — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev
pub fn stub_3a8eec() -> ! {
    todo!("0x3a8eec __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")
}


// 0x3a9130 — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv
pub fn stub_3a9130() -> ! {
    todo!("0x3a9130 __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")
}


// 0x3a91a4 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// was: __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
pub fn stub_3a91a4() -> ! {
    todo!("0x3a91a4 rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")
}


// 0x3a9278 — __ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
pub fn stub_3a9278() -> ! {
    todo!("0x3a9278 boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")
}


// 0x3a9364 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev
pub fn stub_3a9364() -> ! {
    todo!("0x3a9364 boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")
}


// 0x3a9368 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev
pub fn stub_3a9368() -> ! {
    todo!("0x3a9368 boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")
}


// 0x3a936c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv
pub fn stub_3a936c() -> ! {
    todo!("0x3a936c boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")
}


// 0x3a9378 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info
pub fn stub_3a9378() -> ! {
    todo!("0x3a9378 boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")
}


// 0x3a937c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv
pub fn stub_3a937c() -> ! {
    todo!("0x3a937c boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")
}


// 0x3abc48 — __ZNK3RBX11HandlesBase13getHandleTypeEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::getHandleType(void)const")]
// was: __ZNK3RBX11HandlesBase13getHandleTypeEv
pub fn stub_3abc48() -> ! {
    todo!("0x3abc48 RBX::HandlesBase::getHandleType(void)const")
}


// 0x3abc4c — __ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::getHandlesNormalIdMask(void)const")]
// was: __ZNK3RBX11HandlesBase22getHandlesNormalIdMaskEv
pub fn stub_3abc4c() -> ! {
    todo!("0x3abc4c RBX::HandlesBase::getHandlesNormalIdMask(void)const")
}


// 0x3abc50 — __ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: int()
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3abc50() -> ! {
    todo!("0x3abc50 __ZN3RBX10Reflection9DescribedINS_10ArcHandlesELZNS_11sArcHandlesEENS_14FactoryProductIS2_NS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}
