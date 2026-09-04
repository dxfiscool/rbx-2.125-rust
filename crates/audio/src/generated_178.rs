//! audio generated_178 — global gap filler batch 178 — next 120 EA not yet in crates/audio/src, EA-sorted asc
//! Filter: FMOD|Audio|Sound (2541 distinct, 0 remaining FMOD) -> global gap filler EA-sorted asc next 120 not yet in audio crate
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x3a2984..0x3a9bb8 | audio 23262 -> 23382 distinct
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x3a2984 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
pub fn stub_0x3a2984() {
    // IDA 0x3a2984: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a2ab4 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff")]
pub fn stub_0x3a2ab4() -> ! {
    todo!("0x3a2ab4 rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}

// 0x3a2abc — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff")]
pub fn stub_0x3a2abc() -> ! {
    todo!("0x3a2abc non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")
}

// 0x3a2ac4 — __ZNK5boost9function4IvffffEclEffff
// type: void __fastcall(_DWORD *, int, int, int, float)
#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
#[doc(alias = "__ZNK5boost9function4IvffffEclEffff")]
pub fn stub_0x3a2ac4() -> ! {
    todo!("0x3a2ac4 boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")
}

// 0x3a2bac — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev")]
pub fn stub_0x3a2bac() {
    // IDA 0x3a2bac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a2cbc — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev")]
pub fn stub_0x3a2cbc() {
    // IDA 0x3a2cbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a2dec — __ZN5boost9function4IvffffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
#[doc(alias = "__ZN5boost9function4IvffffE13assign_to_ownERKS1_")]
pub fn stub_0x3a2dec() -> ! {
    todo!("0x3a2dec boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")
}

// 0x3a2e1c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x3a2e1c() -> ! {
    todo!("0x3a2e1c RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x3a30e8 — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_0x3a30e8() {
    // IDA 0x3a30e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a310c — __ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_19AnimationTrackStateEFvffffEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_0x3a310c() {
    // IDA 0x3a310c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a31c0 — __ZN3rbx13remote_signalIFvffffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvffffEED2Ev")]
pub fn stub_0x3a31c0() {
    // IDA 0x3a31c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a330c — __ZN3rbx13remote_signalIFvfffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvfffEED2Ev")]
pub fn stub_0x3a330c() {
    // IDA 0x3a330c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a3458 — __ZN3rbx13remote_signalIFvffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvffEED2Ev")]
pub fn stub_0x3a3458() {
    // IDA 0x3a3458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a395c — __ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, __guard *, const shared_count *, int)
#[doc(alias = "RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Animator13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x3a395c() {
    // IDA 0x3a395c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a3d44 — __ZN3RBX8AnimatorC1EPNS_8InstanceE
// type: int __fastcall(RBX::Animator *this, RBX::Instance *)
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8AnimatorC1EPNS_8InstanceE")]
pub fn stub_0x3a3d44() -> ! {
    todo!("0x3a3d44 RBX::Animator::Animator(RBX::Instance *)")
}

// 0x3a3d48 — __ZN3RBX8AnimatorC2EPNS_8InstanceE
// type: RBX::Instance *__fastcall(RBX::Animator *this, RBX::Instance *)
#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8AnimatorC2EPNS_8InstanceE")]
pub fn stub_0x3a3d48() -> ! {
    todo!("0x3a3d48 RBX::Animator::Animator(RBX::Instance *)")
}

// 0x3a4364 — __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
// type: int *__fastcall(int, int *, int, int)
#[doc(alias = "RBX::Animator::reloadAnimation(rbx_core::SharedPtr<RBX::AnimationTrackState>)")]
#[doc(alias = "__ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE")]
pub fn stub_0x3a4364() {
    // IDA 0x3a4364: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a4474 — __ZN3RBX8Animator23getReplicatingContainerEv
// type: void __fastcall(RBX::Animator *this, int)
#[doc(alias = "RBX::Animator::getReplicatingContainer(void)")]
#[doc(alias = "__ZN3RBX8Animator23getReplicatingContainerEv")]
pub fn stub_0x3a4474() -> ! {
    todo!("0x3a4474 RBX::Animator::getReplicatingContainer(void)")
}

// 0x3a4598 — __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
// type: void __fastcall(int, int *, double, int, _DWORD *)
#[doc(alias = "RBX::Animator::onTrackStepped(rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")]
#[doc(alias = "__ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE")]
pub fn stub_0x3a4598() {
    // IDA 0x3a4598: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a46a0 — __ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE
// type: void *__fastcall(int, const void **)
#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Animator25appendAnimatableJointsRecEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x3a46a0() {
    // IDA 0x3a46a0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a4ea0 — __ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Animator *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Animator11askAddChildEPKNS_8InstanceE")]
pub fn stub_0x3a4ea0() -> ! {
    todo!("0x3a4ea0 RBX::Animator::askAddChild(RBX::Instance const*)const")
}

// 0x3a4edc — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED1Ev")]
pub fn stub_0x3a4edc() {
    // IDA 0x3a4edc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a4fe8 — __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0x3a4fe8() {
    // IDA 0x3a4fe8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a5158 — __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
// type: _DWORD **__fastcall(_DWORD **)
#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev")]
pub fn stub_0x3a5158() {
    // IDA 0x3a5158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a5218 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// type: void __fastcall(int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, char, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19AnimationTrackStateEN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_")]
pub fn stub_0x3a5218() {
    // IDA 0x3a5218: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a5380 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_
// type: void __fastcall(int, const shared_count *, int *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, void *, char, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_14AnimationTrackEN5boost10shared_ptrINS_19AnimationTrackStateEEENS6_INS_8AnimatorEEEEENS6_IT_EET0_T1_")]
pub fn stub_0x3a5380() {
    // IDA 0x3a5380: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a55fc — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_8AnimatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0x3a55fc() {
    // IDA 0x3a55fc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x3a5704 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_
// type: int __fastcall(unsigned __int64 *, _DWORD *, _DWORD *, unsigned int, boost::detail::sp_counted_base *, int, int, int, unsigned __int64)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")]
#[doc(alias = "__ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf4IvNS3_8AnimatorES5_dNS3_16KeyframeSequence8PriorityEPSt6vectorINS3_15PoseAccumulatorESaISF_EEEENS7_5list5INS7_5valueIPSB_EENS1_3argILi1EEENSL_IdEENSL_ISD_EENSL_ISI_EEEEEEET0_T_SW_SV_")]
pub fn stub_0x3a5704() {
    // IDA 0x3a5704: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

// 0x3a5880 — __ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Animator *this, const RBX::Instance *)
#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Animator12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x3a5880() -> ! {
    todo!("0x3a5880 RBX::Animator::askSetParent(RBX::Instance const*)const")
}

// 0x3a5acc — __ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, int, int **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEENS2_IdEENS2_INS3_16KeyframeSequence8PriorityEEENS2_IPSt6vectorINS3_15PoseAccumulatorESaISE_EEEEEclINS_4_mfi3mf4IvS4_NS_10shared_ptrINS3_19AnimationTrackStateEEEdSB_SH_EENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x3a5acc() {
    // IDA 0x3a5acc: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0x3a5bb4 — __ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_
// type: void __fastcall(int *, int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf4IvN3RBX8AnimatorENS_10shared_ptrINS2_19AnimationTrackStateEEEdNS2_16KeyframeSequence8PriorityEPSt6vectorINS2_15PoseAccumulatorESaISA_EEEclEPS3_S6_dS8_SD_")]
pub fn stub_0x3a5bb4() {
    // IDA 0x3a5bb4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0x3a5cb4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// type: int __fastcall(int, boost::detail::sp_counted_base **this)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")]
pub fn stub_0x3a5cb4() {
    // IDA 0x3a5cb4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a5cd4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX8AnimatorEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x3a5cd4() {
    // IDA 0x3a5cd4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

// 0x3a5dac — __ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX8AnimatorENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")]
pub fn stub_0x3a5dac() {
    // IDA 0x3a5dac: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0x3a61fc — __ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AnimationTrackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3a61fc() {
    // IDA 0x3a61fc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a62c4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14AnimationTrackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrack,RBX::AnimationTrack>(rbx_core::SharedPtr<RBX::AnimationTrack> const*,RBX::AnimationTrack *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14AnimationTrackES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x3a62c4() {
    // IDA 0x3a62c4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x3a63ac — __ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX14AnimationTrackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3a63ac() {
    // IDA 0x3a63ac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a64b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3a64b4() {
    // IDA 0x3a64b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a64b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3a64b8() {
    // IDA 0x3a64b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a64bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3a64bc() {
    // IDA 0x3a64bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a64dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a64dc() {
    // IDA 0x3a64dc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a64f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AnimationTrackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3a64f4() {
    // IDA 0x3a64f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a64f8 — __ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19AnimationTrackStateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3a64f8() {
    // IDA 0x3a64f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a65c0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19AnimationTrackStateES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrackState,RBX::AnimationTrackState>(rbx_core::SharedPtr<RBX::AnimationTrackState> const*,RBX::AnimationTrackState *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19AnimationTrackStateES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x3a65c0() {
    // IDA 0x3a65c0: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x3a66a8 — __ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX19AnimationTrackStateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3a66a8() {
    // IDA 0x3a66a8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a67b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3a67b0() {
    // IDA 0x3a67b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a67b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3a67b4() {
    // IDA 0x3a67b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a67b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3a67b8() {
    // IDA 0x3a67b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a67d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a67d8() {
    // IDA 0x3a67d8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a67f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19AnimationTrackStateENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3a67f0() {
    // IDA 0x3a67f0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a67f4 — __ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_
// type: shared_count *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_create_node(rbx_core::SharedPtr<RBX::AnimationTrackState> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE14_M_create_nodeERKS4_")]
pub fn stub_0x3a67f4() {
    // IDA 0x3a67f4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a68d8 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12PartInstanceEEC2IS2_EEPT_")]
pub fn stub_0x3a68d8() {
    // IDA 0x3a68d8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a69c4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12PartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PartInstance,RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const*,RBX::PartInstance *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12PartInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x3a69c4() {
    // IDA 0x3a69c4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x3a6aac — __ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX12PartInstanceEEEPT_")]
pub fn stub_0x3a6aac() {
    // IDA 0x3a6aac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a6ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED1Ev")]
pub fn stub_0x3a6ba4() {
    // IDA 0x3a6ba4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEED0Ev")]
pub fn stub_0x3a6ba8() {
    // IDA 0x3a6ba8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6bac — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE7disposeEv")]
pub fn stub_0x3a6bac() {
    // IDA 0x3a6bac: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a6bbc — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a6bbc() {
    // IDA 0x3a6bbc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a6bc0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12PartInstanceEE19get_untyped_deleterEv")]
pub fn stub_0x3a6bc0() {
    // IDA 0x3a6bc0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a6bc4 — __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3a6bc4() {
    // IDA 0x3a6bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6bc8 — __ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3a6bc8() {
    // IDA 0x3a6bc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6c68 — __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3a6c68() {
    // IDA 0x3a6c68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6c70 — __ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3a6c70() {
    // IDA 0x3a6c70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6d14 — __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3a6d14() {
    // IDA 0x3a6d14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6d1c — __ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3a6d1c() {
    // IDA 0x3a6d1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a6dc0 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EEC2EMS2_FS6_S6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x3a6dc0() {
    // IDA 0x3a6dc0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a6f58 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x3a6f58() {
    // IDA 0x3a6f58: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a6f88 — __ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EED0Ev")]
pub fn stub_0x3a6f88() {
    // IDA 0x3a6f88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a70a4 — __ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8AnimatorEFN5boost10shared_ptrINS_8InstanceEEES6_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x3a70a4() {
    // IDA 0x3a70a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a718c — __ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_
// type: void __fastcall(int, char *, int, _DWORD *, const shared_count *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_8AnimatorEMS2_FN5boost10shared_ptrINS_8InstanceEEES6_ES6_S6_E4callEPS2_S8_RNS0_7VariantERKS6_")]
pub fn stub_0x3a718c() {
    // IDA 0x3a718c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a72b8 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX19AnimationTrackStateEEESaIS4_EE8_M_clearEv")]
pub fn stub_0x3a72b8() {
    // IDA 0x3a72b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a7b58 — __ZN3RBX10ArcHandles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::ArcHandles *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX10ArcHandles17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0x3a7b58() -> ! {
    todo!("0x3a7b58 RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3a7efc — __ZN3RBX10Reflection14PropDescriptorINS_10ArcHandlesENS_4AxesEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_10ArcHandlesENS_4AxesEED1Ev")]
pub fn stub_0x3a7efc() {
    // IDA 0x3a7efc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a7f20 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEEN3rbx13remote_signalIS6_EEED1Ev")]
pub fn stub_0x3a7f20() {
    // IDA 0x3a7f20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a7f44 — __ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEN3rbx13remote_signalIS6_EEED1Ev")]
pub fn stub_0x3a7f44() {
    // IDA 0x3a7f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb")]
pub fn stub_0x3a7f68() -> ! {
    todo!("0x3a7f68 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")
}

// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb")]
pub fn stub_0x3a80c8() -> ! {
    todo!("0x3a80c8 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")
}

// 0x3a8228 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0x3a8228() -> ! {
    todo!("0x3a8228 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3a8288 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0x3a8288() -> ! {
    todo!("0x3a8288 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3a82e8 — __ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff")]
pub fn stub_0x3a82e8() -> ! {
    todo!("0x3a82e8 rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")
}

// 0x3a8440 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_")]
pub fn stub_0x3a8440() -> ! {
    todo!("0x3a8440 rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")
}

// 0x3a8584 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEE5resetIS3_EEvPT_")]
pub fn stub_0x3a8584() {
    // IDA 0x3a8584: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a8664 — __ZNK3RBX11HandlesBase14shouldRender2dEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "RBX::HandlesBase::shouldRender2d(void)const")]
#[doc(alias = "__ZNK3RBX11HandlesBase14shouldRender2dEv")]
pub fn stub_0x3a8664() -> ! {
    todo!("0x3a8664 RBX::HandlesBase::shouldRender2d(void)const")
}

// 0x3a87dc — __ZThn96_NK3RBX11HandlesBase14shouldRender2dEv
// type: int __fastcall(RBX::HandlesBase *this)
#[doc(alias = "non-virtual thunk toRBX::HandlesBase::shouldRender2d(void)const")]
#[doc(alias = "__ZThn96_NK3RBX11HandlesBase14shouldRender2dEv")]
pub fn stub_0x3a87dc() -> ! {
    todo!("0x3a87dc non-virtual thunk toRBX::HandlesBase::shouldRender2d(void)const")
}

// 0x3a8a58 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_10ArcHandlesEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x3a8a58() {
    // IDA 0x3a8a58: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a8b0c — __ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ArcHandlesEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3a8b0c() {
    // IDA 0x3a8b0c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a8bd4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ArcHandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ArcHandles,RBX::ArcHandles>(rbx_core::SharedPtr<RBX::ArcHandles> const*,RBX::ArcHandles *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ArcHandlesES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x3a8bd4() {
    // IDA 0x3a8bd4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x3a8cbc — __ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ArcHandlesENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x3a8cbc() {
    // IDA 0x3a8cbc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a8dc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x3a8dc4() {
    // IDA 0x3a8dc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a8dc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x3a8dc8() {
    // IDA 0x3a8dc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a8dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x3a8dcc() {
    // IDA 0x3a8dcc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a8dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a8dec() {
    // IDA 0x3a8dec: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a8e04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ArcHandlesENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x3a8e04() {
    // IDA 0x3a8e04: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a91a4 — __ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11HandlesBase20MouseDownCaptureInfoEEC2IS3_EEPT_")]
pub fn stub_0x3a91a4() {
    // IDA 0x3a91a4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a9278 — __ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX11HandlesBase20MouseDownCaptureInfoEEEPT_")]
pub fn stub_0x3a9278() {
    // IDA 0x3a9278: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a9364 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED1Ev")]
pub fn stub_0x3a9364() {
    // IDA 0x3a9364: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a9368 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEED0Ev")]
pub fn stub_0x3a9368() {
    // IDA 0x3a9368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a936c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE7disposeEv")]
pub fn stub_0x3a936c() {
    // IDA 0x3a936c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a9378 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE11get_deleterERKSt9type_info")]
pub fn stub_0x3a9378() {
    // IDA 0x3a9378: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a937c — __ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX11HandlesBase20MouseDownCaptureInfoEE19get_untyped_deleterEv")]
pub fn stub_0x3a937c() {
    // IDA 0x3a937c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x3a9380 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_0x3a9380() {
    // IDA 0x3a9380: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a94e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception")]
pub fn stub_0x3a94e0() -> ! {
    todo!("0x3a94e0 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")
}

// 0x3a9508 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSERKSA_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSERKSA_")]
pub fn stub_0x3a9508() {
    // IDA 0x3a9508: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a952c — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE22safe_static_init_mutexEv")]
pub fn stub_0x3a952c() -> ! {
    todo!("0x3a952c rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")
}

// 0x3a9530 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE24safe_static_do_get_mutexEv")]
pub fn stub_0x3a9530() -> ! {
    todo!("0x3a9530 rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_do_get_mutex(void)")
}

// 0x3a9628 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_0x3a9628() {
    // IDA 0x3a9628: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a9788 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE8on_errorERSt9exception")]
pub fn stub_0x3a9788() -> ! {
    todo!("0x3a9788 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::on_error(std::exception &)")
}

// 0x3a97b0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSERKSA_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE4slotEEaSERKSA_")]
pub fn stub_0x3a97b0() {
    // IDA 0x3a97b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x3a97d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE22safe_static_init_mutexEv")]
pub fn stub_0x3a97d4() -> ! {
    todo!("0x3a97d4 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_init_mutex(void)")
}

// 0x3a97d8 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE24safe_static_do_get_mutexEv")]
pub fn stub_0x3a97d8() -> ! {
    todo!("0x3a97d8 rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_do_get_mutex(void)")
}

// 0x3a98d0 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3a98d0() -> ! {
    todo!("0x3a98d0 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")
}

// 0x3a9944 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv")]
pub fn stub_0x3a9944() -> ! {
    todo!("0x3a9944 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")
}

// 0x3a9990 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED1Ev")]
pub fn stub_0x3a9990() {
    // IDA 0x3a9990: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a99bc — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEED0Ev")]
pub fn stub_0x3a99bc() {
    // IDA 0x3a99bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a9a90 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3a9a90() -> ! {
    todo!("0x3a9a90 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9a98 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x3a9a98() -> ! {
    todo!("0x3a9a98 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")
}

// 0x3a9aa0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_0x3a9aa0() -> ! {
    todo!("0x3a9aa0 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")
}

// 0x3a9ab8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x3a9ab8() {
    // IDA 0x3a9ab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a9ae4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS7_5list1INS7_5valueIPSI_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x3a9ae4() {
    // IDA 0x3a9ae4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x3a9bb8 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x3a9bb8() -> ! {
    todo!("0x3a9bb8 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")
}
