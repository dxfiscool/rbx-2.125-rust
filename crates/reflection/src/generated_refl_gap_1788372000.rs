//! reflection — generated_refl_gap_1788372000 — 120 stubs EA-sorted asc 0x399304..0x3a2bac (reflection gap filler distinct not yet in crates/reflection/src — next 120 uncovered; RBX::Reflection filter yielded 0 remaining so global reflection-gap asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc not yet in crates/reflection/src — next 120 uncovered sorted asc (RBX::Reflection strict filter exhausted 0 remaining -> fallback to global reflection-gap)
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x399304 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
pub fn stub_0x399304() -> &'static str {
    // IDA 0x399304: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x399304 family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "AnimationTrack"
}

// 0x39932c — __ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv")]
pub fn stub_0x39932c() -> ! {
    todo!("0x39932c __ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv")
}

// 0x399330 — __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")]
pub fn stub_0x399330() -> ! {
    todo!("0x399330 __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")
}

// 0x399410 — __ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Animator>::shared_ptr<RBX::Animator>(boost::weak_ptr<RBX::Animator> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0x399410() -> ! {
    todo!("0x399410 boost::shared_ptr<RBX::Animator>::shared_ptr<RBX::Animator>(boost::weak_ptr<RBX::Animator> const&,boost::detail::sp_nothrow_tag)")
}

// 0x39948c — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x39948c() -> ! {
    todo!("0x39948c rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>> const&)")
}

// 0x399500 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x399500() {
    // IDA 0x399500: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39952c — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x39952c() {
    // IDA 0x39952c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x399600 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs")]
pub fn stub_0x399600() -> ! {
    todo!("0x399600 rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")
}

// 0x39961c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs")]
pub fn stub_0x39961c() {
    // IDA 0x39961c: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<` — this/arg-adjust + tail-call (this += 20) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x399638 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string> &,boost::_bi::list1<std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x399638() -> ! {
    todo!("0x399638 void boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string> &,boost::_bi::list1<std::string &> &,int)")
}

// 0x399758 — __ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>::operator()(RBX::AnimationTrack*,std::string)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss")]
pub fn stub_0x399758() -> ! {
    todo!("0x399758 boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>::operator()(RBX::AnimationTrack*,std::string)const")
}

// 0x39988c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")]
pub fn stub_0x39988c() {
    // IDA 0x39988c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3998b8 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")]
pub fn stub_0x3998b8() {
    // IDA 0x3998b8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39b100 — __GLOBAL__I_a_156
#[doc(alias = "global constructor keyed to_a_156")]
#[doc(alias = "__GLOBAL__I_a_156")]
pub fn stub_0x39b100() -> ! {
    todo!("0x39b100 global constructor keyed to _a_156")
}

// 0x39b490 — __ZN3RBX19AnimationTrackStateC1EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")]
#[doc(alias = "__ZN3RBX19AnimationTrackStateC1EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE")]
pub fn stub_0x39b490() -> ! {
    todo!("0x39b490 RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")
}

// 0x39b494 — __ZN3RBX19AnimationTrackStateC2EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")]
#[doc(alias = "__ZN3RBX19AnimationTrackStateC2EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE")]
pub fn stub_0x39b494() -> ! {
    todo!("0x39b494 RBX::AnimationTrackState::AnimationTrackState(boost::shared_ptr<RBX::KeyframeSequence const>,boost::weak_ptr<RBX::Animator const>)")
}

// 0x39b950 — __ZN3RBX19AnimationTrackState6onPlayEffff
#[doc(alias = "RBX::AnimationTrackState::onPlay(float,float,float,float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState6onPlayEffff")]
pub fn stub_0x39b950() -> ! {
    todo!("0x39b950 RBX::AnimationTrackState::onPlay(float,float,float,float)")
}

// 0x39b9ac — __ZN3RBX19AnimationTrackState6onStopEff
#[doc(alias = "RBX::AnimationTrackState::onStop(float,float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState6onStopEff")]
pub fn stub_0x39b9ac() -> ! {
    todo!("0x39b9ac RBX::AnimationTrackState::onStop(float,float)")
}

// 0x39b9f4 — __ZN3RBX19AnimationTrackState14onAdjustWeightEfff
#[doc(alias = "RBX::AnimationTrackState::onAdjustWeight(float,float,float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState14onAdjustWeightEfff")]
pub fn stub_0x39b9f4() -> ! {
    todo!("0x39b9f4 RBX::AnimationTrackState::onAdjustWeight(float,float,float)")
}

// 0x39ba40 — __ZN3RBX19AnimationTrackState13onAdjustSpeedEff
#[doc(alias = "RBX::AnimationTrackState::onAdjustSpeed(float,float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState13onAdjustSpeedEff")]
pub fn stub_0x39ba40() -> ! {
    todo!("0x39ba40 RBX::AnimationTrackState::onAdjustSpeed(float,float)")
}

// 0x39ba88 — __ZN3RBX19AnimationTrackState9isStoppedEd
#[doc(alias = "RBX::AnimationTrackState::isStopped(double)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState9isStoppedEd")]
pub fn stub_0x39ba88() -> ! {
    todo!("0x39ba88 RBX::AnimationTrackState::isStopped(double)")
}

// 0x39bb00 — __ZN3RBX19AnimationTrackState11getGameTimeEv
#[doc(alias = "RBX::AnimationTrackState::getGameTime(void)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState11getGameTimeEv")]
pub fn stub_0x39bb00() -> ! {
    todo!("0x39bb00 RBX::AnimationTrackState::getGameTime(void)")
}

// 0x39bc00 — __ZN3RBX19AnimationTrackState15getWeightAtTimeEd
#[doc(alias = "RBX::AnimationTrackState::getWeightAtTime(double)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState15getWeightAtTimeEd")]
pub fn stub_0x39bc00() -> ! {
    todo!("0x39bc00 RBX::AnimationTrackState::getWeightAtTime(double)")
}

// 0x39bc5c — __ZN3RBX19AnimationTrackState4playEfff
#[doc(alias = "RBX::AnimationTrackState::play(float,float,float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState4playEfff")]
pub fn stub_0x39bc5c() -> ! {
    todo!("0x39bc5c RBX::AnimationTrackState::play(float,float,float)")
}

// 0x39bcbc — __ZN3RBX19AnimationTrackState4stopEf
#[doc(alias = "RBX::AnimationTrackState::stop(float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState4stopEf")]
pub fn stub_0x39bcbc() -> ! {
    todo!("0x39bcbc RBX::AnimationTrackState::stop(float)")
}

// 0x39bd0c — __ZN3RBX19AnimationTrackState12adjustWeightEff
#[doc(alias = "RBX::AnimationTrackState::adjustWeight(float,float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState12adjustWeightEff")]
pub fn stub_0x39bd0c() -> ! {
    todo!("0x39bd0c RBX::AnimationTrackState::adjustWeight(float,float)")
}

// 0x39bd64 — __ZN3RBX19AnimationTrackState11adjustSpeedEf
#[doc(alias = "RBX::AnimationTrackState::adjustSpeed(float)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState11adjustSpeedEf")]
pub fn stub_0x39bd64() -> ! {
    todo!("0x39bd64 RBX::AnimationTrackState::adjustSpeed(float)")
}

// 0x39bdb4 — __ZN3RBX19AnimationTrackState28triggerKeyframeReachedSignalERKN5boost10shared_ptrINS_8InstanceEEEdd
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::AnimationTrackState::triggerKeyframeReachedSignal(boost::shared_ptr<RBX::Instance> const&,double,double)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState28triggerKeyframeReachedSignalERKN5boost10shared_ptrINS_8InstanceEEEdd")]
pub fn stub_0x39bdb4() -> ! {
    todo!("0x39bdb4 RBX::AnimationTrackState::triggerKeyframeReachedSignal(boost::shared_ptr<RBX::Instance> const&,double,double)")
}

// 0x39bf44 — __ZN3RBX19AnimationTrackState4stepERSt6vectorINS_15PoseAccumulatorESaIS2_EEd
#[doc(alias = "RBX::AnimationTrackState::step(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double)")]
#[doc(alias = "__ZN3RBX19AnimationTrackState4stepERSt6vectorINS_15PoseAccumulatorESaIS2_EEd")]
pub fn stub_0x39bf44() -> ! {
    todo!("0x39bf44 RBX::AnimationTrackState::step(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> &,double)")
}

// 0x39c370 — __ZN3RBX19AnimationTrackStateD1Ev
#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZN3RBX19AnimationTrackStateD1Ev")]
pub fn stub_0x39c370() {
    // IDA 0x39c370: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39c374 — __ZN3RBX19AnimationTrackStateD0Ev
#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZN3RBX19AnimationTrackStateD0Ev")]
pub fn stub_0x39c374() {
    // IDA 0x39c374: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39c414 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
pub fn stub_0x39c414() -> ! {
    todo!("0x39c414 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")
}

// 0x39c43c — __ZThn32_N3RBX19AnimationTrackStateD1Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZThn32_N3RBX19AnimationTrackStateD1Ev")]
pub fn stub_0x39c43c() {
    // IDA 0x39c43c: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x39c444 — __ZThn32_N3RBX19AnimationTrackStateD0Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZThn32_N3RBX19AnimationTrackStateD0Ev")]
pub fn stub_0x39c444() {
    // IDA 0x39c444: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39c44c — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEE12getClassNameEv")]
pub fn stub_0x39c44c() -> &'static str {
    // IDA 0x39c44c: __ZThn getClassName — `Creator = static_getCreator(); return Creator::getClassName_shim(Creator)` (decompiled 0x39c44c family; e.g. 0x28e128). The Creator name is the class name. Rust: no vtable/Creator needed.
    "AnimationTrackState"
}

// 0x39c474 — __ZThn36_N3RBX19AnimationTrackStateD1Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZThn36_N3RBX19AnimationTrackStateD1Ev")]
pub fn stub_0x39c474() {
    // IDA 0x39c474: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x39c47c — __ZThn36_N3RBX19AnimationTrackStateD0Ev
#[doc(alias = "non-virtual thunk toRBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZThn36_N3RBX19AnimationTrackStateD0Ev")]
pub fn stub_0x39c47c() {
    // IDA 0x39c47c: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39c484 — __ZN3RBX19AnimationTrackStateD2Ev
#[doc(alias = "RBX::AnimationTrackState::~AnimationTrackState()")]
#[doc(alias = "__ZN3RBX19AnimationTrackStateD2Ev")]
pub fn stub_0x39c484() {
    // IDA 0x39c484: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x39c640 — __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")]
pub fn stub_0x39c640() -> ! {
    todo!("0x39c640 __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")
}

// 0x39c644 — __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")]
pub fn stub_0x39c644() -> ! {
    todo!("0x39c644 __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")
}

// 0x39cb28 — __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff")]
pub fn stub_0x39cb28() -> ! {
    todo!("0x39cb28 rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")
}

// 0x39cc88 — __ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
pub fn stub_0x39cc88() -> ! {
    todo!("0x39cc88 rbx::signals::signal<void ()(float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")
}

// 0x39cde8 — __ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception")]
pub fn stub_0x39cde8() -> ! {
    todo!("0x39cde8 rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")
}

// 0x39ce10 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_")]
pub fn stub_0x39ce10() -> ! {
    todo!("0x39ce10 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")
}

// 0x39ce34 — __ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv")]
pub fn stub_0x39ce34() -> ! {
    todo!("0x39ce34 rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")
}

// 0x39ce38 — __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv")]
pub fn stub_0x39ce38() -> ! {
    todo!("0x39ce38 rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")
}

// 0x39d260 — __ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff")]
pub fn stub_0x39d260() -> ! {
    todo!("0x39d260 rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")
}

// 0x39d3dc — __ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
pub fn stub_0x39d3dc() -> ! {
    todo!("0x39d3dc rbx::signals::signal<void ()(float,float,float,float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")
}

// 0x39d53c — __ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception")]
pub fn stub_0x39d53c() -> ! {
    todo!("0x39d53c rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")
}

// 0x39d564 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_")]
pub fn stub_0x39d564() -> ! {
    todo!("0x39d564 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")
}

// 0x39d588 — __ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv")]
pub fn stub_0x39d588() -> ! {
    todo!("0x39d588 rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")
}

// 0x39d58c — __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv")]
pub fn stub_0x39d58c() -> ! {
    todo!("0x39d58c rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")
}

// 0x39d684 — __ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Animator const>::shared_ptr<RBX::Animator const>(boost::weak_ptr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0x39d684() -> ! {
    todo!("0x39d684 boost::shared_ptr<RBX::Animator const>::shared_ptr<RBX::Animator const>(boost::weak_ptr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")
}

// 0x39d700 — __ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x39d700() -> ! {
    todo!("0x39d700 rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0x39d774 — __ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE")]
pub fn stub_0x39d774() -> ! {
    todo!("0x39d774 rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")
}

// 0x39d980 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_")]
pub fn stub_0x39d980() -> ! {
    todo!("0x39d980 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")
}

// 0x39d9a4 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev")]
pub fn stub_0x39d9a4() {
    // IDA 0x39d9a4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39d9d0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev")]
pub fn stub_0x39d9d0() {
    // IDA 0x39d9d0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39daa4 — __ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv")]
pub fn stub_0x39daa4() -> ! {
    todo!("0x39daa4 rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")
}

// 0x39dbb4 — __ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv")]
pub fn stub_0x39dbb4() -> ! {
    todo!("0x39dbb4 rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")
}

// 0x39dbc0 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff")]
pub fn stub_0x39dbc0() -> ! {
    todo!("0x39dbc0 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")
}

// 0x39dbec — __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff")]
pub fn stub_0x39dbec() {
    // IDA 0x39dbec: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,b` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x39dc18 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x39dc18() -> ! {
    todo!("0x39dc18 void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")
}

// 0x39dc54 — __ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE6removeEPNS3_4slotE")]
pub fn stub_0x39dc54() -> ! {
    todo!("0x39dc54 rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")
}

// 0x39dd44 — __ZN3rbx7signals6signalIFvfffEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x39dd44() -> ! {
    todo!("0x39dd44 rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_init_mutex(void)")
}

// 0x39dd48 — __ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x39dd48() -> ! {
    todo!("0x39dd48 rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")
}

// 0x39de38 — __ZN3rbx7signals6signalIFvfffEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4slotD1Ev")]
pub fn stub_0x39de38() {
    // IDA 0x39de38: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39de64 — __ZN3rbx7signals6signalIFvfffEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4slotD0Ev")]
pub fn stub_0x39de64() {
    // IDA 0x39de64: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39df38 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev")]
pub fn stub_0x39df38() {
    // IDA 0x39df38: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39df64 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev")]
pub fn stub_0x39df64() {
    // IDA 0x39df64: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39e038 — __ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x39e038() -> ! {
    todo!("0x39e038 rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x39e0ac — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev")]
pub fn stub_0x39e0ac() {
    // IDA 0x39e0ac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39e0d8 — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev")]
pub fn stub_0x39e0d8() {
    // IDA 0x39e0d8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39e1ac — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff")]
pub fn stub_0x39e1ac() -> ! {
    todo!("0x39e1ac rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")
}

// 0x39e1d4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff")]
pub fn stub_0x39e1d4() {
    // IDA 0x39e1d4: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::l` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x39e1fc — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x39e1fc() -> ! {
    todo!("0x39e1fc void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")
}

// 0x39e228 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev")]
pub fn stub_0x39e228() {
    // IDA 0x39e228: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39e254 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev")]
pub fn stub_0x39e254() {
    // IDA 0x39e254: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39e328 — __ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x39e328() -> ! {
    todo!("0x39e328 rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")
}

// 0x39e39c — __ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE6insertEPNS3_4slotE")]
pub fn stub_0x39e39c() -> ! {
    todo!("0x39e39c rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")
}

// 0x39e5a8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_")]
pub fn stub_0x39e5a8() -> ! {
    todo!("0x39e5a8 boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")
}

// 0x39e5cc — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED1Ev")]
pub fn stub_0x39e5cc() {
    // IDA 0x39e5cc: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39e5f8 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED0Ev")]
pub fn stub_0x39e5f8() {
    // IDA 0x39e5f8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39e6cc — __ZN3rbx7signals6signalIFvffffEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4slot10disconnectEv")]
pub fn stub_0x39e6cc() -> ! {
    todo!("0x39e6cc rbx::signals::signal<void ()(float,float,float,float)>::slot::disconnect(void)")
}

// 0x39e7dc — __ZNK3rbx7signals6signalIFvffffEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvffffEE4slot9connectedEv")]
pub fn stub_0x39e7dc() -> ! {
    todo!("0x39e7dc rbx::signals::signal<void ()(float,float,float,float)>::slot::connected(void)const")
}

// 0x39e7e8 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff")]
pub fn stub_0x39e7e8() -> ! {
    todo!("0x39e7e8 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}

// 0x39e824 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff")]
pub fn stub_0x39e824() {
    // IDA 0x39e824: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,fl` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x39e860 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x39e860() -> ! {
    todo!("0x39e860 void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")
}

// 0x39e8b0 — __ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE6removeEPNS3_4slotE")]
pub fn stub_0x39e8b0() -> ! {
    todo!("0x39e8b0 rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")
}

// 0x39e9a0 — __ZN3rbx7signals6signalIFvffffEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x39e9a0() -> ! {
    todo!("0x39e9a0 rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_init_mutex(void)")
}

// 0x39e9a4 — __ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x39e9a4() -> ! {
    todo!("0x39e9a4 rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")
}

// 0x39ea94 — __ZN3rbx7signals6signalIFvffffEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4slotD1Ev")]
pub fn stub_0x39ea94() {
    // IDA 0x39ea94: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39eac0 — __ZN3rbx7signals6signalIFvffffEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4slotD0Ev")]
pub fn stub_0x39eac0() {
    // IDA 0x39eac0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39eb94 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED1Ev")]
pub fn stub_0x39eb94() {
    // IDA 0x39eb94: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x39ebc0 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED0Ev")]
pub fn stub_0x39ebc0() {
    // IDA 0x39ebc0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x39ec94 — __ZN3rbx13remote_signalIFvfffEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEEC2Ev")]
pub fn stub_0x39ec94() -> ! {
    todo!("0x39ec94 rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")
}

// 0x39edf0 — __ZN3rbx7signals6signalIFvfffEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13disconnectAllEv")]
pub fn stub_0x39edf0() -> ! {
    todo!("0x39edf0 rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")
}

// 0x39ef68 — __ZN3rbx13remote_signalIFvffEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(float,float)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvffEEC2Ev")]
pub fn stub_0x39ef68() -> ! {
    todo!("0x39ef68 rbx::remote_signal<void ()(float,float)>::remote_signal(void)")
}

// 0x39f0c4 — __ZN3rbx13remote_signalIFvffffEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEEC2Ev")]
pub fn stub_0x39f0c4() -> ! {
    todo!("0x39f0c4 rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")
}

// 0x39f220 — __ZN3rbx7signals6signalIFvffffEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13disconnectAllEv")]
pub fn stub_0x39f220() -> ! {
    todo!("0x39f220 rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")
}

// 0x3a017c — __ZN5boost9function3IvfffE5clearEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function3<void,float,float,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvfffE5clearEv")]
pub fn stub_0x3a017c() -> ! {
    todo!("0x3a017c boost::function3<void,float,float,float>::clear(void)")
}

// 0x3a08d0 — __ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
pub fn stub_0x3a08d0() -> ! {
    todo!("0x3a08d0 rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")
}

// 0x3a09c4 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")]
pub fn stub_0x3a09c4() -> ! {
    todo!("0x3a09c4 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")
}

// 0x3a0ac0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
pub fn stub_0x3a0ac0() {
    // IDA 0x3a0ac0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a0bd0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
pub fn stub_0x3a0bd0() {
    // IDA 0x3a0bd0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a0d00 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff")]
pub fn stub_0x3a0d00() -> ! {
    todo!("0x3a0d00 rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")
}

// 0x3a0d08 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff")]
pub fn stub_0x3a0d08() {
    // IDA 0x3a0d08: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call( i` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a0d10 — __ZNK5boost9function3IvfffEclEfff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function3<void,float,float,float>::operator()(float,float,float)const")]
#[doc(alias = "__ZNK5boost9function3IvfffEclEfff")]
pub fn stub_0x3a0d10() -> ! {
    todo!("0x3a0d10 boost::function3<void,float,float,float>::operator()(float,float,float)const")
}

// 0x3a0dec — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")]
pub fn stub_0x3a0dec() {
    // IDA 0x3a0dec: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a0efc — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")]
pub fn stub_0x3a0efc() {
    // IDA 0x3a0efc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a102c — __ZN5boost9function3IvfffE13assign_to_ownERKS1_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")]
#[doc(alias = "__ZN5boost9function3IvfffE13assign_to_ownERKS1_")]
pub fn stub_0x3a102c() -> ! {
    todo!("0x3a102c boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")
}

// 0x3a1f18 — __ZN5boost9function4IvffffE5clearEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function4<void,float,float,float,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function4IvffffE5clearEv")]
pub fn stub_0x3a1f18() -> ! {
    todo!("0x3a1f18 boost::function4<void,float,float,float,float>::clear(void)")
}

// 0x3a2684 — __ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
pub fn stub_0x3a2684() -> ! {
    todo!("0x3a2684 rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")
}

// 0x3a2778 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_")]
pub fn stub_0x3a2778() -> ! {
    todo!("0x3a2778 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")
}

// 0x3a2874 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
pub fn stub_0x3a2874() {
    // IDA 0x3a2874: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3a2984 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
pub fn stub_0x3a2984() {
    // IDA 0x3a2984: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3a2ab4 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff")]
pub fn stub_0x3a2ab4() -> ! {
    todo!("0x3a2ab4 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}

// 0x3a2abc — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff")]
pub fn stub_0x3a2abc() {
    // IDA 0x3a2abc: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>` — this/arg-adjust + tail-call (this += 12) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x3a2ac4 — __ZNK5boost9function4IvffffEclEffff
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
#[doc(alias = "__ZNK5boost9function4IvffffEclEffff")]
pub fn stub_0x3a2ac4() -> ! {
    todo!("0x3a2ac4 boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")
}

// 0x3a2bac — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev")]
pub fn stub_0x3a2bac() {
    // IDA 0x3a2bac: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}
