//! reflection — generated_refl_wdog3B_1788372279 — 120 stubs EA-sorted asc 0x3a2cbc..0x3a9278 (reflection gap filler distinct not yet in crates/reflection/src — next 120 uncovered; RBX::Reflection filter yielded 0 remaining so global reflection-gap asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc not yet in crates/reflection/src — next 120 uncovered sorted asc (RBX::Reflection strict filter exhausted 0 remaining -> fallback to global reflection-gap)
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3a2cbc — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev")]
pub fn stub_0x3a2cbc() {
    // IDA 0x3a2cbc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a2dec — __ZN5boost9function4IvffffE13assign_to_ownERKS1_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
#[doc(alias = "__ZN5boost9function4IvffffE13assign_to_ownERKS1_")]
pub fn stub_0x3a2dec() -> ! {
    todo!("0x3a2dec boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")
}

// 0x3a31c0 — __ZN3rbx13remote_signalIFvffffEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEED2Ev")]
pub fn stub_0x3a31c0() {
    // IDA 0x3a31c0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3a330c — __ZN3rbx13remote_signalIFvfffEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEED2Ev")]
pub fn stub_0x3a330c() {
    // IDA 0x3a330c: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3a3458 — __ZN3rbx13remote_signalIFvffEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvffEED2Ev")]
pub fn stub_0x3a3458() {
    // IDA 0x3a3458: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3a35a4 — __GLOBAL__I_a_157
#[doc(alias = "global constructor keyed to_a_157")]
#[doc(alias = "__GLOBAL__I_a_157")]
pub fn stub_0x3a35a4() -> ! {
    todo!("0x3a35a4 global constructor keyed to_a_157")
}

// 0x3a395c — __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Animator::loadAnimation(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x3a395c() -> ! {
    todo!("0x3a395c RBX::Animator::loadAnimation(boost::shared_ptr<RBX::Instance>)")
}

// 0x3a3d44 — __ZN3RBX8AnimatorC1EPNS_8InstanceE
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8AnimatorC1EPNS_8InstanceE")]
pub fn stub_0x3a3d44() -> ! {
    todo!("0x3a3d44 RBX::Animator::Animator(RBX::Instance *)")
}

// 0x3a3d48 — __ZN3RBX8AnimatorC2EPNS_8InstanceE
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8AnimatorC2EPNS_8InstanceE")]
pub fn stub_0x3a3d48() -> ! {
    todo!("0x3a3d48 RBX::Animator::Animator(RBX::Instance *)")
}

// 0x3a40b8 — __ZN3RBX8AnimatorD0Ev
#[doc(alias = "RBX::Animator::~Animator()")]
#[doc(alias = "__ZN3RBX8AnimatorD0Ev")]
pub fn stub_0x3a40b8() {
    // IDA 0x3a40b8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a4158 — __ZN3RBX8AnimatorD1Ev
#[doc(alias = "RBX::Animator::~Animator()")]
#[doc(alias = "__ZN3RBX8AnimatorD1Ev")]
pub fn stub_0x3a4158() {
    // IDA 0x3a4158: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a415c — __ZThn32_N3RBX8AnimatorD0Ev
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
#[doc(alias = "__ZThn32_N3RBX8AnimatorD0Ev")]
pub fn stub_0x3a415c() {
    // IDA 0x3a415c: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a4164 — __ZThn36_N3RBX8AnimatorD0Ev
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
#[doc(alias = "__ZThn36_N3RBX8AnimatorD0Ev")]
pub fn stub_0x3a4164() {
    // IDA 0x3a4164: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a416c — __ZThn92_N3RBX8AnimatorD0Ev
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
#[doc(alias = "__ZThn92_N3RBX8AnimatorD0Ev")]
pub fn stub_0x3a416c() {
    // IDA 0x3a416c: __ZThn92 thunk (D0 deleting dtor): `this -= 92`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a4174 — __ZN3RBX8AnimatorD2Ev
#[doc(alias = "RBX::Animator::~Animator()")]
#[doc(alias = "__ZN3RBX8AnimatorD2Ev")]
pub fn stub_0x3a4174() {
    // IDA 0x3a4174: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3a434c — __ZThn32_N3RBX8AnimatorD1Ev
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
#[doc(alias = "__ZThn32_N3RBX8AnimatorD1Ev")]
pub fn stub_0x3a434c() {
    // IDA 0x3a434c: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3a4354 — __ZThn36_N3RBX8AnimatorD1Ev
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
#[doc(alias = "__ZThn36_N3RBX8AnimatorD1Ev")]
pub fn stub_0x3a4354() {
    // IDA 0x3a4354: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3a435c — __ZThn92_N3RBX8AnimatorD1Ev
#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
#[doc(alias = "__ZThn92_N3RBX8AnimatorD1Ev")]
pub fn stub_0x3a435c() {
    // IDA 0x3a435c: __ZThn92 thunk (D1 base dtor): `this -= 92`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3a4364 — __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Animator::reloadAnimation(boost::shared_ptr<RBX::AnimationTrackState>)")]
#[doc(alias = "__ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE")]
pub fn stub_0x3a4364() -> ! {
    todo!("0x3a4364 RBX::Animator::reloadAnimation(boost::shared_ptr<RBX::AnimationTrackState>)")
}

// 0x3a439c — __ZNK3RBX8Animator11getGameTimeEv
#[doc(alias = "RBX::Animator::getGameTime(void)const")]
#[doc(alias = "__ZNK3RBX8Animator11getGameTimeEv")]
pub fn stub_0x3a439c() -> ! {
    todo!("0x3a439c RBX::Animator::getGameTime(void)const")
}

// 0x3a4474 — __ZN3RBX8Animator23getReplicatingContainerEv
#[doc(alias = "RBX::Animator::getReplicatingContainer(void)")]
#[doc(alias = "__ZN3RBX8Animator23getReplicatingContainerEv")]
pub fn stub_0x3a4474() -> ! {
    todo!("0x3a4474 RBX::Animator::getReplicatingContainer(void)")
}

// 0x3a4598 — __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Animator::onTrackStepped(boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")]
#[doc(alias = "__ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE")]
pub fn stub_0x3a4598() -> ! {
    todo!("0x3a4598 RBX::Animator::onTrackStepped(boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")
}

// 0x3a46a0 — __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x3a46a0() -> ! {
    todo!("0x3a46a0 RBX::Animator::appendAnimatableJointsRec(boost::shared_ptr<RBX::Instance>)")
}

// 0x3a46e8 — __ZN3RBX8Animator20calcAnimatableJointsEv
#[doc(alias = "RBX::Animator::calcAnimatableJoints(void)")]
#[doc(alias = "__ZN3RBX8Animator20calcAnimatableJointsEv")]
pub fn stub_0x3a46e8() -> ! {
    todo!("0x3a46e8 RBX::Animator::calcAnimatableJoints(void)")
}

// 0x3a4870 — __ZN3RBX8Animator9onSteppedERKNS_7SteppedE
#[doc(alias = "RBX::Animator::onStepped(RBX::Stepped const&)")]
#[doc(alias = "__ZN3RBX8Animator9onSteppedERKNS_7SteppedE")]
pub fn stub_0x3a4870() -> ! {
    todo!("0x3a4870 RBX::Animator::onStepped(RBX::Stepped const&)")
}

// 0x3a4e98 — __ZThn92_N3RBX8Animator9onSteppedERKNS_7SteppedE
#[doc(alias = "non-virtual thunk toRBX::Animator::onStepped(RBX::Stepped const&)")]
#[doc(alias = "__ZThn92_N3RBX8Animator9onSteppedERKNS_7SteppedE")]
pub fn stub_0x3a4e98() {
    // IDA 0x3a4e98: non-virtual thunk to `RBX::Animator::onStepped(int a1, _DWORD *a2) { RBX::Animator::onStepped(a1 - 92, a2); } "` — this/arg-adjust + tail-call (arg a1 -= 92) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a4ea0 — __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE")]
pub fn stub_0x3a4ea0() -> ! {
    todo!("0x3a4ea0 RBX::Animator::askAddChild(RBX::Instance const*)const")
}

// 0x3a4fe8 — __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0x3a4fe8() -> ! {
    todo!("0x3a4fe8 boost::shared_ptr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")
}

// 0x3a5158 — __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev")]
pub fn stub_0x3a5158() {
    // IDA 0x3a5158: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a5218 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_")]
pub fn stub_0x3a5218() -> ! {
    todo!("0x3a5218 boost::shared_ptr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::KeyframeSequence const>,boost::shared_ptr<RBX::Animator>)")
}

// 0x3a5380 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_")]
pub fn stub_0x3a5380() -> ! {
    todo!("0x3a5380 boost::shared_ptr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>>(boost::shared_ptr<RBX::AnimationTrackState>,boost::shared_ptr<RBX::Animator>)")
}

// 0x3a54e8 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE7reserveEm")]
pub fn stub_0x3a54e8() -> ! {
    todo!("0x3a54e8 std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")
}

// 0x3a5590 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE9push_backERKS1_")]
pub fn stub_0x3a5590() -> ! {
    todo!("0x3a5590 std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")
}

// 0x3a55d0 — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE9push_backERKS2_")]
pub fn stub_0x3a55d0() -> ! {
    todo!("0x3a55d0 std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")
}

// 0x3a55fc — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0x3a55fc() -> ! {
    todo!("0x3a55fc void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")
}

// 0x3a5704 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")]
#[doc(alias = "__ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_")]
pub fn stub_0x3a5704() -> ! {
    todo!("0x3a5704 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")
}

// 0x3a5778 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EEaSERKS3_")]
pub fn stub_0x3a5778() -> ! {
    todo!("0x3a5778 std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")
}

// 0x3a5878 — __ZN3RBX8Animator17onServiceProviderEPNS_15ServiceProviderES2_
#[doc(alias = "RBX::Animator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX8Animator17onServiceProviderEPNS_15ServiceProviderES2_")]
pub fn stub_0x3a5878() -> ! {
    todo!("0x3a5878 RBX::Animator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x3a5880 — __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x3a5880() -> ! {
    todo!("0x3a5880 RBX::Animator::askSetParent(RBX::Instance const*)const")
}

// 0x3a5884 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
pub fn stub_0x3a5884() -> ! {
    todo!("0x3a5884 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")
}

// 0x3a58ac — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
pub fn stub_0x3a58ac() -> &'static str {
    // IDA 0x3a58ac: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x3a58ac family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "Animator"
}

// 0x3a58d4 — __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")]
pub fn stub_0x3a58d4() -> ! {
    todo!("0x3a58d4 __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")
}

// 0x3a58d8 — __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
pub fn stub_0x3a58d8() -> ! {
    todo!("0x3a58d8 __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")
}

// 0x3a59b8 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
#[doc(alias = "RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")]
pub fn stub_0x3a59b8() -> ! {
    todo!("0x3a59b8 RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")
}

// 0x3a5a30 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_
#[doc(alias = "RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX15PoseAccumulatorEPS4_EET0_T_S9_S8_")]
pub fn stub_0x3a5a30() -> ! {
    todo!("0x3a5a30 RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")
}

// 0x3a5aa8 — __ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX15PoseAccumulatorESaIS1_EE11_M_allocateEm")]
pub fn stub_0x3a5aa8() -> ! {
    todo!("0x3a5aa8 std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")
}

// 0x3a5acc — __ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x3a5acc() -> ! {
    todo!("0x3a5acc void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::AnimationTrackState>&> &,int)")
}

// 0x3a5bb4 — __ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_")]
pub fn stub_0x3a5bb4() -> ! {
    todo!("0x3a5bb4 boost::_mfi::mf4<void,RBX::Animator,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")
}

// 0x3a5cb4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")]
pub fn stub_0x3a5cb4() -> ! {
    todo!("0x3a5cb4 std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::AnimationTrackState>>)")
}

// 0x3a5cd4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x3a5cd4() -> ! {
    todo!("0x3a5cd4 void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")
}

// 0x3a5dac — __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Animator*,boost::shared_ptr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")]
pub fn stub_0x3a5dac() -> ! {
    todo!("0x3a5dac boost::_mfi::mf1<void,RBX::Animator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Animator*,boost::shared_ptr<RBX::Instance>)const")
}

// 0x3a5e94 — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x3a5e94() -> ! {
    todo!("0x3a5e94 std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")
}

// 0x3a5f74 — __ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX16IAnimatableJointESaIS2_EE11_M_allocateEm")]
pub fn stub_0x3a5f74() -> ! {
    todo!("0x3a5f74 std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")
}

// 0x3a5f8c — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
pub fn stub_0x3a5f8c() -> ! {
    todo!("0x3a5f8c std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")
}

// 0x3a6160 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_
#[doc(alias = "RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX15PoseAccumulatorES5_EET0_T_S7_S6_")]
pub fn stub_0x3a6160() -> ! {
    todo!("0x3a6160 RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")
}

// 0x3a61fc — __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3a61fc() -> ! {
    todo!("0x3a61fc boost::shared_ptr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a63ac — __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3a63ac() -> ! {
    todo!("0x3a63ac boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a64b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3a64b4() {
    // IDA 0x3a64b4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a64b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3a64b8() {
    // IDA 0x3a64b8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a64bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3a64bc() -> ! {
    todo!("0x3a64bc boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3a64dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a64dc() -> ! {
    todo!("0x3a64dc boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3a64f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3a64f4() -> ! {
    todo!("0x3a64f4 boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3a64f8 — __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3a64f8() -> ! {
    todo!("0x3a64f8 boost::shared_ptr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a66a8 — __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3a66a8() -> ! {
    todo!("0x3a66a8 boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a67b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3a67b0() {
    // IDA 0x3a67b0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a67b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3a67b4() {
    // IDA 0x3a67b4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a67b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3a67b8() -> ! {
    todo!("0x3a67b8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3a67d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a67d8() -> ! {
    todo!("0x3a67d8 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3a67f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3a67f0() -> ! {
    todo!("0x3a67f0 boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3a67f4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_create_node(boost::shared_ptr<RBX::AnimationTrackState> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_")]
pub fn stub_0x3a67f4() -> ! {
    todo!("0x3a67f4 std::list<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_create_node(boost::shared_ptr<RBX::AnimationTrackState> const&)")
}

// 0x3a68d8 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_")]
pub fn stub_0x3a68d8() -> ! {
    todo!("0x3a68d8 boost::shared_ptr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")
}

// 0x3a6aac — __ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_")]
pub fn stub_0x3a6aac() -> ! {
    todo!("0x3a6aac boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")
}

// 0x3a6ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev")]
pub fn stub_0x3a6ba4() {
    // IDA 0x3a6ba4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a6ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev")]
pub fn stub_0x3a6ba8() {
    // IDA 0x3a6ba8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a6bac — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv")]
pub fn stub_0x3a6bac() -> ! {
    todo!("0x3a6bac boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")
}

// 0x3a6bbc — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a6bbc() -> ! {
    todo!("0x3a6bbc boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")
}

// 0x3a6bc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv")]
pub fn stub_0x3a6bc0() -> ! {
    todo!("0x3a6bc0 boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")
}

// 0x3a72b8 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_List_base<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv")]
pub fn stub_0x3a72b8() -> ! {
    todo!("0x3a72b8 std::_List_base<boost::shared_ptr<RBX::AnimationTrackState>,std::allocator<boost::shared_ptr<RBX::AnimationTrackState>>>::_M_clear(void)")
}

// 0x3a72e0 — __GLOBAL__I_a_158
#[doc(alias = "global constructor keyed to_a_158")]
#[doc(alias = "__GLOBAL__I_a_158")]
pub fn stub_0x3a72e0() -> ! {
    todo!("0x3a72e0 global constructor keyed to_a_158")
}

// 0x3a7640 — __ZN3RBX10ArcHandles7setAxesENS_4AxesE
#[doc(alias = "RBX::ArcHandles::setAxes(RBX::Axes)")]
#[doc(alias = "__ZN3RBX10ArcHandles7setAxesENS_4AxesE")]
pub fn stub_0x3a7640() -> ! {
    todo!("0x3a7640 RBX::ArcHandles::setAxes(RBX::Axes)")
}

// 0x3a7660 — __ZN3RBX10ArcHandlesC2Ev
#[doc(alias = "RBX::ArcHandles::ArcHandles(void)")]
#[doc(alias = "__ZN3RBX10ArcHandlesC2Ev")]
pub fn stub_0x3a7660() -> ! {
    todo!("0x3a7660 RBX::ArcHandles::ArcHandles(void)")
}

// 0x3a7a9c — __ZNK3RBX10ArcHandles22getHandlesNormalIdMaskEv
#[doc(alias = "RBX::ArcHandles::getHandlesNormalIdMask(void)const")]
#[doc(alias = "__ZNK3RBX10ArcHandles22getHandlesNormalIdMaskEv")]
pub fn stub_0x3a7a9c() -> ! {
    todo!("0x3a7a9c RBX::ArcHandles::getHandlesNormalIdMask(void)const")
}

// 0x3a7af0 — __ZN3RBX10ArcHandles18setServerGuiObjectEv
#[doc(alias = "RBX::ArcHandles::setServerGuiObject(void)")]
#[doc(alias = "__ZN3RBX10ArcHandles18setServerGuiObjectEv")]
pub fn stub_0x3a7af0() -> ! {
    todo!("0x3a7af0 RBX::ArcHandles::setServerGuiObject(void)")
}

// 0x3a7b94 — __ZN3RBX10ArcHandles7processERKNS_8GuiEventE
#[doc(alias = "RBX::ArcHandles::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX10ArcHandles7processERKNS_8GuiEventE")]
pub fn stub_0x3a7b94() -> ! {
    todo!("0x3a7b94 RBX::ArcHandles::process(RBX::GuiEvent const&)")
}

// 0x3a7ee4 — __ZThn92_N3RBX10ArcHandles7processERKNS_8GuiEventE
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn92_N3RBX10ArcHandles7processERKNS_8GuiEventE")]
pub fn stub_0x3a7ee4() {
    // IDA 0x3a7ee4: non-virtual thunk to `RBX::ArcHandles::process(int a1, int a2) { return RBX::ArcHandles::process(a1, a2 - 92); } "` — this/arg-adjust + tail-call (arg a2 -= 92) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a7ef0 — __ZNK3RBX10ArcHandles13getHandleTypeEv
#[doc(alias = "RBX::ArcHandles::getHandleType(void)const")]
#[doc(alias = "__ZNK3RBX10ArcHandles13getHandleTypeEv")]
pub fn stub_0x3a7ef0() -> ! {
    todo!("0x3a7ef0 RBX::ArcHandles::getHandleType(void)const")
}

// 0x3a7ef4 — __ZNK3RBX10ArcHandles7getAxesEv
#[doc(alias = "RBX::ArcHandles::getAxes(void)const")]
#[doc(alias = "__ZNK3RBX10ArcHandles7getAxesEv")]
pub fn stub_0x3a7ef4() -> ! {
    todo!("0x3a7ef4 RBX::ArcHandles::getAxes(void)const")
}

// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb")]
pub fn stub_0x3a7f68() -> ! {
    todo!("0x3a7f68 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")
}

// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb")]
pub fn stub_0x3a80c8() -> ! {
    todo!("0x3a80c8 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")
}

// 0x3a82e8 — __ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff")]
pub fn stub_0x3a82e8() -> ! {
    todo!("0x3a82e8 rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")
}

// 0x3a8440 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_")]
pub fn stub_0x3a8440() -> ! {
    todo!("0x3a8440 rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")
}

// 0x3a8584 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_")]
pub fn stub_0x3a8584() -> ! {
    todo!("0x3a8584 void boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")
}

// 0x3a85b0 — __ZN3RBX10ArcHandlesD1Ev
#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "__ZN3RBX10ArcHandlesD1Ev")]
pub fn stub_0x3a85b0() {
    // IDA 0x3a85b0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a85b4 — __ZN3RBX10ArcHandlesD0Ev
#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "__ZN3RBX10ArcHandlesD0Ev")]
pub fn stub_0x3a85b4() {
    // IDA 0x3a85b4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a8654 — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3a8654() -> ! {
    todo!("0x3a8654 __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")
}

// 0x3a8664 — __ZNK3RBX11HandlesBase14shouldRender2dEv
#[doc(alias = "RBX::HandlesBase::shouldRender2d(void)const")]
#[doc(alias = "__ZNK3RBX11HandlesBase14shouldRender2dEv")]
pub fn stub_0x3a8664() -> ! {
    todo!("0x3a8664 RBX::HandlesBase::shouldRender2d(void)const")
}

// 0x3a8674 — __ZThn32_N3RBX10ArcHandlesD1Ev
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "__ZThn32_N3RBX10ArcHandlesD1Ev")]
pub fn stub_0x3a8674() {
    // IDA 0x3a8674: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3a867c — __ZThn32_N3RBX10ArcHandlesD0Ev
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "__ZThn32_N3RBX10ArcHandlesD0Ev")]
pub fn stub_0x3a867c() {
    // IDA 0x3a867c: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a8720 — __ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3a8720() -> &'static str {
    // IDA 0x3a8720: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x3a8720 family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "ArcHandles"
}

// 0x3a8730 — __ZThn36_N3RBX10ArcHandlesD1Ev
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "__ZThn36_N3RBX10ArcHandlesD1Ev")]
pub fn stub_0x3a8730() {
    // IDA 0x3a8730: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x3a8738 — __ZThn36_N3RBX10ArcHandlesD0Ev
#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
#[doc(alias = "__ZThn36_N3RBX10ArcHandlesD0Ev")]
pub fn stub_0x3a8738() {
    // IDA 0x3a8738: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a87dc — __ZThn96_NK3RBX11HandlesBase14shouldRender2dEv
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::shouldRender2d(void)const")]
#[doc(alias = "__ZThn96_NK3RBX11HandlesBase14shouldRender2dEv")]
pub fn stub_0x3a87dc() {
    // IDA 0x3a87dc: non-virtual thunk to `RBX::HandlesBase::shouldRender2d(RBX::HandlesBase *this) { return (*(int (__fastcall **)(char *))(*((_DWORD *)this - 24) + 144))((char *)this - 96); } "` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a87ec — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3a87ec() {
    // IDA 0x3a87ec: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a87f0 — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3a87f0() {
    // IDA 0x3a87f0: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x3a888c — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x3a888c() -> ! {
    todo!("0x3a888c __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x3a8914 — __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x3a8914() -> ! {
    todo!("0x3a8914 __ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")
}

// 0x3a8a58 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x3a8a58() -> ! {
    todo!("0x3a8a58 boost::shared_ptr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")
}

// 0x3a8b0c — __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3a8b0c() -> ! {
    todo!("0x3a8b0c boost::shared_ptr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a8cbc — __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3a8cbc() -> ! {
    todo!("0x3a8cbc boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3a8dc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3a8dc4() {
    // IDA 0x3a8dc4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a8dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3a8dc8() {
    // IDA 0x3a8dc8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a8dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3a8dcc() -> ! {
    todo!("0x3a8dcc boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3a8dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a8dec() -> ! {
    todo!("0x3a8dec boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3a8e04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3a8e04() -> ! {
    todo!("0x3a8e04 boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3a8e08 — __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")]
pub fn stub_0x3a8e08() -> ! {
    todo!("0x3a8e08 __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")
}

// 0x3a8e0c — __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
pub fn stub_0x3a8e0c() -> ! {
    todo!("0x3a8e0c __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")
}

// 0x3a8eec — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x3a8eec() -> ! {
    todo!("0x3a8eec __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")
}

// 0x3a9130 — __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3a9130() -> ! {
    todo!("0x3a9130 __ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")
}

// 0x3a91a4 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_")]
pub fn stub_0x3a91a4() -> ! {
    todo!("0x3a91a4 boost::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")
}

// 0x3a9278 — __ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_")]
pub fn stub_0x3a9278() -> ! {
    todo!("0x3a9278 boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")
}

