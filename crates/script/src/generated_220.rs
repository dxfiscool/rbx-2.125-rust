// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3a435c..0x3a8664 | script 22502->22602 distinct (filler 0x3a435c asc, not-in-script 63043->62943)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator() [0x3a435c]")]
pub fn stub_0x3a435c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Animator::reloadAnimation(rbx_core::SharedPtr<RBX::AnimationTrackState>)")]
pub fn stub_0x3a4364() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Animator::getGameTime(void)const")]
pub fn stub_0x3a439c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Animator getter.
cell.get()
}

#[doc(alias = "RBX::Animator::getReplicatingContainer(void)")]
pub fn stub_0x3a4474(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Animator getter.
cell.get()
}

#[doc(alias = "RBX::Animator::onTrackStepped(rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")]
pub fn stub_0x3a4598() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Animator::appendAnimatableJointsRec(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3a46a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Animator::calcAnimatableJoints(void)")]
pub fn stub_0x3a46e8(handle: &crate::slot::InstanceHandle) {
// RBX::Animator::calcAnimatableJoints() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Animator::onStepped(RBX::Stepped const&)")]
pub fn stub_0x3a4870(handle: &crate::slot::InstanceHandle) {
// RBX::Animator::onStepped(RBX::Stepped const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Animator::onStepped(RBX::Stepped const&)")]
pub fn stub_0x3a4e98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Animator::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x3a4ea0(handle: &crate::slot::InstanceHandle) {
// RBX::Animator::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_0x3a4edc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
pub fn stub_0x3a4fe8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Animator")
}

#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
pub fn stub_0x3a5158(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrackState,rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::SharedPtr<RBX::Animator>)")]
pub fn stub_0x3a5218() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack> RBX::Creatable<RBX::Instance>::create<RBX::AnimationTrack,rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>>(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::SharedPtr<RBX::Animator>)")]
pub fn stub_0x3a5380() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrack")
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::reserve(unsigned long)")]
pub fn stub_0x3a54e8(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::push_back(RBX::PoseAccumulator const&)")]
pub fn stub_0x3a5590(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::push_back(RBX::IAnimatableJoint * const&)")]
pub fn stub_0x3a55d0() -> crate::slot::PortedFn {
// IDA 0x3a55d0: std::vector<RBX::IAnimatableJoint*, std::allocator<RBX::IAnimatableJoint*>>::push_back(RBX::IAnimatableJoint* const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a55d0, "std::vector<RBX::IAnimatableJoint*, std::allocator<RBX::IAnimatableJoint*>>::push_back(RBX::IAnimata~")
}

#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Animator*>,boost::arg<1>>> const&)const")]
pub fn stub_0x3a55fc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list5<boost::_bi::value<RBX::Animator*>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>>)")]
pub fn stub_0x3a5704() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> const&)")]
pub fn stub_0x3a5778() -> crate::slot::PortedFn {
// IDA 0x3a5778: std::vector<RBX::PoseAccumulator, std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::PoseAccumulator, std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a5778, "std::vector<RBX::PoseAccumulator, std::allocator<RBX::PoseAccumulator>>::operator=(std::vector<RBX::~")
}

#[doc(alias = "RBX::Animator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x3a5878(handle: &crate::slot::InstanceHandle) {
// RBX::Animator::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Animator::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x3a5880(handle: &crate::slot::InstanceHandle) {
// RBX::Animator::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
pub fn stub_0x3a5884() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEE12getClassNameEv")]
pub fn stub_0x3a58ac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")]
pub fn stub_0x3a58d4() -> crate::slot::PortedFn {
// IDA 0x3a58d4: void RBX::Name::callDoDeclare<RBX::sAnimator>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a58d4, "void RBX::Name::callDoDeclare<RBX::sAnimator>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
pub fn stub_0x3a58d8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAnimator>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PoseAccumulator * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
pub fn stub_0x3a59b8(handle: &crate::slot::InstanceHandle) {
// RBX::PoseAccumulator* std::__copy<false, std::random_access_iterator_tag>::copy<RBX::PoseA~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PoseAccumulator* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::PoseAccumulator const*,RBX::PoseAccumulator*>(RBX::PoseAccumulator const*,RBX::PoseAccumulator const*,RBX::PoseAccumulator*)")]
pub fn stub_0x3a5a30(handle: &crate::slot::InstanceHandle) {
// RBX::PoseAccumulator* std::__copy<false, std::random_access_iterator_tag>::copy<RBX::PoseA~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_allocate(unsigned long)")]
pub fn stub_0x3a5aa8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Animator *>,boost::arg<1>,boost::_bi::value<double>,boost::_bi::value<RBX::KeyframeSequence::Priority>,boost::_bi::value<std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>>::operator()<boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::AnimationTrackState>&> &,int)")]
pub fn stub_0x3a5acc(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3a5acc: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::Animator,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)const")]
pub fn stub_0x3a5bb4() -> crate::slot::BindPiece {
// boost::bind fragment (mf4) composing a host BoundCall.
crate::slot::BindPiece::new("mf4")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::AnimationTrackState>>)")]
pub fn stub_0x3a5cb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Animator *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_0x3a5cd4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3a5cd4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Animator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x3a5dac() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAnimatableJoint **,std::vector<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>>,RBX::IAnimatableJoint * const&)")]
pub fn stub_0x3a5e94(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Vector_base<RBX::IAnimatableJoint *,std::allocator<RBX::IAnimatableJoint *>>::_M_allocate(unsigned long)")]
pub fn stub_0x3a5f74() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PoseAccumulator*,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>>>,RBX::PoseAccumulator const&)")]
pub fn stub_0x3a5f8c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::PoseAccumulator * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PoseAccumulator *,RBX::PoseAccumulator *>(RBX::PoseAccumulator *,RBX::PoseAccumulator *,RBX::PoseAccumulator *)")]
pub fn stub_0x3a6160(handle: &crate::slot::InstanceHandle) {
// RBX::PoseAccumulator* std::__copy_backward<false, std::random_access_iterator_tag>::__copy~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrack>::shared_ptr<RBX::AnimationTrack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3a61fc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrack")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrack,RBX::AnimationTrack>(rbx_core::SharedPtr<RBX::AnimationTrack> const*,RBX::AnimationTrack *)const")]
pub fn stub_0x3a62c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrack")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3a63ac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3a64b4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3a64b8]")]
pub fn stub_0x3a64b8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3a64bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3a64dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3a64f4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnimationTrackState>::shared_ptr<RBX::AnimationTrackState,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3a64f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AnimationTrackState,RBX::AnimationTrackState>(rbx_core::SharedPtr<RBX::AnimationTrackState> const*,RBX::AnimationTrackState *)const")]
pub fn stub_0x3a65c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3a66a8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3a67b0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3a67b4]")]
pub fn stub_0x3a67b4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3a67b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3a67d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnimationTrackState *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3a67f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_create_node(rbx_core::SharedPtr<RBX::AnimationTrackState> const&)")]
pub fn stub_0x3a67f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PartInstance>::shared_ptr<RBX::PartInstance>(RBX::PartInstance *)")]
pub fn stub_0x3a68d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PartInstance,RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const*,RBX::PartInstance *)const")]
pub fn stub_0x3a69c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PartInstance>(RBX::PartInstance *)")]
pub fn stub_0x3a6aac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p()")]
pub fn stub_0x3a6ba4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::~sp_counted_impl_p() [0x3a6ba8]")]
pub fn stub_0x3a6ba8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::dispose(void)")]
pub fn stub_0x3a6bac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_deleter(std::type_info const&)")]
pub fn stub_0x3a6bbc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PartInstance>::get_untyped_deleter(void)")]
pub fn stub_0x3a6bc0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3a6bc4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3a6bc8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3a6c68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3a6c70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3a6d14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8AnimatorELZNS_9sAnimatorEENS_17NonFactoryProductINS_8InstanceELZNS_9sAnimatorEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3a6d1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3a6dc0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x3a6f58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc() [0x3a6f88]")]
pub fn stub_0x3a6f88(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3a70a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Animator,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Animator*,rbx_core::SharedPtr<RBX::Instance> (RBX::Animator::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x3a718c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::AnimationTrackState>,std::allocator<rbx_core::SharedPtr<RBX::AnimationTrackState>>>::_M_clear(void)")]
pub fn stub_0x3a72b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AnimationTrackState")
}

#[doc(alias = "global constructor keyed to_a_158")]
pub fn stub_0x3a72e0() -> crate::slot::PortedFn {
// IDA 0x3a72e0: __GLOBAL__I_a_158.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3a72e0, "__GLOBAL__I_a_158")
}

#[doc(alias = "RBX::ArcHandles::setAxes(RBX::Axes)")]
pub fn stub_0x3a7640(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ArcHandles setter.
cell.set(value)
}

#[doc(alias = "RBX::ArcHandles::ArcHandles(void)")]
pub fn stub_0x3a7660() -> crate::slot::InstanceHandle {
// RBX::ArcHandles ctor.
crate::slot::InstanceHandle::new("RBX::ArcHandles")
}

#[doc(alias = "RBX::ArcHandles::getHandlesNormalIdMask(void)const")]
pub fn stub_0x3a7a9c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ArcHandles getter.
cell.get()
}

#[doc(alias = "RBX::ArcHandles::setServerGuiObject(void)")]
pub fn stub_0x3a7af0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ArcHandles setter.
cell.set(value)
}

#[doc(alias = "RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3a7b58(handle: &crate::slot::InstanceHandle) {
// RBX::ArcHandles::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ArcHandles::process(RBX::GuiEvent const&)")]
pub fn stub_0x3a7b94(handle: &crate::slot::InstanceHandle) {
// RBX::ArcHandles::process(RBX::GuiEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ArcHandles::process(RBX::GuiEvent const&)")]
pub fn stub_0x3a7ee4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::ArcHandles::getHandleType(void)const")]
pub fn stub_0x3a7ef0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ArcHandles getter.
cell.get()
}

#[doc(alias = "RBX::ArcHandles::getAxes(void)const")]
pub fn stub_0x3a7ef4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ArcHandles getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ArcHandles,RBX::Axes>::~PropDescriptor()")]
pub fn stub_0x3a7efc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::~RemoteEventDesc()")]
pub fn stub_0x3a7f20(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::~RemoteEventDesc()")]
pub fn stub_0x3a7f44(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
pub fn stub_0x3a7f68(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::EventReplicatorBase setter.
cell.set(value)
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
pub fn stub_0x3a80c8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::EventReplicatorBase setter.
cell.set(value)
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3a8228(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::ArcHandles, void (G3D::Vector3::Axis)>::onPropertyChanged(RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3a8288(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::ArcHandles, void (G3D::Vector3::Axis, float, float)>::onProp~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3a82e8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<3, void (G3D::Vector3::Axis, float, float)>::operator()(G3D~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")]
pub fn stub_0x3a8440(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::reset<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
pub fn stub_0x3a8584() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HandlesBase::MouseDownCaptureInfo")
}

#[doc(alias = "RBX::ArcHandles::~ArcHandles()")]
pub fn stub_0x3a85b0(handle: crate::slot::InstanceHandle) {
// RBX::ArcHandles dtor.
drop(handle);
}

#[doc(alias = "RBX::ArcHandles::~ArcHandles() [0x3a85b4]")]
pub fn stub_0x3a85b4(handle: crate::slot::InstanceHandle) {
// RBX::ArcHandles dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3a8654() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "RBX::HandlesBase::shouldRender2d(void)const")]
pub fn stub_0x3a8664(handle: &crate::slot::InstanceHandle) {
// RBX::HandlesBase::shouldRender2d() const — engine-side; linkage preserved via the alias.
let _ = handle;
}
